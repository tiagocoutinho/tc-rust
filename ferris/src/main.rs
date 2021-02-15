use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = "Hello fellow Rustaceans!".as_bytes();
    let mut writer = BufWriter::new(stdout.lock());
    say(message, message.len(), &mut writer).unwrap();
}
