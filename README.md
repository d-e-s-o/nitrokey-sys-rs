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

## Contributing

Contributions to this project are welcome!  Please submit patches to the
mailing list [~ireas/nitrokey-rs-dev@lists.sr.ht][] ([archive][]) using the
`[PATCH nitrokey-sys-rs]` subject prefix.  For more information, see the
[Contributing Guide][].

## Contact

For bug reports, patches, feature requests or other messages, please send a
mail to the mailing list [~ireas/nitrokey-rs-dev@lists.sr.ht][] ([archive][]).

## License

This project as well as `libnitrokey` are licensed under the [LGPL-3.0][].

[`libnitrokey`]: https://github.com/nitrokey/libnitrokey
[~ireas/nitrokey-rs-dev@lists.sr.ht]: mailto:~ireas/nitrokey-rs-dev@lists.sr.ht
[archive]: https://lists.sr.ht/~ireas/nitrokey-rs-dev
[Contributing Guide]: https://man.sr.ht/~ireas/guides/contributing.md
[LGPL-3.0]: https://opensource.org/licenses/lgpl-3.0.html
