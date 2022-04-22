use yew_agent::{Agent, AgentLink, HandlerId, Job};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use gloo_file::Blob;
use web_sys::{ImageData, HtmlCanvasElement, CanvasRenderingContext2d, PermissionState, PermissionStatus, window, Performance};
use png_encode_mini::write_rgba_from_u8;
use js_sys::{Object, Reflect, Promise};


use wasm_bindgen::prelude::*;
use std::fmt::{Debug, Formatter};
use gloo::timers::future::TimeoutFuture;



#[wasm_bindgen(inline_js = "export const ClipboardItem = window.ClipboardItem")]
extern "C" {
    pub type ClipboardItem;

    #[wasm_bindgen(constructor, js_class = "ClipboardItem")]
    pub fn new(files: &js_sys::Object) -> ClipboardItem;
}

struct RawData {
    data: ImageData,
    rows: usize,
    cols: usize
}

struct FlippedData {
    data: Vec<u8>,
    rows: usize,
    cols: usize
}

struct PngData {
    data: Vec<u8>,
}

enum Stage {
    Init,
    Processing,
    Flip(RawData),
    PngEncode(FlippedData),
    MakeItems(PngData),
    Copy(js_sys::Array)
}

enum StageRes {
    Done,
    Pending
}

impl Debug for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Stage::Init => "Init",
            Stage::Flip(_) => "Flip",
            Stage::PngEncode(_) => "PngEncode",
            Stage::MakeItems(_) => "MakeItems",
            Stage::Copy(_) => "Copy",
            Stage::Processing => "Processing"
        })
    }
}



#[derive(Deserialize, Serialize, Debug)]
pub enum WorkerStatus {
    Complete,
    Pending,
    Failure(String)
}

pub struct ClipboardWorker {
    performance: Performance,
    start: Option<f64>,
    link: AgentLink<ClipboardWorker>,
    stage: Stage
}

impl ClipboardWorker {
    #[inline]
    fn get_canvas_and_context() -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), String> {
        let canvas = window().ok_or("Window not found".to_owned())?
            .document().ok_or("Document not found in window".to_owned())?
            .get_element_by_id("canvas").ok_or("canvas not found in document")?
            .dyn_into::<HtmlCanvasElement>().map_err(|err| format!("Failed to cast to HtmlCanvasElement: {:?}", err))?;
        //.expect("Failed to cast to HtmlCanvasElement");

        let tmp1: Object = canvas.get_context("2d")
            .map_or_else(|err| {
                panic!("failed to retrieve CanvasRenderingContext2d {:?}", err)
            }, |v| v)
            .expect("2d context not found in canvas");
        let tmp2: &JsValue = tmp1.as_ref();
        Ok((canvas, tmp2.clone().dyn_into::<CanvasRenderingContext2d>().expect("Failed to cast to CanvasRenderingContext2d")))
    }

    fn handle_init(&mut self) -> Result<StageRes,String> {
        let (canvas,context) = Self::get_canvas_and_context()?;
        let image_data = context
            .get_image_data(
                0.0,
                0.0,
                canvas.width().into(),
                canvas.height().into(),
            ).map_err(|err| format!("Failed to retrieve image data, error: {:?}", err))?
            .dyn_into::<ImageData>()
            .map_err(|err| format!("Failed to cast to ImageData, error: {:?}", err))?;

        self.stage = Stage::Flip(RawData {
            data: image_data,
            rows: canvas.height() as usize,
            cols: canvas.width() as usize,
        });
        Ok(StageRes::Done)
    }

    fn handle_flip(&mut self, stage_data: RawData) -> Result<StageRes,String> {
        let data = stage_data.data.data();
        let mut flipped_u8: Vec<u8> = Vec::with_capacity(data.len());
        let bytes_per_row = stage_data.cols * 4;
        assert_eq!(data.len(),bytes_per_row * stage_data.rows);
        for row in (0..stage_data.rows).rev() {
            flipped_u8.extend_from_slice(
                &data[(row * bytes_per_row)..((row + 1) * bytes_per_row)]);
        }
        assert_eq!(data.len(),flipped_u8.len());
        info!("handle_flip bytes_per_row: {}, rows: {}, data.len(): {}, flipped len: {}", bytes_per_row, stage_data.rows, data.len(), flipped_u8.len());
        self.stage = Stage::PngEncode(FlippedData {
            data: flipped_u8,
            rows: stage_data.rows,
            cols: stage_data.cols
        });
        Ok(StageRes::Done)
    }

    fn handle_png_encode(&mut self, data: FlippedData) -> Result<StageRes,String> {
        let mut img_u8: Vec<u8> = Vec::new();
        write_rgba_from_u8(&mut img_u8,
                           &(*data.data)[..],
                           data.cols as u32,
                           data.rows as u32)
            .map_err(|err| format!("Failed to convert image data to png, error: {:?}", err))?;
        self.stage = Stage::MakeItems(PngData{
            data: img_u8
        });
        Ok(StageRes::Done)
    }

    fn handle_make_items(&mut self, data: PngData)  -> Result<StageRes,String> {
        info!("copy_to_clipboard: creating gloo::Blob from png");
        let blob = Blob::new_with_options(&data.data[..], Some("image/png"));
        info!("copy_to_clipboard: creating ClipboardItem from Blob");
        let clip_content = Object::new();
        Reflect::set(&clip_content, &JsValue::from("image/png"), &JsValue::from(blob))
            .map_err(|err| format!("Failed to write blob to object, error: {:?}", err))?;

        let item = ClipboardItem::new(&clip_content);
        self.stage = Stage::Copy([item].iter().collect::<js_sys::Array>());
        Ok(StageRes::Done)
    }


    async fn process_async(query_promise: Promise, data: js_sys::Array) -> Result<(),String> {
        let query_res = JsFuture::from(query_promise).await
            .map_err(|err| format!("Failure awaiting copy permission query result: error {:?}", err))?;

        let status = PermissionStatus::from(query_res);
        // info!("copy_to_clipboard: got PermissionStatus {:?}", status.state());
        if status.state() == PermissionState::Granted {
            info!("copy_to_clipboard: got permission to copy to clipboard");
            let clipboard = window().ok_or("Window not found".to_owned())?
                .navigator()
                .clipboard().ok_or("Clipboard not found".to_owned())?;

            JsFuture::from(clipboard.write(&data.into())).await
                .map_err(|err| format!("Failed to copy image to clipboard: {:?}", err))?;

            Ok(())
        } else {
            Err(format!("copy_to_clipboard: not permitted to copy to clipboard: {:?}", status.state()))
        }
    }

    fn handle_copy(&mut self, data: js_sys::Array, id: HandlerId) -> Result<StageRes,String> {
        info!("copy_to_clipboard: creating query for clipboard permissions");
        let query_obj = Object::new();
        Reflect::set(&query_obj, &JsValue::from("name"), &JsValue::from("clipboard-write"))
            .map_err(|err| format!("Failed to write query to object, error: {:?}", err))?;

        // info!("copy_to_clipboard: got query_object: {:?}", query_obj);
        let query_res = window().ok_or("Window not found".to_owned())?
            .navigator()
            .permissions().map_err(|err| format!("Permissions not found, error: {:?}", err))?
            .query(&query_obj).map_err(|err| format!("Failed to query permissions, error: {:?}", err))?;
        let link_copy = self.link.clone();
        spawn_local(async move {
            link_copy.respond(
                id,
                match Self::process_async(query_res, data).await {
                    Ok(()) => WorkerStatus::Complete,
                    Err(err) => WorkerStatus::Failure(err)
                }
            );
        });
        Ok(StageRes::Pending)
    }

    fn send_result(&self, status: WorkerStatus, id: HandlerId) {
        let link_copy = self.link.clone();
        spawn_local(async move {
            TimeoutFuture::new(0).await;
            link_copy.respond(
                id,
                status,
            );
        });
    }

    fn process_stage(&mut self, id: HandlerId) {
        let stage = std::mem::replace(&mut self.stage, Stage::Processing);
        match match stage {
            Stage::Init => self.handle_init(),
            Stage::Flip(data) => self.handle_flip(data),
            Stage::PngEncode(data) => self.handle_png_encode(data),
            Stage::MakeItems(data) => self.handle_make_items(data),
            Stage::Copy(data) => self.handle_copy(data, id),
            Stage::Processing => Err("Invalid stage encountered".to_owned())
        } {
            Ok(res) => {
                match res {
                    StageRes::Done => {
                        self.send_result(WorkerStatus::Pending, id);
                    }
                    StageRes::Pending => {
                        ()
                    }
                }
            },
            Err(err) => {
                self.send_result(WorkerStatus::Failure(err), id);
            }
        }
    }
}

impl Agent for ClipboardWorker {
    type Reach = Job<Self>;
    type Message = ();
    type Input = ();
    type Output = WorkerStatus;

    fn create(link: AgentLink<Self>) -> Self {
        info!("Creating clipboard worker");
        ClipboardWorker{
            performance: window()
                .expect("Window not found")
                .performance()
                .expect("performance should be available"),
            start: None,
            stage: Stage::Init,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) { }


    fn handle_input(&mut self, _msg: Self::Input, id: HandlerId) {
        let offset_sec = if let Some(start) = self.start {
            (self.performance.now() - start) / 1000.0
        } else {
            0.0
        };

        info!("ClipboardWorker::handle_input: stage: {:?} @ offset {} secs", self.stage, offset_sec);
        self.process_stage(id);
    }
}
