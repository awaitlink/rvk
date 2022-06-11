use super::*;

/// Undocumented
#[derive(Deserialize, Clone, Debug)]
pub struct Podcast {
    pub url: String,
    pub title: String,
    pub access_key: Option<String>,
}
