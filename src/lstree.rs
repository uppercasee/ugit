use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Read, Write},
};
use std::fs::File;

pub fn ls_tree (object_hash: String) -> anyhow::Result<()> {
    let hash = &object_hash[..2];
    let file = &object_hash[2..];
    let path = format!("./ugit/objects/{}/{}", hash, file);
    let f = File::open(path).context("couldn't open ugit/objects file")?;
    let z = ZlibDecoder::new(f);
    let mut z = BufReader::new(z);
    let mut buf = Vec::new();
    z.read_until(0, &mut buf)
        .context("read header from ugit/objects")?;
    let header = CStr::from_bytes_with_nul(&buf).expect("there is one null at the end.");
    let header = header
        .to_str()
        .context("ugit/objects file header isn't valid UTF-8")?;
    let Some(size) = header.strip_prefix("tree ") else {
        anyhow::bail!("ugit/object header didn't start with 'tree ': '{header}'")
    };
    let size: usize = size.parse().context("couldn't parse size")?;
    let mut contents = Vec::with_capacity(size);
    z.read_to_end(&mut contents)
        .context("read contents from ugit/objects")?;
    let mut i = 0;
    while i < size {
        // <mode> <name>\0<20_byte_sha>
        let mode = u32::from_str_radix(std::str::from_utf8(&contents[i..i+6])?, 8)?;
        i += 6;
        let name = &contents[i..];
        let name = name.split(|&x| x == 0).collect::<Vec<&[u8]>>()[0];
        let name = String::from_utf8_lossy(name);
        i += name.len() + 1;
        let sha = &contents[i..i+20];
        let sha = hex::encode(sha);
        i += 20;
        println!("{:06o} {} {}", mode, sha, name);
    }
    Ok(())
}