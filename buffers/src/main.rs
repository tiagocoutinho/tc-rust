use std::fs::File;
use std::path::PathBuf;
use std::io::{BufReader, LineWriter, Read, Write, Result};

struct Meadowlark {
    path: PathBuf,
    reader: Option<BufReader<File>>,
    writer: Option<LineWriter<std::io::Stdout>>,
}

impl Meadowlark {
    fn new(path: &str) -> Self {
        Self { path: PathBuf::from(path), reader: None, writer: None }
    }

    fn open(&mut self) -> Result<()> {
        let source = File::open(&self.path)?;
        self.reader = Some(BufReader::new(source));
        self.writer = Some(LineWriter::new(std::io::stdout()));
        Ok(())
    }

    fn close(&mut self) {
        // `source` goes out of scope, and the "hello.txt" file gets closed
        self.reader = None;
        self.writer = None;
    }
}

impl Write for Meadowlark {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.as_mut().unwrap().write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.as_mut().unwrap().flush()?;
        Ok(())
    }
}

impl Read for Meadowlark {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {

    }
}

fn main() -> Result<()> {
    let mut meadowlark = Meadowlark::new("poem.txt");
    meadowlark.open()?;
    meadowlark.write(b"Hello, World!\nHello 2\nI won't show up until the end")?;
    println!("I am doing some work!");
    meadowlark.flush()?;
    println!("I am dying now. Bye, bye!");
    meadowlark.close();
    Ok(())
}
