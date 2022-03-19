use yew::prelude::*;
use crate::components::root::{JuliaSetCfg, MandelbrotCfg, FractalType};

pub struct ControlPanel {
    paused: bool,
    edit_mode: bool,

}

impl Component for ControlPanel {
    type Message = Msg;
    type Properties = ControlPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ControlPanel{
            paused: true,
            edit_mode: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // TODO: implement
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sel_type = match ctx.props().config {
            PanelConfig::ConfigMandelbrot(_) => "Mandelbrot",
            PanelConfig::ConfigJuliaSet(_) => "Julia Set",
        };

        let on_start = ctx.link().callback(|_| Msg::Start);
        let on_stop = ctx.link().callback(|_| Msg::Stop);
        let on_edit = ctx.link().callback(|_| Msg::Edit);
        let on_clear = ctx.link().callback(|_| Msg::Clear);
        let on_type_changed = ctx.link().callback(|_| Msg::TypeChanged);
        let on_view_stats_changed = ctx.link().callback(|_| Msg::ViewStatsChanged);


        html![
            <div class="button_cntr">
                <button class="menu_button" id="start" onclick={on_start}
                        disabled={ !self.paused }>
                    {"Start"}
                </button>
                <button class="menu_button" id="stop" onclick={on_stop}
                        disabled={ self.paused }>
                    {"Stop"}
                </button>
                <button class="menu_button" id="clear" onclick={on_clear}
                        disabled={ !self.paused }>
                    {"Start"}
                </button>
                <button class="menu_button" id="edit" onclick={on_edit}
                        disabled={ !self.paused }>
                    {"Start"}
                </button>
                <label class="type_select_label" for="type_select">
                    {"Select Type"}
                </label>
                <select class="type_select" id="type_select" name="type_select" value={sel_type}
                    disabled={!self.paused || self.edit_mode } onchange={on_type_changed}>
                    <option value="type_mandelbrot">{"Mandelbrot Set"}</option>
                    <option value="type_julia_set">{"Julia Set"}</option>
                </select>
                <div class="cb_stats_cntr">
                    <label class="type_select_label" for="stats_cb">
                        {"View Stats"}
                    </label>
                    <input class="stats_cb" id="stats_cb" name="stats_cb" type="checkbox"
                        disabled={!self.paused} checked={ctx.props().view_stats}
                        onchange={on_view_stats_changed}
                    />
                </div>
                <div class={ if ctx.props().view_stats {"stats_cntr_visible"} else {"stats_cntr_hidden"}}>
                    <textarea class="stats_text" readonly=true rows="6" placeholder="No Stats yet">
                        {model.stats_text.as_str()}
                    </textarea>
                </div>
            </div>
        ]
    }
}

pub enum Msg {
    Start,
    Stop,
    Clear,
    Edit,
    TypeChanged,
    ViewStatsChanged
}

#[derive(Properties,PartialEq, Clone)]
pub struct ControlPanelProps {
    pub config: PanelConfig,
    pub view_stats: bool,
    pub on_type_changed: Callback<FractalType>,
    pub on_edit: Callback<()>,
    pub on_view_stats_changed: Callback<bool>,
}

#[derive(PartialEq, Clone)]
pub enum PanelConfig {
    ConfigJuliaSet(JuliaSetCfg),
    ConfigMandelbrot(MandelbrotCfg)
}
