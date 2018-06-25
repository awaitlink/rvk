#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]

//! # Overview
//! This is a crate for accessing VK API (synchronously).
//!
//! It consists of:
//!
//! - **The** [`api`](api/index.html) **module**, which works with the API;
//! - **API [methods](https://vk.com/dev/methods)** (in the
//! [`methods`](methods/index.html) module);
//! - **API [objects](https://vk.com/dev/objects)** (in the
//! [`objects`](objects/index.html) module),
//!
//! which collectively make accessing the VK API a lot easier, as shown in the example below.
//!
//! # Example
//! ```no_run
//! extern crate rvk;
//! extern crate serde_json;
//!
//! use rvk::{APIClient, Params, methods::*, objects};
//! use serde_json::from_value;
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
//!             let users: Vec<objects::User> = from_value(v).unwrap();
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
pub mod objects;

pub use api::APIClient;
pub use api::Params;

/// Defines the version of VK API that is used
pub const API_VERSION: &str = "5.80";
