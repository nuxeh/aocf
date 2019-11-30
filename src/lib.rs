extern crate chrono;
extern crate failure;

use chrono::{Utc, Datelike};
use failure::Error;

//mod input;
//mod submit;
//mod html;
mod http;

#[derive(Clone)]
enum Part {
    One,
    Two,
}

impl Default for Part {
    fn default() -> Self {
        Part::One
    }
}

#[derive(Default, Clone)]
struct Aoc {
    year: Option<i32>,
    day: Option<u32>,
    part: Part,
    brief: Vec<String>,
    input: Option<String>,
    solution: Vec<String>
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

    /// Initialise (finish building)
    pub fn init(&mut self) -> Self {
        let now = Utc::now();
        self.year = self.year.or(Some(now.year()));
        self.day = self.day.or(Some(now.day()));
        self.clone()
    }

    /// Restore the problem from JSON or TOML
    pub fn restore(&mut self, path: impl AsRef<Path>) -> Self {
        Self::default()
    }

    /// Get the problem brief as HTML and sanitise it to plain text
    pub fn get_brief() -> Result<String, Error> {
        Ok("".to_string())
    }

    /// Get the input data
    pub fn get_input() -> Result<String, Error> {
        Ok("".to_string())
    }

    /// Submit the solution
    pub fn submit() -> Result<(), Error> {
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
