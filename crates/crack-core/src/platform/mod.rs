#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "windows")]
mod windows;

pub mod common_abstract {

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct MemoryAddress(usize);

    impl MemoryAddress {
        pub const fn as_raw(self) -> usize {
            self.0
        }

        pub fn with_offset(&self, offset: usize) -> Self {
            MemoryAddress(self.0 + offset)
        }
    }

    /// `32` is a proper size for ProcessId, since
    /// - the default maximum of PID on Windows is 32 bits
    /// - the default maximum of PID on Linux is 22 bits
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ProcessId(u32);

    /// `32` is a proper size for ThreadId, since
    /// - the default maximum of PID on Windows is 32 bits
    /// - the default maximum of PID on Linux is 22 bits
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ThreadId(u32);

    impl ThreadId {
        pub const fn as_raw(self) -> u32 {
            self.0
        }
    }
}

pub mod os_debugger_abstract {
    use anyhow::Result;

    use super::common_abstract::{MemoryAddress, ThreadId};

    pub trait MemoryAccessor {
        fn read_memory_of_thread<const N: usize>(
            thread_id: ThreadId,
            address: MemoryAddress,
        ) -> Result<[u8; N]>;

        fn write_memory_of_thread<const N: usize>(
            thread_id: ThreadId,
            address: MemoryAddress,
            data: [u8; N],
        ) -> Result<()>;
    }
}
