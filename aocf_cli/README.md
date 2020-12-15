# `aocf_cli` - Advent of Code fetch CLI

[![crates.io](https://img.shields.io/crates/v/aocf_cli)](https://crates.io/crates/aocf_cli)

A CLI helper tool for [Advent of Code](https://adventofcode.com/).

This is a wrapper around the `aocf` crate, and freely interoperates with it if
needed.

## Install

```
cargo install aocf_cli
```

Or from source:

```
git clone https://github.com/aocf
cd aocf/aocf_cli
cargo install --path .
```

The tool will now be available as `aocf` for the user who ran this command. The
cargo binary install path provided after cargo completes may need to be added
to your shell's `PATH` variable.

## Commands

### `init`

Initialise an aocf repository, similar to how you would initialise a git
repository. Data and configuration for aocf is kept in `./.aocf`.

### `set-cookie`

Write the cookie into cache using provided session cookie string.

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

### `summary`

Show a summary of all challenges which exist in the cache, and stars achieved,
for a given challenge year. The year shown is either the currently checked out
year, or the year provided by the `--year` command line option.

### `help`

Get help on command usage, for a given subcommand.

## Option flags

### Time based flags

* `--now`
* `--year`
* `--day`

These can be used to override the current day or year used when fetching, or
other commands. Generally commands will get the day and year from the
root configuration if it exists, and these are not provided.

### Viewing flags

* `--view`

If no flags are given, `aocf` will generally output content on standard output
or stderr; if `--view` is provided, output will be piped to the pager programme
specified in the root configuration (`.aocf/config`) as e.g. `pager = "less"`.

* `--pretty`

Only available for the `brief` subcommand, pretty formats the challenge brief
in a similar format to viewing on the Advent of Code website, in a scrollable,
pager fashion.

### Retrieval flags

* `--force`

This will force retrieval from the AoC site, even if a cache exists already for
the current day/problem part.

### Cache

The cache for data for each day is stored as JSON under `.aocf/cache`.
