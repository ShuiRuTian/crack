use anyhow::Result;
use libc::c_long;
use libc::c_void;
use libc::process_vm_readv;
use nix::sys::ptrace;
use nix::{sys::wait::waitpid, unistd::Pid};
use std::mem::size_of;
use std::mem::transmute;

use crate::common::breakpoint::SoftwareBreakpoint;
use crate::common::general::MemoryAccessor;
use crate::common::general::ProcessId;
use crate::common::general::RegisterAccessor;
use crate::common::general::{MemoryAddress, ThreadId};

struct NativeProcessLinux {
    pub process_id: ProcessId,
}

impl NativeProcessLinux {}

const PTRACE_WORD_SIZE: usize = size_of::<c_long>();

fn read_bytes_core(
    thread_id: ThreadId,
    address: MemoryAddress,
) -> anyhow::Result<[u8; PTRACE_WORD_SIZE]> {
    let read_result = nix::sys::ptrace::read(
        Pid::from_raw(thread_id.as_raw() as i32),
        address.as_raw() as nix::sys::ptrace::AddressType,
    )?;

    // TODO: should we use byteorder libaray?
    let bytes: [u8; PTRACE_WORD_SIZE] = unsafe { transmute(read_result.to_ne_bytes()) };

    anyhow::Result::Ok(bytes)
}

fn write_bytes_core(
    thread_id: ThreadId,
    address: MemoryAddress,
    data: [u8; PTRACE_WORD_SIZE],
) -> anyhow::Result<()> {
    unsafe {
        nix::sys::ptrace::write(
            Pid::from_raw(thread_id.as_raw() as i32),
            address.as_raw() as nix::sys::ptrace::AddressType,
            c_long::from_ne_bytes(data) as *mut c_void,
        )
    }?;

    anyhow::Result::Ok(())
}

impl RegisterAccessor for NativeProcessLinux {
    fn get_register_value(&self, register: gimli::Register) -> anyhow::Result<libc::c_ulonglong> {
        let register = ptrace::getregs(Pid::from_raw(0))?;
        gimli::X86_64::RAX
        register.

        Result::Ok(1)
    }

    fn set_register_value(
        &self,
        register: gimli::Register,
        value: libc::c_ulonglong,
    ) -> Result<()> {
        Result::Ok(())
    }
}

/// Safety: On Linux, the main thread's Id is the same with Process Id
fn from_process_id_to_thread_id(pid: ProcessId) -> ThreadId {
    ThreadId::from_raw(pid.as_raw())
}

impl MemoryAccessor for NativeProcessLinux {
    fn read_memory<const N: usize>(&self, address: MemoryAddress) -> anyhow::Result<[u8; N]> {
        // TODO: impl process_vm_readv, LLDB says it's 50x faster than ptrace
        // process_vm_readv();

        let mut res: [u8; N] = [0; N];

        let mut remainder = res.len();

        while remainder > 0 {
            let bytes = read_bytes_core(from_process_id_to_thread_id(self.process_id), address)?;

            let move_size = remainder.min(PTRACE_WORD_SIZE);

            // Safety:
            // Edge checked. It's able to use safe code, but memcopy is a bit logic cleaner
            unsafe { crate::utils::memcopy(bytes.as_ptr(), res.as_mut_ptr(), move_size) };

            remainder = remainder - PTRACE_WORD_SIZE;
        }

        return anyhow::Result::Ok(res);
    }

    fn write_memory<const N: usize>(
        &self,
        address: MemoryAddress,
        data: [u8; N],
    ) -> anyhow::Result<()> {
        let size = data.len();

        let mut aligned_index = 0;

        while aligned_index + PTRACE_WORD_SIZE < size {
            let bytes: [u8; PTRACE_WORD_SIZE] = data
                [aligned_index..aligned_index + PTRACE_WORD_SIZE]
                .try_into()
                .expect("Safe, the size is checked");

            write_bytes_core(
                from_process_id_to_thread_id(self.process_id),
                address,
                bytes,
            )?;

            aligned_index = aligned_index + PTRACE_WORD_SIZE;
        }

        let mut remain_bytes = read_bytes_core(
            from_process_id_to_thread_id(self.process_id),
            address.with_offset(aligned_index),
        )?;

        // Safety:
        // Edge checked. It's able to use safe code, but memcopy is a bit logic cleaner
        unsafe {
            crate::utils::memcopy(
                data.as_ptr(),
                remain_bytes.as_mut_ptr(),
                size - aligned_index,
            )
        };

        write_bytes_core(
            from_process_id_to_thread_id(self.process_id),
            address,
            remain_bytes,
        )?;

        return anyhow::Result::Ok(());
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use libc::{execl, fork, WIFSTOPPED};
    use nix::{
        sys::wait::{wait, waitpid},
        unistd::Pid,
    };

    #[test]
    fn test_1() -> anyhow::Result<()> {
        let prog = CString::new("/bin/ls").expect("msg");

        let pid = unsafe { fork() };

        if (pid == 0) {
            //we're in the child process
            //execute debugee
            println!("In debugee PID {:?}", pid);

            let resgister = nix::sys::ptrace::getregs(Pid::from_raw(pid))?;

            let res = nix::sys::ptrace::traceme();
            let mut array: [i32; 3] = [0; 3];
            print!("{:?}", res);

            unsafe {
                execl(prog.as_ptr(), prog.as_ptr(), 0);
            }
        } else if (pid >= 1) {
            //we're in the parent process
            //execute debugger
            println!("In debuger PID {:?}", pid);
            use libc::c_void;

            nix::sys::ptrace::read(Pid::from_raw(pid), 0x00112233 as *mut c_void);

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
