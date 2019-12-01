#[macro_use] extern crate slog;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate aocf;
extern crate docopt;
extern crate toml;

use aocf::Aoc;
use docopt::Docopt;
use failure::Error;

const USAGE: &str = "
Advent of Code Swiss army knife.

Usage:
    aocf [<command>] [options]

Examples:
    aocf brief
    aocf input
    aocf add cookie
    aocf init
    aocf fetch

Options:
    -h --help                   Show this help message.
    --version                   Print version.
    --day=<day>                 Specify challenge day.
    --year=<year>               Specify challenge year.
";

#[derive(Deserialize)]
struct Cliargs {
    arg_command: Command,
    flag_day: Option<u32>,
    flag_year: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    Input,
    Brief,
}

fn main() {
    let args: Cliargs = Docopt::new(USAGE)
        .and_then(|d| d.version(Some("0.1.0".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut aoc = Aoc::new()
        .year(args.flag_year)
        .day(args.flag_day)
        .cookie("cookie")
        .init();


    match args.arg_command {
        Command::Brief => println!("{}", aoc.get_brief().unwrap()),
        Command::Input => println!("{}", aoc.get_input().unwrap()),
    };

    aoc.write().unwrap();
}
