use super::*;

/// <https://vk.com/dev/objects/group>
#[derive(Deserialize, Clone, Debug)]
pub struct Group {
    // Main fields
    pub id: Integer,
    pub name: String,
    pub screen_name: String,
    pub is_closed: Integer,
    pub deactivated: String,
    pub is_admin: Option<Integer>,
    pub admin_level: Option<Integer>,
    pub is_member: Option<Integer>,
    pub invited_by: Option<Integer>,

    #[serde(rename = "type")]
    pub type_: String,

    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String,

    // Optional fields
    pub activity: Option<String>,
    pub age_limits: Option<Integer>,
    pub ban_info: Option<BlacklistInfo>,
    pub can_create_topic: Option<Integer>,
    pub can_message: Option<Integer>,
    pub can_post: Option<Integer>,
    pub can_see_all_posts: Option<Integer>,
    pub can_upload_doc: Option<Integer>,
    pub can_upload_video: Option<Integer>,
    pub city: Option<geo::City>,
    pub contacts: Option<Vec<Contact>>,
    pub counters: Option<Counters>,
    pub country: Option<geo::Country>,
    pub cover: Option<Cover>,
    pub crop_photo: Option<photo::Cropped>,
    pub description: Option<String>,
    pub fixed_post: Option<Integer>,
    pub has_photo: Option<Integer>,
    pub is_favorite: Option<Integer>,
    pub is_hidden_from_feed: Option<Integer>,
    pub is_messages_blocked: Option<Integer>,
    pub links: Option<Vec<Link>>,
    pub main_album_id: Option<Integer>,
    pub main_section: Option<Integer>,
    pub market: Option<Market>,
    pub member_status: Option<Integer>,
    pub place: Option<geo::Place>,
    pub public_date_label: Option<String>,
    pub site: Option<String>,
    pub start_date: Option<Integer>,
    pub finish_date: Option<Integer>,
    pub status: Option<String>,
    pub trending: Option<Integer>,
    pub verified: Option<Integer>,
    pub wall: Option<Integer>,
    pub wiki_page: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlacklistInfo {
    pub end_date: Integer,
    pub comment: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Contact {
    pub user_id: Option<Integer>,
    pub desc: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Counters {
    pub photos: Option<Integer>,
    pub albums: Option<Integer>,
    pub audios: Option<Integer>,
    pub videos: Option<Integer>,
    pub topics: Option<Integer>,
    pub docs: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Cover {
    pub enabled: Integer,
    pub images: Vec<photo::Image>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Link {
    pub id: Integer,
    pub url: String,
    pub name: String,
    pub desc: String,
    pub photo_50: String,
    pub photo_100: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Market {
    pub enabled: Integer,
    pub price_min: Option<Integer>,
    pub price_max: Option<Integer>,
    pub main_album_id: Option<Integer>,
    pub contact_id: Option<Integer>,
    pub currency: Option<link::Price>,
    pub currency_text: Option<String>,
}
