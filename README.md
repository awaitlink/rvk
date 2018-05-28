# `rvk`
[![version](https://img.shields.io/crates/v/rvk.svg?style=flat-square)](https://crates.io/crates/rvk)
[![downloads](https://img.shields.io/crates/d/rvk.svg?style=flat-square)](https://crates.io/crates/rvk)
[![license](https://img.shields.io/crates/l/rvk.svg?style=flat-square)](https://github.com/u32i64/rvk/blob/master/LICENSE)

A crate for accessing VK API in Rust.

# Usage
Add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
rvk = "0.1"
```

Also, add this line to your crate:

```rust
extern crate rvk;
```

Now you can take a look at this crate's [API documentation](https://docs.rs/rvk) to learn more about the available functions.

# Example

```rust
extern crate rvk;
use rvk::{APIClient, Params, methods::*};

fn main() {
    let mut api = APIClient::new("your_access_token").unwrap(); // Create an API Client

    let mut params = Params::new(); // Create a HashMap to store parameters
    params.insert("user_ids", "1");

    let work = users::get(&api, params, |res| match res {
        Ok(v) => { // If the API returned a response, you get `serde_json::Value` here

            // In this example, `v` corresponds to this JSON:
            // [
            //   {
            //     "id": 1,
            //     "first_name": "Pavel",
            //     "last_name": "Durov"
            //   }
            // ]

            let user = v.as_array().unwrap().get(0).unwrap();

            let first_name = user.get("first_name").unwrap().as_str().unwrap();
            let last_name = user.get("last_name").unwrap().as_str().unwrap();
            let id = user.get("id").unwrap().as_u64().unwrap();

            println!("Success: User #{} is {} {}.", id, first_name, last_name);
        }
        Err(e) => println!("Error {}: {}", e.code(), e.msg()),
    }); // This returns a Future

    api.run(work); // Do not forget to run the Future to make it actually do something!
}
```
