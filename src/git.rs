use git2::{Config, Repository};
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
