mod init;
mod commands;
mod cli;

pub use init::{clear_git, init_git};
pub use commands::cat_file;
pub use commands::hash_objects;
pub use commands::ls_tree;
pub use commands::write_tree;
pub use commands::add_to_index;
pub use commands::index_read;
pub use commands::rm;
pub use cli::{Args, Commands};
