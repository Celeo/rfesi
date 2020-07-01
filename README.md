# rfesi

[![CI](https://github.com/Celeo/rfesi/workflows/CI/badge.svg?branch=master)](https://github.com/celeo/rfesi/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/rfesi.svg)](https://crates.io/crates/rfesi)
[![Docs.rs](https://docs.rs/rfesi/badge.svg)](https://docs.rs/rfesi)
[![License](https://img.shields.io/crates/l/rfesi)](https://github.com/Celeo/rfesi/blob/master/Cargo.toml#L10)

Rust API for the EVE online ESI

## Installing

Add the latest version to your `Cargo.toml`.

## Using

See [the docs](https://docs.rs/rfesi).

Note that adding all of the endpoints would very tedious, so I'm adding endpoints in a rough order of what I think are most useful.

## Developing

### Building

### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/rfesi
cd rfesi
cargo test
```

If you have [Just](https://github.com/casey/just), then running `just` in the project will check for compilation and clippy violations.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
