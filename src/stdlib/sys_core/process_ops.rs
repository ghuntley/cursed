use crate::error::CursedError;
/// Low-level process management operations
use std::ffi::CString;
// use crate::stdlib::sys_core::error::{SysCoreError, SysCoreResult, system_call_error, not_supported};

/// Process identifier
pub type ProcessId = u32;

/// Thread identifier  
pub type ThreadId = u64;

/// Process group identifier
pub type ProcessGroup = u32;

/// Session identifier
pub type SessionId = u32;

/// Fork a new process (Unix only)
pub fn fork_process() -> SysCoreResult<ForkResult> {
    #[cfg(unix)]
    {
        let pid = unsafe { libc::fork() };
        match pid {
            -1 => {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                Err(system_call_error("fork", errno))
            }
        }
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("fork not supported on this platform"))
    }
}

/// Fork result
#[derive(Debug, Clone, PartialEq)]
pub enum ForkResult {
/// Execute a program (Unix only)
pub fn exec_process<P, A, S>(program: P, args: A) -> SysCoreResult<()>
where
{
    #[cfg(unix)]
    {
        let program_cstr = CString::new(program.as_ref())
            .map_err(|_| SysCoreError::InvalidArgument("Invalid program path".to_string()))?;
        
        let args_cstr: Result<Vec<CString>, _> = args.into_iter()
            .map(|arg| CString::new(arg.as_ref()))
            .collect();
        
        let args_cstr = args_cstr
            .map_err(|_| SysCoreError::InvalidArgument("Invalid argument".to_string()))?;
        
        let mut args_ptr: Vec<*const i8> = args_cstr.iter()
            .map(|cstr| cstr.as_ptr())
            .collect();
        args_ptr.push(std::ptr::null());
        
        unsafe {
            libc::execvp(program_cstr.as_ptr(), args_ptr.as_ptr());
        // If execvp returns, it means it failed
        let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
        Err(system_call_error("execvp", errno))
    #[cfg(not(unix))]
    {
        Err(not_supported("exec not supported on this platform"))
    }
}

/// Wait for a child process
pub fn wait_process(pid: Option<ProcessId>) -> SysCoreResult<WaitResult> {
    #[cfg(unix)]
    {
        let mut status: i32 = 0;
        let wait_pid = match pid {
            None => -1, // Wait for any child
        
        let result_pid = unsafe { libc::waitpid(wait_pid, &mut status, 0) };
        
        if result_pid == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("waitpid", errno));
        let exit_status = if unsafe { libc::WIFEXITED(status) } {
            ExitStatus::Exited(unsafe { libc::WEXITSTATUS(status) })
        } else if unsafe { libc::WIFSIGNALED(status) } {
            ExitStatus::Signaled(unsafe { libc::WTERMSIG(status) })
        } else {
            ExitStatus::Other(status)
        
        Ok(WaitResult {
        })
    #[cfg(not(unix))]
    {
        Err(not_supported("wait not supported on this platform"))
    }
}

/// Wait result
#[derive(Debug, Clone)]
pub struct WaitResult {
/// Process exit status
#[derive(Debug, Clone)]
pub enum ExitStatus {
/// Send a signal to a process
pub fn kill_process(pid: ProcessId, signal: i32) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::kill(pid as i32, signal) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("kill", errno));
        }
        Ok(())
    #[cfg(not(unix))]
    {
        Err(not_supported("kill not supported on this platform"))
    }
}

/// Set process group ID
pub fn setpgid(pid: ProcessId, pgid: ProcessGroup) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::setpgid(pid as i32, pgid as i32) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("setpgid", errno));
        }
        Ok(())
    #[cfg(not(unix))]
    {
        Err(not_supported("setpgid not supported on this platform"))
    }
}

/// Create a new session
pub fn setsid() -> SysCoreResult<SessionId> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::setsid() };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("setsid", errno));
        }
        Ok(result as SessionId)
    #[cfg(not(unix))]
    {
        Err(not_supported("setsid not supported on this platform"))
    }
}

/// Get current process ID
pub fn getpid() -> ProcessId {
    #[cfg(unix)]
    {
        unsafe { libc::getpid() as ProcessId }
    }
    
    #[cfg(not(unix))]
    {
        std::process::id()
    }
}

/// Get parent process ID
pub fn getppid() -> SysCoreResult<ProcessId> {
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getppid() as ProcessId })
    #[cfg(not(unix))]
    {
        Err(not_supported("getppid not supported on this platform"))
    }
}

/// Get process group ID
pub fn getpgid(pid: ProcessId) -> SysCoreResult<ProcessGroup> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::getpgid(pid as i32) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("getpgid", errno));
        }
        Ok(result as ProcessGroup)
    #[cfg(not(unix))]
    {
        Err(not_supported("getpgid not supported on this platform"))
    }
}

/// Get session ID
pub fn getsid(pid: ProcessId) -> SysCoreResult<SessionId> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::getsid(pid as i32) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("getsid", errno));
        }
        Ok(result as SessionId)
    #[cfg(not(unix))]
    {
        Err(not_supported("getsid not supported on this platform"))
    }
}

/// Check if fork is supported on this platform
pub fn supports_fork() -> bool {
    cfg!(unix)
/// Get current thread ID
pub fn get_current_thread_id() -> ThreadId {
    #[cfg(target_os = "linux")]
    {
        unsafe { libc::syscall(libc::SYS_gettid) as ThreadId }
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        std::thread::current().id().as_u64().get()
    }
}

/// Thread creation and management
pub struct ThreadInfo {
/// Get information about the current thread
pub fn get_current_thread_info() -> ThreadInfo {
    let current = std::thread::current();
    ThreadInfo {
        stack_size: None, // Would require platform-specific code to get actual stack size
    }
}

/// Process priority levels
#[derive(Debug, Clone, Copy)]
pub enum ProcessPriority {
/// Set process priority (nice value on Unix)
pub fn set_process_priority(pid: ProcessId, priority: ProcessPriority) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let nice_value = priority as i32;
        let result = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid, nice_value) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("setpriority", errno));
        }
        Ok(())
    #[cfg(not(unix))]
    {
        Err(not_supported("setpriority not supported on this platform"))
    }
}

/// Get process priority (nice value on Unix)
pub fn get_process_priority(pid: ProcessId) -> SysCoreResult<i32> {
    #[cfg(unix)]
    {
        // Reset errno before calling getpriority (not needed with std::io::Error)
        // errno cleared automatically when using std::io::Error::last_os_error()
        
        let result = unsafe { libc::getpriority(libc::PRIO_PROCESS, pid) };
        let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
        
        if errno != 0 {
            return Err(system_call_error("getpriority", errno));
        Ok(result)
    #[cfg(not(unix))]
    {
        Err(not_supported("getpriority not supported on this platform"))
    }
}
