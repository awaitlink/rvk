use super::*;

/// <https://vk.com/dev/objects/market_album>
#[derive(Deserialize, Clone, Debug)]
pub struct MarketAlbum {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub photo: photo::Photo,
    pub count: Integer,
    pub updated_time: Integer,
}
