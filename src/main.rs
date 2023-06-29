use config_reader::get_config;

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
