use yew::prelude::*;
use web_sys::window;

use serde::{Serialize, Deserialize};

use crate::work::complex::Complex;
use super::{disclaimer::Disclaimer, control_panel::ControlPanel, canvas_element::CanvasElement,
            edit_julia_cfg::EditJuliaCfg,
            control_panel::PanelConfig::{ConfigJuliaSet, ConfigMandelbrot}};


pub const JULIA_DEFAULT_X_MAX: (f64, f64) = (1.5, 1.0);
pub const JULIA_DEFAULT_X_MIN: (f64, f64) = (-1.5, -1.0);

pub const JULIA_DEFAULT_C: (f64, f64) = (-0.8, 0.156);
pub const JULIA_DEFAULT_ITERATIONS: u32 = 400;

pub const MANDELBROT_DEFAULT_C_MAX: (f64, f64) = (0.47, 1.12);
pub const MANDELBROT_DEFAULT_C_MIN: (f64, f64) = (-2.00, -1.12);
pub const MANDELBROT_DEFAULT_ITERATIONS: u32 = 400;

const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 800;


const STORAGE_KEY: &str = "yew_fractals_v1";


// TODO: make canvas its own component and setup communication with editors

pub struct Root {
    config: Config,
    canvas_width: u32,
    canvas_height: u32,
    edit_mode: bool
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            config: Config::default(),
            canvas_width: DEFAULT_WIDTH,
            canvas_height: DEFAULT_HEIGHT,
            edit_mode: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
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
                    />
                    <div class="fractal_container">
                        <EditJuliaCfg config={self.config.julia_set_cfg.clone()}
                                      cb_saved={ctx.link().callback(|config: JuliaSetCfg| Msg::JuliaSetCfgChanged(config))}
                                      cb_canceled={ctx.link().callback(|_| Msg::JuliaSetCfgCanceled)}
                        />
                        <CanvasElement
                            width={self.canvas_width}
                            height={self.canvas_height}
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
    JuliaSetCfgCanceled,
    TypeChanged(FractalType),
    ViewStatsChanged(bool),
    EditConfig,
}

#[derive(Serialize, Deserialize)]
struct Config {
    pub view_stats: bool,
    pub active_config: FractalType,
    pub julia_set_cfg: JuliaSetCfg,
    pub mandelbrot_cfg: MandelbrotCfg,
}

impl Default for Config {
    fn default() -> Self {
        match window().expect("window no found")
            .local_storage().expect("error retrieving storage").expect("no storage available")
            .get(STORAGE_KEY).expect("error retrieving key from storage") {
            Some(config_str) => serde_json::from_str(config_str.as_str()).expect("Deserialization of cofig failed"),
            None => Self {
                view_stats: false,
                active_config: FractalType::Mandelbrot,
                julia_set_cfg: JuliaSetCfg::default(),
                mandelbrot_cfg: MandelbrotCfg::default(),
            }
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
            x_min: Complex::new(-JULIA_DEFAULT_X_MIN.0, -JULIA_DEFAULT_X_MIN.1),
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

