
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::from_args();
    let content = match std::fs::read_to_string(&cli.path) {
        Ok(content) => { content },
        Err(_error) => { panic!("cannot read file"); }
    };
    for line in content.lines() {
        if line.contains(&cli.pattern) {
            print!("{}", line);
        }
    }
}
