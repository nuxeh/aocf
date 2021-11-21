use aocf::{
    Aoc,
    cookie::get_session_cookie,
    find_root,
    Level,
};
use aocf_cli::{
    cli::{Aocf, AocfTimeDateOpts, generate_completion},
    conf::Conf,
    pretty::make_pretty,
};
use dirs::home_dir;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{self, Stdio};
use tempfile::tempdir;
use glob::glob;
use failure::{Error, bail, format_err};
use regex::Regex;
use structopt::StructOpt;
use chrono::{Utc, Datelike};
use std::iter;

fn main() {
    let opt = Aocf::from_args();

    run(&opt).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
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

fn run(args: &Aocf) -> Result<(), Error> {
    match args {
        Aocf::Init => return Ok(init()?),
        Aocf::SetCookie { token } => return Ok(set_cookie(&token)?),
        Aocf::GetCookie => return Ok(get_cookie()?),
        Aocf::Completion { shell } => return Ok(generate_completion(*shell)),
        _ => (),
    };

    let mut conf = find_config()
        .map_err(|e| format_err!("loading config: {}", e))?;

    let conf_hash = conf.calc_hash();

    // Check that the cookie is in place
    let cookie_path = find_root()?.join(".aocf/cookie");
    if !cookie_path.exists() {
        bail!("cookie not found, please run set-cookie or get-cookie");
    }

    let mut aoc = Aoc::new()
        .parse_cli(false)
        .year(Some(conf.year))
        .day(Some(conf.day))
        .init()?;

    match args {
        Aocf::Fetch { force, now, day } => {
            aoc = if *now {
                let now = Utc::now();
                Aoc::new()
                    .parse_cli(false)
                    .year(Some(now.year()))
                    .day(Some(now.day()))
                    .init()?
            } else if let Some(d) = day {
                Aoc::new()
                    .parse_cli(false)
                    .year(aoc.year)
                    .day(Some(*d))
                    .init()?
            } else {
                aoc
            };

            let _ = aoc.get_brief(*force)?;
            let _ = aoc.get_input(*force)?;
            aoc.write()?;
        },
        Aocf::Brief { pretty, view, force, now, day } => {
            aoc = if *now {
                let now = Utc::now();
                Aoc::new()
                    .parse_cli(false)
                    .year(Some(now.year()))
                    .day(Some(now.day()))
                    .init()?
            } else if let Some(d) = day {
                Aoc::new()
                    .parse_cli(false)
                    .year(aoc.year)
                    .day(Some(*d))
                    .init()?
            } else {
                aoc
            };

            let brief = aoc.get_brief(*force)?;
            aoc.write()?;
            display(*pretty, *view, &conf, &brief)?
        },
        Aocf::Input { view, force } => {
            let input = aoc.get_input(*force)?;
            aoc.write()?;
            display(false, *view, &conf, &input)?
        },
        Aocf::Submit { answer } => {
            println!("{}", aoc.submit(answer)?);
            aoc.write()?;
        },
        Aocf::Status { .. } => status(&aoc)?,
        Aocf::Summary { year } => summary(*year, conf.year)?,
        Aocf::Checkout ( args ) => checkout(&mut conf, conf_hash, &args)?,
        Aocf::Init | Aocf::SetCookie { .. } | Aocf::GetCookie | Aocf::Completion { .. } => (),
    };

    // Update configuration if changed since start
    if conf.calc_hash() != conf_hash {
        write_conf(&conf)?;
    }

    Ok(())
}

fn display(pretty: bool, view: bool, conf: &Conf, text: &str) -> Result<(), Error> {
    if pretty {
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
    } else if view {
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
    if !aoc.solution.is_empty() {
        eprintln!("solutions:");

        if let Some(s) = aoc.solution.get(&Level::First) {
            eprintln!("    1) {} ", s);
        }
        if let Some(s) = aoc.solution.get(&Level::Second) {
            eprintln!("    2) {} ", s);
        }
    }
    if let Some(s) = aoc.stars {
        eprint!("stars: ");
        for _ in 0..s { eprint!("*"); };
        eprint!("\n");
    };
    Ok(())
}

fn summary(arg_year: Option<i32>, conf_year: i32) -> Result<(), Error> {
    let year = arg_year.unwrap_or(conf_year);

    let mut configs: Vec<_> = fs::read_dir(find_root()?.join(".aocf/cache"))?
        .map(|r| r.map(|e| e.path()))
        .flatten()
        .map(Aoc::load_json_from)
        .flatten()
        .filter(|a| a.year == Some(year))
        .collect();

    configs.sort_by(|a, b| a.day.cmp(&b.day));

    configs
        .iter()
        .for_each(|p| {
            if let (Some(y), Some(d), Some(t), Some(s)) = (p.year, p.day, &p.title, p.stars) {
                let s: String = iter::repeat('*').take(s.into()).collect();
                println!("{} {:2} {:2} {}", y, d, s, t);
            }
        });

    Ok(())
}

fn init() -> Result<(), Error> {
    let conf_path = env::current_dir()?.join(".aocf");
    fs::create_dir_all(&conf_path)?;

    let config_path = conf_path.join("config");
    if config_path.exists() {
        bail!("configuration already exists at {}", config_path.display());
    };

    let conf = Conf::default();
    conf.write(&config_path)?;

    eprintln!("initialised config at {}", config_path.display());

    Ok(())
}

fn checkout(conf: &mut Conf, conf_hash: u64, args: &AocfTimeDateOpts) -> Result<(), Error> {
    let (day, year) = args.get_day_year();

    if let Some(d) = day {
        conf.day = d;
    } else {
        bail!("no day provided");
    }

    if let Some(y) = year {
        conf.year = y;
    }

    if conf.calc_hash() != conf_hash {
        eprintln!("switched to year {}, day {}", conf.year, conf.day);
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
