use super::*;

/// <https://vk.com/dev/objects/conversation>
#[derive(Deserialize, Clone, Debug)]
pub struct Conversation {
    pub peer: Peer,
    pub in_read: Integer,
    pub out_read: Integer,
    pub unread_count: Integer,
    pub important: Boolean,
    pub unanswered: Boolean,
    pub push_settings: PushSettings,
    pub can_write: WritePermission,
    pub chat_settings: ChatSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Peer {
    pub id: Integer,

    #[serde(rename = "type")]
    pub type_: Integer,

    pub local_id: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PushSettings {
    pub disabled_until: Integer,
    pub disabled_forever: Option<Boolean>,
    pub no_sound: Option<Boolean>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WritePermission {
    pub allowed: Boolean,
    pub reason: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChatSettings {
    pub members_count: Integer,
    pub title: String,
    pub pinned_message: Option<message::Pinned>,
    pub state: String,
    pub photo: message::Photo,
    pub active_ids: Vec<Integer>,
    pub is_group_channel: Boolean,
}
