use std::process::Command;

fn main() {
    let result = Command::new("../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .output();

    if let Ok(output) = result {
        let returned_text = String::from_utf8(output.stdout).unwrap();
        println!("Process returned: {returned_text}");
    }
}
