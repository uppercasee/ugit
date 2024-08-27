use anyhow::{Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

use crate::{find_index, hash_objects};

#[derive(Debug)]
pub struct IndexEntry {
    mode: u16,      // 2 bytes 100644 -> file and 040000 -> directory
    file_size: u32, // 4 bytes file size in bytes
    mtime: u32,     // 4 bytes last modified time in seconds since the epoch
    ctime: u32,     // 4 bytes last changed time in seconds since the epoch
    sha1: [u8; 20],
    path: String,
}

pub struct Index {
    signature: [u8; 4],
    version: u32,
    number_of_entries: u32,
    entries: Vec<IndexEntry>,
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
            file.write_u32::<LittleEndian>(4).expect("Failed to write version");

            // Write the entry count (4 bytes, little-endian)
            file.write_u32::<LittleEndian>(0).expect("Failed to write entry count");
        }

        // Return a default `Index` instance
        Index {
            signature: *b"DIRC",  // Default signature for Git index files
            version: 4,           // Default version
            number_of_entries: 0, // Default number of entries
            entries: Vec::new(),  // Empty vector for entries
        }
    }
}

impl Index {
    pub fn add_entries(&mut self, entries: Vec<IndexEntry>) {
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
        if signature != self.signature {
            return Err(anyhow::anyhow!("Invalid signature in index file").into());
        }

        // Read the version (4 bytes)
        let mut version_bytes = [0u8; 4];
        reader
            .read_exact(&mut version_bytes)
            .with_context(|| "Failed to read version from index file")?;
        let version = u32::from_le_bytes(version_bytes);
        if version != self.version {
            return Err(anyhow::anyhow!("Invalid version in index file").into());
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

            let mut ctime_bytes = [0u8; 4];
            reader
                .read_exact(&mut ctime_bytes)
                .with_context(|| "Failed to read ctime from index file")?;
            let ctime = u32::from_le_bytes(ctime_bytes);

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
                ctime,
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
        bytes.extend_from_slice(&self.ctime.to_le_bytes());
        bytes.extend_from_slice(&self.sha1);
        bytes.extend_from_slice(self.path.as_bytes());
        bytes.push(0);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<IndexEntry> {
        let mode = u16::from_le_bytes([bytes[0], bytes[1]]);
        let file_size = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        let mtime = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);
        let ctime = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]);
        let mut sha1 = [0; 20];
        sha1.copy_from_slice(&bytes[14..34]);
        let path = String::from_utf8(bytes[34..].to_vec())
            .with_context(|| "Failed to convert path to string")?;
        Ok(IndexEntry {
            mode,
            file_size,
            mtime,
            ctime,
            sha1,
            path,
        })
    }

    pub fn from_path(path: &str) -> Result<IndexEntry> {
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to read metadata for path: {}", path))?;

        let mode = if metadata.is_dir() {
            0o040000
        } else {
            0o100644
        };

        let file_size = metadata.len() as u32;

        let mtime = metadata
            .modified()?
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs() as u32;

        let ctime = metadata
            .created()
            .ok()
            .and_then(|created_time| {
                created_time
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .ok()
                    .map(|dur| dur.as_secs() as u32)
            })
            .unwrap_or(0); // Use a default value of 0 if ctime is not available

        let sha1 = hash_objects(&path.to_string())?;

        println!("mode: {:o}, file_size: {}, mtime: {}, ctime: {}, sha1: {:?}, path: {}", mode, file_size, mtime, ctime, sha1, path);

        Ok(IndexEntry {
            mode,
            file_size,
            mtime,
            ctime,
            sha1,
            path: path.to_string(),
        })
    }
}
