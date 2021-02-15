use ferris_says::say;
use std::io::stdout;

fn main() {
    let mut stdout = stdout();
    let message = "Hello fellow Rustaceans!".as_bytes();
    say(message, message.len(), &mut stdout).unwrap();
}
