#![doc(html_logo_url = "https://raw.githubusercontent.com/u32i64/rvk/master/logo.png")]

//! # Overview
//! This is a crate for accessing VK API.
//!
//! All of the API [methods](https://vk.com/dev/methods) are located in the `methods`
//! [module](#modules) of this crate (in the corresponding submodules).
//!
//! # Example
//! ```no_run
//! extern crate rvk;
//! use rvk::{APIClient, Params, methods::*};
//!
//! fn main() {
//!     let mut api = APIClient::new("your_access_token").unwrap(); // Create an API Client
//!
//!     let mut params = Params::new(); // Create a HashMap to store parameters
//!     params.insert("user_ids", "1");
//!
//!     let res = users::get(&api, params).expect("Error happened during request");
//! 
//!     match res {
//!         Ok(v) => { // v is `serde_json::Value`
//!
//!             // In this example, `v` corresponds to this JSON:
//!             // [
//!             //   {
//!             //     "id": 1,
//!             //     "first_name": "Pavel",
//!             //     "last_name": "Durov"
//!             //   }
//!             // ]
//!
//!             let user = v.as_array().unwrap().get(0).unwrap();
//! 
//!             let first_name = user.get("first_name").unwrap().as_str().unwrap();
//!             let last_name = user.get("last_name").unwrap().as_str().unwrap();
//!             let id = user.get("id").unwrap().as_u64().unwrap();
//! 
//!             println!("User #{} is {} {}.", id, first_name, last_name);
//!         }
//!         Err(e) => println!("API Error {}: {}", e.code(), e.msg()),
//!     };
//! }
//! ```

extern crate heck;
extern crate reqwest;
extern crate serde_json;

pub mod client;
pub mod methods;

pub use client::APIClient;

/// A HashMap which contains method parameters
pub type Params<'a> = std::collections::HashMap<&'a str, &'a str>;
