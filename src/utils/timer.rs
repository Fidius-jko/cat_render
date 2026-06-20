use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start: Instant,
    dur: Duration,
}

impl Timer {
    pub fn new(dur: Duration) -> Self {
        Self {
            start: Instant::now(),
            dur,
        }
    }
    pub fn is_ended(&self) -> bool {
        Instant::now() - self.start >= self.dur
    }
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

#[derive(Debug)]
pub struct TimeStep {
    last_time: Instant,
    delta_time: f32,
    frame_count: u32,
    frame_time: f32,
    last_frame_rate: u32,
}
impl Default for TimeStep {
    fn default() -> Self {
        TimeStep::new()
    }
}
impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
            last_frame_rate: 0,
        }
    }

    pub fn delta(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros() as f32 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }

    pub fn frame_rate(&mut self) -> Option<u32> {
        self.frame_count += 1;
        self.frame_time += self.delta_time;
        let tmp;
        if self.frame_time >= 1.0 {
            tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            self.last_frame_rate = tmp;
            return Some(tmp);
        }
        Some(self.last_frame_rate)
    }
}
