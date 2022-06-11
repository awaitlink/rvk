//! <https://vk.com/dev/objects/appWidget_2?f=9.%20Donation>

use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Donation {
    pub title: String,
    pub title_url: Option<String>,
    pub title_counter: Option<Integer>,
    pub text: Option<String>,
    pub text_url: Option<String>,
    pub button_url: Option<String>,
    pub date: Date,
    pub goal: Integer,
    pub funded: Integer,
    pub backers: Integer,
    pub currency: String,
    pub more: Option<String>,
    pub more_url: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Date {
    pub start: Integer,
    pub end: Integer,
}
