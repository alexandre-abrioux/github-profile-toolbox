use crate::config_schema::ToolConfig;
use crate::simple_icons::SimpleIcons;

pub struct Tool {
    pub label: String,
    pub color: String,
    pub icon: Option<String>,
}

pub fn generate_tool_from_config(tool_config: &ToolConfig) -> Tool {
    let mut color_option: Option<String> = None;
    if tool_config.color.is_some() {
        color_option = Some(tool_config.color.clone().unwrap().replace("#", ""))
    } else if tool_config.icon.is_some() {
        let tool_from_slug = generate_tool_from_slug(&tool_config.icon.clone().unwrap());
        color_option = Some(tool_from_slug.color)
    }
    let label = tool_config.label.clone();
    let color = color_option
        .expect(format!("missing color or icon for item {label}").as_str())
        .to_string();
    Tool {
        label,
        color,
        icon: tool_config.icon.clone(),
    }
}

pub fn generate_tool_from_slug(slug: &String) -> Tool {
    let icons = &SimpleIcons::global().icons;
    let icon = icons
        .get(slug)
        .expect(format!("Could not find icon for slug {slug}").as_str());
    let title = &icon.title;
    let hex = &icon.hex;
    Tool {
        label: title.clone(),
        color: hex.clone(),
        icon: Some(slug.clone()),
    }
}
