use clap::{Parser, Subcommand};

/// Command-line arguments for the application.
#[derive(Parser, Debug)]
#[clap(version, about = "A tool for managing Git-like operations")]
pub struct Args {
    /// The command to execute.
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands for the application.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clears the Git repository.
    Clear,

    /// Initializes a new Git repository.
    Init,

    /// Displays the contents of a file in a Git repository.
    CatFile {
        /// Pretty print the file content.
        #[clap(short, long)]
        pretty_print: bool,

        /// The hash of the object to display.
        object_hash: String,
    },

    /// Calculates the hash of a given object file.
    HashObject {
        /// Path to the object file.
        objectfile: String,
    },

    /// Lists the contents of a tree object in a Git repository.
    LsTree {
        /// The hash of the object to list.
        object_hash: String,
    },

    /// Writes a tree object to the Git repository.
    WriteTree {
        /// The tree to write.
        tree: String,
    },

    /// Adds a file to the staging area.
    Add {
        /// Path to the object file.
        objectfile: String,
    },
    
    /// Lists the index file
    LsFiles {},
}
