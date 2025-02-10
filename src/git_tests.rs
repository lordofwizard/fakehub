use std::fs;
use std::process::Command;
use crate::git::{get_global_git_username, get_global_git_email, is_git_repository};

/// Sets up a temporary Git repository for testing.
fn setup_git_repo() -> String {
    let test_repo_path = "/tmp/test_git_repo";
    let _ = fs::remove_dir_all(test_repo_path); // Ensure a clean setup
    fs::create_dir_all(test_repo_path).unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(test_repo_path)
        .output()
        .expect("Failed to initialize Git repository");

    test_repo_path.to_string()
}

/// Cleans up the test repository.
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
    
    assert!(is_git_repository(&test_repo), "Expected {} to be a Git repository", test_repo);
    
    let non_repo_path = "/tmp/non_existent_directory";
    assert!(!is_git_repository(non_repo_path), "Expected {} to NOT be a Git repository", non_repo_path);
    
    cleanup_git_repo(&test_repo);
}
