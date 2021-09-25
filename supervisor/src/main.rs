use async_std::process::Command;

#[async_std::main]
async fn main() {
    let out = Command::new("python")
        .arg("-m")
        .arg("http.server")
        .output()
        .await;
    println!("{:?}", out);
}

/*
use std::process::Command;

fn main() {
    let out = Command::new("python").arg("-m").arg("http.server").output();
    println!("{:?}", out);
}
*/
