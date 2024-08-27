use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use anyhow::{Context, Result};

use crate::find_index;

pub fn index_read() -> Result<(u32, Vec<String>)> {
    // Find and open the index file
    let index_file = find_index()?;
    let file = File::open(&index_file)
        .with_context(|| format!("Failed to open file: {}", index_file.display()))?;

    let mut reader = BufReader::new(file);

    // Read and validate the signature (4 bytes)
    let mut signature = [0u8; 4];
    reader.read_exact(&mut signature)
        .with_context(|| "Failed to read signature from index file")?;
    if &signature != b"DIRC" {
        return Err(anyhow::anyhow!("Invalid signature in index file").into());
    }

    // Read the version (4 bytes)
    let mut version_bytes = [0u8; 4];
    reader.read_exact(&mut version_bytes)
        .with_context(|| "Failed to read version from index file")?;
    let _version = u32::from_le_bytes(version_bytes);

    // Read the entry count (4 bytes)
    let mut entry_count_bytes = [0u8; 4];
    reader.read_exact(&mut entry_count_bytes)
        .with_context(|| "Failed to read entry count from index file")?;
    let entry_count = u32::from_le_bytes(entry_count_bytes);

    // Read the lines (if any)
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("Failed to read lines from file: {}", index_file.display()))?;

    // Return the entry count and lines
    Ok((entry_count, lines))
}