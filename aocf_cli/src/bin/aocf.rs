use aocf::{
    Aoc,
    cookie::get_session_cookie,
    find_root,
    Level,
    ensure_parent_dir,
};
use aocf_cli::{
    pretty::make_pretty,
    day::Day,
};
use aocf_cli::conf::Conf;
use chrono::{Utc, Datelike};
use dirs::home_dir;
use docopt::Docopt;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{self, Stdio};
use tempfile::tempdir;
use glob::glob;
use serde::Deserialize;
use failure::{Error, bail, format_err};
use regex::Regex;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

const USAGE: &str = "
Advent of Code Swiss army knife.

Usage:
    aocf [options] [<command>] [<arguments>...]

Examples:
    aocf init
    aocf set-cookie <content>
    aocf get-cookie
    aocf checkout <day> [<year>]
    aocf fetch
    aocf brief
    aocf input
    aocf submit <solution>
    aocf exec <command>...
    aocf run [--profile]

Options:
    -h --help         Show this help message.
    --version         Print version.
    -d --day=<day>    Specify challenge day.
    -y --year=<year>  Specify challenge year.
    -n --now          Use current day of the month.
    -v --view         Open in pager.
    -p --pretty       Pretty print brief output.
    -P --profile      Run executables with profiling.
    --force           Force overwriting the cache.
";

/*
    aocf set [--global] <name> <value>
    aocf gen-readme

    --global                    Set variable globally for AoC root.
    --edit                      Open in editor.
*/

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
    flag_view: bool,
    flag_pretty: bool,
    flag_profile: bool,
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
    GetCookie,
    Edit,
    Checkout,
    Exec,
    Run,
}

fn main() {
    let args: Cliargs = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(PKG_VERSION.to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    run(&args).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
    });
}

fn find_config() -> Result<Conf, Error> {
    let conf_path = find_root()?.join(".aocf/config");
    Ok(Conf::load(&conf_path)?)
}

fn write_conf(conf: &Conf) -> Result<(), Error> {
    let conf_path = find_root()?.join(".aocf/config");
    Ok(conf.write(&conf_path)?)
}

fn get_day_year(args: &Cliargs) -> (Option<u32>, Option<i32>) {
    let (mut day, mut year) = if args.flag_now {
        let now = Utc::now();
        (Some(now.day()), Some(now.year()))
    } else {
        (None, None)
    };

    day = day.or(args.flag_day);
    year = year.or(args.flag_year);

    (day, year)
}

fn run(args: &Cliargs) -> Result<(), Error> {
    match args.arg_command {
        Command::Init => return Ok(init(&args)?),
        Command::SetCookie => return Ok(set_cookie(&args.arg_arguments[0])?),
        Command::GetCookie => return Ok(get_cookie()?),
        _ => (),
    };

    let (day, year) = get_day_year(args);

    let mut conf = if day.is_none() || year.is_none() {
        find_config().map_err(|e| format_err!("loading config: {}", e))?
    } else {
        find_config().unwrap_or_else(|_| Conf::default())
    };

    let conf_hash = conf.calc_hash();

    // Check that the cookie is in place
    let cookie_path = find_root()?.join(".aocf/cookie");
    if !cookie_path.exists() {
        bail!("cookie not found, please run set-cookie or get-cookie");
    }

    let mut aoc = Aoc::new()
        .year(year.or(Some(conf.year)))
        .day(day.or(Some(conf.day)))
        .init()?;

    match args.arg_command {
        Command::Fetch => {
            let _ = aoc.get_brief(args.flag_force)?;
            let _ = aoc.get_input(args.flag_force)?;
        },
        Command::Brief => display(args, &conf, &aoc.get_brief(args.flag_force)?)?,
        Command::Input => display(args, &conf, &aoc.get_input(args.flag_force)?)?,
        Command::Submit => {
            println!("{}", aoc.submit(&args.arg_arguments[0])?);
        },
        Command::Advance => aoc.advance()?,
        Command::Status => status(&aoc)?,
        Command::Checkout => checkout(&mut conf, conf_hash, &args)?,
        Command::Init => (),
        Command::SetCookie => (),
        Command::GetCookie => (),
        Command::Exec => set_exec(aoc.year, aoc.day, aoc.level, &args.arg_arguments)?,
        Command::Run => run_exec(aoc.year, aoc.day, aoc.level, args.flag_profile)?,
        _ => bail!("command \"{:?}\" not implemented", args.arg_command),
    };

    if conf.calc_hash() != conf_hash {
        write_conf(&conf)?;
    }

    aoc.write()?;

    Ok(())
}

fn display(args: &Cliargs, conf: &Conf, text: &str) -> Result<(), Error> {
    if args.flag_pretty {
        // Cludgily post-process markdown
        let re = Regex::new(r"`\*(?P<content>.+?)\*`").unwrap();
        let display_text: String = text.lines()
            .map(|l| format!("{}\n", l))
            .map(|l| re.replace_all(&l, "*`$content`*").to_string())
            .collect::<String>()
            .replace(": \\*\\*", ": **`**`**")
            .replace(": \\*", ": **`*`**")
            .replace("\\---", "---");
        make_pretty(&display_text)?;
    } else if args.flag_view {
        pager(conf, text)?;
    } else {
        print!("{}", text);
    }
    Ok(())
}

fn pager(conf: &Conf, text: &str) -> Result<(), Error> {
    let mut process = process::Command::new(&conf.pager)
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = process.stdin.take().unwrap();
    stdin.write_all(text.as_bytes())?;
    stdin.flush()?;
    drop(stdin);
    process.wait()?;
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

    eprintln!("initialised config at {}", config_path.display());

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
    let cookie_path = find_root()?.join(".aocf/cookie");
    let mut file = fs::File::create(cookie_path)?;

    // write out a .gitignore to avoid committing the cookie data (which
    // would be very insecure!)
    let gitignore_path = find_root()?.join(".aocf/.gitignore");
    if !gitignore_path.exists() {
        let mut gitignore = fs::File::create(gitignore_path)?;
        gitignore.write_all(b"cookie\n")?;
    }

    Ok(file.write_all(cookie.as_bytes())?)
}

fn get_cookie() -> Result<(), Error> {
    let cookie_store_dir = match home_dir() {
        None => bail!("can't get home directory"),
        Some(d) => {
            if let Some(p) = d.join(".mozilla/firefox/*.default/cookies.sqlite").to_str() {
                match glob(p) {
                    Ok(mut path) => path.next(),
                    Err(e) => bail!("{:?}", e),
                }
            } else {
                bail!("can't get cookie store path");
            }
        }
    };

    // copy the cookie store to a temporary location, if firefox is open, the
    // store will be locked
    let tmp_dir = tempdir()?;
    let tmp_path = tmp_dir.path().join("cookies.sqlite");
    if let Some(Ok(path)) = cookie_store_dir {
        eprintln!("found cookie store: {}", path.display());
        fs::copy(&path, &tmp_path)?;
    } else {
        bail!("couldn't get cookie store path");
    }

    let cookie_value = get_session_cookie(&tmp_path)?;
    set_cookie(&cookie_value)
}

fn set_exec(year: Option<i32>, day: Option<u32>, level: Level, args: &Vec<String>) -> Result<(), Error> {
    if let (Some(year), Some(day)) = (year, day) {
        let conf_path = find_root()?.join(&year.to_string()).join(day.to_string());

        let mut day_conf = if conf_path.exists() {
            Day::load(&conf_path)?
        } else {
            ensure_parent_dir(&conf_path);
            Day::default()
        };

        day_conf.exec.insert(level, args.to_vec());
        day_conf.write(conf_path)?;
    } else {
        bail!("can't get day and year");
    }

    Ok(())
}

fn run_exec(year: Option<i32>, day: Option<u32>, level: Level, profile: bool) -> Result<(), Error> {
    if let (Some(year), Some(day)) = (year, day) {
        let conf_path = find_root()?.join(&year.to_string()).join(day.to_string());
        let mut day_conf = Day::load(&conf_path)?;
    } else {
        bail!("can't get day and year");
    }

    Ok(())
}
