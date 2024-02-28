use byteorder::{BigEndian, ReadBytesExt};
use libc::c_long;
use libc::c_void;
use nix::{sys::wait::waitpid, unistd::Pid};
use std::cell::Cell;
use std::mem::size_of;
use std::mem::transmute;

use super::{
    common_abstract::{MemoryAddress, ThreadId},
    os_debugger_abstract::MemoryAccessor,
};
struct LinuxDebugger {}

impl LinuxDebugger {}

fn read_bytes_core(
    thread_id: ThreadId,
    address: MemoryAddress,
) -> anyhow::Result<[u8; size_of::<c_long>()]> {
    let read_result = nix::sys::ptrace::read(
        Pid::from_raw(thread_id.as_raw() as i32),
        address.as_raw() as nix::sys::ptrace::AddressType,
    )?;

    // TODO: should we use byteorder libaray?
    let bytes: [u8; size_of::<c_long>()] = unsafe { transmute(read_result.to_ne_bytes()) };

    anyhow::Result::Ok(bytes)
}

fn write_bytes_core(
    thread_id: ThreadId,
    address: MemoryAddress,
    data: [u8; size_of::<c_long>()],
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

impl MemoryAccessor for LinuxDebugger {
    fn read_memory_of_thread<const N: usize>(
        thread_id: ThreadId,
        address: MemoryAddress,
    ) -> anyhow::Result<[u8; N]> {
        let mut res: [u8; N] = [0; N];

        let size = res.len();

        let bytes_once_read = size_of::<c_long>();

        let mut aligned_index = 0;

        while aligned_index + bytes_once_read < size {
            let bytes: [u8; size_of::<c_long>()] = read_bytes_core(thread_id, address)?;

            // TODO: memory copy might be faster
            for byte in bytes {
                res[aligned_index] = byte;
                aligned_index += 1;
            }
        }

        let mut remain_index = 0;
        let remain_bytes = read_bytes_core(thread_id, address)?;

        while aligned_index + remain_index < size {
            res[aligned_index + remain_index] = remain_bytes[remain_index];
            remain_index += 1;
        }

        return anyhow::Result::Ok(res);
    }

    fn write_memory_of_thread<const N: usize>(
        thread_id: ThreadId,
        address: MemoryAddress,
        data: [u8; N],
    ) -> anyhow::Result<()> {
        let size = data.len();

        let bytes_once_write = size_of::<c_long>();

        let mut aligned_index = 0;

        while aligned_index + bytes_once_write < size {
            let mut bytes: [u8; size_of::<c_long>()] = [0; size_of::<c_long>()];

            bytes.iter_mut().for_each(|byte| {
                *byte = data[aligned_index];
                aligned_index += 1;
            });

            write_bytes_core(thread_id, address, bytes)?;
        }

        let mut remain_index = 0;
        let mut remain_bytes = read_bytes_core(thread_id, address.with_offset(aligned_index))?;

        while aligned_index + remain_index < size {
            remain_bytes[remain_index] = data[aligned_index + remain_index];
            remain_index += 1;
        }

        write_bytes_core(thread_id, address, remain_bytes)?;

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
