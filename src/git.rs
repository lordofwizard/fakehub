use chrono::{Duration, NaiveDate};
use git2::{Config, IndexAddOption, Repository, Signature};
use rand::Rng;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn get_global_git_username() -> Option<String> {
    let config = Config::open_default().ok()?;
    config.get_string("user.name").ok()
}

pub fn get_global_git_email() -> Option<String> {
    let config = Config::open_default().ok()?;
    config.get_string("user.email").ok()
}

pub fn is_git_repository(path: &str) -> bool {
    Repository::discover(Path::new(path)).is_ok()
}

pub fn create_directory(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create directory");
    }
}

pub fn create_commit_file(path: &Path) {
    let file_path = path.join("commit_maker.txt");
    fs::write(&file_path, "Initial commit").expect("Failed to write file");
}

pub fn add_and_commit(repo: &Repository, path: &Path) {
    let mut index = repo.index().expect("Failed to get index");
    index
        .add_path(Path::new("commit_maker.txt"))
        .expect("Failed to add file");
    index.write().expect("Failed to write index");

    let oid = index.write_tree().expect("Failed to write tree");
    let tree = repo.find_tree(oid).expect("Failed to find tree");
    let sig = Signature::now(
        &get_global_git_username().unwrap_or("Your Name".to_string()),
        &get_global_git_email().unwrap_or("your.email@example.com".to_string()),
    )
    .expect("Failed to create signature");

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .expect("Failed to create commit");
}

pub fn setup_git_folder(folder_name: &str) {
    let path = Path::new(folder_name);
    create_directory(path);

    if is_git_repository(folder_name) {
        let repo = Repository::open(folder_name).expect("Failed to open repository");
        if repo.is_empty().unwrap_or(false) {
            create_commit_file(path);
            add_and_commit(&repo, path);
        }
    } else {
        let repo = Repository::init(folder_name).expect("Failed to initialize git repository");
        create_commit_file(path);
        add_and_commit(&repo, path);
    }
}

/// Adds a blank line to commit_maker.txt and commits it with a date-based signature.
pub fn add_commits_in_date_range(
    repo_path: &str,
    start_date: &str,
    end_date: &str,
    commit_range_start: u32,
    commit_range_end: u32,
    fixed_number: u32,
) {
    let start =
        NaiveDate::parse_from_str(start_date, "%d-%m-%Y").expect("Invalid start date format");
    let end = NaiveDate::parse_from_str(end_date, "%d-%m-%Y").expect("Invalid end date format");
    let mut current_date = start;

    let username = get_global_git_username().expect("Failed to get Git username");
    let email = get_global_git_email().expect("Failed to get Git email");

    while current_date <= end {
        if fixed_number != 0 {
            for _ in 0..fixed_number {
                let file_path = format!("{}/commit_maker.txt", repo_path);
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_path)
                    .expect("Failed to open commit_maker.txt")
                    .write_all(b"\n")
                    .expect("Failed to write to commit_maker.txt");

                let formatted_date = current_date.format("%Y-%m-%d").to_string();
                Command::new("git")
                    .args(["add", "commit_maker.txt"])
                    .current_dir(repo_path)
                    .output()
                    .expect("Failed to add file to Git");

                Command::new("git")
                    .args([
                        "commit",
                        "-m",
                        "Automated commit",
                        "--author",
                        &format!("{} <{}>", username, email),
                        "--date",
                        &formatted_date,
                    ])
                    .current_dir(repo_path)
                    .output()
                    .expect("Failed to commit changes");
            }
        } else {
            for _ in 0..rand::rng().random_range(commit_range_start..=commit_range_end) {
                let file_path = format!("{}/commit_maker.txt", repo_path);
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_path)
                    .expect("Failed to open commit_maker.txt")
                    .write_all(b"\n")
                    .expect("Failed to write to commit_maker.txt");

                let formatted_date = current_date.format("%Y-%m-%d").to_string();
                Command::new("git")
                    .args(["add", "commit_maker.txt"])
                    .current_dir(repo_path)
                    .output()
                    .expect("Failed to add file to Git");

                Command::new("git")
                    .args([
                        "commit",
                        "-m",
                        "Automated commit",
                        "--author",
                        &format!("{} <{}>", username, email),
                        "--date",
                        &formatted_date,
                    ])
                    .current_dir(repo_path)
                    .output()
                    .expect("Failed to commit changes");
            }
        }

        current_date += Duration::days(1);
    }
}
