use std::time::{Duration, Instant};

pub struct StopWatch {
    pub time_started: Option<Instant>,
}

impl StopWatch {
    pub fn start() -> Self {
        Self {
            time_started: Some(Instant::now()),
        }
    }
    pub fn duration(&self) -> Option<u64> {
        match self.time_started {
            Some(time_started) => Some(time_started.elapsed().as_secs()),
            None => Some(Instant::now().elapsed().as_secs()),
        }
    }
}
