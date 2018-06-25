//! Represents various objects that are returned as JSON by the API

/// The 'integer' type that is used in all objects
pub type Integer = i64;

/// The 'number' type that is used in all objects
pub type Number = f64;

/// The 'boolean' type that is used in all objects
pub type Boolean = bool;

pub mod user;
pub mod message;
pub mod poll;
pub mod photo;
pub mod audio;
pub mod video;
pub mod document;
pub mod attachment;
pub mod post_source;
pub mod privacy;
pub mod push_settings;