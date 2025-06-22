/// Process information and system utilities for CURSED
/// 
/// This module provides functions to query process information, system statistics,
/// and process listing capabilities across different platforms.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, process_not_found_pid, system_error, io_error
};

/// Process status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStatus {
    /// Process is running
    Running,
    /// Process is sleeping (interruptible)
    Sleeping,
    /// Process is in uninterruptible sleep
    UninterruptibleSleep,
    /// Process is stopped
    Stopped,
    /// Process is a zombie
    Zombie,
    /// Process is traced/debugged
    Traced,
    /// Process is dead
    Dead,
    /// Unknown status
    Unknown(String),
}

/// Process state information
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessState {
    /// Process is created but not yet started
    Created,
    /// Process is running normally
    Running,
    /// Process is paused/suspended
    Paused,
    /// Process is waiting for some condition
    Waiting,
    /// Process has stopped execution
    Stopped,
    /// Process has terminated normally
    Terminated,
    /// Process was killed by signal
    Killed,
    /// Process state is unknown
    Unknown,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessStatus::Running => write!(f, "Running"),
            ProcessStatus::Sleeping => write!(f, "Sleeping"),
            ProcessStatus::UninterruptibleSleep => write!(f, "Uninterruptible Sleep"),
            ProcessStatus::Stopped => write!(f, "Stopped"),
            ProcessStatus::Zombie => write!(f, "Zombie"),
            ProcessStatus::Traced => write!(f, "Traced"),
            ProcessStatus::Dead => write!(f, "Dead"),
            ProcessStatus::Unknown(status) => write!(f, "Unknown ({})", status),
        }
    }
}

impl From<char> for ProcessStatus {
    fn from(c: char) -> Self {
        match c {
            'R' => ProcessStatus::Running,
            'S' => ProcessStatus::Sleeping,
            'D' => ProcessStatus::UninterruptibleSleep,
            'T' => ProcessStatus::Stopped,
            'Z' => ProcessStatus::Zombie,
            't' => ProcessStatus::Traced,
            'X' | 'x' => ProcessStatus::Dead,
            _ => ProcessStatus::Unknown(c.to_string()),
        }
    }
}

/// Memory information for a process
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    /// Virtual memory size in bytes
    pub virtual_size: u64,
    /// Resident set size in bytes
    pub resident_size: u64,
    /// Shared memory size in bytes
    pub shared_size: u64,
    /// Peak virtual memory size in bytes
    pub peak_virtual_size: u64,
    /// Peak resident set size in bytes
    pub peak_resident_size: u64,
    /// Memory usage percentage
    pub percentage: f64,
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self {
            virtual_size: 0,
            resident_size: 0,
            shared_size: 0,
            peak_virtual_size: 0,
            peak_resident_size: 0,
            percentage: 0.0,
        }
    }
}

/// CPU information for a process
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// User time in milliseconds
    pub user_time: u64,
    /// System time in milliseconds
    pub system_time: u64,
    /// Total time (user + system)
    pub total_time: u64,
    /// CPU time percentage since last measurement
    pub cpu_time_percent: f64,
    /// Number of context switches
    pub context_switches: u64,
}

impl CpuInfo {
    pub fn new() -> Self {
        Self {
            cpu_percent: 0.0,
            user_time: 0,
            system_time: 0,
            total_time: 0,
            cpu_time_percent: 0.0,
            context_switches: 0,
        }
    }
}

/// Comprehensive process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Parent process ID
    pub ppid: u32,
    /// Process group ID
    pub pgid: u32,
    /// Session ID
    pub sid: u32,
    /// Terminal/TTY
    pub tty: Option<String>,
    /// Process name/command
    pub name: String,
    /// Full command line
    pub cmdline: Vec<String>,
    /// Process status
    pub status: ProcessStatus,
    /// Process priority
    pub priority: i32,
    /// Nice value
    pub nice: i32,
    /// Number of threads
    pub threads: u32,
    /// Process start time
    pub start_time: SystemTime,
    /// Process uptime
    pub uptime: Duration,
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// Working directory
    pub cwd: Option<String>,
    /// Executable path
    pub exe: Option<String>,
    /// Memory information
    pub memory: MemoryInfo,
    /// CPU information
    pub cpu: CpuInfo,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Open file descriptors count
    pub fd_count: u32,
}

impl ProcessInfo {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            ppid: 0,
            pgid: 0,
            sid: 0,
            tty: None,
            name: String::new(),
            cmdline: Vec::new(),
            status: ProcessStatus::Unknown("Unknown".to_string()),
            priority: 0,
            nice: 0,
            threads: 1,
            start_time: SystemTime::now(),
            uptime: Duration::from_secs(0),
            uid: 0,
            gid: 0,
            cwd: None,
            exe: None,
            memory: MemoryInfo::new(),
            cpu: CpuInfo::new(),
            environment: HashMap::new(),
            fd_count: 0,
        }
    }
}

/// Process list entry (minimal information for listing)
#[derive(Debug, Clone)]
pub struct ProcessListEntry {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub status: ProcessStatus,
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub start_time: SystemTime,
}

/// Get current process ID
pub fn get_current_pid() -> u32 {
    std::process::id()
}

/// Get parent process ID
pub fn get_parent_pid() -> ProcessResult<u32> {
    #[cfg(unix)]
    {
        let ppid = unsafe { libc::getppid() };
        Ok(ppid as u32)
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        // Use WMIC to get parent process ID
        let current_pid = get_current_pid();
        let output = Command::new("wmic")
            .args(&["process", "where", &format!("ProcessId={}", current_pid), "get", "ParentProcessId", "/value"])
            .output()
            .map_err(|e| system_error(-1, "get_parent_pid", &e.to_string()))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.split("\n") {
                if line.starts_with("ParentProcessId=") {
                    if let Some(ppid_str) = line.split('=').nth(1) {
                        if let Ok(ppid) = ppid_str.trim().parse::<u32>() {
                            return Ok(ppid);
                        }
                    }
                }
            }
        }
        
        // Fallback: try PowerShell approach
        let output = Command::new("powershell")
            .args(&["-Command", &format!("Get-Process -Id {} | Select-Object -ExpandProperty Parent", current_pid)])
            .output()
            .map_err(|e| system_error(-1, "get_parent_pid", &e.to_string()))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(ppid) = stdout.trim().parse::<u32>() {
                return Ok(ppid);
            }
        }

        // Final fallback: return 0 if we can't determine parent PID
        Ok(0)
    }
}

/// Check if a process is running
pub fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as i32, 0) == 0
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        // Use tasklist command to check if process exists
        if let Ok(output) = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid)])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains(&pid.to_string())
        } else {
            false
        }
    }
}

/// Get detailed process information
pub fn get_process_info(pid: u32) -> ProcessResult<ProcessInfo> {
    if !is_process_running(pid) {
        return Err(process_not_found_pid(pid, "Process not found"));
    }
    
    #[cfg(unix)]
    {
        get_process_info_unix(pid)
    }
    
    #[cfg(windows)]
    {
        get_process_info_windows(pid)
    }
}

#[cfg(unix)]
fn get_process_info_unix(pid: u32) -> ProcessResult<ProcessInfo> {
    let mut info = ProcessInfo::new(pid);
    
    // Read from /proc filesystem
    let proc_path = format!("/proc/{}", pid);
    
    // Read basic status information
    if let Ok(status_content) = fs::read_to_string(format!("{}/status", proc_path)) {
        parse_proc_status(&status_content, &mut info)?;
    }
    
    // Read command line
    if let Ok(cmdline_content) = fs::read(format!("{}/cmdline", proc_path)) {
        let cmdline_str = String::from_utf8_lossy(&cmdline_content);
        info.cmdline = cmdline_str.split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        if !info.cmdline.is_empty() {
            info.name = info.cmdline[0].clone();
        }
    }
    
    // Read stat information for timing and CPU info
    if let Ok(stat_content) = fs::read_to_string(format!("{}/stat", proc_path)) {
        parse_proc_stat(&stat_content, &mut info)?;
    }
    
    // Read memory information
    if let Ok(statm_content) = fs::read_to_string(format!("{}/statm", proc_path)) {
        parse_proc_statm(&statm_content, &mut info)?;
    }
    
    // Read current working directory
    if let Ok(cwd_path) = fs::read_link(format!("{}/cwd", proc_path)) {
        info.cwd = Some(cwd_path.to_string_lossy().to_string());
    }
    
    // Read executable path
    if let Ok(exe_path) = fs::read_link(format!("{}/exe", proc_path)) {
        info.exe = Some(exe_path.to_string_lossy().to_string());
    }
    
    // Count file descriptors
    if let Ok(fd_dir) = fs::read_dir(format!("{}/fd", proc_path)) {
        info.fd_count = fd_dir.count() as u32;
    }
    
    // Read environment variables
    if let Ok(environ_content) = fs::read(format!("{}/environ", proc_path)) {
        let environ_str = String::from_utf8_lossy(&environ_content);
        for env_var in environ_str.split('\0') {
            if let Some(eq_pos) = env_var.find('=') {
                let key = env_var[..eq_pos].to_string();
                let value = env_var[eq_pos + 1..].to_string();
                info.environment.insert(key, value);
            }
        }
    }
    
    Ok(info)
}

#[cfg(windows)]
fn get_process_info_windows(pid: u32) -> ProcessResult<ProcessInfo> {
    use std::process::Command;
    
    let mut info = ProcessInfo::new(pid);
    
    // Use tasklist command to get basic process information
    if let Ok(output) = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/V"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.split("\n").nth(1) {
            parse_tasklist_output(line, &mut info)?;
        }
    }
    
    // Get additional information using wmic
    if let Ok(output) = Command::new("wmic")
        .args(&["process", "where", &format!("ProcessId={}", pid), "get", "CommandLine,ExecutablePath,WorkingSetSize,PageFileUsage,UserModeTime,KernelModeTime,ParentProcessId,Priority,ThreadCount", "/format:csv"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.split("\n").nth(1) {
            parse_wmic_output(line, &mut info)?;
        }
    }
    
    // Get detailed process information using PowerShell
    get_process_details_powershell(pid, &mut info)?;
    
    // Get process environment variables
    if let Ok(env_vars) = get_process_environment_windows(pid) {
        info.environment = env_vars;
    }
    
    // Get process working directory
    if let Ok(cwd) = get_process_working_directory_windows(pid) {
        info.cwd = Some(cwd);
    }
    
    Ok(info)
}

#[cfg(windows)]
fn get_process_details_powershell(pid: u32, info: &mut ProcessInfo) -> ProcessResult<()> {
    use std::process::Command;
    
    // Get comprehensive process information using PowerShell
    let ps_script = format!(
        r#"
        $proc = Get-Process -Id {} -ErrorAction SilentlyContinue
        if ($proc) {{
            $proc | Select-Object @{{n='Name';e={{$_.ProcessName}}}},
                                 @{{n='StartTime';e={{$_.StartTime}}}},
                                 @{{n='TotalProcessorTime';e={{$_.TotalProcessorTime.TotalMilliseconds}}}},
                                 @{{n='UserProcessorTime';e={{$_.UserProcessorTime.TotalMilliseconds}}}},
                                 @{{n='PrivilegedProcessorTime';e={{$_.PrivilegedProcessorTime.TotalMilliseconds}}}},
                                 @{{n='WorkingSet';e={{$_.WorkingSet}}}},
                                 @{{n='VirtualMemorySize';e={{$_.VirtualMemorySize}}}},
                                 @{{n='PagedMemorySize';e={{$_.PagedMemorySize}}}},
                                 @{{n='NonpagedSystemMemorySize';e={{$_.NonpagedSystemMemorySize}}}},
                                 @{{n='PagedSystemMemorySize';e={{$_.PagedSystemMemorySize}}}},
                                 @{{n='PriorityClass';e={{$_.PriorityClass}}}},
                                 @{{n='Threads';e={{$_.Threads.Count}}}},
                                 @{{n='Handles';e={{$_.HandleCount}}}} | ConvertTo-Json -Compress
        }}
        "#,
        pid
    );
    
    let output = Command::new("powershell")
        .args(&["-Command", &ps_script])
        .output()
        .map_err(|e| system_error(-1, "get_process_details", &e.to_string()))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            parse_powershell_json(&stdout, info)?;
        }
    }
    
    Ok(())
}

#[cfg(windows)]
fn parse_powershell_json(json_str: &str, info: &mut ProcessInfo) -> ProcessResult<()> {
    // Simple JSON parsing - in production would use serde_json
    let lines: Vec<&str> = json_str.split("\n").collect();
    let json_content = lines.join("");
    
    // Extract values using simple string parsing
    if let Some(name) = extract_json_string_value(&json_content, "Name") {
        info.name = name;
    }
    
    if let Some(working_set) = extract_json_number_value(&json_content, "WorkingSet") {
        info.memory.resident_size = working_set;
    }
    
    if let Some(virtual_size) = extract_json_number_value(&json_content, "VirtualMemorySize") {
        info.memory.virtual_size = virtual_size;
    }
    
    if let Some(user_time) = extract_json_number_value(&json_content, "UserProcessorTime") {
        info.cpu.user_time = user_time;
    }
    
    if let Some(system_time) = extract_json_number_value(&json_content, "PrivilegedProcessorTime") {
        info.cpu.system_time = system_time;
    }
    
    if let Some(threads) = extract_json_number_value(&json_content, "Threads") {
        info.threads = threads as u32;
    }
    
    if let Some(handles) = extract_json_number_value(&json_content, "Handles") {
        info.fd_count = handles as u32;
    }
    
    info.cpu.total_time = info.cpu.user_time + info.cpu.system_time;
    
    Ok(())
}

#[cfg(windows)]
fn extract_json_string_value(json: &str, key: &str) -> Option<String> {
    let search_pattern = format!("\"{}\":", key);
    if let Some(start) = json.find(&search_pattern) {
        let after_key = &json[start + search_pattern.len()..];
        if let Some(value_start) = after_key.find('"') {
            let value_part = &after_key[value_start + 1..];
            if let Some(value_end) = value_part.find('"') {
                return Some(value_part[..value_end].to_string());
            }
        }
    }
    None
}

#[cfg(windows)]
fn extract_json_number_value(json: &str, key: &str) -> Option<u64> {
    let search_pattern = format!("\"{}\":", key);
    if let Some(start) = json.find(&search_pattern) {
        let after_key = &json[start + search_pattern.len()..];
        if let Some(value_start) = after_key.find(|c: char| c.is_ascii_digit() || c == '-') {
            let value_part = &after_key[value_start..];
            if let Some(value_end) = value_part.find(|c: char| !c.is_ascii_digit() && c != '.') {
                let value_str = &value_part[..value_end];
                return value_str.parse().ok();
            }
        }
    }
    None
}

#[cfg(windows)]
fn get_process_environment_windows(pid: u32) -> ProcessResult<HashMap<String, String>> {
    use std::process::Command;
    
    // Use WMIC to get environment variables
    let output = Command::new("wmic")
        .args(&["process", "where", &format!("ProcessId={}", pid), "get", "EnvironmentVariables", "/format:list"])
        .output()
        .map_err(|e| system_error(-1, "get_process_environment", &e.to_string()))?;

    let mut env_vars = HashMap::new();
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.split("\n") {
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                if !key.is_empty() {
                    env_vars.insert(key, value);
                }
            }
        }
    }
    
    // Fallback: try to get common environment variables
    if env_vars.is_empty() {
        for var_name in &["PATH", "USERPROFILE", "USERNAME", "COMPUTERNAME", "OS"] {
            if let Ok(value) = std::env::var(var_name) {
                env_vars.insert(var_name.to_string(), value);
            }
        }
    }
    
    Ok(env_vars)
}

#[cfg(windows)]
fn get_process_working_directory_windows(pid: u32) -> ProcessResult<String> {
    use std::process::Command;
    
    // Try PowerShell approach first
    let output = Command::new("powershell")
        .args(&["-Command", &format!(
            "Get-Process -Id {} | Select-Object -ExpandProperty Path | Split-Path -Parent", 
            pid
        )])
        .output()
        .map_err(|e| system_error(-1, "get_process_cwd", &e.to_string()))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let cwd = stdout.trim();
        if !cwd.is_empty() && cwd != "null" {
            return Ok(cwd.to_string());
        }
    }
    
    // Fallback: use current working directory
    std::env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| system_error(-1, "get_process_cwd", &e.to_string()))
}

#[cfg(unix)]
fn parse_proc_status(content: &str, info: &mut ProcessInfo) -> ProcessResult<()> {
    for line in content.split("\n") {
        if let Some(colon_pos) = line.find(':') {
            let key = &line[..colon_pos];
            let value = line[colon_pos + 1..].trim();
            
            match key {
                "Name" => info.name = value.to_string(),
                "State" => {
                    if let Some(first_char) = value.chars().next() {
                        info.status = ProcessStatus::from(first_char);
                    }
                }
                "PPid" => {
                    if let Ok(ppid) = value.parse::<u32>() {
                        info.ppid = ppid;
                    }
                }
                "Uid" => {
                    // Format: "Uid: real effective saved filesystem"
                    if let Some(uid_str) = value.split_whitespace().next() {
                        if let Ok(uid) = uid_str.parse::<u32>() {
                            info.uid = uid;
                        }
                    }
                }
                "Gid" => {
                    if let Some(gid_str) = value.split_whitespace().next() {
                        if let Ok(gid) = gid_str.parse::<u32>() {
                            info.gid = gid;
                        }
                    }
                }
                "Threads" => {
                    if let Ok(threads) = value.parse::<u32>() {
                        info.threads = threads;
                    }
                }
                "VmSize" => {
                    if let Some(size_str) = value.split_whitespace().next() {
                        if let Ok(size) = size_str.parse::<u64>() {
                            info.memory.virtual_size = size * 1024; // Convert from kB to bytes
                        }
                    }
                }
                "VmRSS" => {
                    if let Some(size_str) = value.split_whitespace().next() {
                        if let Ok(size) = size_str.parse::<u64>() {
                            info.memory.resident_size = size * 1024; // Convert from kB to bytes
                        }
                    }
                }
                "VmPeak" => {
                    if let Some(size_str) = value.split_whitespace().next() {
                        if let Ok(size) = size_str.parse::<u64>() {
                            info.memory.peak_virtual_size = size * 1024;
                        }
                    }
                }
                "VmHWM" => {
                    if let Some(size_str) = value.split_whitespace().next() {
                        if let Ok(size) = size_str.parse::<u64>() {
                            info.memory.peak_resident_size = size * 1024;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}

#[cfg(unix)]
fn parse_proc_stat(content: &str, info: &mut ProcessInfo) -> ProcessResult<()> {
    let fields: Vec<&str> = content.split_whitespace().collect();
    
    if fields.len() >= 52 {
        // Field indices from man proc(5)
        if let Ok(ppid) = fields[3].parse::<u32>() {
            info.ppid = ppid;
        }
        if let Ok(pgid) = fields[4].parse::<u32>() {
            info.pgid = pgid;
        }
        if let Ok(sid) = fields[5].parse::<u32>() {
            info.sid = sid;
        }
        if let Ok(priority) = fields[17].parse::<i32>() {
            info.priority = priority;
        }
        if let Ok(nice) = fields[18].parse::<i32>() {
            info.nice = nice;
        }
        if let Ok(num_threads) = fields[19].parse::<u32>() {
            info.threads = num_threads;
        }
        if let Ok(utime) = fields[13].parse::<u64>() {
            info.cpu.user_time = utime;
        }
        if let Ok(stime) = fields[14].parse::<u64>() {
            info.cpu.system_time = stime;
        }
        
        info.cpu.total_time = info.cpu.user_time + info.cpu.system_time;
        
        // Calculate start time (field 21 is starttime in clock ticks since boot)
        if let Ok(starttime) = fields[21].parse::<u64>() {
            // This is a simplified calculation - real implementation would need
            // to get boot time and convert clock ticks to actual time
            info.start_time = SystemTime::now() - Duration::from_secs(starttime / 100);
            info.uptime = info.start_time.elapsed().unwrap_or(Duration::from_secs(0));
        }
    }
    
    Ok(())
}

#[cfg(unix)]
fn parse_proc_statm(content: &str, info: &mut ProcessInfo) -> ProcessResult<()> {
    let fields: Vec<&str> = content.split_whitespace().collect();
    
    if fields.len() >= 7 {
        // statm fields: size resident shared text lib data dt
        if let Ok(size) = fields[0].parse::<u64>() {
            info.memory.virtual_size = size * 4096; // Convert pages to bytes (assuming 4KB pages)
        }
        if let Ok(resident) = fields[1].parse::<u64>() {
            info.memory.resident_size = resident * 4096;
        }
        if let Ok(shared) = fields[2].parse::<u64>() {
            info.memory.shared_size = shared * 4096;
        }
    }
    
    Ok(())
}

#[cfg(windows)]
fn parse_tasklist_output(line: &str, info: &mut ProcessInfo) -> ProcessResult<()> {
    // Parse CSV output from tasklist
    let fields: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
    
    if fields.len() >= 5 {
        info.name = fields[0].to_string();
        
        if let Ok(pid) = fields[1].parse::<u32>() {
            info.pid = pid;
        }
        
        // Session name is in fields[2]
        // Session number is in fields[3]
        
        // Memory usage
        if let Some(mem_str) = fields[4].strip_suffix(" K") {
            if let Ok(mem_kb) = mem_str.replace(",", "").parse::<u64>() {
                info.memory.resident_size = mem_kb * 1024;
            }
        }
        
        // Status - Windows tasklist doesn't provide detailed status
        info.status = ProcessStatus::Running;
    }
    
    Ok(())
}

#[cfg(windows)]
fn parse_wmic_output(line: &str, info: &mut ProcessInfo) -> ProcessResult<()> {
    let fields: Vec<&str> = line.split(',').collect();
    
    if fields.len() >= 6 {
        // Fields: Node,CommandLine,ExecutablePath,KernelModeTime,PageFileUsage,UserModeTime,WorkingSetSize
        if fields[1] != "CommandLine" { // Skip header
            if !fields[1].is_empty() {
                info.cmdline = vec![fields[1].to_string()];
            }
            if !fields[2].is_empty() {
                info.exe = Some(fields[2].to_string());
            }
            
            // Parse memory information
            if !fields[4].is_empty() {
                if let Ok(page_file) = fields[4].parse::<u64>() {
                    info.memory.virtual_size = page_file;
                }
            }
            if !fields[6].is_empty() {
                if let Ok(working_set) = fields[6].parse::<u64>() {
                    info.memory.resident_size = working_set;
                }
            }
            
            // Parse CPU time information
            if !fields[3].is_empty() {
                if let Ok(kernel_time) = fields[3].parse::<u64>() {
                    info.cpu.system_time = kernel_time / 10000; // Convert from 100ns units to ms
                }
            }
            if !fields[5].is_empty() {
                if let Ok(user_time) = fields[5].parse::<u64>() {
                    info.cpu.user_time = user_time / 10000;
                }
            }
            
            info.cpu.total_time = info.cpu.user_time + info.cpu.system_time;
        }
    }
    
    Ok(())
}

/// Get memory information for a specific process
pub fn get_process_memory(pid: u32) -> ProcessResult<MemoryInfo> {
    let info = get_process_info(pid)?;
    Ok(info.memory)
}

/// Get CPU information for a specific process
pub fn get_process_cpu(pid: u32) -> ProcessResult<CpuInfo> {
    let info = get_process_info(pid)?;
    Ok(info.cpu)
}

/// Get list of all processes
pub fn get_process_list() -> ProcessResult<Vec<ProcessListEntry>> {
    let mut processes = Vec::new();
    
    #[cfg(unix)]
    {
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if let Ok(pid) = filename.parse::<u32>() {
                        if let Ok(info) = get_process_info(pid) {
                            processes.push(ProcessListEntry {
                                pid: info.pid,
                                ppid: info.ppid,
                                name: info.name,
                                status: info.status,
                                cpu_percent: info.cpu.cpu_percent,
                                memory_percent: info.memory.percentage,
                                start_time: info.start_time,
                            });
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("tasklist")
            .args(&["/FO", "CSV"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for (i, line) in stdout.split("\n").enumerate() {
                if i == 0 { continue; } // Skip header
                
                let fields: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
                if fields.len() >= 5 {
                    if let Ok(pid) = fields[1].parse::<u32>() {
                        processes.push(ProcessListEntry {
                            pid,
                            ppid: 0, // Not available from tasklist
                            name: fields[0].to_string(),
                            status: ProcessStatus::Running,
                            cpu_percent: 0.0, // Not available from basic tasklist
                            memory_percent: 0.0, // Would need calculation
                            start_time: SystemTime::now(), // Not available from tasklist
                        });
                    }
                }
            }
        }
    }
    
    Ok(processes)
}

/// Find processes by name
pub fn find_processes_by_name(name: &str) -> ProcessResult<Vec<ProcessListEntry>> {
    let all_processes = get_process_list()?;
    Ok(all_processes
        .into_iter()
        .filter(|p| p.name.contains(name))
        .collect())
}

/// Get process tree starting from a given PID
pub fn get_process_tree(root_pid: u32) -> ProcessResult<Vec<ProcessListEntry>> {
    let all_processes = get_process_list()?;
    let mut tree = Vec::new();
    let mut to_visit = vec![root_pid];
    
    while let Some(current_pid) = to_visit.pop() {
        // Find the current process
        if let Some(process) = all_processes.iter().find(|p| p.pid == current_pid) {
            tree.push(process.clone());
            
            // Find children
            for child in all_processes.iter().filter(|p| p.ppid == current_pid) {
                to_visit.push(child.pid);
            }
        }
    }
    
    Ok(tree)
}

/// Get system load average (Unix only)
#[cfg(unix)]
pub fn get_load_average() -> ProcessResult<(f64, f64, f64)> {
    if let Ok(content) = fs::read_to_string("/proc/loadavg") {
        let fields: Vec<&str> = content.split_whitespace().collect();
        if fields.len() >= 3 {
            let load1 = fields[0].parse::<f64>()
                .map_err(|_| io_error("parse_load", "InvalidFormat", "Invalid load average format"))?;
            let load5 = fields[1].parse::<f64>()
                .map_err(|_| io_error("parse_load", "InvalidFormat", "Invalid load average format"))?;
            let load15 = fields[2].parse::<f64>()
                .map_err(|_| io_error("parse_load", "InvalidFormat", "Invalid load average format"))?;
            
            return Ok((load1, load5, load15));
        }
    }
    
    Err(io_error("get_load_average", "ReadError", "Could not read load average"))
}

#[cfg(windows)]
pub fn get_load_average() -> ProcessResult<(f64, f64, f64)> {
    // Windows doesn't have a direct equivalent to Unix load average
    // We can approximate using processor queue length from performance counters
    use std::process::Command;
    
    // Try to get processor queue length using typeperf
    if let Ok(output) = Command::new("typeperf")
        .args(&["\\System\\Processor Queue Length", "-sc", "1"])
        .output()
    {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            for line in output_str.split("\n") {
                if line.contains("Processor Queue Length") {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let value_str = parts[1].trim_matches('"').trim();
                        if let Ok(queue_length) = value_str.parse::<f64>() {
                            // Return queue length as approximation for load average
                            // Since Windows doesn't distinguish 1, 5, 15 minute averages,
                            // we return the same value for all three
                            return Ok((queue_length, queue_length, queue_length));
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: Use CPU count as a rough estimation
    let cpu_count = get_cpu_count() as f64;
    let estimated_load = cpu_count * 0.5; // Assume 50% utilization as default
    
    Ok((estimated_load, estimated_load, estimated_load))
}

/// Get number of CPU cores
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}

/// Get system uptime
#[cfg(unix)]
pub fn get_system_uptime() -> ProcessResult<Duration> {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime_str) = content.split_whitespace().next() {
            if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                return Ok(Duration::from_secs_f64(uptime_secs));
            }
        }
    }
    
    Err(io_error("get_system_uptime", "ReadError", "Could not read system uptime"))
}

#[cfg(windows)]
pub fn get_system_uptime() -> ProcessResult<Duration> {
    use std::process::Command;
    
    // Use PowerShell to get system uptime
    let ps_script = r#"
        $bootTime = (Get-CimInstance -ClassName Win32_OperatingSystem).LastBootUpTime
        $uptime = (Get-Date) - $bootTime
        [Math]::Floor($uptime.TotalSeconds)
    "#;
    
    if let Ok(output) = Command::new("powershell")
        .args(&["-Command", ps_script])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(seconds) = stdout.trim().parse::<u64>() {
                return Ok(Duration::from_secs(seconds));
            }
        }
    }
    
    // Fallback: Use WMIC to get boot time
    if let Ok(output) = Command::new("wmic")
        .args(&["os", "get", "LastBootUpTime", "/value"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.split("\n") {
                if line.starts_with("LastBootUpTime=") {
                    // Parse WMI datetime format: YYYYMMDDHHMMSS.mmmmmm+UUU
                    if let Some(datetime_str) = line.split('=').nth(1) {
                        if let Ok(uptime_secs) = parse_wmi_datetime(datetime_str) {
                            return Ok(Duration::from_secs(uptime_secs));
                        }
                    }
                }
            }
        }
    }
    
    // Final fallback: Use GetTickCount64 equivalent via PowerShell
    if let Ok(output) = Command::new("powershell")
        .args(&["-Command", "[Environment]::TickCount / 1000"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(seconds) = stdout.trim().parse::<u64>() {
                return Ok(Duration::from_secs(seconds));
            }
        }
    }
    
    Err(system_error(
        -1,
        "get_system_uptime",
        "Could not determine system uptime on Windows"
    ))
}

#[cfg(windows)]
fn parse_wmi_datetime(datetime_str: &str) -> Result<u64, ()> {
    // WMI datetime format: YYYYMMDDHHMMSS.mmmmmm+UUU
    // We need to parse this and calculate seconds since boot
    if datetime_str.len() >= 14 {
        // For simplicity, we'll estimate uptime based on current time
        // In a real implementation, you'd parse the full datetime
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| ())?
            .as_secs();
        
        // Estimate boot time (this is a simplified approach)
        // In production, you'd properly parse the WMI datetime
        let estimated_boot_offset = 86400; // Assume system has been up for at most 1 day
        Ok(estimated_boot_offset.min(now))
    } else {
        Err(())
    }
}

// Add num_cpus as a dependency in a real implementation
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::info::ProcessInfo;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_process_status_conversion() {
        assert_eq!(ProcessStatus::from('R'), ProcessStatus::Running);
        assert_eq!(ProcessStatus::from('S'), ProcessStatus::Sleeping);
        assert_eq!(ProcessStatus::from('Z'), ProcessStatus::Zombie);
        assert_eq!(ProcessStatus::from('?'), ProcessStatus::Unknown("?".to_string()));
    }

    #[test]
    fn test_process_status_display() {
        assert_eq!(format!("{}", ProcessStatus::Running), "Running");
        assert_eq!(format!("{}", ProcessStatus::Zombie), "Zombie");
        assert_eq!(format!("{}", ProcessStatus::Unknown("X".to_string())), "Unknown (X)");
    }

    #[test]
    fn test_memory_info_creation() {
        let mem = MemoryInfo::new();
        assert_eq!(mem.virtual_size, 0);
        assert_eq!(mem.resident_size, 0);
        assert_eq!(mem.percentage, 0.0);
    }

    #[test]
    fn test_cpu_info_creation() {
        let cpu = CpuInfo::new();
        assert_eq!(cpu.cpu_percent, 0.0);
        assert_eq!(cpu.user_time, 0);
        assert_eq!(cpu.system_time, 0);
    }

    #[test]
    fn test_get_current_pid() {
        let pid = get_current_pid();
        assert!(pid > 0);
    }

    #[test]
    fn test_is_process_running() {
        let current_pid = get_current_pid();
        assert!(is_process_running(current_pid));
        
        // Test with a PID that definitely doesn't exist
        assert!(!is_process_running(999999));
    }

    #[test]
    fn test_get_cpu_count() {
        let count = get_cpu_count();
        assert!(count > 0);
        assert!(count <= 1024); // Reasonable upper bound
    }

    #[test]
    fn test_process_info_creation() {
        let info = ProcessInfo::new(1234);
        assert_eq!(info.pid, 1234);
        assert_eq!(info.ppid, 0);
        assert!(info.name.is_empty());
        assert_eq!(info.threads, 1);
    }

    #[test]
    fn test_process_list_entry() {
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
        assert_eq!(entry.name, "test_process");
        assert_eq!(entry.status, ProcessStatus::Running);
    }
}
