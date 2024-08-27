pub struct Commit {
    tree: String,                // SHA-1 hash of the tree object
    parent_commits: Vec<String>, // SHA-1 hashes of parent commits
    author: String,
    committer: String,
    message: String,
}
