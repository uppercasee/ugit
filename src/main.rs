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

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Commands::Init) => {
            if let Err(err) = fs::create_dir_all("./ugit/objects/info"){
                eprintln!("Error: {}", err)
            }else if let Err(err) = fs::create_dir_all("./ugit/objects/pack") {
                eprintln!("Error: {}", err)
            }else if let Err(err) = fs::create_dir_all("./ugit/refs/heads") {
                eprintln!("Error: {}", err)
            }else if let Err(err) = fs::create_dir_all("./ugit/refs/tags") {
                eprintln!("Error: {}", err)
            }else if let Err(err) = fs::write("./ugit/HEAD", "ref: refs/heads/master\n") {
                eprintln!("Error: {}", err);
            }else{
                println!("Initialized git: Created directory structure in './ugit'");}
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
