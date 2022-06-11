This crate provides VK API [methods](https://vk.com/dev/methods) to avoid the need to specify them as strings.

It has two complementary crates: [`rvk`](https://docs.rs/rvk) (which this crate depends on
to actually call the methods) and [`rvk_objects`](https://docs.rs/rvk_objects), which provides objects.
These crates can also be used separately.

Note that for `rvk_methods` and `rvk_objects`, the supported versions of the VK API may be different.
Consult the `API_VERSION` constant in these crates to learn which versions they support.

See [here](https://github.com/u32i64/rvk#readme) for an example of using all 3 crates.

## Note about naming
Rust prefers `snake_case` in the function names instead of `camelCase` used by the VK API,
which means all of the API method's corresponding functions are named using `snake_case`.

**Example:** To call the `appWidgets.getAppImageUploadServer` API method, use the `rvk::methods::app_widgets::get_app_image_upload_server` function.

## Note: `execute`
The `execute` method has no category, so its path is `rvk::methods::execute`.

## Note: `photos.move`
Since `move` is a Rust keyword, the function for calling `photos.move` API method is `rvk::methods::photos::move_` (**with the underscore!**)
