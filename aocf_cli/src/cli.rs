use chrono::{Datelike, Utc};
use clap::{Args, CommandFactory, Parser};
use clap_complete::Shell;

pub fn generate_completion(shell: Shell) {
    clap_complete::generate(shell, &mut Aocf::command(), "aocf", &mut std::io::stdout());
}

/// Advent of Code Swiss army knife
#[derive(Parser, Debug)]
#[clap(version)]
pub enum Aocf {
    /// Switch to a specified year and day
    Checkout(AocfTimeDateOpts),

    /// Get input data for the current problem
    Input {
        /// View in pager
        #[clap(short, long)]
        view: bool,

        /// Don't use cache
        #[clap(short, long)]
        force: bool,

        /// Show input data stats
        #[clap(short, long, conflicts_with = "view")]
        info: bool,
    },

    /// Get instructions for the current problem
    Brief {
        /// View pretty
        #[clap(short, long, conflicts_with = "view")]
        pretty: bool,

        /// View in pager
        #[clap(short, long, conflicts_with = "pretty")]
        view: bool,

        /// View in web browser
        #[clap(short, long, conflicts_with_all = &["pretty", "view"])]
        web: bool,

        /// View current day and year
        #[clap(short, long, conflicts_with = "day")]
        now: bool,

        /// Problem day to view
        #[clap(short, long, conflicts_with = "now")]
        day: Option<u32>,

        /// Don't use cache
        #[clap(short, long)]
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
        #[clap(short, long)]
        force: bool,

        /// Use current day and year
        #[clap(short, long, conflicts_with = "day")]
        now: bool,

        /// Problem day to use
        #[clap(short, long, conflicts_with = "now")]
        day: Option<u32>,
    },

    /// Get current status
    Status,

    /// Get summary of challenges and stars
    Summary {
        /// Specify the challenge year to view
        #[clap(short, long)]
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

    /// Generate shell completion script
    Completion {
        /// Shell type
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Args, Debug)]
pub struct AocfTimeDateOpts {
    /// Check out current day and year
    #[clap(short, long, conflicts_with_all = &["problem-day", "problem-year", "day", "year"])]
    now: bool,

    /// Problem day
    #[clap(short, long)]
    day: Option<u32>,

    /// Problem year
    #[clap(short, long)]
    year: Option<i32>,

    /// Problem day
    #[clap(conflicts_with_all = &["now", "day"], required_unless_present_any = &["now", "day"])]
    problem_day: Option<u32>,

    /// Problem year
    #[clap(conflicts_with_all = &["now", "year"])]
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
