# `aocf_cli` - Advent of Code fetch CLI

[![crates.io](https://img.shields.io/crates/v/aocf_cli)](https://crates.io/crates/aocf_cli)

A CLI helper tool for [Advent of Code](https://adventofcode.com/).

## Install

```
cargo install aocf_cli
```

The tool will now be available as `aocf` for the user who ran this command.

## Commands

### `init`

Initialise an aocf repository, similar to how you would initialise a git
repository. Data and configuration for aocf is kept in `./.aocf`.

### `set-cookie`

Write out the cookie using provided session cookie string.

### `get-cookie`

Extract your session cookie from the Firefox cookie store. You should have
previously logged in to Advent of code using Firefox. This command will then
set the cookie as with `set-cookie`.

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

## Option flags

### Time based flags

* `--now`
* `--year`
* `--day`

These can be used to override the current day or year used when fetching,
initialising, or other commands. Generally commands will get the day and year
from the configuration if these are not provided.

### Viewing flags

* `--view`

If no flags are given, `aocf` will generally output content on standard output;
if `--view` is provided, output will be piped to the pager programme specified
in the root configuration (`.aocf/config`) as e.g. `pager = "less"`.

### Retrieval flags

* `--force`

This will force retrieval from the AoC site, even if a cache eists already for
the current day/problem part.

### Cache

The cache for data for each day is stored as JSON under `.aocf/cache`.
