use super::*;

/// <https://vk.com/dev/objects/story>
#[derive(Deserialize, Clone, Debug)]
pub struct Story {
    pub id: Integer,
    pub owner_id: Integer,
    pub date: Option<Integer>,
    pub expires_at: Option<Integer>,
    pub is_expired: Option<Boolean>,
    pub is_deleted: Option<Boolean>,
    pub can_see: Option<Integer>,
    pub seen: Option<Integer>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub photo: Option<photo::Photo>,
    pub video: Option<video::Video>,
    pub link: Option<Link>,
    pub parent_story_owner_id: Option<Integer>,
    pub parent_story_id: Option<Integer>,
    pub parent_story: Option<Box<Story>>,
    pub replies: Option<Replies>,
    pub can_reply: Option<Integer>,
    pub can_share: Option<Integer>,
    pub can_comment: Option<Integer>,
    pub clickable_stickers: Option<clickable_stickers::ClickableStickers>,
    pub views: Option<Integer>,
    pub access_key: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Replies {
    pub count: Integer,
    pub new: Option<Integer>,
}
