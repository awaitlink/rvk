# `rvk`
[![version][badges/version]][crates.io/rvk]
[![downloads][badges/downloads]][crates.io/rvk]
[![license][badges/license]][license]

A crate for accessing VK API in Rust (synchronously).

# Modules

- [`api`][modules/api] - works with the API;
- [`error`][modules/error] - handles errors that may occur during an API call;
- [`methods`][modules/methods] - contains **API [methods][vk/methods]**;
- [`objects`][modules/objects] - contains **API [objects][vk/objects]**.

# Usage
Add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
rvk = "0.4"
```

Also, add this line to your crate's `main.rs` or `lib.rs`:

```rust
extern crate rvk;
```

Now you can take a look at `rvk`'s [API documentation][docs.rs/rvk] to learn more about the available functions.

# Example

To use this example, you will **also** need the [`serde_json`][crates.io/serde_json] crate to deserialize the API response:

```toml
[dependencies]
serde_json = "1.0"
```

```rust
extern crate rvk;
extern crate serde_json;

use rvk::{methods::*, objects::user::User, APIClient, Params};
use serde_json::from_value;

fn main() {
    let api = APIClient::new("your_access_token".into()); // Create an API Client

    let mut params = Params::new(); // Create a HashMap to store parameters
    params.insert("user_ids".into(), "1".into());

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

[crates.io/rvk]: https://crates.io/crates/rvk
[crates.io/serde_json]: https://crates.io/crates/serde_json

[docs.rs/rvk]: https://docs.rs/rvk

[license]: https://github.com/u32i64/rvk/blob/master/LICENSE

[badges/version]: https://img.shields.io/crates/v/rvk.svg?style=for-the-badge
[badges/downloads]: https://img.shields.io/crates/d/rvk.svg?style=for-the-badge
[badges/license]: https://img.shields.io/crates/l/rvk.svg?style=for-the-badge

[modules/api]: https://docs.rs/rvk/*/rvk/api/index.html
[modules/error]: https://docs.rs/rvk/*/rvk/error/index.html
[modules/methods]: https://docs.rs/rvk/*/rvk/methods/index.html
[modules/objects]: https://docs.rs/rvk/*/rvk/objects/index.html

[vk/methods]: https://vk.com/dev/methods
[vk/objects]: https://vk.com/dev/objects