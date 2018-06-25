//! Represents various objects that are returned as JSON by the API

/// The 'integer' type that is used in all objects
pub type Integer = i64;

/// The 'number' type that is used in all objects
pub type Number = f64;

// Main Objects
pub mod user;
pub mod poll;

// Media Objects
pub mod photo;
pub mod audio;
pub mod video;
pub mod document;

// Extra Objects
pub mod privacy;