use async_std::{
    io::{stdout, BufReader},
    prelude::*,
    process::{Child, Command, Stdio},
};
use std::error::Error;

fn cmd(exec: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(exec);
    cmd.args(args).stdout(Stdio::piped());
    cmd
}

async fn wait(proc: &mut Child) -> Result<(), Box<dyn Error>> {
    let status = proc.status().await?;
    match status.code() {
        None => println!("Process terminated by signal"),
        Some(code) => println!("exited with: {}", code),
    }
    Ok(())
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
    wait(&mut proc).await?;
    Ok(())
}

async fn line_output(exec: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut proc = cmd(exec, args).spawn()?;
    let child_out = proc.stdout.take().ok_or("no stdout?")?;
    let reader = BufReader::new(child_out);
    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        println!("{}", line.expect("Expected line"));
    }
    wait(&mut proc).await?;
    Ok(())
}

#[async_std::main]
async fn main() {
    if let Err(error) = non_buffered_output("./go.sh", &["1", "0"]).await {
        eprintln!("> Error: {}", error);
    }
    if let Err(error) = line_output("./go.sh", &["1.1", "5"]).await {
        eprintln!("> Error: {}", error);
    }
}
