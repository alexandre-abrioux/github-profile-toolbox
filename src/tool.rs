use crate::schema::{Tool, ToolYaml};
use crate::simple_icons::SimpleIcons;

pub fn generate_tool_from_struct(tool_yaml: &ToolYaml) -> Tool {
    let mut color_option: Option<String> = None;
    if tool_yaml.color.is_some() {
        color_option = Some(tool_yaml.color.clone().unwrap().replace("#", ""))
    } else if tool_yaml.icon.is_some() {
        let tool_from_slug = generate_tool_from_slug(&tool_yaml.icon.clone().unwrap());
        color_option = Some(tool_from_slug.color)
    }
    let label = tool_yaml.label.clone();
    let color = color_option
        .expect(format!("missing color or icon for item {label}").as_str())
        .to_string();
    Tool {
        label,
        color,
        icon: tool_yaml.icon.clone(),
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
