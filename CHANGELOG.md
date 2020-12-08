# v0.1.10:

- **Added** Parse CLI arguments with `structopt` rather than `docopt`
  - This vastly improves command line parsing, and gives help on any available
    subcommands.

# v0.1.9:

- **Fixed** a bug with long running commands
  - Previously, long-running commands such as `aocf {input, brief} {--view,
    --pretty}` could cause regressions in the cache state, if the cache was
    updated while they were open.
