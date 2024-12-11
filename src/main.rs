use std::sync::{Arc, Mutex};
use std::{thread::sleep, time::Duration};
use zbus::{connection::Builder, Result};

use crate::config::PomdConfig;
use crate::interface::PomdInterface;
use crate::pomd::Pomd;

mod config;
mod interface;
mod pomd;

#[async_std::main]
async fn main() -> Result<()> {
    let config: PomdConfig = confy::load("pomd", "config").expect("Failed to load config!");
    let pomd = Arc::new(Mutex::new(Pomd::new(config)));
    let mut _connection;
    loop {
        let pomd_interface = PomdInterface::new(pomd.clone());
        match Builder::session()?
            .name("dev.exvacuum.pomd")?
            .serve_at("/dev/exvacuum/pomd", pomd_interface)?
            .build()
            .await
        {
            Ok(connection) => {
                _connection = connection;
                break;
            }
            Err(e) => {
                eprintln!("Failed to start D-Bus session: {e:?}");
                eprintln!("Trying again in 10 seconds...");
                sleep(Duration::from_secs(10));
                continue;
            }
        }
    }
    loop {
        pomd.lock().unwrap().update();
        sleep(Duration::from_millis(100));
    }
}
