default: build

build:
  cargo check --all
  cargo +nightly clippy --all
  cargo build --all

t:
  cargo t

rp:
  RUST_LOG=parser=debug cargo r --bin parser -- /media/sf_VirtualShareed/enwiki-20200701-pages-articles-multistream1.xml-p1p30303.bz2

rg:
  RUST_LOG=grapher=debug cargo r --bin grapher

release:
  cargo build --release --all
  cp target/release/parser .
  cp target/release/grapher .
  strip parser
  strip grapher

alias rel := release

db_to_csv:
  #!/usr/bin/bash
  sqlite3 data.db << EOF
  .mode csv
  .output links.csv
  select name, page_to from links inner join pages on pages.id == links.page_from;
  EOF

clean:
  rm data.*
  rm *.csv
  rm parser
