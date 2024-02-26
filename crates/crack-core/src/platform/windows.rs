#[cfg(test)]
mod tests {
    use windows::Win32::System::{Diagnostics::Debug::DebugActiveProcess, ProcessStatus::EnumProcesses};

    #[test]
    fn test_1() {
        EnumProcesses;
    }
}
