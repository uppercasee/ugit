use std::fs;
use std::io;

fn create_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path).map_err(|err| {
        eprintln!("Error creating directory '{}': {}", path, err);
        err
    })
}

pub fn init_git() -> Result<(), io::Error> {
    create_directory("./ugit/objects/info")?;
    create_directory("./ugit/objects/pack")?;
    create_directory("./ugit/refs/heads")?;
    create_directory("./ugit/refs/tags")?;
    fs::write("./ugit/HEAD", "ref: refs/heads/master\n")?;
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
