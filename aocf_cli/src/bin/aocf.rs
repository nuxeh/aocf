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
    aocf brief [--edit]
    aocf input [--edit]
    aocf submit <solution>
    aocf advance
    aocf set-cookie
    aocf init
    aocf fetch
    aocf set [--global] <name> <value>
    aocf gen-readme

Options:
    -h --help                   Show this help message.
    --version                   Print version.
    --day=<day>                 Specify challenge day.
    --year=<year>               Specify challenge year.
    --global                    Set variable globally for AoC root.
    --edit                      Open in editor.
";

// - https://github.com/rabuf/advent-of-code/blob/master/2019/2019.03.org
// - tag git log

#[derive(Deserialize)]
struct Cliargs {
    arg_command: Command,
    arg_arguments: Vec<String>,
    flag_day: Option<u32>,
    flag_year: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum Command {
    Fetch,
    Input,
    Brief,
    Submit,
    Advance,
    Status,
    SetCookie,
    Edit,
}

fn main() {
    let args: Cliargs = Docopt::new(USAGE)
        .and_then(|d| d.version(Some("0.1.0".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    run(&args).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
    });
}

fn run(args: &Cliargs) -> Result<(), Error> {
    let mut aoc = Aoc::new()
        .year(args.flag_year)
        .day(args.flag_day)
        .cookie("cookie")
        .init();

    match args.arg_command {
        Command::Fetch => {
            let _ = aoc.get_brief()?;
            let _ = aoc.get_input()?;
        },
        Command::Brief => println!("{}", aoc.get_brief()?),
        Command::Input => println!("{}", aoc.get_input()?),
        Command::Submit => {
            println!("{}", aoc.submit(&args.arg_arguments[0])?);
        },
        Command::Advance => aoc.advance()?,
        Command::Status => status(&aoc),
        Command::SetCookie => {},
        _ => bail!("command \"{:?}\" not implemented", args.arg_command),
    };

    aoc.write()?;

    Ok(())
}

fn status(aoc: &Aoc) {
    eprintln!("{:<6} {}", "year:", aoc.year.unwrap());
    eprintln!("day:   {}", aoc.day.unwrap());
    eprintln!("level: {}", aoc.level);
    if let Some(t) = &aoc.title {
        eprintln!("title: {}", t);
    };
}
