use super::*;

/// <https://vk.com/dev/objects/app>
#[derive(Deserialize, Clone, Debug)]
pub struct App {
    // Main fields
    pub id: Integer,
    pub title: String,
    pub icon_278: String,
    pub icon_139: String,
    pub icon_150: String,
    pub icon_75: String,
    pub banner_560: String,
    pub banner_1120: String,

    #[serde(rename = "type")]
    pub type_: String,

    pub section: String,
    pub author_url: String,
    pub author_id: Integer,
    pub author_group: Integer,
    pub members_count: Integer,
    pub published_date: Integer,
    pub catalog_position: Integer,
    pub international: Option<Integer>,
    pub leaderboard_type: Integer,
    pub genre_id: Integer,
    pub genre: String,
    pub platform_id: String,
    pub is_in_catalog: Integer,
    pub friends: Option<Vec<Integer>>,
    pub installed: Option<Integer>,
    pub screen_orientation: Integer,

    // Optional fields
    pub description: Option<String>,
    pub screen_name: Option<String>,
    pub icon_16: Option<String>,
    pub screenshots: Option<Vec<photo::Photo>>,
    pub push_enabled: Option<Integer>,
}
