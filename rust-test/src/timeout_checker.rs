use std::time::{Duration, Instant};

pub struct TimeoutChecker {
    timeout_instant: Instant
}

impl TimeoutChecker {

    pub fn new(duration: Duration) -> TimeoutChecker {
        TimeoutChecker {
            timeout_instant: Instant::now() + duration
        }
    }

    pub fn is_timeout(&self) -> bool {
        Instant::now() > self.timeout_instant
    }

}
