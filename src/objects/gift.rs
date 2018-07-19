use super::*;

/// <https://vk.com/dev/objects/gift>
#[derive(Deserialize, Clone, Debug)]
pub struct Gift {
    pub id: Integer,
    pub thumb_256: String,
    pub thumb_96: String,
    pub thumb_48: String,
}
