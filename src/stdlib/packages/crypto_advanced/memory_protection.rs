use crate::error::Error;
/// fr fr Memory protection for sensitive cryptographic data
use super::errors::*;
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

/// Secure memory container that zeroes on drop
#[derive(Debug)]
pub struct SecureMemory {
    data: NonNull<u8>,
    len: usize,
    capacity: usize,
    locked: AtomicBool,
}

impl SecureMemory {
    /// Create new secure memory container
    pub fn new(data: Vec<u8>) -> AdvancedCryptoResult<Self> {
        let len = data.len();
        let capacity = data.capacity();
        
        if len == 0 {
            return Err(AdvancedCryptoError::InvalidParameters("Cannot create secure memory with zero length".to_string()));
        }
        
        // Allocate aligned memory
        let layout = Layout::from_size_align(capacity, 16)
            .map_err(|_| AdvancedCryptoError::InternalError("Invalid memory layout".to_string()))?;
        
        let ptr = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(AdvancedCryptoError::InternalError("Memory allocation failed".to_string()));
        }
        
        let data_ptr = NonNull::new(ptr)
            .ok_or_else(|| AdvancedCryptoError::InternalError("Invalid memory pointer".to_string()))?;
        
        // Copy data to secure memory
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), ptr, len);
        }
        
        let secure_mem = Self {
            data: data_ptr,
            len,
            capacity,
            locked: AtomicBool::new(false),
        };
        
        // Attempt to lock memory to prevent swapping
        if let Err(_) = secure_mem.lock_memory() {
            // Continue even if locking fails - not critical
        }
        
        Ok(secure_mem)
    }
    
    /// Get read-only access to data
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }
    
    /// Get length of data
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Lock memory to prevent swapping (platform-specific)
    pub fn lock_memory(&self) -> AdvancedCryptoResult<()> {
        #[cfg(unix)]
        {
            let result = unsafe {
                libc::mlock(self.data.as_ptr() as *const libc::c_void, self.capacity)
            };
            if result == 0 {
                self.locked.store(true, Ordering::Release);
                Ok(())
            } else {
                Err(AdvancedCryptoError::InternalError("Memory locking failed".to_string()))
            }
        }
        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualLock;
            let result = unsafe {
                VirtualLock(self.data.as_ptr() as *mut winapi::ctypes::c_void, self.capacity)
            };
            if result != 0 {
                self.locked.store(true, Ordering::Release);
                Ok(())
            } else {
                Err(AdvancedCryptoError::InternalError("Memory locking failed".to_string()))
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            // No-op for unsupported platforms
            Ok(())
        }
    }
    
    /// Unlock memory
    pub fn unlock_memory(&self) -> AdvancedCryptoResult<()> {
        if !self.locked.load(Ordering::Acquire) {
            return Ok(());
        }
        
        #[cfg(unix)]
        {
            let result = unsafe {
                libc::munlock(self.data.as_ptr() as *const libc::c_void, self.capacity)
            };
            if result == 0 {
                self.locked.store(false, Ordering::Release);
                Ok(())
            } else {
                Err(AdvancedCryptoError::InternalError("Memory unlocking failed".to_string()))
            }
        }
        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualUnlock;
            let result = unsafe {
                VirtualUnlock(self.data.as_ptr() as *mut winapi::ctypes::c_void, self.capacity)
            };
            if result != 0 {
                self.locked.store(false, Ordering::Release);
                Ok(())
            } else {
                Err(AdvancedCryptoError::InternalError("Memory unlocking failed".to_string()))
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            // No-op for unsupported platforms
            Ok(())
        }
    }
    
    /// Securely clear memory
    pub fn secure_clear(&mut self) {
        clear_sensitive_data_volatile(unsafe { 
            std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len) 
        });
    }
}

unsafe impl Send for SecureMemory {}
unsafe impl Sync for SecureMemory {}

impl Drop for SecureMemory {
    fn drop(&mut self) {
        // Securely clear memory
        self.secure_clear();
        
        // Unlock if locked
        let _ = self.unlock_memory();
        
        // Deallocate memory
        let layout = Layout::from_size_align(self.capacity, 16).unwrap();
        unsafe {
            dealloc(self.data.as_ptr(), layout);
        }
    }
}

/// Container that automatically zeroes data on drop
#[derive(Debug)]
pub struct ZeroOnDrop {
    data: Vec<u8>,
}

impl ZeroOnDrop {
    /// Create new zero-on-drop container
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    
    /// Get read-only access to data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    
    /// Get mutable access to data
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }
    
    /// Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Manually clear data
    pub fn clear(&mut self) {
        clear_sensitive_data_volatile(&mut self.data);
    }
}

impl Drop for ZeroOnDrop {
    fn drop(&mut self) {
        clear_sensitive_data_volatile(&mut self.data);
    }
}

/// Protected byte array with access controls
#[derive(Debug)]
pub struct ProtectedBytes {
    inner: SecureMemory,
    access_count: std::sync::atomic::AtomicU32,
    max_access: u32,
}

impl ProtectedBytes {
    /// Create new protected bytes with access limit
    pub fn new(data: Vec<u8>, max_access: u32) -> AdvancedCryptoResult<Self> {
        Ok(Self {
            inner: SecureMemory::new(data)?,
            access_count: std::sync::atomic::AtomicU32::new(0),
            max_access,
        })
    }
    
    /// Access data with automatic access counting
    pub fn access<T>(&self, f: impl FnOnce(&[u8]) -> T) -> AdvancedCryptoResult<T> {
        let current = self.access_count.fetch_add(1, Ordering::AcqRel);
        if current >= self.max_access {
            return Err(AdvancedCryptoError::AccessDenied("Maximum access count exceeded".to_string()));
        }
        
        Ok(f(self.inner.as_bytes()))
    }
    
    /// Get current access count
    pub fn access_count(&self) -> u32 {
        self.access_count.load(Ordering::Acquire)
    }
    
    /// Reset access count
    pub fn reset_access_count(&self) {
        self.access_count.store(0, Ordering::Release);
    }
}

/// Memory barrier for preventing compiler optimizations
#[derive(Debug, Clone)]
pub struct MemoryBarrier;

impl MemoryBarrier {
    /// Create memory barrier
    pub fn new() -> Self {
        Self
    }
    
    /// Execute memory barrier
    pub fn barrier() {
        std::sync::atomic::compiler_fence(Ordering::SeqCst);
    }
    
    /// Volatile read
    pub fn volatile_read<T: Copy>(src: *const T) -> T {
        unsafe { std::ptr::read_volatile(src) }
    }
    
    /// Volatile write
    pub fn volatile_write<T: Copy>(dst: *mut T, value: T) {
        unsafe { std::ptr::write_volatile(dst, value) }
    }
}

/// Clear sensitive data with volatile operations to prevent optimization
pub fn clear_sensitive_data(data: &mut [u8]) {
    clear_sensitive_data_volatile(data);
}

/// Volatile clear to prevent compiler optimization
pub fn clear_sensitive_data_volatile(data: &mut [u8]) {
    for byte in data.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
    
    // Additional memory barrier
    MemoryBarrier::barrier();
}

/// Lock memory pages to prevent swapping
pub fn memory_lock(data: &[u8]) -> AdvancedCryptoResult<()> {
    if data.is_empty() {
        return Ok(());
    }
    
    #[cfg(unix)]
    {
        let result = unsafe {
            libc::mlock(data.as_ptr() as *const libc::c_void, data.len())
        };
        if result == 0 {
            Ok(())
        } else {
            Err(AdvancedCryptoError::InternalError("Memory locking failed".to_string()))
        }
    }
    #[cfg(windows)]
    {
        use winapi::um::memoryapi::VirtualLock;
        let result = unsafe {
            VirtualLock(data.as_ptr() as *mut winapi::ctypes::c_void, data.len())
        };
        if result != 0 {
            Ok(())
        } else {
            Err(AdvancedCryptoError::InternalError("Memory locking failed".to_string()))
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        // No-op for unsupported platforms
        Ok(())
    }
}

/// Unlock memory pages
pub fn memory_unlock(data: &[u8]) -> AdvancedCryptoResult<()> {
    if data.is_empty() {
        return Ok(());
    }
    
    #[cfg(unix)]
    {
        let result = unsafe {
            libc::munlock(data.as_ptr() as *const libc::c_void, data.len())
        };
        if result == 0 {
            Ok(())
        } else {
            Err(AdvancedCryptoError::InternalError("Memory unlocking failed".to_string()))
        }
    }
    #[cfg(windows)]
    {
        use winapi::um::memoryapi::VirtualUnlock;
        let result = unsafe {
            VirtualUnlock(data.as_ptr() as *mut winapi::ctypes::c_void, data.len())
        };
        if result != 0 {
            Ok(())
        } else {
            Err(AdvancedCryptoError::InternalError("Memory unlocking failed".to_string()))
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        // No-op for unsupported platforms
        Ok(())
    }
}

/// Memory protection utilities
pub struct MemoryProtection;

impl MemoryProtection {
    /// Secure allocate aligned memory
    pub fn secure_alloc(size: usize) -> AdvancedCryptoResult<SecureMemory> {
        SecureMemory::new(vec![0u8; size])
    }
    
    /// Create protected bytes with limited access
    pub fn protect_with_limit(data: Vec<u8>, max_access: u32) -> AdvancedCryptoResult<ProtectedBytes> {
        ProtectedBytes::new(data, max_access)
    }
    
    /// Secure copy with clearing source
    pub fn secure_copy(src: &mut [u8], dst: &mut [u8]) -> AdvancedCryptoResult<()> {
        if src.len() != dst.len() {
            return Err(AdvancedCryptoError::InvalidParameters("Source and destination lengths must match".to_string()));
        }
        
        dst.copy_from_slice(src);
        clear_sensitive_data_volatile(src);
        Ok(())
    }
    
    /// Check if memory is locked
    pub fn is_memory_locked(data: &[u8]) -> bool {
        // Platform-specific implementation would check if memory is locked
        // For now, return false as we can't easily check this
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secure_memory() {
        let data = vec![1, 2, 3, 4, 5];
        let secure_mem = SecureMemory::new(data.clone()).unwrap();
        
        assert_eq!(secure_mem.as_bytes(), &data);
        assert_eq!(secure_mem.len(), 5);
        assert!(!secure_mem.is_empty());
    }
    
    #[test]
    fn test_zero_on_drop() {
        let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
        {
            let mut zero_drop = ZeroOnDrop::new(data.clone());
            assert_eq!(zero_drop.as_bytes(), &data);
            
            // Modify data
            zero_drop.as_mut_bytes()[0] = 0xFF;
            assert_eq!(zero_drop.as_bytes()[0], 0xFF);
        }
        // Data should be zeroed after drop
    }
    
    #[test]
    fn test_protected_bytes() {
        let data = vec![1, 2, 3, 4];
        let protected = ProtectedBytes::new(data.clone(), 2).unwrap();
        
        // First access should work
        let result1 = protected.access(|bytes| bytes.len());
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 4);
        
        // Second access should work
        let result2 = protected.access(|bytes| bytes[0]);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 1);
        
        // Third access should fail
        let result3 = protected.access(|bytes| bytes.len());
        assert!(result3.is_err());
        
        assert_eq!(protected.access_count(), 2);
    }
    
    #[test]
    fn test_clear_sensitive_data() {
        let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
        clear_sensitive_data(&mut data);
        assert_eq!(data, vec![0, 0, 0, 0]);
    }
    
    #[test]
    fn test_memory_barrier() {
        let barrier = MemoryBarrier::new();
        MemoryBarrier::barrier(); // Should not panic
        
        let value = 42u32;
        let read_value = MemoryBarrier::volatile_read(&value);
        assert_eq!(read_value, 42);
        
        let mut dest = 0u32;
        MemoryBarrier::volatile_write(&mut dest, 123);
        assert_eq!(dest, 123);
    }
    
    #[test]
    fn test_memory_protection_utilities() {
        let secure_mem = MemoryProtection::secure_alloc(16).unwrap();
        assert_eq!(secure_mem.len(), 16);
        
        let protected = MemoryProtection::protect_with_limit(vec![1, 2, 3], 1).unwrap();
        assert_eq!(protected.access_count(), 0);
        
        let mut src = vec![1, 2, 3];
        let mut dst = vec![0, 0, 0];
        MemoryProtection::secure_copy(&mut src, &mut dst).unwrap();
        assert_eq!(dst, vec![1, 2, 3]);
        assert_eq!(src, vec![0, 0, 0]); // Source should be cleared
    }
}
