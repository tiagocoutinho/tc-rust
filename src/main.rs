use log::info;
use structopt::StructOpt;
use anyhow::{Context, Result};


#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
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
    find_matches(&content, &args.pattern, &mut std::io::stdout()).
        with_context(|| format!("could not print results!"))?;
    Ok(())
}


#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}
