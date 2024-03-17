use std::mem::{size_of, size_of_val};

use anyhow::Result;

use super::general::{MemoryAccessor, MemoryAddress, ThreadId};

// From LLDB:
// g_aarch64_opcode[] = {};
// g_i386_opcode[] = {0xCC};
// g_mips64_opcode[] = {0x00, 0x00, 0x00, 0x0d};
// g_mips64el_opcode[] = {0x0d, 0x00, 0x00, 0x00};
// g_msp430_opcode[] = {0x43, 0x43};
// g_s390x_opcode[] = {0x00, 0x01};
// g_ppc_opcode[] = {0x7f, 0xe0, 0x00, 0x08};   // trap
// g_ppcle_opcode[] = {0x08, 0x00, 0xe0, 0x7f}; // trap
// g_riscv_opcode[] = {0x73, 0x00, 0x10, 0x00}; // ebreak
// g_riscv_opcode_c[] = {0x02, 0x90};           // c.ebreak
// g_loongarch_opcode[] = {0x05, 0x00, 0x2a, 0x00}; // break 0x5

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// refer https://handwiki.org/wiki/INT_(x86_instruction)#INT3
const SOFTWARE_BREAKPOINT_TRAP_OPCODE: [u8; 1] = [0xcc];
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(nightly)))]
const SOFTWARE_BREAKPOINT_TRAP_OPCODE_SIZE: usize = 1;

#[cfg(target_arch = "arm")]
const SOFTWARE_BREAKPOINT_TRAP_OPCODE: [u8; 4] = [0x00, 0x00, 0x20, 0xd4];
#[cfg(all(target_arch = "arm", not(nightly)))]
const SOFTWARE_BREAKPOINT_TRAP_OPCODE_SIZE: usize = 4;

#[cfg(nightly)]
const SOFTWARE_BREAKPOINT_TRAP_OPCODE_SIZE: usize = size_of_val(&SOFTWARE_BREAKPOINT_TRAP_OPCODE);

pub struct SoftwareBreakpoint {
    thread_id: ThreadId,
    memory_address: MemoryAddress,
    is_enabled: bool,
    saved_instruction: Option<[u8; SOFTWARE_BREAKPOINT_TRAP_OPCODE_SIZE]>,
}

impl SoftwareBreakpoint {
    pub fn new(thread_id: ThreadId, memory_address: MemoryAddress) -> Self {
        SoftwareBreakpoint {
            thread_id,
            memory_address,
            is_enabled: false,
            saved_instruction: Option::None,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn get_address(&self) -> MemoryAddress {
        self.memory_address
    }

    pub fn enable<T>(&mut self, memory_accessor: T) -> Result<()>
    where
        T: MemoryAccessor,
    {
        let origin_instruction = memory_accessor
            .read_memory::<SOFTWARE_BREAKPOINT_TRAP_OPCODE_SIZE>(
                self.memory_address,
            )?;

        memory_accessor.write_memory(
            self.memory_address,
            SOFTWARE_BREAKPOINT_TRAP_OPCODE,
        )?;

        // TODO: Does it need to check it's None firstly?
        self.saved_instruction.replace(origin_instruction);

        self.is_enabled = true;

        Result::Ok(())
    }

    pub fn disable<T>(&mut self, memory_accessor: T) -> Result<()>
    where
        T: MemoryAccessor,
    {
        memory_accessor.write_memory(
            self.thread_id,
            self.memory_address,
            self.saved_instruction
                .expect("Failed when disable software breakpoint, no valid data to write back"),
        )?;

        self.is_enabled = false;

        Result::Ok(())
    }
}
