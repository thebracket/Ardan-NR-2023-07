use std::{process, thread, time::Duration};

fn main() {
    let mut child = process::Command::new("ping")
        .arg("8.8.8.8")
        .arg("-t")
        .spawn()
        .expect("Couldn't run 'ping'");

    thread::sleep(Duration::from_secs(5));
    child.kill().expect("!kill");
}