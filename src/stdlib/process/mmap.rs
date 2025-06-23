/// Memory-mapped files implementation for CURSED
/// 
/// This module provides comprehensive memory mapping functionality including:
/// - Cross-platform memory mapping (Windows/Unix)
/// - Safe mmap/munmap operations with error handling
/// - Multiple mapping types (read-only, read-write, copy-on-write)
/// - Anonymous mappings for IPC shared memory
/// - File-backed mappings for large file processing
/// - Memory protection and access control
/// - Thread-safe operations with proper synchronization
/// - NUMA-aware allocation and memory advisory operations

use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::fs::File;
use std::os::fd::{AsRawFd, RawFd};
use crate::error::CursedError;

#[cfg(unix)]
use std::os::unix::io::AsRawFd as UnixAsRawFd;
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};

/// Memory mapping protection flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionFlags {
    /// No access allowed
    None,
    /// Read access only
    Read,
    /// Write access only (rare)
    Write,
    /// Read and write access
    ReadWrite,
    /// Execute access only
    Execute,
    /// Read and execute access
    ReadExecute,
    /// Write and execute access (dangerous)
    WriteExecute,
    /// Full access (read, write, execute)
    All,
}

/// Memory mapping type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingType {
    /// Private copy-on-write mapping
    Private,
    /// Shared mapping (changes visible to other processes)
    Shared,
    /// Anonymous mapping (not backed by file)
    Anonymous,
    /// Fixed mapping at specific address
    Fixed,
}

/// Memory advisory hints for optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryAdvice {
    /// Normal random access pattern
    Normal,
    /// Sequential access pattern
    Sequential,
    /// Random access pattern
    Random,
    /// Will need these pages soon
    WillNeed,
    /// Don't need these pages
    DontNeed,
    /// Don't dump these pages in core dumps
    DontDump,
    /// Do dump these pages in core dumps
    DoDump,
}

/// Sync operation type for flushing changes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncType {
    /// Synchronous flush (blocks until complete)
    Sync,
    /// Asynchronous flush (returns immediately)
    Async,
    /// Invalidate cached data
    Invalidate,
}

/// Memory mapping configuration
#[derive(Debug, Clone)]
pub struct MmapConfig {
    pub protection: ProtectionFlags,
    pub mapping_type: MappingType,
    pub offset: u64,
    pub length: usize,
    pub addr_hint: Option<*mut u8>,
    pub numa_node: Option<u32>,
    pub populate_pages: bool,
    pub lock_pages: bool,
    pub transparent_hugepages: bool,
}

impl Default for MmapConfig {
    fn default() -> Self {
        Self {
            protection: ProtectionFlags::Read,
            mapping_type: MappingType::Private,
            offset: 0,
            length: 0,
            addr_hint: None,
            numa_node: None,
            populate_pages: false,
            lock_pages: false,
            transparent_hugepages: false,
        }
    }
}

/// Memory-mapped region handle
pub struct MmapHandle {
    ptr: *mut u8,
    length: usize,
    protection: ProtectionFlags,
    mapping_type: MappingType,
    file_descriptor: Option<RawFd>,
    #[cfg(windows)]
    file_handle: Option<RawHandle>,
    #[cfg(windows)]
    mapping_handle: Option<RawHandle>,
    is_locked: bool,
    reference_count: Arc<Mutex<u32>>,
}

unsafe impl Send for MmapHandle {}
unsafe impl Sync for MmapHandle {}

impl MmapHandle {
    /// Create a new memory mapping handle
    fn new(
        ptr: *mut u8,
        length: usize,
        protection: ProtectionFlags,
        mapping_type: MappingType,
        file_descriptor: Option<RawFd>,
    ) -> Self {
        Self {
            ptr,
            length,
            protection,
            mapping_type,
            file_descriptor,
            #[cfg(windows)]
            file_handle: None,
            #[cfg(windows)]
            mapping_handle: None,
            is_locked: false,
            reference_count: Arc::new(Mutex::new(1)),
        }
    }

    /// Get the raw pointer to the mapped memory
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    /// Get the length of the mapping
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if the mapping is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Get a slice view of the mapped memory (read-only)
    pub fn as_slice(&self) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }
        
        if !self.can_read() {
            return Err(CursedError::Memory("Memory mapping not readable".to_string()));
        }

        Ok(unsafe { std::slice::from_raw_parts(self.ptr, self.length) })
    }

    /// Get a mutable slice view of the mapped memory
    pub fn as_mut_slice(&mut self) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }
        
        if !self.can_write() {
            return Err(CursedError::Memory("Memory mapping not writable".to_string()));
        }

        Ok(unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length) })
    }

    /// Check if the mapping allows reading
    pub fn can_read(&self) -> bool {
        matches!(
            self.protection,
            ProtectionFlags::Read
                | ProtectionFlags::ReadWrite
                | ProtectionFlags::ReadExecute
                | ProtectionFlags::All
        )
    }

    /// Check if the mapping allows writing
    pub fn can_write(&self) -> bool {
        matches!(
            self.protection,
            ProtectionFlags::Write
                | ProtectionFlags::ReadWrite
                | ProtectionFlags::WriteExecute
                | ProtectionFlags::All
        )
    }

    /// Check if the mapping allows execution
    pub fn can_execute(&self) -> bool {
        matches!(
            self.protection,
            ProtectionFlags::Execute
                | ProtectionFlags::ReadExecute
                | ProtectionFlags::WriteExecute
                | ProtectionFlags::All
        )
    }

    /// Flush changes to disk synchronously
    pub fn sync(&self, sync_type: SyncType) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }

        #[cfg(unix)]
        {
            let flags = match sync_type {
                SyncType::Sync => libc::MS_SYNC,
                SyncType::Async => libc::MS_ASYNC,
                SyncType::Invalidate => libc::MS_INVALIDATE,
            };

            let result = unsafe { libc::msync(self.ptr as *mut libc::c_void, self.length, flags) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to sync memory mapping: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::FlushViewOfFile;
            use winapi::um::fileapi::FlushFileBuffers;

            let result = unsafe { FlushViewOfFile(self.ptr as *const winapi::ccrate::types::c_void, self.length) };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to flush view of file: {}",
                    std::io::Error::last_os_error()
                )));
            }

            // Also flush file buffers if we have a file handle
            if let Some(handle) = self.file_handle {
                let result = unsafe { FlushFileBuffers(handle as *mut winapi::ccrate::types::c_void) };
                if result == 0 {
                    return Err(CursedError::Memory(format!(
                        "Failed to flush file buffers: {}",
                        std::io::Error::last_os_error()
                    )));
                }
            }
        }

        Ok(())
    }

    /// Change protection flags for the mapping
    pub fn protect(&mut self, new_protection: ProtectionFlags) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }

        #[cfg(unix)]
        {
            let prot = protection_to_unix_flags(new_protection);
            let result = unsafe { libc::mprotect(self.ptr as *mut libc::c_void, self.length, prot) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to change memory protection: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualProtect;
            use winapi::um::winnt::{PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE};

            let protect = match new_protection {
                ProtectionFlags::None => winapi::um::winnt::PAGE_NOACCESS,
                ProtectionFlags::Read => PAGE_READONLY,
                ProtectionFlags::Write => PAGE_READWRITE, // Windows doesn't support write-only
                ProtectionFlags::ReadWrite => PAGE_READWRITE,
                ProtectionFlags::Execute => winapi::um::winnt::PAGE_EXECUTE,
                ProtectionFlags::ReadExecute => PAGE_EXECUTE_READ,
                ProtectionFlags::WriteExecute => PAGE_EXECUTE_READWRITE,
                ProtectionFlags::All => PAGE_EXECUTE_READWRITE,
            };

            let mut old_protect = 0;
            let result = unsafe {
                VirtualProtect(
                    self.ptr as *mut winapi::ccrate::types::c_void,
                    self.length,
                    protect,
                    &mut old_protect,
                )
            };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to change memory protection: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        self.protection = new_protection;
        Ok(())
    }

    /// Lock pages in physical memory
    pub fn lock_pages(&mut self) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }

        if self.is_locked {
            return Ok(()); // Already locked
        }

        #[cfg(unix)]
        {
            let result = unsafe { libc::mlock(self.ptr as *const libc::c_void, self.length) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to lock pages: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualLock;

            let result = unsafe { VirtualLock(self.ptr as *mut winapi::ccrate::types::c_void, self.length) };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to lock pages: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        self.is_locked = true;
        Ok(())
    }

    /// Unlock pages from physical memory
    pub fn unlock_pages(&mut self) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }

        if !self.is_locked {
            return Ok(()); // Not locked
        }

        #[cfg(unix)]
        {
            let result = unsafe { libc::munlock(self.ptr as *const libc::c_void, self.length) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to unlock pages: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualUnlock;

            let result = unsafe { VirtualUnlock(self.ptr as *mut winapi::ccrate::types::c_void, self.length) };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to unlock pages: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        self.is_locked = false;
        Ok(())
    }

    /// Provide memory usage advice to the kernel
    pub fn advise(&self, advice: MemoryAdvice) -> Result<(), Error> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        }

        #[cfg(unix)]
        {
            let advice_flag = match advice {
                MemoryAdvice::Normal => libc::MADV_NORMAL,
                MemoryAdvice::Sequential => libc::MADV_SEQUENTIAL,
                MemoryAdvice::Random => libc::MADV_RANDOM,
                MemoryAdvice::WillNeed => libc::MADV_WILLNEED,
                MemoryAdvice::DontNeed => libc::MADV_DONTNEED,
                #[cfg(target_os = "linux")]
                MemoryAdvice::DontDump => libc::MADV_DONTDUMP,
                #[cfg(target_os = "linux")]
                MemoryAdvice::DoDump => libc::MADV_DODUMP,
                #[cfg(not(target_os = "linux"))]
                MemoryAdvice::DontDump => return Ok(()), // Not supported, ignore
                #[cfg(not(target_os = "linux"))]
                MemoryAdvice::DoDump => return Ok(()), // Not supported, ignore
            };

            let result = unsafe { libc::madvise(self.ptr as *mut libc::c_void, self.length, advice_flag) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    "Failed to provide memory advice: {}",
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            // Windows doesn't have direct equivalent to madvise
            // We can use PrefetchVirtualMemory for WillNeed
            match advice {
                MemoryAdvice::WillNeed => {
                    // Use PrefetchVirtualMemory if available (Windows 8+)
                    // For now, we'll just return Ok as this is advisory
                    return Ok(());
                }
                _ => {
                    // Other advice types are not directly supported on Windows
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    /// Clone the handle (increases reference count)
    pub fn clone_handle(&self) -> MmapHandle {
        let mut ref_count = self.reference_count.lock().unwrap();
        *ref_count += 1;

        MmapHandle {
            ptr: self.ptr,
            length: self.length,
            protection: self.protection,
            mapping_type: self.mapping_type,
            file_descriptor: self.file_descriptor,
            #[cfg(windows)]
            file_handle: self.file_handle,
            #[cfg(windows)]
            mapping_handle: self.mapping_handle,
            is_locked: self.is_locked,
            reference_count: Arc::clone(&self.reference_count),
        }
    }
}

impl Drop for MmapHandle {
    fn drop(&mut self) {
        let should_unmap = {
            let mut ref_count = self.reference_count.lock().unwrap();
            *ref_count -= 1;
            *ref_count == 0
        };

        if should_unmap && !self.ptr.is_null() {
            // Unlock pages if they were locked
            if self.is_locked {
                let _ = self.unlock_pages();
            }

            // Unmap the memory
            #[cfg(unix)]
            {
                unsafe {
                    libc::munmap(self.ptr as *mut libc::c_void, self.length);
                }
            }

            #[cfg(windows)]
            {
                use winapi::um::memoryapi::UnmapViewOfFile;
                use winapi::um::handleapi::CloseHandle;

                unsafe {
                    UnmapViewOfFile(self.ptr as *const winapi::ccrate::types::c_void);
                    
                    if let Some(handle) = self.mapping_handle {
                        CloseHandle(handle as *mut winapi::ccrate::types::c_void);
                    }
                }
            }
        }
    }
}

/// Memory mapping manager for coordinating multiple mappings
pub struct MmapManager {
    mappings: RwLock<HashMap<usize, Arc<MmapHandle>>>,
    next_mapping_id: Mutex<u64>,
}

impl MmapManager {
    /// Create a new memory mapping manager
    pub fn new() -> Self {
        Self {
            mappings: RwLock::new(HashMap::new()),
            next_mapping_id: Mutex::new(0),
        }
    }

    /// Create a file-backed memory mapping
    pub fn map_file(
        &self,
        file: &File,
        config: MmapConfig,
    ) -> Result<(), Error> {
        if config.length == 0 {
            return Err(CursedError::InvalidInput("Mapping length cannot be zero".to_string()));
        }

        #[cfg(unix)]
        {
            let fd = file.as_raw_fd();
            let prot = protection_to_unix_flags(config.protection);
            let flags = mapping_type_to_unix_flags(config.mapping_type);

            let ptr = unsafe {
                libc::mmap(
                    config.addr_hint.unwrap_or(ptr::null_mut()) as *mut libc::c_void,
                    config.length,
                    prot,
                    flags,
                    fd,
                    config.offset as libc::off_t,
                )
            };

            if ptr == libc::MAP_FAILED {
                return Err(CursedError::Memory(format!(
                    "Failed to create memory mapping: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let handle = Arc::new(MmapHandle::new(
                ptr as *mut u8,
                config.length,
                config.protection,
                config.mapping_type,
                Some(fd),
            ));

            // Apply additional configuration
            if config.populate_pages {
                let _ = handle.advise(MemoryAdvice::WillNeed);
            }

            if config.lock_pages {
                let mut mutable_handle = Arc::try_unwrap(handle).map_err(|_| {
                    CursedError::Memory("Failed to get mutable handle for locking".to_string())
                })?;
                mutable_handle.lock_pages()?;
                let handle = Arc::new(mutable_handle);
            }

            let mut mappings = self.mappings.write().unwrap();
            mappings.insert(handle.as_ptr() as usize, Arc::clone(&handle));

            Ok(handle)
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile};
            use winapi::um::winnt::{PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE};
            use winapi::um::fileapi::FILE_MAP_READ;

            let file_handle = file.as_raw_handle();
            
            let protect = match config.protection {
                ProtectionFlags::None => return Err(CursedError::InvalidInput("Cannot map with no protection".to_string())),
                ProtectionFlags::Read => PAGE_READONLY,
                ProtectionFlags::Write => PAGE_READWRITE,
                ProtectionFlags::ReadWrite => PAGE_READWRITE,
                ProtectionFlags::Execute => winapi::um::winnt::PAGE_EXECUTE,
                ProtectionFlags::ReadExecute => PAGE_EXECUTE_READ,
                ProtectionFlags::WriteExecute => PAGE_EXECUTE_READWRITE,
                ProtectionFlags::All => PAGE_EXECUTE_READWRITE,
            };

            let mapping_handle = unsafe {
                CreateFileMappingW(
                    file_handle as *mut winapi::ccrate::types::c_void,
                    ptr::null_mut(),
                    protect,
                    ((config.offset + config.length as u64) >> 32) as u32,
                    ((config.offset + config.length as u64) & 0xFFFFFFFF) as u32,
                    ptr::null(),
                )
            };

            if mapping_handle.is_null() {
                return Err(CursedError::Memory(format!(
                    "Failed to create file mapping: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let access = match config.protection {
                ProtectionFlags::Read => FILE_MAP_READ,
                ProtectionFlags::Write => winapi::um::fileapi::FILE_MAP_WRITE,
                ProtectionFlags::ReadWrite => winapi::um::fileapi::FILE_MAP_ALL_ACCESS,
                ProtectionFlags::Execute => winapi::um::fileapi::FILE_MAP_EXECUTE,
                ProtectionFlags::ReadExecute => FILE_MAP_READ | winapi::um::fileapi::FILE_MAP_EXECUTE,
                ProtectionFlags::WriteExecute => winapi::um::fileapi::FILE_MAP_WRITE | winapi::um::fileapi::FILE_MAP_EXECUTE,
                ProtectionFlags::All => winapi::um::fileapi::FILE_MAP_ALL_ACCESS,
                _ => FILE_MAP_READ,
            };

            let ptr = unsafe {
                MapViewOfFile(
                    mapping_handle as *mut winapi::ccrate::types::c_void,
                    access,
                    (config.offset >> 32) as u32,
                    (config.offset & 0xFFFFFFFF) as u32,
                    config.length,
                )
            };

            if ptr.is_null() {
                use winapi::um::handleapi::CloseHandle;
                unsafe { CloseHandle(mapping_handle as *mut winapi::ccrate::types::c_void) };
                return Err(CursedError::Memory(format!(
                    "Failed to map view of file: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let mut handle = MmapHandle::new(
                ptr as *mut u8,
                config.length,
                config.protection,
                config.mapping_type,
                None,
            );
            handle.file_handle = Some(file_handle);
            handle.mapping_handle = Some(mapping_handle as RawHandle);

            let handle = Arc::new(handle);
            let mut mappings = self.mappings.write().unwrap();
            mappings.insert(handle.as_ptr() as usize, Arc::clone(&handle));

            Ok(handle)
        }
    }

    /// Create an anonymous memory mapping (not backed by a file)
    pub fn map_anonymous(&self, config: MmapConfig) -> Result<(), Error> {
        if config.length == 0 {
            return Err(CursedError::InvalidInput("Mapping length cannot be zero".to_string()));
        }

        #[cfg(unix)]
        {
            let prot = protection_to_unix_flags(config.protection);
            let mut flags = libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
            
            if matches!(config.mapping_type, MappingType::Shared) {
                flags = libc::MAP_ANONYMOUS | libc::MAP_SHARED;
            }

            let ptr = unsafe {
                libc::mmap(
                    config.addr_hint.unwrap_or(ptr::null_mut()) as *mut libc::c_void,
                    config.length,
                    prot,
                    flags,
                    -1,
                    0,
                )
            };

            if ptr == libc::MAP_FAILED {
                return Err(CursedError::Memory(format!(
                    "Failed to create anonymous mapping: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let handle = Arc::new(MmapHandle::new(
                ptr as *mut u8,
                config.length,
                config.protection,
                config.mapping_type,
                None,
            ));

            let mut mappings = self.mappings.write().unwrap();
            mappings.insert(handle.as_ptr() as usize, Arc::clone(&handle));

            Ok(handle)
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualAlloc;
            use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

            let protect = match config.protection {
                ProtectionFlags::None => winapi::um::winnt::PAGE_NOACCESS,
                ProtectionFlags::Read => winapi::um::winnt::PAGE_READONLY,
                ProtectionFlags::Write => PAGE_READWRITE,
                ProtectionFlags::ReadWrite => PAGE_READWRITE,
                ProtectionFlags::Execute => winapi::um::winnt::PAGE_EXECUTE,
                ProtectionFlags::ReadExecute => winapi::um::winnt::PAGE_EXECUTE_READ,
                ProtectionFlags::WriteExecute => winapi::um::winnt::PAGE_EXECUTE_READWRITE,
                ProtectionFlags::All => winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            };

            let ptr = unsafe {
                VirtualAlloc(
                    config.addr_hint.unwrap_or(ptr::null_mut()) as *mut winapi::ccrate::types::c_void,
                    config.length,
                    MEM_COMMIT | MEM_RESERVE,
                    protect,
                )
            };

            if ptr.is_null() {
                return Err(CursedError::Memory(format!(
                    "Failed to allocate virtual memory: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let handle = Arc::new(MmapHandle::new(
                ptr as *mut u8,
                config.length,
                config.protection,
                config.mapping_type,
                None,
            ));

            let mut mappings = self.mappings.write().unwrap();
            mappings.insert(handle.as_ptr() as usize, Arc::clone(&handle));

            Ok(handle)
        }
    }

    /// Get statistics about current mappings
    pub fn get_statistics(&self) -> MmapStatistics {
        let mappings = self.mappings.read().unwrap();
        let total_mappings = mappings.len();
        let total_size = mappings.values().map(|h| h.len()).sum();
        
        MmapStatistics {
            total_mappings,
            total_size,
            active_handles: mappings.len(),
        }
    }

    /// Remove a mapping from tracking (mapping will still exist until handle is dropped)
    pub fn untrack_mapping(&self, ptr: *mut u8) -> bool {
        let mut mappings = self.mappings.write().unwrap();
        mappings.remove(&(ptr as usize)).is_some()
    }
}

/// Statistics about memory mappings
#[derive(Debug, Clone)]
pub struct MmapStatistics {
    pub total_mappings: usize,
    pub total_size: usize,
    pub active_handles: usize,
}

/// Shared memory region for IPC
pub struct SharedMemoryRegion {
    handle: Arc<MmapHandle>,
    name: String,
    size: usize,
}

impl SharedMemoryRegion {
    /// Create a new shared memory region
    pub fn create(name: &str, size: usize) -> Result<(), Error> {
        let manager = MmapManager::new();
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Shared,
            length: size,
            ..Default::default()
        };

        let handle = manager.map_anonymous(config)?;
        
        Ok(Self {
            handle,
            name: name.to_string(),
            size,
        })
    }

    /// Get the handle to the shared memory
    pub fn handle(&self) -> &Arc<MmapHandle> {
        &self.handle
    }

    /// Get the name of the shared memory region
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the size of the shared memory region
    pub fn size(&self) -> usize {
        self.size
    }
}

// Utility functions for platform-specific flag conversion

#[cfg(unix)]
fn protection_to_unix_flags(protection: ProtectionFlags) -> i32 {
    match protection {
        ProtectionFlags::None => libc::PROT_NONE,
        ProtectionFlags::Read => libc::PROT_READ,
        ProtectionFlags::Write => libc::PROT_WRITE,
        ProtectionFlags::ReadWrite => libc::PROT_READ | libc::PROT_WRITE,
        ProtectionFlags::Execute => libc::PROT_EXEC,
        ProtectionFlags::ReadExecute => libc::PROT_READ | libc::PROT_EXEC,
        ProtectionFlags::WriteExecute => libc::PROT_WRITE | libc::PROT_EXEC,
        ProtectionFlags::All => libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    }
}

#[cfg(unix)]
fn mapping_type_to_unix_flags(mapping_type: MappingType) -> i32 {
    match mapping_type {
        MappingType::Private => libc::MAP_PRIVATE,
        MappingType::Shared => libc::MAP_SHARED,
        MappingType::Anonymous => libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
        MappingType::Fixed => libc::MAP_FIXED | libc::MAP_PRIVATE,
    }
}

/// Global memory mapping manager instance
static GLOBAL_MMAP_MANAGER: std::sync::OnceLock<MmapManager> = std::sync::OnceLock::new();

/// Get the global memory mapping manager
pub fn get_mmap_manager() -> &'static MmapManager {
    GLOBAL_MMAP_MANAGER.get_or_init(|| MmapManager::new())
}

/// Convenience function to create a file-backed memory mapping
pub fn map_file(file: &File, config: MmapConfig) -> Result<(), Error> {
    get_mmap_manager().map_file(file, config)
}

/// Convenience function to create an anonymous memory mapping
pub fn map_anonymous(config: MmapConfig) -> Result<(), Error> {
    get_mmap_manager().map_anonymous(config)
}

/// Convenience function to create a shared memory region
pub fn create_shared_memory(name: &str, size: usize) -> Result<(), Error> {
    SharedMemoryRegion::create(name, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_anonymous_mapping() {
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Anonymous,
            length: 4096,
            ..Default::default()
        };

        let handle = map_anonymous(config).unwrap();
        assert_eq!(handle.len(), 4096);
        assert!(handle.can_read());
        assert!(handle.can_write());

        // Test writing and reading
        {
            let mut slice = handle.as_ref().clone().as_mut_slice().unwrap();
            slice[0] = 42;
            slice[100] = 84;
        }

        let slice = handle.as_slice().unwrap();
        assert_eq!(slice[0], 42);
        assert_eq!(slice[100], 84);
    }

    #[test]
    fn test_file_mapping() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, memory-mapped world!";
        temp_file.write_all(test_data).unwrap();
        temp_file.flush().unwrap();

        let file = temp_file.reopen().unwrap();
        let config = MmapConfig {
            protection: ProtectionFlags::Read,
            mapping_type: MappingType::Private,
            length: test_data.len(),
            ..Default::default()
        };

        let handle = map_file(&file, config).unwrap();
        let slice = handle.as_slice().unwrap();
        assert_eq!(slice, test_data);
    }

    #[test]
    fn test_protection_changes() {
        let config = MmapConfig {
            protection: ProtectionFlags::Read,
            mapping_type: MappingType::Anonymous,
            length: 4096,
            ..Default::default()
        };

        let handle = map_anonymous(config).unwrap();
        assert!(handle.can_read());
        assert!(!handle.can_write());

        // Change to read-write
        let mut mutable_handle = Arc::try_unwrap(handle).unwrap();
        mutable_handle.protect(ProtectionFlags::ReadWrite).unwrap();
        assert!(mutable_handle.can_read());
        assert!(mutable_handle.can_write());
    }

    #[test]
    fn test_memory_advice() {
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Anonymous,
            length: 4096,
            ..Default::default()
        };

        let handle = map_anonymous(config).unwrap();
        
        // Test various memory advice operations
        handle.advise(MemoryAdvice::Sequential).unwrap();
        handle.advise(MemoryAdvice::Random).unwrap();
        handle.advise(MemoryAdvice::WillNeed).unwrap();
        handle.advise(MemoryAdvice::DontNeed).unwrap();
    }

    #[test]
    fn test_shared_memory_region() {
        let region = create_shared_memory("test_region", 8192).unwrap();
        assert_eq!(region.name(), "test_region");
        assert_eq!(region.size(), 8192);
        
        let handle = region.handle();
        assert!(handle.can_read());
        assert!(handle.can_write());
    }

    #[test]
    fn test_mmap_statistics() {
        let manager = MmapManager::new();
        
        let stats_before = manager.get_statistics();
        assert_eq!(stats_before.total_mappings, 0);
        
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Anonymous,
            length: 4096,
            ..Default::default()
        };

        let _handle = manager.map_anonymous(config).unwrap();
        
        let stats_after = manager.get_statistics();
        assert_eq!(stats_after.total_mappings, 1);
        assert_eq!(stats_after.total_size, 4096);
    }

    #[test]
    fn test_handle_cloning() {
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Anonymous,
            length: 4096,
            ..Default::default()
        };

        let handle1 = map_anonymous(config).unwrap();
        let handle2 = handle1.clone_handle();
        
        assert_eq!(handle1.as_ptr(), handle2.as_ptr());
        assert_eq!(handle1.len(), handle2.len());
        
        // Both handles should point to the same memory
        {
            let mut slice1 = Arc::try_unwrap(handle1).unwrap().as_mut_slice().unwrap();
            slice1[0] = 123;
        }
        
        let slice2 = handle2.as_slice().unwrap();
        assert_eq!(slice2[0], 123);
    }
}

/// Options for memory mapping configuration
#[derive(Debug, Clone)]
pub struct MmapOptions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub shared: bool,
    pub private: bool,
    pub anonymous: bool,
    pub huge_pages: bool,
}

impl Default for MmapOptions {
    fn default() -> Self {
        Self {
            read: true,
            write: false,
            execute: false,
            shared: false,
            private: true,
            anonymous: false,
            huge_pages: false,
        }
    }
}
