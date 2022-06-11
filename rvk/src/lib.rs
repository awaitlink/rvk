#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]
#![doc = include_str!("../README.md")]

pub mod api;
pub mod error;

pub use crate::api::{APIClient, Params};
