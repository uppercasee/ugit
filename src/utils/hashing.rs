use anyhow::{Context, Result};
use std::fs;

/// Given a hash, return the full path to the object file.
pub fn get_full_path_from_hash(hash: &str) -> Result<String> {
    let prefix = &hash[..2];
    let suffix = &hash[2..];

    let object_dir = format!("./ugit/objects/{}", prefix);

    // Search for a matching file in the directory
    let entries = fs::read_dir(object_dir).context("Couldn't read the object directory")?;

    for entry in entries {
        let entry = entry.context("Couldn't get entry in object directory")?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with(suffix) {
            return Ok(format!("{}/{}", prefix, file_name));
        }
    }

    anyhow::bail!("No matching object found for the given hash prefix.")
}
