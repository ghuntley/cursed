/// Memory mapping and management operations
use std::ptr;
use crate::stdlib::sys_core::error::{SysCoreError, SysCoreResult, system_call_error, not_supported, invalid_argument};

/// Memory mapping handle
pub struct MemoryMap {
    addr: *mut libc::c_void,
    size: usize,
}

/// Memory protection flags
#[derive(Debug, Clone, Copy)]
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
    
    pub fn read_execute() -> Self {
        Self { read: true, write: false, execute: true }
    }
    
    pub fn read_write_execute() -> Self {
        Self { read: true, write: true, execute: true }
    }
}

/// Memory mapping flags
#[derive(Debug, Clone, Copy)]
pub struct MemoryFlags {
    pub shared: bool,
    pub private: bool,
    pub anonymous: bool,
    pub fixed: bool,
}

impl Default for MemoryFlags {
    fn default() -> Self {
        Self {
            shared: false,
            private: true,
            anonymous: true,
            fixed: false,
        }
    }
}

impl MemoryMap {
    /// Get the mapped address
    pub fn as_ptr(&self) -> *mut u8 {
        self.addr as *mut u8
    }
    
    /// Get the mapped size
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get a slice view of the memory
    pub unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.addr as *const u8, self.size)
    }
    
    /// Get a mutable slice view of the memory
    pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.addr as *mut u8, self.size)
    }
}

/// Map memory into the address space
pub fn mmap_memory(
    addr: Option<*mut libc::c_void>,
    size: usize,
    protection: MemoryProtection,
    flags: MemoryFlags,
    fd: Option<i32>,
    offset: i64,
) -> SysCoreResult<MemoryMap> {
    #[cfg(unix)]
    {
        let mut prot = 0;
        if protection.read { prot |= libc::PROT_READ; }
        if protection.write { prot |= libc::PROT_WRITE; }
        if protection.execute { prot |= libc::PROT_EXEC; }
        
        let mut map_flags = 0;
        if flags.shared { map_flags |= libc::MAP_SHARED; }
        if flags.private { map_flags |= libc::MAP_PRIVATE; }
        if flags.anonymous { map_flags |= libc::MAP_ANONYMOUS; }
        if flags.fixed { map_flags |= libc::MAP_FIXED; }
        
        let fd = fd.unwrap_or(-1);
        let addr = addr.unwrap_or(ptr::null_mut());
        
        let result = unsafe {
            libc::mmap(addr, size, prot, map_flags, fd, offset)
        };
        
        if result == libc::MAP_FAILED {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_call_error("mmap", errno));
        }
        
        Ok(MemoryMap {
            addr: result,
            size,
        })
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("mmap not supported on this platform"))
    }
}

/// Unmap memory from the address space
pub fn munmap_memory(map: MemoryMap) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::munmap(map.addr, map.size) };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_call_error("munmap", errno));
        }
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("munmap not supported on this platform"))
    }
}

/// Change memory protection for a mapped region
pub fn mprotect_memory(addr: *mut libc::c_void, size: usize, protection: MemoryProtection) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let mut prot = 0;
        if protection.read { prot |= libc::PROT_READ; }
        if protection.write { prot |= libc::PROT_WRITE; }
        if protection.execute { prot |= libc::PROT_EXEC; }
        
        let result = unsafe { libc::mprotect(addr, size, prot) };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_call_error("mprotect", errno));
        }
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("mprotect not supported on this platform"))
    }
}

/// Lock memory pages in RAM
pub fn mlock_memory(addr: *const libc::c_void, size: usize) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::mlock(addr, size) };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_call_error("mlock", errno));
        }
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("mlock not supported on this platform"))
    }
}

/// Unlock memory pages
pub fn munlock_memory(addr: *const libc::c_void, size: usize) -> SysCoreResult<()> {
    #[cfg(unix)]
    {
        let result = unsafe { libc::munlock(addr, size) };
        if result == -1 {
            let errno = unsafe { *libc::__errno_location() };
            return Err(system_call_error("munlock", errno));
        }
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("munlock not supported on this platform"))
    }
}

/// Get system page size
pub fn get_page_size() -> usize {
    #[cfg(unix)]
    {
        unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
    }
    
    #[cfg(not(unix))]
    {
        4096 // Default page size
    }
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub available_memory: u64,
    pub free_memory: u64,
    pub used_memory: u64,
    pub page_size: usize,
    pub swap_total: u64,
    pub swap_free: u64,
}

/// Get system memory information
pub fn get_memory_info() -> SysCoreResult<MemoryInfo> {
    #[cfg(target_os = "linux")]
    {
        let meminfo = std::fs::read_to_string("/proc/meminfo")
            .map_err(|e| SysCoreError::IoError(format!("Failed to read /proc/meminfo: {}", e)))?;
        
        let mut total = 0u64;
        let mut available = 0u64;
        let mut free = 0u64;
        let mut swap_total = 0u64;
        let mut swap_free = 0u64;
        
        for line in meminfo.lines() {
            if let Some(value) = parse_meminfo_line(line, "MemTotal:") {
                total = value;
            } else if let Some(value) = parse_meminfo_line(line, "MemAvailable:") {
                available = value;
            } else if let Some(value) = parse_meminfo_line(line, "MemFree:") {
                free = value;
            } else if let Some(value) = parse_meminfo_line(line, "SwapTotal:") {
                swap_total = value;
            } else if let Some(value) = parse_meminfo_line(line, "SwapFree:") {
                swap_free = value;
            }
        }
        
        Ok(MemoryInfo {
            total_memory: total * 1024, // Convert from KB to bytes
            available_memory: available * 1024,
            free_memory: free * 1024,
            used_memory: (total - free) * 1024,
            page_size: get_page_size(),
            swap_total: swap_total * 1024,
            swap_free: swap_free * 1024,
        })
    }
    
    #[cfg(unix)]
    #[cfg(not(target_os = "linux"))]
    {
        // Simplified implementation for other Unix systems
        let page_size = get_page_size();
        let total_pages = unsafe { libc::sysconf(libc::_SC_PHYS_PAGES) } as u64;
        let available_pages = unsafe { libc::sysconf(libc::_SC_AVPHYS_PAGES) } as u64;
        
        let total_memory = total_pages * page_size as u64;
        let available_memory = available_pages * page_size as u64;
        
        Ok(MemoryInfo {
            total_memory,
            available_memory,
            free_memory: available_memory, // Simplified
            used_memory: total_memory - available_memory,
            page_size,
            swap_total: 0, // Not easily available on all Unix systems
            swap_free: 0,
        })
    }
    
    #[cfg(not(unix))]
    {
        Err(not_supported("Memory info not supported on this platform"))
    }
}

#[cfg(target_os = "linux")]
fn parse_meminfo_line(line: &str, prefix: &str) -> Option<u64> {
    if line.starts_with(prefix) {
        line.split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
    } else {
        None
    }
}

/// Check if memory mapping is supported
pub fn supports_mmap() -> bool {
    cfg!(unix)
}

/// Memory allocation using system allocator
pub fn alloc_memory(size: usize, alignment: usize) -> SysCoreResult<*mut u8> {
    if size == 0 {
        return Err(invalid_argument("Cannot allocate zero bytes"));
    }
    
    #[cfg(unix)]
    {
        let mut ptr: *mut libc::c_void = ptr::null_mut();
        let result = unsafe { libc::posix_memalign(&mut ptr, alignment, size) };
        
        if result != 0 {
            return Err(system_call_error("posix_memalign", result));
        }
        
        Ok(ptr as *mut u8)
    }
    
    #[cfg(not(unix))]
    {
        // Fallback to standard allocator
        use std::alloc::{alloc, Layout};
        
        let layout = Layout::from_size_align(size, alignment)
            .map_err(|_| invalid_argument("Invalid layout for allocation"))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(SysCoreError::MemoryError("Failed to allocate memory".to_string()));
        }
        
        Ok(ptr)
    }
}

/// Free memory allocated by alloc_memory
pub fn free_memory(ptr: *mut u8, size: usize, alignment: usize) -> SysCoreResult<()> {
    if ptr.is_null() {
        return Ok(()); // Free of null pointer is a no-op
    }
    
    #[cfg(unix)]
    {
        unsafe { libc::free(ptr as *mut libc::c_void) };
        Ok(())
    }
    
    #[cfg(not(unix))]
    {
        use std::alloc::{dealloc, Layout};
        
        let layout = Layout::from_size_align(size, alignment)
            .map_err(|_| invalid_argument("Invalid layout for deallocation"))?;
        
        unsafe { dealloc(ptr, layout) };
        Ok(())
    }
}

/// Memory barrier operations
pub fn memory_barrier() {
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
}

/// Read barrier
pub fn read_barrier() {
    std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);
}

/// Write barrier
pub fn write_barrier() {
    std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
}
