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
    // support for both blob and tree objects
    if header.starts_with("blob") {
        let mut content = Vec::new();
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;
        if pretty_print {
            let content = String::from_utf8(content).context("content isn't valid UTF-8")?;
            println!("{}", content);
        } else {
            std::io::stdout().write_all(&content).context("write content to stdout")?;
        }
    } else if header.starts_with("tree") {
        let mut content = Vec::new();
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;
        let content = String::from_utf8(content).context("content isn't valid UTF-8")?;
        let mut entries = Vec::new();
        for line in content.lines() {
            let mut parts = line.split(' ');
            let mode = parts.next().expect("mode is the first part");
            let path = parts.next().expect("path is the second part");
            let hash = parts.next().expect("hash is the third part");
            entries.push((mode, path, hash));
        }
        entries.sort_by(|a, b| a.1.cmp(b.1));
        for (mode, path, hash) in entries {
            println!("{} {} {}", mode, hash, path);
        }
    } else if header.starts_with("commit") {
        let mut content = Vec::new();
        z.read_to_end(&mut content)
            .context("read content from ugit/objects")?;
        let content = String::from_utf8(content).context("content isn't valid UTF-8")?;
        println!("{}", content);
    }
    else {
        panic!("unknown object type: {}", header);
    }
    Ok(())
}
