use crate::error::CursedError;
/// File descriptor operations and management
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::os::unix::io::RawFd;
use std::time::Duration;

// use crate::stdlib::sys_core::error::{SysCoreError, SysCoreResult, system_call_error, invalid_argument, not_supported};

/// File descriptor wrapper with metadata
#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: RawFd,
    pub flags: FileDescriptorFlags,
    pub created_at: std::time::Instant,
    pub last_accessed: std::time::Instant,
    pub description: String,
}

/// File descriptor flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FileDescriptorFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
    pub append: bool,
    pub readable: bool,
    pub writable: bool,
}

impl Default for FileDescriptorFlags {
    fn default() -> Self {
        Self {
            non_blocking: false,
            close_on_exec: false,
            append: false,
            readable: true,
            writable: true,
        }
    }
}

/// File descriptor table for process management
pub struct FileDescriptorTable {
    descriptors: RwLock<HashMap<RawFd, FileDescriptor>>,
    next_fd: Mutex<RawFd>,
}

impl FileDescriptorTable {
    pub fn new() -> Self {
        Self {
            descriptors: RwLock::new(HashMap::new()),
            next_fd: Mutex::new(3), // Start after stdin, stdout, stderr
        }
    }

    pub fn register_fd(&self, fd: RawFd, description: &str) -> SysCoreResult<()> {
        let mut descriptors = self.descriptors.write().unwrap();
        let file_desc = FileDescriptor {
            fd,
            flags: FileDescriptorFlags::default(),
            created_at: std::time::Instant::now(),
            last_accessed: std::time::Instant::now(),
            description: description.to_string(),
        };
        descriptors.insert(fd, file_desc);
        Ok(())
    }

    pub fn get_fd(&self, fd: RawFd) -> Option<FileDescriptor> {
        let descriptors = self.descriptors.read().unwrap();
        descriptors.get(&fd).cloned()
    }

    pub fn remove_fd(&self, fd: RawFd) -> Option<FileDescriptor> {
        let mut descriptors = self.descriptors.write().unwrap();
        descriptors.remove(&fd)
    }

    pub fn list_fds(&self) -> Vec<FileDescriptor> {
        let descriptors = self.descriptors.read().unwrap();
        descriptors.values().cloned().collect()
    }
}

/// Create a new file descriptor
pub fn create_fd(path: &str, flags: i32, mode: u32) -> SysCoreResult<RawFd> {
    #[cfg(unix)]
    {
        use std::ffi::CString;
        
        let c_path = CString::new(path).map_err(|_| invalid_argument("Invalid path"))?;
        let fd = unsafe { libc::open(c_path.as_ptr(), flags, mode) };
        
        if fd == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("open", errno));
        }
        
        Ok(fd)
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("File descriptor operations not supported on this platform"))
    }
}

/// Close a file descriptor
pub fn close_fd(fd: RawFd) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::close(fd) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("close", errno));
        }
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("File descriptor operations not supported on this platform"))
    }
}

/// Duplicate a file descriptor
pub fn duplicate_fd(old_fd: RawFd, new_fd: Option<RawFd>) -> SysCoreResult<RawFd> {
    #[cfg(unix)]
    {
        let result = match new_fd {
            Some(new) => unsafe { libc::dup2(old_fd, new) },
            None => unsafe { libc::dup(old_fd) },
        };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("dup/dup2", errno));
        }
        
        Ok(result)
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("File descriptor operations not supported on this platform"))
    }
}

/// Get file descriptor flags
pub fn get_fd_flags(fd: RawFd) -> SysCoreResult<FileDescriptorFlags> {
    #[cfg(unix)]
    {
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("fcntl F_GETFL", errno));
        }
        
        let fd_flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
        if fd_flags == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("fcntl F_GETFD", errno));
        }
        
        Ok(FileDescriptorFlags {
            non_blocking: (flags & libc::O_NONBLOCK) != 0,
            append: (flags & libc::O_APPEND) != 0,
            readable: (flags & libc::O_RDONLY) == libc::O_RDONLY || (flags & libc::O_RDWR) == libc::O_RDWR,
            writable: (flags & libc::O_WRONLY) == libc::O_WRONLY || (flags & libc::O_RDWR) == libc::O_RDWR,
            close_on_exec: (fd_flags & libc::FD_CLOEXEC) != 0,
        })
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("File descriptor operations not supported on this platform"))
    }
}

/// Set file descriptor flags
pub fn set_fd_flags(fd: RawFd, flags: FileDescriptorFlags) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let mut file_flags = 0;
        if flags.non_blocking {
            file_flags |= libc::O_NONBLOCK;
        }
        if flags.append {
            file_flags |= libc::O_APPEND;
        }
        
        let result = unsafe { libc::fcntl(fd, libc::F_SETFL, file_flags) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("fcntl F_SETFL", errno));
        }
        
        let mut fd_flags = 0;
        if flags.close_on_exec {
            fd_flags |= libc::FD_CLOEXEC;
        }
        
        let result = unsafe { libc::fcntl(fd, libc::F_SETFD, fd_flags) };
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("fcntl F_SETFD", errno));
        }
        
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("File descriptor operations not supported on this platform"))
    }
}

/// Poll file descriptors for events
pub fn poll_fds(fds: &[(RawFd, i16)], timeout: Option<Duration>) -> SysCoreResult<Vec<(RawFd, i16)>> {
    #[cfg(unix)]
    {
        let mut poll_fds: Vec<libc::pollfd> = fds.iter().map(|(fd, events)| {
            libc::pollfd {
                fd: *fd,
                events: *events,
                revents: 0,
            }
        }).collect();
        
        let timeout_ms = timeout.map_or(-1, |t| t.as_millis() as i32);
        
        let result = unsafe { libc::poll(poll_fds.as_mut_ptr(), poll_fds.len() as u64, timeout_ms) };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("poll", errno));
        }
        
        let ready_fds = poll_fds.iter()
            .filter(|pfd| pfd.revents != 0)
            .map(|pfd| (pfd.fd, pfd.revents))
            .collect();
        
        Ok(ready_fds)
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("Poll operations not supported on this platform"))
    }
}

/// Select file descriptors for events
pub fn select_fds(
    read_fds: &[RawFd],
    write_fds: &[RawFd],
    error_fds: &[RawFd],
    timeout: Option<Duration>
) -> SysCoreResult<(Vec<RawFd>, Vec<RawFd>, Vec<RawFd>)> {
    #[cfg(unix)]
    {
        use std::mem;
        
        let mut read_set: libc::fd_set = unsafe { mem::zeroed() };
        let mut write_set: libc::fd_set = unsafe { mem::zeroed() };
        let mut error_set: libc::fd_set = unsafe { mem::zeroed() };
        
        let mut max_fd = 0;
        
        unsafe {
            libc::FD_ZERO(&mut read_set);
            libc::FD_ZERO(&mut write_set);
            libc::FD_ZERO(&mut error_set);
        }
        
        for &fd in read_fds {
            unsafe { libc::FD_SET(fd, &mut read_set) };
            max_fd = max_fd.max(fd);
        }
        
        for &fd in write_fds {
            unsafe { libc::FD_SET(fd, &mut write_set) };
            max_fd = max_fd.max(fd);
        }
        
        for &fd in error_fds {
            unsafe { libc::FD_SET(fd, &mut error_set) };
            max_fd = max_fd.max(fd);
        }
        
        let mut tv = timeout.map(|t| libc::timeval {
            tv_sec: t.as_secs() as i64,
            tv_usec: t.subsec_micros() as i64,
        });
        
        let tv_ptr = tv.as_mut().map_or(std::ptr::null_mut(), |t| t as *mut _);
        
        let result = unsafe {
            libc::select(
                max_fd + 1,
                &mut read_set,
                &mut write_set,
                &mut error_set,
                tv_ptr,
            )
        };
        
        if result == -1 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_call_error("select", errno));
        }
        
        let ready_read: Vec<RawFd> = read_fds.iter()
            .filter(|&&fd| unsafe { libc::FD_ISSET(fd, &read_set) })
            .copied()
            .collect();
        
        let ready_write: Vec<RawFd> = write_fds.iter()
            .filter(|&&fd| unsafe { libc::FD_ISSET(fd, &write_set) })
            .copied()
            .collect();
        
        let ready_error: Vec<RawFd> = error_fds.iter()
            .filter(|&&fd| unsafe { libc::FD_ISSET(fd, &error_set) })
            .copied()
            .collect();
        
        Ok((ready_read, ready_write, ready_error))
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("Select operations not supported on this platform"))
    }
}

/// Epoll operations (Linux-specific)
pub fn epoll_operations() -> EpollOperations {
    EpollOperations::new()
}

/// Epoll operations wrapper
pub struct EpollOperations {
    epoll_fd: Option<RawFd>,
}

impl EpollOperations {
    pub fn new() -> Self {
        Self { epoll_fd: None }
    }
    
    pub fn create(&mut self) -> SysCoreResult<()> {
        #[cfg(target_os = "linux")]
        {
            let fd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
            if fd == -1 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_call_error("epoll_create1", errno));
            }
            self.epoll_fd = Some(fd);
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Err(not_supported("Epoll is only available on Linux"))
        }
    }
    
    pub fn add_fd(&self, fd: RawFd, events: u32) -> SysCoreResult<()> {
        #[cfg(target_os = "linux")]
        {
            let epoll_fd = self.epoll_fd.ok_or_else(|| invalid_argument("Epoll not created"))?;
            
            let mut event = libc::epoll_event {
                events,
                u64: fd as u64,
            };
            
            let result = unsafe { libc::epoll_ctl(epoll_fd, libc::EPOLL_CTL_ADD, fd, &mut event) };
            if result == -1 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_call_error("epoll_ctl ADD", errno));
            }
            
            Ok(())
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Err(not_supported("Epoll is only available on Linux"))
        }
    }
    
    pub fn wait(&self, max_events: usize, timeout: Option<Duration>) -> SysCoreResult<Vec<(RawFd, u32)>> {
        #[cfg(target_os = "linux")]
        {
            let epoll_fd = self.epoll_fd.ok_or_else(|| invalid_argument("Epoll not created"))?;
            
            let mut events = vec![unsafe { std::mem::zeroed::<libc::epoll_event>() }; max_events];
            let timeout_ms = timeout.map_or(-1, |t| t.as_millis() as i32);
            
            let result = unsafe {
                libc::epoll_wait(
                    epoll_fd,
                    events.as_mut_ptr(),
                    max_events as i32,
                    timeout_ms,
                )
            };
            
            if result == -1 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_call_error("epoll_wait", errno));
            }
            
            let ready_events = events[..result as usize]
                .iter()
                .map(|event| (event.u64 as RawFd, event.events))
                .collect();
            
            Ok(ready_events)
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Err(not_supported("Epoll is only available on Linux"))
        }
    }
}

impl Drop for EpollOperations {
    fn drop(&mut self) {
        if let Some(fd) = self.epoll_fd {
            let _ = close_fd(fd);
        }
    }
}

/// Check if epoll is supported on this platform
pub fn supports_epoll() -> bool {
    cfg!(target_os = "linux")
}

// Global file descriptor table
static mut GLOBAL_FD_TABLE: Option<FileDescriptorTable> = None;
static INIT_FD_TABLE: std::sync::Once = std::sync::Once::new();

/// Get the global file descriptor table
pub fn get_fd_table() -> &'static FileDescriptorTable {
    unsafe {
        INIT_FD_TABLE.call_once(|| {
            GLOBAL_FD_TABLE = Some(FileDescriptorTable::new());
        });
        GLOBAL_FD_TABLE.as_ref().unwrap()
    }
}
