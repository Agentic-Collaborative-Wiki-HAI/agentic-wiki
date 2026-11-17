use std::{path::PathBuf, time::Duration};
#[tokio::main]
async fn main() {
    let binary = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("codex"));
    let directory = tempfile::tempdir().expect("unable to create an empty probe directory");
    match agentic_wiki::codex::inspect(&binary, directory.path(), Duration::from_secs(30)).await {
        Ok(report) => println!("{}", serde_json::to_string_pretty(&report).unwrap()),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
