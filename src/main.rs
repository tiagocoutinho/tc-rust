use log::info;
use structopt::StructOpt;
use anyhow::{Context, Result};


#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

//
// RUST_LOG=tc_grrs=info cargo run --bin tc-grrs -- struct ./Cargo.toml

fn main() -> Result<()> {
    env_logger::init();
    info!("parsing command line...");
    let cli = Cli::from_args();
    info!("parsed command line");
    info!("loading {:?}...", cli.path);
    let content = std::fs::read_to_string(&cli.path).
        with_context(|| format!("could not read file `{:?}`", cli.path))?;
    info!("loaded {:?}", cli.path);
    info!("searching for '{}'...", cli.pattern);
    for line in content.lines() {
        if line.contains(&cli.pattern) {
            print!("{}", line);
        }
    }
    Ok(())
}
