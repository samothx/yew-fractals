use crate::components::root::Config;
use crate::work::util::find_escape_radius;

use crate::work::{
    complex::Complex,
    fractal::Fractal,
};

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
        let max = find_escape_radius(c.norm()).powi(2);
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
        last.map_or(self.iterations + 1, |last| last)
    }

    fn get_scale(&self, config: &Config, canvas_width: u32, canvas_height: u32) -> Complex {
        Complex::new((config.mandelbrot_cfg.c_max.real()
            - config.mandelbrot_cfg.c_min.real())
            / f64::from(canvas_width),
        (config.mandelbrot_cfg.c_max.imag()
            - config.mandelbrot_cfg.c_min.imag())
            / f64::from(canvas_height))
    }

    fn get_offset(&self, config: &Config) -> Complex {
        config.mandelbrot_cfg.c_min.clone()
    }
}
