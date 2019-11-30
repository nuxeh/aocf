extern crate reqwest;

use crate::Aoc;
use failure::Error;
use reqwest::Client;
use reqwest::header::COOKIE;

const BASE: &str = "https://adventofcode.com";

fn get_url(aoc: &Aoc) -> Result<String, Error> {
    let url = match (aoc.day, aoc.year) {
        (Some(d), Some(y)) => format!("{}/{}/day/{}", BASE, y, d),
        _ => bail!("day or year not initialised"),
    };
    Ok(url)
}

fn get_content(aoc: &Aoc, suffix: &str) -> Result<String, Error> {
    let url = format!("{}/{}", get_url(aoc)?, suffix);
    let cookie = format!("session={}", aoc.cookie);
    let input = Client::new()
        .get(&url)
        .header(COOKIE, cookie)
        .send()?
        .error_for_status()?
        .text()?;

    Ok(input)
}

pub fn get_brief(aoc: &Aoc) -> Result<String, Error> {
    let brief = get_content(aoc, "input")?;
    Ok(brief)
}

pub fn get_input(aoc: &Aoc) -> Result<String, Error> {
    let input = get_content(aoc, "input")?;
    Ok(input)
}
