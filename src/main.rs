use std::{
    sync::{Arc, Mutex},
    time::Duration, thread::sleep,
};

use pausable_clock::{PausableClock, PausableInstant};
use serde::{Serialize, Deserialize};
use zbus::{dbus_interface, ConnectionBuilder, Result};

use notify_rust::Notification;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct PomdConfig {
    work_duration: f32,
    short_break_duration: f32,
    long_break_duration: f32,
    num_iterations: u8,
    notify: bool,
}

impl Default for PomdConfig {
    fn default() -> Self {
        Self {
            work_duration: 15.0 * 60.0,
            short_break_duration: 5.0 * 60.0,
            long_break_duration: 25.0 * 60.0,
            num_iterations: 4,
            notify: true,
        }
    }
}


struct Pomd {
    config: PomdConfig,
    duration: Duration,
    iteration: u8,
    on_break: bool,
    clock: PausableClock,
    start: PausableInstant
}

struct PomdInterface {
    data: Arc<Mutex<Pomd>>,
    config: PomdConfig
}

impl PomdInterface {
    fn new(config: PomdConfig) -> Self {
        Self { 
            data: Arc::new(Mutex::new(Pomd::new(config))),
            config,
        }
    }
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
        *self.data.lock().unwrap() = Pomd::new(self.config);
    }

    async fn skip(&self) {
        self.data.lock().unwrap().setup_next_iteration();
    }
}

impl  Pomd {
    fn new(config: PomdConfig) -> Self {
        let clock = PausableClock::new(Duration::ZERO, true);
        let start  = clock.now();
        Self {
            config,
            duration: Duration::from_secs_f32(config.work_duration),
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
                if self.config.notify {
                    self.notify();
                }
                self.setup_next_iteration();
        }
    }

    fn setup_next_iteration(&mut self) {
        self.clock.pause();
        self.start  = self.clock.now();
        self.on_break ^= true;
        self.duration = if self.on_break {
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
                    self.config.num_iterations
                ))
                .body("Click to dismiss")
                .show()
                .unwrap();
        }
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let config: PomdConfig = confy::load("pomd", "config").expect("Failed to load config!");
    let pomd_interface = PomdInterface::new(config);
    let pomd = pomd_interface.data.clone();
    let _connection = ConnectionBuilder::session()?
        .name("dev.exvacuum.pomd")?
        .serve_at("/dev/exvacuum/pomd", pomd_interface)?
        .build()
        .await?;
    loop {
        pomd.lock().unwrap().update();
        sleep(Duration::from_millis(100));
    }
}
