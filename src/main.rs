pub mod config_reader;
pub mod message_printer;

use config_reader::config::read_config;
use config_reader::models::Config;
use message_printer::printer::my_print;


pub fn get_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config = read_config(file_path)?;
    my_print();
    Ok(config)
}


fn main() {
    let file_path = "config_file/config.json";
    match get_config(file_path) {
        Ok(config) => {
            println!("Service Name: {}", config.service_name);
            println!("Socket: {}", config.wiring_config.socket);
        }
        Err(err) => {
            eprintln!("Error reading config: {}", err);
        }
    }
}
