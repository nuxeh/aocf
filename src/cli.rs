use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about = "Advent of Code problem\n<https://github.com/nuxeh/aocf>")]
pub struct AocOpts {
    /// File to read as input
    pub input: Option<PathBuf>,
}

