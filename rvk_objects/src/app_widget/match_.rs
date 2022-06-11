//! <https://vk.com/dev/objects/appWidget_2?f=7.%20Match>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Match {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,

    #[serde(rename = "match")]
    pub match_: MatchInformation,

    pub more: Option<String>,
    pub more_url: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MatchInformation {
    pub state: String,

    /// Used in <https://vk.com/dev/objects/appWidget_2?f=8.%20Matches>
    pub live_url: Option<String>,

    /// Used in <https://vk.com/dev/objects/appWidget_2?f=8.%20Matches>
    pub url: Option<String>,

    pub team_a: TeamInformation,
    pub team_b: TeamInformation,
    pub score: Score,
    pub events: Events,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TeamInformation {
    pub name: String,
    pub descr: String,
    pub icon_id: Option<String>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Score {
    pub team_a: Integer,
    pub team_b: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Events {
    pub team_a: Vec<Event>,
    pub team_b: Vec<Event>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Event {
    pub event: String,
    pub minute: Integer,
}
