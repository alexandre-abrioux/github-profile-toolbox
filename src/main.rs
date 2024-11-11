mod simple_icons;

use crate::simple_icons::color::is_relatively_light_icon_hex;
use crate::simple_icons::SimpleIcons;
use clap::Parser;
use markdown_table_formatter::format_tables;
use std::fs;
use yaml_rust2::yaml::Hash;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: String,
    /// Path to the README file to update
    #[arg(short, long)]
    readme: Option<String>,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.config).expect("Configuration file not found");
    let toolbox_markdown = generate_toolbox(&contents);
    let readme_path = args.readme.unwrap_or("".to_string());
    if readme_path.is_empty() {
        print!("{}", toolbox_markdown);
        return;
    }
    update_readme(&readme_path, &toolbox_markdown);
}

fn generate_toolbox(contents: &String) -> String {
    let tools = load_yaml(&contents);
    generate_markdown(&tools)
}

fn load_yaml(contents: &String) -> Hash {
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let yaml = &docs[0];
    let tools = yaml["tools"].as_hash().unwrap();
    tools.clone()
}

fn generate_markdown(tools: &Hash) -> String {
    let mut markdown = String::new();
    markdown.push_str(&generate_header(tools));
    markdown.push_str(&generate_items(tools));
    format_tables(markdown)
}

fn generate_header(tools: &Hash) -> String {
    let mut header = String::new();
    header.push_str("|");
    tools.keys().for_each(|category| {
        header.push_str(category.as_str().unwrap());
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

fn generate_items(tools: &Hash) -> String {
    let lines_nb = tools.keys().fold(0, |max_items_count, category| {
        let category_items_count = tools[category].as_vec().unwrap().len();
        if category_items_count > max_items_count {
            return category_items_count;
        }
        return max_items_count;
    });

    let mut items = String::new();
    for line in 1..=lines_nb {
        items.push_str("|");
        tools.keys().for_each(|category| {
            let category_items = tools[category].as_vec().unwrap();
            if line <= category_items.len() {
                let item = &category_items[line - 1];
                items.push_str(generate_img_tag(item).as_str());
            }
            items.push_str("|");
        });
        items.push_str("\n");
    }
    items
}

struct Item {
    label: String,
    color: String,
    icon: Option<String>,
}

fn extract_string_from_hash(hash: &Hash, key: &str) -> String {
    let value = Yaml::String(key.to_string());
    hash.get(&value)
        .expect(format!("missing {key} for item").as_str())
        .as_str()
        .expect(format!("{key} is not a string").as_str())
        .to_string()
}

fn generate_img_tag(item: &Yaml) -> String {
    if !item.is_hash() {
        return generate_img_tag_from_slug(item.as_str().unwrap());
    }
    let item_hash = item.as_hash().unwrap();
    let icon = item_hash
        .get(&Yaml::String("icon".to_string()))
        .map(|s| s.as_str().unwrap().to_string());
    let mut color_option: Option<String> = None;
    if icon.is_some() {
        let icon_from_slug = generate_item_from_slug(icon.unwrap().as_str());
        color_option = Some(icon_from_slug.color)
    }
    if item_hash.get(&Yaml::String("color".to_string())).is_some() {
        color_option = Some(extract_string_from_hash(item_hash, "color"))
    }
    let label = extract_string_from_hash(item_hash, "label");
    let color = color_option
        .expect(format!("missing color or icon for item {label}").as_str())
        .to_string();
    let item = Item {
        label,
        color,
        icon: item_hash
            .get(&Yaml::String("icon".to_string()))
            .map(|s| s.as_str().unwrap().to_string()),
    };
    generate_img_tag_from_item(&item)
}

fn generate_img_tag_from_slug(slug: &str) -> String {
    generate_img_tag_from_item(&generate_item_from_slug(&slug))
}

fn generate_item_from_slug(slug: &str) -> Item {
    let icons = &SimpleIcons::global().icons;
    let icon = icons
        .get(slug)
        .expect(format!("Could not find icon for slug {slug}").as_str());
    let title = &icon.title;
    let hex = &icon.hex;
    Item {
        label: title.to_string(),
        color: hex.to_string(),
        icon: Some(slug.to_string()),
    }
}

fn generate_img_tag_from_item(item: &Item) -> String {
    let mut tag = String::new();
    let label = &item.label;
    let color = &item.color;
    let background = if is_relatively_light_icon_hex(&color) {
        "black"
    } else {
        "white"
    };
    tag.push_str(format!(r#"[<img align="left" alt="{label}" src="https://img.shields.io/badge/-{label}-{color}?logoColor={background}"#).as_str());
    if item.icon.is_some() {
        let icon = item.icon.clone().unwrap();
        tag.push_str(format!("&logo={icon}").as_str());
    }
    tag.push_str(r#"">](#)"#);
    tag
}

fn update_readme(readme_path: &String, toolbox_markdown: &String) {
    let readme_content = fs::read_to_string(&readme_path).expect("Unable to read README file");
    let toolbox_section_start_bytes = readme_content
        .find("<!-- START_SECTION:toolbox -->")
        .expect("Could not find START_SECTION comment in README file");
    let toolbox_section_end_bytes = readme_content
        .find("<!-- STOP_SECTION:toolbox -->")
        .expect("Could not find STOP_SECTION comment in README file");
    let mut readme_content_updated = String::new();
    readme_content_updated.push_str(&readme_content[0..toolbox_section_start_bytes]);
    readme_content_updated.push_str("<!-- START_SECTION:toolbox -->\n");
    readme_content_updated.push_str("<!-- Generated by github-profile-toolbox GitHub action -->\n");
    readme_content_updated.push_str(&toolbox_markdown);
    readme_content_updated
        .push_str(&readme_content[toolbox_section_end_bytes..readme_content.len()]);
    fs::write(&readme_path, readme_content_updated).expect("Unable to write to README file");
}

#[cfg(test)]
mod tests {
    use crate::generate_toolbox;

    #[test]
    fn should_generate_markdown() {
        let input = "
tools:
  ides:
    - jetbrains
    - neovim
  languages:
    - javascript
    - rust";
        let markdown = generate_toolbox(&input.to_string());
        assert_eq!(
            markdown,
            r###"| ides                                                                                                                        | languages                                                                                                                      |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| [<img align="left" alt="JetBrains" src="https://img.shields.io/badge/-JetBrains-000000?logoColor=white&logo=jetbrains">](#) | [<img align="left" alt="JavaScript" src="https://img.shields.io/badge/-JavaScript-F7DF1E?logoColor=black&logo=javascript">](#) |
| [<img align="left" alt="Neovim" src="https://img.shields.io/badge/-Neovim-57A143?logoColor=white&logo=neovim">](#)          | [<img align="left" alt="Rust" src="https://img.shields.io/badge/-Rust-000000?logoColor=white&logo=rust">](#)                   |
"###
        );
    }

    #[test]
    fn should_handle_tools_with_different_number_of_items() {
        let input = "
tools:
  ides:
    - neovim
  languages:
    - javascript
    - rust";
        let markdown = generate_toolbox(&input.to_string());
        assert_eq!(
            markdown,
            r#"| ides                                                                                                               | languages                                                                                                                      |
| ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| [<img align="left" alt="Neovim" src="https://img.shields.io/badge/-Neovim-57A143?logoColor=white&logo=neovim">](#) | [<img align="left" alt="JavaScript" src="https://img.shields.io/badge/-JavaScript-F7DF1E?logoColor=black&logo=javascript">](#) |
|                                                                                                                    | [<img align="left" alt="Rust" src="https://img.shields.io/badge/-Rust-000000?logoColor=white&logo=rust">](#)                   |
"#
        );
    }

    #[test]
    fn should_handle_tools_with_custom_config() {
        let input = "
tools:
  ides:
    - label: VSCode
      color: 29a9f2
    - label: RustRover
      icon: jetbrains
    - label: RustRover
      icon: jetbrains
      color: feab02
";
        let markdown = generate_toolbox(&input.to_string());
        assert_eq!(
            markdown,
            r#"| ides                                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------------- |
| [<img align="left" alt="VSCode" src="https://img.shields.io/badge/-VSCode-29a9f2?logoColor=white">](#)                      |
| [<img align="left" alt="RustRover" src="https://img.shields.io/badge/-RustRover-000000?logoColor=white&logo=jetbrains">](#) |
| [<img align="left" alt="RustRover" src="https://img.shields.io/badge/-RustRover-feab02?logoColor=black&logo=jetbrains">](#) |
"#
        );
    }
}
