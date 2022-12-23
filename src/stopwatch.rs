use std::time::{Instant, Duration};

pub struct StopWatch {
    pub time_started: Option<Instant>
}

impl StopWatch {
    pub fn start() -> Self {
        Self {
            time_started: Some(Instant::now())
        }
    }
    pub fn duration(&self) -> Option<Duration> {
        match self.time_started {
            Some(time_started) => Some(time_started.elapsed()),
            None => Some(Instant::now().elapsed()),
        }
    }
}
