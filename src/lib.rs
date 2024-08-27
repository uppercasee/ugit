mod cli;
mod commands;
mod init;
mod utils;

pub use cli::{Args, Commands};
pub use commands::add_to_index;
pub use commands::cat_file;
pub use commands::hash_objects;
pub use commands::index_read;
pub use commands::ls_tree;
pub use commands::rm;
pub use commands::write_tree;
pub use init::{clear_git, init_git};
pub use utils::{find_index, get_full_path_from_hash};
