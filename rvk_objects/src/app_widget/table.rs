//! <https://vk.com/dev/objects/appWidget?f=3.%20Table>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Table {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,
    pub head: Vec<HeadObject>,
    pub body: Vec<Vec<Object>>,
    pub more: Option<String>,
    pub more_url: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HeadObject {
    pub text: String,
    pub align: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Object {
    pub text: String,
    pub url: String,
    pub icon_id: Option<String>,
}
