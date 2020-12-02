#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use std::collections::{HashMap, BTreeMap};
use std::fmt;
use std::fs::{File, read_to_string, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::env::current_dir;
use serde::{Serialize, Serializer};
use failure::{Error, bail};

mod http;
pub mod cookie;

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    First,
    Second,
}

impl Default for Level {
    fn default() -> Self {
        Self::First
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Level::First => "first",
            Level::Second => "second",
        };
        write!(f, "{}", s)
    }
}

/// A cache entry for a single day, containing all data related to that day's problem
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Aoc {
    pub year: Option<i32>,
    pub day: Option<u32>,
    pub level: Level,
    pub title: Option<String>,
    pub stars: Option<u8>,
    input: Option<String>,
    #[serde(serialize_with = "ordered_map")]
    brief: HashMap<Level, String>,
    #[serde(serialize_with = "ordered_map")]
    solution: HashMap<Level, String>,
    #[serde(skip)]
    cache_path: Option<PathBuf>,
    #[serde(skip)]
    cookie_path: Option<PathBuf>,
    #[serde(skip)]
    cookie: String,
}

impl Aoc {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the year
    pub fn year(mut self, year: Option<i32>) -> Self {
        self.year = year;
        self
    }

    /// Set the day
    pub fn day(mut self, day: Option<u32>) -> Self {
        self.day = day;
        self
    }

    /// Set cookie string
    pub fn cookie(mut self, cookie: &str) -> Self {
        self.cookie = cookie.to_string();
        self
    }

    /// Set cookie file
    pub fn cookie_file(mut self, path: impl AsRef<Path>) -> Self {
        self.cookie_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the cache path
//    pub fn cache<P>(&mut self, path: P) -> &mut Self
//        where P: AsRef<Path> + std::clone::Clone,
//    {
    pub fn cache<P>(mut self, path: Option<&Path>) -> Self {
        self.cache_path = path.as_ref().map(PathBuf::from);
        self
    }

    /// Initialise (finish building)
    pub fn init(mut self) -> Result<Self, Error> {
        // Attempt to load cookie data
        if self.cookie.is_empty() {
            if let Some(p) = &self.cookie_path {
                self.cookie = read_to_string(p)?.trim().to_string()
            } else if let Ok(p) = self.get_default_cookie_path() {
                self.cookie = read_to_string(p)?.trim().to_string()
            };
        }

        if let Ok(mut aoc) = self.load() {
            aoc.cookie = self.cookie;
            Ok(aoc)
        } else {
            Ok(self)
        }
    }

    /// Get the problem brief as HTML and sanitise it to markdown
    pub fn get_brief(&mut self, force: bool) -> Result<String, Error> {
        if self.brief.get(&self.level).is_none() || force {
            let brief = http::get_brief(self)?;
            self.title = Some(brief.0);
            self.brief.insert(self.level, brief.1);
        };
        self.write()?;
        Ok(self.brief.get(&self.level).unwrap().to_string())
    }

    /// Get the input data
    pub fn get_input(&mut self, force: bool) -> Result<String, Error> {
        if self.input.is_none() || force {
            let input = http::get_input(self)?;
            self.input = Some(input);
        }
        self.write()?;
        Ok(self.input.clone().unwrap())
    }

    /// Submit the solution
    pub fn submit(&mut self, solution: &str) -> Result<String, Error> {
        let resp = http::submit(self, solution)?;
        if http::verify(&resp) {
            self.solution.insert(self.level, solution.to_string());
            self.get_brief(true).ok(); // Update brief (force) to update stars
            self.add_star();
            self.advance().unwrap_or(());
        }
        self.write()?;
        Ok(resp)
    }

    fn add_star(&mut self) {
        if let Some(ref stars) = self.stars {
            self.stars = Some(stars + 1);
        } else {
            self.stars = Some(1);
        };
    }

    /// get a JSON representation for the AoC problem
    pub fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// get an AoC problem from JSON representation
    pub fn from_json(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    /// Save problem to path as JSON
    pub fn write_json_to(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        ensure_parent_dir(path.as_ref())?;
        let mut file = File::create(path)?;
        file.write_all(self.to_json()?.as_bytes())?;
        Ok(())
    }

    /// Load the problem from JSON
    pub fn load_json_from(path: impl AsRef<Path>) -> Result<Self, Error> {
        let json = read_to_string(path)?;
        Self::from_json(&json)
    }

    /// Write JSON cache
    pub fn write(&self) -> Result<(), Error> {
        if let Some(ref p) = self.cache_path {
            self.write_json_to(p)
        } else {
            self.write_json_to(&self.get_default_cache_path()?)
        }
    }

    pub fn advance(&mut self) -> Result<(), Error> {
        match self.level {
            Level::First => { self.level = Level::Second; Ok(()) },
            Level::Second => bail!("already on part 2"),
        }
    }

    fn load(&self) -> Result<Self, Error> {
        if let Some(ref p) = self.cache_path {
            Self::load_json_from(p)
        } else {
            Self::load_json_from(&self.get_default_cache_path()?)
        }
    }

    fn get_default_cookie_path(&self) -> Result<PathBuf, Error> {
        let p = PathBuf::from("./.aocf/cookie");
        if let Ok(r) = find_root() {
            Ok(r.join(p))
        } else {
            Ok(p)
        }
    }

    fn get_default_cache_path(&self) -> Result<PathBuf, Error> {
        if let (Some(y), Some(d)) = (self.year, self.day) {
            let p = PathBuf::from(&format!("./.aocf/cache/aoc{}_{}.json", y, d));
            if let Ok(r) = find_root() {
                Ok(r.join(p))
            } else {
                Ok(p)
            }
        } else {
            bail!("day or year not set");
        }
    }

    /// Get time until release
    pub fn get_time_until_release() {

    }
}

/// Get an ordered hashmap representation when serialising
fn ordered_map<S>(value: &HashMap<Level, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

fn ensure_parent_dir(file: impl AsRef<Path>) -> Result<(), Error> {
    let without_path = file.as_ref().components().count() == 1;
    match file.as_ref().parent() {
        Some(dir) if !without_path => create_dir_all(dir)?,
        _ => (),
    };
    Ok(())
}

/// Find configuration directory in current directory or its ancestors
pub fn find_root() -> Result<PathBuf, Error> {
    let cwd = current_dir()?;

    let conf_dir = cwd.ancestors()
        .find(|dir| dir.join(".aocf").is_dir())
        .filter(|dir| dir.join(".aocf/config").is_file());

    match conf_dir {
        Some(dir) => Ok(dir.to_path_buf()),
        None => bail!("no configuration found, maybe you need to run `aocf init`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;
    use std::fs;

    #[test]
    fn test_find_root() {
        let tmp = tempdir().unwrap();
        let tmp_path = tmp.path();
        let tmp_sub = tmp_path.join("im/in/a-subdir");
        fs::create_dir_all(&tmp_sub).unwrap();

        env::set_current_dir(tmp_path).unwrap();
        assert!(find_root().is_err());
        fs::create_dir(tmp_path.join(".aocf")).unwrap();
        assert!(find_root().is_err());
        File::create(tmp_path.join(".aocf/config")).unwrap();
        assert!(find_root().is_ok());
        env::set_current_dir(tmp_sub).unwrap();
        if cfg!(linux) || cfg!(windows) {
            /* Very strange result on macos...
             *
             * ---- tests::test_find_root stdout ----
             * thread 'tests::test_find_root' panicked at 'assertion failed: `(left == right)`
             * left: `"/private/var/folders/24/8k48jl6d249_n_qfxwsl6xvm0000gn/T/.tmpUwUaSn"`,
             * right: `"/var/folders/24/8k48jl6d249_n_qfxwsl6xvm0000gn/T/.tmpUwUaSn"`', src/lib.rs:292:13
             * note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
             *
            let left: PathBuf = find_root().unwrap().components().skip(5).collect();
            let right: PathBuf = tmp_path.components().skip(4).collect();
            assert_eq!(left, right);
            */
            assert_eq!(find_root().unwrap(), tmp_path);
        }
    }
}
