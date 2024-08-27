use super::index_read::{self, find_index, find_repo_root, read_from_file};


pub fn rm(objectfile: String) -> anyhow::Result<()> {
    let repo = find_repo_root()?;
    let index_file = find_index()?;
    let index_read = read_from_file(index_file)?;

    for line in index_read {
        println!("{:?}", line);
    }

    Ok(())
}
