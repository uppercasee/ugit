use clap::{Parser, Subcommand};
use ugit::{cat_file, hash_objects, clear_git, init_git, ls_tree, write_tree};

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Clear,
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    },
    HashObject {
        objectfile: String,
    },
    LsTree {
        object_hash: String,
    },
    WriteTree {
        treeish: String,
    },
    // Commit {
    //     message: String,
    // },
    // Clone {
    //     url: String,
    // },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Init) => {
            init_git().unwrap();
        }
        Some(Commands::Clear) => {
            clear_git().unwrap();
        }
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => {
            cat_file(pretty_print, object_hash).unwrap();
        }
        Some(Commands::HashObject { objectfile }) => {
            hash_objects(objectfile).unwrap();
        }
        Some(Commands::LsTree { object_hash }) => {
            ls_tree(object_hash).unwrap();
        }
        Some(Commands::WriteTree { treeish }) => {
            write_tree(treeish).unwrap();
        }
        None => {
            println!("No commands provided");
        }
    }
    Ok(())
}
