// use yew::{Component, Context, Html, Callback};
use yew::prelude::*;
use web_sys::Element;
use crate::work::util::{get_u32_from_ref, get_f64_from_ref, set_value_on_ref};
use crate::work::complex::Complex;
use crate::components::root::{MANDELBROT_DEFAULT_C_MIN, MANDELBROT_DEFAULT_C_MAX, MANDELBROT_DEFAULT_ITERATIONS, MandelbrotCfg};
use crate::agents::canvas_msg_bus::{CanvasSelectMsgBus, CanvasMsgRequest};
use yew_agent::{Bridge, Bridged};


pub enum Msg {
    ResetParams,
    ZoomOut,
    ResetArea,
    SaveConfig,
    Cancel,
    CanvasMsg(CanvasMsgRequest)
}

pub struct EditMandelbrotCfg {
    container_ref: NodeRef,
    iter_ref: NodeRef,
    c_min_real_ref: NodeRef,
    c_min_imag_ref: NodeRef,
    c_max_real_ref: NodeRef,
    c_max_imag_ref: NodeRef,
    _producer: Box<dyn Bridge<CanvasSelectMsgBus>>,
}
// config: Option<JuliaSetCfg>

impl Component for EditMandelbrotCfg {
    type Message = Msg;
    type Properties = EditMandelbrotCfgProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditMandelbrotCfg {
            container_ref: NodeRef::default(),
            iter_ref: NodeRef::default(),
            c_max_real_ref: NodeRef::default(),
            c_max_imag_ref: NodeRef::default(),
            c_min_real_ref: NodeRef::default(),
            c_min_imag_ref: NodeRef::default(),
            _producer: CanvasSelectMsgBus::bridge(ctx.link().callback(Msg::CanvasMsg)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // TODO: receive event from canvas select
        match msg {
            Msg::Cancel => {
                info!("EditMandelbrotCfg: got msg Cancel");
                self.container_ref
                    .cast::<Element>()
                    .expect("Container not found")
                    .set_class_name("edit_cntr_hidden");
                ctx.props().cb_canceled.emit(());
                // self.config = None;
                false
            }
            Msg::SaveConfig => {
                info!("EditMandelbrotCfg: got msg SaveConfig");
                self.container_ref
                    .cast::<Element>()
                    .expect("Container not found")
                    .set_class_name("edit_cntr_hidden");

                // TODO: add user visible error handlers
                let max_iterations = get_u32_from_ref(&self.iter_ref, "iterations")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.max_iterations
                    }, |v| v);

                let c_max_real = get_f64_from_ref(&self.c_max_real_ref, "c_max_real")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.c_max.real()
                    }, |v| v);

                let c_max_imag = get_f64_from_ref(&self.c_max_imag_ref, "c_max_imag")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.c_max.imag()
                    }, |v| v);

                let c_min_real = get_f64_from_ref(&self.c_max_real_ref, "c_min_real")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.c_min.real()
                    }, |v| v);

                let c_min_imag = get_f64_from_ref(&self.c_min_imag_ref, "c_min_imag")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.c_min.imag()
                    }, |v| v);

                ctx.props().cb_saved.emit(MandelbrotCfg {
                    max_iterations,
                    c_max: Complex::new(c_max_real, c_max_imag),
                    c_min: Complex::new(c_min_real, c_min_imag),
                });
                false
            }
            Msg::ResetArea => {
                info!("EditMandelbrotCfg: got msg ResetArea");
                set_value_on_ref(&self.c_max_real_ref,
                                 "x_max_real",
                                 MANDELBROT_DEFAULT_C_MAX.0.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                set_value_on_ref(&self.c_max_imag_ref,
                                 "x_max_imag",
                                 MANDELBROT_DEFAULT_C_MAX.1.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                set_value_on_ref(&self.c_min_real_ref,
                                 "x_min_real",
                                 MANDELBROT_DEFAULT_C_MIN.0.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                set_value_on_ref(&self.c_min_imag_ref,
                                 "x_min_imag",
                                 MANDELBROT_DEFAULT_C_MIN.1.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                true
            }
            Msg::ZoomOut => {
                info!("EditMandelbrotCfg: got msg ZoomOut");
                true
            }
            Msg::ResetParams => {
                info!("EditMandelbrotCfg: got msg ResetParams");
                true
            }
            Msg::CanvasMsg(canvas_msg) => {
                match canvas_msg {
                    CanvasMsgRequest::CanvasSelectMsg(coords) => {
                        info!("EditMandelbrotCfg: got msg CanvasSelect");
                        if ctx.props().edit_mode {
                            // TODO: implement
                            let x_scale = ctx.props().config.c_max.real() - ctx.props().config.c_min.real();
                            let y_scale = ctx.props().config.c_max.imag() - ctx.props().config.c_min.imag();
                            let c_min = Complex::new(ctx.props().config.c_min.real() + x_scale * f64::from(coords.0),
                                                     ctx.props().config.c_min.imag() + y_scale * f64::from(coords.1));
                            let c_max = Complex::new(ctx.props().config.c_max.real() + x_scale * f64::from(coords.2),
                                                     ctx.props().config.c_max.imag() + y_scale * f64::from(coords.3));
                            set_value_on_ref(&self.c_max_real_ref,
                                             "c_max_real",
                                             c_max.real().to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            set_value_on_ref(&self.c_max_imag_ref,
                                             "x_max_imag",
                                             c_max.imag().to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            set_value_on_ref(&self.c_min_real_ref,
                                             "c_min_real",
                                             c_min.real().to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            set_value_on_ref(&self.c_min_imag_ref,
                                             "c_min_imag",
                                             c_min.imag().to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset_area = ctx.link().callback(|_| Msg::ResetArea);
        let reset_params = ctx.link().callback(|_| Msg::ResetParams);
        let zoom_out = ctx.link().callback(|_| Msg::ZoomOut);
        let save_config = ctx.link().callback(|_| Msg::SaveConfig);
        let cancel = ctx.link().callback(|_| Msg::Cancel);
        let cntr_class = if ctx.props().edit_mode { "edit_cntr_visible" } else { "edit_cntr_hidden" };
        html![
            <div class={cntr_class} id="mandelbrot_edit_cntr" ref={self.container_ref.clone()}>
                <div class="input_cntr">
                    <p class="hint_text">
                        {"Hint: You can select a rectangle in the draw area to import the coordiates into the editor."}
                    </p>
                </div>
                <div class="input_cntr">
                    <div class="input_inner">
                        <label class="input_label" for="mandelbrot_iterations">
                            {"Iterations"}
                        </label>
                        <input class="input" id="mandelbrot_iterations" name="mandelbrot_iterations"
                            type="number" min="100" max="1000" ref={self.iter_ref.clone()}
                            value={ctx.props().config.max_iterations.to_string()}/>
                    </div>
                    <button class="editor_button" id="mandelbrot_reset_params" onclick={reset_params}>
                        {"Reset to Default"}
                    </button>
                </div>
                <div class="input_cntr">
                    <div class="input_inner">
                        <div class="area_cntr">
                            <div class="input_inner">
                                <label class="input_label" for="mandelbrot_c_max_real">
                                    {"C Max. Real"}
                                </label>
                                <input class="input" id="mandelbrot_c_max_real" name="mandelbrot_c_max_real"
                                    type="number" step="0.0000001" ref={self.c_max_real_ref.clone()}/>
                            </div>
                            <div class="input_inner">
                                <label class="input_label" for="mandelbrot_c_min_real">
                                    {"C Min. Real"}
                                </label>
                                <input class="input" id="mandelbrot_c_min_real" name="mandelbrot_c_min_real"
                                    type="number" step="0.0000001" ref={self.c_min_real_ref.clone()}/>
                            </div>
                        </div>
                        <div class="area_cntr">
                            <div class="input_inner">
                                <label class="input_label" for="mandelbrot_c_max_imag">
                                    {"C Max. Imag"}
                                </label>
                                <input class="input" id="mandelbrot_c_max_imag" name="mandelbrot_c_max_imag"
                                    type="number" step="0.0000001" ref={self.c_max_imag_ref.clone()}/>
                            </div>
                            <div class="input_inner">
                                <label class="input_label" for="mandelbrot_c_min_imag">
                                    {"C Min. Imag"}
                                </label>
                                <input class="input" id="mandelbrot_c_min_imag" name="mandelbrot_c_min_imag"
                                    type="number" step="0.0000001" ref={self.c_min_imag_ref.clone()}/>
                            </div>
                        </div>
                        <div class="area_cntr">
                            <button class="editor_button" id="mandelbrot_reset_area"
                                    onclick={reset_area}>
                                {"Reset to Default"}
                            </button>
                            <button class="editor_button" id="mandelbrot_zoom_out"
                                    onclick={zoom_out}>
                                {"Zoom Out"}
                            </button>
                        </div>
                    </div>
                </div>
                <div class="edit_button_cntr">
                    <button class="editor_button" id="mandelbrot_save" onclick={save_config}>
                        {"Save"}
                    </button>
                    <button class="editor_button" id="mandelbrot_cancel" onclick={cancel}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        ]
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct EditMandelbrotCfgProps {
    pub edit_mode: bool,
    pub config: MandelbrotCfg,
    pub cb_saved: Callback<MandelbrotCfg>,
    pub cb_canceled: Callback<()>,
}