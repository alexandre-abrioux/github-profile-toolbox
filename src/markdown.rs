use crate::config_schema::{ToolEnum, Tools};
use crate::simple_icons::color::is_relatively_light_icon_hex;
use crate::tool::{generate_tool_from_config, generate_tool_from_slug, Tool};
use markdown_table_formatter::format_tables;

pub fn generate_markdown(tools: &Tools) -> String {
    let mut markdown = String::new();
    markdown.push_str(&generate_markdown_table_headers(tools));
    markdown.push_str(&generate_markdown_table_content(tools));
    format_tables(markdown)
}

fn generate_markdown_table_headers(tools: &Tools) -> String {
    let mut header = String::new();
    header.push_str("|");
    tools.keys().for_each(|category| {
        header.push_str(category.as_str());
        header.push_str("|");
    });
    header.push_str("\n");

    header.push_str("|");
    tools.keys().for_each(|_category| {
        header.push_str("-");
        header.push_str("|");
    });
    header.push_str("\n");

    header
}

fn generate_markdown_table_content(tools: &Tools) -> String {
    let lines_nb = tools.keys().fold(0, |max_tools_count, category| {
        let tools_count = tools[category].len();
        if tools_count > max_tools_count {
            return tools_count;
        }
        return max_tools_count;
    });

    let mut content = String::new();
    for line in 1..=lines_nb {
        content.push_str("|");
        tools.keys().for_each(|category| {
            let category_items = &tools[category];
            if line <= category_items.len() {
                let tool = &category_items[line - 1];
                content.push_str(generate_img_tag(tool).as_str());
            }
            content.push_str("|");
        });
        content.push_str("\n");
    }
    content
}

fn generate_img_tag(tool_enum: &ToolEnum) -> String {
    let tool: Tool;
    match tool_enum {
        ToolEnum::StringLike(slug) => {
            tool = generate_tool_from_slug(slug);
        }
        ToolEnum::StructLike(tool_config) => {
            tool = generate_tool_from_config(tool_config);
        }
    }
    generate_img_tag_from_tool(&tool)
}

fn generate_img_tag_from_tool(tool: &Tool) -> String {
    let mut tag = String::new();
    let label = &tool.label;
    let color = &tool.color;
    let background = if is_relatively_light_icon_hex(&color) {
        "black"
    } else {
        "white"
    };
    tag.push_str(format!(r#"[<img align="left" alt="{label}" src="https://img.shields.io/badge/-{label}-{color}?logoColor={background}"#).as_str());
    if tool.icon.is_some() {
        let icon = tool.icon.clone().unwrap();
        tag.push_str(format!("&logo={icon}").as_str());
    }
    tag.push_str(r#"">](#)"#);
    tag
}
