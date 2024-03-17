use anyhow::Result;

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

impl ProcessId {
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}

/// `32` is a proper size for ThreadId, since
/// - the default maximum of PID on Windows is 32 bits
/// - the default maximum of PID on Linux is 22 bits
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ThreadId(u32);

impl ThreadId {
    pub fn from_raw(v: u32) -> Self {
        Self(v)
    }

    pub fn as_raw(self) -> u32 {
        self.0
    }
}

pub trait MemoryAccessor {
    fn read_memory<const N: usize>(&self, address: MemoryAddress) -> Result<[u8; N]>;

    fn write_memory<const N: usize>(
        &self,
        address: MemoryAddress,
        data: [u8; N],
    ) -> Result<()>;
}

pub trait RegisterAccessor {
    fn get_register_value(&self, register: gimli::Register) -> Result<libc::c_ulonglong>;
    /// In 32 bit machine, the value will be truncated to `c_long`
    fn set_register_value(&self, register: gimli::Register, value: libc::c_ulonglong)
        -> Result<()>;
}

pub trait NativeThreadProtocol {
    fn continute_thread(thread_id: ThreadId) -> Result<()>;

    fn start_debugged_process(
        application_name: String,
        commands: String,
        auto_attach_children: bool,
    ) -> Result<()>;

    fn attach_to_debugged_process() -> Result<()>;
}
