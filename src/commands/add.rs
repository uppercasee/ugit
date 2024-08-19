//use anyhow::Context;
//use crypto_hash::{hex_digest, Algorithm};

use std::{fs::File, io::Write};

use crate::hash_objects;

pub fn add_to_index(objectfile: String) -> anyhow::Result<String> {
    //let contents = fs::read(objectfile).context("couldn't read object file")?;
    //let header = format!("blob {}\0", contents.len());
    //let data = [header.as_bytes(), &contents].concat();
    //let hash = hex_digest(Algorithm::SHA1, &data);
    let hash = hash_objects(&objectfile)?;
    println!("{}", hash);

    let f_format = format!("{} {}\n", &hash, &objectfile);
    println!("{}", f_format);

    let mut ff = File::options().write(true).read(true).append(true).open("ugit/index")?;
    ff.write_all(f_format.as_bytes())?;

    // Generate file path based on hash
    //let object_path = format!("./ugit/objects/{}", &hash[..2]);
    //fs::create_dir_all(&object_path)
    //    .with_context(|| format!("couldn't create object directory: {}", object_path))?;
    //
    //let object_file = format!("{}/{}", object_path, &hash[2..]);
    //let z = flate2::write::ZlibEncoder::new(file, flate2::Compression::default());
    //let mut z = flate2::write::ZlibEncoder::new(file, flate2::Compression::default());
    //z.write_all(&data)
    //    .with_context(|| format!("write data to object file: {}", object_file))?;
    //z.finish()
    //    .with_context(|| format!("finishing writing to object file: {}", object_file))?;
    //
    // println!("Object written to: {}", object_file);
    Ok(hash)
}
