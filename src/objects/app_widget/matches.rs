//! <https://vk.com/dev/objects/appWidget_2?f=8.%20Matches>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Matches {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,
    pub matches: Vec<match_::MatchInformation>,
    pub more: Option<String>,
    pub more_url: Option<String>,
}
