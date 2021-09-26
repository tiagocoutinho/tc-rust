use async_std::{
    io::{stdout, BufReader},
    prelude::*,
    process::{Child, Command, Stdio},
};
use std::error::Error;

fn cmd(exec: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(exec);
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    cmd.kill_on_drop(true);
    cmd
}

async fn wait(proc: &mut Child) -> Result<(), Box<dyn Error>> {
    let status = proc.status().await?;
    match status.code() {
        None => println!("> Process terminated by signal: {:?}", status),
        Some(code) => println!("> Exited with: {}", code),
    }
    Ok(())
}

pub async fn non_buffered_output(
    exec: &str,
    args: &[&str],
    kill: bool,
) -> Result<(), Box<dyn Error>> {
    let mut proc = cmd(exec, args).spawn()?;
    eprintln!("> Run {} {:?} pid={} kill={}", exec, args, proc.id(), kill);
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
    if kill {
        proc.kill()?;
    }
    wait(&mut proc).await?;
    Ok(())
}

pub async fn line_output(exec: &str, args: &[&str], kill: bool) -> Result<(), Box<dyn Error>> {
    let mut proc = cmd(exec, args).spawn()?;
    eprintln!("> Run {} {:?} pid={} kill={}", exec, args, proc.id(), kill);
    let child_out = proc.stdout.take().ok_or("no stdout?")?;
    let reader = BufReader::new(child_out);
    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        println!("{}", line?);
    }
    if kill {
        proc.kill()?;
    }
    wait(&mut proc).await?;
    Ok(())
}
