default:
  cargo check
  cargo build
  cargo +nightly clippy
