/**
* Code imported from
* https://github.com/mondeja/simple-icons-website-rs/blob/master/simple-icons/src/sdk/mod.rs
*/
pub mod color;
mod data;
mod slug;

use crate::simple_icons::data::fetch_simple_icons_data;
use crate::simple_icons::slug::title_to_slug;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SimpleIcons {
    pub icons: HashMap<String, SimpleIcon>,
}

#[derive(Debug)]
pub struct SimpleIcon {
    pub title: String,
    pub hex: String,
}

static SIMPLE_ICONS_INSTANCE: OnceCell<SimpleIcons> = OnceCell::new();
impl SimpleIcons {
    pub fn global() -> &'static SimpleIcons {
        SIMPLE_ICONS_INSTANCE.get_or_init(|| SimpleIcons {
            icons: init_simple_icons(None),
        })
    }
}

fn init_simple_icons(max_icons: Option<usize>) -> HashMap<String, SimpleIcon> {
    let simple_icons_data = fetch_simple_icons_data();
    let mut simple_icons: HashMap<String, SimpleIcon> =
        HashMap::with_capacity(simple_icons_data.icons.len());

    for icon_data in simple_icons_data.icons {
        let slug = match icon_data.slug {
            Some(slug) => slug,
            None => title_to_slug(&icon_data.title),
        };
        let icon = SimpleIcon {
            title: icon_data.title,
            hex: icon_data.hex,
        };
        simple_icons.insert(slug, icon);
        if let Some(max_icons) = max_icons {
            if simple_icons.len() == max_icons {
                break;
            }
        }
    }

    simple_icons
}
