# nitrokey-sys-rs

Low-level Rust bindings for `libnitrokey`, providing access to Nitrokey
devices.

This crate contains a copy of the [`libnitrokey`][] library, builds it from
source and links it statically.  The host system must provide its dependencies
in the library search path:

- `libhidapi-libusb0` (on Linux)
- `libhidapi` (on non-Linux systems)

If you set the `USE_SYSTEM_LIBNITROKEY` environment variable when building this
crate, it links directly against `libnitrokey` instead of building it from
source.  In this case, `libnitrokey` must be available in the library search
path.

Per default, this crate uses bindings that have been generated using Rust’s
`x86_64-unknown-linux-gnu` target.  To the best of our knowledge, these
bindings are platform-independent.  If you want to generate the bindings,
including layout tests, specifically for your platform during the build,
activate the `bindgen` feature.  In this case, you will also need `clang` and
`libclang` in the default search path.

Alternatively, you can execute `make verify-bindings` to compare the
pre-generated bindings with the bindings that `bindgen` generates for your
platform.  This check only works on a clean Git working tree and requires the
`bindgen` binary, `git` and `quilt`.

## Versioning

The major and minor version of the `nitrokey-sys` crate map to the major and
minor version of `libnitrokey`.  The `nitrokey-sys` patch version may be
increased independently.

## Contact

For bug reports, patches, feature requests or other messages, please send a
mail to [nitrokey-rs-dev@ireas.org][].

## License

This project as well as `libnitrokey` are licensed under the [LGPL-3.0][].

[`libnitrokey`]: https://github.com/nitrokey/libnitrokey
[nitrokey-rs-dev@ireas.org]: mailto:nitrokey-rs-dev@ireas.org
[LGPL-3.0]: https://opensource.org/licenses/lgpl-3.0.html
