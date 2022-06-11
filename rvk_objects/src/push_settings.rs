use super::*;

/// <https://vk.com/dev/objects/push_settings>
#[derive(Deserialize, Clone, Debug)]
pub struct PushSettings {
    pub msg: Option<String>,
    pub chat: Option<String>,
    pub friend: Option<String>,
    pub friend_found: Option<String>,
    pub friend_accepted: Option<String>,
    pub reply: Option<String>,
    pub comment: Option<String>,
    pub mention: Option<String>,
    pub like: Option<String>,
    pub repost: Option<String>,
    pub wall_post: Option<String>,
    pub wall_publish: Option<String>,
    pub group_invite: Option<String>,
    pub group_accepted: Option<String>,
    pub event_soon: Option<String>,
    pub tag_photo: Option<String>,
    pub app_request: Option<String>,
    pub sdk_open: Option<String>,
    pub new_post: Option<String>,
    pub birthday: Option<String>,
}
