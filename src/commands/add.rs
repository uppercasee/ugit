use anyhow::{Context, Result};

use crate::{Index, IndexEntry};

pub fn add_to_index(objectfile: String) -> Result<()> {
    // TODO: Not optimized at all. We should make sure that
    // there is any change in the file before updating the index.
    // We just add the file no matter what.
    
    let mut index = Index::default().read()?; // Read the index

    let entry = IndexEntry::update_index(&objectfile)
        .with_context(|| format!("Failed to create index entry for file: {}", objectfile))?;

    index.add_entries(entry);

    index
        .write()
        .with_context(|| "Failed to write updated index file")?;

    println!("File added to index: {}", objectfile);

    Ok(())
}
