use async_std::{
    io::{stdout, BufReader},
    prelude::*,
    process::{Command, Stdio},
};
use std::error::Error;

fn cmd(sleep: u8) -> Command {
    let mut cmd = Command::new("./go.sh");
    cmd.arg(sleep.to_string()).stdout(Stdio::piped());
    cmd
}

async fn non_buffered_output(sleep: u8) -> Result<(), Box<dyn Error>> {
    let mut proc = cmd(sleep).spawn()?;
    let mut child_out = proc.stdout.take().ok_or("no stdout?")?;
    let mut buf = [0_u8; 128];
    let mut out = stdout();
    loop {
        match child_out.read(&mut buf).await {
            Ok(0) => {
                println!("---EOF---");
                break;
            }
            Ok(n) => {
                out.write_all(&buf[..n])
                    .await
                    .expect("could not write to stdout!");
                out.flush().await.expect("failed to flush!");
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    match proc.try_status()? {
        None => println!("still running"),
        Some(status) => println!("exited with: {}", status),
    }
    Ok(())
}

async fn line_output(sleep: u8) -> Result<(), Box<dyn Error>> {
    let proc = cmd(sleep).spawn()?;
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
    match non_buffered_output(1).await {
        Ok(_) => println!("---END---"),
        Err(e) => println!("Error: {}", e),
    }
    match line_output(1).await {
        Ok(_) => println!("---END---"),
        Err(e) => println!("Error: {}", e),
    }
}
