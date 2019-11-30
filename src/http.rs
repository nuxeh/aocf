extern crate reqwest;

use crate::Aoc;
use failure::Error;
use reqwest::Client;
use reqwest::header::COOKIE;

const BASE: &str = "https://adventofcode.com";

fn get_url(y: i32, d: u32) -> String {
    format!("{}/{}/day/{}", BASE, y, d)
}

pub fn get_input(aoc: &Aoc) -> Result<String, Error> {
    let url = match (aoc.day, aoc.year) {
        (Some(d), Some(y)) => format!("{}/input", get_url(y, d)),
        _ => bail!("day or year not initialised"),
    };
    let cookie = format!("session={}", aoc.cookie);
    let input = Client::new()
        .get(&url)
        .header(COOKIE, cookie)
        .send()?
        .error_for_status()?
        .text()?;

    Ok(input)
}
