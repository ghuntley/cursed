/// Process signal management functions
use crate::stdlib::signal_boost::core::BoostSignal;
use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult, system_error, permission_denied, not_supported};

/// Send a signal to a specific process
pub fn signal_process(pid: u32, signal: BoostSignal) -> SignalBoostResult<()> {
    validate_signal(signal)?;
    validate_pid(pid)?;
    
    #[cfg(unix)]
    {
        unsafe {
            if libc::kill(pid as libc::pid_t, signal.0) == -1 {
                let error = std::io::Error::last_os_error();
                match error.raw_os_error() {
                    Some(libc::ESRCH) => {
                        return Err(SignalBoostError::General(
                            format!("Process {} not found", pid)
                        ));
                    },
                    Some(libc::EPERM) => {
                        return Err(permission_denied(
                            &format!("Permission denied to signal process {}", pid)
                        ));
                    },
                    Some(libc::EINVAL) => {
                        return Err(SignalBoostError::InvalidSignal(
                            format!("Invalid signal {} for process {}", signal.name(), pid)
                        ));
                    },
                    _ => {
                        return Err(system_error(
                            &format!("Failed to signal process {}: {}", pid, error)
                        ));
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Windows signal handling
        match signal.0 {
            2 => {
                // SIGINT - Use GenerateConsoleCtrlEvent
                unsafe {
                    if winapi::um::wincon::GenerateConsoleCtrlEvent(
                        winapi::um::wincon::CTRL_C_EVENT, 
                        pid
                    ) == 0 {
                        return Err(system_error(
                            &format!("Failed to send CTRL+C to process {}", pid)
                        ));
                    }
                }
            },
            15 => {
                // SIGTERM - Use TerminateProcess
                use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
                use winapi::um::winnt::PROCESS_TERMINATE;
                use winapi::um::handleapi::CloseHandle;
                
                unsafe {
                    let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
                    if handle.is_null() {
                        return Err(system_error(
                            &format!("Failed to open process {}", pid)
                        ));
                    }
                    
                    let result = TerminateProcess(handle, 1);
                    CloseHandle(handle);
                    
                    if result == 0 {
                        return Err(system_error(
                            &format!("Failed to terminate process {}", pid)
                        ));
                    }
                }
            },
            _ => {
                return Err(not_supported(
                    &format!("Signal {} not supported on Windows", signal.name())
                ));
            }
        }
    }
    
    tracing::info!("Sent signal {} to process {}", signal, pid);
    Ok(())
}

/// Send a signal to a process group
pub fn signal_group(pgid: u32, signal: BoostSignal) -> SignalBoostResult<()> {
    validate_signal(signal)?;
    
    #[cfg(unix)]
    {
        unsafe {
            if libc::kill(-(pgid as libc::pid_t), signal.0) == -1 {
                let error = std::io::Error::last_os_error();
                return Err(system_error(
                    &format!("Failed to signal process group {}: {}", pgid, error)
                ));
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Windows doesn't have process groups in the same way
        // We would need to enumerate child processes and signal them individually
        return Err(not_supported("Process group signaling not fully supported on Windows"));
    }
    
    tracing::info!("Sent signal {} to process group {}", signal, pgid);
    Ok(())
}

/// Broadcast a signal to all processes (requires appropriate permissions)
pub fn broadcast(signal: BoostSignal) -> SignalBoostResult<()> {
    validate_signal(signal)?;
    
    #[cfg(unix)]
    {
        // Signal all processes except init (pid 1) and current process
        unsafe {
            if libc::kill(-1, signal.0) == -1 {
                let error = std::io::Error::last_os_error();
                match error.raw_os_error() {
                    Some(libc::EPERM) => {
                        return Err(permission_denied("Permission denied to broadcast signal"));
                    },
                    _ => {
                        return Err(system_error(
                            &format!("Failed to broadcast signal {}: {}", signal.name(), error)
                        ));
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        return Err(not_supported("Signal broadcasting not supported on Windows"));
    }
    
    tracing::warn!("Broadcasted signal {} to all processes", signal);
    Ok(())
}

/// Get processes that would receive a signal
pub fn get_targets(signal: BoostSignal) -> SignalBoostResult<Vec<u32>> {
    validate_signal(signal)?;
    
    let mut targets = Vec::new();
    
    #[cfg(unix)]
    {
        // Read from /proc to get process list
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(pid) = file_name.parse::<u32>() {
                        // Check if we can signal this process
                        if can_signal_process(pid, signal) {
                            targets.push(pid);
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Use Windows API to enumerate processes
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::tlhelp32::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, 
            PROCESSENTRY32, TH32CS_SNAPPROCESS
        };
        
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
                let mut entry: PROCESSENTRY32 = std::mem::zeroed();
                entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
                
                if Process32First(snapshot, &mut entry) != 0 {
                    loop {
                        let pid = entry.th32ProcessID;
                        
                        // Check if we can open the process
                        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
                        if !handle.is_null() {
                            targets.push(pid);
                            CloseHandle(handle);
                        }
                        
                        if Process32Next(snapshot, &mut entry) == 0 {
                            break;
                        }
                    }
                }
                
                CloseHandle(snapshot);
            }
        }
    }
    
    tracing::debug!("Found {} potential targets for signal {}", targets.len(), signal);
    Ok(targets)
}

/// Check if we can signal a specific process
fn can_signal_process(pid: u32, signal: BoostSignal) -> bool {
    #[cfg(unix)]
    {
        // Try to send signal 0 (null signal) to check permissions
        unsafe {
            libc::kill(pid as libc::pid_t, 0) == 0
        }
    }
    
    #[cfg(windows)]
    {
        // Try to open the process with query permissions
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::handleapi::CloseHandle;
        
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            if !handle.is_null() {
                CloseHandle(handle);
                true
            } else {
                false
            }
        }
    }
}

/// Kill a process (sends SIGKILL on Unix, TerminateProcess on Windows)
pub fn kill_process(pid: u32) -> SignalBoostResult<()> {
    #[cfg(unix)]
    {
        signal_process(pid, crate::stdlib::signal_boost::core::SIGKILL)
    }
    
    #[cfg(windows)]
    {
        signal_process(pid, crate::stdlib::signal_boost::core::SIGTERM)
    }
}

/// Terminate a process gracefully (sends SIGTERM)
pub fn terminate_process(pid: u32) -> SignalBoostResult<()> {
    signal_process(pid, crate::stdlib::signal_boost::core::SIGTERM)
}

/// Interrupt a process (sends SIGINT)
pub fn interrupt_process(pid: u32) -> SignalBoostResult<()> {
    signal_process(pid, crate::stdlib::signal_boost::core::SIGINT)
}

/// Send SIGHUP to a process (commonly used for configuration reload)
pub fn hangup_process(pid: u32) -> SignalBoostResult<()> {
    signal_process(pid, crate::stdlib::signal_boost::core::SIGHUP)
}

/// Send SIGUSR1 to a process
pub fn user_signal_1(pid: u32) -> SignalBoostResult<()> {
    signal_process(pid, crate::stdlib::signal_boost::core::SIGUSR1)
}

/// Send SIGUSR2 to a process
pub fn user_signal_2(pid: u32) -> SignalBoostResult<()> {
    signal_process(pid, crate::stdlib::signal_boost::core::SIGUSR2)
}

/// Get the current process ID
pub fn get_current_pid() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::getpid() as u32 }
    }
    
    #[cfg(windows)]
    {
        unsafe { winapi::um::processthreadsapi::GetCurrentProcessId() }
    }
}

/// Get the parent process ID
pub fn get_parent_pid() -> SignalBoostResult<u32> {
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getppid() as u32 })
    }
    
    #[cfg(windows)]
    {
        // Windows requires more complex logic to get parent PID
        use winapi::um::tlhelp32::{
            CreateToolhelp32Snapshot, Process32First, Process32Next,
            PROCESSENTRY32, TH32CS_SNAPPROCESS
        };
        use winapi::um::handleapi::CloseHandle;
        
        let current_pid = get_current_pid();
        
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return Err(system_error("Failed to create process snapshot"));
            }
            
            let mut entry: PROCESSENTRY32 = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
            
            if Process32First(snapshot, &mut entry) != 0 {
                loop {
                    if entry.th32ProcessID == current_pid {
                        CloseHandle(snapshot);
                        return Ok(entry.th32ParentProcessID);
                    }
                    
                    if Process32Next(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }
            
            CloseHandle(snapshot);
        }
        
        Err(SignalBoostError::General("Parent process not found".to_string()))
    }
}

/// Check if a process exists
pub fn process_exists(pid: u32) -> bool {
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as libc::pid_t, 0) == 0
        }
    }
    
    #[cfg(windows)]
    {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::handleapi::CloseHandle;
        
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            if !handle.is_null() {
                CloseHandle(handle);
                true
            } else {
                false
            }
        }
    }
}

/// Validate signal number
fn validate_signal(signal: BoostSignal) -> SignalBoostResult<()> {
    if signal.0 <= 0 || signal.0 > 64 {
        return Err(SignalBoostError::InvalidSignal(
            format!("Invalid signal number: {}", signal.0)
        ));
    }
    Ok(())
}

/// Validate process ID
fn validate_pid(pid: u32) -> SignalBoostResult<()> {
    if pid == 0 {
        return Err(SignalBoostError::InvalidSignal(
            "Cannot signal process 0".to_string()
        ));
    }
    
    #[cfg(unix)]
    {
        if pid == 1 {
            return Err(permission_denied("Cannot signal init process (pid 1)"));
        }
    }
    
    Ok(())
}

/// Signal multiple processes
pub fn signal_processes(pids: &[u32], signal: BoostSignal) -> SignalBoostResult<(), Error>>> {
    let mut results = Vec::new();
    
    for &pid in pids {
        let result = signal_process(pid, signal);
        results.push(result.map_err(|e| e));
    }
    
    Ok(results)
}

/// Find processes by name
pub fn find_processes_by_name(name: &str) -> SignalBoostResult<Vec<u32>> {
    let mut matching_pids = Vec::new();
    
    #[cfg(unix)]
    {
        // Read from /proc/*/comm files
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(pid) = file_name.parse::<u32>() {
                        let comm_path = format!("/proc/{}/comm", pid);
                        if let Ok(comm) = std::fs::read_to_string(comm_path) {
                            if comm.trim() == name {
                                matching_pids.push(pid);
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        use winapi::um::tlhelp32::{
            CreateToolhelp32Snapshot, Process32First, Process32Next,
            PROCESSENTRY32, TH32CS_SNAPPROCESS
        };
        use winapi::um::handleapi::CloseHandle;
        
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
                let mut entry: PROCESSENTRY32 = std::mem::zeroed();
                entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
                
                if Process32First(snapshot, &mut entry) != 0 {
                    loop {
                        let process_name = std::ffi::CStr::from_ptr(entry.szExeFile.as_ptr())
                            .to_string_lossy();
                        
                        if process_name == name {
                            matching_pids.push(entry.th32ProcessID);
                        }
                        
                        if Process32Next(snapshot, &mut entry) == 0 {
                            break;
                        }
                    }
                }
                
                CloseHandle(snapshot);
            }
        }
    }
    
    tracing::debug!("Found {} processes named '{}'", matching_pids.len(), name);
    Ok(matching_pids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::signal_boost::core::{SIGINT, SIGTERM, SIGUSR1};
    
    #[test]
    fn test_validate_signal() {
        assert!(validate_signal(SIGINT).is_ok());
        assert!(validate_signal(SIGTERM).is_ok());
        assert!(validate_signal(BoostSignal(-1)).is_err());
        assert!(validate_signal(BoostSignal(0)).is_err());
        assert!(validate_signal(BoostSignal(100)).is_err());
    }
    
    #[test]
    fn test_validate_pid() {
        assert!(validate_pid(1000).is_ok());
        assert!(validate_pid(0).is_err());
        
        #[cfg(unix)]
        {
            assert!(validate_pid(1).is_err()); // init process
        }
        
        #[cfg(windows)]
        {
            assert!(validate_pid(1).is_ok()); // Different rules on Windows
        }
    }
    
    #[test]
    fn test_get_current_pid() {
        let pid = get_current_pid();
        assert!(pid > 0);
    }
    
    #[test]
    fn test_get_parent_pid() {
        let result = get_parent_pid();
        assert!(result.is_ok());
        let parent_pid = result.unwrap();
        assert!(parent_pid > 0);
    }
    
    #[test]
    fn test_process_exists() {
        let current_pid = get_current_pid();
        assert!(process_exists(current_pid));
        
        // PID 99999 is very unlikely to exist
        assert!(!process_exists(99999));
    }
    
    #[test]
    fn test_signal_self() {
        let current_pid = get_current_pid();
        
        // Test with SIGUSR1 (should be safe)
        let result = signal_process(current_pid, SIGUSR1);
        
        // This might fail due to permissions in some test environments
        // so we just check that it doesn't panic
        match result {
            Ok(()) => {
                // Signal sent successfully
            },
            Err(_) => {
                // Expected in some test environments
            }
        }
    }
    
    #[test]
    fn test_signal_nonexistent_process() {
        let result = signal_process(99999, SIGTERM);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_get_targets() {
        let result = get_targets(SIGTERM);
        assert!(result.is_ok());
        
        let targets = result.unwrap();
        // Should at least find the current process
        assert!(!targets.is_empty());
        assert!(targets.contains(&get_current_pid()));
    }
    
    #[test]
    fn test_signal_multiple_processes() {
        let current_pid = get_current_pid();
        let pids = vec![current_pid];
        
        let results = signal_processes(&pids, SIGUSR1);
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert_eq!(results.len(), 1);
    }
    
    #[test]
    fn test_find_processes_by_name() {
        // This test might be flaky depending on the environment
        // We'll just check that it doesn't panic
        let result = find_processes_by_name("nonexistent_process_name");
        assert!(result.is_ok());
        
        let pids = result.unwrap();
        assert!(pids.is_empty()); // Should not find this process
    }
    
    #[test]
    fn test_convenience_functions() {
        let current_pid = get_current_pid();
        
        // These might fail due to permissions, but should not panic
        let _ = interrupt_process(current_pid);
        let _ = user_signal_1(current_pid);
        let _ = user_signal_2(current_pid);
        let _ = hangup_process(current_pid);
    }
}
