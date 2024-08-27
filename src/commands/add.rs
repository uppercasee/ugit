use anyhow::{Context, Result};

use crate::{Index, IndexEntry};

pub fn add_to_index(objectfile: String) -> Result<()> {
    let index = Index::default();

    let mut index = index.read()?;

    let entry = IndexEntry::from_path(&objectfile)
        .with_context(|| format!("Failed to create index entry for file: {}", objectfile))?;

    // Step 3: Add or update the entry in the index
    index.remove_entry(&objectfile); // Ensure no duplicate entries
    index.add_entries(vec![entry]);

    // Step 4: Write the updated index to the index file
    index
        .write()
        .with_context(|| "Failed to write updated index file")?;

    println!("File added to index: {}", objectfile);

    Ok(())
}
