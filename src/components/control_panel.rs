use yew::prelude::*;

use yew_agent::{Dispatcher, Dispatched, Bridge, Bridged};

use web_sys::{HtmlSelectElement, HtmlInputElement};


use crate::{agents::{canvas_msg_bus::{CanvasMsgRequest, CanvasSelectMsgBus},
                     command_msg_bus::{CommandMsgBus, CommandRequest}},
            work::util::set_value_on_txt_area_ref,
            components::root::{JuliaSetCfg, MandelbrotCfg, FractalType},

};

pub struct ControlPanel {
    event_bus: Option<Dispatcher<CommandMsgBus>>,
    paused: bool,
    type_sel_ref: NodeRef,
    view_stats_cb_ref: NodeRef,
    view_stats_txt_ref: NodeRef,
    _producer: Box<dyn Bridge<CanvasSelectMsgBus>>,
}

impl Component for ControlPanel {
    type Message = Msg;
    type Properties = ControlPanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        ControlPanel {
            event_bus: None,
            paused: true,
            type_sel_ref: NodeRef::default(),
            view_stats_cb_ref: NodeRef::default(),
            view_stats_txt_ref: NodeRef::default(),
            _producer: CanvasSelectMsgBus::bridge(ctx.link().callback(Msg::CanvasMsg)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                info!("ControlPanel::Start");
                if self.paused && !ctx.props().edit_mode {
                    self.event_bus.as_mut().expect("Eventbus not initialized")
                        .send(CommandRequest::Start);
                }
                true
            }
            Msg::Stop => {
                info!("ControlPanel::Stop");
                if !self.paused {
                    self.event_bus.as_mut().expect("Eventbus not initialized")
                        .send(CommandRequest::Stop);
                }
                true
            }
            Msg::Clear => {
                info!("ControlPanel::Clear");
                if !self.paused {
                    self.paused = true;
                    self.event_bus.as_mut().expect("Eventbus not initialized")
                        .send(CommandRequest::Stop);
                }
                self.event_bus.as_mut().expect("Eventbus not initialized")
                    .send(CommandRequest::Clear);
                true
            }
            Msg::Edit => {
                info!("ControlPanel::Edit");
                if !ctx.props().edit_mode {
                    ctx.props().on_edit.emit(());
                }
                true
            }
            Msg::Copy => {
                info!("ControlPanel::Copy");
                self.event_bus.as_mut().expect("CommandMsgBus not initialized").send(CommandRequest::Copy);
                false
            }
            Msg::TypeChanged => {
                info!("ControlPanel::TypeChanged");
                let fractal_type = match self.type_sel_ref.cast::<HtmlSelectElement>()
                    .expect("Type select not found")
                    .value().as_str() {
                    "type_mandelbrot" => Some(FractalType::Mandelbrot),
                    "type_julia_set" => Some(FractalType::JuliaSet),
                    val => {
                        error!("invalid fractal type '{}'", val);
                        None
                    }
                };

                if let Some(fractal_type) = fractal_type {
                    self.event_bus.as_mut().expect("Eventbus not initialized").send(CommandRequest::Clear);
                    ctx.props().on_type_changed.emit(fractal_type)
                }
                true
            },
            Msg::ViewStatsChanged => {
                info!("ControlPanel::ViewStatsChanged");
                let checked = self.view_stats_cb_ref.cast::<HtmlInputElement>()
                    .expect("Type select not found")
                    .checked();
                ctx.props().on_view_stats_changed.emit(checked);
                true
            }
            Msg::CanvasMsg(canvas_msg) => {
                // TODO: implement
                match canvas_msg {
                    CanvasMsgRequest::FractalStarted => {
                        self.paused = false;
                        true
                    }
                    CanvasMsgRequest::FractalPaused => {
                        self.paused = true;
                        true
                    }
                    CanvasMsgRequest::FractalProgress(msg) => {
                        set_value_on_txt_area_ref(&self.view_stats_txt_ref,
                                               "stats_txt",
                                               msg.as_str())
                            .map_or_else(|err| {
                                error!("{}",err.as_str());
                            }, |v| v);
                        false
                    }
                    _ => false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sel_type = match ctx.props().config {
            PanelConfig::ConfigMandelbrot(_) => "type_mandelbrot",
            PanelConfig::ConfigJuliaSet(_) => "type_julia_set",
        };

        info!("ControlPanel::view initial type {}", sel_type);
        let on_start = ctx.link().callback(|_| Msg::Start);
        let on_stop = ctx.link().callback(|_| Msg::Stop);
        let on_edit = ctx.link().callback(|_| Msg::Edit);
        let on_clear = ctx.link().callback(|_| Msg::Clear);
        let on_copy = ctx.link().callback(|_| Msg::Copy);
        let on_type_changed = ctx.link().callback(|_| Msg::TypeChanged);
        let on_view_stats_changed = ctx.link().callback(|_| Msg::ViewStatsChanged);


        html![
            <div class="button_cntr">
                <button class="menu_button" id="start" onclick={on_start}
                        disabled={ !self.paused || ctx.props().edit_mode }>
                    {"Start"}
                </button>
                <button class="menu_button" id="stop" onclick={on_stop}
                        disabled={ self.paused }>
                    {"Stop"}
                </button>
                <button class="menu_button" id="clear" onclick={on_clear}
                        disabled={ !self.paused || ctx.props().edit_mode }>
                    {"Clear"}
                </button>
                <button class="menu_button" id="edit" onclick={on_edit}
                        disabled={ !self.paused || ctx.props().edit_mode }>
                    {"Edit"}
                </button>
                <button class="menu_button" id="copy" onclick={on_copy}
                        disabled={ !self.paused || ctx.props().edit_mode }>
                    {"Copy"}
                </button>
                <label class="type_select_label" for="type_select">
                    {"Select Type"}
                </label>
                <select class="type_select" id="type_select" name="type_select" value={sel_type}
                    disabled={!self.paused || ctx.props().edit_mode } onchange={on_type_changed}
                    ref={self.type_sel_ref.clone()}
                    >
                    <option value="type_mandelbrot" selected={sel_type=="type_mandelbrot"}>{"Mandelbrot Set"}</option>
                    <option value="type_julia_set" selected={sel_type=="type_julia_set"}>{"Julia Set"}</option>
                </select>
                <div class="cb_stats_cntr">
                    <label class="type_select_label" for="stats_cb">
                        {"View Stats"}
                    </label>
                    <input class="stats_cb" id="stats_cb" name="stats_cb" type="checkbox"
                        disabled={!self.paused} checked={ctx.props().view_stats}
                        onchange={on_view_stats_changed}
                        ref={self.view_stats_cb_ref.clone()}
                    />
                </div>
                <div class={ if ctx.props().view_stats {"stats_cntr_visible"} else {"stats_cntr_hidden"}}>
                    <textarea class="stats_text" readonly=true rows="6" placeholder="No Stats yet"
                        ref={self.view_stats_txt_ref.clone()} >
                        {model.stats_text.as_str()}
                    </textarea>
                </div>
            </div>
        ]
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render && self.event_bus.is_none() {
            self.event_bus = Some(CommandMsgBus::dispatcher());
        }
    }
}

#[allow(clippy::enum_variant_names)]
pub enum Msg {
    Start,
    Stop,
    Clear,
    Edit,
    Copy,
    TypeChanged,
    ViewStatsChanged,
    CanvasMsg(CanvasMsgRequest),
}

#[derive(Properties, PartialEq, Clone)]
pub struct ControlPanelProps {
    pub config: PanelConfig,
    pub view_stats: bool,
    pub edit_mode: bool,
    pub on_type_changed: Callback<FractalType>,
    pub on_edit: Callback<()>,
    pub on_view_stats_changed: Callback<bool>,
}

#[derive(PartialEq, Clone)]
pub enum PanelConfig {
    ConfigJuliaSet(JuliaSetCfg),
    ConfigMandelbrot(MandelbrotCfg),
}

