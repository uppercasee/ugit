mod catfile;
mod hashobject;
mod init;

use clap::{Parser, Subcommand};

use catfile::cat_file;
use hashobject::hash_objects;
use init::{clear_git, init_git};

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
        None => {
            println!("No commands provided");
        }
    }
    Ok(())
}
