extern crate reqwest;
extern crate html2md;

use crate::{Aoc, Level};
use html2md::parse_html;
use failure::Error;
use reqwest::Client;
use reqwest::header::COOKIE;
use std::collections::HashMap;

const BASE: &str = "https://adventofcode.com";

fn get_url(aoc: &Aoc) -> Result<String, Error> {
    let url = match (aoc.day, aoc.year) {
        (Some(d), Some(y)) => format!("{}/{}/day/{}", BASE, y, d),
        _ => bail!("day or year not initialised"),
    };
    Ok(url)
}

fn get_content(aoc: &Aoc, suffix: &str) -> Result<String, Error> {
    let url = format!("{}{}", get_url(aoc)?, suffix);
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
    let brief = get_content(aoc, "/input")?;
    let brief = parse_html(&brief);
    Ok(brief)
}

pub fn get_input(aoc: &Aoc) -> Result<String, Error> {
    let input = get_content(aoc, "input")?;
    Ok(input)
}

pub fn submit(aoc: &Aoc, solution: &str) -> Result<String, Error> {
    let url = format!("{}/answer", get_url(aoc)?);
    let cookie = format!("session={}", aoc.cookie);

    let level = match aoc.level {
        Level::First => 0,
        Level::Second => 1,
    };

    let mut params = HashMap::new();
    params.insert("level", level.to_string());
    params.insert("answer", solution.into());

    let resp = Client::new()
        .post(&url)
        .header(COOKIE, cookie)
        .form(&params)
        .send()?
        .error_for_status()?
        .text()?;

    let resp = parse_html(&resp);
    Ok(resp)
}
