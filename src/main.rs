// use std::path::PathBuf;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Clear,
    Init, 
}

fn create_directory(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(path).map_err(|err| {
        eprintln!("Error creating directory '{}': {}", path, err);
        err
    })
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Commands::Init) => {
            create_directory("./ugit/objects/info").unwrap();
            create_directory("./ugit/objects/pack").unwrap();
            create_directory("./ugit/refs/heads").unwrap();
            create_directory("./ugit/refs/tags").unwrap();
            fs::write("./ugit/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git: Created directory structure in './ugit'");
        }
        Some(Commands::Clear) => {
            if let Err(err) = fs::remove_dir_all("./ugit"){
                eprintln!("Error: {}", err)
            } else {
                println!("Directory './ugit' removed successfully");
            }
        }
        None => {
            println!("No commands provided");
        }
    }
}
