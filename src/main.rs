use std::path::Path;

use clap::Parser;
use ugit::{add_to_index, clear_git, index_read, init_git, rm};
use ugit::{cat_file, hash_objects, ls_tree, write_tree};
use ugit::{Args, Commands};

fn is_git_repo() -> bool {
    Path::new("./ugit").exists()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !is_git_repo() && !matches!(args.command, Some(Commands::Init)) {
        println!("Not a Git repository (or any of the parent directories): /ugit");
        std::process::exit(1);
    }

    match args.command {
        Some(Commands::Init) => {
            init_git()?;
        }
        Some(Commands::Clear) => {
            clear_git()?;
        }
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => {
            cat_file(pretty_print, object_hash)?;
        }
        Some(Commands::HashObject { objectfile }) => {
            let hash_vec = hash_objects(&objectfile)?;
            let hash = hex::encode(hash_vec);
            println!("{}", hash);
        }
        Some(Commands::LsTree { object_hash }) => {
            ls_tree(object_hash)?;
        }
        Some(Commands::WriteTree { tree }) => {
            let hash_vec = write_tree(tree)?;
            let hash = hex::encode(hash_vec);
            println!("{}", hash);
        }
        Some(Commands::Add { objectfile }) => {
            add_to_index(objectfile)?;
        }
        Some(Commands::LsFiles {}) => {
            index_read()?;
        }
        Some(Commands::Rm {
            cached,
            objectfile,
        }) => {
            rm(cached, objectfile)?;
        }
        None => {
            println!("No commands provided");
        }
    }
    Ok(())
}
