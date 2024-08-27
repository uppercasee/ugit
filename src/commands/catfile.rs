use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Read, Write},
};

use super::get_full_path_from_hash;

pub fn cat_file(pretty_print: bool, object_hash: String) -> anyhow::Result<()> {
    let path = get_full_path_from_hash(&object_hash)?;
    let mut parts = path.split('/');

    let hash = parts.next().expect("hash is missing");
    let file = parts.next().expect("file is missing");

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
    // support for both blob and tree objects
    if header.starts_with("blob") {
        let mut content = Vec::new();
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;
        if pretty_print {
            let content = String::from_utf8(content).context("content isn't valid UTF-8")?;
            println!("{}", content);
        } else {
            std::io::stdout()
                .write_all(&content)
                .context("write content to stdout")?;
        }
    } else if header.starts_with("tree") {
        let Some(size) = header.strip_prefix("tree ") else {
            anyhow::bail!("ugit/object header didn't start with 'tree ': '{header}'")
        };
        let size: usize = size.parse().context("couldn't parse size")?;
        // println!("size: {}", size);
        let mut content = Vec::with_capacity(size);
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;

        if pretty_print {
            let content = String::from_utf8(content).context("content isn't valid UTF-8")?;
            println!("{}", content);
        } else {
            std::io::stdout()
                .write_all(&content)
                .context("write content to stdout")?;
        }
    } else if header.starts_with("commit") {
        let Some(size) = header.strip_prefix("commit ") else {
            anyhow::bail!("ugit/object header didn't start with 'commit ': '{header}'")
        };
        let size: usize = size.parse().context("couldn't parse size")?;
        // println!("size: {}", size);
        let mut content = Vec::with_capacity(size);
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;

        if pretty_print {
            let content = String::from_utf8(content).context("content isn't valid UTF-8")?;
            println!("{}", content);
        } else {
            std::io::stdout()
                .write_all(&content)
                .context("write content to stdout")?;
        }
    } else {
        panic!("unknown object type: {}", header);
    }
    Ok(())
}
