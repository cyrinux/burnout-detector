use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum Status {
    Active(Instant, Duration),
    Idle(Instant, Duration),
}

impl Status {
    /// Update the elapsed time
    pub fn update(&mut self, elapsed: Duration) -> Status {
        match self {
            Status::Active(start, _) => Status::Active(*start, elapsed),
            Status::Idle(start, _) => Status::Idle(*start, elapsed),
        }
    }

    /// TODO: remove if useless
    ///  Return the start time
    #[allow(dead_code)]
    pub fn start_time(&self) -> Instant {
        match self {
            Status::Active(start, _) => *start,
            Status::Idle(start, _) => *start,
        }
    }

    /// Return the elapsed time since the start
    pub fn elapsed_time(&self) -> Duration {
        match self {
            Status::Active(_, elapsed) => *elapsed,
            Status::Idle(_, elapsed) => *elapsed,
        }
    }
}
