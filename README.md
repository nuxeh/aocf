# `aocf` - Advent of Code fetch

[![build](https://github.com/nuxeh/aocf/workflows/build/badge.svg)](https://github.com/nuxeh/aocf/actions?query=branch%3Amaster+event%3Apush+workflow%3Abuild)
[![test](https://github.com/nuxeh/aocf/workflows/tests/badge.svg)](https://github.com/nuxeh/aocf/actions?query=branch%3Amaster+event%3Apush+workflow%3Atests)
[![clippy](https://github.com/nuxeh/aocf/workflows/clippy/badge.svg)](https://github.com/nuxeh/aocf/actions?query=branch%3Amaster+event%3Apush+workflow%3Aclippy)
[![macOS](https://github.com/nuxeh/aocf/workflows/macOS/badge.svg)](https://github.com/nuxeh/aocf/actions?query=branch%3Amaster+event%3Apush+workflow%3AmacOS)
[![windows](https://github.com/nuxeh/aocf/workflows/windows/badge.svg)](https://github.com/nuxeh/aocf/actions?query=branch%3Amaster+event%3Apush+workflow%3Awindows)
[![coveralls](https://img.shields.io/coveralls/github/nuxeh/aocf/master)](https://coveralls.io/github/nuxeh/aocf?branch=master)
[![crates.io](https://img.shields.io/crates/v/aocf)](https://crates.io/crates/aocf)

A crate and CLI helper tool for [Advent of Code](https://adventofcode.com/).

Written in Rust, but the CLI should be useful for development in any language.

To use directly within Rust, he crate may be used as follows, for getting input
data for a task as a string:

```rust
use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2019))
        .day(Some(1))
        .cookie_file("./cookie")
        .init()
        .unwrap();

    let input = aoc.get_input();

    println!("{}", input);
}
```

Downloaded data is cached as JSON and queried each time the `Aoc` is
initialised, to avoid unecessary requests.

The CLI has a workflow similar to Git, e.g.

```
$ aocf init
$ aocf get-cookie # get cookie from firefox cookie store
$ aocf set-cookie <your-cookie-text>
$ aocf checkout 1 2019
$ aocf fetch
$ aocf brief
$ aocf input
$ aocf status
```

More details can be found in in the CLI's [readme](./aocf_cli/README.md).

How to get your [session cookie](./cookie.md).
