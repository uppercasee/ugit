use anyhow::{Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use ignore::WalkBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;

use crate::{find_index, hash_objects};

#[derive(Debug)]
pub struct IndexEntry {
    pub mode: u16,  // 2 bytes 100644 -> file and 040000 -> directory
    file_size: u32, // 4 bytes file size in bytes
    mtime: u32,     // 4 bytes last modified time in seconds since the epoch
    // TODO: ctime: u32,
    sha1: [u8; 20],
    pub path: String,
    // TODO: Flags
}

pub struct Index {
    signature: [u8; 4],
    version: u32,
    number_of_entries: u32,
    pub entries: Vec<IndexEntry>,
}

impl Default for Index {
    fn default() -> Self {
        let index_path = "./ugit/index";

        // Check if the index file exists
        if !std::path::Path::new(index_path).exists() {
            // Create the index file if it doesn't exist
            let mut file = File::create(index_path).expect("Failed to create index file");

            // Write the signature "DIRC"
            file.write_all(b"DIRC").expect("Failed to write signature");

            // Write the version (4 bytes, little-endian)
            file.write_u32::<LittleEndian>(2)
                .expect("Failed to write version");

            // Write the entry count (4 bytes, little-endian)
            file.write_u32::<LittleEndian>(0)
                .expect("Failed to write entry count");
        }

        // Return a default `Index` instance
        Index {
            signature: *b"DIRC",  // Default signature for Git index files
            version: 2,           // Default version
            number_of_entries: 0, // Default number of entries
            entries: Vec::new(),  // Empty vector for entries
        }
    }
}

impl Index {
    pub fn add_entries(&mut self, entries: Vec<IndexEntry>) {
        // this helps remove duplicate entries....
        self.entries
            .retain(|entry| !entries.iter().any(|new_entry| new_entry.path == entry.path));

        self.entries.extend(entries);
        self.number_of_entries = self.entries.len() as u32;
    }

    pub fn remove_entry(&mut self, path: &str) {
        self.entries.retain(|entry| entry.path != path);
        self.number_of_entries = self.entries.len() as u32;
    }

    pub fn write(&self) -> Result<()> {
        // Find and open the index file
        let index_file = find_index()?;
        let mut file = File::create(&index_file)
            .with_context(|| format!("Failed to create file: {}", index_file.display()))?;

        // Write the signature "DIRC"
        file.write_all(&self.signature)
            .with_context(|| "Failed to write signature to index file")?;

        // Write the version (4 bytes, little-endian)
        file.write_u32::<LittleEndian>(self.version)
            .with_context(|| "Failed to write version to index file")?;

        // Write the entry count (4 bytes, little-endian)
        file.write_u32::<LittleEndian>(self.number_of_entries)
            .with_context(|| "Failed to write entry count to index file")?;

        // Write the entries
        for entry in &self.entries {
            file.write_all(&entry.to_bytes())
                .with_context(|| "Failed to write entry to index file")?;
        }

        Ok(())
    }

    pub fn read(&self) -> Result<Index> {
        // Find and open the index file
        let index_file = find_index()?;
        let file = File::open(&index_file)
            .with_context(|| format!("Failed to open file: {}", index_file.display()))?;

        let mut reader = BufReader::new(file);

        // Read and validate the signature (4 bytes)
        let mut signature = [0u8; 4];
        reader
            .read_exact(&mut signature)
            .with_context(|| "Failed to read signature from index file")?;
        if signature != *b"DIRC" {
            return Err(anyhow::anyhow!("Invalid signature in index file"));
        }

        // Read the version (4 bytes)
        let mut version_bytes = [0u8; 4];
        reader
            .read_exact(&mut version_bytes)
            .with_context(|| "Failed to read version from index file")?;
        let version = u32::from_le_bytes(version_bytes);
        if version != self.version {
            return Err(anyhow::anyhow!("Invalid version in index file"));
        }

        // Read the entry count (4 bytes)
        let mut entry_count_bytes = [0u8; 4];
        reader
            .read_exact(&mut entry_count_bytes)
            .with_context(|| "Failed to read entry count from index file")?;
        let number_of_entries = u32::from_le_bytes(entry_count_bytes);

        // Read the entries
        let mut entries = Vec::new();
        for _ in 0..number_of_entries {
            let mut mode_bytes = [0u8; 2];
            reader
                .read_exact(&mut mode_bytes)
                .with_context(|| "Failed to read mode from index file")?;
            let mode = u16::from_le_bytes(mode_bytes);

            let mut file_size_bytes = [0u8; 4];
            reader
                .read_exact(&mut file_size_bytes)
                .with_context(|| "Failed to read file size from index file")?;
            let file_size = u32::from_le_bytes(file_size_bytes);

            let mut mtime_bytes = [0u8; 4];
            reader
                .read_exact(&mut mtime_bytes)
                .with_context(|| "Failed to read mtime from index file")?;
            let mtime = u32::from_le_bytes(mtime_bytes);

            let mut sha1 = [0u8; 20];
            reader
                .read_exact(&mut sha1)
                .with_context(|| "Failed to read sha1 from index file")?;

            let mut path_bytes = Vec::new();
            reader
                .read_until(0, &mut path_bytes)
                .with_context(|| "Failed to read path from index file")?;
            let path = String::from_utf8(path_bytes[..path_bytes.len() - 1].to_vec())
                .with_context(|| "Failed to convert path to string")?;

            entries.push(IndexEntry {
                mode,
                file_size,
                mtime,
                sha1,
                path,
            });
        }

        // Return the entry count and lines
        Ok(Index {
            signature,
            version,
            number_of_entries,
            entries,
        })
    }
}

impl IndexEntry {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.mode.to_le_bytes());
        bytes.extend_from_slice(&self.file_size.to_le_bytes());
        bytes.extend_from_slice(&self.mtime.to_le_bytes());
        bytes.extend_from_slice(&self.sha1);
        bytes.extend_from_slice(self.path.as_bytes());
        bytes.push(0);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<IndexEntry> {
        let mode = u16::from_le_bytes([bytes[0], bytes[1]]);
        let file_size = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        let mtime = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);
        let mut sha1 = [0; 20];
        sha1.copy_from_slice(&bytes[10..30]);
        let path = String::from_utf8(bytes[30..].to_vec())
            .with_context(|| "Failed to convert path to string")?;
        Ok(IndexEntry {
            mode,
            file_size,
            mtime,
            sha1,
            path,
        })
    }

    pub fn get_sha(path: &str) -> Result<[u8; 20]> {
        // Check if the path is a directory
        if std::fs::metadata(path)?.is_dir() {
            return Err(anyhow::anyhow!("Directories are not supported"));
        }
    
        // If the path is a valid file, compute its SHA hash
        let sha = hash_objects(path)?;
        Ok(sha)
    }
    

    pub fn update_index(path: &str) -> Result<Vec<IndexEntry>> {
        let mut is_first_entry = true;
        let mut path_entries = Vec::new();
        
        if std::fs::metadata(path)?.is_dir() {
            // If the path is a directory, walk through it and collect all the files
            for entry in WalkBuilder::new(path)
                .hidden(true)
                .git_ignore(true)
                .git_exclude(false)
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
                    path_entries.push(path.to_string()); // Collect the path as a String
                                                    // println!("{}", path);
                }
            }
            // sort entries
            path_entries.sort();
        } else {
            path_entries.push(path.to_string());
        }

        let mut entries = Vec::new();
        for entry in path_entries {
            let metadata = std::fs::metadata(&entry).context("couldn't get metadata")?;
            let is_dir = metadata.is_dir();

            if is_dir {
                continue;
            }

            let sha = IndexEntry::get_sha(&entry)?;

            let mode = if metadata.is_dir() {
                    0o040000 // Directory
                } else if metadata.file_type().is_symlink() {
                    0o120000 // Symbolic link
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    0o100755 // Executable file
                } else {
                    0o100644 // Regular file
                };

            let entry = IndexEntry {
                mode,
                file_size: metadata.len() as u32,
                mtime: metadata.modified()?.duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs() as u32,
                sha1: sha,
                path: entry,
            };
            entries.push(entry);
        }
        Ok(entries)
    }
}
