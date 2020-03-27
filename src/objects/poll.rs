use super::*;

/// <https://vk.com/dev/objects/poll>
#[derive(Deserialize, Clone, Debug)]
pub struct Poll {
    pub id: Integer,
    pub owner_id: Integer,
    pub created: Integer,
    pub question: String,
    pub votes: Integer,
    pub answers: Vec<Answer>,
    pub anonymous: Boolean,
    pub multiple: Boolean,
    pub answer_ids: Option<Vec<Integer>>,
    pub end_date: Integer,
    pub closed: Boolean,
    pub is_board: Boolean,
    pub can_edit: Boolean,
    pub can_vote: Boolean,
    pub can_report: Boolean,
    pub can_share: Boolean,
    pub author_id: Integer,
    pub photo: Option<photo::Photo>,
    pub background: Option<Background>,
    pub friends: Vec<PollFriend>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Answer {
    pub id: Integer,
    pub text: String,
    pub votes: Integer,
    pub rate: Number,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Background {
    pub id: Integer,

    #[serde(rename = "type")]
    pub type_: String,

    pub angle: Option<Integer>,
    pub color: String,
    pub width: Option<Integer>,
    pub height: Option<Integer>,
    pub images: Option<photo::Size>,
    pub points: Option<GradientPoint>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GradientPoint {
    pub position: Number,
    pub color: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PollFriend {
    pub id: Integer,
}
