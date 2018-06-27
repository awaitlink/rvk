use super::*;

/// <https://vk.com/dev/objects/attachments_w>
#[derive(Deserialize, Clone, Debug)]
pub struct WallAttachment {
    #[serde(rename = "type")]
    pub type_: String,

    // type = photo
    pub photo: Option<photo::Photo>,

    // type = posted_photo
    pub posted_photo: Option<PostedPhoto>,

    // type = video
    pub video: Option<video::Video>,

    // type = audio
    pub audio: Option<audio::Audio>,

    // type = doc
    pub doc: Option<document::Document>,

    // type = graffiti
    pub graffiti: Option<Graffiti>,

    // type = link
    pub link: Option<link::Link>,

    // type = note
    // TODO: pub note: Option<?>,

    // type = app
    pub app: Option<App>,

    // type = poll
    pub poll: Option<poll::Poll>,

    // type = page
    // TODO: pub page: Option<?>,

    // type = album
    pub album: Option<photo::Album>,

    // type = photos_list
    pub photos_list: Option<Vec<String>>,

    // type = market
    // TODO: pub market: Option<?>,

    // type = market_album
    // TODO: pub market_album: Option<?>,

    // type = sticker
    pub sticker: Option<sticker::Sticker>,

    // type = pretty_cards
    pub cards: Option<Vec<Card>>,
}

/// For posts created before 2013
#[derive(Deserialize, Clone, Debug)]
pub struct PostedPhoto {
    pub id: Integer,
    pub owner_id: Integer,
    pub photo_130: String,
    pub photo_604: String,

    /// Access key may be present in attachments
    /// (
    /// <https://vk.com/dev/objects/attachments_w>
    /// or
    /// <https://vk.com/dev/objects/attachments_m>
    /// )
    pub access_key: Option<String>,
}

/// For posts created before 2013
#[derive(Deserialize, Clone, Debug)]
pub struct Graffiti {
    pub id: Integer,
    pub owner_id: Integer,
    pub photo_130: String,
    pub photo_604: String,

    /// Access key may be present in attachments
    /// (
    /// <https://vk.com/dev/objects/attachments_w>
    /// or
    /// <https://vk.com/dev/objects/attachments_m>
    /// )
    pub access_key: Option<String>,
}

/// For posts created before 2013
#[derive(Deserialize, Clone, Debug)]
pub struct App {
    pub id: Integer,
    pub name: String,
    pub photo_130: String,
    pub photo_604: String,

    /// Access key may be present in attachments
    /// (
    /// <https://vk.com/dev/objects/attachments_w>
    /// or
    /// <https://vk.com/dev/objects/attachments_m>
    /// )
    pub access_key: Option<String>,
}

///
#[derive(Deserialize, Clone, Debug)]
pub struct Card {
    pub card_id: String,
    pub link_url: String,
    pub title: String,
    pub images: Vec<photo::Image>,
    pub button: button::Button,
    pub price: String,
    pub price_old: Option<String>,
}

/// <https://vk.com/dev/objects/attachments_m>
#[derive(Deserialize, Clone, Debug)]
pub struct MessageAttachment {
    // TODO: Implement
}
