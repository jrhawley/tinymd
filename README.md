# tinymd

A Markdown file parser written in Rust, inspired by [Jesse Lawson](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/).

## Installation

### GitHub

```shell
git clone git@github.com:jrhawley/tinymd.git
cd tinymd
cargo build --release
./target/release/tinymd
```

## Compliance

To check compliance with the [CommonMark standard](https://commonmark.org), use the [CommonMark specification](https://github.com/commonmark/commonmark-spec).

```shell
git submodule update --init --recursive
cd commonmark-spec/
python test/spec_tests.py -p tinymd
```

This project is still in progress.

Parsing features:

* paragraph text
* headers of all levels

Surprisingly, just being able to parse `h\d` and `p` tags passes 82/652 tests.