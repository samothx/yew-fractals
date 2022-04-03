use yew::prelude::*;
use web_sys::window;

use serde::{Serialize, Deserialize};

use crate::work::complex::Complex;
use super::{disclaimer::Disclaimer, control_panel::ControlPanel, canvas_element::CanvasElement,
            edit_julia_cfg::EditJuliaCfg,
            edit_mandelbrot_cfg::EditMandelbrotCfg,
            control_panel::PanelConfig::{ConfigJuliaSet, ConfigMandelbrot}};


pub const JULIA_DEFAULT_X_MAX: (f64, f64) = (1.5, 1.0);
pub const JULIA_DEFAULT_X_MIN: (f64, f64) = (-1.5, -1.0);

pub const JULIA_DEFAULT_C: (f64, f64) = (-0.8, 0.156);
pub const JULIA_DEFAULT_ITERATIONS: u32 = 400;

pub const MANDELBROT_DEFAULT_C_MAX: (f64, f64) = (0.47, 1.12);
pub const MANDELBROT_DEFAULT_C_MIN: (f64, f64) = (-2.00, -1.12);
pub const MANDELBROT_DEFAULT_ITERATIONS: u32 = 400;

const STORAGE_KEY: &str = "yew_fractals_v2.1";
const DEBUG_NO_STORAGE: bool = true;

pub const DEFAULT_WIDTH: u32 = 1024;

// TODO: make canvas its own component and setup communication with editors

pub struct Root {
    config: Config,
    edit_mode: bool,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            config: Config::default(),
            edit_mode: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::JuliaSetCfgChanged(config) => {
                self.edit_mode = false;
                self.config.julia_set_cfg = config;
                self.config.store();
                true
            },
            Msg::MandelbrotCfgChanged(config) => {
                self.edit_mode = false;
                self.config.mandelbrot_cfg = config;
                self.config.store();
                true
            },
            Msg::EditCfgCanceled => {
                self.edit_mode = false;
                true
            },
            Msg::TypeChanged(fractal_type) => {
                self.config.active_config = fractal_type;
                self.config.store();
                true
            },
            Msg::ViewStatsChanged(status) => {
                info!("Root::update: ViewStatsChanged: {}", status);
                self.config.view_stats = status;
                self.config.store();
                true
            },
            Msg::EditConfig=> {
                self.edit_mode = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ctrl_panel_cfg = match self.config.active_config {
            FractalType::JuliaSet => ConfigJuliaSet(self.config.julia_set_cfg.clone()),
            FractalType::Mandelbrot => ConfigMandelbrot(self.config.mandelbrot_cfg.clone())
        };

        html! {
            <div class="outer_cntr"> 
                <h1>{if self.config.active_config == FractalType::Mandelbrot {"Mandelbrot Set"} else {"Julia Set"}}</h1>
                <Disclaimer/>
                <div class="inner_cntr">
                    <ControlPanel
                        config={ctrl_panel_cfg}
                        view_stats={self.config.view_stats}
                        on_type_changed={ctx.link().callback(|fractal_type: FractalType| Msg::TypeChanged(fractal_type))}
                        on_edit={ctx.link().callback(|_| Msg::EditConfig)}
                        on_view_stats_changed={ctx.link().callback(|value: bool| Msg::ViewStatsChanged(value))}
                        edit_mode={self.edit_mode}
                    />
                    <div class="fractal_container">
                        <EditJuliaCfg edit_mode={self.edit_mode && self.config.active_config == FractalType::JuliaSet}
                                        config={self.config.julia_set_cfg.clone()}
                                        canvas_width={self.config.canvas_width}
                                        canvas_height={self.config.canvas_height}
                                        cb_saved={ctx.link().callback(|config: JuliaSetCfg| Msg::JuliaSetCfgChanged(config))}
                                        cb_canceled={ctx.link().callback(|_| Msg::EditCfgCanceled)}
                        />
                        <EditMandelbrotCfg edit_mode={self.edit_mode && self.config.active_config == FractalType::Mandelbrot}
                                        config={self.config.mandelbrot_cfg.clone()}
                                        canvas_width={self.config.canvas_width}
                                        canvas_height={self.config.canvas_height}
                                        cb_saved={ctx.link().callback(|config: MandelbrotCfg| Msg::MandelbrotCfgChanged(config))}
                                        cb_canceled={ctx.link().callback(|_| Msg::EditCfgCanceled)}
                        />
                        <CanvasElement
                            config={self.config.clone()}
                            edit_mode={self.edit_mode}
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
    EditConfig,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub view_stats: bool,
    pub active_config: FractalType,
    pub julia_set_cfg: JuliaSetCfg,
    pub mandelbrot_cfg: MandelbrotCfg,
    pub canvas_width: u32,
    pub canvas_height: u32,
}

impl Default for Config {
    fn default() -> Self {
        if DEBUG_NO_STORAGE {
            Self::std_cfg()
        } else {
            match window().expect("window no found")
                .local_storage().expect("error retrieving storage").expect("no storage available")
                .get(STORAGE_KEY).expect("error retrieving key from storage") {
                Some(config_str) => serde_json::from_str(config_str.as_str()).expect("Deserialization of config failed"),
                None => {
                    Self::std_cfg()
                }
            }
        }
    }
}

impl Config {
    pub fn store(&self) {
        if DEBUG_NO_STORAGE {} else {
            let config_str = serde_json::to_string(self).expect("Serialization of config failed");
            window().expect("window no found")
                .local_storage().expect("error retrieving storage").expect("no storage available")
                .set(STORAGE_KEY, config_str.as_str()).expect("error writing key to storage");
        }
    }

    fn std_cfg() -> Self {
        let def_config = MandelbrotCfg::default();
        let height = (f64::from(DEFAULT_WIDTH) *
            (def_config.c_max.imag() - def_config.c_min.imag()) /
            (def_config.c_max.real() - def_config.c_min.real())) as u32;
        Self {
            view_stats: false,
            active_config: FractalType::Mandelbrot,
            julia_set_cfg: JuliaSetCfg::default(),
            mandelbrot_cfg: def_config,
            canvas_width: DEFAULT_WIDTH,
            canvas_height: height,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct JuliaSetCfg {
    pub max_iterations: u32,
    pub x_max: Complex,
    pub x_min: Complex,
    pub c: Complex,
}

impl Default for JuliaSetCfg {
    fn default() -> Self {
        Self {
            max_iterations: JULIA_DEFAULT_ITERATIONS,
            x_max: Complex::new(JULIA_DEFAULT_X_MAX.0, JULIA_DEFAULT_X_MAX.1),
            x_min: Complex::new(JULIA_DEFAULT_X_MIN.0, JULIA_DEFAULT_X_MIN.1),
            c: Complex::new(JULIA_DEFAULT_C.0, JULIA_DEFAULT_C.1),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MandelbrotCfg {
    pub max_iterations: u32,
    pub c_max: Complex,
    pub c_min: Complex,
}

impl Default for MandelbrotCfg {
    fn default() -> Self {
        Self {
            max_iterations: MANDELBROT_DEFAULT_ITERATIONS,
            c_max: Complex::new(MANDELBROT_DEFAULT_C_MAX.0, MANDELBROT_DEFAULT_C_MAX.1),
            c_min: Complex::new(MANDELBROT_DEFAULT_C_MIN.0, MANDELBROT_DEFAULT_C_MIN.1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum FractalType {
    Mandelbrot,
    JuliaSet,
}

