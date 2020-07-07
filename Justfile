default:
  cargo check
  cargo +nightly clippy
  cargo build

t:
  cargo t

r:
  cargo r -- /media/sf_VirtualShareed/enwiki-20200701-pages-articles-multistream.xml.bz2

rel:
  cargo build --release
  cp target/release/wiki_grapher .
  strip wiki_grapher
  @echo "Binary available at ./wiki_grapher"
