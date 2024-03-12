use anyhow::Context;
use std::io::Write;
// use walkdir::WalkDir;


pub fn write_tree(treeish: String) -> anyhow::Result<()> {
    // You're expected to write the entire working directory as a tree object and print the 40-char SHA to stdout.
    // ignore .git directory

    // let mut entries = Vec::new();
    // let mut paths = Vec::new();
    // for entry in walkdir::WalkDir::new(".") {
    //     let entry = entry?;
    //     let path = entry.path();
    //     if path.starts_with("./.git") {
    //         println!("skipping .git");
    //         continue;
    //     }
    //     let relative_path = path.strip_prefix(".").context("couldn't strip prefix")?;
    //     let relative_path = relative_path.to_str().context("couldn't convert to str")?;
    //     paths.push(relative_path);
    // }
    // paths.sort();
    // for path in paths {
    //     let contents = std::fs::read(path).context("couldn't read file")?;
    // }
    //
    Ok(())
}
