use anyhow::{Context, Result};
use crate::Index;

pub fn rm(cached: bool, objectfile: String) -> Result<()> {
    let mut index = Index::default().read()?; // Read the index

    index.remove_entry(&objectfile);

    index
        .write()
        .with_context(|| "Failed to write updated index file")?;

    if !cached {
        std::fs::remove_file(&objectfile).context("couldn't remove file")?;
        println!("File removed: {}", objectfile);
    }
    else {
        println!("File removed from index: {}", objectfile);
    }


    Ok(())
}
