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

    #[serde(rename = "ref")]
    pub ref_: Option<String>,

    pub ref_source: Option<Integer>,
    pub attachments: Option<Vec<attachment::MessageAttachment>>,
    pub important: Option<Boolean>,
    pub geo: Option<geo::Geo>,
    pub payload: Option<String>,
    pub fwd_messages: Option<Vec<Message>>,
    pub reply_message: Option<Box<Message>>,
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

/// <https://vk.com/dev/objects/pinned_message>
#[derive(Deserialize, Clone, Debug)]
pub struct Pinned {
    pub id: Integer,
    pub date: Integer,
    pub from_id: Integer,
    pub text: Option<String>,
    pub attachments: Option<Vec<attachment::MessageAttachment>>,
    pub geo: Option<geo::Geo>,
    pub fwd_messages: Option<Vec<Message>>,
}
