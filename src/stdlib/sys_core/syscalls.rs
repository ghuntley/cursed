/// Direct system call interface
use crate::stdlib::sys_core::error::{SysCoreResult, system_call_error, not_supported, invalid_argument};

/// System call identifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemCall {
    Read,
    Write,
    Open,
    Close,
    Fork,
    Execve,
    Wait,
    Kill,
    Mmap,
    Munmap,
    Custom(u64),
}

/// System call result
#[derive(Debug, Clone)]
pub enum SystemCallResult {
    Success(i64),
    Error(i32),
}

/// Direct system call interface (platform-specific)
pub fn direct_syscall(call: SystemCall, args: &[u64]) -> SysCoreResult<i64> {
    #[cfg(target_arch = "x86_64")]
    #[cfg(target_os = "linux")]
    {
        let syscall_number = match call {
            SystemCall::Read => 0,
            SystemCall::Write => 1,
            SystemCall::Open => 2,
            SystemCall::Close => 3,
            SystemCall::Fork => 57,
            SystemCall::Execve => 59,
            SystemCall::Wait => 61,
            SystemCall::Kill => 62,
            SystemCall::Mmap => 9,
            SystemCall::Munmap => 11,
            SystemCall::Custom(num) => num,
        };
        
        let result = match args.len() {
            0 => unsafe { syscall0(syscall_number) },
            1 => unsafe { syscall1(syscall_number, args[0]) },
            2 => unsafe { syscall2(syscall_number, args[0], args[1]) },
            3 => unsafe { syscall3(syscall_number, args[0], args[1], args[2]) },
            4 => unsafe { syscall4(syscall_number, args[0], args[1], args[2], args[3]) },
            5 => unsafe { syscall5(syscall_number, args[0], args[1], args[2], args[3], args[4]) },
            6 => unsafe { syscall6(syscall_number, args[0], args[1], args[2], args[3], args[4], args[5]) },
            _ => return Err(invalid_argument("Too many syscall arguments")),
        };
        
        if result < 0 {
            Err(system_call_error("direct_syscall", (-result) as i32))
        } else {
            Ok(result)
        }
    }
    
    #[cfg(not(all(target_arch = "x86_64", target_os = "linux")))]
    {
        Err(not_supported("Direct syscalls not supported on this platform"))
    }
}

/// Safe system call wrapper that handles errors
pub fn safe_syscall(call: SystemCall, args: &[u64]) -> SystemCallResult {
    match direct_syscall(call, args) {
        Ok(result) => SystemCallResult::Success(result),
        Err(SysCoreError::SystemCall(_, errno)) => SystemCallResult::Error(errno),
        Err(_) => SystemCallResult::Error(-1),
    }
}

// Raw syscall implementations for x86_64 Linux
#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall0(number: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall1(number: u64, arg1: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        in("rdi") arg1,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall2(number: u64, arg1: u64, arg2: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall3(number: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall4(number: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall5(number: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_os = "linux")]
unsafe fn syscall6(number: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> i64 {
    let result: i64;
    std::arch::asm!(
        "syscall",
        in("rax") number,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        out("rax") result,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    result
}

/// System call utilities
pub mod utils {
    use super::*;
    
    /// Read from file descriptor using direct syscall
    pub fn syscall_read(fd: i32, buffer: &mut [u8]) -> SysCoreResult<usize> {
        let result = direct_syscall(
            SystemCall::Read,
            &[fd as u64, buffer.as_mut_ptr() as u64, buffer.len() as u64]
        )?;
        Ok(result as usize)
    }
    
    /// Write to file descriptor using direct syscall
    pub fn syscall_write(fd: i32, data: &[u8]) -> SysCoreResult<usize> {
        let result = direct_syscall(
            SystemCall::Write,
            &[fd as u64, data.as_ptr() as u64, data.len() as u64]
        )?;
        Ok(result as usize)
    }
    
    /// Open file using direct syscall
    pub fn syscall_open(path: &str, flags: i32, mode: u32) -> SysCoreResult<i32> {
        use std::ffi::CString;
        
        let path_cstr = CString::new(path)
            .map_err(|_| invalid_argument("Invalid path"))?;
        
        let result = direct_syscall(
            SystemCall::Open,
            &[path_cstr.as_ptr() as u64, flags as u64, mode as u64]
        )?;
        Ok(result as i32)
    }
    
    /// Close file descriptor using direct syscall
    pub fn syscall_close(fd: i32) -> SysCoreResult<()> {
        direct_syscall(SystemCall::Close, &[fd as u64])?;
        Ok(())
    }
    
    /// Get process ID using getpid syscall
    pub fn syscall_getpid() -> SysCoreResult<u32> {
        #[cfg(target_os = "linux")]
        {
            let result = direct_syscall(SystemCall::Custom(39), &[])?; // getpid
            Ok(result as u32)
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Err(not_supported("getpid syscall not supported on this platform"))
        }
    }
}

/// System call performance monitoring
pub struct SyscallProfiler {
    call_counts: std::collections::HashMap<SystemCall, u64>,
    total_time: std::time::Duration,
    start_time: Option<std::time::Instant>,
}

impl SyscallProfiler {
    pub fn new() -> Self {
        Self {
            call_counts: std::collections::HashMap::new(),
            total_time: std::time::Duration::ZERO,
            start_time: None,
        }
    }
    
    pub fn start_call(&mut self, call: SystemCall) {
        *self.call_counts.entry(call).or_insert(0) += 1;
        self.start_time = Some(std::time::Instant::now());
    }
    
    pub fn end_call(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.total_time += start.elapsed();
        }
    }
    
    pub fn get_call_count(&self, call: SystemCall) -> u64 {
        self.call_counts.get(&call).copied().unwrap_or(0)
    }
    
    pub fn get_total_time(&self) -> std::time::Duration {
        self.total_time
    }
    
    pub fn reset(&mut self) {
        self.call_counts.clear();
        self.total_time = std::time::Duration::ZERO;
        self.start_time = None;
    }
}

use crate::stdlib::sys_core::error::SysCoreError;
