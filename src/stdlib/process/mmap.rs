use crate::error::CursedError;
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

#[cfg(unix)]
use std::os::unix::io::AsRawFd as UnixAsRawFd;
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};

/// Memory mapping protection flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionFlags {
    /// No access allowed
    /// Read access only
    /// Write access only (rare)
    /// Read and write access
    /// Execute access only
    /// Read and execute access
    /// Write and execute access (dangerous)
    /// Full access (read, write, execute)
/// Memory mapping type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingType {
    /// Private copy-on-write mapping
    /// Shared mapping (changes visible to other processes)
    /// Anonymous mapping (not backed by file)
    /// Fixed mapping at specific address
/// Memory advisory hints for optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryAdvice {
    /// Normal random access pattern
    /// Sequential access pattern
    /// Random access pattern
    /// Will need these pages soon
    /// Don't need these pages
    /// Don't dump these pages in core dumps
    /// Do dump these pages in core dumps
/// Sync operation type for flushing changes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncType {
    /// Synchronous flush (blocks until complete)
    /// Asynchronous flush (returns immediately)
    /// Invalidate cached data
/// Memory mapping configuration
#[derive(Debug, Clone)]
pub struct MmapConfig {
impl Default for MmapConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Memory-mapped region handle
pub struct MmapHandle {
    #[cfg(windows)]
    #[cfg(windows)]
unsafe impl Send for MmapHandle {}
unsafe impl Sync for MmapHandle {}

impl MmapHandle {
    /// Create a new memory mapping handle
    fn new(
    ) -> Self {
        Self {
            #[cfg(windows)]
            #[cfg(windows)]
        }
    }

    /// Get the raw pointer to the mapped memory
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    /// Get the length of the mapping
    pub fn len(&self) -> usize {
        self.length
    /// Check if the mapping is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    /// Get a slice view of the mapped memory (read-only)
    pub fn as_slice(&self) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        if !self.can_read() {
            return Err(CursedError::Memory("Memory mapping not readable".to_string()));
        Ok(unsafe { std::slice::from_raw_parts(self.ptr, self.length) })
    /// Get a mutable slice view of the mapped memory
    pub fn as_mut_slice(&mut self) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        if !self.can_write() {
            return Err(CursedError::Memory("Memory mapping not writable".to_string()));
        Ok(unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length) })
    /// Check if the mapping allows reading
    pub fn can_read(&self) -> bool {
        matches!(
            ProtectionFlags::Read
                | ProtectionFlags::ReadWrite
                | ProtectionFlags::ReadExecute
                | ProtectionFlags::All
        )
    /// Check if the mapping allows writing
    pub fn can_write(&self) -> bool {
        matches!(
            ProtectionFlags::Write
                | ProtectionFlags::ReadWrite
                | ProtectionFlags::WriteExecute
                | ProtectionFlags::All
        )
    /// Check if the mapping allows execution
    pub fn can_execute(&self) -> bool {
        matches!(
            ProtectionFlags::Execute
                | ProtectionFlags::ReadExecute
                | ProtectionFlags::WriteExecute
                | ProtectionFlags::All
        )
    /// Flush changes to disk synchronously
    pub fn sync(&self, sync_type: SyncType) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        #[cfg(unix)]
        {
            let flags = match sync_type {

            let result = unsafe { libc::msync(self.ptr as *mut libc::c_void, self.length, flags) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::FlushViewOfFile;
            use winapi::um::fileapi::FlushFileBuffers;

            let result = unsafe { FlushViewOfFile(self.ptr as *const winapi::ctypes::c_void, self.length) };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            // Also flush file buffers if we have a file handle
            if let Some(handle) = self.file_handle {
                let result = unsafe { FlushFileBuffers(handle as *mut winapi::ctypes::c_void) };
                if result == 0 {
                    return Err(CursedError::Memory(format!(
                        std::io::Error::last_os_error()
                    )));
                }
            }
        Ok(())
    /// Change protection flags for the mapping
    pub fn protect(&mut self, new_protection: ProtectionFlags) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        #[cfg(unix)]
        {
            let prot = protection_to_unix_flags(new_protection);
            let result = unsafe { libc::mprotect(self.ptr as *mut libc::c_void, self.length, prot) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualProtect;
            use winapi::um::winnt::{PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE};

            let protect = match new_protection {
                ProtectionFlags::Write => PAGE_READWRITE, // Windows doesn't support write-only

            let mut old_protect = 0;
            let result = unsafe {
                VirtualProtect(
                )
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        self.protection = new_protection;
        Ok(())
    /// Lock pages in physical memory
    pub fn lock_pages(&mut self) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        if self.is_locked {
            return Ok(()); // Already locked
        #[cfg(unix)]
        {
            let result = unsafe { libc::mlock(self.ptr as *const libc::c_void, self.length) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualLock;

            let result = unsafe { VirtualLock(self.ptr as *mut winapi::ctypes::c_void, self.length) };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        self.is_locked = true;
        Ok(())
    /// Unlock pages from physical memory
    pub fn unlock_pages(&mut self) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        if !self.is_locked {
            return Ok(()); // Not locked
        #[cfg(unix)]
        {
            let result = unsafe { libc::munlock(self.ptr as *const libc::c_void, self.length) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualUnlock;

            let result = unsafe { VirtualUnlock(self.ptr as *mut winapi::ctypes::c_void, self.length) };
            if result == 0 {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            }
        }

        self.is_locked = false;
        Ok(())
    /// Provide memory usage advice to the kernel
    pub fn advise(&self, advice: MemoryAdvice) -> crate::error::Result<()> {
        if self.ptr.is_null() {
            return Err(CursedError::Memory("Invalid memory mapping".to_string()));
        #[cfg(unix)]
        {
            let advice_flag = match advice {
                #[cfg(target_os = "linux")]
                #[cfg(target_os = "linux")]
                #[cfg(not(target_os = "linux"))]
                MemoryAdvice::DontDump => return Ok(()), // Not supported, ignore
                #[cfg(not(target_os = "linux"))]
                MemoryAdvice::DoDump => return Ok(()), // Not supported, ignore

            let result = unsafe { libc::madvise(self.ptr as *mut libc::c_void, self.length, advice_flag) };
            if result != 0 {
                return Err(CursedError::Memory(format!(
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
        Ok(())
    /// Clone the handle (increases reference count)
    pub fn clone_handle(&self) -> MmapHandle {
        let mut ref_count = self.reference_count.lock().unwrap();
        *ref_count += 1;

        MmapHandle {
            #[cfg(windows)]
            #[cfg(windows)]
        }
    }
impl Drop for MmapHandle {
    fn drop(&mut self) {
        let should_unmap = {
            let mut ref_count = self.reference_count.lock().unwrap();
            *ref_count -= 1;
            *ref_count == 0

        if should_unmap && !self.ptr.is_null() {
            // Unlock pages if they were locked
            if self.is_locked {
                let _ = self.unlock_pages();
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
                    UnmapViewOfFile(self.ptr as *const winapi::ctypes::c_void);
                    
                    if let Some(handle) = self.mapping_handle {
                        CloseHandle(handle as *mut winapi::ctypes::c_void);
                    }
                }
            }
        }
    }
}

/// Memory mapping manager for coordinating multiple mappings
pub struct MmapManager {
impl MmapManager {
    /// Create a new memory mapping manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a file-backed memory mapping
    pub fn map_file(
    ) -> crate::error::Result<()> {
        if config.length == 0 {
            return Err(CursedError::InvalidInput("Mapping length cannot be zero".to_string()));
        #[cfg(unix)]
        {
            let fd = file.as_raw_fd();
            let prot = protection_to_unix_flags(config.protection);
            let flags = mapping_type_to_unix_flags(config.mapping_type);

            let ptr = unsafe {
                libc::mmap(
                )

            if ptr == libc::MAP_FAILED {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            let handle = Arc::new(MmapHandle::new(
            ));

            // Apply additional configuration
            if config.populate_pages {
                let _ = handle.advise(MemoryAdvice::WillNeed);
            if config.lock_pages {
                let mut mutable_handle = Arc::try_unwrap(handle).map_err(|_| {
                    CursedError::Memory("Failed to get mutable handle for locking".to_string())
                })?;
                mutable_handle.lock_pages()?;
                let handle = Arc::new(mutable_handle);
            let mut mappings = self.mappings.write().unwrap();
            mappings.insert(handle.as_ptr() as usize, Arc::clone(&handle));

            Ok(handle)
        #[cfg(windows)]
        {
            use winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile};
            use winapi::um::winnt::{PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE};
            use winapi::um::fileapi::FILE_MAP_READ;

            let file_handle = file.as_raw_handle();
            
            let protect = match config.protection {

            let mapping_handle = unsafe {
                CreateFileMappingW(
                )

            if mapping_handle.is_null() {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            let access = match config.protection {

            let ptr = unsafe {
                MapViewOfFile(
                )

            if ptr.is_null() {
                use winapi::um::handleapi::CloseHandle;
                unsafe { CloseHandle(mapping_handle as *mut winapi::ctypes::c_void) };
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            let mut handle = MmapHandle::new(
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
    pub fn map_anonymous(&self, config: MmapConfig) -> crate::error::Result<()> {
        if config.length == 0 {
            return Err(CursedError::InvalidInput("Mapping length cannot be zero".to_string()));
        #[cfg(unix)]
        {
            let prot = protection_to_unix_flags(config.protection);
            let mut flags = libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
            
            if matches!(config.mapping_type, MappingType::Shared) {
                flags = libc::MAP_ANONYMOUS | libc::MAP_SHARED;
            let ptr = unsafe {
                libc::mmap(
                )

            if ptr == libc::MAP_FAILED {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            let handle = Arc::new(MmapHandle::new(
            ));

            let mut mappings = self.mappings.write().unwrap();
            mappings.insert(handle.as_ptr() as usize, Arc::clone(&handle));

            Ok(handle)
        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualAlloc;
            use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

            let protect = match config.protection {

            let ptr = unsafe {
                VirtualAlloc(
                )

            if ptr.is_null() {
                return Err(CursedError::Memory(format!(
                    std::io::Error::last_os_error()
                )));
            let handle = Arc::new(MmapHandle::new(
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
/// Shared memory region for IPC
pub struct SharedMemoryRegion {
impl SharedMemoryRegion {
    /// Create a new shared memory region
    pub fn create(name: &str, size: usize) -> crate::error::Result<()> {
        let manager = MmapManager::new();
        let config = MmapConfig {
            ..Default::default()

        let handle = manager.map_anonymous(config)?;
        
        Ok(Self {
        })
    /// Get the handle to the shared memory
    pub fn handle(&self) -> &Arc<MmapHandle> {
        &self.handle
    /// Get the name of the shared memory region
    pub fn name(&self) -> &str {
        &self.name
    /// Get the size of the shared memory region
    pub fn size(&self) -> usize {
        self.size
    }
}

// Utility functions for platform-specific flag conversion

#[cfg(unix)]
fn protection_to_unix_flags(protection: ProtectionFlags) -> i32 {
    match protection {
    }
}

#[cfg(unix)]
fn mapping_type_to_unix_flags(mapping_type: MappingType) -> i32 {
    match mapping_type {
    }
}

/// Global memory mapping manager instance
static GLOBAL_MMAP_MANAGER: std::sync::OnceLock<MmapManager> = std::sync::OnceLock::new();

/// Get the global memory mapping manager
pub fn get_mmap_manager() -> &'static MmapManager {
    GLOBAL_MMAP_MANAGER.get_or_init(|| MmapManager::new())
/// Convenience function to create a file-backed memory mapping
pub fn map_file(file: &File, config: MmapConfig) -> crate::error::Result<()> {
    get_mmap_manager().map_file(file, config)
/// Convenience function to create an anonymous memory mapping
pub fn map_anonymous(config: MmapConfig) -> crate::error::Result<()> {
    get_mmap_manager().map_anonymous(config)
/// Convenience function to create a shared memory region
pub fn create_shared_memory(name: &str, size: usize) -> crate::error::Result<()> {
    SharedMemoryRegion::create(name, size)

impl Default for MmapOptions {
    fn default() -> Self {
        Self {
        }
    }
}
