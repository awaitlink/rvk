//! <https://vk.com/dev/objects/appWidget_2?f=6.%20Cover%20List>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct CoverList {
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
    pub button: Option<String>,
    pub cover_id: String,
    pub button_url: Option<String>,
    pub url: Option<String>,
    pub descr: Option<String>,
}
