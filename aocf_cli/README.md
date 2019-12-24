# `aocf_cli` - Advent of Code fetch CLI

A CLI helper tool for [Advent of Code](https://adventofcode.com/).

## Install

```
cargo install aocf_cli
```

## Commands

### `init`

Initialise an aocf repository, similar to how you would initialise a git
repository. Data and configuration for aocf is kept in `./.aocf`.

### `set-cookie`

Write out the cookie using provided session cookie string.

### `checkout`

Move to a given day, and optionally year. May be combined with `--now`.

### `fetch`

Fetch input and brief for the current day.

### `brief`

Show the brief for the currently checked out day.

### `input`

Print the currently checked out day's input.

### `status`

Show the current status (for the currently checked out day).

