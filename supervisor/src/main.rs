use async_std::{
    io::{stdout, BufReader},
    prelude::*,
    process::{Command, Stdio},
};

fn cmd(sleep: u8) -> Command {
    let mut cmd = Command::new("./go.sh");
    cmd.arg(sleep.to_string()).stdout(Stdio::piped());
    cmd
}

async fn non_buffered_output(sleep: u8) {
    let proc = cmd(sleep).spawn().expect("Error creating process");
    let mut child_out = proc.stdout.expect("No stdout available!");
    let mut buf = [0_u8; 128];
    let mut out = stdout();
    loop {
        match child_out.read(&mut buf).await {
            Ok(0) => {
                println!("---EOF---");
                break;
            }
            Ok(n) => {
                //print!("{}", std::str::from_utf8(&buf[..n]).expect("ups!"));

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
    println!("---END---");
}

async fn line_output(sleep: u8) {
    let proc = cmd(sleep).spawn().expect("Error creating process");
    let child_out = proc.stdout.expect("No stdout available!");
    let reader = BufReader::new(child_out);
    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        println!("{}", line.expect("Expected line"));
    }
    println!("---END---");
}

#[async_std::main]
async fn main() {
    non_buffered_output(1).await;
    line_output(1).await;
}
