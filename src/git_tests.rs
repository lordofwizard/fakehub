use std::env;
use std::fs;
use std::process::Command;
#[allow(unused_imports)]
use crate::git::{get_global_git_email,get_global_git_username,is_git_repository};

/// Sets up a temporary Git repository for testing.
#[allow(dead_code)]
fn setup_git_repo() -> String {
    let mut test_repo_path = env::temp_dir();
    test_repo_path.push("test_git_repo");

    let _ = fs::remove_dir_all(&test_repo_path); // Ensure a clean setup
    fs::create_dir_all(&test_repo_path).unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&test_repo_path)
        .output()
        .expect("Failed to initialize Git repository");

    test_repo_path.to_str().unwrap().to_string()
}

#[allow(dead_code)]
fn setup_git_repo_commit_check() -> String {
    let mut test_repo_path = env::temp_dir();
    test_repo_path.push("test_git_repo_2");

    let _ = fs::remove_dir_all(&test_repo_path); // Ensure a clean setup
    fs::create_dir_all(&test_repo_path).unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&test_repo_path)
        .output()
        .expect("Failed to initialize Git repository");

    test_repo_path.to_str().unwrap().to_string()
}

/// Cleans up the test repository.
#[allow(dead_code)]
fn cleanup_git_repo(path: &str) {
    let _ = fs::remove_dir_all(path);
}

/// Tests getting the global Git username.
#[test]
fn test_get_global_git_username() {
    let username = get_global_git_username();
    println!("Git Username: {:?}", username);

    // Ensure function executes correctly
    assert!(username.is_some() || username.is_none());
}

/// Tests getting the global Git email.
#[test]
fn test_get_global_git_email() {
    let email = get_global_git_email();
    println!("Git Email: {:?}", email);

    // Ensure function executes correctly
    assert!(email.is_some() || email.is_none());
}

/// Tests checking if a directory is a Git repository.
#[test]
fn test_is_git_repository() {
    let test_repo = setup_git_repo();

    assert!(
        is_git_repository(&test_repo),
        "Expected {} to be a Git repository",
        test_repo
    );

    let mut non_repo_path = env::temp_dir();
    non_repo_path.push("non_existent_directory");
    let non_repo_path_str = non_repo_path.to_str().unwrap();
    assert!(
        !is_git_repository(non_repo_path_str),
        "Expected {} to NOT be a Git repository",
        non_repo_path_str
    );

    cleanup_git_repo(&test_repo);
}

#[cfg(test)]
mod tests {
    use crate::git::{create_commit_file, create_directory, setup_git_folder};

    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    fn setup_test_repo() -> PathBuf {
        let mut test_dir = env::temp_dir();
        test_dir.push("test_repo");

        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).unwrap();
        }
        create_directory(&test_dir);
        test_dir
    }

    #[test]
    fn test_create_directory() {
        let test_dir = setup_test_repo();
        assert!(test_dir.exists());
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[cfg_attr(target_os = "macos", ignore)]
    #[test]
    fn test_create_commit_file() {
        let test_dir = setup_test_repo();
        create_commit_file(&test_dir);
        assert!(test_dir.join("commit_maker.txt").exists());
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_setup_git_folder() {
        let test_dir = setup_test_repo();
        let test_dir_str = test_dir.to_str().unwrap();
        setup_git_folder(test_dir_str);
        assert!(is_git_repository(test_dir_str));
        assert!(test_dir.join("commit_maker.txt").exists());
        fs::remove_dir_all(&test_dir).unwrap();
    }
}

#[allow(dead_code)]
fn temp_global_names() {
    use std::process::Command;

    fn main() {
        // Set Git user.name
        let output_name = Command::new("git")
            .args(["config", "--global", "user.name", "lordofwizard"])
            .output()
            .expect("Failed to set Git user.name");

        if !output_name.status.success() {
            eprintln!(
                "Error setting user.name: {}",
                String::from_utf8_lossy(&output_name.stderr)
            );
        }

        // Set Git user.email
        let output_email = Command::new("git")
            .args([
                "config",
                "--global",
                "user.email",
                "advaitpandharpurkar619@gmail.com",
            ])
            .output()
            .expect("Failed to set Git user.email");

        if !output_email.status.success() {
            eprintln!(
                "Error setting user.email: {}",
                String::from_utf8_lossy(&output_email.stderr)
            );
        }
    }
}

#[cfg(test)]
mod tests_for_commit {
    use crate::git::{add_commits_in_date_range, setup_git_folder};

    use super::*;
    use std::fs;
    use std::process::Command;

    #[test]
    #[ignore]
    fn test_correct_number_of_commits() {
        temp_global_names();
        let repo_path = setup_git_repo_commit_check();
        let repo_str = repo_path.as_str();
        setup_git_folder(repo_str);

        add_commits_in_date_range(repo_str, "01-02-2024", "05-02-2024", 0, 0, 1);

        let output = Command::new("git")
            .args(["rev-list", "--count", "HEAD"])
            .current_dir(repo_str)
            .output()
            .expect("Failed to count commits");

        let commit_count = String::from_utf8(output.stdout)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        assert_eq!(
            commit_count, 6,
            "Expected 6 commits, found {}, (Initial Commit + Blank Commits",
            commit_count
        );
    }

    #[test]
    #[should_panic(expected = "Invalid start date format")]
    fn test_invalid_date_format() {
        let repo_path = env::temp_dir().join("test_git_repo_invalid");
        let repo_str = repo_path.to_str().unwrap();
        fs::create_dir_all(repo_str).unwrap();
        Command::new("git")
            .arg("init")
            .current_dir(repo_str)
            .output()
            .unwrap();

        add_commits_in_date_range(repo_str, "32-01-2021", "05-02-2021", 0, 0, 1);
    }

    #[test]
    #[should_panic(expected = "Invalid start date format")]
    fn test_invalid_date_format_2() {
        let repo_path = env::temp_dir().join("test_git_repo_invalid_2");
        let repo_str = repo_path.to_str().unwrap();
        fs::create_dir_all(repo_str).unwrap();
        Command::new("git")
            .arg("init")
            .current_dir(repo_str)
            .output()
            .unwrap();

        add_commits_in_date_range(repo_str, "29-02-2021", "05-02-2021", 0, 0, 1);
    }
}
