use supervisor::output;

#[async_std::main]
async fn main() {
    if let Err(error) = output::non_buffered_output("./go.sh", &["1", "0"], false).await {
        eprintln!("> Error: {}", error);
    }
    if let Err(error) = output::non_buffered_output("./go.sh", &[".1", "3"], true).await {
        eprintln!("> Error: {}", error);
    }
    if let Err(error) = output::line_output("./go.sh", &["1.1", "5"], false).await {
        eprintln!("> Error: {}", error);
    }
}
