//! <https://vk.com/dev/objects/appWidget?f=1.%20Text>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Text {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,
    pub text: Option<String>,
    pub descr: Option<String>,
    pub more: Option<String>,
    pub more_url: Option<String>,
}
