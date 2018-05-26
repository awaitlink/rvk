//! Contains all of the API methods in the respective submodules

/// A macro which creates a function that calls a specified VK API method
macro_rules! api_method {
    ($func_name: ident, $method_name: expr) => (
        /// Calls a VK API method
        ///
        /// # General documentation for all methods
        ///
        /// ## Note about naming
        /// Rust (and this crate too) prefers `snake_case` in the function names instead of `camelCase` used by the VK API.
        ///
        /// **Example:** To call the `appWidgets.getAppImageUploadServer` API method, use the `rvk::methods::app_widgets::get_app_image_upload_server` function.
        ///
        /// ## Note about `photos.move`
        /// Since `move` is a Rust keyword, the function for calling `photos.move` method is `rvk::methods::photos::move_` (**with the underscore!**)
        pub fn $func_name<F>(api: &::client::APIClient, params: ::Params, then: F)
            -> impl ::futures::Future
            where
                F: Fn(::client::APIResponse)
        {
            api.call_method($method_name, params, then)
        }
    );
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
