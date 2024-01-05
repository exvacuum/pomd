use std::{sync::{Arc, Mutex}, time::Duration};
use zbus::dbus_interface;

use crate::pomd::Pomd;

/// D-Bus interface for the program
pub struct PomdInterface {
    pub state: Arc<Mutex<Pomd>>,
}

impl PomdInterface {
    /// Create a new instance of the interface with a reference to the program state
    pub fn new(state: Arc<Mutex<Pomd>>) -> Self {
        Self { 
            state
        }
    }
}

#[dbus_interface(name = "dev.exvacuum.pomd")]
impl PomdInterface {
    fn get_remaining(&self) -> Duration {
        let data = self.state.lock().unwrap();
        data.duration.checked_sub(data.start.elapsed(&data.clock)).unwrap_or_default()
    }

    fn get_iteration(&self) -> u8 {
        self.state.lock().unwrap().iteration
    }

    fn is_running(&self) -> bool {
        !self.state.lock().unwrap().clock.is_paused()
    }

    fn is_on_break(&self) -> bool {
        self.state.lock().unwrap().on_break
    }

    fn start(&self) {
        self.state.lock().unwrap().clock.resume();
    }

    fn pause(&self) {
        self.state.lock().unwrap().clock.pause();
    }

    fn stop(&self) {
        let mut data = self.state.lock().unwrap();
        *data = Pomd::new(data.config);
    }

    fn skip(&self) {
        self.state.lock().unwrap().setup_next_iteration();
    }
}
