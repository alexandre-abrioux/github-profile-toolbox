mod simple_icons;

use std::fs;
use yaml_rust2::yaml::Hash;
use yaml_rust2::YamlLoader;

use crate::simple_icons::color::is_relatively_light_icon_hex;
use crate::simple_icons::SimpleIcons;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.config).expect("Configuration file not found");
    let markdown = generate_toolbox(&contents);
    print!("{}", markdown);
}

fn generate_toolbox(contents: &String) -> String {
    let sections = load_yaml(&contents);
    generate_markdown(&sections)
}

fn load_yaml(contents: &String) -> Hash {
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let yaml = &docs[0];
    let sections = yaml["sections"].as_hash().unwrap();
    sections.clone()
}

fn generate_markdown(sections: &Hash) -> String {
    let mut markdown = String::new();
    markdown.push_str(&generate_header(sections));
    markdown.push_str(&generate_items(sections));
    markdown
}

fn generate_header(sections: &Hash) -> String {
    let mut header = String::new();
    header.push_str("|");
    sections.keys().for_each(|k| {
        header.push_str(k.as_str().unwrap());
        header.push_str("|");
    });
    header.push_str("\n");

    header.push_str("|");
    sections.keys().for_each(|_k| {
        header.push_str("-");
        header.push_str("|");
    });
    header.push_str("\n");

    header
}

fn generate_items(sections: &Hash) -> String {
    let lines_nb = sections.keys().fold(0, |max_sections_length, key| {
        let section_items = sections[key].as_vec().unwrap();
        if section_items.len() > max_sections_length {
            return section_items.len();
        }
        return max_sections_length;
    });

    let mut items = String::new();
    for line in 1..=lines_nb {
        items.push_str("|");
        sections.keys().for_each(|k| {
            let section_items = sections[k].as_vec().unwrap();
            if line <= section_items.len() {
                let item = &section_items[line - 1];
                items.push_str(generate_img_tag(item.as_str().unwrap()).as_str());
            }
            items.push_str("|");
        });
        items.push_str("\n");
    }
    items
}

fn generate_img_tag(slug: &str) -> String {
    let icons = &SimpleIcons::global().icons;
    let icon = icons
        .get(slug)
        .expect(format!("Could not find icon for slug {slug}").as_str());
    let title = &icon.title;
    let hex = &icon.hex;
    let background = if is_relatively_light_icon_hex(&hex) {
        "black"
    } else {
        "white"
    };
    format!("[<img align=\"left\" alt=\"{title}\" src=\"https://img.shields.io/badge/-{title}-{hex}?logo={slug}&logoColor={background}\">](#)")
}

#[cfg(test)]
mod tests {
    use crate::generate_toolbox;

    #[test]
    fn should_generate_markdown() {
        let input = "
sections:
  ides:
    - jetbrains
    - neovim
  languages:
    - javascript
    - rust";
        let markdown = generate_toolbox(&input.to_string());
        assert_eq!(
            markdown,
            "|ides|languages|
|-|-|
|[<img align=\"left\" alt=\"JetBrains\" src=\"https://img.shields.io/badge/-JetBrains-000000?logo=jetbrains&logoColor=white\">](#)|[<img align=\"left\" alt=\"JavaScript\" src=\"https://img.shields.io/badge/-JavaScript-F7DF1E?logo=javascript&logoColor=black\">](#)|
|[<img align=\"left\" alt=\"Neovim\" src=\"https://img.shields.io/badge/-Neovim-57A143?logo=neovim&logoColor=white\">](#)|[<img align=\"left\" alt=\"Rust\" src=\"https://img.shields.io/badge/-Rust-000000?logo=rust&logoColor=white\">](#)|
"
        );
    }

    #[test]
    fn should_handle_sections_with_different_number_of_items() {
        let input = "
sections:
  ides:
    - neovim
  languages:
    - javascript
    - rust";
        let markdown = generate_toolbox(&input.to_string());
        assert_eq!(
            markdown,
            "|ides|languages|
|-|-|
|[<img align=\"left\" alt=\"Neovim\" src=\"https://img.shields.io/badge/-Neovim-57A143?logo=neovim&logoColor=white\">](#)|[<img align=\"left\" alt=\"JavaScript\" src=\"https://img.shields.io/badge/-JavaScript-F7DF1E?logo=javascript&logoColor=black\">](#)|
||[<img align=\"left\" alt=\"Rust\" src=\"https://img.shields.io/badge/-Rust-000000?logo=rust&logoColor=white\">](#)|
"
        );
    }
}
