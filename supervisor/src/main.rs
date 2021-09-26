use async_std::{
    io::BufReader,
    prelude::*,
    process::{Command, Stdio},
};

fn cmd(sleep: u32) -> Command {
    let mut cmd = Command::new("./go.sh");
    cmd.arg(sleep.to_string()).stdout(Stdio::piped());
    cmd
}

#[async_std::main]
async fn main() {
    let proc = cmd(1).spawn().expect("Error creating process");
    let stdout = proc.stdout.expect("No stdout available!");
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        println!("line: {:?}", line);
    }
    println!("---END---");
}
