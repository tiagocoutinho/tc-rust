
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::from_args();
    let content = std::fs::read_to_string(&cli.path)?;
    for line in content.lines() {
        if line.contains(&cli.pattern) {
            print!("{}", line);
        }
    }
    Ok(())
}
