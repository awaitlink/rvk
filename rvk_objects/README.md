Represents various [objects](https://vk.com/dev/objects) that are returned as JSON by the VK API.

It has two complementary crates: [`rvk`](https://docs.rs/rvk), which allows accessing the API,
and [`rvk_methods`](https://docs.rs/rvk_methods), which provides methods to avoid the need to
specify them as strings in `rvk` (it depends on `rvk` to call the methods).
These crates can also be used separately.

Note that for `rvk_methods` and `rvk_objects`, the supported versions of the VK API may be different.
Consult the `API_VERSION` constant in these crates to learn which versions they support.

See [here](https://github.com/u32i64/rvk#readme) for an example of using all 3 crates.

## Note: `type` and `ref` fields
Since `type` and `ref` are Rust keywords, an underscore (`_`) is added at the end of these:
- `type` **->** `type_`,
- `ref` **->** `ref_`.

## Note
Due to the nature of the VK API documentation, it is not always clear if the value is always passed or not, and sometimes the data type is not defined.

If you spot any mistakes or bugs, please [report them](https://github.com/u32i64/rvk/issues)!
