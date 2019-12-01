#[macro_use] extern crate log;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate aocf;
extern crate docopt;
extern crate toml;
extern crate slog;

use docopt::Docopt;
use failure::Error;

const USAGE: &str = "
Advent of Code Swiss army knife.

Usage:
    aocf [options]

Options:
    -h --help           Show this help message.
    --version           Print version.
";

#[derive(Deserialize)]
struct Cliargs {

}

fn main() {
    let args: Cliargs = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Hello, world!");
}
