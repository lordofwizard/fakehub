use git2::{Config, Repository, IndexAddOption, Signature};
use std::fs;
use std::path::Path;

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
    index.add_path(Path::new("commit_maker.txt")).expect("Failed to add file");
    index.write().expect("Failed to write index");

    let oid = index.write_tree().expect("Failed to write tree");
    let tree = repo.find_tree(oid).expect("Failed to find tree");
    let sig = Signature::now(
        &get_global_git_username().unwrap_or("Your Name".to_string()),
        &get_global_git_email().unwrap_or("your.email@example.com".to_string())
    ).expect("Failed to create signature");

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).expect("Failed to create commit");
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