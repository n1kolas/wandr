use serde::{Deserialize, Serialize};
use crate::monitor::Setup;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub interval: u64,
    pub setups: Vec<Setup>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            setups: Vec::default(),
            interval: 5,
        }
    }
}
