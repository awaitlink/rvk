use super::*;

/// <https://vk.com/dev/objects/note>
#[derive(Deserialize, Clone, Debug)]
pub struct Note {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub text: String,
    pub date: Integer,
    pub comments: Integer,
    pub read_comments: Option<Integer>,
    pub view_url: String,
}
