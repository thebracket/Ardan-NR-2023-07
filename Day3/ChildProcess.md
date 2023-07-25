# Child Processes

> See the `child_process` code in the `code` folder.

When you `spawn` a process with `std::process::Command`, it returns  a `Child`. This is cross-platform, so the PID (or equivalent) is obtained and wrapped by the `Child` type.

Spawn automatically detaches the process, and doesn't capture `stdout` (or feed `stdin`). The `Child` implements `Send`, so you can send it to another thread if you wish---but the spawning thread will keep running.

The following example assumes that you have `ping` in your path. If you're not running Windows, remove the `-t` (which tells Windows to keep pinging):

```rust
use std::{process, thread, time::Duration};

fn main() {
    let mut child = process::Command::new("ping")
        .arg("8.8.8.8")
        .arg("-t")
        .spawn()
        .expect("Couldn't run 'ping'");

    // Let ping run for 5 seconds, and then terminate it
    thread::sleep(Duration::from_secs(5));
    child.kill().expect("kill didn't work");
}
```

## Receiving output

> See the `child_process_wait_or_kill` code in the `code` folder.

If you need to run a process, and then either wait for it to complete (and read its output) or kill it, you can use the following:

```rust
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
```