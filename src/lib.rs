#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]

//! # Overview
//! This is a crate for accessing VK API (synchronously).
//!
//! It consists of:
//!
//! - [`api`](api/index.html) **module**, which works with the API;
//! - [`error`](error/index.html) **module**, which handles errors that may occur during an API call;
//! - [`methods`](methods/index.html) **module**, which contains **API [methods](https://vk.com/dev/methods)**;
//! - [`objects`](objects/index.html) **module**, which contains **API [objects](https://vk.com/dev/objects)**,
//!
//! # Example
//! ```no_run
//! extern crate rvk;
//! extern crate serde_json;
//!
//! use rvk::{methods::*, objects::user::User, APIClient, Params};
//! use serde_json::from_value;
//!
//! fn main() {
//!     let mut api = APIClient::new("your_access_token".into()); // Create an API Client
//!
//!     let mut params = Params::new(); // Create a HashMap to store parameters
//!     params.insert("user_ids".into(), "1".into());
//!
//!     let res = users::get(&api, params);
//!
//!     match res {
//!         Ok(v) => { // v is `serde_json::Value`
//!             let users: Vec<User> = from_value(v).unwrap();
//!             let user = &users[0];
//!
//!             println!(
//!                 "User #{} is {} {}.",
//!                 user.id, user.first_name, user.last_name
//!             );
//!         }
//!         Err(e) => println!("{}", e),
//!     };
//! }
//! ```

pub mod api;
pub mod error;
pub mod methods;
pub mod objects;

pub use crate::api::{APIClient, Params};

/// Defines the version of VK API that is used.
pub const API_VERSION: &str = "5.92";
