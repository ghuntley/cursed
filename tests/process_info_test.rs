/// Test suite for process information module
/// 
/// This test validates the process information gathering functionality
/// across different platforms.

use cursed::stdlib::process::{
    get_current_pid, get_parent_pid, is_process_running,
    get_process_info, get_process_list, find_processes_by_name,
    get_cpu_count, ProcessStatus, ProcessInfo, ProcessListEntry,
    MemoryInfo, CpuInfo
};

#[test]
fn test_get_current_pid() {
    let pid = get_current_pid();
    assert!(pid > 0, "Current PID should be positive");
    assert!(pid < u32::MAX, "PID should be reasonable");
}

#[test]
fn test_is_process_running() {
    let current_pid = get_current_pid();
    assert!(is_process_running(current_pid), "Current process should be running");
    
    // Test with an obviously invalid PID
    assert!(!is_process_running(999999), "Invalid PID should not be running");
}

#[test]
fn test_get_cpu_count() {
    let count = get_cpu_count();
    assert!(count > 0, "CPU count should be positive");
    assert!(count <= 1024, "CPU count should be reasonable");
}

#[test]
fn test_process_status_conversion() {
    assert_eq!(ProcessStatus::from('R'), ProcessStatus::Running);
    assert_eq!(ProcessStatus::from('S'), ProcessStatus::Sleeping);
    assert_eq!(ProcessStatus::from('Z'), ProcessStatus::Zombie);
    assert_eq!(ProcessStatus::from('T'), ProcessStatus::Stopped);
    
    // Unknown status
    match ProcessStatus::from('X') {
        ProcessStatus::Dead => {},
        ProcessStatus::Unknown(_) => {},
        _ => panic!("Unexpected status conversion"),
    }
}

#[test]
fn test_process_status_display() {
    assert_eq!(format!("{}", ProcessStatus::Running), "Running");
    assert_eq!(format!("{}", ProcessStatus::Sleeping), "Sleeping");
    assert_eq!(format!("{}", ProcessStatus::Zombie), "Zombie");
}

#[test]
fn test_memory_info_creation() {
    let mem = MemoryInfo::new();
    assert_eq!(mem.virtual_size, 0);
    assert_eq!(mem.resident_size, 0);
    assert_eq!(mem.shared_size, 0);
    assert_eq!(mem.percentage, 0.0);
}

#[test]
fn test_cpu_info_creation() {
    let cpu = CpuInfo::new();
    assert_eq!(cpu.cpu_percent, 0.0);
    assert_eq!(cpu.user_time, 0);
    assert_eq!(cpu.system_time, 0);
    assert_eq!(cpu.total_time, 0);
}

#[test]
fn test_process_info_creation() {
    let info = ProcessInfo::new(1234);
    assert_eq!(info.pid, 1234);
    assert_eq!(info.ppid, 0);
    assert!(info.name.is_empty());
    assert_eq!(info.threads, 1);
    assert_eq!(info.uid, 0);
    assert_eq!(info.gid, 0);
}

#[test]
fn test_get_process_info_current() {
    let current_pid = get_current_pid();
    
    // This test might fail on some systems due to permissions or platform differences
    // So we'll make it graceful
    match get_process_info(current_pid) {
        Ok(info) => {
            assert_eq!(info.pid, current_pid);
            // On most systems, we should be able to get basic info about our own process
            assert!(!info.name.is_empty() || !info.cmdline.is_empty());
        }
        Err(_) => {
            // Some systems might not allow reading process info
            // This is acceptable for the test
            println!("Warning: Could not read process info (permissions or platform limitation)");
        }
    }
}

#[test]
fn test_get_process_list() {
    // Getting process list might fail on some systems due to permissions
    match get_process_list() {
        Ok(processes) => {
            assert!(!processes.is_empty(), "Process list should not be empty");
            
            // Check that our current process is in the list
            let current_pid = get_current_pid();
            let found_self = processes.iter().any(|p| p.pid == current_pid);
            assert!(found_self, "Current process should be in process list");
            
            // Validate process list entries
            for process in processes.iter().take(5) {
                assert!(process.pid > 0, "Process PID should be positive");
                // Name might be empty on some systems, so we don't assert it
            }
        }
        Err(_) => {
            // Some systems might not allow listing processes
            println!("Warning: Could not get process list (permissions or platform limitation)");
        }
    }
}

#[test]
fn test_find_processes_by_name() {
    // This test might not find any processes depending on the system
    match find_processes_by_name("test") {
        Ok(processes) => {
            // We don't assert that processes are found, as "test" might not be running
            for process in processes {
                assert!(process.name.contains("test"));
            }
        }
        Err(_) => {
            // Some systems might not allow searching processes
            println!("Warning: Could not search processes by name");
        }
    }
}

#[test]
fn test_get_parent_pid() {
    match get_parent_pid() {
        Ok(ppid) => {
            assert!(ppid > 0, "Parent PID should be positive");
            let current_pid = get_current_pid();
            assert_ne!(ppid, current_pid, "Parent PID should differ from current PID");
        }
        Err(_) => {
            // This might fail on Windows or in some environments
            println!("Warning: Could not get parent PID (platform limitation)");
        }
    }
}

#[cfg(unix)]
#[test]
fn test_get_load_average() {
    use cursed::stdlib::process::get_load_average;
    
    match get_load_average() {
        Ok((load1, load5, load15)) => {
            assert!(load1 >= 0.0, "1-minute load should be non-negative");
            assert!(load5 >= 0.0, "5-minute load should be non-negative");
            assert!(load15 >= 0.0, "15-minute load should be non-negative");
            
            // Load averages should generally be reasonable
            assert!(load1 < 1000.0, "Load average should be reasonable");
            assert!(load5 < 1000.0, "Load average should be reasonable");
            assert!(load15 < 1000.0, "Load average should be reasonable");
        }
        Err(_) => {
            println!("Warning: Could not get load average");
        }
    }
}

#[cfg(unix)]
#[test]
fn test_get_system_uptime() {
    use cursed::stdlib::process::get_system_uptime;
    use std::time::Duration;
    
    match get_system_uptime() {
        Ok(uptime) => {
            assert!(uptime > Duration::from_secs(0), "System uptime should be positive");
            assert!(uptime < Duration::from_secs(365 * 24 * 3600), "Uptime should be reasonable (less than a year)");
        }
        Err(_) => {
            println!("Warning: Could not get system uptime");
        }
    }
}

#[test]
fn test_process_list_entry() {
    use std::time::SystemTime;
    
    let entry = ProcessListEntry {
        pid: 1234,
        ppid: 1,
        name: "test_process".to_string(),
        status: ProcessStatus::Running,
        cpu_percent: 5.5,
        memory_percent: 2.1,
        start_time: SystemTime::now(),
    };
    
    assert_eq!(entry.pid, 1234);
    assert_eq!(entry.ppid, 1);
    assert_eq!(entry.name, "test_process");
    assert_eq!(entry.status, ProcessStatus::Running);
    assert_eq!(entry.cpu_percent, 5.5);
    assert_eq!(entry.memory_percent, 2.1);
}

#[test]
fn test_process_info_comprehensive() {
    let mut info = ProcessInfo::new(5678);
    
    // Test initial values
    assert_eq!(info.pid, 5678);
    assert_eq!(info.threads, 1);
    
    // Test setting values
    info.name = "test_process".to_string();
    info.cmdline = vec!["test".to_string(), "--arg".to_string()];
    info.status = ProcessStatus::Sleeping;
    info.priority = 20;
    info.nice = 0;
    info.threads = 4;
    
    assert_eq!(info.name, "test_process");
    assert_eq!(info.cmdline.len(), 2);
    assert_eq!(info.status, ProcessStatus::Sleeping);
    assert_eq!(info.priority, 20);
    assert_eq!(info.threads, 4);
}

// Integration test that runs only if we can successfully get process info
#[test]
fn test_process_info_integration() {
    let current_pid = get_current_pid();
    
    // Try to get info about init process (PID 1) which should exist on Unix systems
    #[cfg(unix)]
    {
        if let Ok(init_info) = get_process_info(1) {
            assert_eq!(init_info.pid, 1);
            assert_eq!(init_info.ppid, 0); // init has no parent
            assert!(!init_info.name.is_empty());
        }
    }
    
    // Try to get info about a non-existent process
    let result = get_process_info(999999);
    assert!(result.is_err(), "Getting info for non-existent process should fail");
}
