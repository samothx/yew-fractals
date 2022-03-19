use seed::{log, prelude::web_sys};

use super::{util::find_escape_radius, MAX_DURATION};

use super::{
    complex::Complex,
    fractal::{Fractal, Points},
    Model,
    stats::Stats
};

pub struct Mandelbrot {
    scale_real: f64,
    scale_imag: f64,
    offset: Complex,
    x_curr: u32,
    y_curr: u32,
    width: u32,
    height: u32,
    iterations: u32,
    res: Points,
    done: bool,
}

impl Mandelbrot {
    pub fn new(model: &Model) -> Self {
        log!(format!(
            "creating fractal with: x_max: {}, x_min: {}",
            model.config.mandelbrot_cfg.c_max, model.config.mandelbrot_cfg.c_min,
        ));

        let scale_real = (model.config.mandelbrot_cfg.c_max.real()
            - model.config.mandelbrot_cfg.c_min.real())
            / f64::from(model.width);
        let scale_imag = (model.config.mandelbrot_cfg.c_max.imag()
            - model.config.mandelbrot_cfg.c_min.imag())
            / f64::from(model.height);

        Self {
            scale_real,
            scale_imag,
            offset: model.config.mandelbrot_cfg.c_min,
            x_curr: 0,
            y_curr: 0,
            width: model.width,
            height: model.height,
            iterations: model.config.mandelbrot_cfg.max_iterations,
            res: Points::default(),
            done: false,
        }
    }

    fn iterate(&self, c: &Complex) -> u32 {
        let max = find_escape_radius(c.norm()).powi(2);
        let mut x = Complex::new(0.0, 0.0);
        // log!(format!("iterate: start: {}", curr));
        let mut last: Option<u32> = None;
        for idx in 1..=self.iterations {
            x = x * x + *c;
            if x.square_length() >= max {
                last = Some(idx);
                break;
            }
        }

        // log!(format!("iterate: end:  {} norm: {} last: {:?}", curr, curr.square_length(), last));
        last.map_or(self.iterations + 1, |last| last)
    }
}

impl Fractal for Mandelbrot {
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
                f64::from(x).mul_add(self.scale_real,self.offset.real()),
                f64::from(y).mul_add(self.scale_imag,self.offset.imag()),
            );
            let curr = self.iterate(&calc);
            self.res.values[count] = curr;

            if x < self.width - 1 {
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

            iterations += curr as usize;
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

    fn is_done(&self) -> bool {
        self.done
    }
}
