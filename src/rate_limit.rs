use std::collections::VecDeque;

pub struct RateLimiterDetector {
    window: VecDeque<u16>,
    max_size: usize,
}

impl RateLimiterDetector {
    pub fn new(size: usize) -> Self {
        Self {
            window: VecDeque::with_capacity(size),
            max_size: size,
        }
    }

    pub fn record(&mut self, status: u16) {
        if self.window.len() >= self.max_size {
            self.window.pop_front();
        }
        self.window.push_back(status);
    }

    pub fn rate_limited(&self) -> bool {
        let total = self.window.len();
        if total == 0 {
            return false;
        }

        let rate_429 = self.window.iter().filter(|&&s| s == 429).count();
        (rate_429 as f64 / total as f64) > 0.05
    }
}
