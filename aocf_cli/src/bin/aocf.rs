#[macro_use] extern crate slog;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate aocf;
extern crate chrono;
extern crate docopt;
extern crate toml;

use aocf::Aoc;
use aocf_cli::conf;
use aocf_cli::conf::Conf;
use chrono::{Utc, Datelike};
use docopt::Docopt;
use failure::Error;
use std::env;
use std::fs;

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
    --now                       Use current day of the month.
    --global                    Set variable globally for AoC root.
    --edit                      Open in editor.
    --force                     Force overwriting the cache.
";

// - https://github.com/rabuf/advent-of-code/blob/master/2019/2019.03.org
// - tag git log

#[derive(Deserialize)]
struct Cliargs {
    arg_command: Command,
    arg_arguments: Vec<String>,
    flag_day: Option<u32>,
    flag_year: Option<i32>,
    flag_now: bool,
    flag_force: bool,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
enum Command {
    Fetch,
    Input,
    Brief,
    Submit,
    Advance,
    Status,
    Init,
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

fn find_config() -> Result<Conf, Error> {
    let conf_path = conf::find()?;
    Ok(Conf::load(&conf_path)?)
}

fn get_day_year(args: &Cliargs) -> (Option<i32>, Option<u32>) {
    let (mut day, mut year) = if args.flag_now {
        let now = Utc::now();
        (Some(now.year()), Some(now.day()))
    } else {
        (None, None)
    };

    day = day.or_else(|| args.flag_year);
    year = year.or_else(|| args.flag_day);

    (day, year)
}

fn run(args: &Cliargs) -> Result<(), Error> {
    let (day, year) = get_day_year(args);

    let init_flagged = args.arg_command == Command::Init;

    let conf = if (day.is_none() || year.is_none()) && !init_flagged {
        find_config().map_err(|e| format_err!("loading config: {}", e))?
    } else {
        Conf::default()
    };

    let mut aoc = Aoc::new()
        .year(year.or_else(|| Some(conf.year)))
        .day(day.or_else(|| Some(conf.day)))
        .cookie("cookie")
        .init();

    match args.arg_command {
        Command::Fetch => {
            let _ = aoc.get_brief(args.flag_force)?;
            let _ = aoc.get_input()?;
        },
        Command::Brief => println!("{}", aoc.get_brief(args.flag_force)?),
        Command::Input => println!("{}", aoc.get_input()?),
        Command::Submit => {
            println!("{}", aoc.submit(&args.arg_arguments[0])?);
        },
        Command::Advance => aoc.advance()?,
        Command::Status => status(&aoc),
        Command::Init => init(&args)?,
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
    if let Some(s) = aoc.stars {
        eprint!("stars: ");
        for _ in 0..s { eprint!("*"); };
        eprint!("\n");
    };
}

fn init(args: &Cliargs) -> Result<(), Error> {
    let conf_path = env::current_dir()?.join(".aocf");
    fs::create_dir_all(&conf_path)?;

    let config_path = conf_path.join("config");
    if config_path.exists() {
        bail!("configuration already exists at {}", config_path.display());
    };

    let (day, year) = get_day_year(args);
    let mut conf = Conf::default();
    if let Some(d) = day {
        conf.day = d;
    };
    if let Some(y) = year {
        conf.year = y;
    };
    conf.write(&config_path)?;

    Ok(())
}
