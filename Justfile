default:
  cargo check
  cargo +nightly clippy
  cargo build
