default:
  cargo check
  cargo +nightly clippy

test:
  cargo test

alias t := test
