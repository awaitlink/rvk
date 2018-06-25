use super::*;

// TODO: Option<>

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    // Main fields
    pub id: Integer,
    pub first_name: String,
    pub last_name: String,
    pub deactivated: Option<String>,

    // Optional fields A-L
    pub about: Option<String>,
    pub activities: Option<String>,
    pub bdate: Option<String>,
    pub blacklisted: Option<Integer>,
    pub blacklisted_by_me: Option<Integer>,
    pub books: Option<String>,
    pub can_post: Option<Integer>,
    pub can_see_all_posts: Option<Integer>,
    pub can_see_audio: Option<Integer>,
    pub can_send_friend_request: Option<Integer>,
    pub can_write_private_message: Option<Integer>,
    pub career: Option<Vec<Career>>,
    pub city: Option<City>,
    pub common_count: Option<Integer>,

    // connections
    pub skype: Option<String>,
    pub facebook: Option<String>,
    pub facebook_name: Option<String>,
    pub twitter: Option<String>,
    pub livejournal: Option<String>,
    pub instagram: Option<String>,

    pub contacts: Option<Contacts>,
    pub counters: Option<Counters>,
    pub country: Option<Country>,
    pub crop_photo: Option<photo::Cropped>,
    pub domain: Option<String>,

    // education
    pub university: Option<Integer>,
    pub university_name: Option<String>,
    pub faculty: Option<Integer>,
    pub faculty_name: Option<String>,
    pub graduation: Option<Integer>,

    pub exports: Option<Exports>,

    // first_name_{case}
    pub first_name_nom: Option<String>,
    pub first_name_gen: Option<String>,
    pub first_name_dat: Option<String>,
    pub first_name_acc: Option<String>,
    pub first_name_ins: Option<String>,
    pub first_name_abl: Option<String>,

    pub followers_count: Option<Integer>,
    pub friend_status: Option<Integer>,
    pub games: Option<String>,
    pub has_mobile: Option<Integer>,
    pub has_photo: Option<Integer>,
    pub home_town: Option<String>,
    pub interests: Option<String>,
    pub is_favorite: Option<Integer>,
    pub is_friend: Option<Integer>,
    pub is_hidden_from_feed: Option<Integer>,

    // last_name_{case}
    pub last_name_nom: Option<String>,
    pub last_name_gen: Option<String>,
    pub last_name_dat: Option<String>,
    pub last_name_acc: Option<String>,
    pub last_name_ins: Option<String>,
    pub last_name_abl: Option<String>,

    pub last_seen: Option<LastSeen>,
    pub lists: Option<String>,

    // Optional fields M-W
    pub maiden_name: Option<String>,
    pub military: Option<Vec<Military>>,
    pub movies: Option<String>,
    pub music: Option<String>,
    pub nickname: Option<String>,
    pub occupation: Option<Occupation>,

    // online
    pub online: Option<Integer>,
    pub online_mobile: Option<Integer>,
    pub online_app: Option<String>,

    pub personal: Option<Personal>,
    pub photo_50: Option<String>,
    pub photo_100: Option<String>,
    pub photo_200_orig: Option<String>,
    pub photo_200: Option<String>,
    pub photo_400_orig: Option<String>,
    pub photo_id: Option<String>,
    pub photo_max: Option<String>,
    pub photo_max_orig: Option<String>,
    pub quotes: Option<String>,
    pub relatives: Option<Vec<Relative>>,

    // relation
    pub relation: Option<Integer>,
    pub relation_partner: Option<Box<User>>,

    pub schools: Option<Vec<School>>,
    pub screen_name: Option<String>,
    pub sex: Option<Integer>,
    pub site: Option<String>,

    // status
    pub status: Option<String>,
    pub status_audio: Option<audio::Audio>,

    pub timezone: Option<Integer>,
    pub trending: Option<Integer>,
    pub tv: Option<String>,
    pub universities: Option<Vec<University>>,
    pub verified: Option<Integer>,
    pub wall_default: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Career {
    pub group_id: Option<Integer>,
    pub company: Option<String>,
    pub country_id: Option<Integer>,
    pub city_id: Option<Integer>,
    pub city_name: Option<String>,
    pub from: Option<Integer>,
    pub until: Option<Integer>,
    pub position: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct City {
    pub id: Option<Integer>,
    pub title: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Contacts {
    pub mobile_phone: Option<String>,
    pub home_phone: Option<String>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Counters {
    pub albums: Option<Integer>,
    pub videos: Option<Integer>,
    pub audios: Option<Integer>,
    pub photos: Option<Integer>,
    pub notes: Option<Integer>,
    pub friends: Option<Integer>,
    pub groups: Option<Integer>,
    pub online_friends: Option<Integer>,
    pub mutual_friends: Option<Integer>,
    pub user_videos: Option<Integer>,
    pub followers: Option<Integer>,
    pub pages: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Country {
    pub id: Option<Integer>,
    pub title: Option<String>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Exports {
    pub twitter: Option<Integer>,
    pub facebook: Option<Integer>,
    pub livejournal: Option<Integer>,
    pub instagram: Option<Integer>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct LastSeen {
    pub time: Option<Integer>,
    pub platform: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Military {
    pub unit: Option<String>,
    pub unit_id: Option<Integer>,
    pub country_id: Option<Integer>,
    pub from: Option<Integer>,
    pub until: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Occupation {
    #[serde(rename = "type")]
    pub type_: Option<String>,

    pub id: Option<Integer>,
    pub name: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Personal {
    pub political: Option<Integer>,
    pub langs: Option<Vec<String>>,
    pub religion: Option<String>,
    pub inspired_by: Option<String>,
    pub people_main: Option<Integer>,
    pub life_main: Option<Integer>,
    pub smoking: Option<Integer>,
    pub alcohol: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Relative {
    pub id: Option<Integer>,
    pub name: Option<String>,

    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct School {
    pub id: Option<String>,
    pub country: Option<Integer>,
    pub city: Option<Integer>,
    pub name: Option<String>,
    pub year_from: Option<Integer>,
    pub year_to: Option<Integer>,
    pub year_graduated: Option<Integer>,
    pub class: Option<String>,
    pub speciality: Option<String>,

    #[serde(rename = "type")]
    pub type_: Option<Integer>,

    pub type_str: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct University {
    pub id: Option<Integer>,
    pub country: Option<Integer>,
    pub city: Option<Integer>,
    pub name: Option<String>,
    pub faculty: Option<Integer>,
    pub faculty_name: Option<String>,
    pub chair: Option<Integer>,
    pub chair_name: Option<String>,
    pub graduation: Option<Integer>,
    pub education_form: Option<String>,
    pub education_status: Option<String>,
}