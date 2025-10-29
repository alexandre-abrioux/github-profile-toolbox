use crate::config_schema::Config;
use serde_saphyr::from_str;
use std::fs;

pub fn load_config(config_file_path: &String) -> Config {
    let content = fs::read_to_string(&config_file_path).expect("Configuration file not found");
    load_config_from_content(&content)
}

pub fn load_config_from_content(config_content: &String) -> Config {
    from_str(&config_content).unwrap()
}
