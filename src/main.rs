
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let content = std::fs::read_to_string(&cli.path).
        with_context(|| format!("could not read file `{:?}`", cli.path))?;
    for line in content.lines() {
        if line.contains(&cli.pattern) {
            print!("{}", line);
        }
    }
    Ok(())
}
