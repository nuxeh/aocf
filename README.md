# `aocf` - Advent of Code fetch

A crate and CLI helper tool for [Advent of Code](https://adventofcode.com/).

The crate may be used as follows, for getting input data for a task:

```
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
