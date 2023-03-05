use std::process::Command;

fn main() {
    let mut command = Command::new("cat");
    command.arg("/etc/hosts");

    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}
