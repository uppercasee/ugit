pub struct Tree {
    entries: Vec<TreeEntry>,
}

pub struct TreeEntry {
    mode: String, // e.g., "100644" for file, "040000" for directory
    name: String,
    hash: String, // SHA-1 hash of the blob or tree object
}
