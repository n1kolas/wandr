#![macro_use]
use serde::{Deserialize, Serialize};
use x11rb::connection::Connection;
use x11rb::errors::ConnectionErrorOrX11Error;
use x11rb::generated::randr;
use x11rb::generated::xproto::*;

#[derive(Debug)]
pub struct Monitor {
    pub connected: bool,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Setup {
    pub name: String,
    on: Vec<String>,
    off: Vec<String>,
    exec: Vec<String>,
}

impl Setup {
    pub fn run_scripts(&self) -> Result<(), std::io::Error> {
        let options = run_script::ScriptOptions::new();
        let args = vec![];
        for cmd in &self.exec {
            let (_, _, _) = run_script::run(
                &cmd.to_owned(),
                &args,
                &options
            ).unwrap();
        }
        Ok(())
    }
}

pub fn grab_monitors<C: Connection>(
    conn: &C,
    screen: &Screen,
) -> Result<Vec<Monitor>, ConnectionErrorOrX11Error> {
    let mut monitors: Vec<Monitor> = Vec::new();

    let res = randr::get_screen_resources_current(conn, screen.root)?.reply()?;
    for out in res.outputs {
        let info = randr::get_output_info(conn, out, 0)?.reply()?;

        monitors.push(Monitor {
            connected: info.connection == randr::Connection::Connected as u8,
            name: String::from_utf8_lossy(&info.name).to_string(),
        });
    }

    Ok(monitors)
}

pub fn detect_setup(monitors: Vec<Monitor>, setups: Vec<Setup>) -> Option<Setup> {
    let on_monitors: Vec<String> = monitors.iter().filter(|m| m.connected).map(|m| m.name.clone()).collect();
    let off_monitors: Vec<String> = monitors.iter().filter(|m| !m.connected).map(|m| m.name.clone()).collect();

    let possible_setups: Vec<Setup> = setups.iter()
    .filter(|s| {
        for should_on in &s.on {
            if !on_monitors.contains(&should_on) {
                return false;
            }
        }
        true
    })
    .filter(|s| {
        for should_off in &s.off {
            if !off_monitors.contains(&should_off) {
                return false;
            }
        }
        true
    }).cloned().collect();

    possible_setups.first().cloned()
}
