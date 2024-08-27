use crate::hash_objects;
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use ignore::WalkBuilder;
use crypto_hash::{hex_digest, Algorithm};


pub fn write_tree(tree: String) -> anyhow::Result<String> {
    let mut is_first_entry = true;
    let mut entries = Vec::new();
    for entry in WalkBuilder::new(tree)
        .hidden(true)
        .git_ignore(true)
        .git_exclude(false)
        .max_depth(Some(1))
        .build()
    {
        let entry = entry?;
        let path = entry.path();
        let path = path.strip_prefix("./")?;
        let path = path.to_str().context("couldn't convert path to string")?;

        if is_first_entry {
            is_first_entry = false;
            continue; // Skip processing the first entry
        }

        if !path.trim().is_empty() {
            entries.push(path.to_string()); // Collect the path as a String
                                            // println!("{}", path);
        }
    }
    // sort entries
    entries.sort();

    let mut tree = Vec::new();
    for entry in entries {
        // println!("{}", entry);
        // check if entry is a file or directory
        let metadata = std::fs::metadata(&entry).context("couldn't get metadata")?;
        if metadata.is_file() {
            let hash_vec = hash_objects(&entry)?;
            let hash = hex::encode(hash_vec);
            let mode = "100644";
            let entry_line = format!("{} blob {} {}", mode, entry, hash);
            // println!("{}", entry_line);
            tree.push(entry_line);
        } else if metadata.is_dir() {
            let new_entry = format!("./{}", entry);
            let mode = "040000";
            let hash = write_tree(new_entry)?;
            let entry_line = format!("{} tree {} {}", mode, entry, hash);
            tree.push(entry_line);
        }
    }

    // Write tree to object file
    let tree_content = tree.join("\n");
    // println!("{}", tree_content);
    let header = format!("tree {}\0", tree_content.len());
    let mut data = header.into_bytes();
    data.append(&mut tree_content.into_bytes());

    // println!("{:?}", data);

    let hash = store_tree_object(&String::from_utf8(data).context("couldn't convert data to string")?)?;
    // println!("{}", hash);
    Ok(hash)
}

pub fn store_tree_object(tree_content: &str) -> Result<String> {
    // Calculate the SHA-1 hash of the tree object content
    let hash = hex_digest(Algorithm::SHA1, tree_content.as_bytes());

    // Construct the object file path
    let object_path = Path::new("ugit/objects")
        .join(&hash[0..2])
        .join(&hash[2..]);

    // Create directories if they don't exist
    fs::create_dir_all(object_path.parent().unwrap()).context("couldn't create object directory")?;

    // Write the tree object content to the object file
    let mut file = File::create(&object_path)
        .with_context(|| format!("couldn't create object file: {:?}", object_path))?;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(tree_content.as_bytes()).context("couldn't write to encoder")?;
    let compressed_data = encoder.finish().context("couldn't finish encoding")?;
    file.write_all(&compressed_data).context("couldn't write to object file")?;

    Ok(hash)
}
