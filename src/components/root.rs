use web_sys::window;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

use super::{
    canvas_element::CanvasElement,
    control_panel::ControlPanel,
    control_panel::PanelConfig::{ConfigJuliaSet, ConfigMandelbrot},
    edit_color_cfg::EditColorConfig,
    edit_julia_cfg::EditJuliaCfg,
    edit_mandelbrot_cfg::EditMandelbrotCfg,
    modal::{ModalMsg, ModalOk, ModalPlain},
};
use crate::agents::clipboard_worker::WorkerStatus;
use crate::components::edit_color_cfg::ColorCfg;
use crate::work::fractal::{FractalType, JuliaSetCfg, MandelbrotCfg};

const STORAGE_KEY: &str = "yew_fractals_v2.5";
const DEBUG_NO_STORAGE: bool = true;

pub const DEFAULT_WIDTH: u32 = 1024;

// TODO: make canvas its own component and setup communication with editors

pub struct Root {
    config: Config,
    edit_mode: bool,
    color_edit_mode: bool,
    canvas_height: u32,
    show_ctc_preparing: bool,
    show_ctc_done: bool,
    show_disclaimer: bool,
    ctc_done_msg: String,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let show_disclaimer = window()
            .expect("Window not found")
            .match_media("(max-width: 600px)")
            .expect("Failed to query media")
            .expect("No media query result")
            .matches();

        // info!("Root::create: media query result: {}", media_match);
        let config = Config::default();
        let canvas_height = config.get_canvas_height(DEFAULT_WIDTH);
        Self {
            config,
            edit_mode: false,
            color_edit_mode: false,
            canvas_height,
            show_ctc_preparing: false,
            show_ctc_done: false,
            show_disclaimer,
            ctc_done_msg: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::JuliaSetCfgChanged(config) => {
                self.edit_mode = false;
                self.config.julia_set_cfg = config;
                self.canvas_height = self.config.get_canvas_height(DEFAULT_WIDTH);
                self.config.store();
                true
            }
            Msg::MandelbrotCfgChanged(config) => {
                self.edit_mode = false;
                self.config.mandelbrot_cfg = config;
                self.canvas_height = self.config.get_canvas_height(DEFAULT_WIDTH);
                self.config.store();
                true
            }
            Msg::EditCfgCanceled => {
                self.edit_mode = false;
                true
            }
            Msg::TypeChanged(fractal_type) => {
                self.config.active_config = fractal_type;
                self.canvas_height = self.config.get_canvas_height(DEFAULT_WIDTH);
                self.config.store();
                true
            }
            Msg::ViewStatsChanged(status) => {
                info!("Root::update: ViewStatsChanged: {}", status);
                self.config.view_stats = status;
                self.config.store();
                true
            }
            Msg::EditConfig => {
                self.edit_mode = true;
                true
            }
            Msg::CtcActive(status) => {
                info!("Root::update: CtcActive");
                self.show_ctc_preparing = status;
                true
            }
            Msg::CtcDone(output) => {
                info!("Root::update: CtcDone: {:?}", output);
                self.show_ctc_preparing = false;
                self.show_ctc_done = true;
                self.ctc_done_msg = match output {
                    WorkerStatus::Complete => {
                        "The image was copied to the clipboard succesfully.".to_owned()
                    }
                    WorkerStatus::Failure(msg) => {
                        format!("Could not copy image to clipboard, error: {}.", msg)
                    }
                    _ => panic!("Invalid WorkerStatus encountered"),
                };
                true
            }
            Msg::CtcModalOk => {
                self.show_ctc_preparing = false;
                self.show_ctc_done = false;
                true
            }
            Msg::DisclaimerOk => {
                self.show_disclaimer = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ctrl_panel_cfg = match self.config.active_config {
            FractalType::JuliaSet => ConfigJuliaSet(self.config.julia_set_cfg.clone()),
            FractalType::Mandelbrot => ConfigMandelbrot(self.config.mandelbrot_cfg.clone()),
        };

        let disclaimer_msg = vec!["\
By design calculating and displaying fractals requires ample processing power and a good screen resolution.".to_owned(),
            "\
So far I have not gotten around to creating alternative layouts and solutions for small screens, so this page
is best viewed on a computer.".to_owned()
        ];

        html! {
            <div class="outer_cntr">
                <h1>{if self.config.active_config == FractalType::Mandelbrot {"Mandelbrot Set"} else {"Julia Set"}}</h1>
                <div class="inner_cntr">
                    <ControlPanel
                        config={ctrl_panel_cfg}
                        view_stats={self.config.view_stats}
                        on_type_changed={ctx.link().callback(Msg::TypeChanged)}
                        on_edit={ctx.link().callback(|_| Msg::EditConfig)}
                        on_view_stats_changed={ctx.link().callback(Msg::ViewStatsChanged)}
                        on_ctc_active={ctx.link().callback(Msg::CtcActive)}
                        on_ctc_done={ctx.link().callback(Msg::CtcDone)}
                        edit_mode={self.edit_mode}
                    />
                    <div class="fractal_container">
                        <ModalPlain
                            visible={self.show_ctc_preparing}
                            title={"Copy to Clipboard".to_owned()}
                            message={ ModalMsg::String("Preparing image to be copied to clipboard.".to_owned()) }
                        />
                        <ModalOk
                            visible={self.show_ctc_done}
                            title={"Copy to Clipboard".to_owned()}
                            message={ ModalMsg::StringList(vec![self.ctc_done_msg.clone()]) }
                            on_ok={ctx.link().callback(|_| Msg::CtcModalOk)}
                        />
                        <ModalOk
                            visible={self.show_disclaimer}
                            title={"Sorry - this page is currently not yet mobile friendly".to_owned()}
                            message={ ModalMsg::StringList(disclaimer_msg.clone()) }
                            on_ok={ctx.link().callback(|_| Msg::DisclaimerOk)}
                            background_color={Some("salmon")}
                        />
                        <EditColorConfig
                            config={self.config.color_cfg.clone()}
                            edit_mode={self.color_edit_mode}
                        />
                        <EditJuliaCfg edit_mode={self.edit_mode && self.config.active_config == FractalType::JuliaSet}
                                        config={self.config.julia_set_cfg.clone()}
                                        canvas_width={DEFAULT_WIDTH}
                                        canvas_height={self.canvas_height}
                                        cb_saved={ctx.link().callback(Msg::JuliaSetCfgChanged)}
                                        cb_canceled={ctx.link().callback(|_| Msg::EditCfgCanceled)}
                        />
                        <EditMandelbrotCfg edit_mode={self.edit_mode && self.config.active_config == FractalType::Mandelbrot}
                                        config={self.config.mandelbrot_cfg.clone()}
                                        canvas_width={DEFAULT_WIDTH}
                                        canvas_height={self.canvas_height}
                                        cb_saved={ctx.link().callback(Msg::MandelbrotCfgChanged)}
                                        cb_canceled={ctx.link().callback(|_| Msg::EditCfgCanceled)}
                        />
                        <CanvasElement
                            config={self.config.clone()}
                            edit_mode={self.edit_mode}
                            canvas_width={DEFAULT_WIDTH}
                            canvas_height={self.canvas_height}
                        />
                    </div>
                </div>
            </div>
        }
    }
}

pub enum Msg {
    JuliaSetCfgChanged(JuliaSetCfg),
    MandelbrotCfgChanged(MandelbrotCfg),
    EditCfgCanceled,
    TypeChanged(FractalType),
    ViewStatsChanged(bool),
    CtcActive(bool),
    CtcDone(WorkerStatus),
    CtcModalOk,
    DisclaimerOk,
    EditConfig,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub view_stats: bool,
    pub color_cfg: ColorCfg,
    pub active_config: FractalType,
    pub julia_set_cfg: JuliaSetCfg,
    pub mandelbrot_cfg: MandelbrotCfg,
}

impl Default for Config {
    fn default() -> Self {
        if DEBUG_NO_STORAGE {
            Self::std_cfg()
        } else {
            match window()
                .expect("window no found")
                .local_storage()
                .expect("error retrieving storage")
                .expect("no storage available")
                .get(STORAGE_KEY)
                .expect("error retrieving key from storage")
            {
                Some(config_str) => serde_json::from_str(config_str.as_str())
                    .expect("Deserialization of config failed"),
                None => Self::std_cfg(),
            }
        }
    }
}

impl Config {
    pub fn store(&self) {
        if DEBUG_NO_STORAGE {
        } else {
            let config_str = serde_json::to_string(self).expect("Serialization of config failed");
            window()
                .expect("window no found")
                .local_storage()
                .expect("error retrieving storage")
                .expect("no storage available")
                .set(STORAGE_KEY, config_str.as_str())
                .expect("error writing key to storage");
        }
    }

    fn std_cfg() -> Self {
        Self {
            view_stats: false,
            color_cfg: ColorCfg::default(),
            active_config: FractalType::Mandelbrot,
            julia_set_cfg: JuliaSetCfg::default(),
            mandelbrot_cfg: MandelbrotCfg::default(),
        }
    }

    pub fn get_canvas_height(&self, canvas_width: u32) -> u32 {
        match self.active_config {
            FractalType::Mandelbrot => {
                (f64::from(canvas_width)
                    * (self.mandelbrot_cfg.c_max.imag() - self.mandelbrot_cfg.c_min.imag())
                    / (self.mandelbrot_cfg.c_max.real() - self.mandelbrot_cfg.c_min.real()))
                    as u32
            }
            FractalType::JuliaSet => {
                (f64::from(canvas_width)
                    * (self.julia_set_cfg.x_max.imag() - self.julia_set_cfg.x_min.imag())
                    / (self.julia_set_cfg.x_max.real() - self.julia_set_cfg.x_min.real()))
                    as u32
            }
        }
    }
}
