use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use zbus::{dbus_interface, ConnectionBuilder, Result};

use notify_rust::Notification;

const WORK_DURATION_SECS: f32 = 15.0 * 60.0;
const SHORT_BREAK_DURATION_SECS: f32 = 5.0 * 60.0;
const LONG_BREAK_DURATION_SECS: f32 = 15.0 * 60.0;
const NUM_ITERATIONS: u8 = 4;

struct Pomd {
    remaining: Duration,
    iteration: u8,
    running: bool,
    on_break: bool,
    last_instant: Instant,
}

#[derive(Default)]
struct PomdInterface {
    data: Arc<Mutex<Pomd>>,
}

#[dbus_interface(name = "dev.exvacuum.pomd")]
impl PomdInterface {
    async fn get_remaining(&self) -> Duration {
        self.data.lock().unwrap().remaining
    }

    async fn get_iteration(&self) -> u8 {
        self.data.lock().unwrap().iteration
    }

    async fn is_running(&self) -> bool {
        self.data.lock().unwrap().running
    }

    async fn is_on_break(&self) -> bool {
        self.data.lock().unwrap().on_break
    }

    async fn start(&self) {
        self.data.lock().unwrap().running = true;
    }

    async fn pause(&self) {
        self.data.lock().unwrap().running = false;
    }

    async fn stop(&self) {
        *self.data.lock().unwrap() = Pomd::default();
    }

    async fn skip(&self) {
        self.data.lock().unwrap().running = false;
        self.data.lock().unwrap().setup_next_iteration();
    }
}

impl Default for Pomd {
    fn default() -> Self {
        Self {
            remaining: Duration::from_secs_f32(WORK_DURATION_SECS),
            iteration: 0,
            running: false,
            on_break: false,
            last_instant: Instant::now(),
        }
    }
}

impl Pomd {
    fn update(&mut self) {
        let elapsed = self.last_instant.elapsed();
        self.last_instant = Instant::now();
        if self.running {
            if self.remaining > elapsed {
                self.remaining -= elapsed;
            } else {
                self.running = false;
                self.notify();
                self.setup_next_iteration();
            }
        }
    }

    fn setup_next_iteration(&mut self) {
        self.on_break ^= true;
        self.remaining = if self.on_break {
            if self.iteration == NUM_ITERATIONS-1 { Duration::from_secs_f32(LONG_BREAK_DURATION_SECS) } else { Duration::from_secs_f32(SHORT_BREAK_DURATION_SECS) }
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
                .show().unwrap();
        } else {
            Notification::new()
                .summary(&format!("Pomodoro Complete ({}/{})", self.iteration + 1, NUM_ITERATIONS))
                .body("Click to dismiss")
                .show().unwrap();
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
