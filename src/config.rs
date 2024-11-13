use indexmap::IndexMap;
use serde::Deserialize;
use serde_yml::from_str;
use std::fs;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub tools: Tools,
}

pub type Tools = IndexMap<String, Vec<ToolEnum>>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolEnum {
    StringLike(String),
    StructLike(ToolConfig),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ToolConfig {
    pub label: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

pub fn load_config(config_file_path: &String) -> Config {
    let content = fs::read_to_string(&config_file_path).expect("Configuration file not found");
    load_config_from_content(&content)
}

pub fn load_config_from_content(config_content: &String) -> Config {
    from_str(&config_content).unwrap()
}
