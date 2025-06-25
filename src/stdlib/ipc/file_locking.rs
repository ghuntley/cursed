use crate::error::CursedError;
/// File-based locking mechanisms for CURSED IPC
/// 
/// This module provides file locking capabilities for inter-process coordination
/// using advisory and mandatory locking mechanisms. File locks are portable
/// across Unix-like systems and provide a simple way to coordinate access to
/// shared resources.

use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use crate::stdlib::ipc::error::{IpcError, IpcResult};
// Placeholder imports disabled
    ResourceLimits
// };

// Placeholder imports disabled
    IpcResource, IpcFileLocking, ResourceState, LockType, LockInfo
// };

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};

/// Configuration for file lock creation
#[derive(Debug, Clone)]
pub struct LockConfig {
impl LockConfig {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            length: 0, // 0 means to end of file
        }
    }
    
    pub fn with_create(mut self, create: bool) -> Self {
        self.create_if_missing = create;
        self
    pub fn with_permissions(mut self, perms: u32) -> Self {
        self.permissions = perms;
        self
    pub fn with_shared_lock(mut self) -> Self {
        self.exclusive = false;
        self
    pub fn with_exclusive_lock(mut self) -> Self {
        self.exclusive = true;
        self
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    pub fn with_retry_interval(mut self, interval: Duration) -> Self {
        self.retry_interval = interval;
        self
    pub fn with_range(mut self, start: u64, length: u64) -> Self {
        self.lock_entire_file = false;
        self.start_offset = start;
        self.length = length;
        self
    }
}

/// Handle to a file lock resource
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LockHandle {
impl LockHandle {
    pub fn new(path: String, lock_type: Option<LockType>, start: u64, length: u64) -> Self {
        Self {
        }
    }
    
    pub fn is_exclusive(&self) -> bool {
        matches!(self.lock_type, Some(LockType::Exclusive))
    pub fn is_shared(&self) -> bool {
        matches!(self.lock_type, Some(LockType::Shared))
    pub fn covers_range(&self, start: u64, length: u64) -> bool {
        if self.length == 0 {
            // Lock covers entire file
            return true;
        let self_end = self.start_offset + self.length;
        let range_end = start + length;
        
        self.start_offset <= start && range_end <= self_end
    }
}

/// Statistics for file locking operations
#[derive(Debug, Clone, Default)]
pub struct LockStatistics {
impl LockStatistics {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    pub fn record_lock_acquired(&mut self, exclusive: bool, is_range: bool, duration: Duration) {
        self.locks_acquired += 1;
        self.current_locks += 1;
        
        if self.current_locks > self.max_concurrent_locks {
            self.max_concurrent_locks = self.current_locks;
        if exclusive {
            self.exclusive_locks += 1;
        } else {
            self.shared_locks += 1;
        if is_range {
            self.range_locks += 1;
        } else {
            self.file_locks += 1;
        self.total_lock_time += duration;
        if self.locks_acquired > 0 {
            self.average_lock_duration = self.total_lock_time / self.locks_acquired as u32;
        self.base.record_operation(true, duration);
    pub fn record_lock_released(&mut self) {
        self.locks_released += 1;
        self.current_locks = self.current_locks.saturating_sub(1);
    pub fn record_contention(&mut self) {
        self.lock_contentions += 1;
    pub fn record_timeout(&mut self) {
        self.timeouts += 1;
        self.base.record_error("timeout");
    }
}

/// File locking implementation
pub struct FileLock {
    #[cfg(unix)]
impl FileLock {
    /// Create a new file lock
    pub fn create<P: AsRef<Path>>(path: P) -> IpcResult<Self> {
        let config = LockConfig::new(path);
        Self::create_with_config(config)
    /// Create a file lock with custom configuration
    pub fn create_with_config(config: LockConfig) -> IpcResult<Self> {
        let path_str = config.path.to_string_lossy().to_string();
        let handle = IpcHandle::new(path_str.clone(), "file_lock".to_string());
        let address = IpcAddress::path(path_str.clone());
        let permissions = IpcPermissions::new(config.permissions);
        
        let mut file_lock = Self {
            #[cfg(unix)]
        
        file_lock.initialize()?;
        Ok(file_lock)
    fn initialize(&mut self) -> IpcResult<()> {
        self.state = ResourceState::Initializing;
        
        let mut options = OpenOptions::new();
        options.read(true).write(true);
        
        if self.config.create_if_missing {
            options.create(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(self.config.permissions);
        let file = options.open(&self.config.path)
            .map_err(|e| IpcError::IoError(format!("Failed to open lock file: {}", e)))?;
        
        #[cfg(unix)]
        {
            self.fd = Some(file.as_raw_fd());
        self.file = Some(file);
        self.state = ResourceState::Ready;
        
        // Register with IPC system
        let lock_handle = LockHandle::new(
        );
//         crate::stdlib::ipc::register_file_lock(self.handle.id.clone(), lock_handle)?;
        
        Ok(())
    #[cfg(unix)]
    fn flock_operation(&self, operation: i32) -> IpcResult<()> {
        if let Some(fd) = self.fd {
            let result = unsafe { libc::flock(fd, operation) };
            if result == 0 {
                Ok(())
            } else {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                match errno {
                    libc::EWOULDBLOCK | libc::EAGAIN => {
                        Err(IpcError::ResourceExhausted("Lock would block".to_string()))
                    }
                    libc::EINTR => {
                        Err(IpcError::Timeout("Lock operation interrupted".to_string()))
                    }
                    _ => {
                        Err(IpcError::System(errno, format!("flock failed with errno {}", errno)))
                    }
                }
            }
        } else {
            Err(IpcError::InvalidOperation("File not open".to_string()))
        }
    }
    
    #[cfg(unix)]
    fn fcntl_lock(&self, lock_type: i16, start: u64, length: u64, wait: bool) -> IpcResult<()> {
        if let Some(fd) = self.fd {
            let mut flock = libc::flock {
            
            let cmd = if wait { libc::F_SETLKW } else { libc::F_SETLK };
            let result = unsafe { libc::fcntl(fd, cmd, &mut flock) };
            
            if result == 0 {
                Ok(())
            } else {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                match errno {
                    libc::EACCES | libc::EAGAIN => {
                        Err(IpcError::ResourceExhausted("Lock unavailable".to_string()))
                    }
                    libc::EINTR => {
                        Err(IpcError::Timeout("Lock operation interrupted".to_string()))
                    }
                    _ => {
                        Err(IpcError::System(errno, format!("fcntl lock failed with errno {}", errno)))
                    }
                }
            }
        } else {
            Err(IpcError::InvalidOperation("File not open".to_string()))
        }
    }
    
    #[cfg(windows)]
    fn windows_lock(&self, exclusive: bool, start: u64, length: u64, wait: bool) -> IpcResult<()> {
        use std::os::windows::io::AsRawHandle;
        use winapi::um::fileapi::{LockFileEx, UnlockFileEx};
        use winapi::um::minwinbase::{LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY};
        use winapi::um::winnt::HANDLE;
        
        if let Some(ref file) = self.file {
            let handle = file.as_raw_handle() as HANDLE;
            let mut flags = 0u32;
            
            if exclusive {
                flags |= LOCKFILE_EXCLUSIVE_LOCK;
            }
            if !wait {
                flags |= LOCKFILE_FAIL_IMMEDIATELY;
            let mut overlapped = std::mem::zeroed();
            let result = unsafe {
                LockFileEx(
                )
            
            if result != 0 {
                Ok(())
            } else {
                let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
                Err(IpcError::System(error as i32, format!("LockFileEx failed with error {}", error)))
            }
        } else {
            Err(IpcError::InvalidOperation("File not open".to_string()))
        }
    }
    
    fn acquire_lock_with_timeout(&mut self, exclusive: bool, timeout: IpcTimeout) -> IpcResult<()> {
        let start_time = Instant::now();
        self.state = ResourceState::Busy;
        
        loop {
            match self.try_acquire_lock(exclusive) {
                Ok(()) => {
                    let duration = start_time.elapsed();
                    if let Ok(mut stats) = self.statistics.lock() {
                        let is_range = !self.config.lock_entire_file;
                        stats.record_lock_acquired(exclusive, is_range, duration);
                    // Record the lock
                    let lock_info = LockInfo {
                    self.current_locks.push(lock_info);
                    
                    self.state = ResourceState::Ready;
                    return Ok(());
                }
                Err(IpcError::ResourceExhausted(_)) => {
                    // Lock contention, check timeout
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.record_contention();
                    match timeout {
                        IpcTimeout::None => {
                            // Blocking mode, retry immediately
                            continue;
                        }
                        IpcTimeout::Immediate => {
                            self.state = ResourceState::Ready;
                            return Err(IpcError::ResourceExhausted("Lock not available".to_string()));
                        }
                        IpcTimeout::Duration(max_duration) => {
                            if start_time.elapsed() >= max_duration {
                                if let Ok(mut stats) = self.statistics.lock() {
                                    stats.record_timeout();
                                }
                                self.state = ResourceState::Ready;
                                return Err(IpcError::Timeout("Lock acquisition timed out".to_string()));
                            }
                        }
                        IpcTimeout::Absolute(deadline) => {
                            if SystemTime::now() >= deadline {
                                if let Ok(mut stats) = self.statistics.lock() {
                                    stats.record_timeout();
                                }
                                self.state = ResourceState::Ready;
                                return Err(IpcError::Timeout("Lock acquisition deadline exceeded".to_string()));
                            }
                        }
                    // Wait before retrying
                    std::thread::sleep(self.config.retry_interval);
                }
                Err(e) => {
                    self.state = ResourceState::CursedError;
                    return Err(e);
                }
            }
        }
    }
    
    fn try_acquire_lock(&mut self, exclusive: bool) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if self.config.lock_entire_file {
                // Use flock for entire file
                let operation = if exclusive {
                    libc::LOCK_EX | libc::LOCK_NB
                } else {
                    libc::LOCK_SH | libc::LOCK_NB
                self.flock_operation(operation)
            } else {
                // Use fcntl for range locking
                let lock_type = if exclusive {
                    libc::F_WRLCK
                } else {
                    libc::F_RDLCK
                self.fcntl_lock(lock_type, self.config.start_offset, self.config.length, false)
            }
        }
        
        #[cfg(windows)]
        {
            self.windows_lock(exclusive, self.config.start_offset, self.config.length, false)
        #[cfg(not(any(unix, windows)))]
        {
            Err(IpcError::PlatformError("File locking not supported on this platform".to_string()))
        }
    }
    
    fn release_lock(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if self.config.lock_entire_file {
                self.flock_operation(libc::LOCK_UN)
            } else {
                self.fcntl_lock(libc::F_UNLCK, self.config.start_offset, self.config.length, false)
            }
        }
        
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            use winapi::um::fileapi::UnlockFileEx;
            use winapi::um::winnt::HANDLE;
            
            if let Some(ref file) = self.file {
                let handle = file.as_raw_handle() as HANDLE;
                let mut overlapped = std::mem::zeroed();
                let result = unsafe {
                    UnlockFileEx(
                    )
                
                if result != 0 {
                    Ok(())
                } else {
                    let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
                    Err(IpcError::System(error as i32, format!("UnlockFileEx failed with error {}", error)))
                }
            } else {
                Err(IpcError::InvalidOperation("File not open".to_string()))
            }
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            Err(IpcError::PlatformError("File locking not supported on this platform".to_string()))
        }
    }
impl IpcResource for FileLock {
    fn handle(&self) -> &IpcHandle {
        &self.handle
    fn address(&self) -> &IpcAddress {
        &self.address
    fn permissions(&self) -> &IpcPermissions {
        &self.permissions
    fn statistics(&self) -> IpcResult<IpcStatistics> {
        let stats = self.statistics.lock()
            .map_err(|_| IpcError::Internal("Failed to acquire statistics lock".to_string()))?;
        Ok(stats.base.clone())
    fn is_active(&self) -> bool {
        self.state.is_operational() && self.file.is_some()
    fn close(&mut self) -> IpcResult<()> {
        if !self.current_locks.is_empty() {
            self.release_lock()?;
            self.current_locks.clear();
        self.file = None;
        #[cfg(unix)]
        {
            self.fd = None;
        }
        self.state = ResourceState::Closed;
        
        // Unregister from IPC system
//         crate::stdlib::ipc::unregister_file_lock(&self.handle.id)?;
        
        Ok(())
    fn resource_type(&self) -> &'static str {
        "file_lock"
    fn set_metadata(&mut self, key: String, value: String) -> IpcResult<()> {
        self.handle.metadata.insert(key, value);
        Ok(())
    fn check_limits(&self, _limits: &ResourceLimits) -> IpcResult<()> {
        // File locks don't consume significant resources
        Ok(())
    fn state(&self) -> ResourceState {
        self.state
    fn wait_ready(&self, timeout: IpcTimeout) -> IpcResult<()> {
        let start_time = Instant::now();
        
        loop {
            if self.state.is_available() {
                return Ok(());
            match timeout {
                IpcTimeout::None => {
                    std::thread::sleep(Duration::from_millis(10));
                }
                IpcTimeout::Immediate => {
                    return Err(IpcError::Timeout("Resource not ready".to_string()));
                }
                IpcTimeout::Duration(max_duration) => {
                    if start_time.elapsed() >= max_duration {
                        return Err(IpcError::Timeout("Timeout waiting for resource".to_string()));
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                IpcTimeout::Absolute(deadline) => {
                    if SystemTime::now() >= deadline {
                        return Err(IpcError::Timeout("Deadline exceeded waiting for resource".to_string()));
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
impl IpcFileLocking for FileLock {
    fn lock_exclusive(&mut self) -> IpcResult<()> {
        self.acquire_lock_with_timeout(true, IpcTimeout::None)
    fn lock_shared(&mut self) -> IpcResult<()> {
        self.acquire_lock_with_timeout(false, IpcTimeout::None)
    fn try_lock_exclusive(&mut self) -> IpcResult<bool> {
        match self.acquire_lock_with_timeout(true, IpcTimeout::Immediate) {
        }
    }
    
    fn try_lock_shared(&mut self) -> IpcResult<bool> {
        match self.acquire_lock_with_timeout(false, IpcTimeout::Immediate) {
        }
    }
    
    fn lock_timeout(&mut self, exclusive: bool, timeout: IpcTimeout) -> IpcResult<()> {
        self.acquire_lock_with_timeout(exclusive, timeout)
    fn unlock(&mut self) -> IpcResult<()> {
        if !self.current_locks.is_empty() {
            self.release_lock()?;
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_lock_released();
            }
            self.current_locks.clear();
        }
        Ok(())
    fn is_locked(&self) -> bool {
        !self.current_locks.is_empty()
    fn lock_type(&self) -> Option<LockType> {
        self.current_locks.first().map(|lock| lock.lock_type)
    fn lock_range(&mut self, start: u64, length: u64, exclusive: bool) -> IpcResult<()> {
        // Create a temporary config for range locking
        let mut range_config = self.config.clone();
        range_config.lock_entire_file = false;
        range_config.start_offset = start;
        range_config.length = length;
        
        let original_config = std::mem::replace(&mut self.config, range_config);
        let result = self.acquire_lock_with_timeout(exclusive, IpcTimeout::None);
        self.config = original_config;
        
        result
    fn unlock_range(&mut self, start: u64, length: u64) -> IpcResult<()> {
        // Find and remove locks that match this range
        let mut removed = false;
        self.current_locks.retain(|lock| {
            if lock.start == start && lock.length == length {
                removed = true;
                false
            } else {
                true
            }
        });
        
        if removed {
            // Create a temporary config for range unlocking
            let mut range_config = self.config.clone();
            range_config.lock_entire_file = false;
            range_config.start_offset = start;
            range_config.length = length;
            
            let original_config = std::mem::replace(&mut self.config, range_config);
            let result = self.release_lock();
            self.config = original_config;
            
            if result.is_ok() {
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_lock_released();
                }
            }
            
            result
        } else {
            Err(IpcError::InvalidOperation("No lock found for specified range".to_string()))
        }
    }
    
    fn lock_info(&self) -> IpcResult<Vec<LockInfo>> {
        Ok(self.current_locks.clone())
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Create a file lock with default configuration
pub fn create_file_lock<P: AsRef<Path>>(path: P) -> IpcResult<FileLock> {
    FileLock::create(path)
/// Acquire a file lock with timeout
pub fn acquire_file_lock<P: AsRef<Path>>(path: P, exclusive: bool, timeout: Duration) -> IpcResult<FileLock> {
    let config = LockConfig::new(path)
        .with_timeout(timeout)
        .with_exclusive_lock(); // Will be overridden by exclusive parameter
    
    let mut lock = FileLock::create_with_config(config)?;
    lock.lock_timeout(exclusive, IpcTimeout::Duration(timeout))?;
    Ok(lock)
/// Release a file lock
pub fn release_file_lock(mut lock: FileLock) -> IpcResult<()> {
    lock.unlock()
/// Try to lock a file without blocking
pub fn try_lock_file<P: AsRef<Path>>(path: P, exclusive: bool) -> IpcResult<Option<FileLock>> {
    let config = LockConfig::new(path);
    let mut lock = FileLock::create_with_config(config)?;
    
    let acquired = if exclusive {
        lock.try_lock_exclusive()?
    } else {
        lock.try_lock_shared()?
    
    if acquired {
        Ok(Some(lock))
    } else {
        Ok(None)
    }
}

/// Lock a file with timeout
pub fn lock_file_timeout<P: AsRef<Path>>(path: P, exclusive: bool, timeout: Duration) -> IpcResult<FileLock> {
    acquire_file_lock(path, exclusive, timeout)
