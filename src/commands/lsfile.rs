use anyhow::Result;
use crate::repository::Index;

pub fn index_read() -> Result<()> {
    let index = Index::default();
    let index = index.read()?;

    for entry in &index.entries {
        println!("{}", entry.path);
    }

    Ok(())
}
