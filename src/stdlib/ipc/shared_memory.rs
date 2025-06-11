/// Cross-platform shared memory implementation for IPC
use crate::stdlib::ipc::error::{IpcResult, shared_memory_error, system_error, permission_denied, resource_error, timeout_error};
use crate::stdlib::ipc::types::{SharedMemoryId, IpcPermissions, IpcConfig, ProcessId, IpcHandle, IpcHandleType};
use crate::stdlib::ipc::traits::{IpcChannel, IpcResource, ResourceInfo, ResourceUsageStats, ChannelStatistics};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, Instant};
use std::ptr::NonNull;
use std::slice;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::mem;

// Platform-specific imports
#[cfg(unix)]
use std::os::unix::io::RawFd;
#[cfg(windows)]
use std::os::windows::io::RawHandle;

/// Shared memory configuration with enhanced options
#[derive(Debug, Clone)]
pub struct SharedMemoryConfig {
    pub id: SharedMemoryId,
    pub size: usize,
    pub permissions: IpcPermissions,
    pub create_if_missing: bool,
    pub truncate_if_exists: bool,
    pub enable_sync: bool,
    pub sync_type: SyncType,
    pub access_mode: AccessMode,
    pub auto_cleanup: bool,
    pub page_alignment: bool,
}

/// Types of synchronization mechanisms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncType {
    None,
    Mutex,
    ReadWriteLock,
    Semaphore(u32), // Initial count
    Custom(String),
}

/// Memory access patterns for optimization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    Random,      // Random access pattern
    Sequential,  // Sequential reading/writing
    ReadMostly,  // Mostly read operations
    WriteMostly, // Mostly write operations
    Concurrent,  // High concurrency expected
}

impl SharedMemoryConfig {
    pub fn new(id: SharedMemoryId, size: usize) -> IpcResult<Self> {
        if id.is_empty() {
            return Err(shared_memory_error("create", &id, "ID cannot be empty"));
        }
        if size == 0 {
            return Err(shared_memory_error("create", &id, "Size cannot be zero"));
        }
        // Check size limits (1GB max for safety)
        if size > 1024 * 1024 * 1024 {
            return Err(shared_memory_error("create", &id, "Size exceeds maximum limit (1GB)"));
        }

        Ok(Self {
            id,
            size,
            permissions: IpcPermissions::read_write(),
            create_if_missing: true,
            truncate_if_exists: false,
            enable_sync: true,
            sync_type: SyncType::Mutex,
            access_mode: AccessMode::Random,
            auto_cleanup: true,
            page_alignment: true,
        })
    }

    pub fn with_permissions(mut self, permissions: IpcPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_sync_type(mut self, sync_type: SyncType) -> Self {
        self.sync_type = sync_type;
        self
    }

    pub fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.access_mode = access_mode;
        self
    }

    pub fn without_sync(mut self) -> Self {
        self.enable_sync = false;
        self.sync_type = SyncType::None;
        self
    }

    pub fn with_auto_cleanup(mut self, enabled: bool) -> Self {
        self.auto_cleanup = enabled;
        self
    }
}

/// Memory mapping information
#[derive(Debug)]
pub struct MemoryMapping {
    pub start_addr: usize, // Store as usize for thread safety
    pub size: usize,
    pub is_writable: bool,
    pub is_executable: bool,
    pub offset: usize,
}

impl MemoryMapping {
    /// Get a safe slice view of the memory
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.start_addr as *const u8, self.size) }
    }

    /// Get a mutable slice view of the memory (if writable)
    pub fn as_mut_slice(&mut self) -> IpcResult<&mut [u8]> {
        if !self.is_writable {
            return Err(permission_denied("write", "shared_memory"));
        }
        Ok(unsafe { slice::from_raw_parts_mut(self.start_addr as *mut u8, self.size) })
    }

    /// Read data from offset
    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> IpcResult<usize> {
        if offset >= self.size {
            return Err(resource_error("Offset exceeds memory region size"));
        }
        
        let available = self.size - offset;
        let to_read = buf.len().min(available);
        
        if to_read > 0 {
            let src = unsafe { (self.start_addr as *const u8).add(offset) };
            unsafe {
                std::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), to_read);
            }
        }
        
        Ok(to_read)
    }

    /// Write data at offset
    pub fn write_at(&mut self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        if !self.is_writable {
            return Err(permission_denied("write", "shared_memory"));
        }
        if offset >= self.size {
            return Err(resource_error("Offset exceeds memory region size"));
        }
        
        let available = self.size - offset;
        let to_write = data.len().min(available);
        
        if to_write > 0 {
            let dst = unsafe { (self.start_addr as *mut u8).add(offset) };
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), dst, to_write);
            }
        }
        
        Ok(to_write)
    }

    /// Check if address is within bounds
    pub fn contains_address(&self, addr: *const u8) -> bool {
        let start = self.start_addr;
        let end = start + self.size;
        let check_addr = addr as usize;
        check_addr >= start && check_addr < end
    }
}

/// Protection settings for memory regions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryProtection {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl MemoryProtection {
    pub fn read_only() -> Self {
        Self { read: true, write: false, execute: false }
    }

    pub fn read_write() -> Self {
        Self { read: true, write: true, execute: false }
    }

    pub fn none() -> Self {
        Self { read: false, write: false, execute: false }
    }

    #[cfg(unix)]
    pub fn to_mmap_prot(&self) -> libc::c_int {
        let mut prot = 0;
        if self.read { prot |= libc::PROT_READ; }
        if self.write { prot |= libc::PROT_WRITE; }
        if self.execute { prot |= libc::PROT_EXEC; }
        prot
    }

    #[cfg(windows)]
    pub fn to_page_protect(&self) -> u32 {
        match (self.read, self.write, self.execute) {
            (false, false, false) => 0x01, // PAGE_NOACCESS
            (true, false, false) => 0x02,  // PAGE_READONLY
            (true, true, false) => 0x04,   // PAGE_READWRITE
            (false, false, true) => 0x10,  // PAGE_EXECUTE
            (true, false, true) => 0x20,   // PAGE_EXECUTE_READ
            (true, true, true) => 0x40,    // PAGE_EXECUTE_READWRITE
            _ => 0x04, // Default to READ-write
        }
    }
}

/// Shared memory region with cross-platform implementation
#[derive(Debug)]
pub struct SharedMemoryRegion {
    config: SharedMemoryConfig,
    mapping: Option<MemoryMapping>,
    #[cfg(unix)]
    fd: Option<RawFd>,
    #[cfg(windows)]
    handle: Option<RawHandle>,
    is_creator: bool,
    is_attached: bool,
    created_at: SystemTime,
    last_accessed: Mutex<SystemTime>,
    ref_count: Arc<AtomicUsize>,
}

impl SharedMemoryRegion {
    /// Create a new shared memory region
    pub fn create(config: SharedMemoryConfig) -> IpcResult<Self> {
        let mut region = Self {
            config,
            mapping: None,
            #[cfg(unix)]
            fd: None,
            #[cfg(windows)]
            handle: None,
            is_creator: true,
            is_attached: false,
            created_at: SystemTime::now(),
            last_accessed: Mutex::new(SystemTime::now()),
            ref_count: Arc::new(AtomicUsize::new(1)),
        };

        region.create_platform_specific()?;
        Ok(region)
    }

    /// Open existing shared memory region
    pub fn open(id: &str, permissions: IpcPermissions) -> IpcResult<Self> {
        let config = SharedMemoryConfig {
            id: id.to_string(),
            size: 0, // Will be determined when mapping
            permissions,
            create_if_missing: false,
            truncate_if_exists: false,
            enable_sync: true,
            sync_type: SyncType::Mutex,
            access_mode: AccessMode::Random,
            auto_cleanup: false,
            page_alignment: true,
        };

        let mut region = Self {
            config,
            mapping: None,
            #[cfg(unix)]
            fd: None,
            #[cfg(windows)]
            handle: None,
            is_creator: false,
            is_attached: false,
            created_at: SystemTime::now(),
            last_accessed: Mutex::new(SystemTime::now()),
            ref_count: Arc::new(AtomicUsize::new(1)),
        };

        region.open_platform_specific()?;
        Ok(region)
    }

    /// Map memory region into current process
    pub fn map_memory(&mut self, protection: MemoryProtection) -> IpcResult<()> {
        if self.is_attached {
            return Err(shared_memory_error("map", &self.config.id, "Already mapped"));
        }

        self.map_platform_specific(protection)?;
        self.is_attached = true;
        Ok(())
    }

    /// Unmap memory region from current process
    pub fn unmap_memory(&mut self) -> IpcResult<()> {
        if !self.is_attached {
            return Ok(()); // Already unmapped
        }

        self.unmap_platform_specific()?;
        self.mapping = None;
        self.is_attached = false;
        Ok(())
    }

    /// Get memory mapping (must be mapped first)
    pub fn mapping(&self) -> IpcResult<&MemoryMapping> {
        self.mapping.as_ref()
            .ok_or_else(|| shared_memory_error("access", &self.config.id, "Memory not mapped"))
    }

    /// Get mutable memory mapping
    pub fn mapping_mut(&mut self) -> IpcResult<&mut MemoryMapping> {
        self.mapping.as_mut()
            .ok_or_else(|| shared_memory_error("access", &self.config.id, "Memory not mapped"))
    }

    /// Synchronize memory to persistent storage
    pub fn sync(&self) -> IpcResult<()> {
        if let Some(mapping) = &self.mapping {
            self.sync_platform_specific(mapping)?;
        }
        Ok(())
    }

    /// Get size of memory region
    pub fn size(&self) -> usize {
        self.config.size
    }

    /// Get ID of memory region
    pub fn id(&self) -> &str {
        &self.config.id
    }

    /// Check if this process created the region
    pub fn is_creator(&self) -> bool {
        self.is_creator
    }

    /// Check if memory is currently mapped
    pub fn is_mapped(&self) -> bool {
        self.is_attached && self.mapping.is_some()
    }

    /// Increment reference count
    pub fn add_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Decrement reference count
    pub fn release_ref(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst).saturating_sub(1)
    }

    /// Get current reference count
    pub fn ref_count(&self) -> usize {
        self.ref_count.load(Ordering::SeqCst)
    }

    // Platform-specific implementations
    #[cfg(unix)]
    fn create_platform_specific(&mut self) -> IpcResult<()> {
        use std::ffi::CString;
        use std::os::raw::c_char;

        // Create POSIX shared memory object
        let name = CString::new(format!("/{}", self.config.id))
            .map_err(|_| shared_memory_error("create", &self.config.id, "Invalid name"))?;

        let mut flags = libc::O_CREAT | libc::O_RDWR;
        if !self.config.truncate_if_exists {
            flags |= libc::O_EXCL;
        }

        let mode = self.config.permissions.to_octal() as libc::mode_t;

        let fd = unsafe {
            libc::shm_open(name.as_ptr(), flags, mode)
        };

        if fd == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_error(errno, &format!("shm_open failed for {}", self.config.id)));
        }

        // Set size of shared memory object
        let result = unsafe {
            libc::ftruncate(fd, self.config.size as libc::off_t)
        };

        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            unsafe { libc::close(fd); }
            return Err(system_error(errno, &format!("ftruncate failed for {}", self.config.id)));
        }

        self.fd = Some(fd);
        Ok(())
    }

    #[cfg(windows)]
    fn create_platform_specific(&mut self) -> IpcResult<()> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use std::ptr;

        let name: Vec<u16> = OsString::from(&self.config.id)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let size_high = (self.config.size >> 32) as u32;
        let size_low = self.config.size as u32;

        let handle = unsafe {
            winapi::um::memoryapi::CreateFileMappingW(
                winapi::um::handleapi::INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                winapi::um::winnt::PAGE_READWRITE,
                size_high,
                size_low,
                name.as_ptr(),
            )
        };

        if handle.is_null() {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(system_error(error as i32, &format!("CreateFileMappingW failed for {}", self.config.id)));
        }

        self.handle = Some(handle);
        Ok(())
    }

    #[cfg(unix)]
    fn open_platform_specific(&mut self) -> IpcResult<()> {
        use std::ffi::CString;

        let name = CString::new(format!("/{}", self.config.id))
            .map_err(|_| shared_memory_error("open", &self.config.id, "Invalid name"))?;

        let fd = unsafe {
            libc::shm_open(name.as_ptr(), libc::O_RDWR, 0)
        };

        if fd == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_error(errno, &format!("shm_open failed for {}", self.config.id)));
        }

        // Get size of existing shared memory
        let mut stat: libc::stat = unsafe { mem::zeroed() };
        let result = unsafe { libc::fstat(fd, &mut stat) };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            unsafe { libc::close(fd); }
            return Err(system_error(errno, "fstat failed"));
        }

        self.config.size = stat.st_size as usize;
        self.fd = Some(fd);
        Ok(())
    }

    #[cfg(windows)]
    fn open_platform_specific(&mut self) -> IpcResult<()> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        let name: Vec<u16> = OsString::from(&self.config.id)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let handle = unsafe {
            winapi::um::memoryapi::OpenFileMappingW(
                winapi::um::winnt::FILE_MAP_ALL_ACCESS,
                winapi::shared::minwindef::FALSE,
                name.as_ptr(),
            )
        };

        if handle.is_null() {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(system_error(error as i32, &format!("OpenFileMappingW failed for {}", self.config.id)));
        }

        // Get size (Windows doesn't provide direct way, we'll need to try mapping)
        let ptr = unsafe {
            winapi::um::memoryapi::MapViewOfFile(
                handle,
                winapi::um::winnt::FILE_MAP_ALL_ACCESS,
                0,
                0,
                0, // Map entire file
            )
        };

        if ptr.is_null() {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            unsafe { winapi::um::handleapi::CloseHandle(handle); }
            return Err(system_error(error as i32, "MapViewOfFile failed"));
        }

        // Query size
        let mut info: winapi::um::winnt::MEMORY_BASIC_INFORMATION = unsafe { mem::zeroed() };
        let result = unsafe {
            winapi::um::memoryapi::VirtualQuery(
                ptr,
                &mut info,
                mem::size_of::<winapi::um::winnt::MEMORY_BASIC_INFORMATION>(),
            )
        };

        if result == 0 {
            unsafe {
                winapi::um::memoryapi::UnmapViewOfFile(ptr);
                winapi::um::handleapi::CloseHandle(handle);
            }
            return Err(system_error(-1, "VirtualQuery failed"));
        }

        self.config.size = info.RegionSize;

        // Clean up temporary mapping
        unsafe {
            winapi::um::memoryapi::UnmapViewOfFile(ptr);
        }

        self.handle = Some(handle);
        Ok(())
    }

    #[cfg(unix)]
    fn map_platform_specific(&mut self, protection: MemoryProtection) -> IpcResult<()> {
        let fd = self.fd.ok_or_else(|| shared_memory_error("map", &self.config.id, "No file descriptor"))?;

        let prot = protection.to_mmap_prot();
        let flags = libc::MAP_SHARED;

        let addr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                self.config.size,
                prot,
                flags,
                fd,
                0,
            )
        };

        if addr == libc::MAP_FAILED {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_error(errno, "mmap failed"));
        }

        if addr.is_null() {
            return Err(shared_memory_error("map", &self.config.id, "Invalid mapping address"));
        }

        self.mapping = Some(MemoryMapping {
            start_addr: addr as usize,
            size: self.config.size,
            is_writable: protection.write,
            is_executable: protection.execute,
            offset: 0,
        });

        Ok(())
    }

    #[cfg(windows)]
    fn map_platform_specific(&mut self, protection: MemoryProtection) -> IpcResult<()> {
        let handle = self.handle.ok_or_else(|| shared_memory_error("map", &self.config.id, "No handle"))?;

        let access = if protection.write {
            winapi::um::winnt::FILE_MAP_ALL_ACCESS
        } else {
            winapi::um::winnt::FILE_MAP_READ
        };

        let addr = unsafe {
            winapi::um::memoryapi::MapViewOfFile(
                handle,
                access,
                0,
                0,
                self.config.size,
            )
        };

        if addr.is_null() {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(system_error(error as i32, "MapViewOfFile failed"));
        }

        self.mapping = Some(MemoryMapping {
            start_addr: addr as usize,
            size: self.config.size,
            is_writable: protection.write,
            is_executable: protection.execute,
            offset: 0,
        });

        Ok(())
    }

    #[cfg(unix)]
    fn unmap_platform_specific(&mut self) -> IpcResult<()> {
        if let Some(mapping) = &self.mapping {
            let result = unsafe {
                libc::munmap(mapping.start_addr as *mut libc::c_void, mapping.size)
            };
            if result == -1 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "munmap failed"));
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn unmap_platform_specific(&mut self) -> IpcResult<()> {
        if let Some(mapping) = &self.mapping {
            let result = unsafe {
                winapi::um::memoryapi::UnmapViewOfFile(mapping.start_addr as *mut libc::c_void)
            };
            if result == 0 {
                let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
                return Err(system_error(error as i32, "UnmapViewOfFile failed"));
            }
        }
        Ok(())
    }

    #[cfg(unix)]
    fn sync_platform_specific(&self, mapping: &MemoryMapping) -> IpcResult<()> {
        let result = unsafe {
            libc::msync(
                mapping.start_addr as *mut libc::c_void,
                mapping.size,
                libc::MS_SYNC,
            )
        };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_error(errno, "msync failed"));
        }
        Ok(())
    }

    #[cfg(windows)]
    fn sync_platform_specific(&self, mapping: &MemoryMapping) -> IpcResult<()> {
        let result = unsafe {
            winapi::um::memoryapi::FlushViewOfFile(
                mapping.start_addr as *const libc::c_void,
                mapping.size,
            )
        };
        if result == 0 {
            let error = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(system_error(error as i32, "FlushViewOfFile failed"));
        }
        Ok(())
    }
}

impl Drop for SharedMemoryRegion {
    fn drop(&mut self) {
        // Unmap memory if still mapped
        let _ = self.unmap_memory();

        // Close handles/file descriptors
        #[cfg(unix)]
        if let Some(fd) = self.fd {
            unsafe { libc::close(fd); }
        }

        #[cfg(windows)]
        if let Some(handle) = self.handle {
            unsafe { winapi::um::handleapi::CloseHandle(handle); }
        }

        // Remove shared memory object if we're the creator and auto_cleanup is enabled
        if self.is_creator && self.config.auto_cleanup {
            let _ = remove_shared_memory(&self.config.id);
        }
    }
}

/// High-level shared memory interface
#[derive(Debug)]
pub struct SharedMemory {
    config: SharedMemoryConfig,
    handle: IpcHandle,
    region: Option<SharedMemoryRegion>,
    is_open: bool,
    statistics: Mutex<ChannelStatistics>,
    usage_stats: Mutex<ResourceUsageStats>,
}

impl SharedMemory {
    /// Create new shared memory segment
    pub fn create(config: SharedMemoryConfig) -> IpcResult<Self> {
        let handle = IpcHandle::new(
            config.id.clone(),
            IpcHandleType::SharedMemory,
        );

        let region = SharedMemoryRegion::create(config.clone())?;

        Ok(Self {
            config,
            handle,
            region: Some(region),
            is_open: true,
            statistics: Mutex::new(ChannelStatistics::new()),
            usage_stats: Mutex::new(ResourceUsageStats::new()),
        })
    }

    /// Open existing shared memory segment
    pub fn open(id: &str, permissions: IpcPermissions) -> IpcResult<Self> {
        let region = SharedMemoryRegion::open(id, permissions.clone())?;
        let size = region.size();

        let config = SharedMemoryConfig {
            id: id.to_string(),
            size,
            permissions,
            create_if_missing: false,
            truncate_if_exists: false,
            enable_sync: true,
            sync_type: SyncType::Mutex,
            access_mode: AccessMode::Random,
            auto_cleanup: false,
            page_alignment: true,
        };

        let handle = IpcHandle::new(
            config.id.clone(),
            IpcHandleType::SharedMemory,
        );

        Ok(Self {
            config,
            handle,
            region: Some(region),
            is_open: true,
            statistics: Mutex::new(ChannelStatistics::new()),
            usage_stats: Mutex::new(ResourceUsageStats::new()),
        })
    }

    /// Map memory with specified protection
    pub fn map(&mut self, protection: MemoryProtection) -> IpcResult<()> {
        let region = self.region.as_mut()
            .ok_or_else(|| shared_memory_error("map", &self.config.id, "No region"))?;
        
        region.map_memory(protection)?;
        
        if let Ok(mut stats) = self.usage_stats.lock() {
            stats.record_operation();
        }
        
        Ok(())
    }

    /// Unmap memory
    pub fn unmap(&mut self) -> IpcResult<()> {
        let region = self.region.as_mut()
            .ok_or_else(|| shared_memory_error("unmap", &self.config.id, "No region"))?;
        
        region.unmap_memory()
    }

    /// Read data from memory at offset
    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> IpcResult<usize> {
        let region = self.region.as_ref()
            .ok_or_else(|| shared_memory_error("read", &self.config.id, "No region"))?;
        
        let mapping = region.mapping()?;
        let bytes_read = mapping.read_at(offset, buf)?;
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.record_read(bytes_read);
        }
        
        Ok(bytes_read)
    }

    /// Write data to memory at offset
    pub fn write_at(&mut self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        if !self.config.permissions.can_write() {
            return Err(permission_denied("write", &self.config.id));
        }

        let region = self.region.as_mut()
            .ok_or_else(|| shared_memory_error("write", &self.config.id, "No region"))?;
        
        let mapping = region.mapping_mut()?;
        let bytes_written = mapping.write_at(offset, data)?;
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.record_write(bytes_written);
        }
        
        Ok(bytes_written)
    }

    /// Get raw pointer to memory (unsafe)
    pub unsafe fn as_ptr(&self) -> IpcResult<*const u8> {
        let region = self.region.as_ref()
            .ok_or_else(|| shared_memory_error("access", &self.config.id, "No region"))?;
        
        let mapping = region.mapping()?;
        Ok(mapping.start_addr as *const u8)
    }

    /// Get mutable raw pointer to memory (unsafe)
    pub unsafe fn as_mut_ptr(&mut self) -> IpcResult<*mut u8> {
        if !self.config.permissions.can_write() {
            return Err(permission_denied("write", &self.config.id));
        }

        let region = self.region.as_mut()
            .ok_or_else(|| shared_memory_error("access", &self.config.id, "No region"))?;
        
        let mapping = region.mapping()?;
        Ok(mapping.start_addr as *mut u8)
    }

    /// Get memory slice (safe read-only access)
    pub fn as_slice(&self) -> IpcResult<&[u8]> {
        let region = self.region.as_ref()
            .ok_or_else(|| shared_memory_error("access", &self.config.id, "No region"))?;
        
        let mapping = region.mapping()?;
        Ok(mapping.as_slice())
    }

    /// Synchronize memory to storage
    pub fn sync(&self) -> IpcResult<()> {
        let region = self.region.as_ref()
            .ok_or_else(|| shared_memory_error("sync", &self.config.id, "No region"))?;
        
        region.sync()
    }

    /// Get size of shared memory
    pub fn size(&self) -> usize {
        self.config.size
    }

    /// Check if memory is currently mapped
    pub fn is_mapped(&self) -> bool {
        self.region.as_ref()
            .map(|r| r.is_mapped())
            .unwrap_or(false)
    }

    /// Get configuration
    pub fn config(&self) -> &SharedMemoryConfig {
        &self.config
    }
}

impl IpcChannel for SharedMemory {
    fn handle(&self) -> &IpcHandle {
        &self.handle
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn close(&mut self) -> IpcResult<()> {
        if let Some(mut region) = self.region.take() {
            region.unmap_memory()?;
        }
        self.is_open = false;
        Ok(())
    }

    fn permissions(&self) -> &IpcPermissions {
        &self.config.permissions
    }

    fn set_permissions(&mut self, permissions: IpcPermissions) -> IpcResult<()> {
        self.config.permissions = permissions;
        Ok(())
    }

    fn mode(&self) -> crate::stdlib::ipc::types::IpcMode {
        if self.config.permissions.can_write() {
            crate::stdlib::ipc::types::IpcMode::ReadWrite
        } else {
            crate::stdlib::ipc::types::IpcMode::ReadOnly
        }
    }

    fn statistics(&self) -> ChannelStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_default()
    }
}

impl IpcResource for SharedMemory {
    fn resource_info(&self) -> ResourceInfo {
        ResourceInfo {
            resource_type: "SharedMemory".to_string(),
            id: self.config.id.clone(),
            owner: None, // Would need system info to determine
            created_at: self.handle.created_at,
            size: Some(self.config.size),
            permissions: self.config.permissions.clone(),
        }
    }

    fn is_valid(&self) -> bool {
        self.is_open && self.region.is_some()
    }

    fn cleanup(&mut self) -> IpcResult<()> {
        self.close()
    }

    fn usage_stats(&self) -> ResourceUsageStats {
        self.usage_stats.lock()
            .map(|stats| stats.clone())
            .unwrap_or_default()
    }

    fn set_limits(&mut self, _limits: crate::stdlib::ipc::traits::ResourceLimits) -> IpcResult<()> {
        // Could implement memory usage limits here
        Ok(())
    }
}

/// Global shared memory manager
#[derive(Debug)]
pub struct SharedMemoryManager {
    regions: RwLock<HashMap<String, Arc<Mutex<SharedMemoryRegion>>>>,
    active_count: AtomicUsize,
    total_memory_usage: AtomicUsize,
    allocation_failures: AtomicUsize,
}

impl SharedMemoryManager {
    fn new() -> Self {
        Self {
            regions: RwLock::new(HashMap::new()),
            active_count: AtomicUsize::new(0),
            total_memory_usage: AtomicUsize::new(0),
            allocation_failures: AtomicUsize::new(0),
        }
    }

    pub fn global() -> &'static Self {
        static MANAGER: std::sync::OnceLock<SharedMemoryManager> = std::sync::OnceLock::new();
        MANAGER.get_or_init(SharedMemoryManager::new)
    }

    pub fn register_region(&self, id: String, region: SharedMemoryRegion) {
        let region_arc = Arc::new(Mutex::new(region));
        
        if let Ok(mut regions) = self.regions.write() {
            regions.insert(id, region_arc);
            self.active_count.fetch_add(1, Ordering::SeqCst);
            // Note: We'd need to track memory usage based on region size
        }
    }

    pub fn unregister_region(&self, id: &str) {
        if let Ok(mut regions) = self.regions.write() {
            if regions.remove(id).is_some() {
                self.active_count.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }

    pub fn get_region(&self, id: &str) -> Option<Arc<Mutex<SharedMemoryRegion>>> {
        self.regions.read().ok()?.get(id).cloned()
    }

    pub fn active_regions(&self) -> Vec<String> {
        self.regions.read()
            .map(|regions| regions.keys().cloned().collect())
            .unwrap_or_default()
    }
}

// Public API functions
pub fn create_shared_memory(config: SharedMemoryConfig) -> IpcResult<SharedMemory> {
    SharedMemory::create(config)
}

pub fn open_shared_memory(id: &str) -> IpcResult<SharedMemory> {
    SharedMemory::open(id, IpcPermissions::read_write())
}

pub fn remove_shared_memory(id: &str) -> IpcResult<()> {
    #[cfg(unix)]
    {
        use std::ffi::CString;
        let name = CString::new(format!("/{}", id))
            .map_err(|_| shared_memory_error("remove", id, "Invalid name"))?;
        
        let result = unsafe { libc::shm_unlink(name.as_ptr()) };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_error(errno, &format!("shm_unlink failed for {}", id)));
        }
    }

    #[cfg(windows)]
    {
        // Windows shared memory is automatically cleaned up when all handles are closed
        // No explicit removal needed
    }

    SharedMemoryManager::global().unregister_region(id);
    Ok(())
}

// Utility types and functions for module initialization
pub struct SharedMemoryAccess {
    region: Arc<Mutex<SharedMemoryRegion>>,
}

impl SharedMemoryAccess {
    pub fn new(region: Arc<Mutex<SharedMemoryRegion>>) -> Self {
        Self { region }
    }
    
    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> IpcResult<usize> {
        let region = self.region.lock().map_err(|_| shared_memory_error("read", "unknown", "Lock poisoned"))?;
        region.mapping()?.read_at(offset, buf)
    }
    
    pub fn write_at(&self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        let mut region = self.region.lock().map_err(|_| shared_memory_error("write", "unknown", "Lock poisoned"))?;
        region.mapping_mut()?.write_at(offset, data)
    }
}

pub struct SharedMemoryIterator {
    region: Arc<Mutex<SharedMemoryRegion>>,
    current_offset: usize,
    chunk_size: usize,
}

impl SharedMemoryIterator {
    pub fn new(region: Arc<Mutex<SharedMemoryRegion>>, chunk_size: usize) -> Self {
        Self {
            region,
            current_offset: 0,
            chunk_size,
        }
    }
    
    pub fn next_chunk(&mut self) -> IpcResult<Option<Vec<u8>>> {
        let region = self.region.lock().map_err(|_| shared_memory_error("read", "unknown", "Lock poisoned"))?;
        let mapping = region.mapping()?;
        
        if self.current_offset >= mapping.size {
            return Ok(None);
        }
        
        let remaining = mapping.size - self.current_offset;
        let chunk_size = self.chunk_size.min(remaining);
        
        let mut buffer = vec![0u8; chunk_size];
        let bytes_read = mapping.read_at(self.current_offset, &mut buffer)?;
        buffer.truncate(bytes_read);
        
        self.current_offset += bytes_read;
        Ok(Some(buffer))
    }
}

pub struct SharedMemoryView {
    region: Arc<Mutex<SharedMemoryRegion>>,
    offset: usize,
    size: usize,
}

impl SharedMemoryView {
    pub fn new(region: Arc<Mutex<SharedMemoryRegion>>, offset: usize, size: usize) -> IpcResult<Self> {
        {
            let region_guard = region.lock().map_err(|_| shared_memory_error("view", "unknown", "Lock poisoned"))?;
            let mapping = region_guard.mapping()?;
            
            if offset + size > mapping.size {
                return Err(shared_memory_error("view", "unknown", "View exceeds memory bounds"));
            }
        } // Drop the lock here
        
        Ok(Self { region, offset, size })
    }
    
    pub fn read(&self, buf: &mut [u8]) -> IpcResult<usize> {
        let region = self.region.lock().map_err(|_| shared_memory_error("read", "unknown", "Lock poisoned"))?;
        let mapping = region.mapping()?;
        
        let to_read = buf.len().min(self.size);
        mapping.read_at(self.offset, &mut buf[..to_read])
    }
    
    pub fn write(&self, data: &[u8]) -> IpcResult<usize> {
        let mut region = self.region.lock().map_err(|_| shared_memory_error("write", "unknown", "Lock poisoned"))?;
        let mapping = region.mapping_mut()?;
        
        let to_write = data.len().min(self.size);
        mapping.write_at(self.offset, &data[..to_write])
    }
}

pub fn initialize_shared_memory_subsystem() -> IpcResult<()> {
    // Initialize the global manager
    let _ = SharedMemoryManager::global();
    Ok(())
}

pub fn shutdown_shared_memory_subsystem() -> IpcResult<()> {
    // Clean up any remaining regions
    cleanup_all_regions()?;
    Ok(())
}

pub fn get_active_region_count() -> usize {
    SharedMemoryManager::global().active_count.load(Ordering::SeqCst)
}

pub fn cleanup_all_regions() -> IpcResult<()> {
    let manager = SharedMemoryManager::global();
    let region_ids = manager.active_regions();
    
    for id in region_ids {
        let _ = remove_shared_memory(&id); // Best effort cleanup
    }
    
    Ok(())
}

pub fn get_memory_usage() -> usize {
    SharedMemoryManager::global().total_memory_usage.load(Ordering::SeqCst)
}

pub fn get_transfer_rate() -> f64 {
    // This would need to be calculated based on actual usage statistics
    // For now, return a placeholder
    0.0
}

pub fn get_allocation_failure_count() -> u64 {
    SharedMemoryManager::global().allocation_failures.load(Ordering::SeqCst) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_memory_config() {
        let config = SharedMemoryConfig::new("test_mem".to_string(), 4096).unwrap();
        assert_eq!(config.id, "test_mem");
        assert_eq!(config.size, 4096);
        assert!(config.permissions.can_read());
        assert!(config.permissions.can_write());
    }

    #[test]
    fn test_memory_protection() {
        let rw = MemoryProtection::read_write();
        assert!(rw.read);
        assert!(rw.write);
        assert!(!rw.execute);

        let ro = MemoryProtection::read_only();
        assert!(ro.read);
        assert!(!ro.write);
        assert!(!ro.execute);
    }

    #[test]
    fn test_config_validation() {
        // Empty ID should fail
        let result = SharedMemoryConfig::new("".to_string(), 4096);
        assert!(result.is_err());

        // Zero size should fail
        let result = SharedMemoryConfig::new("test".to_string(), 0);
        assert!(result.is_err());

        // Size too large should fail
        let result = SharedMemoryConfig::new("test".to_string(), 2 * 1024 * 1024 * 1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_operations() {
        let manager = SharedMemoryManager::global();
        let initial_count = manager.active_regions().len();
        
        // The manager should be accessible
        assert!(manager.active_regions().len() >= initial_count);
    }

    #[test]
    fn test_sync_types() {
        assert_eq!(SyncType::None, SyncType::None);
        assert_ne!(SyncType::Mutex, SyncType::None);
        
        let sem = SyncType::Semaphore(5);
        assert!(matches!(sem, SyncType::Semaphore(5)));
    }

    #[test]
    fn test_access_modes() {
        assert_eq!(AccessMode::Random, AccessMode::Random);
        assert_ne!(AccessMode::Sequential, AccessMode::Random);
    }

    // Note: Platform-specific tests would require actual shared memory creation
    // which might not work in all test environments. These would be integration tests.
}
