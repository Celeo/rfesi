# rfesi

[![CI](https://github.com/Celeo/rfesi/actions/workflows/ci.yml/badge.svg)](https://github.com/Celeo/rfesi/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rfesi.svg)](https://crates.io/crates/rfesi)
[![Docs.rs](https://docs.rs/rfesi/badge.svg)](https://docs.rs/rfesi)
[![License](https://img.shields.io/crates/l/rfesi)](https://github.com/Celeo/rfesi/blob/master/Cargo.toml#L10)

Rust API for the [EVE Online](https://www.eveonline.com/) [ESI](https://docs.esi.evetech.net/docs/esi_introduction.html)

## Installing

Add the latest version to your `Cargo.toml`.

This crate has several features that are enabled by default.

- If you don't want or need random SSO state string generation, you can disable the "random_state" feature.
- If you don't want or need SSO token verification, you can disable the "validate_jwt" feature.
- If you prefer to use [rustls](https://crates.io/crates/rustls) instead of your system's TLS implementation ([more info here](https://docs.rs/reqwest/latest/reqwest/tls/)) to make requests, you can disable the default features and add the "rustls-tls" feature.

## Using

[Docs link](https://docs.rs/rfesi).

Note that adding all of the endpoints would very tedious, so not all of them have been implemented. I've added a handful that I found useful, and I will be happy to add missing ones on request. Additionally, I'm happy to take PRs for adding endpoint support.

In the docs, a [group](https://docs.rs/rfesi/latest/rfesi/groups/index.html) without any endpoints mapped to functions will look like [this](https://docs.rs/rfesi/0.5.0/rfesi/groups/struct.MarketGroup.html): a struct without any functions. This maps to [this file](https://github.com/Celeo/rfesi/blob/0e7a5bfe6118bc8e57d1196afea481b786f4460e/src/groups/market.rs), which does not implement any functions. Contrast that with [this file](https://github.com/Celeo/rfesi/blob/0e7a5bfe6118bc8e57d1196afea481b786f4460e/src/groups/character.rs), which contains a struct with several functions.

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

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
