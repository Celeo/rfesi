default:
  cargo check
  cargo +nightly clippy

test:
  cargo test

alias t := test

docs:
  cargo doc --no-deps
  miniserve target/doc
