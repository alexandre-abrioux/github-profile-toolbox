use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimpleIconsData {
    pub icons: Vec<SimpleIconData>,
}

#[derive(Deserialize)]
pub struct SimpleIconData {
    pub slug: Option<String>,
    pub title: String,
    pub hex: String,
}

pub fn fetch_simple_icons_data() -> SimpleIconsData {
    let target = "https://raw.githubusercontent.com/simple-icons/simple-icons/develop/_data/simple-icons.json";
    let body: String = ureq::get(&target)
        .call()
        .expect("Could not download simple-icons")
        .into_string()
        .expect("Could not convert simple-icons to string");
    serde_json::from_str(&body).expect("JSON was not well-formatted")
}
