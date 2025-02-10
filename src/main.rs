mod args;
mod git;
mod time;
mod git_tests;

use crate::args::Args;
use clap::Parser;


fn main() {
    let _arg = Args::parse();
}
