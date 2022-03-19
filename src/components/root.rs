use yew::prelude::*;
use web_sys::window;

use serde::{Serialize, Deserialize};

use crate::work::complex::Complex;
use super::disclaimer::Disclaimer;
use super::edit_julia_cfg::EditJuliaCfg;


pub const JULIA_DEFAULT_X_MAX: (f64, f64) = (1.5, 1.0);
pub const JULIA_DEFAULT_X_MIN: (f64, f64) = (-1.5, -1.0);

pub const JULIA_DEFAULT_C: (f64, f64) = (-0.8, 0.156);
pub const JULIA_DEFAULT_ITERATIONS: u32 = 400;

pub const MANDELBROT_DEFAULT_C_MAX: (f64, f64) = (0.47, 1.12);
pub const MANDELBROT_DEFAULT_C_MIN: (f64, f64) = (-2.00, -1.12);
pub const MANDELBROT_DEFAULT_ITERATIONS: u32 = 400;

const STORAGE_KEY: &str = "yew_fractals_v1";

pub enum Msg {
    JuliaSetCfgChanged(JuliaSetCfg),
    JuliaSetCfgCanceled
}

pub struct Root {
    config: Config,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            config: Config::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        /* let julia_props = EditJuliaCfgProps{
           config: self.config.julia_set_cfg.clone(),
           callback:
        };*/
        // let onclick = ctx.link().callback(|config: u32| Msg::TestMsg(config));
        html! {
            <div class="outer_cntr"> 
                <h1>{if self.config.active_config == FractalType::Mandelbrot {"Mandelbrot Set"} else {"Julia Set"}}</h1>
                <Disclaimer/>
                <EditJuliaCfg config={self.config.julia_set_cfg.clone()}
                              cb_saved={ctx.link().callback(|config: JuliaSetCfg| Msg::JuliaSetCfgChanged(config))}
                              cb_canceled={ctx.link().callback(|_| Msg::JuliaSetCfgCanceled)}
                ></EditJuliaCfg>
                <div class="inner_cntr">
                </div>
            </div>
        }
    }
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

#[derive(Serialize, Deserialize,PartialEq,Clone)]
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

#[derive(Serialize, Deserialize,PartialEq)]
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
enum FractalType {
    Mandelbrot,
    JuliaSet,
}
