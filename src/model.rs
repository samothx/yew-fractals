use web_sys::{ImageData, window};
use serde::{Serialize, Deserialize};

use super::work::{complex::Complex, fractal::Fractal, stats::Stats};

const JULIA_DEFAULT_X: (f64, f64) = (1.5, 1.0);
const JULIA_DEFAULT_C: (f64, f64) = (-0.8, 0.156);
const JULIA_DEFAULT_ITERATIONS: u32 = 400;

const MANDELBROT_DEFAULT_C_MAX: (f64, f64) = (0.47, 1.12);
const MANDELBROT_DEFAULT_C_MIN: (f64, f64) = (-2.00, -1.12);
const MANDELBROT_DEFAULT_ITERATIONS: u32 = 400;

const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 800;

const ENTER_KEY: &str = "Enter";
const BACKGROUND_COLOR: &str = "#000000";
const STORAGE_KEY: &str = "seed_fractals_v1";




pub struct Model {
    width: u32,
    height: u32,
    config: Config,
    background_color: String,
    // canvas: Option<Canvas>,
    fractal: Option<Box<dyn Fractal>>,
    mouse_drag: Option<MouseDrag>,
    paused: bool,
    edit_mode: bool,
    stats_text: String,
    stats: Option<Stats>
}

impl Default for Model {
    fn default() -> Self {
        let config = match window().expect("window no found")
            .local_storage().expect("error retrieving storage").expect("no storage available")
            .get(STORAGE_KEY).expect("error retrieving key from storage") {
                Some(config_str) => serde_json::from_str(config_str.as_str()).expect("Deserialization of cofig failed"),
                None => Config::default()
            };


        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            config,
            background_color: BACKGROUND_COLOR.to_string(),
            // canvas: None,
            fractal: None,
            mouse_drag: None,
            paused: true,
            edit_mode: false,
            stats_text: "".to_string(),
            stats: None    
        }
    }
}


#[derive(Serialize, Deserialize)]
struct Config {
    view_stats: bool,
    active_config: FractalType,
    julia_set_cfg: JuliaSetCfg,
    mandelbrot_cfg: MandelbrotCfg,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            view_stats: false,
            active_config: FractalType::Mandelbrot,
            julia_set_cfg: JuliaSetCfg::default(),
            mandelbrot_cfg: MandelbrotCfg::default()
        }
    }
}


struct MouseDrag {
    start: (u32, u32),
    curr: (u32, u32),
    image_data: Option<ImageData>,
}

