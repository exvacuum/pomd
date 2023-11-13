use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use pausable_clock::{PausableClock, PausableInstant};
use zbus::{dbus_interface, ConnectionBuilder, Result};

use notify_rust::Notification;

const WORK_DURATION_SECS: f32 = 15.0 * 60.0;
const SHORT_BREAK_DURATION_SECS: f32 = 5.0 * 60.0;
const LONG_BREAK_DURATION_SECS: f32 = 25.0 * 60.0;
const NUM_ITERATIONS: u8 = 4;

struct Pomd {
    duration: Duration,
    iteration: u8,
    on_break: bool,
    clock: PausableClock,
    start: PausableInstant
}

#[derive(Default)]
struct PomdInterface {
    data: Arc<Mutex<Pomd>>,
}

#[dbus_interface(name = "dev.exvacuum.pomd")]
impl PomdInterface {
    async fn get_remaining(&self) -> Duration {
        let data = self.data.lock().unwrap();
        data.duration.checked_sub(data.start.elapsed(&data.clock)).unwrap_or_default()
    }

    async fn get_iteration(&self) -> u8 {
        self.data.lock().unwrap().iteration
    }

    async fn is_running(&self) -> bool {
        !self.data.lock().unwrap().clock.is_paused()
    }

    async fn is_on_break(&self) -> bool {
        self.data.lock().unwrap().on_break
    }

    async fn start(&self) {
        self.data.lock().unwrap().clock.resume();
    }

    async fn pause(&self) {
        self.data.lock().unwrap().clock.pause();
    }

    async fn stop(&self) {
        *self.data.lock().unwrap() = Pomd::default();
    }

    async fn skip(&self) {
        self.data.lock().unwrap().setup_next_iteration();
    }
}

impl Default for Pomd {
    fn default() -> Self {
        let clock = PausableClock::new(Duration::ZERO, true);
        let start  = clock.now();
        Self {
            duration: Duration::from_secs_f32(WORK_DURATION_SECS),
            iteration: 0,
            on_break: false,
            clock,
            start,
        }
    }
}

impl Pomd {
    fn update(&mut self) {
        if self.duration < self.start.elapsed(&self.clock) {
                self.notify();
                self.setup_next_iteration();
        }
    }

    fn setup_next_iteration(&mut self) {
        self.clock.pause();
        self.start  = self.clock.now();
        self.on_break ^= true;
        self.duration = if self.on_break {
            if self.iteration == NUM_ITERATIONS - 1 {
                Duration::from_secs_f32(LONG_BREAK_DURATION_SECS)
            } else {
                Duration::from_secs_f32(SHORT_BREAK_DURATION_SECS)
            }
        } else {
            self.iteration = (self.iteration + 1) % NUM_ITERATIONS;
            Duration::from_secs_f32(WORK_DURATION_SECS)
        }
    }

    fn notify(&self) {
        if self.on_break {
            Notification::new()
                .summary("Break Complete")
                .body("Click to dismiss")
                .show()
                .unwrap();
        } else {
            Notification::new()
                .summary(&format!(
                    "Pomodoro Complete ({}/{})",
                    self.iteration + 1,
                    NUM_ITERATIONS
                ))
                .body("Click to dismiss")
                .show()
                .unwrap();
        }
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let pomd_interface = PomdInterface::default();
    let pomd = pomd_interface.data.clone();
    let _connection = ConnectionBuilder::session()?
        .name("dev.exvacuum.pomd")?
        .serve_at("/dev/exvacuum/pomd", pomd_interface)?
        .build()
        .await?;
    loop {
        pomd.lock().unwrap().update();
    }
}
