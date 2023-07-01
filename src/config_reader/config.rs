use std::fs::File;
use std::io::Read;
use super::models::Config;



pub fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let json_str = read_file(file_path)?;
    let config = serde_json::from_str(&json_str)?;
    Ok(config)
}