use std::{fs::File, io::{BufRead, BufReader}};
use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::find_index;

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