use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut echo = Command::new("../target/debug/echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let echo_stdin = echo.stdin.as_mut().unwrap();

    echo_stdin.write_all(b"Hello, world!\n").unwrap();
    let output = echo.wait_with_output().unwrap();
    println!("output = {:?}", output);
}