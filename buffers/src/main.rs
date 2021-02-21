use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter, Read, Write, Result};

struct Meadowlark {
    path: PathBuf,
    source: Option<File>
}

impl Meadowlark {
    fn new(path: &str) -> Self {
        Self { path: PathBuf::from(path), source: None }
    }

    fn open(&mut self) -> Result<()> {
        self.source = Some(File::open(&self.path)?);
        Ok(())
    }
}

impl Drop for Meadowlark {
    fn drop(&mut self) {
        // eventualy release unsafe resources
        self.source = None;
    }
}

impl Write for &Meadowlark {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        std::io::stderr().write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        std::io::stderr().flush()?;
        Ok(())
    }
}

impl Read for &Meadowlark {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.source.as_ref().unwrap().read(buf)
    }
}

fn main() -> Result<()> {
    let mut meadowlark = Meadowlark::new("poem.txt");
    meadowlark.open()?;
    let mut reader = BufReader::new(&meadowlark);
    let mut writer = LineWriter::new(&meadowlark);
    writer.write(b"Hello, World!\nHello 2\nI won't show up until the end")?;
    println!("I am doing some work!");
    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("I am dying now. Bye, bye!");
    println!("First line is {} bytes long", len);
    Ok(())
}
