use super::*;

/// <https://vk.com/dev/objects/message>
#[derive(Deserialize, Clone, Debug)]
pub struct Message {
    pub id: Integer,
    pub date: Integer,
    pub peer_id: Integer,
    pub from_id: Integer,
    pub text: Option<String>,
    pub random_id: Option<Integer>,
    pub attachments: Option<Vec<attachment::MessageAttachment>>,
    pub important: Option<Boolean>,
    pub geo: Option<geo::Geo>,
    pub payload: Option<String>,
    pub fwd_messages: Option<Vec<Message>>,
    pub action: Option<Action>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_: String,

    pub member_id: Option<Integer>,
    pub text: Option<String>,
    pub email: Option<String>,
    pub photo: Option<Photo>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Photo {
    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String,
}
