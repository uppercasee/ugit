use std::io;
use std::path::PathBuf;
use anyhow::Result;

pub fn find_repo_root() -> Result<PathBuf, io::Error> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        let git_dir_path = current_dir.join("ugit");

        if git_dir_path.exists() && git_dir_path.is_dir() {
            return Ok(current_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Git repository not found",
    ))
}

pub fn find_index() -> Result<PathBuf, io::Error> {
    let repo_root = find_repo_root()?;
    let index_path = repo_root.join("ugit").join("index");

    if index_path.exists() && index_path.is_file() {
        Ok(index_path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            ".git/index file not found",
        ))
    }
}
