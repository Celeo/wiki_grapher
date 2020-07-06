default:
  cargo check
  cargo +nightly clippy
  cargo build

r:
  cargo r -- /media/sf_VirtualShareed/enwiki-20200401-pages-articles-multistream.xml.bz2

rel:
  cargo build --release
  cp target/release/wiki_grapher .
  strip wiki_grapher
  @echo "Binary available at ./wiki_grapher"
