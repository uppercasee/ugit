use std::fs;
use std::io;
use anyhow::{Result, Context};

fn create_directory(path: &str) -> Result<()> {
    fs::create_dir_all(path).with_context(|| "Error creating directory")?;

    Ok(())
}

pub fn init_git() -> Result<()> {
    create_directory("./ugit/objects/info")?;
    create_directory("./ugit/objects/pack")?;
    create_directory("./ugit/refs/heads")?;
    create_directory("./ugit/refs/tags")?;
    create_directory("./ugit/hooks")?;
    create_directory("./ugit/info")?;
    create_directory("./ugit/logs")?;

    fs::write("./ugit/HEAD", "ref: refs/heads/main\n")?;
    fs::write("./ugit/config", "[core]\n\trepositoryformatversion = 0\n\tbare = false\n")?;

    println!("Initialized git: Created directory structure in './ugit'");

    Ok(())
}

pub fn clear_git() -> Result<(), io::Error> {
    if let Err(err) = fs::remove_dir_all("./ugit") {
        eprintln!("Error: {}", err)
    } else {
        println!("Directory './ugit' removed successfully");
    }
    Ok(())
}
