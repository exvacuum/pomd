use std::sync::{Mutex, Arc};
use std::{thread::sleep, time::Duration};
use zbus::{ConnectionBuilder, Result};

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
    let pomd_interface = PomdInterface::new(pomd.clone());
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
