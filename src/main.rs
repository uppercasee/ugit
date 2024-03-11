use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Read, Write},
};
use crypto_hash::{Algorithm, hex_digest};

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

fn create_directory(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(path).map_err(|err| {
        eprintln!("Error creating directory '{}': {}", path, err);
        err
    })
}

fn main() -> anyhow::Result<()> {
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
            if let Err(err) = fs::remove_dir_all("./ugit") {
                eprintln!("Error: {}", err)
            } else {
                println!("Directory './ugit' removed successfully");
            }
        }
        Some(Commands::CatFile {
            pretty_print,
            object_hash,
        }) => {
            // TODO: shortest hash match
            let hash = &object_hash[..2];
            let file = &object_hash[2..];
            let path = format!("./ugit/objects/{}/{}", hash, file);
            let f = fs::File::open(path).context("couldn't open ugit/objects file")?;
            let z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            let mut buf = Vec::new();
            z.read_until(0, &mut buf)
                .context("read header from ugit/objects")?;
            let header = CStr::from_bytes_with_nul(&buf).expect("there is one null at the end.");
            let header = header
                .to_str()
                .context("ugit/objects file header isn't valid UTF-8")?;
            let Some(size) = header.strip_prefix("blob ") else {
                anyhow::bail!("ugit/object header didn't start with 'blob ': '{header}'")
            };
            let size: usize = size.parse().context("couldn't parse size")?;
            let mut contents = Vec::with_capacity(size);
            z.read_to_end(&mut contents)
                .context("read contents from ugit/objects")?;
            if pretty_print {
                print!("{}", String::from_utf8_lossy(&contents));
            } else {
                std::io::stdout()
                    .write_all(&contents)
                    .context("write contents to stdout")?;
            }
        }
        Some(Commands::HashObject { objectfile }) => {
            let contents = fs::read(objectfile).context("couldn't read object file")?;
            let header = format!("blob {}\0", contents.len());
            let data = [header.as_bytes(), &contents].concat();
            let hash = hex_digest(Algorithm::SHA1, &data);
            println!("{}", hash);
            // Generate file path based on hash
            let object_path = format!("./ugit/objects/{}", &hash[..2]);
            fs::create_dir_all(&object_path)
                .with_context(|| format!("couldn't create object directory: {}", object_path))?;

            let object_file = format!("{}/{}", object_path, &hash[2..]);
            let file = fs::File::create(&object_file)
                .with_context(|| format!("couldn't create object file: {}", object_file))?;
            let mut z = flate2::write::ZlibEncoder::new(file, flate2::Compression::default());
            z.write_all(&data)
                .with_context(|| format!("write data to object file: {}", object_file))?;
            z.finish()
                .with_context(|| format!("finishing writing to object file: {}", object_file))?;

            // println!("Object written to: {}", object_file);
        }
        None => {
            println!("No commands provided");
        }
    }
    Ok(())
}
