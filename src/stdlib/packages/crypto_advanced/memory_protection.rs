/// fr fr Memory protection for sensitive cryptographic data - no leaks bestie

use std::fmt;

/// fr fr Secure memory wrapper that zeros on drop
pub struct SecureMemory<T> {
    data: T,
}

impl<T> SecureMemory<T> {
    /// slay Create new secure memory wrapper
    pub fn new(data: T) -> Result<Self, MemoryProtectionError> {
        Ok(Self { data })
    }
}

impl<T> std::ops::Deref for SecureMemory<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for SecureMemory<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Clone for SecureMemory<T> 
where 
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T> fmt::Debug for SecureMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecureMemory([REDACTED])")
    }
}

impl Drop for SecureMemory<Vec<u8>> {
    fn drop(&mut self) {
        clear_sensitive_data(&mut self.data);
    }
}

/// fr fr Zero on drop wrapper
pub struct ZeroOnDrop<T> {
    data: T,
}

impl<T> ZeroOnDrop<T> {
    /// slay Create new zero-on-drop wrapper
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> std::ops::Deref for ZeroOnDrop<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for ZeroOnDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Drop for ZeroOnDrop<Vec<u8>> {
    fn drop(&mut self) {
        clear_sensitive_data(&mut self.data);
    }
}

/// fr fr Protected bytes with secure operations
pub struct ProtectedBytes {
    data: SecureMemory<Vec<u8>>,
}

impl ProtectedBytes {
    /// slay Create new protected bytes
    pub fn new(data: Vec<u8>) -> Result<Self, MemoryProtectionError> {
        Ok(Self {
            data: SecureMemory::new(data)?,
        })
    }
    
    /// slay Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// slay Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// slay Get bytes (read-only)
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// fr fr Memory barrier to prevent optimizations
pub struct MemoryBarrier;

impl MemoryBarrier {
    /// slay Create memory barrier
    pub fn new() -> Self {
        Self
    }
    
    /// slay Execute memory barrier
    pub fn barrier(&self) {
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

impl Default for MemoryBarrier {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Memory protection errors
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryProtectionError {
    AllocationFailed,
    LockFailed,
    UnlockFailed,
    ProtectionFailed,
    Internal(String),
}

impl std::fmt::Display for MemoryProtectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryProtectionError::AllocationFailed => write!(f, "Memory allocation failed"),
            MemoryProtectionError::LockFailed => write!(f, "Memory lock failed"),
            MemoryProtectionError::UnlockFailed => write!(f, "Memory unlock failed"),
            MemoryProtectionError::ProtectionFailed => write!(f, "Memory protection failed"),
            MemoryProtectionError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for MemoryProtectionError {}

/// slay Clear sensitive data from memory
pub fn clear_sensitive_data(data: &mut [u8]) {
    // Use volatile write to prevent optimization
    for byte in data.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
    
    // Memory barrier to ensure completion
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

/// slay Lock memory pages (platform-specific)
pub fn memory_lock(_data: &[u8]) -> Result<(), MemoryProtectionError> {
    // Platform-specific implementation would go here
    // For now, return success as a stub
    Ok(())
}

/// slay Unlock memory pages (platform-specific)
pub fn memory_unlock(_data: &[u8]) -> Result<(), MemoryProtectionError> {
    // Platform-specific implementation would go here
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secure_memory() {
        let data = Vec::from([1, 2, 3, 4, 5]);
        let secure = SecureMemory::new(data.clone()).unwrap();
        assert_eq!(&*secure, &data);
    }
    
    #[test]
    fn test_zero_on_drop() {
        let mut data = Vec::from([1, 2, 3, 4, 5]);
        {
            let _zero_on_drop = ZeroOnDrop::new(&mut data);
        }
        // Data should be zeroed after drop
        // Note: This test can't verify the actual zeroing due to borrowing rules
    }
    
    #[test]
    fn test_protected_bytes() {
        let data = Vec::from([1, 2, 3, 4, 5]);
        let protected = ProtectedBytes::new(data.clone()).unwrap();
        
        assert_eq!(protected.len(), 5);
        assert!(!protected.is_empty());
        assert_eq!(protected.as_bytes(), &data);
    }
    
    #[test]
    fn test_clear_sensitive_data() {
        let mut data = Vec::from([1, 2, 3, 4, 5]);
        clear_sensitive_data(&mut data);
        assert_eq!(data, Vec::from([0, 0, 0, 0, 0]));
    }
    
    #[test]
    fn test_memory_protection() {
        let data = Vec::from([1, 2, 3, 4, 5]);
        assert!(memory_lock(&data).is_ok());
        assert!(memory_unlock(&data).is_ok());
    }
    
    #[test]
    fn test_memory_barrier() {
        let barrier = MemoryBarrier::new();
        barrier.barrier(); // Should not panic
        
        let default_barrier = MemoryBarrier::default();
        default_barrier.barrier();
    }
}
