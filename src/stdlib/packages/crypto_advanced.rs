// Advanced cryptographic utilities package

use std::ptr;

/// Result type for advanced crypto operations
pub type AdvancedCryptoResult<T> = Result<T, AdvancedCryptoError>;

/// Advanced crypto error type
#[derive(Debug, Clone)]
pub enum AdvancedCryptoError {
    InvalidInput,
    MemoryAllocationFailed,
    EncryptionFailed,
    DecryptionFailed,
    KeyGenerationFailed,
    SecurityViolation,
}

impl std::fmt::Display for AdvancedCryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancedCryptoError::InvalidInput => write!(f, "Invalid input"),
            AdvancedCryptoError::MemoryAllocationFailed => write!(f, "Memory allocation failed"),
            AdvancedCryptoError::EncryptionFailed => write!(f, "Encryption failed"),
            AdvancedCryptoError::DecryptionFailed => write!(f, "Decryption failed"),
            AdvancedCryptoError::KeyGenerationFailed => write!(f, "Key generation failed"),
            AdvancedCryptoError::SecurityViolation => write!(f, "Security violation"),
        }
    }
}

impl std::error::Error for AdvancedCryptoError {}
use std::alloc::{alloc_zeroed, dealloc, Layout};

/// Secure memory management for sensitive data
pub struct SecureMemory {
    ptr: *mut u8,
    size: usize,
    layout: Layout,
}

impl SecureMemory {
    pub fn new(size: usize) -> Result<Self, String> {
        if size == 0 {
            return Err("Size cannot be zero".to_string());
        }

        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| "Invalid layout")?;

        let ptr = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err("Failed to allocate secure memory".to_string());
        }

        Ok(Self { ptr, size, layout })
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn zero(&mut self) {
        unsafe {
            ptr::write_bytes(self.ptr, 0, self.size);
        }
    }
}

impl Drop for SecureMemory {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // Zero the memory before deallocation
            unsafe {
                ptr::write_bytes(self.ptr, 0, self.size);
                dealloc(self.ptr, self.layout);
            }
        }
    }
}

unsafe impl Send for SecureMemory {}
unsafe impl Sync for SecureMemory {}

/// Trait for types that should be zeroed on drop
pub trait ZeroOnDrop {
    fn zero_on_drop(&mut self);
}

impl ZeroOnDrop for Vec<u8> {
    fn zero_on_drop(&mut self) {
        clear_sensitive_data(self);
    }
}

impl ZeroOnDrop for String {
    fn zero_on_drop(&mut self) {
        unsafe {
            let bytes = self.as_bytes_mut();
            ptr::write_bytes(bytes.as_mut_ptr(), 0, bytes.len());
        }
    }
}

impl ZeroOnDrop for [u8] {
    fn zero_on_drop(&mut self) {
        unsafe {
            ptr::write_bytes(self.as_mut_ptr(), 0, self.len());
        }
    }
}

/// Clear sensitive data from memory
pub fn clear_sensitive_data<T: ZeroOnDrop>(data: &mut T) {
    data.zero_on_drop();
}

/// Secure string type that zeros on drop
pub struct SecureString {
    memory: SecureMemory,
    len: usize,
}

impl SecureString {
    pub fn new(capacity: usize) -> Result<Self, String> {
        Ok(Self {
            memory: SecureMemory::new(capacity)?,
            len: 0,
        })
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut secure = Self::new(s.len())?;
        secure.set_content(s.as_bytes())?;
        Ok(secure)
    }

    pub fn set_content(&mut self, content: &[u8]) -> Result<(), String> {
        if content.len() > self.memory.size() {
            return Err("Content too large for secure string".to_string());
        }

        let slice = self.memory.as_mut_slice();
        slice[..content.len()].copy_from_slice(content);
        self.len = content.len();
        Ok(())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.memory.as_slice()[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.memory.zero();
        self.len = 0;
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        self.clear();
    }
}

impl ZeroOnDrop for SecureString {
    fn zero_on_drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_memory() {
        let mut mem = SecureMemory::new(32).unwrap();
        assert_eq!(mem.size(), 32);
        
        let slice = mem.as_mut_slice();
        slice[0] = 42;
        assert_eq!(slice[0], 42);
        
        mem.zero();
        assert_eq!(slice[0], 0);
    }

    #[test]
    fn test_secure_string() {
        let mut s = SecureString::from_str("hello").unwrap();
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_bytes(), b"hello");
        
        s.clear();
        assert_eq!(s.len(), 0);
    }
}
