use std::time::{Duration, Instant};

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
