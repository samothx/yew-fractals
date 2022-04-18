use web_sys::window;

pub struct Stats {
    start_time: f64,
    total_time: f64,
    time_in_fractal: f64,
    iterations: usize,
    points: usize,
}

impl Stats {
    pub fn new() -> Self {
        let performance = window()
            .expect("Window not found")
            .performance()
            .expect("performance should be available");

        Self {
            start_time: performance.now(),
            total_time: 0.0,
            time_in_fractal: 0.0,
            iterations: 0,
            points: 0,
        }
    }

    pub fn update(&mut self, iterations: usize, points: usize, start_time: f64) {
        let performance = web_sys::window()
            .expect("Window not found")
            .performance()
            .expect("performance should be available");
        let end = performance.now();
        self.iterations += iterations;
        self.points += points;
        self.time_in_fractal += end - start_time;
        self.total_time = end - self.start_time;
    }

    pub fn format_stats(&self) -> String {
        format!(
            "\
Iterations: {:.4E}
Points:     {:.4E}
Time Calc:  {}
Tot. Time:  {}
Iter/Sec:   {:.3}
Points/Sec: {:.3}
        ",
            self.iterations, self.points, Stats::format_time(self.time_in_fractal),
            Stats::format_time(self.total_time),self.iterations as f64/ self.time_in_fractal,
            self.points as f64 / self.time_in_fractal
        )
    }

    fn format_time(time: f64) -> String {
        let time_in_secs = time / 1000.0; 
        let hours = (time_in_secs / 3600.0).floor();
        let minutes = ((time_in_secs % 3600.0) / 60.0).floor();
        let seconds = (time_in_secs % 60.0).floor();
        format!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
 
    }
}

#[cfg(test)] 
mod test {
    use super::Stats;
    #[test]
    fn test_format_time() {
        assert_eq!(Stats::format_time(3600000.0).as_str(), "1:00:00");
        assert_eq!(Stats::format_time(3665000.0).as_str(), "1:01:05");
    }
}
