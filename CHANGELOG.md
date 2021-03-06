# v0.1.13

- **Fixed** clippy warning

# v0.1.12

- **Added** `summary` subcommand
  - Gives a handy overview of all stars obtained.
- **Added** printing of solutions for `aocf status` subcommand

# v0.1.11

- **Added** `--now` and `--day` flags to brief subcommand
- **Added** `--day` and `--now` flags for fetch subcommand
  - Allows fetching a given day without checking out that day first, useful
    for peeking at a new problem, for example.
- **Added** `git-journal` configuration
- **Added** make diesel and sqlite optional dependencies behind sqlite feature
- **Added** CLI parsing for input file to crate struct
  - This allows an alternative input file to be used rather than fetched
    input, useful for testing, or modifying the input.
- **Added** buffering of input data from a pipe
  - Alternative input data can now be provided by piping it to the built
    binary implementing the Aoc stuct.

# v0.1.10:

- **Added** Parse CLI arguments with `structopt` rather than `docopt`
  - This vastly improves command line parsing, and gives help on any available
    subcommands.

# v0.1.9:

- **Fixed** a bug with long running commands
  - Previously, long-running commands such as `aocf {input, brief} {--view,
    --pretty}` could cause regressions in the cache state, if the cache was
    updated while they were open.
