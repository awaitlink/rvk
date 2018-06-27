//! Represents various objects that are returned as JSON by the API

/// The 'integer' type that is used in objects
pub type Integer = i64;

/// The 'number' type that is used in objects
pub type Number = f64;

/// The 'boolean' type that is used in objects
pub type Boolean = bool;

pub mod attachment;
pub mod audio;
pub mod document;
pub mod message;
pub mod photo;
pub mod poll;
pub mod post_source;
pub mod privacy;
pub mod push_settings;
pub mod user;
pub mod video;
