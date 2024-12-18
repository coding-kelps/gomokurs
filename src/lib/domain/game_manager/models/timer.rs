//! A timer module for managing game or task durations with pausing, resuming, and resetting capabilities.
//!
//! This module provides a `Timer` struct that allows tracking time for player.
//! It supports pausing and resuming the timer, resetting the elapsed time,
//! and querying the remaining duration of the timer.

use tokio::time::{sleep, Duration, Instant};
use tokio::sync::{Mutex, Notify};
use std::sync::Arc;

/// The `Timer` struct represents a timer with configurable turn and match
/// durations.
///
/// The player timer is composed of two internal timers:
/// * **Turn Timer** - Runs for the specified `turn_duration` and resets at the
/// start of each turn.
/// * **Match Timer** - Acts as a time reserve that only starts decrementing
/// once the turn timer has expired. When the match timer is exhausted, the
/// player loses.
#[derive(Debug, Clone)]
pub struct Timer {
    /// The duration of a single turn.
    turn_duration: Duration,
    /// The total duration of the match.
    match_duration: Duration,
    /// Tracks the elapsed time.
    elapsed: Arc<Mutex<Duration>>,
    /// Notify handle for pausing the timer.
    notify_pause: Arc<Notify>,
    /// Notify handle for resuming the timer.
    notify_resume: Arc<Notify>,
}

impl Timer {
    /// Creates a new `Timer` instance with the specified turn and match
    /// durations.
    ///
    /// # Arguments
    /// * `turn_duration` - The duration of a single turn.
    /// * `match_duration` - The total duration of the match.
    ///
    /// # Returns
    /// A new `Timer` instance.
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

    /// Runs the timer, starting in a paused state if specified. Returns when
    /// the timer runs out.
    ///
    /// This method loops until the timer expires or is paused. The elapsed time
    /// is updated as the timer progresses, and it responds to pause and resume
    /// notifications.
    ///
    /// # Arguments
    /// * `start_paused` - If `true`, the timer starts in a paused state.
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
                // Sleep for the turn duration, then sleep for the remaining
                // time of the match duration.
                _ = {
                    sleep(self.turn_duration).await;

                    sleep(*self.elapsed.lock().await)
                 } => {
                    return;
                },
                // Pause timer if a pause notification was send.
                _ = self.notify_pause.notified() => {
                    *self.elapsed.lock().await = start_time.elapsed();
                    self.notify_resume.notified().await;
    
                    continue;
                }
            }
        }
    }

    /// Pauses the timer by sending a pause notification.
    pub async fn pause(&self) {
        self.notify_pause.notify_one();
    }

    /// Resumes the timer by sending a resume notification.
    pub async fn resume(&self) {
        self.notify_resume.notify_one();
    }

    /// Resets the timer's elapsed time to zero.
    pub async fn reset(&self) {
        *self.elapsed.lock().await = Duration::ZERO;
    }

    /// Gets the remaining match duration.
    ///
    /// # Returns
    /// The remaining time before the match duration expires. If the elapsed
    /// time exceeds the match duration, returns `Duration::ZERO`.
    pub async fn get_remaining(&self) -> Duration {
        self.match_duration.saturating_sub(*self.elapsed.lock().await)
    }
}
