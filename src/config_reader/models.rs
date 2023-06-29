
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct Config {
    pub service_name: String,
    pub port: i32,
    pub wiring_config: WiringConfig,
}

#[derive(Debug, Deserialize)]
pub struct WiringConfig {
    pub socket: String,
    pub queue_size: u32,
}
