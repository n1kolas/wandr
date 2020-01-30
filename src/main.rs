mod config;
mod monitor;

use config::*;
use monitor::*;
use monitor::Setup;

use log::{info, warn};
use std::time::Duration;
use tokio::time;
use x11rb::connection::Connection;
use x11rb::xcb_ffi::XCBConnection;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let cfg: Config = confy::load("wandr").expect("config not found");
    let mut interval = time::interval(Duration::from_secs(cfg.interval));

    // Setup X connection.
    let (conn, screen_num) = XCBConnection::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let mut active_setup: Option<Setup> = None;
    loop {
        interval.tick().await;

        let monitors = grab_monitors(&conn, screen).unwrap();
        let possible_setup = detect_setup(monitors, cfg.setups.clone());
        if possible_setup.is_none() {
            warn!("no matching setup found");
            continue
        }

        // No active setup has been found, it's likely the tool checks for the first time.
        // In that case just set the found setup.
        if active_setup.as_ref().is_none() {
            info!("{}", possible_setup.as_ref().unwrap().name);
            active_setup = possible_setup;
            active_setup.as_ref().unwrap().run_scripts()?;
            continue
        }
        // The found setup differs from the active setup.
        if active_setup.as_ref().unwrap().name != possible_setup.as_ref().unwrap().name {
            info!("{} <- {}", possible_setup.as_ref().unwrap().name, active_setup.as_ref().unwrap().name);
            active_setup = possible_setup;

            active_setup.as_ref().unwrap().run_scripts()?;
        }
    }
}
