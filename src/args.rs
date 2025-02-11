use chrono::{Duration, Local, NaiveDate};
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author="Advait Pandharpurkar", version="0.0.1", about="A Fake GitHub Commit history generator", long_about = None)]
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

    /// Minimum number of range of commits to generate per day (default 5)
    #[arg(short = 'r', long = "range-start")]
    commit_range_start: Option<u32>,

    /// Maximum number of range of commits to generate per day (default 10)
    #[arg(short = 'x', long = "range-end")]
    commit_range_end: Option<u32>,

    /// fixed number of commits (default 5)
    #[arg(short = 'q', long = "fixed-number")]
    fixed_number: Option<u32>,
}

impl Args {
    pub fn run(self) -> (PathBuf, String, String, u32, u32, u32, u32) {
        // Default directory is "./fakehub_repo"
        let directory = self.directory.unwrap_or_else(|| {
            let default_dir = PathBuf::from("./fakehub_repo");
            if !default_dir.exists() {
                fs::create_dir_all(&default_dir).expect("Failed to create directory");
            }
            default_dir
        });

        let today = Local::now().naive_local().date();

        // Handle dates based on days_back presence
        let (start_date, end_date) = if let Some(days) = self.days_back {
            // If days_back is provided, ignore start_date and end_date
            let end = today;
            let start = today - Duration::days(days as i64);
            (
                start.format("%d-%m-%Y").to_string(),
                end.format("%d-%m-%Y").to_string()
            )
        } else {
            // Use start_date and end_date logic only if days_back is None
            let default_start_date = today - Duration::days(365);
            (
                self.start_date.unwrap_or_else(|| default_start_date.format("%d-%m-%Y").to_string()),
                self.end_date.unwrap_or_else(|| today.format("%d-%m-%Y").to_string())
            )
        };

        // Default days_back (only used if both start_date and end_date are None)
        let days_back = self.days_back.unwrap_or(365);

        // Handle commit counts based on fixed_number presence
        let (commit_range_start, commit_range_end, fixed_number) = if let Some(fixed) = self.fixed_number {
            // If fixed_number is provided, ignore range values
            (0, 0, fixed)
        } else {
            // Use range values only if fixed_number is None
            (
                self.commit_range_start.unwrap_or(5),
                self.commit_range_end.unwrap_or(10),
                0
            )
        };

        (
            directory,
            start_date,
            end_date,
            days_back,
            commit_range_start,
            commit_range_end,
            fixed_number
        )
    }
}

// Helper function to parse date string (DD-MM-YYYY)
fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%d-%m-%Y").ok()
}
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert();
}
