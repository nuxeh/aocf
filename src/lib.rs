#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
extern crate chrono;
extern crate serde;

use chrono::{Utc, Datelike};
use failure::Error;
use std::path::Path;

mod http;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Level {
    First,
    Second,
}

impl Default for Level {
    fn default() -> Self {
        Self::First
    }
}

#[derive(Default, Clone)]
pub struct Aoc {
    year: Option<i32>,
    day: Option<u32>,
    level: Level,
    brief: Vec<String>,
    input: Option<String>,
    solution: Vec<String>,
    cookie: String,
}

impl Aoc {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the year
    pub fn year(&mut self, year: Option<i32>) -> &mut Self {
        self.year = year;
        self
    }

    /// Set the day
    pub fn day(&mut self, day: Option<u32>) -> &mut Self {
        self.day = day;
        self
    }

    pub fn cookie(&mut self, cookie: String) -> &mut Self {
        self.cookie = cookie;
        self
    }

    /// Initialise (finish building)
    pub fn init(&mut self) -> Self {
        let now = Utc::now();
        self.year = self.year.or_else(|| Some(now.year()));
        self.day = self.day.or_else(|| Some(now.day()));
        self.clone()
    }

    /// Restore the problem from JSON or TOML
    pub fn restore(&mut self, path: impl AsRef<Path>) -> Self {
        Self::default()
    }

    /// Get the problem brief as HTML and sanitise it to plain text
    pub fn get_brief(&self) -> Result<String, Error> {
        http::get_brief(self)
    }

    /// Get the input data
    pub fn get_input(&mut self) -> Result<String, Error> {
        if self.input.is_none() {
            let input = http::get_input(self)?;
            self.input = Some(input);
        }
        Ok(self.input.clone().unwrap())
    }

    /// Submit the solution
    pub fn submit(&mut self, solution: &str) -> Result<(), Error> {
        let response = http::submit(self, solution)?;
        Ok(())
    }

    /// get a JSON representation for the AoC problem
    pub fn as_json() -> String {
        "".to_string()
    }

    /// get a TOML representation for the AoC problem
    pub fn as_toml() -> String {
        "".to_string()
    }

    /// Save JSON to path
    pub fn write_json(&self, path: impl AsRef<Path>) {

    }

    /// Save TOML to path
    pub fn write_toml(&self, path: impl AsRef<Path>) {

    }

    /// Get time until release
    pub fn get_time_until_release() {

    }
}
