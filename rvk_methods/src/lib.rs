#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]
#![doc = include_str!("../README.md")]

use rvk::APIClient;

/// Defines the version of VK API that is used by default and generally considered "supported" by this version of the crate.
pub const API_VERSION: &str = "5.103";

/// Convenience function to create a new `APIClient`
/// with the API version that is supported by this crate
/// (as indicated by [`API_VERSION`]), given an access token.
///
/// See [`rvk::APIClient::new`] for further information including potential panics.
pub fn supported_api_client(token: impl Into<String>) -> APIClient {
    APIClient::new(API_VERSION, token)
}

macro_rules! api_category {
    ($category:expr; methods { $($name:ident),* }) => {
        use heck::MixedCase;
        use std::collections::HashMap;
        use lazy_static::lazy_static;
        const CATEGORY: &str = $category;

        lazy_static! {
            static ref METHOD_NAMES: HashMap<&'static str, String> = {
                let mut m = HashMap::new();

                $(
                    m.insert(stringify!($name), CATEGORY.to_owned() + "." + &stringify!($name).to_mixed_case());
                )*

                m
            };
        }

        $(
            api_method!(
                $name,
                METHOD_NAMES
                    .get(stringify!($name))
                    .expect(&format!("No method with name {} found in METHOD_NAMES.
This is a bug.
Please report it at <https://github.com/u32i64/rvk>", stringify!($name)))
            );
        )*
    };
}

macro_rules! api_method {
    ($func_name:ident, $method_name:expr) => {
        /// Calls the corresponding VK API method.
        pub async fn $func_name<T: serde::de::DeserializeOwned>(
            api: &rvk::api::APIClient,
            params: rvk::Params,
        ) -> rvk::error::Result<T> {
            api.call_method::<T>($method_name, params).await
        }
    };
}

api_method!(execute, "execute");

pub mod account;
pub mod ads;
pub mod app_widgets;
pub mod apps;
pub mod auth;
pub mod board;
pub mod database;
pub mod docs;
pub mod fave;
pub mod friends;
pub mod gifts;
pub mod groups;
pub mod leads;
pub mod likes;
pub mod market;
pub mod messages;
pub mod newsfeed;
pub mod notes;
pub mod notifications;
pub mod orders;
pub mod pages;
pub mod photos;
pub mod places;
pub mod polls;
pub mod search;
pub mod secure;
pub mod stats;
pub mod status;
pub mod storage;
pub mod stories;
pub mod streaming;
pub mod users;
pub mod utils;
pub mod video;
pub mod wall;
pub mod widgets;
