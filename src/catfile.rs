use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Read, Write},
};

pub fn cat_file(pretty_print: bool, object_hash: String) -> anyhow::Result<()> {
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
    Ok(())
}
