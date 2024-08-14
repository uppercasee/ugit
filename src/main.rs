use clap::Parser;
use ugit::{clear_git, init_git};
use ugit::{cat_file, hash_objects, ls_tree, write_tree};
use ugit::{Args, Commands};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Init) => {
            init_git()?;
        }
        Some(Commands::Clear) => {
            clear_git()?;
        }
        Some(Commands::CatFile { pretty_print, object_hash, }) => {
            cat_file(pretty_print, object_hash)?;
        }
        Some(Commands::HashObject { objectfile }) => {
            let hash = hash_objects(objectfile)?;
            println!("{}", hash);
        }
        Some(Commands::LsTree { object_hash }) => {
            ls_tree(object_hash)?;
        }
        Some(Commands::WriteTree { tree }) => {
            let hash = write_tree(tree)?;
            println!("{}", hash);
        }
        None => {
            println!("No commands provided");
        }
    }
    Ok(())
}
