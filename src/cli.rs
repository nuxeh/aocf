use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(about = "Advent of Code problem\n<https://github.com/nuxeh/aocf>")]
pub struct AocOpts {
    /// File to read as input
    pub input: Option<PathBuf>,
}

