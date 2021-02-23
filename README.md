# `rvk`
[![version][badges/version]][crates.io/rvk]
[![downloads][badges/downloads]][crates.io/rvk]
[![license][badges/license]][license]
[![api version][badges/api-version]][vk-api-version]

> A crate for accessing VK (VKontakte) API in Rust (asynchronously).

Changelog is available [here][changelog].

# Modules

- [`api`][modules/api] - works with the API;
- [`error`][modules/error] - handles errors that may occur during an API call;
- [`methods`][modules/methods] - contains **API [methods][vk/methods]**;
- [`objects`][modules/objects] - contains **API [objects][vk/objects]**. See also [note about objects](#objects).

# Usage
Add the dependency to your project:

<sub>`Cargo.toml`</sub>
```toml
[dependencies]
rvk = "0.21"
```

Now you can take a look at `rvk`'s [API documentation][docs.rs/rvk] to learn more about the available functions.

# Example

To use this example, you will **also** need the [`tokio`](https://crates.io/tokio) crate for the `tokio::main` attribute proc macro.

<sub>`Cargo.toml`</sub>
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

<sub>`main.rs`</sub>
```rust
use rvk::{methods::users, objects::user::User, APIClient, Params};

#[tokio::main]
async fn main() {
    let api = APIClient::new("your_access_token"); // Create an API Client

    let mut params = Params::new(); // Create a HashMap to store parameters
    params.insert("user_ids".into(), "1".into());

    let res = users::get::<Vec<User>>(&api, params).await;

    match res {
        Ok(users) => {
            let user: &User = &users[0];

            println!(
                "User #{} is {} {}.",
                user.id, user.first_name, user.last_name
            );
        }
        Err(e) => println!("{}", e),
    };
}
```

# Notes
### Objects
Due to the nature of the VK API documentation, it is not always clear if the value is always passed or not, and sometimes the data type is not defined.

If you spot any mistakes or bugs, please [report them][issues]!

[crates.io/rvk]: https://crates.io/crates/rvk

[docs.rs/rvk]: https://docs.rs/rvk

[license]: https://github.com/u32i64/rvk/blob/master/LICENSE
[changelog]: https://github.com/u32i64/rvk/blob/master/CHANGELOG.md

[issues]: https://github.com/u32i64/rvk/issues

[badges/version]: https://img.shields.io/crates/v/rvk.svg?style=for-the-badge
[badges/downloads]: https://img.shields.io/crates/d/rvk.svg?style=for-the-badge
[badges/license]: https://img.shields.io/crates/l/rvk.svg?style=for-the-badge
[badges/api-version]: https://img.shields.io/endpoint?style=for-the-badge&url=https%3A%2F%2Frvk-api-version-badge.warp.workers.dev
[vk-api-version]: https://github.com/u32i64/rvk/blob/master/src/lib.rs#L47-L48

[modules/api]: https://docs.rs/rvk/*/rvk/api/index.html
[modules/error]: https://docs.rs/rvk/*/rvk/error/index.html
[modules/methods]: https://docs.rs/rvk/*/rvk/methods/index.html
[modules/objects]: https://docs.rs/rvk/*/rvk/objects/index.html

[vk/methods]: https://vk.com/dev/methods
[vk/objects]: https://vk.com/dev/objects
