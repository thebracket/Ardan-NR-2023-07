use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let future = Command::new("../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .output();

    if let Ok(Ok(output)) = timeout(Duration::from_secs(1), future).await {
        let returned_text = String::from_utf8(output.stdout).unwrap();
        println!("Process returned: {returned_text}");
    }
}
