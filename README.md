# NOTE: `add-objects` branch
Not all objects are added yet, and some may receive further tweaks before their release.

# `rvk`
[![version](https://img.shields.io/crates/v/rvk.svg?style=flat-square)](https://crates.io/crates/rvk)
[![downloads](https://img.shields.io/crates/d/rvk.svg?style=flat-square)](https://crates.io/crates/rvk)
[![license](https://img.shields.io/crates/l/rvk.svg?style=flat-square)](https://github.com/u32i64/rvk/blob/master/LICENSE)

A crate for accessing VK API in Rust (synchronously).

# Usage
Add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
rvk = "0.4"
```

Also, add this line to your crate:

```rust
extern crate rvk;
```

Now you can take a look at this crate's [API documentation](https://docs.rs/rvk) to learn more about the available functions.

# Example

To use this example, you will **also** need the `serde_json` crate to deserialize the API response:

```toml
[dependencies]
serde_json = "1.0"
```

```rust
extern crate rvk;
extern crate serde_json;

use rvk::objects::user::User;
use rvk::{methods::*, APIClient, Params};
use serde_json::from_value;

fn main() {
    let api = APIClient::new("your_access_token"); // Create an API Client

    let mut params = Params::new(); // Create a HashMap to store parameters
    params.insert("user_ids", "1");

    let res = users::get(&api, params);

    match res {
        Ok(v) => { // v is `serde_json::Value`
            let users: Vec<User> = from_value(v).unwrap();
            let user = &users[0];

            println!(
                "User #{} is {} {}.",
                user.id, user.first_name, user.last_name
            );
        }
        Err(e) => println!("{}", e),
    };
}
```
