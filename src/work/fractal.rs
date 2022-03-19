use super::stats::Stats;

const MAX_POINTS: usize = 5000;

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

pub trait Fractal {
    fn calculate(&mut self, stats: Option<&mut Stats>) -> &Points;
    fn is_done(&self) -> bool;
}
