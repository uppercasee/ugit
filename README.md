# ugit (Unusual Git)

**ugit** is an unusual Git implementation written in Rust which is currently in development, with a limited set of commands available. 
*Also I have no idea when I will ever complete this project.*

## Features
- **Command Line Interface:** Provides a simple and intuitive command-line interface for version control operations.
- **Partial Functionality:** Offers a subset of Git commands, with more functionality under development.

## Commands
- **clear:** Delete the git repository.(made only for development)
- **init:** Initialize a new Git repository.
- **cat-file:** Provide content or type and size information for repository objects. (currently only blob)
- **hash-object:** Compute the object ID and creates a blob from a file.
- **ls-tree:** List the contents of a tree object.

## Planned Features
- [x] **write-tree:** Write a tree object from the current index.
- [ ] **commit-tree:** Create a new commit object.
- [ ] **clone:** Clone a repository into a new directory.
- [ ] **branch:** Create, list, rename, or delete branches.
- [ ] **checkout:** Switch branches or restore working tree files.
- [ ] **merge:** Join two or more development histories together.
- [ ] **rebase:** Reapply commits on top of another base tip.
- [ ] **log:** Show commit logs.
- [ ] **status:** Show the status of working tree files.
- [ ] **diff:** Show changes between commits, commit and working tree, etc.
- [ ] **remote:** Manage set of tracked repositories.
- [ ] **fetch:** Download objects and refs from another repository.
- [ ] **push:** Update remote refs along with associated objects.
- [ ] **pull:** Fetch from and integrate with another repository or a local branch.

## Getting Started
To get started with ugit, you can clone the repository and build it locally. 

```bash
git clone https://github.com/uppercasee/ugit.git
cd ugit
cargo build --release
```

## Usage
After building ugit, you can start using it with the available commands. 

```bash
# Initialize a new repository
ugit init

# Add files to the repository
ugit hash-object <file_name>

# List the contents of a tree object
ugit ls-tree <tree_object_id>

# Print help information
ugit help
```

## References
- [Build Your Own Git](https://github.com/codecrafters-io/build-your-own-git)
- [Write Yourself a Git](https://wyag.thb.lt/)
- [PyGit: A Git implementation in Python](https://benhoyt.com/writings/pygit/)

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to help improve ugit.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

