This is a simple crate for accessing VK API (using `async`/`await`).

It has two complementary crates: [`rvk_methods`](https://docs.rs/rvk_methods)
and [`rvk_objects`](https://docs.rs/rvk_objects), providing methods (to avoid
the need to specify them as strings) and objects respectively.

These crates can also be used separately (note that `rvk_methods` depends on
`rvk` to actually call the methods).

Note that for `rvk_methods` and `rvk_objects`, the supported versions of the VK API may be different.
Consult the `API_VERSION` constant in these crates to learn which versions they support.
