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
    aocf [options] [<command>] [<arguments>...]

Examples:
    aocf [options] submit <solution>
    aocf brief
    aocf input
    aocf submit <solution>
    aocf advance
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
    arg_arguments: Vec<String>,
    flag_day: Option<u32>,
    flag_year: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    Fetch,
    Input,
    Brief,
    Submit,
    Advance,
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
        Command::Fetch => {
            let _ = aoc.get_brief().unwrap();
            let _ = aoc.get_input().unwrap();
        },
        Command::Brief => println!("{}", aoc.get_brief().unwrap()),
        Command::Input => println!("{}", aoc.get_input().unwrap()),
        Command::Submit => {
            println!("{}", aoc.submit(&args.arg_arguments[0]).unwrap());
        },
        Command::Advance => aoc.advance().unwrap(),
    };

    aoc.write().unwrap();
}
