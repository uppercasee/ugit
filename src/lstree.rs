use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Read, Write},
};

pub fn ls_tree (object_hash: String) -> anyhow::Result<()> {
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
    if header.starts_with("tree"){
        let Some(size) = header.strip_prefix("tree ") else {
            anyhow::bail!("ugit/object header didn't start with 'tree ': '{header}'")
        };
        let size: usize = size.parse().context("couldn't parse size")?;
        // println!("size: {}", size);
        let mut content = Vec::with_capacity(size);
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;
        std::io::stdout().write_all(&content).context("write content to stdout")?;
    }
        Ok(())
}