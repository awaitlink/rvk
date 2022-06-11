//! <https://vk.com/dev/objects/appWidget?f=4.%20Tiles>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Tiles {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,
    pub tiles: Vec<Tile>,
    pub more: Option<String>,
    pub more_url: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Tile {
    pub title: String,
    pub descr: Option<String>,
    pub url: String,
    pub link: Option<String>,
    pub link_url: Option<String>,
    pub icon_id: String,
}
