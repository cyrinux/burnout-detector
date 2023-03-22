use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum Status {
    Active(Instant, Duration),
    Idle(Instant, Duration),
}

impl Status {
    /// Update the elapsed time
    pub fn update(&mut self, elapsed: Duration) -> Status {
        self.status = match self.status {
            Status::Active(start, _) => Status::Active(start, Instant::now() - start),
            Status::Idle(start, _) => Status::Idle(start, Instant::now() - start),
        }
    }
}
