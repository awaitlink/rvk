use super::*;

/// <https://vk.com/dev/objects/market_item>
#[derive(Deserialize, Clone, Debug)]
pub struct MarketItem {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub description: String,
    pub price: link::Price,
    pub category: Category,
    pub thumb_photo: String,
    pub date: Integer,
    pub availability: Integer,

    // extended
    pub photos: Option<Vec<photo::Photo>>,
    pub can_comment: Option<Integer>,
    pub can_repost: Option<Integer>,
    pub likes: Option<Likes>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Category {
    pub id: Integer,
    pub name: String,
    pub section: Section,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Section {
    pub id: Integer,
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Likes {
    pub user_likes: Integer,
    pub count: Integer,
}
