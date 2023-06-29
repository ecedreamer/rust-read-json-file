pub mod config_reader;
pub mod message_printer;

use config_reader::models::Config;
use config_reader::config::read_config;
use message_printer::printer::my_print;


pub fn get_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config = read_config(file_path)?;
    my_print();
    Ok(config)
}
