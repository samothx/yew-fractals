use yew::prelude::*;
use yew_agent::{Dispatcher, Dispatched};
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Object;

use crate::agents::canvas_msg_bus::{CanvasMsgBus, Request};
use web_sys::{ImageData, HtmlCanvasElement, CanvasRenderingContext2d};
use yew::NodeRef;

pub struct CanvasElement {
    event_bus: Dispatcher<CanvasMsgBus>,
    mouse_drag: Option<MouseDrag>,
    canvas_ref: NodeRef,
    canvas: Option<HtmlCanvasElement>,
}


impl Component for CanvasElement {
    type Message = Msg;
    type Properties = CanvasProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            event_bus: CanvasMsgBus::dispatcher(),
            mouse_drag: None,
            canvas_ref: NodeRef::default(),
            canvas: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseMove(event) => {
                let mut res = false;
                if self.mouse_drag.is_some() {
                    let mouse_drag = self.mouse_drag.as_ref().expect("failed to unpack mouse_drag");
                    if let Some(image_data) = mouse_drag.image_data.as_ref() {
                        self.undraw(image_data);
                    }
                    let canvas_coords = self.viewport_to_canvas_coords(
                        event.client_x(), event.client_y())
                        .expect("Failed to retrieve canvas coordinates");
                    let image_data = self.draw_frame(mouse_drag.start.0, mouse_drag.start.1,
                                                     mouse_drag.curr.0, mouse_drag.curr.1);
                    let mouse_drag = self.mouse_drag.as_mut().expect("failed to unpack mouse_drag");
                    mouse_drag.curr = canvas_coords;
                    mouse_drag.image_data = Some(image_data);
                    res = true;
                }
                res
            }
            Msg::MouseUp(event) => {
                let mut res = false;
                if self.mouse_drag.is_some() {
                    let mut canvas_coords: Option<(u32,u32,u32,u32)> = None;

                    if let Some(mouse_drag) = self.mouse_drag.as_ref() {
                        if let Some(image_data) = mouse_drag.image_data.as_ref() {
                            self.undraw(image_data);
                        }
                        let canvas_coords_end = self.viewport_to_canvas_coords(
                            event.client_x(), event.client_y())
                            .expect("Failed to retrieve canvas coordinates");
                        canvas_coords = Some((mouse_drag.start.0,mouse_drag.start.1,
                                canvas_coords_end.0, canvas_coords_end.1));
                    }
                    self.mouse_drag = None;
                    if let Some(canvas_coords) = canvas_coords {
                        self.event_bus.send(Request::CanvasSelectMsg(canvas_coords));
                    }
                    res = true;
                }
                res
            }
            Msg::MouseDown(event) => {
                if ctx.props().edit_mode {
                    let canvas_coords = self.viewport_to_canvas_coords(
                        event.client_x(), event.client_y())
                        .expect("Failed to retrieve canvas coordinates");
                    self.mouse_drag = Some(MouseDrag {
                        start: canvas_coords,
                        curr: canvas_coords,
                        image_data: None,
                    });
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_mouse_up = ctx.link().callback(|ev| Msg::MouseUp(ev));
        let on_mouse_down = ctx.link().callback(|ev| Msg::MouseDown(ev));
        let on_mouse_move = ctx.link().callback(|ev| Msg::MouseMove(ev));

        html![
            <div class="canvas_cntr">
                <canvas class="canvas" id="canvas"
                    width={ctx.props().width.to_string()}
                    height={ctx.props().height.to_string()}
                    onmousedown={on_mouse_down}
                    onmouseup={on_mouse_up}
                    onmousemove={on_mouse_move}
                    ref={self.canvas_ref.clone()}
                >
                    {"Your browser does not support the canvas tag."}
                </canvas>
            </div>
        ]
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                self.canvas = Some(canvas)
            }
        }
    }
}

impl CanvasElement {
    pub fn draw_frame(&self, x_start: u32, y_start: u32, x_end: u32, y_end: u32) -> ImageData {
        // log!(format!("draw_frame: ({},{}),({},{})", x_start,y_start, x_end, y_end));

        if let Some(canvas) = self.canvas.as_ref() {
            let ctx = CanvasElement::get_2d_context(&canvas);
            // TODO: try this again later
            /*
            let image_width = f64::max(canvas_right - canvas_left, 1.0);
            let image_height = f64::max(canvas_bottom - canvas_top, 1.0);
            log!(format!("draw_frame: image coords: ({},{}),({},{})", canvas_left,canvas_top, image_width, image_height));
            let image_data =
                ctx.get_image_data(canvas_left, canvas_top, image_width, image_height)
                    .expect("failed to retrieve image data")
                    .dyn_into::<ImageData>().expect("Failed to cast to ImageData");
            */
            let image_data = ctx
                .get_image_data(
                    0.0,
                    0.0,
                    canvas.width().into(),
                    canvas.height().into(),
                )
                .expect("failed to retrieve image data");

            ctx.begin_path();
            ctx.set_stroke_style(&JsValue::from_str("#FFFFFF"));
            ctx.move_to(x_start.into(), y_start.into());
            ctx.line_to(x_end.into(), y_start.into());
            ctx.line_to(x_end.into(), y_end.into());
            ctx.line_to(x_start.into(), y_end.into());
            ctx.line_to(x_start.into(), y_start.into());
            ctx.stroke();
            image_data
        } else {
            panic!("canvas not initialized")
        }
    }

    pub fn undraw(&self, image_data: &ImageData) {
        // log!(format!("undraw: ({},{}) width: {} height: {}", x_start,y_start, image_data.width(), image_data.height()));
        if let Some(canvas) = self.canvas.as_ref() {
            let ctx = CanvasElement::get_2d_context(&canvas);
            ctx.put_image_data(image_data, 0.0, 0.0)
                .expect("cannot draw image data");
        } else {
            error!("canvas not initialized")
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn viewport_to_canvas_coords(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        if let Some(canvas) = self.canvas.as_ref() {
            let bounding_rect = canvas.get_bounding_client_rect();
            let scale_x = f64::from(canvas.width()) / bounding_rect.width();
            let scale_y = f64::from(canvas.height()) / bounding_rect.height();
            let canvas_x = (f64::from(x) - bounding_rect.left()) * scale_x;
            let canvas_y = (f64::from(y) - bounding_rect.top()) * scale_y;
            if canvas_x >= 0.0
                && canvas_x < f64::from(canvas.width())
                && canvas_y >= 0.0
                && canvas_y < f64::from(canvas.height())
            {
                Some((canvas_x.abs() as u32, canvas_y.abs() as u32))
            } else {
                None
            }
        } else {
            error!("canvas not initialized");
            None
        }
    }

    #[inline]
    fn get_2d_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        let tmp1: Object = canvas.get_context("2d")
            .map_or_else(|err| {
                panic!("failed to retrieve CanvasRenderingContext2d {:?}", err)
            }, |v| v)
            .expect("2d context not found in canvas");
        let tmp2: &JsValue = tmp1.as_ref();
        tmp2.clone().dyn_into::<CanvasRenderingContext2d>().expect("Failed to cast to CanvasRenderingContext2d")
    }
}

pub enum Msg {
    MouseUp(MouseEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
}

struct MouseDrag {
    start: (u32, u32),
    curr: (u32, u32),
    image_data: Option<ImageData>,
}


#[derive(Properties, PartialEq, Clone)]
pub struct CanvasProps {
    pub width: u32,
    pub height: u32,
    pub edit_mode: bool,
}
