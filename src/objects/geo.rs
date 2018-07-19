use super::*;

/// <https://vk.com/dev/objects/geo>
#[derive(Deserialize, Clone, Debug)]
pub struct Geo {
    #[serde(rename = "type")]
    pub type_: String,

    pub coordinates: Coordinates,
    pub place: Option<Place>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Coordinates {
    pub latitude: Number,
    pub longitude: Number,
}

#[derive(Deserialize, Clone, Debug)]
pub struct City {
    pub id: Option<Integer>,
    pub title: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Country {
    pub id: Option<Integer>,
    pub title: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Place {
    pub id: Option<Integer>,
    pub title: Option<String>,
    pub latitude: Number,
    pub longitude: Number,
    pub created: Option<Integer>,
    pub icon: String,
    pub country: String,
    pub city: String,

    // Optional
    #[serde(rename = "type")]
    pub type_: Option<String>,

    pub group_id: Option<Integer>,
    pub group_photo: Option<String>,
    pub checkins: Option<Integer>,
    pub updated: Option<Integer>,
    pub address: Option<Integer>,
}
