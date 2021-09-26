use async_std::{
    io::{stdout, BufReader},
    prelude::*,
    process::{Command, Stdio},
};
use std::error::Error;

fn cmd(exec: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(exec);
    cmd.args(args).stdout(Stdio::piped());
    cmd
}

async fn non_buffered_output(exec: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut proc = cmd(exec, args).spawn()?;
    let mut child_out = proc.stdout.take().ok_or("no stdout?")?;
    let mut buf = [0_u8; 128];
    let mut out = stdout();
    loop {
        match child_out.read(&mut buf).await? {
            0 => break,
            n => {
                out.write_all(&buf[..n]).await?;
                out.flush().await?;
            }
        }
    }

    match proc.try_status()? {
        None => println!("still running"),
        Some(status) => println!("exited with: {}", status),
    }
    Ok(())
}

async fn line_output(exec: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let proc = cmd(exec, args).spawn()?;
    // we partially move but we don't care about proc anymore
    let child_out = proc.stdout.ok_or("no stdout?")?;
    let reader = BufReader::new(child_out);
    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        println!("{}", line.expect("Expected line"));
    }
    Ok(())
}

#[async_std::main]
async fn main() {
    const EXEC: &str = "./go.sh";
    const DELAY: &str = "1";
    const ARGS: &[&str] = &[DELAY];

    match non_buffered_output(EXEC, ARGS).await {
        Ok(_) => println!("---END---"),
        Err(e) => println!("Error: {}", e),
    }
    match line_output(EXEC, ARGS).await {
        Ok(_) => println!("---END---"),
        Err(e) => println!("Error: {}", e),
    }
}
