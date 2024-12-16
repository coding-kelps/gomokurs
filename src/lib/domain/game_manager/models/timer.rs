use tokio::time::{sleep, Duration, Instant};
use tokio::sync::{Mutex, Notify};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Timer {
    turn_duration: Duration,
    match_duration: Duration,
    elapsed: Arc<Mutex<Duration>>,
    notify_pause: Arc<Notify>,
    notify_resume: Arc<Notify>,
}

impl Timer {
    pub fn new(
        turn_duration: Duration,
        match_duration: Duration,
    ) -> Self
    {
        Self {
            turn_duration,
            match_duration,
            elapsed: Arc::new(Mutex::new(Duration::ZERO)),
            notify_pause: Arc::new(Notify::new()),
            notify_resume: Arc::new(Notify::new()),
        }
    }

    pub async fn run(
        &self,
        start_paused: bool,
    ) {
        if start_paused {
            self.pause().await;
        }

        loop {
            let start_time = Instant::now();

            tokio::select! {
                _ = {
                    sleep(self.turn_duration).await;

                    sleep(*self.elapsed.lock().await)
                 } => {
                    return;
                },
                _ = self.notify_pause.notified() => {
                    *self.elapsed.lock().await = start_time.elapsed();
                    self.notify_resume.notified().await;
    
                    continue;
                }
            }
        }
    }

    pub async fn pause(&self) {
        self.notify_pause.notify_one();
    }

    pub async fn resume(&self) {
        self.notify_resume.notify_one();
    }

    pub async fn reset(&self) {
        *self.elapsed.lock().await = Duration::ZERO;
    }

    pub async fn get_remaining(&self) -> Duration {
        self.match_duration.saturating_sub(*self.elapsed.lock().await)
    }
}
