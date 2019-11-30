extern crate chrono;
extern crate failure;

use chrono::{Utc, Datelike};
use failure::Error;

//mod input;
//mod submit;
//mod html;

#[derive(Default, Clone)]
struct Aoc {
    year: Option<i32>,
    day: Option<u32>,
    brief: Option<String>,
    input: Option<String>,
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

    pub fn init(&mut self) -> Self {
        let now = Utc::now();
        self.year = self.year.or(Some(now.year()));
        self.day = self.day.or(Some(now.day()));
        self.clone()
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
}
