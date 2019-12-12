//#[macro_use] extern crate slog;
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
use std::io::Write;
use std::path::Path;

const USAGE: &str = "
Advent of Code Swiss army knife.

Usage:
    aocf [options] [<command>] [<arguments>...]

Examples:
    aocf init
    aocf set-cookie <content>
    aocf checkout <day> [<year>]
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
    Checkout,
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
    let conf_path = conf::find_root()?.join(".aocf/config");
    Ok(Conf::load(&conf_path)?)
}

fn write_conf(conf: &Conf) -> Result<(), Error> {
    let conf_path = conf::find_root()?.join(".aocf/config");
    Ok(conf.write(&conf_path)?)
}

fn get_day_year(args: &Cliargs) -> (Option<u32>, Option<i32>) {
    let (mut day, mut year) = if args.flag_now {
        let now = Utc::now();
        (Some(now.day()), Some(now.year()))
    } else {
        (None, None)
    };

    day = day.or_else(|| args.flag_day);
    year = year.or_else(|| args.flag_year);

    (day, year)
}

fn add_line_to_file(path: impl AsRef<Path>, line: String) -> Result<(), Error> {
    Ok(())
}

fn run(args: &Cliargs) -> Result<(), Error> {
    let (day, year) = get_day_year(args);

    let mut conf = if args.arg_command == Command::Init {
        Conf::default()
    } else if day.is_none() || year.is_none() {
        find_config().map_err(|e| format_err!("loading config: {}", e))?
    } else {
        find_config().unwrap_or_else(|_| Conf::default())
    };

    let conf_hash = conf.calc_hash();

    let cookie_path = conf::find_root()?.join(".aocf/cookie");
    if !cookie_path.exists() {
        bail!("cookie not found, please run add-cookie");
    }

    let mut aoc = Aoc::new()
        .year(year.or_else(|| Some(conf.year)))
        .day(day.or_else(|| Some(conf.day)))
        .cookie_file(&cookie_path)
        .init()?;

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
        Command::Status => status(&aoc)?,
        Command::Init => init(&args)?,
        Command::Checkout => checkout(&mut conf, conf_hash, &args)?,
        Command::SetCookie => {set_cookie(&args.arg_arguments[0])?},
        _ => bail!("command \"{:?}\" not implemented", args.arg_command),
    };

    if conf.calc_hash() != conf_hash {
        write_conf(&conf)?;
    }

    aoc.write()?;

    Ok(())
}

fn status(aoc: &Aoc) -> Result<(), Error> {
    if let (Some(d), Some(y)) = (aoc.day, aoc.year) {
        eprintln!("{:<6} {}", "year:", y);
        eprintln!("day:   {}", d);
    } else {
        bail!("day or year not set")
    }
    eprintln!("level: {}", aoc.level);
    if let Some(t) = &aoc.title {
        eprintln!("title: {}", t);
    };
    if let Some(s) = aoc.stars {
        eprint!("stars: ");
        for _ in 0..s { eprint!("*"); };
        eprint!("\n");
    };
    Ok(())
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

fn checkout(conf: &mut Conf, conf_hash: u64, args: &Cliargs) -> Result<(), Error> {
    let (day_f, year_f) = get_day_year(args);

    let day = if let Some(d) = args.arg_arguments.get(0) {
        d.parse()?
    } else if let Some(d) = day_f {
        d
    } else {
        bail!("no day provided")
    };

    let year = if let Some(y) = args.arg_arguments.get(1) {
        Some(y.parse()?)
    } else if let Some(y) = year_f {
        Some(y)
    } else {
        None
    };

    conf.day = day;
    if let Some(y) = year {
        conf.year = y;
    }

    if conf.calc_hash() != conf_hash {
        eprintln!("switched to year {}, day {}", conf.year, conf.day);
        write_conf(conf)?;
    };

    Ok(())
}

fn set_cookie(cookie: &str) -> Result<(), Error> {
    let cookie_path = conf::find_root()?.join(".aocf/cookie");
    let mut file = fs::File::create(cookie_path)?;
    Ok(file.write_all(cookie.as_bytes())?)
}
