//! Represents various objects that are returned as JSON by the API
//!
//! ## Note: `type` fields
//!
//! Since `type` is a Rust keyword, all `type` fields are named `type_` (**with the underscore!**)

/// The 'integer' type that is used in objects
pub type Integer = i64;

/// The 'number' type that is used in objects
pub type Number = f64;

/// The 'boolean' type that is used in objects
pub type Boolean = bool;

pub mod app;
pub mod app_widget;
pub mod attachment;
pub mod audio;
pub mod button;
pub mod comment;
pub mod comment_board;
pub mod conversation;
pub mod document;
pub mod geo;
pub mod gift;
pub mod group;
pub mod link;
pub mod market_album;
pub mod market_item;
pub mod message;
pub mod note;
pub mod page;
pub mod photo;
pub mod poll;
pub mod post;
pub mod post_source;
pub mod privacy;
pub mod push_settings;
pub mod stats;
pub mod sticker;
pub mod topic;
pub mod user;
pub mod video;
