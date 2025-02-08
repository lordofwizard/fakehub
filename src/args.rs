use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory path for git repository
    #[arg(short = 'd', long = "dir")]
    directory: Option<PathBuf>,

    /// Start date for commit generation (format: DD-MM-YYYY)
    #[arg(short = 's', long = "start")]
    start_date: Option<String>,

    /// End date for commit generation (format: DD-MM-YYYY)
    #[arg(short = 'e', long = "end")]
    end_date: Option<String>,

    /// Generate commits for specified number of days back from current date
    #[arg(short = 'b', long = "back")]
    days_back: Option<u32>,

    /// Minimum number of commits to generate per day
    #[arg(short = 'r', long = "range-start")]
    commit_range_start: Option<u32>,

    /// Maximum number of commits to generate per day
    #[arg(short = 'x', long = "range-end")]
    commit_range_end: Option<u32>,
}
