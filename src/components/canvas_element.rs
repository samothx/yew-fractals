use std::cell::RefCell;
use std::rc::Rc;

use gloo::render::request_animation_frame;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, ImageData};
use yew::prelude::*;
use yew::NodeRef;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};

use crate::{
    agents::{
        canvas_msg_bus::{CanvasMsgRequest, CanvasSelectMsgBus},
        command_msg_bus::{CommandMsgBus, CommandRequest},
    },
    components::root::Config,
    work::{canvas::Canvas, fractal::FractalCalculator, stats::Stats},
};

const FPS_RESTRICTED_TIMER: bool = false;

pub struct CanvasElement {
    event_bus: Dispatcher<CanvasSelectMsgBus>,
    mouse_drag: Option<MouseDrag>,
    canvas_ref: NodeRef,
    canvas: Option<Canvas>,
    _producer: Box<dyn Bridge<CommandMsgBus>>,
    // config: Config,
    calculator: Option<FractalCalculator>,
    stats: Option<Stats>,
    paused: bool,
    on_draw: Callback<()>,
}

impl Component for CanvasElement {
    type Message = Msg;
    type Properties = CanvasProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            event_bus: CanvasSelectMsgBus::dispatcher(),
            mouse_drag: None,
            canvas_ref: NodeRef::default(),
            canvas: None,
            _producer: CommandMsgBus::bridge(ctx.link().callback(Msg::Command)),
            // config: ctx.props().config.clone(),
            calculator: None,
            stats: None,
            paused: true,
            on_draw: ctx.link().callback(|_| Msg::OnDraw),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseMove(event) => {
                let mut res = false;
                if self.mouse_drag.is_some() {
                    if let Some(canvas) = self.canvas.as_ref() {
                        let mouse_drag = self
                            .mouse_drag
                            .as_ref()
                            .expect("failed to unpack mouse_drag");
                        if let Some(image_data) = mouse_drag.image_data.as_ref() {
                            canvas.undraw(image_data);
                        }
                        let canvas_coords = canvas
                            .viewport_to_canvas_coords(event.client_x(), event.client_y())
                            .expect("Failed to retrieve canvas coordinates");
                        let image_data = canvas.draw_frame(
                            mouse_drag.start.0,
                            mouse_drag.start.1,
                            mouse_drag.curr.0,
                            mouse_drag.curr.1,
                        );
                        let mouse_drag = self
                            .mouse_drag
                            .as_mut()
                            .expect("failed to unpack mouse_drag");
                        mouse_drag.curr = canvas_coords;
                        mouse_drag.image_data = Some(image_data);
                    }
                    res = true;
                }
                res
            }
            Msg::MouseUp(event) => {
                let mut res = false;
                if self.mouse_drag.is_some() {
                    if let Some(canvas) = self.canvas.as_ref() {
                        let mut canvas_coords: Option<(u32, u32, u32, u32)> = None;

                        if let Some(mouse_drag) = self.mouse_drag.as_ref() {
                            if let Some(image_data) = mouse_drag.image_data.as_ref() {
                                canvas.undraw(image_data);
                            }
                            let canvas_coords_end = canvas
                                .viewport_to_canvas_coords(event.client_x(), event.client_y())
                                .expect("Failed to retrieve canvas coordinates");
                            canvas_coords = Some((
                                mouse_drag.start.0,
                                mouse_drag.start.1,
                                canvas_coords_end.0,
                                canvas_coords_end.1,
                            ));
                        }
                        self.mouse_drag = None;
                        if let Some(canvas_coords) = canvas_coords {
                            self.event_bus
                                .send(CanvasMsgRequest::CanvasSelectMsg(canvas_coords));
                        }
                    }
                    res = true;
                }
                res
            }
            Msg::MouseDown(event) => {
                if ctx.props().edit_mode {
                    if let Some(canvas) = self.canvas.as_ref() {
                        let canvas_coords = canvas
                            .viewport_to_canvas_coords(event.client_x(), event.client_y())
                            .expect("Failed to retrieve canvas coordinates");
                        self.mouse_drag = Some(MouseDrag {
                            start: canvas_coords,
                            curr: canvas_coords,
                            image_data: None,
                        });
                    }
                }
                false
            }
            Msg::TouchStart(ev) => {
                info!("CanvasElement::update: Msg Received: TouchStart: {:?}", ev);
                if ctx.props().edit_mode {
                    if let Some(canvas) = self.canvas.as_ref() {
                        let touch = ev.touches().item(0).expect("No touch found in event");
                        let canvas_coords = canvas
                            .viewport_to_canvas_coords(touch.client_x(), touch.client_y())
                            .expect("Failed to retrieve canvas coordinates");
                        info!(
                            "TouchStart: Client x/y({},{}), Canvas x/y {:?}",
                            touch.client_x(),
                            touch.client_y(),
                            canvas_coords
                        );
                        self.mouse_drag = Some(MouseDrag {
                            start: canvas_coords,
                            curr: canvas_coords,
                            image_data: None,
                        });
                    }
                }

                false
            }
            Msg::TouchEnd(ev) => {
                info!("CanvasElement::update: Msg Received: TouchEnd: {:?}", ev);
                let mut res = false;
                if let Some(mouse_drag) = self.mouse_drag.as_ref() {
                    if let Some(canvas) = self.canvas.as_ref() {
                        // let mut canvas_coords: Option<(u32, u32, u32, u32)> = None;

                        if let Some(image_data) = mouse_drag.image_data.as_ref() {
                            canvas.undraw(image_data);
                        }

                        let canvas_coords = Some((
                            mouse_drag.start.0,
                            mouse_drag.start.1,
                            mouse_drag.curr.0,
                            mouse_drag.curr.1,
                        ));

                        self.mouse_drag = None;
                        if let Some(canvas_coords) = canvas_coords {
                            self.event_bus
                                .send(CanvasMsgRequest::CanvasSelectMsg(canvas_coords));
                        }
                    }
                    res = true;
                }
                res
            }
            Msg::TouchMove(ev) => {
                info!("CanvasElement::update: Msg Received: TouchMove: {:?}", ev);
                let mut res = false;
                if self.mouse_drag.is_some() {
                    if let Some(canvas) = self.canvas.as_ref() {
                        let mouse_drag = self
                            .mouse_drag
                            .as_ref()
                            .expect("failed to unpack mouse_drag");
                        if let Some(image_data) = mouse_drag.image_data.as_ref() {
                            canvas.undraw(image_data);
                        }
                        let touch = ev.touches().item(0).expect("No touch found in event");
                        let canvas_coords = canvas
                            .viewport_to_canvas_coords(touch.client_x(), touch.client_y())
                            .expect("Failed to retrieve canvas coordinates");

                        let image_data = canvas.draw_frame(
                            mouse_drag.start.0,
                            mouse_drag.start.1,
                            mouse_drag.curr.0,
                            mouse_drag.curr.1,
                        );
                        let mouse_drag = self
                            .mouse_drag
                            .as_mut()
                            .expect("failed to unpack mouse_drag");
                        mouse_drag.curr = canvas_coords;
                        mouse_drag.image_data = Some(image_data);
                    }
                    res = true;
                }
                res
            }
            Msg::Command(request) => {
                info!(
                    "CanvasElement::update: Msg Received: Command: {:?}",
                    request
                );
                match request {
                    CommandRequest::Start => {
                        info!("CanvasElement::update: starting");
                        self.event_bus.send(CanvasMsgRequest::FractalStarted);

                        if let Some(canvas) = self.canvas.as_mut() {
                            canvas
                                .clear_canvas(ctx.props().canvas_width, ctx.props().canvas_height);
                        }

                        if ctx.props().config.view_stats {
                            self.stats = Some(Stats::new(
                                ctx.props().canvas_width as usize
                                    * ctx.props().canvas_height as usize,
                            ));
                        }

                        let mut calculator = FractalCalculator::new(
                            &ctx.props().config,
                            ctx.props().canvas_width,
                            ctx.props().canvas_height,
                        );

                        let points = calculator.calculate(self.stats.as_mut());
                        if let Some(stats) = self.stats.as_ref() {
                            self.event_bus
                                .send(CanvasMsgRequest::FractalProgress(stats.format_stats()));
                        }
                        if let Some(canvas) = self.canvas.as_ref() {
                            canvas.draw_results(points);
                        }

                        self.calculator = Some(calculator);
                        self.paused = false;
                        self.send_draw_ev();
                        false
                    }
                    CommandRequest::Stop => {
                        self.paused = true;
                        self.event_bus.send(CanvasMsgRequest::FractalPaused);
                        false
                    }
                    CommandRequest::Clear => {
                        self.paused = true;
                        self.event_bus.send(CanvasMsgRequest::FractalPaused);
                        if let Some(canvas) = self.canvas.as_mut() {
                            canvas
                                .clear_canvas(ctx.props().canvas_width, ctx.props().canvas_height);
                        }
                        false
                    }
                }
            }
            Msg::OnDraw => {
                // info!("CanvasElement::update: OnDraw");
                if !self.paused {
                    if let Some(calculator) = self.calculator.as_mut() {
                        let points = calculator.calculate(self.stats.as_mut());
                        if let Some(stats) = self.stats.as_ref() {
                            self.event_bus
                                .send(CanvasMsgRequest::FractalProgress(stats.format_stats()));
                        }
                        if let Some(canvas) = self.canvas.as_ref() {
                            canvas.draw_results(points);
                            // TODO: send stats
                            if calculator.is_done() {
                                // TODO: send notifications
                                self.paused = true;
                                self.event_bus.send(CanvasMsgRequest::FractalPaused);
                            } else {
                                self.send_draw_ev();
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_mouse_up = ctx.link().callback(Msg::MouseUp);
        let on_mouse_down = ctx.link().callback(Msg::MouseDown);
        let on_mouse_move = ctx.link().callback(Msg::MouseMove);
        // TODO: sort these out
        let on_touch_start = ctx.link().callback(Msg::TouchStart);
        let on_touch_end = ctx.link().callback(Msg::TouchEnd);
        let on_touch_move = ctx.link().callback(Msg::TouchMove);

        html![
            <div class="canvas_cntr">
                <canvas class="canvas" id="canvas"
                    width={ctx.props().canvas_width.to_string()}
                    height={ctx.props().canvas_height.to_string()}
                    onmousedown={on_mouse_down}
                    onmouseup={on_mouse_up}
                    onmousemove={on_mouse_move}
                    ontouchstart={on_touch_start}
                    ontouchend={on_touch_end}
                    ontouchmove={on_touch_move}
                    ref={self.canvas_ref.clone()}
                >
                    {"Your browser does not support the canvas tag."}
                </canvas>
            </div>
        ]
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(canvas_el) = self.canvas_ref.cast::<HtmlCanvasElement>() {
                let mut canvas =
                    Canvas::new(canvas_el, &ctx.props().config, ctx.props().canvas_width);
                canvas.clear_canvas(ctx.props().canvas_width, ctx.props().canvas_height);
                self.canvas = Some(canvas);
            }
        }
    }
}

impl CanvasElement {
    fn send_draw_ev(&self) {
        let callback = self.on_draw.clone();
        if FPS_RESTRICTED_TIMER {
            let cell = Rc::new(RefCell::new(None));
            let future_cell = cell.clone();
            let requested = request_animation_frame(move |_time| {
                let _drop_side_effect = future_cell;
                callback.emit(())
            });
            (*cell).replace(Some(requested));
        } else {
            spawn_local(async move {
                TimeoutFuture::new(0).await;
                callback.emit(())
            });
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    MouseUp(MouseEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    TouchStart(TouchEvent),
    TouchEnd(TouchEvent),
    TouchMove(TouchEvent),
    Command(CommandRequest),
    OnDraw,
}

struct MouseDrag {
    start: (u32, u32),
    curr: (u32, u32),
    image_data: Option<ImageData>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CanvasProps {
    pub config: Config,
    pub edit_mode: bool,
    pub canvas_width: u32,
    pub canvas_height: u32,
}
