
// use yew::{Component, Context, Html, Callback};
use yew::prelude::*;
use super::root::{JuliaSetCfg};
use web_sys::Element;
use crate::work::util::{get_u32_from_ref, get_f64_from_ref, set_value_on_ref};
use crate::work::complex::Complex;
use crate::components::root::{JULIA_DEFAULT_X_MAX, JULIA_DEFAULT_X_MIN};
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

pub struct EditJuliaCfg {
    container_ref: NodeRef,
    iter_ref: NodeRef,
    c_real_ref: NodeRef,
    c_imag_ref: NodeRef,
    x_min_real_ref: NodeRef,
    x_min_imag_ref: NodeRef,
    x_max_real_ref: NodeRef,
    x_max_imag_ref: NodeRef,
    _producer: Box<dyn Bridge<CanvasSelectMsgBus>>,
}
    // config: Option<JuliaSetCfg>

impl  Component for EditJuliaCfg {
    type Message = Msg;
    type Properties = EditJuliaCfgProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditJuliaCfg{
            container_ref: NodeRef::default(),
            iter_ref: NodeRef::default(),
            c_real_ref: NodeRef::default(),
            c_imag_ref: NodeRef::default(),
            x_max_real_ref: NodeRef::default(),
            x_max_imag_ref: NodeRef::default(),
            x_min_real_ref: NodeRef::default(),
            x_min_imag_ref: NodeRef::default(),
            _producer: CanvasSelectMsgBus::bridge(ctx.link().callback(Msg::CanvasMsg)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // TODO: receive event from canvas select
        match msg {
            Msg::Cancel => {
                info!("EditJuliaCfg: got msg Cancel");
                self.container_ref
                    .cast::<Element>()
                    .expect("Container not found")
                    .set_class_name("edit_cntr_hidden");
                ctx.props().cb_canceled.emit(());
                // self.config = None;
                false
            },
            Msg::SaveConfig => {
                info!("EditJuliaCfg: got msg SaveConfig");
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

                let c_real = get_f64_from_ref(&self.c_real_ref, "c_real")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.c.real()
                    }, |v| v);

                let c_imag = get_f64_from_ref(&self.c_imag_ref, "c_imag")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.c.imag()
                    }, |v| v);

                let x_max_real = get_f64_from_ref(&self.x_max_real_ref, "x_max_real")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.x_max.real()
                    }, |v| v);

                let x_max_imag = get_f64_from_ref(&self.x_max_imag_ref, "x_max_imag")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.x_max.imag()
                    }, |v| v);

                let x_min_real = get_f64_from_ref(&self.x_max_real_ref, "x_min_real")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.x_min.real()
                    }, |v| v);

                let x_min_imag = get_f64_from_ref(&self.x_min_imag_ref, "x_min_imag")
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                        ctx.props().config.x_min.imag()
                    }, |v| v);

                ctx.props().cb_saved.emit(JuliaSetCfg{
                    max_iterations,
                    c: Complex::new(c_real, c_imag),
                    x_max: Complex::new(x_max_real, x_max_imag),
                    x_min: Complex::new(x_min_real, x_min_imag)
                });
                false
            },
            Msg::ResetArea => {
                info!("EditJuliaCfg: got msg ResetArea");
                set_value_on_ref(&self.x_max_real_ref,
                                 "x_max_real",
                                 JULIA_DEFAULT_X_MAX.0.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                set_value_on_ref(&self.x_max_imag_ref,
                                 "x_max_imag",
                                 JULIA_DEFAULT_X_MAX.1.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                set_value_on_ref(&self.x_min_real_ref,
                                 "x_min_real",
                                 JULIA_DEFAULT_X_MIN.0.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                set_value_on_ref(&self.x_min_imag_ref,
                                 "x_min_imag",
                                 JULIA_DEFAULT_X_MIN.1.to_string().as_str())
                    .map_or_else(|err| {
                        error!("{}",err.as_str());
                    }, |v| v);
                true
            },
            Msg::ZoomOut => {
                info!("EditJuliaCfg: got msg ZoomOut");
                true
            },
            Msg::ResetParams => {
                info!("EditJuliaCfg: got msg ResetParams");
                true
            }
            Msg::CanvasMsg(canvas_msg) => {
                info!("EditJuliaCfg: got msg CanvasMsg");
                match canvas_msg {
                    CanvasMsgRequest::CanvasSelectMsg(coords) =>
                        if ctx.props().edit_mode {
                            // TODO: implement
                            let x_scale = ctx.props().config.x_max.real() - ctx.props().config.x_min.real();
                            let y_scale = ctx.props().config.x_max.imag() - ctx.props().config.x_min.imag();
                            let x_min = ctx.props().config.x_min.real() + x_scale * f64::from(coords.0);
                            let y_min = ctx.props().config.x_min.imag() + y_scale * f64::from(coords.1);
                            let x_max = ctx.props().config.x_max.real() + x_scale * f64::from(coords.2);
                            let y_max = ctx.props().config.x_max.imag() + y_scale * f64::from(coords.3);
                            set_value_on_ref(&self.x_max_real_ref,
                                             "x_max_real",
                                             x_max.to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            set_value_on_ref(&self.x_max_imag_ref,
                                             "x_max_imag",
                                             y_max.to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            set_value_on_ref(&self.x_min_real_ref,
                                             "x_min_real",
                                             x_min.to_string().as_str())
                                .map_or_else(|err| {
                                    error!("{}",err.as_str());
                                }, |v| v);
                            set_value_on_ref(&self.x_min_imag_ref,
                                             "x_min_imag",
                                             y_min.to_string().as_str())
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset_area = ctx.link().callback(|_| Msg::ResetArea);
        let reset_params = ctx.link().callback(|_| Msg::ResetParams);
        let zoom_out = ctx.link().callback(|_| Msg::ZoomOut);
        let save_config = ctx.link().callback(|_| Msg::SaveConfig);
        let cancel = ctx.link().callback(|_| Msg::Cancel);
        let cntr_class = if ctx.props().edit_mode { "edit_cntr_visible" } else { "edit_cntr_hidden" };

        html![
            <div class={cntr_class} id="julia_edit_cntr" ref={self.container_ref.clone()}>
                <div class="input_cntr">
                    <p class="hint_text">
                        {"Hint: You can select a rectangle in the draw area to import the coordiates into the editor."}
                    </p>
                </div>
                <div class="input_cntr">
                    <div class="input_inner">
                        <label class="input_label" for="julia_iterations">
                            {"Iterations"}
                        </label>
                        <input class="input" id="julia_iterations" name="julia_iterations"
                            type="number" min="100" max="1000" ref={self.iter_ref.clone()}/>
                    </div>
                    <div class="input_inner">
                        <label class="input_label" for="julia_c_real">
                            {"C Real"}
                        </label>
                        <input class="input" id="julia_c_real" name="julia_c_real"
                            type="number" step="0.0000001"/>
                    </div>
                    <div class="input_inner">
                        <label class="input_label" for="julia_c_imag">
                            {"C Imag"}
                        </label>
                        <input class="input" id="julia_c_imag" name="julia_c_imag"
                            type="number" step="0.0000001"/>
                    </div>
                    <button class="editor_button" id="julia_reset_params" onclick={reset_params}>
                        {"Reset to Default"}
                    </button>
                </div>
                <div class="input_cntr">
                    <div class="input_inner">
                        <div class="area_cntr">
                            <div class="input_inner">
                                <label class="input_label" for="julia_max_real">
                                    {"Max. Real"}
                                </label>
                                <input class="input" id="julia_max_real" name="julia_max_real"
                                    type="number" step="0.0000001"/>
                            </div>
                            <div class="input_inner">
                                <label class="input_label" for="julia_min_real">
                                    {"Min. Real"}
                                </label>
                                <input class="input" id="julia_min_real" name="julia_min_real"
                                    type="number" step="0.0000001"/>
                            </div>
                        </div>
                        <div class="area_cntr">
                            <div class="input_inner">
                                <label class="input_label" for="julia_max_imag">
                                    {"Max. Imag"}
                                </label>
                                <input class="input" id="julia_max_imag" name="julia_max_imag"
                                    type="number" step="0.0000001"/>
                            </div>
                            <div class="input_inner">
                                <label class="input_label" for="julia_min_imag">
                                    {"Min. Imag"}
                                </label>
                                <input class="input" id="julia_min_imag" name="julia_min_imag"
                                    type="number" step="0.0000001"/>
                            </div>
                        </div>
                        <div class="area_cntr">
                            <button class="editor_button" id="julia_reset_area"
                                    onclick={reset_area}>
                                {"Reset to Default"}
                            </button>
                            <button class="editor_button" id="julia_zoom_out"
                                    onclick={zoom_out}>
                                {"Zoom Out"}
                            </button>
                        </div>
                    </div>
                </div>
                <div class="edit_button_cntr">
                    <button class="editor_button" id="julia_save" onclick={save_config}>
                        {"Save"}
                    </button>
                    <button class="editor_button" id="julia_cancel" onclick={cancel}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        ]
    }
}

#[derive(Properties,PartialEq, Clone)]
pub struct EditJuliaCfgProps {
    pub edit_mode: bool,
    pub config: JuliaSetCfg,
    pub cb_saved: Callback<JuliaSetCfg>,
    pub cb_canceled: Callback<()>,
}
