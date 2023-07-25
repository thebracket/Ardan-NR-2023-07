use std::io::Read;
use std::process::*;
use std::thread;
use std::time::Duration;

fn wait_on_output(mut out: ChildStdout) {
    //while out.read_exact(&mut [0; 1024]).is_ok() {}
    let mut buf = [0; 1024];
    while let Ok(n) = out.read(&mut buf) {
        if n == 0 {
            break;
        }
        println!("Read {n} bytes");
        println!("{:?}", String::from_utf8(buf[..n].to_vec()));
    }
}

fn wait_or_kill(cmd: &mut Command, max: Duration) {
    let mut child = cmd.stdout(Stdio::piped())
                       .spawn()
                       .expect("Cannot spawn child");

    let out = child.stdout.take().expect("No stdout on child");

    let h = thread::spawn(move || {
        thread::sleep(max);
        println!("Killing child process");
        child.kill().expect("Cannot kill child process");
        println!("{:?}", child.wait());
    });

    wait_on_output(out);
    h.join().expect("join fail");
}

fn main() {
    wait_or_kill(Command::new("ping").args(["8.8.8.8", "-t"]), Duration::new(2, 0));
}