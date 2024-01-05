use std::time::Duration;

use pausable_clock::{PausableClock, PausableInstant};

use notify_rust::Notification;

use crate::config::PomdConfig;

/// Represents the current state of the program
pub struct Pomd {
    pub config: PomdConfig,
    pub duration: Duration,
    pub iteration: u8,
    pub on_break: bool,
    pub clock: PausableClock,
    pub start: PausableInstant,
}

impl Pomd {
    /// Creates a new instance of this struct with a given configuration
    pub fn new(config: PomdConfig) -> Self {
        let clock = PausableClock::new(Duration::ZERO, true);
        let start = clock.now();
        Self {
            config,
            duration: Duration::from_secs_f32(config.work_duration),
            iteration: 0,
            on_break: false,
            clock,
            start,
        }
    }

    /// Check whether sufficient time has elapsed to enter next iteration of cycle
    pub fn update(&mut self) {
        if self.duration < self.start.elapsed(&self.clock) {
            if self.config.notify {
                self.notify();
            }
            self.setup_next_iteration();
        }
    }

    /// Resets state for next iteration
    pub fn setup_next_iteration(&mut self) {
        // Stop clock until user restarts it
        self.clock.pause();

        self.start = self.clock.now();
        self.on_break ^= true;
        self.duration = if self.on_break {
            // Long break on last iteration
            if self.iteration == self.config.num_iterations - 1 {
                Duration::from_secs_f32(self.config.long_break_duration)
            } else {
                Duration::from_secs_f32(self.config.short_break_duration)
            }
        } else {
            self.iteration = (self.iteration + 1) % self.config.num_iterations;
            Duration::from_secs_f32(self.config.work_duration)
        }
    }

    /// Displays a system notification
    pub fn notify(&self) {
        Notification::new()
            .summary(
                &(if self.on_break {
                    "Break Complete".to_string()
                } else {
                    format!(
                        "Pomodoro Complete ({}/{})",
                        self.iteration + 1,
                        self.config.num_iterations
                    )
                }),
            )
            .body("Click to dismiss")
            .show()
            .unwrap();
    }
}
