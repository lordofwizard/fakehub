use crate::{
    args::Args,
    git::{add_commits_in_date_range, setup_git_folder},
};
use clap::Parser;

pub fn run() {
    let arg = Args::parse();
    let options = arg.run();

    let repository = options.0.to_str().unwrap();
    let start_date = options.1.as_str();
    let end_date = options.2.as_str();
    let days_back = options.3;
    let commit_range_start = options.4;
    let commit_range_end = options.5;
    let fixed_number = options.6;

    setup_git_folder(repository);

    add_commits_in_date_range(repository, start_date, end_date,commit_range_start,commit_range_end,fixed_number);
}
