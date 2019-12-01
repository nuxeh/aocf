extern crate regex;
extern crate reqwest;
extern crate html2md;

use crate::{Aoc, Level};
use html2md::parse_html;
use failure::Error;
use regex::Regex;
use reqwest::Client;
use reqwest::header;
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
        .header(header::COOKIE, cookie)
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()?
        .error_for_status()?
        .text()?;
    Ok(input)
}

pub fn get_brief(aoc: &Aoc) -> Result<String, Error> {
    let brief = get_content(aoc, "")?;
    let brief = get_html_section(&brief, "main").unwrap_or("".to_string());
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
        Level::First => 1,
        Level::Second => 2,
    };

    let mut params = HashMap::new();
    params.insert("level", level.to_string());
    params.insert("answer", solution.into());

    let resp = Client::new()
        .post(&url)
        .header(header::COOKIE, cookie)
        .header(header::USER_AGENT, "Mozilla/5.0")
        .form(&params)
        .send()?
        .error_for_status()?
        .text()?;

    let resp = parse_html(&resp);
    Ok(resp)
}

fn get_html_section(contents: &str, section: &str) -> Option<String> {
    let regex = format!("<{}>((.|\n)*?)</{}>", section, section);
    let regex = Regex::new(&regex).unwrap();
    let html = regex.captures(contents)?.get(1)?.as_str();
    Some(html.to_string())
}
