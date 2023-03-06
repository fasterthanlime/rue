use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};

use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::Pid,
};
use owo_colors::OwoColorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json: serde_json::Value = serde_json::from_str(include_str!("syscall.json"))?;
    let syscall_table: HashMap<u64, String> = json["aaData"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            (
                item[0].as_u64().unwrap(),
                item[1].as_str().unwrap().to_owned(),
            )
        })
        .collect();

    let mut command = Command::new("cat");
    command.arg("/etc/hosts");
    unsafe {
        command.pre_exec(|| {
            use nix::sys::ptrace::traceme;
            traceme().map_err(|e| e.into())
        });
    }

    let child = command.spawn()?;
    let child_pid = Pid::from_raw(child.id() as _);
    let res = waitpid(child_pid, None)?;
    eprintln!("first wait: {:?}", res.yellow());

    let mut is_sys_exit = false;
    loop {
        ptrace::syscall(child_pid, None)?;
        _ = waitpid(child_pid, None)?;
        if is_sys_exit {
            let regs = ptrace::getregs(child_pid)?;
            eprintln!(
                "{}({:x}, {:x}, {:x}, ...) = {:x}",
                syscall_table[&regs.orig_rax].green(),
                regs.rdi.blue(),
                regs.rsi.blue(),
                regs.rdx.blue(),
                regs.rax.yellow(),
            );
        }
        is_sys_exit = !is_sys_exit;
    }
}
