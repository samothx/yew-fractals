// use wasm_bindgen::prelude::web_sys;
use super::find_escape_radius;
use crate::components::root::Config;
use crate::work::{complex::Complex, fractal::Fractal};
use serde::{Deserialize, Serialize};

pub const JULIA_DEFAULT_X_MAX: (f64, f64) = (1.5, 1.0);
pub const JULIA_DEFAULT_X_MIN: (f64, f64) = (-1.5, -1.0);

pub const JULIA_DEFAULT_C: (f64, f64) = (-0.8, 0.156);
pub const JULIA_DEFAULT_ITERATIONS: u32 = 400;

pub struct JuliaSet {
    c: Complex,
    max: f64,
    iterations: u32,
}

impl JuliaSet {
    pub fn new(config: &Config) -> Self {
        info!(
            "creating fractal with: x_max: {}, x_min: {}, c: {}",
            config.julia_set_cfg.x_max, config.julia_set_cfg.x_min, config.julia_set_cfg.c
        );

        let max = find_escape_radius(config.julia_set_cfg.c.norm());

        Self {
            c: config.julia_set_cfg.c,
            max: max * max,
            iterations: config.julia_set_cfg.max_iterations,
        }
    }
}

impl Fractal for JuliaSet {
    fn get_scale(&self, config: &Config, canvas_width: u32, canvas_height: u32) -> Complex {
        Complex::new(
            (config.julia_set_cfg.x_max.real() - config.julia_set_cfg.x_min.real())
                / f64::from(canvas_width),
            (config.julia_set_cfg.x_max.imag() - config.julia_set_cfg.x_min.imag())
                / f64::from(canvas_height),
        )
    }

    fn get_offset(&self, config: &Config) -> Complex {
        config.julia_set_cfg.x_min.clone()
    }

    fn iterate(&self, x: &Complex) -> u32 {
        let mut curr = *x;
        // log!(format!("iterate: start: {}", curr));
        let mut last: Option<u32> = None;
        for idx in 1..=self.iterations {
            curr = curr * curr + self.c;
            if curr.square_length() >= self.max {
                last = Some(idx);
                break;
            }
        }

        // log!(format!("iterate: end:  {} norm: {} last: {:?}", curr, curr.square_length(), last));
        last.unwrap_or(self.iterations + 1)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct JuliaSetCfg {
    pub max_iterations: u32,
    pub x_max: Complex,
    pub x_min: Complex,
    pub c: Complex,
    pub color_cfg_name: Option<String>,
}

impl Default for JuliaSetCfg {
    fn default() -> Self {
        Self {
            max_iterations: JULIA_DEFAULT_ITERATIONS,
            x_max: Complex::new(JULIA_DEFAULT_X_MAX.0, JULIA_DEFAULT_X_MAX.1),
            x_min: Complex::new(JULIA_DEFAULT_X_MIN.0, JULIA_DEFAULT_X_MIN.1),
            c: Complex::new(JULIA_DEFAULT_C.0, JULIA_DEFAULT_C.1),
            color_cfg_name: None,
        }
    }
}
