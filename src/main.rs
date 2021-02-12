
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::from_args();
    println!("pattern=\"{:}\" path={:#?}", cli.pattern, cli.path);
}
