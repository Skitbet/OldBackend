use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::Mutex;

/// A watchdog timer that periodically updates a tick timestamp,
/// and logs an error if ticks stop occurring within a configured timeout.
#[derive(Clone)]
pub struct Watchdog {
    last_tick: Arc<Mutex<Instant>>,
    tick_interval: Duration,
    check_interval: Duration,
    max_elapsed: Duration,
}

impl Watchdog {
    /// Creates a new `Watchdog`.
    ///
    /// * `tick_interval`: how often to update the last tick timestamp.
    /// * `check_interval`: how often to check if ticks have stalled.
    /// * `max_elapsed`: maximum allowed time without a tick before logging an error.
    pub fn new(tick_interval: Duration, check_interval: Duration, max_elapsed: Duration) -> Self {
        log::info!("Initializing watchdog");
        Self {
            last_tick: Arc::new(Mutex::new(Instant::now())),
            tick_interval,
            check_interval,
            max_elapsed,
        }
    }

    /// Returns a clone of the internal last tick timestamp lock.
    pub fn last_tick(&self) -> Arc<Mutex<Instant>> {
        self.last_tick.clone()
    }

    /// Starts the watchdog background tasks:
    /// 1. Periodically updates the last tick timestamp.
    /// 2. Periodically checks for staleness and logs an error if no tick occurred in time.
    pub fn start(&self) {
        let last_tick_for_updater = self.last_tick.clone();
        let tick_interval = self.tick_interval;
        tokio::spawn(async move {
            loop {
                {
                    let mut last_tick = last_tick_for_updater.lock().await;
                    *last_tick = Instant::now();
                }
                tokio::time::sleep(tick_interval).await;
            }
        });

        let last_tick_for_checker = self.last_tick.clone();
        let check_interval = self.check_interval;
        let max_elapsed = self.max_elapsed;
        tokio::spawn(async move {
            loop {
                {
                    let last_tick = last_tick_for_checker.lock().await;
                    let elapsed = last_tick.elapsed();
                    if elapsed > max_elapsed {
                        log::error!(
                            "Watchdog: No tick for {:?} (limit: {:?})",
                            elapsed,
                            max_elapsed
                        );
                    }
                }
                tokio::time::sleep(check_interval).await;
            }
        });
    }
}
