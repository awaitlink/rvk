# `rvk`

> A set of crates to be able to easily access VK (VKontakte) API in Rust.

The combined changelog for all crates is available [here](https://github.com/u32i64/rvk/blob/master/CHANGELOG.md).

## Crates

- [`rvk`](https://crates.io/crates/rvk) ([docs](https://docs.rs/rvk)) --- simple crate for accessing VK API (using `async`/`await`);
- [`rvk_methods`](https://crates.io/crates/rvk_methods) ([docs](https://docs.rs/rvk_methods)) --- provides VK API [methods](https://vk.com/dev/methods) to avoid the need to specify them as strings, depends on `rvk` to call the methods;
- [`rvk_objects`](https://crates.io/crates/rvk_objects) ([docs](https://docs.rs/rvk_objects)) --- represents various [objects](https://vk.com/dev/objects) that are returned as JSON by the VK API.

Note that for `rvk_methods` and `rvk_objects`, the supported versions of the VK API may be different.
Consult the `API_VERSION` constant in these crates to learn which versions they support.

## Usage
Add the necessary dependencies to your project. For example, to use all 3:

<sub>`Cargo.toml`</sub>
```toml
[dependencies]
rvk = "0.23"
rvk_methods = "0.1"
rvk_objects = "0.1"
```

Now you can take a look at the documentation (linked above for each crate) to learn more about the available functions.

## Example using all 3 crates

To use this example, you will **also** need the [`tokio`](https://crates.io/tokio) crate for the `tokio::main` attribute proc macro.

### `Cargo.toml`
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

### `main.rs`
```rust
use rvk::Params;
use rvk_methods::users;
use rvk_objects::user::User;

#[tokio::main]
async fn main() {
    // Create an API client that uses the API version supported by `rvk_methods`.
    let api = rvk_methods::supported_api_client("your_access_token");

    // A HashMap to store parameters.
    let mut params = Params::new();
    params.insert("user_ids".into(), "1".into());

    // Use a type from `rvk_objects` as the result type.
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
