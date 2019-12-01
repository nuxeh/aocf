#[macro_use] extern crate slog;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate aocf;
extern crate docopt;
extern crate toml;

use docopt::Docopt;
use failure::Error;

const USAGE: &str = "
Advent of Code Swiss army knife.

Usage:
    aocf [options]

Examples:
    aocf brief
    aocf input
    aocf add cookie
    aocf init
    aocf fetch

Options:
    -h --help   Show this help message.
    --version   Print version.
    --day       Specify challenge day.
    --year      Specify challenge year.
";

#[derive(Deserialize)]
struct Cliargs {

}

fn main() {
    let args: Cliargs = Docopt::new(USAGE)
        .and_then(|d| d.version(Some("0.1.0".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Hello, world!");
}
