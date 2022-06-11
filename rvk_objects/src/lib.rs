#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]
#![doc = include_str!("../README.md")]

/// Defines the version of VK API that is generally considered to be "supported" by this version of the crate.
pub const API_VERSION: &str = "5.131";

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
pub mod podcast;
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
