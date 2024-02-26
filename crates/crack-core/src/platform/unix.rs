use nix::sys::wait::waitpid;
struct LinuxDebugger {
    pid: i32,
}

impl LinuxDebugger {}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use libc::{execl, fork, WIFSTOPPED};
    use nix::sys::wait::{wait, waitpid};

    #[test]
    fn test_1() -> anyhow::Result<()> {
        let prog = CString::new("/bin/ls").expect("msg");

        let pid = unsafe { fork() };

        if (pid == 0) {
            //we're in the child process
            //execute debugee
            println!("In debugee PID {:?}", pid);
            
            let res = nix::sys::ptrace::traceme();

            print!("{:?}", res);

            unsafe {
                execl(prog.as_ptr(), prog.as_ptr(), 0);
            }
        } else if (pid >= 1) {
            //we're in the parent process
            //execute debugger
            println!("In debuger PID {:?}", pid);

            loop {
                let mut status = wait()?;

                println!("Debugger exited wait()");
                match (status) {
                    nix::sys::wait::WaitStatus::Exited(_, _) => break,
                    nix::sys::wait::WaitStatus::Signaled(pid, s, _) => {
                        println!("Child {:?} received signal {:?}", pid, status)
                    }
                    nix::sys::wait::WaitStatus::Stopped(pid, _) => {
                        println!("Child has stopped due to signal {:?}", status)
                    }
                    nix::sys::wait::WaitStatus::PtraceEvent(_, _, _) => todo!(),
                    nix::sys::wait::WaitStatus::PtraceSyscall(_) => todo!(),
                    nix::sys::wait::WaitStatus::Continued(_) => todo!(),
                    nix::sys::wait::WaitStatus::StillAlive => todo!(),
                }
                println!("Debugger exited");
            }
        }
        return Result::Ok(());
    }
}
