use super::*;

/// <https://vk.com/dev/objects/audio>
#[derive(Deserialize, Clone, Debug)]
pub struct Audio {
    pub id: Integer,
    pub owner_id: Integer,
    pub artist: String,
    pub title: String,
    pub duration: Integer,
    pub url: String,
    pub lyrics_id: Option<Integer>,
    pub album_id: Option<Integer>,
    pub genre_id: Integer,
    pub date: Integer,
    pub no_search: Option<Integer>,
    pub is_hq: Option<Integer>,
}
