#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]

//! # Overview
//! This is a crate for accessing VK API (synchronously).
//!
//! All of the API [methods](https://vk.com/dev/methods) are located in the
//! [`methods`](methods/index.html) module of this crate (in the corresponding submodules).
//!
//! # Example
//! ```no_run
//! extern crate rvk;
//! use rvk::{APIClient, Params, methods::*};
//!
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! use serde_json::from_value;
//!
//! #[derive(Deserialize)]
//! struct User {
//!     id: u64,
//!     first_name: String,
//!     last_name: String,
//! }
//!
//! fn main() {
//!     let mut api = APIClient::new("your_access_token"); // Create an API Client
//!
//!     let mut params = Params::new(); // Create a HashMap to store parameters
//!     params.insert("user_ids", "1");
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

extern crate heck;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod api;
pub mod error;
pub mod methods;

pub use api::APIClient;

/// A HashMap which contains method parameters
pub type Params<'a> = std::collections::HashMap<&'a str, &'a str>;

/// Defines the version of VK API that is used
pub const API_VERSION: &str = "5.80";
