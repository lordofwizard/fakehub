mod args;
mod git;
mod git_tests;

use crate::args::Args;
use clap::Parser;
use crate::git::*;


fn main() {
    let _arg = Args::parse();
}
