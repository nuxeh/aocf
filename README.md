# `aocf` - Advent of Code fetch

A crate and CLI helper tool for [Advent of Code](https://adventofcode.com/).

Written in Rust, but the CLI should be useful for development in any language.

To use directly within Rust, he crate may be used as follows, for getting input
data for a task as a string:

```rust
extern crate aocf;

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
$ aocf add-cookie
$ aocf checkout 1 2019
$ aocf fetch
$ aocf brief
$ aocf input
$ aocf status
```

More details can be found in in the CLI's [readme](./aocf_cli/README.md).
