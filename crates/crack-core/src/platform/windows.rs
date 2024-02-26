/// Like nix crate for libc, this file wraps Windows API to be safe call.
/// This is the only module that is able to call unsafe code for windows API.
mod windows_safe {
    use windows::Win32::System::Diagnostics::Debug::{
        WaitForDebugEvent as __WaitForDebugEvent, DEBUG_EVENT,
    };

    pub fn WaitForDebugEvent(dwmilliseconds: u32) -> windows_core::Result<DEBUG_EVENT> {
        let mut debug_event: DEBUG_EVENT = DEBUG_EVENT::default();

        let res = unsafe { __WaitForDebugEvent(&mut debug_event, dwmilliseconds) };

        res.map(|_| debug_event)
    }
}

#[cfg(test)]
mod tests {
    use windows::Win32::System::{
        Diagnostics::Debug::{DebugActiveProcess, DEBUG_EVENT},
        Threading::INFINITE,
    };

    use super::windows_safe::WaitForDebugEvent;

    #[test]
    fn test_1() {
        let mut tmp: DEBUG_EVENT = DEBUG_EVENT::default();
        let res = WaitForDebugEvent(INFINITE);
    }
}
