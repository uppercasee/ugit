use anyhow::{Context, Result};
use crypto_hash::{hex_digest, Algorithm};
use std::{fs, io::Write};

pub fn hash_objects(objectfile: &str) -> Result<[u8; 20]> {
    let contents = fs::read(objectfile).context("couldn't read object file")?;
    let header = format!("blob {}\0", contents.len());

    let data = [header.as_bytes(), &contents].concat();

    let hash = hex_digest(Algorithm::SHA1, &data);

    let mut hash_bytes = [0u8; 20];
    hex::decode_to_slice(&hash, hash_bytes.as_mut())
        .with_context(|| format!("couldn't decode hash: {}", hash))?;

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

    Ok(hash_bytes)
}
