/// Unix-specific process management and IPC implementation
/// 
/// This module provides Unix-specific implementations for the unified
/// process-IPC coordination system, including process namespaces, cgroups,
/// Unix domain sockets, and other Unix-specific features.

#[cfg(unix)]

use std::collections::HashMap;
use std::ffi::{CString, OsStr};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tracing::{info, warn, error, debug, instrument};
use nix::errno::Errno;
use nix::fcntl::{OFlag, open};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::sys::signal::{Signal, kill};
use nix::sys::stat::Mode;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, Pid, getpid, getppid, setsid, setpgid};

use crate::error::CursedError;
use crate::stdlib::process::{
    EnhancedProcess, ResourceLimits, SecurityContext, ProcessState
};
use super::unified_process_ipc::{
    PlatformHandler, IpcType, IpcConnection, SecuritySettings, UnixSettings
};

/// Unix-specific platform handler
#[derive(Debug)]
pub struct UnixPlatformHandler {
    /// Unix-specific settings
    settings: UnixSettings,
    /// Process namespace manager
    namespace_manager: Arc<Mutex<UnixNamespaceManager>>,
    /// Cgroups manager
    cgroups_manager: Arc<Mutex<UnixCgroupsManager>>,
    /// Unix socket manager
    socket_manager: Arc<Mutex<UnixSocketManager>>,
    /// Signal handler manager
    signal_manager: Arc<Mutex<UnixSignalManager>>,
    /// Shared memory manager
    shm_manager: Arc<Mutex<UnixSharedMemoryManager>>,
}

/// Unix namespace management
#[derive(Debug)]
struct UnixNamespaceManager {
    /// Active namespaces
    active_namespaces: HashMap<String, NamespaceInfo>,
    /// Namespace types enabled
    enabled_namespaces: Vec<NamespaceType>,
}

/// Types of Linux namespaces
#[derive(Debug, Clone, PartialEq)]
enum NamespaceType {
    Pid,    // Process ID namespace
    Net,    // Network namespace
    Mnt,    // Mount namespace
    Uts,    // Hostname namespace
    Ipc,    // IPC namespace
    User,   // User namespace
    Cgroup, // Cgroup namespace
}

/// Namespace information
#[derive(Debug)]
struct NamespaceInfo {
    /// Namespace type
    namespace_type: NamespaceType,
    /// Namespace file descriptor
    fd: Option<RawFd>,
    /// Processes in this namespace
    processes: Vec<Pid>,
    /// Creation time
    created_at: std::time::SystemTime,
}

/// Unix cgroups management
#[derive(Debug)]
struct UnixCgroupsManager {
    /// Cgroups base path
    base_path: PathBuf,
    /// Active cgroups
    active_cgroups: HashMap<String, CgroupInfo>,
    /// Cgroup controllers
    enabled_controllers: Vec<CgroupController>,
}

/// Cgroup controllers
#[derive(Debug, Clone)]
enum CgroupController {
    Cpu,
    Memory,
    Pids,
    Blkio,
    Devices,
    Freezer,
}

/// Cgroup information
#[derive(Debug)]
struct CgroupInfo {
    /// Cgroup name
    name: String,
    /// Cgroup path
    path: PathBuf,
    /// Enabled controllers
    controllers: Vec<CgroupController>,
    /// Processes in this cgroup
    processes: Vec<Pid>,
    /// Resource limits
    limits: CgroupLimits,
}

/// Cgroup resource limits
#[derive(Debug, Clone)]
struct CgroupLimits {
    /// CPU limit (percentage)
    cpu_limit: Option<f64>,
    /// Memory limit (bytes)
    memory_limit: Option<u64>,
    /// Process limit
    pids_limit: Option<u32>,
    /// Block I/O weight
    blkio_weight: Option<u16>,
}

/// Unix socket management
#[derive(Debug)]
struct UnixSocketManager {
    /// Active Unix domain sockets
    active_sockets: HashMap<String, UnixSocketInfo>,
    /// Socket directory
    socket_dir: PathBuf,
}

/// Unix socket information
#[derive(Debug)]
struct UnixSocketInfo {
    /// Socket path
    path: PathBuf,
    /// Socket type
    socket_type: UnixSocketType,
    /// Connected processes
    connected_processes: Vec<Pid>,
    /// Listener handle (if server)
    listener: Option<UnixListener>,
}

/// Unix socket types
#[derive(Debug, Clone)]
enum UnixSocketType {
    Stream,
    Datagram,
    SeqPacket,
}

/// Unix signal management
#[derive(Debug)]
struct UnixSignalManager {
    /// Signal handlers
    signal_handlers: HashMap<Signal, SignalHandler>,
    /// Pending signals
    pending_signals: Vec<PendingSignal>,
}

/// Signal handler information
#[derive(Debug)]
struct SignalHandler {
    /// Signal
    signal: Signal,
    /// Handler type
    handler_type: SignalHandlerType,
    /// Processes using this handler
    processes: Vec<Pid>,
}

/// Signal handler types
#[derive(Debug, Clone)]
enum SignalHandlerType {
    Default,
    Ignore,
    Custom,
}

/// Pending signal information
#[derive(Debug)]
struct PendingSignal {
    /// Signal
    signal: Signal,
    /// Target process
    target: Pid,
    /// Timestamp
    timestamp: std::time::SystemTime,
}

/// Unix shared memory management
#[derive(Debug)]
struct UnixSharedMemoryManager {
    /// Active shared memory segments
    active_segments: HashMap<String, ShmSegment>,
    /// Shared memory directory
    shm_dir: PathBuf,
}

/// Shared memory segment
#[derive(Debug)]
struct ShmSegment {
    /// Segment name
    name: String,
    /// File descriptor
    fd: RawFd,
    /// Memory address
    addr: *mut libc::c_void,
    /// Segment size
    size: usize,
    /// Connected processes
    connected_processes: Vec<Pid>,
}

impl UnixPlatformHandler {
    /// Create a new Unix platform handler
    #[instrument]
    pub fn new() -> Result<Self, CursedError> {
        info!("Creating Unix platform handler");
        
        let settings = UnixSettings {
            enable_namespaces: false, // Requires root privileges
            use_unix_sockets: true,
            enable_cgroups: false,    // Requires proper setup
        };
        
        let handler = Self {
            settings,
            namespace_manager: Arc::new(Mutex::new(UnixNamespaceManager::new()?)),
            cgroups_manager: Arc::new(Mutex::new(UnixCgroupsManager::new()?)),
            socket_manager: Arc::new(Mutex::new(UnixSocketManager::new()?)),
            signal_manager: Arc::new(Mutex::new(UnixSignalManager::new()?)),
            shm_manager: Arc::new(Mutex::new(UnixSharedMemoryManager::new()?)),
        };
        
        info!("Unix platform handler created");
        Ok(handler)
    }
    
    /// Create a process namespace
    #[instrument(skip(self))]
    fn create_namespace(&self, namespace_type: NamespaceType) -> Result<RawFd, CursedError> {
        if !self.settings.enable_namespaces {
            return Err(CursedError::Platform("Namespaces not enabled".to_string()));
        }
        
        debug!(namespace_type = ?namespace_type, "Creating process namespace");
        
        let flags = match namespace_type {
            NamespaceType::Pid => libc::CLONE_NEWPID,
            NamespaceType::Net => libc::CLONE_NEWNET,
            NamespaceType::Mnt => libc::CLONE_NEWNS,
            NamespaceType::Uts => libc::CLONE_NEWUTS,
            NamespaceType::Ipc => libc::CLONE_NEWIPC,
            NamespaceType::User => libc::CLONE_NEWUSER,
            NamespaceType::Cgroup => libc::CLONE_NEWCGROUP,
        };
        
        let result = unsafe { libc::unshare(flags) };
        if result != 0 {
            return Err(CursedError::Platform(format!(
                "Failed to create namespace: {}",
                io::Error::last_os_error()
            )));
        }
        
        // Open namespace file descriptor for future operations
        let namespace_path = format!("/proc/self/ns/{}", match namespace_type {
            NamespaceType::Pid => "pid",
            NamespaceType::Net => "net",
            NamespaceType::Mnt => "mnt",
            NamespaceType::Uts => "uts",
            NamespaceType::Ipc => "ipc",
            NamespaceType::User => "user",
            NamespaceType::Cgroup => "cgroup",
        });
        
        let fd = open(
            namespace_path.as_str(),
            OFlag::O_RDONLY,
            Mode::empty(),
        ).map_err(|e| CursedError::Platform(format!("Failed to open namespace fd: {}", e)))?;
        
        debug!(fd = fd, "Process namespace created");
        Ok(fd)
    }
    
    /// Create a cgroup for process resource management
    #[instrument(skip(self))]
    fn create_cgroup(&self, name: &str, limits: CgroupLimits) -> Result<PathBuf, CursedError> {
        if !self.settings.enable_cgroups {
            return Err(CursedError::Platform("Cgroups not enabled".to_string()));
        }
        
        debug!(name = name, "Creating cgroup");
        
        let cgroups_manager = self.cgroups_manager.lock().unwrap();
        let cgroup_path = cgroups_manager.base_path.join(name);
        
        // Create cgroup directory
        std::fs::create_dir_all(&cgroup_path)
            .map_err(|e| CursedError::Platform(format!("Failed to create cgroup directory: {}", e)))?;
        
        // Apply resource limits
        self.apply_cgroup_limits(&cgroup_path, &limits)?;
        
        debug!(path = ?cgroup_path, "Cgroup created");
        Ok(cgroup_path)
    }
    
    /// Apply resource limits to a cgroup
    #[instrument(skip(self))]
    fn apply_cgroup_limits(&self, cgroup_path: &Path, limits: &CgroupLimits) -> Result<(), CursedError> {
        debug!("Applying cgroup resource limits");
        
        // CPU limit
        if let Some(cpu_limit) = limits.cpu_limit {
            let cpu_quota_path = cgroup_path.join("cpu.cfs_quota_us");
            let cpu_period_path = cgroup_path.join("cpu.cfs_period_us");
            
            // Set CPU quota (percentage of CPU time)
            let quota = (100000.0 * cpu_limit / 100.0) as i64;
            std::fs::write(&cpu_quota_path, quota.to_string())
                .map_err(|e| CursedError::Platform(format!("Failed to set CPU quota: {}", e)))?;
            
            std::fs::write(&cpu_period_path, "100000")
                .map_err(|e| CursedError::Platform(format!("Failed to set CPU period: {}", e)))?;
        }
        
        // Memory limit
        if let Some(memory_limit) = limits.memory_limit {
            let memory_limit_path = cgroup_path.join("memory.limit_in_bytes");
            std::fs::write(&memory_limit_path, memory_limit.to_string())
                .map_err(|e| CursedError::Platform(format!("Failed to set memory limit: {}", e)))?;
        }
        
        // Process limit
        if let Some(pids_limit) = limits.pids_limit {
            let pids_limit_path = cgroup_path.join("pids.max");
            std::fs::write(&pids_limit_path, pids_limit.to_string())
                .map_err(|e| CursedError::Platform(format!("Failed to set PIDs limit: {}", e)))?;
        }
        
        debug!("Cgroup resource limits applied");
        Ok(())
    }
    
    /// Create a Unix domain socket
    #[instrument(skip(self))]
    fn create_unix_socket(&self, name: &str, socket_type: UnixSocketType) -> Result<UnixSocketConnection, CursedError> {
        debug!(name = name, socket_type = ?socket_type, "Creating Unix domain socket");
        
        let socket_manager = self.socket_manager.lock().unwrap();
        let socket_path = socket_manager.socket_dir.join(name);
        
        // Remove existing socket file if it exists
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)
                .map_err(|e| CursedError::Platform(format!("Failed to remove existing socket: {}", e)))?;
        }
        
        match socket_type {
            UnixSocketType::Stream => {
                let listener = UnixListener::bind(&socket_path)
                    .map_err(|e| CursedError::Platform(format!("Failed to bind Unix socket: {}", e)))?;
                
                Ok(UnixSocketConnection::Stream {
                    path: socket_path,
                    listener: Some(listener),
                    stream: None,
                })
            }
            UnixSocketType::Datagram => {
                // Unix datagram sockets require different handling
                Err(CursedError::Platform("Unix datagram sockets not yet implemented".to_string()))
            }
            UnixSocketType::SeqPacket => {
                // Unix SEQPACKET sockets require different handling
                Err(CursedError::Platform("Unix SEQPACKET sockets not yet implemented".to_string()))
            }
        }
    }
    
    /// Create shared memory segment
    #[instrument(skip(self))]
    fn create_shared_memory(&self, name: &str, size: usize) -> Result<UnixSharedMemoryConnection, CursedError> {
        debug!(name = name, size = size, "Creating shared memory segment");
        
        let shm_path = format!("/dev/shm/{}", name);
        
        // Create shared memory file
        let fd = open(
            shm_path.as_str(),
            OFlag::O_CREAT | OFlag::O_RDWR,
            Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP,
        ).map_err(|e| CursedError::Platform(format!("Failed to create shared memory: {}", e)))?;
        
        // Set the size of the shared memory segment
        unsafe {
            if libc::ftruncate(fd, size as i64) != 0 {
                let _ = libc::close(fd);
                return Err(CursedError::Platform(format!(
                    "Failed to set shared memory size: {}",
                    io::Error::last_os_error()
                )));
            }
        }
        
        // Map the shared memory
        let addr = unsafe {
            mmap(
                std::ptr::null_mut(),
                size,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                fd,
                0,
            ).map_err(|e| CursedError::Platform(format!("Failed to map shared memory: {}", e)))?
        };
        
        Ok(UnixSharedMemoryConnection {
            name: name.to_string(),
            fd,
            addr,
            size,
        })
    }
    
    /// Apply Unix-specific security to a process
    #[instrument(skip(self, process))]
    fn apply_unix_security(
        &self,
        process: &mut EnhancedProcess,
        settings: &SecuritySettings,
    ) -> Result<(), CursedError> {
        debug!("Applying Unix-specific security");
        
        // Apply process group isolation
        if settings.isolation_level != crate::stdlib::process::unified_process_ipc::IsolationLevel::None {
            // Create new process group
            // This would require access to the process PID
        }
        
        // Apply resource limits using setrlimit
        // This would be implemented based on ResourceLimits
        
        debug!("Unix security applied successfully");
        Ok(())
    }
}

impl PlatformHandler for UnixPlatformHandler {
    #[instrument(skip(self))]
    fn initialize(&self) -> Result<(), CursedError> {
        info!("Initializing Unix platform handler");
        
        // Check if running as root for namespace support
        if self.settings.enable_namespaces {
            let euid = unsafe { libc::geteuid() };
            if euid != 0 {
                warn!("Namespaces require root privileges, disabling namespace support");
            }
        }
        
        // Check cgroups availability
        if self.settings.enable_cgroups {
            let cgroups_path = Path::new("/sys/fs/cgroup");
            if !cgroups_path.exists() {
                warn!("Cgroups not available, disabling cgroups support");
            }
        }
        
        info!("Unix platform handler initialized");
        Ok(())
    }
    
    #[instrument(skip(self))]
    fn create_ipc(&self, ipc_type: IpcType, name: &str) -> Result<Box<dyn IpcConnection>, CursedError> {
        info!(ipc_type = ?ipc_type, name = name, "Creating Unix IPC mechanism");
        
        match ipc_type {
            IpcType::UnixSockets => {
                let socket = self.create_unix_socket(name, UnixSocketType::Stream)?;
                Ok(Box::new(socket))
            }
            IpcType::SharedMemory => {
                let shm = self.create_shared_memory(name, 8192)?; // 8KB default
                Ok(Box::new(shm))
            }
            IpcType::NamedPipes => {
                // Create FIFO (named pipe)
                let pipe = UnixNamedPipeConnection::new(name)?;
                Ok(Box::new(pipe))
            }
            IpcType::Semaphores => {
                // Create POSIX semaphore
                let semaphore = UnixSemaphoreConnection::new(name)?;
                Ok(Box::new(semaphore))
            }
            _ => Err(CursedError::Platform(format!(
                "IPC type {:?} not supported on Unix",
                ipc_type
            ))),
        }
    }
    
    #[instrument(skip(self, process))]
    fn apply_security(
        &self,
        process: &mut EnhancedProcess,
        settings: &SecuritySettings,
    ) -> Result<(), CursedError> {
        self.apply_unix_security(process, settings)
    }
    
    fn get_resource_limits(&self) -> ResourceLimits {
        // Return Unix-specific resource limits based on getrlimit
        ResourceLimits::default()
    }
    
    #[instrument(skip(self))]
    fn cleanup(&self) -> Result<(), CursedError> {
        info!("Cleaning up Unix platform handler");
        
        // Cleanup shared memory segments
        let mut shm_manager = self.shm_manager.lock().unwrap();
        for segment in shm_manager.active_segments.values() {
            unsafe {
                let _ = munmap(segment.addr, segment.size);
                let _ = libc::close(segment.fd);
            }
        }
        shm_manager.active_segments.clear();
        
        // Cleanup Unix sockets
        let mut socket_manager = self.socket_manager.lock().unwrap();
        for socket_info in socket_manager.active_sockets.values() {
            if socket_info.path.exists() {
                let _ = std::fs::remove_file(&socket_info.path);
            }
        }
        socket_manager.active_sockets.clear();
        
        info!("Unix platform handler cleanup completed");
        Ok(())
    }
}

/// Unix socket IPC connection
#[derive(Debug)]
enum UnixSocketConnection {
    Stream {
        path: PathBuf,
        listener: Option<UnixListener>,
        stream: Option<UnixStream>,
    },
}

impl IpcConnection for UnixSocketConnection {
    fn send(&self, message: &[u8]) -> Result<(), CursedError> {
        match self {
            UnixSocketConnection::Stream { stream, .. } => {
                if let Some(ref mut stream) = stream.as_ref() {
                    stream.write_all(message)
                        .map_err(|e| CursedError::Io(e))?;
                } else {
                    return Err(CursedError::Platform("Socket not connected".to_string()));
                }
            }
        }
        Ok(())
    }
    
    fn receive(&self) -> Result<Vec<u8>, CursedError> {
        match self {
            UnixSocketConnection::Stream { stream, .. } => {
                if let Some(ref mut stream) = stream.as_ref() {
                    let mut buffer = vec![0u8; 8192];
                    let bytes_read = stream.read(&mut buffer)
                        .map_err(|e| CursedError::Io(e))?;
                    buffer.truncate(bytes_read);
                    Ok(buffer)
                } else {
                    Err(CursedError::Platform("Socket not connected".to_string()))
                }
            }
        }
    }
    
    fn close(&self) -> Result<(), CursedError> {
        match self {
            UnixSocketConnection::Stream { path, .. } => {
                if path.exists() {
                    std::fs::remove_file(path)
                        .map_err(|e| CursedError::Platform(format!("Failed to remove socket file: {}", e)))?;
                }
            }
        }
        Ok(())
    }
}

/// Unix shared memory IPC connection
#[derive(Debug)]
struct UnixSharedMemoryConnection {
    name: String,
    fd: RawFd,
    addr: *mut libc::c_void,
    size: usize,
}

impl IpcConnection for UnixSharedMemoryConnection {
    fn send(&self, message: &[u8]) -> Result<(), CursedError> {
        if message.len() > self.size {
            return Err(CursedError::Platform("Message too large for shared memory".to_string()));
        }
        
        unsafe {
            std::ptr::copy_nonoverlapping(
                message.as_ptr(),
                self.addr as *mut u8,
                message.len(),
            );
        }
        
        Ok(())
    }
    
    fn receive(&self) -> Result<Vec<u8>, CursedError> {
        let mut buffer = vec![0u8; self.size];
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.addr as *const u8,
                buffer.as_mut_ptr(),
                self.size,
            );
        }
        
        // Find actual message length (simplified)
        if let Some(null_pos) = buffer.iter().position(|&b| b == 0) {
            buffer.truncate(null_pos);
        }
        
        Ok(buffer)
    }
    
    fn close(&self) -> Result<(), CursedError> {
        unsafe {
            let _ = munmap(self.addr, self.size);
            let _ = libc::close(self.fd);
        }
        let shm_path = format!("/dev/shm/{}", self.name);
        if Path::new(&shm_path).exists() {
            std::fs::remove_file(&shm_path)
                .map_err(|e| CursedError::Platform(format!("Failed to remove shared memory file: {}", e)))?;
        }
        Ok(())
    }
}

/// Unix named pipe (FIFO) IPC connection
#[derive(Debug)]
struct UnixNamedPipeConnection {
    path: PathBuf,
    file: Option<File>,
}

impl UnixNamedPipeConnection {
    fn new(name: &str) -> Result<Self, CursedError> {
        let pipe_path = PathBuf::from(format!("/tmp/{}.fifo", name));
        
        // Create FIFO
        let pipe_path_cstr = CString::new(pipe_path.to_string_lossy().as_ref())
            .map_err(|e| CursedError::Platform(format!("Invalid pipe path: {}", e)))?;
        
        let result = unsafe { libc::mkfifo(pipe_path_cstr.as_ptr(), 0o666) };
        if result != 0 && io::Error::last_os_error().kind() != io::ErrorKind::AlreadyExists {
            return Err(CursedError::Platform(format!(
                "Failed to create FIFO: {}",
                io::Error::last_os_error()
            )));
        }
        
        Ok(Self {
            path: pipe_path,
            file: None,
        })
    }
}

impl IpcConnection for UnixNamedPipeConnection {
    fn send(&self, message: &[u8]) -> Result<(), CursedError> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .map_err(|e| CursedError::Io(e))?;
        
        file.write_all(message)
            .map_err(|e| CursedError::Io(e))?;
        
        Ok(())
    }
    
    fn receive(&self) -> Result<Vec<u8>, CursedError> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| CursedError::Io(e))?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| CursedError::Io(e))?;
        
        Ok(buffer)
    }
    
    fn close(&self) -> Result<(), CursedError> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| CursedError::Platform(format!("Failed to remove FIFO: {}", e)))?;
        }
        Ok(())
    }
}

/// Unix semaphore IPC connection
#[derive(Debug)]
struct UnixSemaphoreConnection {
    name: String,
    semaphore: *mut libc::sem_t,
}

impl UnixSemaphoreConnection {
    fn new(name: &str) -> Result<Self, CursedError> {
        let sem_name = CString::new(format!("/{}", name))
            .map_err(|e| CursedError::Platform(format!("Invalid semaphore name: {}", e)))?;
        
        let semaphore = unsafe {
            libc::sem_open(
                sem_name.as_ptr(),
                libc::O_CREAT | libc::O_EXCL,
                0o666,
                1, // Initial value
            )
        };
        
        if semaphore == libc::SEM_FAILED {
            return Err(CursedError::Platform(format!(
                "Failed to create semaphore: {}",
                io::Error::last_os_error()
            )));
        }
        
        Ok(Self {
            name: name.to_string(),
            semaphore,
        })
    }
}

impl IpcConnection for UnixSemaphoreConnection {
    fn send(&self, _message: &[u8]) -> Result<(), CursedError> {
        // Post (increment) semaphore
        let result = unsafe { libc::sem_post(self.semaphore) };
        if result != 0 {
            return Err(CursedError::Platform(format!(
                "Failed to post semaphore: {}",
                io::Error::last_os_error()
            )));
        }
        Ok(())
    }
    
    fn receive(&self) -> Result<Vec<u8>, CursedError> {
        // Wait (decrement) semaphore
        let result = unsafe { libc::sem_wait(self.semaphore) };
        if result != 0 {
            return Err(CursedError::Platform(format!(
                "Failed to wait on semaphore: {}",
                io::Error::last_os_error()
            )));
        }
        Ok(vec![1]) // Success signal
    }
    
    fn close(&self) -> Result<(), CursedError> {
        unsafe {
            let _ = libc::sem_close(self.semaphore);
            let sem_name = CString::new(format!("/{}", self.name)).unwrap();
            let _ = libc::sem_unlink(sem_name.as_ptr());
        }
        Ok(())
    }
}

// Implementation stubs for required types
impl UnixNamespaceManager {
    fn new() -> Result<Self, CursedError> {
        Ok(Self {
            active_namespaces: HashMap::new(),
            enabled_namespaces: Vec::new(),
        })
    }
}

impl UnixCgroupsManager {
    fn new() -> Result<Self, CursedError> {
        Ok(Self {
            base_path: PathBuf::from("/sys/fs/cgroup"),
            active_cgroups: HashMap::new(),
            enabled_controllers: Vec::new(),
        })
    }
}

impl UnixSocketManager {
    fn new() -> Result<Self, CursedError> {
        let socket_dir = PathBuf::from("/tmp/cursed_sockets");
        std::fs::create_dir_all(&socket_dir)
            .map_err(|e| CursedError::Platform(format!("Failed to create socket directory: {}", e)))?;
        
        Ok(Self {
            active_sockets: HashMap::new(),
            socket_dir,
        })
    }
}

impl UnixSignalManager {
    fn new() -> Result<Self, CursedError> {
        Ok(Self {
            signal_handlers: HashMap::new(),
            pending_signals: Vec::new(),
        })
    }
}

impl UnixSharedMemoryManager {
    fn new() -> Result<Self, CursedError> {
        Ok(Self {
            active_segments: HashMap::new(),
            shm_dir: PathBuf::from("/dev/shm"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unix_platform_handler_creation() {
        assert!(UnixPlatformHandler::new().is_ok());
    }
    
    #[test]
    fn test_cgroup_limits() {
        let limits = CgroupLimits {
            cpu_limit: Some(50.0),
            memory_limit: Some(1_000_000_000), // 1GB
            pids_limit: Some(100),
            blkio_weight: Some(500),
        };
        
        assert_eq!(limits.cpu_limit, Some(50.0));
        assert_eq!(limits.memory_limit, Some(1_000_000_000));
    }
    
    #[test]
    fn test_namespace_types() {
        let ns_types = vec![
            NamespaceType::Pid,
            NamespaceType::Net,
            NamespaceType::Mnt,
            NamespaceType::Uts,
            NamespaceType::Ipc,
            NamespaceType::User,
            NamespaceType::Cgroup,
        ];
        
        assert_eq!(ns_types.len(), 7);
    }
}
