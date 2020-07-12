# wiki_grapher

An attempt to parse the [Wikipedia database download](https://en.wikipedia.org/wiki/Wikipedia:Database_download) into a [graph](https://en.wikipedia.org/wiki/Graph_(abstract_data_type)) so that it can be [traversed](https://en.wikipedia.org/wiki/Graph_traversal) to get information on links between pages.

## Note

This project is still very much _in progress_.

| Feature | Status |
| - | - |
| Parse a data dump portion | Done |
| Parse the entire wiki dump | Theoretically done but not proven |
| Load the data into a graph | Not started |
| Able to traverse nodes to find paths | Not started |

## Installing

### The binary

1. `git clone https://github.com/celeo/wiki_grapher`
1. `cd wiki_grapher`
1. `cargo build`

I use [just](https://github.com/casey/just), but you don't have to.

### The database download

There are 2 main options:

1. Go to [this](https://en.wikipedia.org/wiki/Wikipedia:Database_download) page and download & run the torrent
2. Go to [this](https://dumps.wikimedia.org/backup-index.html) page, click on the most recently-completed "enwiki" link, and then download the "enwiki-[date]-pages-articles-multistream.xml.bz2" file

## Using

TODO in-dev

## Developing

### Building

### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/wiki_grapher
cd wiki_grapher
cargo test
```

If you have [Just](https://github.com/casey/just), then running `just` in the project will check for compilation and clippy violations and build.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
