use crate::error::Error;
/// Shared memory implementation for CURSED IPC
/// 
/// Provides System V shared memory and POSIX shared memory for inter-process communication

use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex, RwLock, Condvar, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::time::{Duration, Instant};

#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

use crate::stdlib::ipc::error::{IpcError, IpcResult, shared_memory_error, system_error, not_found, already_exists};

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn CreateFileMappingW(
        hFile: *mut std::ffi::c_void,
        lpFileMappingAttributes: *const std::ffi::c_void,
        flProtect: u32,
        dwMaximumSizeHigh: u32,
        dwMaximumSizeLow: u32,
        lpName: *const u16,
    ) -> *mut std::ffi::c_void;
    
    fn OpenFileMappingW(
        dwDesiredAccess: u32,
        bInheritHandle: i32,
        lpName: *const u16,
    ) -> *mut std::ffi::c_void;
    
    fn MapViewOfFile(
        hFileMappingObject: *mut std::ffi::c_void,
        dwDesiredAccess: u32,
        dwFileOffsetHigh: u32,
        dwFileOffsetLow: u32,
        dwNumberOfBytesToMap: usize,
    ) -> *mut std::ffi::c_void;
    
    fn UnmapViewOfFile(lpBaseAddress: *const std::ffi::c_void) -> i32;
    fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
    fn GetLastError() -> u32;
    fn VirtualLock(lpAddress: *const std::ffi::c_void, dwSize: usize) -> i32;
    fn VirtualUnlock(lpAddress: *const std::ffi::c_void, dwSize: usize) -> i32;
}

#[cfg(windows)]
const INVALID_HANDLE_VALUE: *mut std::ffi::c_void = (-1isize) as *mut std::ffi::c_void;
#[cfg(windows)]
const PAGE_READWRITE: u32 = 0x04;
#[cfg(windows)]
const FILE_MAP_ALL_ACCESS: u32 = 0xF001F;
#[cfg(windows)]
const ERROR_ALREADY_EXISTS: u32 = 183;
#[cfg(windows)]
const ERROR_FILE_NOT_FOUND: u32 = 2;

/// Shared memory registry for cleanup
static MEMORY_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<String, Arc<SharedMemoryInfo>>>>> = std::sync::OnceLock::new();

#[derive(Debug)]
struct SharedMemoryInfo {
    name: String,
    segment_id: i32,
    size: usize,
    created_by_us: bool,
    ref_count: Arc<Mutex<usize>>,
}

fn get_memory_registry() -> &'static Arc<RwLock<HashMap<String, Arc<SharedMemoryInfo>>>> {
    MEMORY_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Shared memory configuration
#[derive(Debug, Clone)]
pub struct SharedMemoryConfig {
    /// Size of the shared memory segment
    pub size: usize,
    /// Permissions for the segment
    pub permissions: u32,
    /// Whether to create the segment if it doesn't exist
    pub create_if_missing: bool,
    /// Whether to use POSIX shared memory (vs System V)
    pub use_posix: bool,
    /// Whether to lock memory in RAM
    pub lock_memory: bool,
    /// Whether to zero-initialize memory
    pub zero_initialize: bool,
}

impl Default for SharedMemoryConfig {
    fn default() -> Self {
        Self {
            size: 4096, // 4KB default
            permissions: 0o666,
            create_if_missing: true,
            use_posix: false, // Default to System V for broader compatibility
            lock_memory: false,
            zero_initialize: true,
        }
    }
}

/// Cross-platform shared memory segment
#[derive(Debug)]
pub struct SharedMemory {
    name: String,
    config: SharedMemoryConfig,
    #[cfg(unix)]
    segment_id: Option<i32>,
    #[cfg(unix)]
    posix_fd: Option<i32>,
    #[cfg(windows)]
    file_mapping: Option<*mut std::ffi::c_void>,
    ptr: Option<*mut u8>,
    actual_size: usize,
    is_attached: bool,
}

unsafe impl Send for SharedMemory {}
unsafe impl Sync for SharedMemory {}

impl SharedMemory {
    /// Create a new shared memory segment
    pub fn new(name: &str, size: usize) -> Self {
        let config = SharedMemoryConfig {
            size,
            ..Default::default()
        };
        
        Self {
            name: name.to_string(),
            config,
            #[cfg(unix)]
            segment_id: None,
            #[cfg(unix)]
            posix_fd: None,
            #[cfg(windows)]
            file_mapping: None,
            ptr: None,
            actual_size: 0,
            is_attached: false,
        }
    }

    /// Create with configuration
    pub fn with_config(name: &str, config: SharedMemoryConfig) -> Self {
        Self {
            name: name.to_string(),
            config,
            #[cfg(unix)]
            segment_id: None,
            #[cfg(unix)]
            posix_fd: None,
            #[cfg(windows)]
            file_mapping: None,
            ptr: None,
            actual_size: 0,
            is_attached: false,
        }
    }

    /// Create and attach to shared memory
    pub fn create_and_attach(&mut self) -> IpcResult<()> {
        self.create()?;
        self.attach()
    }

    /// Create the shared memory segment
    pub fn create(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.create_posix()
            } else {
                self.create_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.create_windows()
        }
    }

    /// Attach to shared memory
    pub fn attach(&mut self) -> IpcResult<()> {
        if self.is_attached {
            return Ok(());
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.attach_posix()
            } else {
                self.attach_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.attach_windows()
        }
    }

    /// Detach from shared memory
    pub fn detach(&mut self) -> IpcResult<()> {
        if !self.is_attached {
            return Ok(());
        }

        #[cfg(unix)]
        {
            if let Some(ptr) = self.ptr {
                let result = unsafe {
                    libc::shmdt(ptr as *const libc::c_void)
                };
                
                if result < 0 {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    return Err(system_error(errno, "shmdt", "Failed to detach shared memory"));
                }
                
                self.ptr = None;
                self.is_attached = false;
            }
            
            if let Some(fd) = self.posix_fd {
                if let Some(ptr) = self.ptr {
                    let result = unsafe {
                        libc::munmap(ptr as *mut libc::c_void, self.actual_size)
                    };
                    
                    if result < 0 {
                        let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                        return Err(system_error(errno, "munmap", "Failed to unmap POSIX shared memory"));
                    }
                }
                
                unsafe { libc::close(fd); }
                self.posix_fd = None;
                self.ptr = None;
                self.is_attached = false;
            }
        }

        #[cfg(windows)]
        {
            self.detach_windows()?;
        }

        self.unregister_segment();
        Ok(())
    }

    /// Delete the shared memory segment
    pub fn delete(&mut self) -> IpcResult<()> {
        self.detach()?;

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.delete_posix()
            } else {
                self.delete_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.delete_windows()
        }
    }

    /// Get raw pointer to shared memory
    pub fn as_ptr(&self) -> Option<*mut u8> {
        self.ptr
    }

    /// Get shared memory as byte slice
    pub fn as_slice(&self) -> Option<&[u8]> {
        if let Some(ptr) = self.ptr {
            Some(unsafe { slice::from_raw_parts(ptr, self.actual_size) })
        } else {
            None
        }
    }

    /// Get shared memory as mutable byte slice
    pub fn as_mut_slice(&mut self) -> Option<&mut [u8]> {
        if let Some(ptr) = self.ptr {
            Some(unsafe { slice::from_raw_parts_mut(ptr, self.actual_size) })
        } else {
            None
        }
    }

    /// Read data from shared memory
    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> IpcResult<usize> {
        if !self.is_attached {
            return Err(shared_memory_error(Some(&self.name), "read", "Not attached to shared memory"));
        }

        if offset >= self.actual_size {
            return Err(shared_memory_error(Some(&self.name), "read", "Offset beyond memory bounds"));
        }

        let available = self.actual_size - offset;
        let to_read = buffer.len().min(available);

        if let Some(ptr) = self.ptr {
            unsafe {
                ptr::copy_nonoverlapping(
                    ptr.add(offset),
                    buffer.as_mut_ptr(),
                    to_read
                );
            }
            Ok(to_read)
        } else {
            Err(shared_memory_error(Some(&self.name), "read", "Invalid memory pointer"))
        }
    }

    /// Write data to shared memory
    pub fn write(&mut self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        if !self.is_attached {
            return Err(shared_memory_error(Some(&self.name), "write", "Not attached to shared memory"));
        }

        if offset >= self.actual_size {
            return Err(shared_memory_error(Some(&self.name), "write", "Offset beyond memory bounds"));
        }

        let available = self.actual_size - offset;
        let to_write = data.len().min(available);

        if let Some(ptr) = self.ptr {
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    ptr.add(offset),
                    to_write
                );
            }
            Ok(to_write)
        } else {
            Err(shared_memory_error(Some(&self.name), "write", "Invalid memory pointer"))
        }
    }

    /// Zero-fill the shared memory
    pub fn zero_fill(&mut self) -> IpcResult<()> {
        if !self.is_attached {
            return Err(shared_memory_error(Some(&self.name), "zero_fill", "Not attached to shared memory"));
        }

        if let Some(ptr) = self.ptr {
            unsafe {
                ptr::write_bytes(ptr, 0, self.actual_size);
            }
            Ok(())
        } else {
            Err(shared_memory_error(Some(&self.name), "zero_fill", "Invalid memory pointer"))
        }
    }

    /// Get segment size
    pub fn size(&self) -> usize {
        self.actual_size
    }

    /// Get segment name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if attached
    pub fn is_attached(&self) -> bool {
        self.is_attached
    }

    /// Get segment statistics
    pub fn stats(&self) -> IpcResult<SharedMemoryStats> {
        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.stats_posix()
            } else {
                self.stats_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.stats_windows()
        }
    }

    #[cfg(unix)]
    fn create_sysv(&mut self) -> IpcResult<()> {
        // Generate key from name
        let key = self.generate_sysv_key()?;
        
        // Try to get existing segment first
        let segment_id = unsafe {
            libc::shmget(key, 0, 0)
        };
        
        if segment_id >= 0 {
            // Check size compatibility
            let mut ds: libc::shmid_ds = unsafe { mem::zeroed() };
            let result = unsafe {
                libc::shmctl(segment_id, libc::IPC_STAT, &mut ds)
            };
            
            if result >= 0 && ds.shm_segsz >= self.config.size {
                self.segment_id = Some(segment_id);
                self.actual_size = ds.shm_segsz;
                self.register_segment(segment_id, false);
                return Ok(());
            }
        }
        
        // Create new segment if allowed
        if !self.config.create_if_missing {
            return Err(not_found("shared_memory", &self.name, "Segment does not exist"));
        }
        
        let segment_id = unsafe {
            libc::shmget(
                key,
                self.config.size,
                libc::IPC_CREAT | libc::IPC_EXCL | (self.config.permissions as i32)
            )
        };
        
        if segment_id < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            if errno == libc::EEXIST {
                // Segment was created by another process
                let segment_id = unsafe { libc::shmget(key, 0, 0) };
                if segment_id >= 0 {
                    self.segment_id = Some(segment_id);
                    self.actual_size = self.config.size;
                    self.register_segment(segment_id, false);
                    return Ok(());
                }
            }
            return Err(system_error(errno, "shmget", "Failed to create shared memory segment"));
        }
        
        self.segment_id = Some(segment_id);
        self.actual_size = self.config.size;
        self.register_segment(segment_id, true);
        Ok(())
    }

    #[cfg(unix)]
    fn create_posix(&mut self) -> IpcResult<()> {
        let segment_name = format!("/{}", self.name);
        let segment_name_cstr = CString::new(segment_name)
            .map_err(|e| shared_memory_error(Some(&self.name), "create", &e.to_string()))?;
        
        // Try to open existing segment first
        let fd = unsafe {
            libc::shm_open(segment_name_cstr.as_ptr(), libc::O_RDWR, 0)
        };
        
        if fd >= 0 {
            // Check size compatibility
            let mut stat: libc::stat = unsafe { mem::zeroed() };
            let result = unsafe { libc::fstat(fd, &mut stat) };
            
            if result >= 0 && stat.st_size >= self.config.size as i64 {
                self.posix_fd = Some(fd);
                self.actual_size = stat.st_size as usize;
                self.register_segment(fd, false);
                return Ok(());
            }
            
            unsafe { libc::close(fd); }
        }
        
        // Create new segment if allowed
        if !self.config.create_if_missing {
            return Err(not_found("shared_memory", &self.name, "Segment does not exist"));
        }
        
        let fd = unsafe {
            libc::shm_open(
                segment_name_cstr.as_ptr(),
                libc::O_CREAT | libc::O_EXCL | libc::O_RDWR,
                self.config.permissions
            )
        };
        
        if fd < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            if errno == libc::EEXIST {
                // Segment was created by another process, try to open it
                let fd = unsafe {
                    libc::shm_open(segment_name_cstr.as_ptr(), libc::O_RDWR, 0)
                };
                if fd >= 0 {
                    self.posix_fd = Some(fd);
                    self.actual_size = self.config.size;
                    self.register_segment(fd, false);
                    return Ok(());
                }
            }
            return Err(system_error(errno, "shm_open", "Failed to create POSIX shared memory"));
        }
        
        // Set the size of the segment
        let result = unsafe {
            libc::ftruncate(fd, self.config.size as i64)
        };
        
        if result < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            unsafe { libc::close(fd); }
            return Err(system_error(errno, "ftruncate", "Failed to set shared memory size"));
        }
        
        self.posix_fd = Some(fd);
        self.actual_size = self.config.size;
        self.register_segment(fd, true);
        Ok(())
    }

    #[cfg(windows)]
    fn create_windows(&mut self) -> IpcResult<()> {
        // Convert segment name to wide string for Windows API
        let segment_name = format!("Global\\{}", self.name);
        let wide_name: Vec<u16> = OsStr::new(&segment_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        // Try to open existing mapping first
        let existing_handle = unsafe {
            OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, wide_name.as_ptr())
        };
        
        if existing_handle != ptr::null_mut() && existing_handle != INVALID_HANDLE_VALUE {
            // Found existing mapping
            self.file_mapping = Some(existing_handle);
            self.actual_size = self.config.size;
            self.register_segment(existing_handle as i32, false);
            return Ok(());
        }
        
        // Create new mapping if allowed
        if !self.config.create_if_missing {
            return Err(not_found("shared_memory", &self.name, "Segment does not exist"));
        }
        
        // Create new file mapping
        let handle = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE, // Use paging file
                ptr::null(),          // Default security
                PAGE_READWRITE,       // Read/write access
                0,                    // High-order size (0 for 32-bit size)
                self.config.size as u32, // Low-order size
                wide_name.as_ptr(),   // Mapping name
            )
        };
        
        if handle == ptr::null_mut() || handle == INVALID_HANDLE_VALUE {
            let error_code = unsafe { GetLastError() };
            return Err(system_error(
                error_code as i32,
                "CreateFileMappingW",
                "Failed to create Windows file mapping"
            ));
        }
        
        // Check if mapping already existed
        let error_code = unsafe { GetLastError() };
        let created_by_us = error_code != ERROR_ALREADY_EXISTS;
        
        self.file_mapping = Some(handle);
        self.actual_size = self.config.size;
        self.register_segment(handle as i32, created_by_us);
        
        Ok(())
    }

    #[cfg(unix)]
    fn attach_sysv(&mut self) -> IpcResult<()> {
        if let Some(segment_id) = self.segment_id {
            let ptr = unsafe {
                libc::shmat(segment_id, ptr::null(), 0)
            };
            
            if ptr == libc::MAP_FAILED {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "shmat", "Failed to attach to shared memory"));
            }
            
            self.ptr = Some(ptr as *mut u8);
            self.is_attached = true;
            
            // Zero-initialize if requested
            if self.config.zero_initialize {
                self.zero_fill()?;
            }
            
            // Lock memory if requested
            if self.config.lock_memory {
                unsafe {
                    libc::mlock(ptr, self.actual_size);
                }
            }
            
            Ok(())
        } else {
            Err(shared_memory_error(Some(&self.name), "attach", "Segment not created"))
        }
    }

    #[cfg(unix)]
    fn attach_posix(&mut self) -> IpcResult<()> {
        if let Some(fd) = self.posix_fd {
            let ptr = unsafe {
                libc::mmap(
                    ptr::null_mut(),
                    self.actual_size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    fd,
                    0
                )
            };
            
            if ptr == libc::MAP_FAILED {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "mmap", "Failed to map POSIX shared memory"));
            }
            
            self.ptr = Some(ptr as *mut u8);
            self.is_attached = true;
            
            // Zero-initialize if requested
            if self.config.zero_initialize {
                self.zero_fill()?;
            }
            
            // Lock memory if requested
            if self.config.lock_memory {
                unsafe {
                    libc::mlock(ptr, self.actual_size);
                }
            }
            
            Ok(())
        } else {
            Err(shared_memory_error(Some(&self.name), "attach", "Segment not created"))
        }
    }

    #[cfg(windows)]
    fn attach_windows(&mut self) -> IpcResult<()> {
        if let Some(handle) = self.file_mapping {
            // Map the view into the process address space
            let ptr = unsafe {
                MapViewOfFile(
                    handle,              // File mapping handle
                    FILE_MAP_ALL_ACCESS, // Access mode
                    0,                   // High-order offset
                    0,                   // Low-order offset
                    self.actual_size,    // Number of bytes to map
                )
            };
            
            if ptr == ptr::null_mut() {
                let error_code = unsafe { GetLastError() };
                return Err(system_error(
                    error_code as i32,
                    "MapViewOfFile",
                    "Failed to map Windows shared memory view"
                ));
            }
            
            self.ptr = Some(ptr as *mut u8);
            self.is_attached = true;
            
            // Zero-initialize if requested
            if self.config.zero_initialize {
                self.zero_fill()?;
            }
            
            // Lock memory if requested
            if self.config.lock_memory {
                unsafe {
                    VirtualLock(ptr, self.actual_size);
                }
            }
            
            Ok(())
        } else {
            Err(shared_memory_error(Some(&self.name), "attach", "File mapping not created"))
        }
    }

    #[cfg(windows)]
    fn detach_windows(&mut self) -> IpcResult<()> {
        if !self.is_attached {
            return Ok(());
        }
        
        // Unlock memory if it was locked
        if self.config.lock_memory {
            if let Some(ptr) = self.ptr {
                unsafe {
                    VirtualUnlock(ptr as *const std::ffi::c_void, self.actual_size);
                }
            }
        }
        
        // Unmap the view
        if let Some(ptr) = self.ptr {
            let result = unsafe {
                UnmapViewOfFile(ptr as *const std::ffi::c_void)
            };
            
            if result == 0 {
                let error_code = unsafe { GetLastError() };
                return Err(system_error(
                    error_code as i32,
                    "UnmapViewOfFile",
                    "Failed to unmap Windows shared memory view"
                ));
            }
            
            self.ptr = None;
        }
        
        // Close the file mapping handle
        if let Some(handle) = self.file_mapping {
            unsafe {
                CloseHandle(handle);
            }
            self.file_mapping = None;
        }
        
        self.is_attached = false;
        Ok(())
    }

    #[cfg(unix)]
    fn delete_sysv(&mut self) -> IpcResult<()> {
        if let Some(segment_id) = self.segment_id {
            let result = unsafe {
                libc::shmctl(segment_id, libc::IPC_RMID, ptr::null_mut())
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "shmctl", "Failed to delete shared memory segment"));
            }
            
            self.segment_id = None;
        }
        
        Ok(())
    }

    #[cfg(unix)]
    fn delete_posix(&mut self) -> IpcResult<()> {
        let segment_name = format!("/{}", self.name);
        let segment_name_cstr = CString::new(segment_name)
            .map_err(|e| shared_memory_error(Some(&self.name), "delete", &e.to_string()))?;
        
        let result = unsafe {
            libc::shm_unlink(segment_name_cstr.as_ptr())
        };
        
        if result < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            if errno != libc::ENOENT {
                return Err(system_error(errno, "shm_unlink", "Failed to delete POSIX shared memory"));
            }
        }
        
        Ok(())
    }

    #[cfg(windows)]
    fn delete_windows(&mut self) -> IpcResult<()> {
        // On Windows, file mappings are automatically cleaned up when the last
        // handle is closed and all views are unmapped. The detach_windows method
        // already handles closing the file mapping handle.
        // 
        // Unlike Unix shared memory, Windows file mappings don't require explicit
        // deletion as they are reference counted by the kernel.
        Ok(())
    }

    #[cfg(unix)]
    fn stats_sysv(&self) -> IpcResult<SharedMemoryStats> {
        if let Some(segment_id) = self.segment_id {
            let mut ds: libc::shmid_ds = unsafe { mem::zeroed() };
            
            let result = unsafe {
                libc::shmctl(segment_id, libc::IPC_STAT, &mut ds)
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "shmctl", "Failed to get segment statistics"));
            }
            
            Ok(SharedMemoryStats {
                size: ds.shm_segsz,
                attach_count: ds.shm_nattch as usize,
                creator_pid: ds.shm_cpid,
                last_attach_pid: ds.shm_lpid,
                creation_time: std::time::SystemTime::UNIX_EPOCH + 
                    std::time::Duration::from_secs(ds.shm_ctime as u64),
                last_attach_time: std::time::SystemTime::UNIX_EPOCH + 
                    std::time::Duration::from_secs(ds.shm_atime as u64),
                last_detach_time: std::time::SystemTime::UNIX_EPOCH + 
                    std::time::Duration::from_secs(ds.shm_dtime as u64),
            })
        } else {
            Err(shared_memory_error(Some(&self.name), "stats", "Segment not created"))
        }
    }

    #[cfg(unix)]
    fn stats_posix(&self) -> IpcResult<SharedMemoryStats> {
        if let Some(fd) = self.posix_fd {
            let mut stat: libc::stat = unsafe { mem::zeroed() };
            
            let result = unsafe { libc::fstat(fd, &mut stat) };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "fstat", "Failed to get POSIX segment statistics"));
            }
            
            Ok(SharedMemoryStats {
                size: stat.st_size as usize,
                attach_count: 0, // Not available in POSIX
                creator_pid: 0, // Not directly available
                last_attach_pid: 0, // Not available in POSIX
                creation_time: std::time::SystemTime::UNIX_EPOCH + 
                    std::time::Duration::from_secs(stat.st_ctime as u64),
                last_attach_time: std::time::SystemTime::UNIX_EPOCH + 
                    std::time::Duration::from_secs(stat.st_atime as u64),
                last_detach_time: std::time::SystemTime::UNIX_EPOCH + 
                    std::time::Duration::from_secs(stat.st_mtime as u64),
            })
        } else {
            Err(shared_memory_error(Some(&self.name), "stats", "Segment not created"))
        }
    }

    #[cfg(windows)]
    fn stats_windows(&self) -> IpcResult<SharedMemoryStats> {
        Ok(SharedMemoryStats {
            size: self.actual_size,
            attach_count: 1,
            creator_pid: std::process::id(),
            last_attach_pid: std::process::id(),
            creation_time: std::time::SystemTime::now(),
            last_attach_time: std::time::SystemTime::now(),
            last_detach_time: std::time::SystemTime::now(),
        })
    }

    #[cfg(unix)]
    fn generate_sysv_key(&self) -> IpcResult<i32> {
        // Generate a key based on the segment name
        let mut hash = 0i32;
        for byte in self.name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as i32);
        }
        
        // Ensure it's a valid System V key (non-zero)
        if hash == 0 {
            hash = 1;
        }
        
        Ok(hash)
    }

    fn register_segment(&self, segment_id: i32, created_by_us: bool) {
        let registry = get_memory_registry();
        if let Ok(mut segments) = registry.write() {
            let info = Arc::new(SharedMemoryInfo {
                name: self.name.clone(),
                segment_id,
                size: self.actual_size,
                created_by_us,
                ref_count: Arc::new(Mutex::new(1)),
            });
            segments.insert(self.name.clone(), info);
        }
    }

    fn unregister_segment(&self) {
        let registry = get_memory_registry();
        if let Ok(mut segments) = registry.write() {
            if let Some(info) = segments.get(&self.name) {
                let mut ref_count = info.ref_count.lock().unwrap();
                *ref_count -= 1;
                if *ref_count == 0 {
                    segments.remove(&self.name);
                }
            }
        }
    }
}

impl Drop for SharedMemory {
    fn drop(&mut self) {
        let _ = self.detach();
    }
}

/// Shared memory segment wrapper for easier use
#[derive(Debug)]
pub struct SharedMemorySegment<T> {
    memory: SharedMemory,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SharedMemorySegment<T> {
    /// Create a new typed shared memory segment
    pub fn new(name: &str, count: usize) -> IpcResult<Self> {
        let size = mem::size_of::<T>() * count;
        let mut memory = SharedMemory::new(name, size);
        memory.create_and_attach()?;
        
        Ok(Self {
            memory,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Create a new shared memory segment (for compatibility)
    pub fn create(name: &str, size: usize) -> IpcResult<Self> {
        let mut memory = SharedMemory::new(name, size);
        memory.create_and_attach()?;
        
        Ok(Self {
            memory,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Get as typed slice
    pub fn as_slice(&self) -> Option<&[T]> {
        if let Some(slice) = self.memory.as_slice() {
            let count = slice.len() / mem::size_of::<T>();
            Some(unsafe {
                slice::from_raw_parts(slice.as_ptr() as *const T, count)
            })
        } else {
            None
        }
    }

    /// Get as mutable typed slice
    pub fn as_mut_slice(&mut self) -> Option<&mut [T]> {
        if let Some(slice) = self.memory.as_mut_slice() {
            let count = slice.len() / mem::size_of::<T>();
            Some(unsafe {
                slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, count)
            })
        } else {
            None
        }
    }

    /// Get capacity (number of T elements)
    pub fn capacity(&self) -> usize {
        self.memory.size() / mem::size_of::<T>()
    }

    /// Access underlying memory
    pub fn memory(&self) -> &SharedMemory {
        &self.memory
    }

    /// Access underlying memory mutably
    pub fn memory_mut(&mut self) -> &mut SharedMemory {
        &mut self.memory
    }

    /// Write data to shared memory at offset
    pub fn write_data(&self, offset: usize, data: &[u8]) -> IpcResult<()> {
        self.memory.write(offset, data)
    }

    /// Read data from shared memory at offset
    pub fn read_data(&self, offset: usize, len: usize) -> IpcResult<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.memory.read(offset, &mut buffer)?;
        Ok(buffer)
    }
}

/// Shared memory statistics
#[derive(Debug, Clone)]
pub struct SharedMemoryStats {
    /// Size of the segment
    pub size: usize,
    /// Number of attached processes
    pub attach_count: usize,
    /// PID of creating process
    pub creator_pid: u32,
    /// PID of last process to attach
    pub last_attach_pid: u32,
    /// Creation time
    pub creation_time: std::time::SystemTime,
    /// Last attach time
    pub last_attach_time: std::time::SystemTime,
    /// Last detach time
    pub last_detach_time: std::time::SystemTime,
}

/// Cleanup all registered shared memory segments
pub fn cleanup_segments() -> IpcResult<()> {
    let registry = get_memory_registry();
    if let Ok(mut segments) = registry.write() {
        for (name, info) in segments.drain() {
            #[cfg(unix)]
            if info.created_by_us {
                // Try to delete the segment
                unsafe {
                    libc::shmctl(info.segment_id, libc::IPC_RMID, ptr::null_mut());
                }
                tracing::debug!(segment_name = name, "Cleaned up shared memory segment");
            }
            
            #[cfg(windows)]
            if info.created_by_us {
                // Close the handle
                unsafe {
                    CloseHandle(info.segment_id as *mut std::ffi::c_void);
                }
                tracing::debug!(segment_name = name, "Cleaned up Windows shared memory segment");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_memory_config() {
        let config = SharedMemoryConfig::default();
        assert_eq!(config.size, 4096);
        assert_eq!(config.permissions, 0o666);
        assert!(config.create_if_missing);
        assert!(!config.use_posix);
    }

    #[test]
    fn test_shared_memory_creation() {
        let memory = SharedMemory::new("test_segment", 1024);
        assert_eq!(memory.name(), "test_segment");
        assert!(!memory.is_attached());
    }

    #[cfg(unix)]
    #[test]
    fn test_shared_memory_sysv() {
        let mut memory = SharedMemory::new("test_sysv_segment", 4096);
        
        // Test creation and attachment
        if memory.create_and_attach().is_ok() {
            assert!(memory.is_attached());
            assert_eq!(memory.size(), 4096);
            
            // Test writing and reading
            let test_data = b"Hello, shared memory!";
            if memory.write(0, test_data).is_ok() {
                let mut buffer = vec![0u8; test_data.len()];
                if memory.read(0, &mut buffer).is_ok() {
                    assert_eq!(&buffer, test_data);
                }
            }
            
            // Test zero fill
            if memory.zero_fill().is_ok() {
                let mut buffer = vec![0u8; 10];
                if memory.read(0, &mut buffer).is_ok() {
                    assert_eq!(buffer, vec![0u8; 10]);
                }
            }
            
            // Test stats
            if let Ok(stats) = memory.stats() {
                assert_eq!(stats.size, 4096);
            }
            
            // Cleanup
            let _ = memory.delete();
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_shared_memory_posix() {
        let config = SharedMemoryConfig {
            use_posix: true,
            size: 2048,
            ..Default::default()
        };
        let mut memory = SharedMemory::with_config("test_posix_segment", config);
        
        // Test creation and attachment
        if memory.create_and_attach().is_ok() {
            assert!(memory.is_attached());
            assert_eq!(memory.size(), 2048);
            
            // Test slice access
            if let Some(slice) = memory.as_mut_slice() {
                slice[0] = 42;
                slice[1] = 84;
            }
            
            if let Some(slice) = memory.as_slice() {
                assert_eq!(slice[0], 42);
                assert_eq!(slice[1], 84);
            }
            
            // Cleanup
            let _ = memory.delete();
        }
    }

    #[test]
    fn test_typed_shared_memory() {
        let result = SharedMemorySegment::<i32>::new("test_typed_segment", 100);
        if let Ok(mut segment) = result {
            assert_eq!(segment.capacity(), 100);
            
            if let Some(slice) = segment.as_mut_slice() {
                for (i, item) in slice.iter_mut().enumerate() {
                    *item = i as i32;
                }
            }
            
            if let Some(slice) = segment.as_slice() {
                assert_eq!(slice[0], 0);
                assert_eq!(slice[10], 10);
                assert_eq!(slice[99], 99);
            }
            
            // Cleanup
            let _ = segment.memory_mut().delete();
        }
    }

    #[cfg(windows)]
    #[test]
    fn test_shared_memory_windows() {
        let mut memory = SharedMemory::new("test_windows_segment", 4096);
        
        // Test creation and attachment
        if memory.create_and_attach().is_ok() {
            assert!(memory.is_attached());
            assert_eq!(memory.size(), 4096);
            
            // Test writing and reading
            let test_data = b"Hello, Windows shared memory!";
            if memory.write(0, test_data).is_ok() {
                let mut buffer = vec![0u8; test_data.len()];
                if memory.read(0, &mut buffer).is_ok() {
                    assert_eq!(&buffer, test_data);
                }
            }
            
            // Test slice access
            if let Some(slice) = memory.as_mut_slice() {
                slice[100] = 42;
                slice[200] = 84;
            }
            
            if let Some(slice) = memory.as_slice() {
                assert_eq!(slice[100], 42);
                assert_eq!(slice[200], 84);
            }
            
            // Test zero fill
            if memory.zero_fill().is_ok() {
                let mut buffer = vec![0u8; 10];
                if memory.read(0, &mut buffer).is_ok() {
                    assert_eq!(buffer, vec![0u8; 10]);
                }
            }
            
            // Cleanup
            let _ = memory.delete();
        }
    }

    #[test]
    fn test_memory_registry() {
        let registry = get_memory_registry();
        assert!(registry.read().is_ok());
        
        // Test cleanup
        assert!(cleanup_segments().is_ok());
    }
}
