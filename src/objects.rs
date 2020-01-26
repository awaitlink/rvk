//! Represents various objects that are returned as JSON by the API.
//!
//! ## Note: `type` and `ref` fields
//!
//! Since `type` and `ref` are Rust keywords, an underscore (`_`) is added at the end of these:
//! - `type` **->** `type_`,
//! - `ref` **->** `ref_`.
//!
//! ## Note
//!
//! Due to the nature of the VK API documentation, it is not always clear if the value is always passed or not, and sometimes the data type is not defined.
//!
//! If you spot any mistakes or bugs, please [report them](https://github.com/u32i64/rvk/issues)!

use serde_derive::Deserialize;

/// The 'integer' type that is used in objects.
pub type Integer = i64;

/// The 'number' type that is used in objects.
pub type Number = f64;

/// The 'boolean' type that is used in objects.
pub type Boolean = bool;

pub mod app;
pub mod app_widget;
pub mod attachment;
pub mod audio;
pub mod button;
pub mod clickable_stickers;
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
pub mod story;
pub mod topic;
pub mod user;
pub mod video;
