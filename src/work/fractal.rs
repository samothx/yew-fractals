use super::stats::Stats;

mod julia_set;
pub use julia_set::JuliaSet;
mod mandelbrot;
pub use mandelbrot::Mandelbrot;
use crate::work::complex::Complex;
use crate::components::root::{Config, FractalType};


const MAX_POINTS: usize = 5000;
pub const MAX_DURATION: f64 = 200.0;


pub struct FractalCalculator {
    fractal: Box<dyn Fractal>,
    res: Points,
    x_curr: u32,
    y_curr: u32,
    width: u32,
    height: u32,
    scale: Complex,
    offset: Complex,
    done: bool,
}

impl FractalCalculator {
    pub fn new(config: &Config, canvas_width: u32, canvas_height: u32) -> FractalCalculator {
        let fractal: Box<dyn Fractal> = match config.active_config {
            FractalType::Mandelbrot => Box::new(Mandelbrot::new(&config )),
            FractalType::JuliaSet => Box::new(JuliaSet::new(&config))
        };

        let scale = fractal.get_scale(config, canvas_width, canvas_height);
        let offset = fractal.get_offset(config);

        FractalCalculator {
            fractal,
            res: Points::default(),
            x_curr: 0,
            y_curr: 0,
            width: canvas_width,
            height: canvas_height,
            scale,
            offset,
            done: false,
        }
    }

    pub fn calculate(&mut self, stats: Option<&mut Stats>) -> &Points {
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
                f64::from(x).mul_add(self.scale.real(),self.offset.real()),
                f64::from(y).mul_add(self.scale.imag(),self.offset.imag()),
            );
            let curr = self.fractal.iterate(&calc);
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

    pub fn is_done(&self) -> bool {
        self.done
    }
}


pub trait Fractal {
    fn get_scale(&self, config: &Config, canvas_width: u32, canvas_height: u32) -> Complex;
    fn get_offset(&self, config: &Config) -> Complex;
    fn iterate(&self, calc: &Complex) -> u32;
}

pub struct Points {
    pub x_start: u32,
    pub y_start: u32,
    pub num_points: usize,
    pub values: [u32; MAX_POINTS],
}

impl Default for Points {
    fn default() -> Self {
        Self {
            x_start: 0,
            y_start: 0,
            num_points: 0,
            values: [0; MAX_POINTS],
        }
    }
}

