use yew_agent::{Agent, AgentLink, HandlerId, Job};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use gloo_file::Blob;
use web_sys::{ImageData, HtmlCanvasElement, CanvasRenderingContext2d, PermissionState,
                  PermissionStatus, window};
use png_encode_mini::write_rgba_from_u8;
use js_sys::{Object,Reflect};


use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export const ClipboardItem = window.ClipboardItem")]
extern "C" {
    pub type ClipboardItem;

    #[wasm_bindgen(constructor, js_class = "ClipboardItem")]
    pub fn new(files: &js_sys::Object) -> ClipboardItem;
}


#[derive(Deserialize, Serialize, Debug)]
pub struct OutputMsg {
    pub success: bool,
    pub msg: Option<String>
}

pub struct ClipboardWorker {
    link: AgentLink<ClipboardWorker>,
}

impl ClipboardWorker {
    #[inline]
    fn get_canvas_and_context() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
        let canvas = window().expect("Window not found").
            document().expect("Document not found in window")
            .get_element_by_id("canvas").expect("canvas not found in document")
            .dyn_into::<HtmlCanvasElement>().expect("Failed to cast to HtmlCanvasElement");

        let tmp1: Object = canvas.get_context("2d")
            .map_or_else(|err| {
                panic!("failed to retrieve CanvasRenderingContext2d {:?}", err)
            }, |v| v)
            .expect("2d context not found in canvas");
        let tmp2: &JsValue = tmp1.as_ref();
        (canvas, tmp2.clone().dyn_into::<CanvasRenderingContext2d>().expect("Failed to cast to CanvasRenderingContext2d"))
    }
}

impl Agent for ClipboardWorker {
    type Reach = Job<Self>;
    type Message = ();
    type Input = ();
    type Output = OutputMsg;

    fn create(link: AgentLink<Self>) -> Self {
        info!("Creating clipboard worker");
        ClipboardWorker{ link }
    }

    fn update(&mut self, _msg: Self::Message) { }

    fn handle_input(&mut self, _msg: Self::Input, id: HandlerId) {
        info!("Clipboard web worker starting");
        let performance = window()
            .expect("Window not found")
            .performance()
            .expect("performance should be available");

        let start = performance.now();

        let (canvas,context) = Self::get_canvas_and_context();

        let image_data = context
            .get_image_data(
                0.0,
                0.0,
                canvas.width().into(),
                canvas.height().into(),
            )
            .expect("failed to retrieve image data")
            .dyn_into::<ImageData>()
            .expect("Failed to cast to ImageData").data();

        info!("copy_to_clipboard: flipping image data vertically at offset: {:.3} secs", (performance.now() - start) / 1000.0);
        let mut flipped_u8: Vec<u8> = Vec::new();
        flipped_u8.resize(image_data.len(),0);
        let rows = canvas.height() as usize;
        let cols = canvas.width() as usize;
        (0..rows).for_each(|row|{
            (0..cols).for_each(|col|{
                let read_offset = (row * cols + col) * 4;
                let write_offset = ((rows - row - 1) * cols + col) * 4;
                (0..4).for_each(|idx| flipped_u8[write_offset + idx] = image_data[read_offset + idx]);
            })
        });

        info!("copy_to_clipboard: converting to png at offset: {:.3} secs", (performance.now() - start) / 1000.0);

        let mut img_u8: Vec<u8> = Vec::new();
        let items = match write_rgba_from_u8(&mut img_u8,
                                             &(*flipped_u8)[..],
                                             canvas.width(),
                                             canvas.height()) {
            Ok(()) => {
                // Create an U8Array
                info!("copy_to_clipboard: creating gloo::Blob from png at offset: {:.3} secs",
                    (performance.now() - start) / 1000.0);
                let blob = Blob::new_with_options(&img_u8[..], Some("image/png"));
                info!("copy_to_clipboard: creating ClipboardItem from Blob  at offset: {:.3} secs", (performance.now() - start) / 1000.0);
                let clip_content = Object::new();
                Reflect::set(&clip_content, &JsValue::from("image/png"), &JsValue::from(blob))
                    .expect("Failed to write blob to object ");

                let item = ClipboardItem::new(&clip_content);

                // info!("copy_to_clipboard: got ClipboardItem: {:?}", item);
                // Create array of ClipboardItems
                info!("copy_to_clipboard: creating Array of ClipboardItems at offset: {:.3} secs", (performance.now() - start) / 1000.0);
                let items = [item].iter().collect::<js_sys::Array>();
                // let items = js_sys::Array::new();
                // items.push(&item);
                items
            },
            Err(err) => {
                error!("Failed to create png from imagedata, error: {:?}", err);
                return;
            }
        };

        info!("copy_to_clipboard: creating query for clipboard permissions at offset: {:.3} secs", (performance.now() - start) / 1000.0);
        let query_obj = Object::new();
        Reflect::set(&query_obj, &JsValue::from("name"), &JsValue::from("clipboard-write"))
            .expect("Failed to write blob to object ");

        // info!("copy_to_clipboard: got query_object: {:?}", query_obj);
        match window().expect("Window not found")
            .navigator()
            .permissions()
            .expect("no permissions found in navigator")
            .query(&query_obj)
        {
            Ok(result) => {
                info!("copy_to_clipboard: query permission returned ok");
                let link_copy = self.link.clone();
                spawn_local(async move {
                    let query_res = JsFuture::from(result)
                        .await
                        .expect("Query promise was rejected");
                    let status = PermissionStatus::from(query_res);
                    // info!("copy_to_clipboard: got PermissionStatus {:?}", status.state());
                    if status.state() == PermissionState::Granted {
                        info!("copy_to_clipboard: got permission to copy to clipboard");

                        let clipboard = window().expect("Window not found")
                            .navigator()
                            .clipboard().expect("Clipboard not found");
                        match JsFuture::from(clipboard.write(&items.into())).await {
                            Ok(_res) => {
                                info!("copy_to_clipboard: png image copied to clipboard");
                                link_copy.respond(
                                    id,
                                    OutputMsg{ success: true, msg: None } ,
                                );
                            },
                            Err(err) => {
                                error!("copy_to_clipboard: failed to copy png image to clipboard, error: {:?}", err);
                                link_copy.respond(
                                    id,
                                    OutputMsg{
                                        success: true,
                                        msg: Some(format!("Failed to copy image to clipboard: {:?}", err))
                                    });
                            }
                        }
                    } else {
                        warn!("copy_to_clipboard: not permitted to copy to clipboard: {:?}", status.state());
                    }
                });
            }
            Err(err) => {
                warn!(
                    "copy_to_clipboard: error from query permissions, msg: {}",
                    err.as_string().unwrap_or_else(|| "None".to_string())
                );
            }
        }

        // TODO: this needs to happen in the Clipboard write future
    }
}

