use std::{os::unix::process::CommandExt, process::Command};

fn main() {
    println!("I'm dad, my PID is {}", std::process::id());

    let mut command = Command::new("cat");
    command.arg("/etc/hosts");
    unsafe {
        command.pre_exec(|| {
            println!("I'm squid, my PID is {}", std::process::id());
            Ok(())
        });
    }

    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}
