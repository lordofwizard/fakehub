use crate::args::Args;
use clap::Parser;

pub fn run() {
    let arg = Args::parse();
    let options = arg.run();

    let repository = options.0;
    let start_date = options.1;
    let end_date = options.2;
    let days_back = options.3;
    let commit_range_start = options.4;
    let commit_range_end = options.5;
    let fixed_number = options.6;

}
