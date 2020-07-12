default: build

build:
  cargo check
  cargo +nightly clippy
  cargo build

t:
  cargo t

r:
  cargo r -- /media/sf_VirtualShareed/enwiki-20200701-pages-articles-multistream1.xml-p1p30303.bz2

rel:
  cargo build --release
  cp target/release/parser .
  strip parser

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
