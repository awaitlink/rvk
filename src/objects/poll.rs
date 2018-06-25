use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Poll {
    pub id: Integer,
    pub owner_id: Integer,
    pub created: Integer,
    pub question: String,
    pub votes: Integer,
    pub answer_id: Integer,
    pub answers: Vec<PollAnswer>,
    pub anonymous: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PollAnswer {
    pub id: Integer,
    pub text: String,
    pub votes: Integer,
    pub rate: Number,
}