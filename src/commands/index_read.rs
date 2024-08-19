use std::{fs::File, io::{self, BufRead, BufReader}};
use anyhow::{Context, Result};
use std::path::PathBuf;


pub fn read_from_file(index_file: PathBuf) -> Result<Vec<String>> {
    let file = File::open(&index_file)
        .with_context(|| format!("Failed to open file: {}", index_file.display()))?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("Failed to read lines from file: {}", index_file.display()))?;

    Ok(lines)
}

pub fn index_read() -> Result<()> {
    let index_file = find_index();
    let lines = read_from_file(index_file?)?;

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

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
    Err(io::Error::new(io::ErrorKind::NotFound, "Git repository not found"))
}

pub fn find_index() -> Result<PathBuf, io::Error> {
    let repo_root = find_repo_root()?;
    let index_path = repo_root.join("ugit").join("index");

    if index_path.exists() && index_path.is_file() {
        Ok(index_path)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, ".git/index file not found"))
    }
}
