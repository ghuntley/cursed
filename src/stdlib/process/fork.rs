use crate::error::CursedError;
/// Process forking and job control implementation for CURSED
/// 
/// Process forking is a fundamental system programming primitive that allows:
/// - Creating child processes for parallel execution
/// - Building process pipelines and complex workflows
/// - Implementing server architectures with worker processes
/// - Creating isolated execution environments
/// - Building shell-like process management tools
/// 
/// This module provides safe, cross-platform process forking capabilities
/// with proper error handling and resource management.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// use crate::stdlib::process::error::{
    ProcessResult, ProcessError, process_not_found_pid, permission_denied_pid,
    invalid_state, execution_failed, timeout_error, system_error
};

// use crate::stdlib::process::core::{ProcessConfig, ProcessInfo, ProcessState};

/// Fork manager alias for process forking
pub type ForkManager = JobControlManager;

/// Fork options configuration
#[derive(Debug, Clone)]
pub struct ForkOptions {
    /// Create new process group
    pub new_process_group: bool,
    /// Create new session
    pub new_session: bool,
    /// Process group ID to join
    pub process_group_id: Option<u32>,
    /// Controlling terminal
    pub controlling_terminal: Option<String>,
}

/// Fork result containing information about the forked process
#[derive(Debug, Clone)]
pub struct ForkResult {
    /// Process ID of the child (from parent's perspective)
    pub child_pid: Option<u32>,
    /// Process ID of the parent (from child's perspective)  
    pub parent_pid: Option<u32>,
    /// Whether this is the parent or child process
    pub is_parent: bool,
    /// Fork creation timestamp
    pub fork_time: Instant,
}

/// Process group management
#[derive(Debug, Clone)]
pub struct ProcessGroup {
    /// Process group ID
    pub pgid: u32,
    /// Session ID
    pub sid: Option<u32>,
    /// Leader process ID
    pub leader_pid: u32,
    /// Member processes
    pub members: Vec<u32>,
    /// Creation time
    pub created_at: Instant,
    /// Controlling terminal
    pub controlling_terminal: Option<String>,
}

/// Session information
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Session ID
    pub sid: u32,
    /// Session leader PID
    pub leader_pid: u32,
    /// Process groups in this session
    pub process_groups: Vec<u32>,
    /// Controlling terminal
    pub controlling_terminal: Option<String>,
    /// Creation time
    pub created_at: Instant,
}

/// Job control manager for managing process groups and sessions
pub struct JobControlManager {
    /// Active process groups
    process_groups: Arc<Mutex<HashMap<u32, ProcessGroup>>>,
    /// Active sessions
    sessions: Arc<Mutex<HashMap<u32, SessionInfo>>>,
    /// Process to group mapping
    process_to_group: Arc<Mutex<HashMap<u32, u32>>>,
    /// Group to session mapping
    group_to_session: Arc<Mutex<HashMap<u32, u32>>>,
}

impl JobControlManager {
    /// Create a new job control manager
    pub fn new() -> Self {
        Self {
            process_groups: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            process_to_group: Arc::new(Mutex::new(HashMap::new())),
            group_to_session: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new process group
    pub fn create_process_group(&self, leader_pid: u32, pgid: Option<u32>) -> ProcessResult<u32> {
        let pgid = pgid.unwrap_or(leader_pid);
        
        #[cfg(unix)]
        {
            let result = unsafe { libc::setpgid(leader_pid as libc::pid_t, pgid as libc::pid_t) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "create_process_group",
                    &format!("Failed to create process group {}", pgid)
                ));
            }
        }

        let group = ProcessGroup {
            pgid,
            sid: None,
            leader_pid,
            members: vec![leader_pid],
            created_at: Instant::now(),
            controlling_terminal: None,
        };

        {
            let mut groups = self.process_groups.lock()
                .map_err(|_| system_error(-1, "create_process_group", "Failed to lock process groups"))?;
            groups.insert(pgid, group);
        }

        {
            let mut mapping = self.process_to_group.lock()
                .map_err(|_| system_error(-1, "create_process_group", "Failed to lock process mapping"))?;
            mapping.insert(leader_pid, pgid);
        }

        Ok(pgid)
    }

    /// Add process to existing group
    pub fn add_to_group(&self, pid: u32, pgid: u32) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            let result = unsafe { libc::setpgid(pid as libc::pid_t, pgid as libc::pid_t) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "add_to_group",
                    &format!("Failed to add process {} to group {}", pid, pgid)
                ));
            }
        }

        {
            let mut groups = self.process_groups.lock()
                .map_err(|_| system_error(-1, "add_to_group", "Failed to lock process groups"))?;
            if let Some(group) = groups.get_mut(&pgid) {
                if !group.members.contains(&pid) {
                    group.members.push(pid);
                }
            } else {
                return Err(process_not_found_pid(pgid, "Process group not found"));
            }
        }

        {
            let mut mapping = self.process_to_group.lock()
                .map_err(|_| system_error(-1, "add_to_group", "Failed to lock process mapping"))?;
            mapping.insert(pid, pgid);
        }

        Ok(())
    }

    /// Create a new session
    pub fn create_session(&self, leader_pid: u32) -> ProcessResult<u32> {
        #[cfg(unix)]
        {
            let sid = unsafe { libc::setsid() };
            if sid == -1 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "create_session",
                    "Failed to create new session"
                ));
            }

            let session = SessionInfo {
                sid: sid as u32,
                leader_pid,
                process_groups: vec![leader_pid], // Leader becomes its own process group
                controlling_terminal: None,
                created_at: Instant::now(),
            };

            {
                let mut sessions = self.sessions.lock()
                    .map_err(|_| system_error(-1, "create_session", "Failed to lock sessions"))?;
                sessions.insert(sid as u32, session);
            }

            // Update process group for session leader
            self.create_process_group(leader_pid, Some(leader_pid))?;
            
            {
                let mut group_mapping = self.group_to_session.lock()
                    .map_err(|_| system_error(-1, "create_session", "Failed to lock group mapping"))?;
                group_mapping.insert(leader_pid, sid as u32);
            }

            Ok(sid as u32)
        }

        #[cfg(windows)]
        {
            // Windows doesn't have sessions in the Unix sense
            // We'll simulate it with a unique session ID
            use std::sync::atomic::{AtomicU32, Ordering};
            static NEXT_SID: AtomicU32 = AtomicU32::new(1);
            let sid = NEXT_SID.fetch_add(1, Ordering::SeqCst);

            let session = SessionInfo {
                sid,
                leader_pid,
                process_groups: vec![leader_pid],
                controlling_terminal: None,
                created_at: Instant::now(),
            };

            {
                let mut sessions = self.sessions.lock()
                    .map_err(|_| system_error(-1, "create_session", "Failed to lock sessions"))?;
                sessions.insert(sid, session);
            }

            Ok(sid)
        }

        #[cfg(not(any(unix, windows)))]
        {
            Err(system_error(-1, "create_session", "Session creation not supported on this platform"))
        }
    }

    /// Get process group information
    pub fn get_process_group(&self, pgid: u32) -> ProcessResult<ProcessGroup> {
        let groups = self.process_groups.lock()
            .map_err(|_| system_error(-1, "get_process_group", "Failed to lock process groups"))?;
        
        groups.get(&pgid).cloned()
            .ok_or_else(|| process_not_found_pid(pgid, "Process group not found"))
    }

    /// Get session information
    pub fn get_session(&self, sid: u32) -> ProcessResult<SessionInfo> {
        let sessions = self.sessions.lock()
            .map_err(|_| system_error(-1, "get_session", "Failed to lock sessions"))?;
        
        sessions.get(&sid).cloned()
            .ok_or_else(|| process_not_found_pid(sid, "Session not found"))
    }

    /// Get process group ID for a process
    pub fn get_process_group_id(&self, pid: u32) -> ProcessResult<u32> {
        #[cfg(unix)]
        {
            let pgid = unsafe { libc::getpgid(pid as libc::pid_t) };
            if pgid == -1 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "get_process_group_id",
                    &format!("Failed to get process group for PID {}", pid)
                ));
            }
            Ok(pgid as u32)
        }

        #[cfg(not(unix))]
        {
            let mapping = self.process_to_group.lock()
                .map_err(|_| system_error(-1, "get_process_group_id", "Failed to lock process mapping"))?;
            
            mapping.get(&pid).cloned()
                .ok_or_else(|| process_not_found_pid(pid, "Process not in any group"))
        }
    }

    /// Send signal to process group
    pub fn signal_process_group(&self, pgid: u32, signal: i32) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            let result = unsafe { libc::killpg(pgid as libc::pid_t, signal) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "signal_process_group",
                    &format!("Failed to send signal {} to process group {}", signal, pgid)
                ));
            }
            Ok(())
        }

        #[cfg(windows)]
        {
            // Windows doesn't have process groups in the Unix sense
            // We'll send the signal to all processes in our simulated group
            let members = {
                let groups = self.process_groups.lock()
                    .map_err(|_| system_error(-1, "signal_process_group", "Failed to lock process groups"))?;
                
                groups.get(&pgid)
                    .map(|group| group.members.clone())
                    .ok_or_else(|| process_not_found_pid(pgid, "Process group not found"))?
            };

            for pid in members {
//                 let _ = crate::stdlib::process::control::send_signal_to_pid(pid, signal);
            }
            
            Ok(())
        }

        #[cfg(not(any(unix, windows)))]
        {
            Err(system_error(-1, "signal_process_group", "Process group signaling not supported on this platform"))
        }
    }

    /// Remove process from group
    pub fn remove_from_group(&self, pid: u32) -> ProcessResult<()> {
        let pgid = {
            let mut mapping = self.process_to_group.lock()
                .map_err(|_| system_error(-1, "remove_from_group", "Failed to lock process mapping"))?;
            mapping.remove(&pid)
        };

        if let Some(pgid) = pgid {
            let mut groups = self.process_groups.lock()
                .map_err(|_| system_error(-1, "remove_from_group", "Failed to lock process groups"))?;
            
            if let Some(group) = groups.get_mut(&pgid) {
                group.members.retain(|&member_pid| member_pid != pid);
                
                // If group is empty, remove it
                if group.members.is_empty() {
                    groups.remove(&pgid);
                }
            }
        }

        Ok(())
    }

    /// List all process groups
    pub fn list_process_groups(&self) -> ProcessResult<Vec<ProcessGroup>> {
        let groups = self.process_groups.lock()
            .map_err(|_| system_error(-1, "list_process_groups", "Failed to lock process groups"))?;
        
        Ok(groups.values().cloned().collect())
    }

    /// List all sessions
    pub fn list_sessions(&self) -> ProcessResult<Vec<SessionInfo>> {
        let sessions = self.sessions.lock()
            .map_err(|_| system_error(-1, "list_sessions", "Failed to lock sessions"))?;
        
        Ok(sessions.values().cloned().collect())
    }
}

/// Fork a new process
/// 
/// This is the fundamental process creation primitive in Unix systems.
/// Returns information about the fork result.
pub fn fork_process() -> ProcessResult<ForkResult> {
    #[cfg(unix)]
    {
        let pid = unsafe { libc::fork() };
        
        match pid {
            -1 => Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "fork_process",
                "Fork failed"
            )),
            0 => {
                // Child process
                Ok(ForkResult {
                    child_pid: None,
                    parent_pid: Some(unsafe { libc::getppid() as u32 }),
                    is_parent: false,
                    fork_time: Instant::now(),
                })
            }
            child_pid => {
                // Parent process
                Ok(ForkResult {
                    child_pid: Some(child_pid as u32),
                    parent_pid: None,
                    is_parent: true,
                    fork_time: Instant::now(),
                })
            }
        }
    }

    #[cfg(windows)]
    {
        // Windows doesn't have fork(), but we can simulate it using CreateProcess
        // This is a simplified simulation - real fork() semantics are complex
        Err(system_error(-1, "fork_process", "Fork not supported on Windows - use spawn_process instead"))
    }

    #[cfg(not(any(unix, windows)))]
    {
        Err(system_error(-1, "fork_process", "Fork not supported on this platform"))
    }
}

/// Execute a new program in the current process (exec family)
pub fn exec_program<S: AsRef<str>>(program: S, args: &[S], env: Option<&[(S, S)]>) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        use std::ffi::CString;
        
        // Convert program name to CString
        let program_cstr = CString::new(program.as_ref().as_bytes())
            .map_err(|_| invalid_state("exec_program", "program", "Invalid program name"))?;
        
        // Convert arguments to CString vector
        let mut arg_cstrs = Vec::new();
        arg_cstrs.push(program_cstr.clone()); // argv[0] is traditionally the program name
        
        for arg in args {
            let arg_cstr = CString::new(arg.as_ref().as_bytes())
                .map_err(|_| invalid_state("exec_program", "args", "Invalid argument"))?;
            arg_cstrs.push(arg_cstr);
        }
        
        // Create null-terminated array of pointers
        let mut argv: Vec<*const libc::c_char> = arg_cstrs.iter().map(|s| s.as_ptr()).collect();
        argv.push(std::ptr::null());
        
        // Handle environment variables
        if let Some(env_vars) = env {
            let mut env_cstrs = Vec::new();
            for (key, value) in env_vars {
                let env_str = format!("{}={}", key.as_ref(), value.as_ref());
                let env_cstr = CString::new(env_str.as_bytes())
                    .map_err(|_| invalid_state("exec_program", "env", "Invalid environment variable"))?;
                env_cstrs.push(env_cstr);
            }
            
            let mut envp: Vec<*const libc::c_char> = env_cstrs.iter().map(|s| s.as_ptr()).collect();
            envp.push(std::ptr::null());
            
            unsafe { libc::execve(program_cstr.as_ptr(), argv.as_ptr(), envp.as_ptr()) };
        } else {
            unsafe { libc::execv(program_cstr.as_ptr(), argv.as_ptr()) };
        }
        
        // If we reach here, exec failed
        Err(system_error(
            std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
            "exec_program",
            &format!("Failed to exec program: {}", program.as_ref())
        ))
    }

    #[cfg(not(unix))]
    {
        Err(system_error(-1, "exec_program", "exec not supported on this platform"))
    }
}

/// Wait for child processes
pub fn wait_for_child(pid: Option<u32>, options: WaitOptions) -> ProcessResult<WaitResult> {
    #[cfg(unix)]
    {
        let mut status = 0;
        let wait_pid = pid.map(|p| p as libc::pid_t).unwrap_or(-1);
        let wait_options = options.to_libc_options();
        
        let result_pid = unsafe { libc::waitpid(wait_pid, &mut status, wait_options) };
        
        match result_pid {
            -1 => Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "wait_for_child",
                "Wait failed"
            )),
            0 => Ok(WaitResult {
                pid: None,
                status: WaitStatus::StillRunning,
            }),
            child_pid => {
                let wait_status = if libc::WIFEXITED(status) {
                    WaitStatus::Exited(libc::WEXITSTATUS(status))
                } else if libc::WIFSIGNALED(status) {
                    WaitStatus::Signaled(libc::WTERMSIG(status))
                } else if libc::WIFSTOPPED(status) {
                    WaitStatus::Stopped(libc::WSTOPSIG(status))
                } else if libc::WIFCONTINUED(status) {
                    WaitStatus::Continued
                } else {
                    WaitStatus::Unknown(status)
                };
                
                Ok(WaitResult {
                    pid: Some(child_pid as u32),
                    status: wait_status,
                })
            }
        }
    }

    #[cfg(not(unix))]
    {
        Err(system_error(-1, "wait_for_child", "wait not supported on this platform"))
    }
}

/// Wait options for child processes
#[derive(Debug, Clone, Copy)]
pub struct WaitOptions {
    /// Don't block if no child has exited
    pub no_hang: bool,
    /// Report stopped children
    pub untraced: bool,
    /// Report continued children
    pub continued: bool,
}

impl WaitOptions {
    pub fn new() -> Self {
        Self {
            no_hang: false,
            untraced: false,
            continued: false,
        }
    }
    
    pub fn no_hang(mut self) -> Self {
        self.no_hang = true;
        self
    }
    
    pub fn untraced(mut self) -> Self {
        self.untraced = true;
        self
    }
    
    pub fn continued(mut self) -> Self {
        self.continued = true;
        self
    }

    #[cfg(unix)]
    fn to_libc_options(&self) -> libc::c_int {
        let mut options = 0;
        
        if self.no_hang {
            options |= libc::WNOHANG;
        }
        if self.untraced {
            options |= libc::WUNTRACED;
        }
        if self.continued {
            options |= libc::WCONTINUED;
        }
        
        options
    }
}

/// Result of waiting for a child process
#[derive(Debug, Clone)]
pub struct WaitResult {
    /// PID of the child that changed state (None if no change)
    pub pid: Option<u32>,
    /// Status of the child process
    pub status: WaitStatus,
}

/// Status of a waited-for child process
#[derive(Debug, Clone)]
pub enum WaitStatus {
    /// Process exited normally with exit code
    Exited(i32),
    /// Process was terminated by signal
    Signaled(i32),
    /// Process was stopped by signal
    Stopped(i32),
    /// Process was continued after being stopped
    Continued,
    /// Process is still running (WNOHANG was used)
    StillRunning,
    /// Unknown status
    Unknown(i32),
}

/// Fork and execute a program in one step
pub fn fork_exec<S: AsRef<str>>(
    program: S, 
    args: &[S], 
    env: Option<&[(S, S)]>
) -> ProcessResult<u32> {
    let fork_result = fork_process()?;
    
    if fork_result.is_parent {
        // Parent process - return child PID
        Ok(fork_result.child_pid.unwrap())
    } else {
        // Child process - exec the program
        if let Err(e) = exec_program(program, args, env) {
            eprintln!("Child exec failed: {}", e);
            std::process::exit(1);
        }
        
        // This line should never be reached
        std::process::exit(0);
    }
}

/// Create a daemon process using double fork
pub fn daemonize() -> ProcessResult<()> {
    #[cfg(unix)]
    {
        // First fork
        let first_fork = fork_process()?;
        if first_fork.is_parent {
            // Parent exits, leaving child as orphan
            std::process::exit(0);
        }
        
        // Child becomes session leader
        unsafe { libc::setsid() };
        
        // Second fork to ensure we're not session leader
        let second_fork = fork_process()?;
        if second_fork.is_parent {
            // First child exits
            std::process::exit(0);
        }
        
        // Grandchild continues as daemon
        // Change to root directory
        let root_cstr = std::ffi::CString::new("/").unwrap();
        unsafe { libc::chdir(root_cstr.as_ptr()) };
        
        // Set file mode creation mask
        unsafe { libc::umask(0) };
        
        // Close file descriptors
        unsafe {
            libc::close(0); // stdin
            libc::close(1); // stdout  
            libc::close(2); // stderr
        }
        
        // Redirect to /dev/null
        let dev_null = std::ffi::CString::new("/dev/null").unwrap();
        unsafe {
            let null_fd = libc::open(dev_null.as_ptr(), libc::O_RDWR);
            if null_fd != -1 {
                libc::dup2(null_fd, 0);
                libc::dup2(null_fd, 1);
                libc::dup2(null_fd, 2);
                if null_fd > 2 {
                    libc::close(null_fd);
                }
            }
        }
        
        Ok(())
    }

    #[cfg(not(unix))]
    {
        Err(system_error(-1, "daemonize", "Daemonize not supported on this platform"))
    }
}

