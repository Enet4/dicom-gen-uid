# DICOM UID generator

[![DICOM-rs on crates.io](https://img.shields.io/crates/v/dicom-gen-uid.svg)](https://crates.io/crates/dicom-gen-uid)
[![continuous integration](https://github.com/Enet4/dicom-gen-uid/actions/workflows/rust.yml/badge.svg)](https://github.com/Enet4/dicom-gen-uid/actions/workflows/ci.yml)
![Minimum Rust Version Stable](https://img.shields.io/badge/Minimum%20Rust%20Version-stable-green.svg)
[![Documentation](https://docs.rs/dicom-gen-uid/badge.svg)](https://docs.rs/dicom-gen-uid)

This is a simple library and command-line tool to generate DICOM UIDs,
written in Rust.

## Using the CLI tool

If you have Cargo, you can install it with just one command:

```sh
cargo install dicom-gen-uid
```

Call `dicom-gen-uid` to print a fresh new UID to stdout.

## Using the library

An example follows.
Please see the [documentation](https://docs.rs/dicom-gen-uid) for more details.

```rust
let uid: String = dicom_gen_uid::gen_uid();
```

For the library to work in WebAssembly,
add `getrandom` with the feature `js`.

```toml
[dependencies.getrandom]
version = "0.2"
features = ["js"]
```

## See also

[DICOM-rs], a DICOM implementation for the next generation of medical imaging systems.

[DICOM-rs]: https://github.com/Enet4/dicom-rs

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
