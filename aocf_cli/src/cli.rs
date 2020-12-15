use structopt::StructOpt;
use chrono::{Utc, Datelike};

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[derive(StructOpt, Debug)]
#[structopt(about = "Advent of Code Swiss army knife", version = PKG_VERSION)]
pub enum Aocf {
    /// Switch to a specified year and day
    Checkout ( AocfTimeDateOpts ),

    /// Get input data for the current problem
    Input {
        /// View in pager
        #[structopt(short, long)]
        view: bool,

        /// Don't use cache
        #[structopt(short, long)]
        force: bool,
    },

    /// Get instructions for the current problem
    Brief {
        /// View pretty
        #[structopt(short, long, conflicts_with = "view")]
        pretty: bool,

        /// View in pager
        #[structopt(short, long, conflicts_with = "pretty")]
        view: bool,

        /// View current day and year
        #[structopt(short, long, conflicts_with = "day")]
        now: bool,

        /// Problem day to view
        #[structopt(short, long, conflicts_with = "now")]
        day: Option<u32>,

        /// Don't use cache
        #[structopt(short, long)]
        force: bool,
    },

    /// Submit an answer for the current problem and level
    Submit {
        /// Your answer
        answer: String,
    },

    /// Fetch brief and input data, if available
    Fetch {
        /// Don't use cache
        #[structopt(short, long)]
        force: bool,

        /// Use current day and year
        #[structopt(short, long, conflicts_with = "day")]
        now: bool,

        /// Problem day to use
        #[structopt(short, long, conflicts_with = "now")]
        day: Option<u32>,
    },

    /// Get current status
    Status,

    /// Get summary of challenges and stars
    Summary {
        /// Specify the challenge year to view
        #[structopt(short, long)]
        year: Option<i32>,
    },

    /// Initialise an aocf repository
    Init,

    /// Set authentication token text
    SetCookie {
        /// Contents of authentication token to store
        token: String,
    },

    /// Get authentication token from firefox cookie store
    GetCookie,
}

#[derive(StructOpt, Debug)]
pub struct AocfTimeDateOpts {
    /// Check out current day and year
    #[structopt(short, long, conflicts_with_all = &["problem_day", "problem_year", "day", "year"])]
    now: bool,

    /// Problem day
    #[structopt(short, long)]
    day: Option<u32>,

    /// Problem year
    #[structopt(short, long)]
    year: Option<i32>,

    /// Problem day
    #[structopt(conflicts_with_all = &["now", "day"], required_unless_one = &["now", "day"])]
    problem_day: Option<u32>,

    /// Problem year
    #[structopt(conflicts_with_all = &["now", "year"])]
    problem_year: Option<i32>,
}

impl AocfTimeDateOpts {
    pub fn get_day_year(&self) -> (Option<u32>, Option<i32>) {
        match self {
            Self { now: true, .. } => {
                let now = Utc::now();
                (Some(now.day()), Some(now.year()))
            },
            Self { day: Some(d), year, .. } => {
                (Some(*d), *year)
            },
            Self { problem_day: Some(d), problem_year, .. } => {
                (Some(*d), *problem_year)
            },
            _ => (None, None),
        }
    }
}
