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
    let args = Cli::from_args();
    info!("parsed command line");
    info!("loading `{}`...", args.path.display());
    let content = std::fs::read_to_string(&args.path).
        with_context(|| format!("could not read file `{}`", args.path.display()))?;
    info!("loaded `{}`", args.path.display());
    info!("searching for `{}`...", args.pattern);
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout()).
        with_context(|| format!("could not print results!"))?;
    Ok(())
}
