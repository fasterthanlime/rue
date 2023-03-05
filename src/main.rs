use std::{os::unix::process::CommandExt, process::Command};

use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::Pid,
};
use owo_colors::OwoColorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    loop {
        ptrace::syscall(child_pid, None)?;
        _ = waitpid(child_pid, None)?;
        let regs = ptrace::getregs(child_pid)?;
        eprintln!(
            "syscall {} called with {:x}, {:x}, {:x}, returned {:x}",
            regs.orig_rax.green(),
            regs.rdi.blue(),
            regs.rsi.blue(),
            regs.rdx.blue(),
            regs.rax.yellow(),
        );
    }
}
