//! <https://vk.com/dev/objects/appWidget?f=2.%20List>
//! and
//! <https://vk.com/dev/objects/appWidget_2?f=5.%2BCompact%2BList>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct List {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,
    pub rows: Vec<Element>,
    pub more: Option<String>,
    pub more_url: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Element {
    pub title: String,
    pub title_url: Option<String>,
    pub button: Option<String>,
    pub button_url: Option<String>,
    pub icon_id: Option<String>,
    pub descr: Option<String>,
    pub address: Option<String>,
    pub time: Option<String>,
    pub text: Option<String>,
}
