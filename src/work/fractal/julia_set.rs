// use wasm_bindgen::prelude::web_sys;
use crate::components::root::Config;
use crate::work::{
    util::find_escape_radius,
    complex::Complex,
    fractal::Fractal,
};


pub struct JuliaSet {
    c: Complex,
    max: f64,
    iterations: u32,
}

impl JuliaSet {
    pub fn new(config: &Config) -> Self {
        info!(
            "creating fractal with: x_max: {}, x_min: {}, c: {}",
            config.julia_set_cfg.x_max,
            config.julia_set_cfg.x_min,
            config.julia_set_cfg.c
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
        Complex::new( (config.julia_set_cfg.x_max.real()
            - config.julia_set_cfg.x_min.real())
            / f64::from(canvas_width),
        (config.julia_set_cfg.x_max.imag()
            - config.julia_set_cfg.x_min.imag())
            / f64::from(canvas_height) )
    }

    /*
    fn calculate(&mut self, stats: Option<&mut Stats>) -> &Points {
        let performance = web_sys::window()
            .expect("Window not found")
            .performance()
            .expect("performance should be available");

        let start = performance.now();

        self.res.x_start = self.x_curr;
        self.res.y_start = self.y_curr;
        self.res.num_points = 0;

        let mut x = self.x_curr;
        let mut y = self.y_curr;

        let mut points_done: Option<usize> = None;
        let mut last_check = 0usize;
        let mut iterations = 0usize;

        for count in 0..self.res.values.len() {
            let calc = Complex::new(
                f64::from(x).mul_add(self.scale_real, self.offset.real()),
                f64::from(y).mul_add(self.scale_imag, self.offset.imag()),
            );
            let curr = self.iterate(&calc);
            self.res.values[count] = curr;

            if x < self.width - 1{
                x += 1;
            } else {
                x = 0;
                y += 1;
                if y >= self.height {
                    self.done = true;
                    points_done = Some(count + 1);
                    break;
                }
            }

            iterations +=  curr as usize;
            if iterations - last_check > 100 {
                last_check = iterations;
                if performance.now() - start >= MAX_DURATION {
                    points_done = Some(count + 1);
                    break;
                }
            }
        }
        if let Some(points) = points_done {
            self.res.num_points = points;
        } else {
            self.res.num_points = self.res.values.len();
        }

        self.x_curr = x;
        self.y_curr = y;

        if let Some(stats) = stats {
            stats.update(iterations, self.res.num_points, start);
        }

        &self.res
    }
*/
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
        last.map_or(self.iterations + 1, |last| last)
    }
}
