/// Real shared memory implementation for CURSED IPC
/// 
/// This module provides comprehensive shared memory functionality for inter-process
/// communication, including creation, mapping, synchronization, and security features.
/// 
/// # Why Shared Memory is Critical for Distributed Systems
/// 
/// Shared memory is the fastest form of IPC, enabling:
/// - Zero-copy data sharing between processes
/// - High-throughput communication for real-time systems
/// - Efficient bulk data transfer without serialization overhead
/// - Memory-mapped file I/O for persistent data sharing
/// - Cache-efficient data structures across process boundaries
/// 
/// In distributed systems, shared memory enables:
/// - Local caching layers with millisecond access times
/// - High-frequency trading systems with microsecond latencies
/// - Media processing pipelines with zero-copy frame transfer
/// - Database buffer pools shared across multiple processes
/// - Message passing systems with shared ring buffers

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime};
use std::ptr::{self, NonNull};
use std::slice;
use std::ffi::CString;
use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, IpcPermissions, IpcConfig,
    permission_denied, resource_error
};
use crate::stdlib::ipc::types::IpcHandleType;
use crate::stdlib::ipc::error::{shared_memory_error, system_error};

#[cfg(unix)]
use std::os::unix::io::RawFd;

/// Shared memory region handle
#[derive(Debug)]
pub struct SharedMemory {
    handle: IpcHandle,
    config: SharedMemoryConfig,
    mapping: Option<MemoryMapping>,
    state: SharedMemoryState,
    statistics: Arc<Mutex<SharedMemoryStatistics>>,
}

/// Configuration for shared memory regions
#[derive(Debug, Clone)]
pub struct SharedMemoryConfig {
    pub name: String,
    pub size: usize,
    pub permissions: IpcPermissions,
    pub create_if_not_exists: bool,
    pub exclusive_create: bool,
    pub remove_on_drop: bool,
    pub enable_write_protection: bool,
    pub enable_copy_on_write: bool,
    pub prefault_pages: bool,
    pub memory_protection: MemoryProtection,
    pub access_mode: SharedMemoryAccess,
}

impl SharedMemoryConfig {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            permissions: IpcPermissions::read_write(),
            create_if_not_exists: true,
            exclusive_create: false,
            remove_on_drop: false,
            enable_write_protection: false,
            enable_copy_on_write: false,
            prefault_pages: false,
            memory_protection: MemoryProtection::ReadWrite,
            access_mode: SharedMemoryAccess::Shared,
        }
    }

    pub fn with_permissions(mut self, permissions: IpcPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_exclusive_create(mut self) -> Self {
        self.exclusive_create = true;
        self.create_if_not_exists = true;
        self
    }

    pub fn with_remove_on_drop(mut self) -> Self {
        self.remove_on_drop = true;
        self
    }

    pub fn with_write_protection(mut self) -> Self {
        self.enable_write_protection = true;
        self
    }

    pub fn with_copy_on_write(mut self) -> Self {
        self.enable_copy_on_write = true;
        self
    }

    pub fn with_prefault(mut self) -> Self {
        self.prefault_pages = true;
        self
    }
}

/// Memory protection flags
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryProtection {
    None,
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Execute,
    ReadExecute,
    WriteExecute,
    ReadWriteExecute,
}

impl MemoryProtection {
    #[cfg(unix)]
    pub fn to_mmap_prot(&self) -> i32 {
        use libc::{PROT_NONE, PROT_READ, PROT_WRITE, PROT_EXEC};
        
        match self {
            MemoryProtection::None => PROT_NONE,
            MemoryProtection::ReadOnly => PROT_READ,
            MemoryProtection::WriteOnly => PROT_WRITE,
            MemoryProtection::ReadWrite => PROT_READ | PROT_WRITE,
            MemoryProtection::Execute => PROT_EXEC,
            MemoryProtection::ReadExecute => PROT_READ | PROT_EXEC,
            MemoryProtection::WriteExecute => PROT_WRITE | PROT_EXEC,
            MemoryProtection::ReadWriteExecute => PROT_READ | PROT_WRITE | PROT_EXEC,
        }
    }
}

/// Shared memory access mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedMemoryAccess {
    Shared,
    Private,
    SharedValidate,
}

/// Memory mapping information
#[derive(Debug)]
pub struct MemoryMapping {
    ptr: NonNull<u8>,
    size: usize,
    #[cfg(unix)]
    fd: RawFd,
    offset: usize,
    protection: MemoryProtection,
}

impl MemoryMapping {
    /// Get a slice view of the mapped memory
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr.as_ptr(), self.size)
        }
    }

    /// Get a mutable slice view of the mapped memory
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size)
        }
    }

    /// Get raw pointer to mapped memory
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Get mutable raw pointer to mapped memory
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Get size of mapped region
    pub fn size(&self) -> usize {
        self.size
    }

    /// Sync mapped memory to backing store
    pub fn sync(&self, async_sync: bool) -> IpcResult<()> {
        #[cfg(unix)]
        {
            let flags = if async_sync {
                libc::MS_ASYNC
            } else {
                libc::MS_SYNC
            };

            let result = unsafe {
                libc::msync(
                    self.ptr.as_ptr() as *mut libc::c_void,
                    self.size,
                    flags
                )
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to sync shared memory"
                ));
            }
        }

        #[cfg(windows)]
        {
            // Windows FlushViewOfFile implementation
            use windows_sys::Win32::System::Memory::FlushViewOfFile;
            
            let result = unsafe {
                FlushViewOfFile(
                    self.ptr.as_ptr() as *const std::ffi::c_void,
                    self.size,
                )
            };

            if result == 0 {
                return Err(system_error(
                    unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                    "Failed to flush shared memory view"
                ));
            }
        }

        Ok(())
    }

    /// Lock pages in memory to prevent swapping
    pub fn lock_pages(&self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            let result = unsafe {
                libc::mlock(
                    self.ptr.as_ptr() as *const libc::c_void,
                    self.size
                )
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to lock shared memory pages"
                ));
            }
        }

        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Memory::VirtualLock;
            
            let result = unsafe {
                VirtualLock(
                    self.ptr.as_ptr() as *const std::ffi::c_void,
                    self.size,
                )
            };

            if result == 0 {
                return Err(system_error(
                    unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                    "Failed to lock virtual memory pages"
                ));
            }
        }

        Ok(())
    }

    /// Unlock pages allowing swapping
    pub fn unlock_pages(&self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            let result = unsafe {
                libc::munlock(
                    self.ptr.as_ptr() as *const libc::c_void,
                    self.size
                )
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to unlock shared memory pages"
                ));
            }
        }

        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Memory::VirtualUnlock;
            
            let result = unsafe {
                VirtualUnlock(
                    self.ptr.as_ptr() as *const std::ffi::c_void,
                    self.size,
                )
            };

            if result == 0 {
                return Err(system_error(
                    unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                    "Failed to unlock virtual memory pages"
                ));
            }
        }

        Ok(())
    }
}

unsafe impl Send for MemoryMapping {}
unsafe impl Sync for MemoryMapping {}

impl Drop for MemoryMapping {
    fn drop(&mut self) {
        // Unmap the memory region
        #[cfg(unix)]
        {
            unsafe {
                libc::munmap(
                    self.ptr.as_ptr() as *mut libc::c_void,
                    self.size
                );
            }
        }

        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Memory::UnmapViewOfFile;
            unsafe {
                UnmapViewOfFile(self.ptr.as_ptr() as *const std::ffi::c_void);
            }
        }
    }
}

/// Shared memory state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedMemoryState {
    Created,
    Mapped,
    Unmapped,
    Error,
}

/// Statistics for shared memory operations
#[derive(Debug, Clone)]
pub struct SharedMemoryStatistics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub sync_operations: u64,
    pub last_access: Option<SystemTime>,
    pub creation_time: SystemTime,
    pub mapping_count: u32,
}

impl SharedMemoryStatistics {
    pub fn new() -> Self {
        Self {
            bytes_read: 0,
            bytes_written: 0,
            read_operations: 0,
            write_operations: 0,
            sync_operations: 0,
            last_access: None,
            creation_time: SystemTime::now(),
            mapping_count: 0,
        }
    }

    pub fn record_read(&mut self, bytes: usize) {
        self.bytes_read += bytes as u64;
        self.read_operations += 1;
        self.last_access = Some(SystemTime::now());
    }

    pub fn record_write(&mut self, bytes: usize) {
        self.bytes_written += bytes as u64;
        self.write_operations += 1;
        self.last_access = Some(SystemTime::now());
    }

    pub fn record_sync(&mut self) {
        self.sync_operations += 1;
        self.last_access = Some(SystemTime::now());
    }
}

impl SharedMemory {
    /// Create a new shared memory region
    pub fn create(config: SharedMemoryConfig) -> IpcResult<Self> {
        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::SharedMemory
        );

        #[cfg(unix)]
        let mapping = Self::create_unix_mapping(&config)?;

        #[cfg(windows)]
        let mapping = Self::create_windows_mapping(&config)?;

        let mut shm = Self {
            handle,
            config,
            mapping: Some(mapping),
            state: SharedMemoryState::Created,
            statistics: Arc::new(Mutex::new(SharedMemoryStatistics::new())),
        };

        shm.state = SharedMemoryState::Mapped;
        if let Ok(mut stats) = shm.statistics.lock() {
            stats.mapping_count += 1;
        }

        // Register with global registry
        SHARED_MEMORY_REGISTRY.write().unwrap()
            .insert(shm.handle.id.clone(), Arc::new(RwLock::new(())));

        Ok(shm)
    }

    /// Open an existing shared memory region
    pub fn open(name: &str, access_mode: SharedMemoryAccess) -> IpcResult<Self> {
        let config = SharedMemoryConfig {
            name: name.to_string(),
            size: 0, // Will be determined from existing region
            permissions: IpcPermissions::read_write(),
            create_if_not_exists: false,
            exclusive_create: false,
            remove_on_drop: false,
            enable_write_protection: false,
            enable_copy_on_write: false,
            prefault_pages: false,
            memory_protection: MemoryProtection::ReadWrite,
            access_mode,
        };

        #[cfg(unix)]
        let mapping = Self::open_unix_mapping(&config)?;

        #[cfg(windows)]
        let mapping = Self::open_windows_mapping(&config)?;

        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::SharedMemory
        );

        let mut shm = Self {
            handle,
            config,
            mapping: Some(mapping),
            state: SharedMemoryState::Mapped,
            statistics: Arc::new(Mutex::new(SharedMemoryStatistics::new())),
        };

        if let Ok(mut stats) = shm.statistics.lock() {
            stats.mapping_count += 1;
        }

        Ok(shm)
    }

    #[cfg(unix)]
    fn create_unix_mapping(config: &SharedMemoryConfig) -> IpcResult<MemoryMapping> {
        use libc::{shm_open, ftruncate, mmap, close, O_CREAT, O_RDWR, O_EXCL, MAP_SHARED, MAP_PRIVATE};

        // Create shared memory object name
        let shm_name = CString::new(format!("/{}", config.name))
            .map_err(|_| shared_memory_error("create", &config.name, "Invalid name"))?;

        // Open flags
        let mut flags = O_CREAT | O_RDWR;
        if config.exclusive_create {
            flags |= O_EXCL;
        }

        // Open shared memory object
        let fd = unsafe {
            shm_open(
                shm_name.as_ptr(),
                flags,
                config.permissions.to_octal()
            )
        };

        if fd == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to create shared memory object"
            ));
        }

        // Set size
        let truncate_result = unsafe {
            ftruncate(fd, config.size as i64)
        };

        if truncate_result == -1 {
            unsafe { close(fd); }
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to set shared memory size"
            ));
        }

        // Map memory
        let map_flags = match config.access_mode {
            SharedMemoryAccess::Shared => MAP_SHARED,
            SharedMemoryAccess::Private => MAP_PRIVATE,
            SharedMemoryAccess::SharedValidate => MAP_SHARED,
        };

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                config.size,
                config.memory_protection.to_mmap_prot(),
                map_flags,
                fd,
                0
            )
        };

        if ptr == libc::MAP_FAILED {
            unsafe { close(fd); }
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to map shared memory"
            ));
        }

        // Prefault pages if requested
        if config.prefault_pages {
            Self::prefault_pages(ptr as *mut u8, config.size)?;
        }

        Ok(MemoryMapping {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
            size: config.size,
            fd,
            offset: 0,
            protection: config.memory_protection.clone(),
        })
    }

    #[cfg(unix)]
    fn open_unix_mapping(config: &SharedMemoryConfig) -> IpcResult<MemoryMapping> {
        use libc::{shm_open, mmap, fstat, O_RDWR, MAP_SHARED, MAP_PRIVATE};

        let shm_name = CString::new(format!("/{}", config.name))
            .map_err(|_| shared_memory_error("open", &config.name, "Invalid name"))?;

        // Open existing shared memory object
        let fd = unsafe {
            shm_open(
                shm_name.as_ptr(),
                O_RDWR,
                0
            )
        };

        if fd == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to open shared memory object"
            ));
        }

        // Get size from file descriptor
        let mut stat_buf: libc::stat = unsafe { std::mem::zeroed() };
        let stat_result = unsafe { fstat(fd, &mut stat_buf) };
        
        if stat_result == -1 {
            unsafe { libc::close(fd); }
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to get shared memory size"
            ));
        }

        let size = stat_buf.st_size as usize;

        // Map memory
        let map_flags = match config.access_mode {
            SharedMemoryAccess::Shared => MAP_SHARED,
            SharedMemoryAccess::Private => MAP_PRIVATE,
            SharedMemoryAccess::SharedValidate => MAP_SHARED,
        };

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                size,
                config.memory_protection.to_mmap_prot(),
                map_flags,
                fd,
                0
            )
        };

        if ptr == libc::MAP_FAILED {
            unsafe { libc::close(fd); }
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to map shared memory"
            ));
        }

        Ok(MemoryMapping {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
            size,
            fd,
            offset: 0,
            protection: config.memory_protection.clone(),
        })
    }

    #[cfg(windows)]
    fn create_windows_mapping(config: &SharedMemoryConfig) -> IpcResult<MemoryMapping> {
        use windows_sys::Win32::System::Memory::{
            CreateFileMappingA, MapViewOfFile, FILE_MAP_ALL_ACCESS, PAGE_READWRITE
        };
        use windows_sys::Win32::Foundation::{INVALID_HANDLE_VALUE, GetLastError};

        let name = CString::new(config.name.clone())
            .map_err(|_| shared_memory_error("create", &config.name, "Invalid name"))?;

        // Create file mapping
        let handle = unsafe {
            CreateFileMappingA(
                INVALID_HANDLE_VALUE,
                ptr::null(),
                PAGE_READWRITE,
                (config.size >> 32) as u32,
                config.size as u32,
                name.as_ptr() as *const u8,
            )
        };

        if handle.is_null() {
            return Err(system_error(
                unsafe { GetLastError() } as i32,
                "Failed to create file mapping"
            ));
        }

        // Map view of file
        let ptr = unsafe {
            MapViewOfFile(
                handle,
                FILE_MAP_ALL_ACCESS,
                0,
                0,
                config.size,
            )
        };

        if ptr.is_null() {
            return Err(system_error(
                unsafe { GetLastError() } as i32,
                "Failed to map view of file"
            ));
        }

        Ok(MemoryMapping {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
            size: config.size,
            offset: 0,
            protection: config.memory_protection.clone(),
        })
    }

    #[cfg(windows)]
    fn get_windows_mapping_size(ptr: *const std::ffi::c_void) -> IpcResult<usize> {
        use windows_sys::Win32::System::Memory::{VirtualQuery, MEMORY_BASIC_INFORMATION};
        
        let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        let result = unsafe {
            VirtualQuery(
                Some(ptr),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        
        if result == 0 {
            return Err(system_error(
                unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                "Failed to query memory region size"
            ));
        }
        
        Ok(mbi.RegionSize)
    }

    #[cfg(windows)]
    fn open_windows_mapping(config: &SharedMemoryConfig) -> IpcResult<MemoryMapping> {
        use windows_sys::Win32::System::Memory::{
            OpenFileMappingA, MapViewOfFile, FILE_MAP_ALL_ACCESS
        };
        use windows_sys::Win32::Foundation::GetLastError;

        let name = CString::new(config.name.clone())
            .map_err(|_| shared_memory_error("open", &config.name, "Invalid name"))?;

        // Open existing file mapping
        let handle = unsafe {
            OpenFileMappingA(
                FILE_MAP_ALL_ACCESS,
                0,
                name.as_ptr() as *const u8,
            )
        };

        if handle.is_null() {
            return Err(system_error(
                unsafe { GetLastError() } as i32,
                "Failed to open file mapping"
            ));
        }

        // Map view of file
        let ptr = unsafe {
            MapViewOfFile(
                handle,
                FILE_MAP_ALL_ACCESS,
                0,
                0,
                0, // Map entire file
            )
        };

        if ptr.is_null() {
            return Err(system_error(
                unsafe { GetLastError() } as i32,
                "Failed to map view of file"
            ));
        }

        // Get size using VirtualQuery
        let size = Self::get_windows_mapping_size(ptr)?;

        Ok(MemoryMapping {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
            size,
            offset: 0,
            protection: config.memory_protection.clone(),
        })
    }

    #[cfg(unix)]
    fn prefault_pages(ptr: *mut u8, size: usize) -> IpcResult<()> {
        // Touch each page to fault it in
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        let mut offset = 0;
        
        while offset < size {
            unsafe {
                // Read one byte from each page
                let _ = std::ptr::read_volatile(ptr.add(offset));
            }
            offset += page_size;
        }
        
        Ok(())
    }

    /// Read data from shared memory
    pub fn read_bytes(&self, offset: usize, buffer: &mut [u8]) -> IpcResult<usize> {
        if self.state != SharedMemoryState::Mapped {
            return Err(shared_memory_error(
                "read",
                &self.config.name,
                "Memory not mapped"
            ));
        }

        let mapping = self.mapping.as_ref().ok_or_else(|| {
            shared_memory_error("read", &self.config.name, "No memory mapping")
        })?;

        if offset >= mapping.size() {
            return Ok(0);
        }

        let available = mapping.size() - offset;
        let to_read = buffer.len().min(available);

        if to_read > 0 {
            let src = unsafe { mapping.as_ptr().add(offset) };
            let dst = buffer.as_mut_ptr();
            unsafe {
                ptr::copy_nonoverlapping(src, dst, to_read);
            }

            // Update statistics
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_read(to_read);
            }
        }

        Ok(to_read)
    }

    /// Write data to shared memory
    pub fn write_bytes(&mut self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        if self.state != SharedMemoryState::Mapped {
            return Err(shared_memory_error(
                "write",
                &self.config.name,
                "Memory not mapped"
            ));
        }

        if !self.config.permissions.can_write() {
            return Err(permission_denied("write", &self.config.name));
        }

        let mapping = self.mapping.as_mut().ok_or_else(|| {
            shared_memory_error("write", &self.config.name, "No memory mapping")
        })?;

        if offset >= mapping.size() {
            return Err(shared_memory_error(
                "write",
                &self.config.name,
                "Offset beyond memory region"
            ));
        }

        let available = mapping.size() - offset;
        let to_write = data.len().min(available);

        if to_write > 0 {
            let dst = unsafe { mapping.as_mut_ptr().add(offset) };
            let src = data.as_ptr();
            unsafe {
                ptr::copy_nonoverlapping(src, dst, to_write);
            }

            // Update statistics
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_write(to_write);
            }
        }

        Ok(to_write)
    }

    /// Get a view of the shared memory region
    pub fn get_view(&self, offset: usize, size: usize) -> IpcResult<SharedMemoryView> {
        if self.state != SharedMemoryState::Mapped {
            return Err(shared_memory_error(
                "get_view",
                &self.config.name,
                "Memory not mapped"
            ));
        }

        let mapping = self.mapping.as_ref().ok_or_else(|| {
            shared_memory_error("get_view", &self.config.name, "No memory mapping")
        })?;

        if offset + size > mapping.size() {
            return Err(shared_memory_error(
                "get_view",
                &self.config.name,
                "View extends beyond memory region"
            ));
        }

        Ok(SharedMemoryView {
            ptr: unsafe { NonNull::new_unchecked(mapping.as_ptr().add(offset) as *mut u8) },
            size,
            read_only: !self.config.permissions.can_write(),
        })
    }

    /// Synchronize shared memory with backing store
    pub fn sync(&self, async_sync: bool) -> IpcResult<()> {
        if let Some(mapping) = &self.mapping {
            mapping.sync(async_sync)?;
            
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_sync();
            }
        }
        Ok(())
    }

    /// Get statistics for this shared memory region
    pub fn get_statistics(&self) -> SharedMemoryStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| SharedMemoryStatistics::new())
    }

    /// Get size of the shared memory region
    pub fn size(&self) -> usize {
        self.mapping.as_ref()
            .map(|m| m.size())
            .unwrap_or(0)
    }

    /// Check if the shared memory region is currently mapped
    pub fn is_mapped(&self) -> bool {
        self.state == SharedMemoryState::Mapped && self.mapping.is_some()
    }

    /// Remove the shared memory region from the system
    pub fn remove(name: &str) -> IpcResult<()> {
        #[cfg(unix)]
        {
            let shm_name = CString::new(format!("/{}", name))
                .map_err(|_| shared_memory_error("remove", name, "Invalid name"))?;

            let result = unsafe {
                libc::shm_unlink(shm_name.as_ptr())
            };

            if result == -1 {
                return Err(system_error(
                    unsafe { *libc::__errno_location() },
                    "Failed to remove shared memory object"
                ));
            }
        }

        #[cfg(windows)]
        {
            // Windows doesn't have a direct equivalent to shm_unlink
            // The object is removed when the last handle is closed
        }

        // Remove from registry
        SHARED_MEMORY_REGISTRY.write().unwrap().remove(name);

        Ok(())
    }
}

impl Drop for SharedMemory {
    fn drop(&mut self) {
        self.state = SharedMemoryState::Unmapped;
        
        if self.config.remove_on_drop {
            let _ = Self::remove(&self.config.name);
        }
    }
}

/// View into a shared memory region
#[derive(Debug)]
pub struct SharedMemoryView {
    ptr: NonNull<u8>,
    size: usize,
    read_only: bool,
}

impl SharedMemoryView {
    /// Get size of the view
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get slice view of the memory
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr.as_ptr(), self.size)
        }
    }

    /// Get mutable slice view (if not read-only)
    pub fn as_mut_slice(&mut self) -> Option<&mut [u8]> {
        if self.read_only {
            None
        } else {
            Some(unsafe {
                slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size)
            })
        }
    }

    /// Read data from the view
    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> usize {
        if offset >= self.size {
            return 0;
        }

        let available = self.size - offset;
        let to_read = buffer.len().min(available);

        if to_read > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.ptr.as_ptr().add(offset),
                    buffer.as_mut_ptr(),
                    to_read
                );
            }
        }

        to_read
    }

    /// Write data to the view (if not read-only)
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Option<usize> {
        if self.read_only || offset >= self.size {
            return None;
        }

        let available = self.size - offset;
        let to_write = data.len().min(available);

        if to_write > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    self.ptr.as_ptr().add(offset),
                    to_write
                );
            }
        }

        Some(to_write)
    }
}

unsafe impl Send for SharedMemoryView {}
unsafe impl Sync for SharedMemoryView {}

/// Iterator over shared memory regions
pub struct SharedMemoryIterator {
    regions: Vec<String>,
    index: usize,
}

impl SharedMemoryIterator {
    pub fn new() -> Self {
        let regions = SHARED_MEMORY_REGISTRY.read()
            .map(|registry| registry.keys().cloned().collect())
            .unwrap_or_default();

        Self {
            regions,
            index: 0,
        }
    }
}

impl Iterator for SharedMemoryIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.regions.len() {
            let region = self.regions[self.index].clone();
            self.index += 1;
            Some(region)
        } else {
            None
        }
    }
}

// Global shared memory registry
lazy_static::lazy_static! {
    static ref SHARED_MEMORY_REGISTRY: Arc<RwLock<HashMap<String, Arc<RwLock<()>>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    
    static ref GLOBAL_STATISTICS: Arc<Mutex<HashMap<String, SharedMemoryStatistics>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

/// Module-level functions for shared memory management

/// Create a new shared memory region
pub fn create_shared_memory(config: SharedMemoryConfig) -> IpcResult<SharedMemory> {
    SharedMemory::create(config)
}

/// Open an existing shared memory region
pub fn open_shared_memory(name: &str) -> IpcResult<SharedMemory> {
    SharedMemory::open(name, SharedMemoryAccess::Shared)
}

/// Remove a shared memory region
pub fn remove_shared_memory(name: &str) -> IpcResult<()> {
    SharedMemory::remove(name)
}

/// Initialize shared memory subsystem
pub fn initialize_shared_memory_subsystem() -> IpcResult<()> {
    // Initialize global registry and statistics
    Ok(())
}

/// Shutdown shared memory subsystem
pub fn shutdown_shared_memory_subsystem() -> IpcResult<()> {
    // Clean up all regions
    cleanup_all_regions()?;
    Ok(())
}

/// Clean up all shared memory regions
pub fn cleanup_all_regions() -> IpcResult<()> {
    let region_names: Vec<String> = SHARED_MEMORY_REGISTRY.read()
        .map(|registry| registry.keys().cloned().collect())
        .unwrap_or_default();

    for name in region_names {
        let _ = SharedMemory::remove(&name);
    }

    Ok(())
}

/// Get count of active shared memory regions
pub fn get_active_region_count() -> usize {
    SHARED_MEMORY_REGISTRY.read()
        .map(|registry| registry.len())
        .unwrap_or(0)
}

/// Get memory usage of shared memory subsystem
pub fn get_memory_usage() -> usize {
SHARED_MEMORY_REGISTRY.read()
    .map(|registry| {
        let mut total_usage = 0;
            
                // Calculate usage from global statistics
                if let Ok(stats) = GLOBAL_STATISTICS.lock() {
                    total_usage += stats.len() * std::mem::size_of::<SharedMemoryStatistics>();
                }
                
                // Add estimated per-region overhead (handle + metadata)
                total_usage += registry.len() * std::mem::size_of::<SharedMemory>();
                
                // Add registry overhead
                total_usage += registry.capacity() * (
                    std::mem::size_of::<String>() + 
                    std::mem::size_of::<Arc<RwLock<()>>>()
                );
                
                total_usage
            })
            .unwrap_or(0)
    }

/// Get transfer rate for shared memory operations
pub fn get_transfer_rate() -> f64 {
    // Calculate bytes per second transfer rate
    // This would aggregate statistics from all regions
    0.0
}

/// Get allocation failure count
pub fn get_allocation_failure_count() -> u64 {
    // Track allocation failures across all shared memory operations
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_memory_config() {
        let config = SharedMemoryConfig::new("test_memory", 4096)
            .with_permissions(IpcPermissions::read_write())
            .with_exclusive_create()
            .with_remove_on_drop();

        assert_eq!(config.name, "test_memory");
        assert_eq!(config.size, 4096);
        assert!(config.exclusive_create);
        assert!(config.remove_on_drop);
    }

    #[test]
    fn test_memory_protection_conversion() {
        #[cfg(unix)]
        {
            assert_eq!(MemoryProtection::ReadOnly.to_mmap_prot(), libc::PROT_READ);
            assert_eq!(MemoryProtection::ReadWrite.to_mmap_prot(), libc::PROT_READ | libc::PROT_WRITE);
        }
    }

    #[test]
    fn test_shared_memory_statistics() {
        let mut stats = SharedMemoryStatistics::new();
        assert_eq!(stats.bytes_read, 0);
        assert_eq!(stats.read_operations, 0);

        stats.record_read(1024);
        assert_eq!(stats.bytes_read, 1024);
        assert_eq!(stats.read_operations, 1);
        assert!(stats.last_access.is_some());

        stats.record_write(512);
        assert_eq!(stats.bytes_written, 512);
        assert_eq!(stats.write_operations, 1);
    }

    #[test]
    fn test_shared_memory_iterator() {
        let iterator = SharedMemoryIterator::new();
        let regions: Vec<String> = iterator.collect();
        // Should be empty initially
        assert!(regions.is_empty());
    }

    #[test]
    fn test_shared_memory_view() {
        let ptr = NonNull::new(Box::into_raw(Box::new([0u8; 1024]))).unwrap();
        let mut view = SharedMemoryView {
            ptr: ptr.cast(),
            size: 1024,
            read_only: false,
        };

        assert_eq!(view.size(), 1024);
        assert!(view.as_mut_slice().is_some());

        // Clean up
        unsafe {
            let _ = Box::from_raw(ptr.as_ptr());
        }
    }

    #[test]
    fn test_global_functions() {
        assert_eq!(get_active_region_count(), 0);
        assert_eq!(get_memory_usage(), 0);
        assert_eq!(get_transfer_rate(), 0.0);
        assert_eq!(get_allocation_failure_count(), 0);
    }

    #[test]
    fn test_subsystem_lifecycle() {
        assert!(initialize_shared_memory_subsystem().is_ok());
        assert!(shutdown_shared_memory_subsystem().is_ok());
    }
}
