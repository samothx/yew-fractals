use crate::components::root::Config;
use serde::{Deserialize, Serialize};

use crate::work::{complex::Complex, fractal::Fractal};

pub const MANDELBROT_DEFAULT_C_MAX: (f64, f64) = (0.47, 1.12);
pub const MANDELBROT_DEFAULT_C_MIN: (f64, f64) = (-2.00, -1.12);
pub const MANDELBROT_DEFAULT_ITERATIONS: u32 = 400;

pub struct Mandelbrot {
    iterations: u32,
    power: u32,
}

impl Mandelbrot {
    pub fn new(config: &Config) -> Self {
        info!(
            "creating fractal with: x_max: {}, x_min: {}",
            config.mandelbrot_cfg.c_max, config.mandelbrot_cfg.c_min,
        );

        Self {
            iterations: config.mandelbrot_cfg.max_iterations,
            power: config.mandelbrot_cfg.power,
        }
    }
}

impl Fractal for Mandelbrot {
    fn iterate(&self, c: &Complex) -> u32 {
        let max = 4.0;
        let mut x = Complex::new(0.0, 0.0);
        // log!(format!("iterate: start: {}", curr));
        let mut last: Option<u32> = None;
        for idx in 1..=self.iterations {
            x = x.powi(self.power) + *c;
            if x.square_length() >= max {
                last = Some(idx);
                break;
            }
        }

        // log!(format!("iterate: end:  {} norm: {} last: {:?}", curr, curr.square_length(), last));
        last.unwrap_or(self.iterations + 1)
    }

    fn get_scale(&self, config: &Config, canvas_width: u32, canvas_height: u32) -> Complex {
        Complex::new(
            (config.mandelbrot_cfg.c_max.real() - config.mandelbrot_cfg.c_min.real())
                / f64::from(canvas_width),
            (config.mandelbrot_cfg.c_max.imag() - config.mandelbrot_cfg.c_min.imag())
                / f64::from(canvas_height),
        )
    }

    fn get_offset(&self, config: &Config) -> Complex {
        config.mandelbrot_cfg.c_min.clone()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MandelbrotCfg {
    pub max_iterations: u32,
    pub c_max: Complex,
    pub c_min: Complex,
    pub power: u32,
    pub color_cfg_name: Option<String>,
}

impl Default for MandelbrotCfg {
    fn default() -> Self {
        Self {
            max_iterations: MANDELBROT_DEFAULT_ITERATIONS,
            c_max: Complex::new(MANDELBROT_DEFAULT_C_MAX.0, MANDELBROT_DEFAULT_C_MAX.1),
            c_min: Complex::new(MANDELBROT_DEFAULT_C_MIN.0, MANDELBROT_DEFAULT_C_MIN.1),
            power: 2,
            color_cfg_name: None,
        }
    }
}
