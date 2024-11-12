use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub tools: Tools,
}

pub type Tools = IndexMap<String, Vec<ToolEnum>>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolEnum {
    StringLike(String),
    StructLike(ToolYaml),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ToolYaml {
    pub label: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

pub struct Tool {
    pub label: String,
    pub color: String,
    pub icon: Option<String>,
}
