use super::index_read::{find_index, read_from_file};


pub fn rm() -> anyhow::Result<()> {
    // let repo = find_repo_root()?;
    let index_file = find_index()?;
    let index_read = read_from_file(index_file)?;

    for line in index_read {
        println!("{:?}", line);
    }

    Ok(())
}
