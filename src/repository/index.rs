#[derive(Debug)]
pub struct IndexEntry {
    mode: u16,
    inode: u32,
    dev: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    mtime: u32,
    ctime: u32,
    sha1: [u8; 20],
    path: String,
}

pub struct Index {
    signature: [u8; 4],
    version: u32,
    number_of_entries: u32,
    entries: Vec<IndexEntry>,
}

impl Index {
    pub fn file_exists(&self, path: &str) -> bool {
        self.entries.iter().any(|entry| entry.path == path)
    }

    pub fn add_entry(&mut self, entry: IndexEntry) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, path: &str) {
        self.entries.retain(|entry| entry.path != path);
    }

    pub fn get_entry(&self, path: &str) -> Option<&IndexEntry> {
        self.entries.iter().find(|entry| entry.path == path)
    }
}
