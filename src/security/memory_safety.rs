//! Memory safety primitives for CURSED runtime
//! Provides secure alternatives to unsafe operations

use crate::error::CursedError;
use std::alloc::{Layout, alloc, dealloc};
use std::ptr::NonNull;
use std::mem::{size_of, align_of, MaybeUninit};

/// Safe wrapper for raw memory allocation
pub struct SecureMemoryRegion {
    ptr: NonNull<u8>,
    size: usize,
    layout: Layout,
    guard_magic: u64,
}

const GUARD_MAGIC: u64 = 0xDEADBEEFCAFEBABE;
const CANARY_SIZE: usize = 8;

impl SecureMemoryRegion {
    /// Allocate memory with guard pages and canaries
    pub fn allocate(size: usize) -> Result<Self, CursedError> {
        if size == 0 {
            return Err(CursedError::runtime_error("Cannot allocate zero bytes"));
        }

        // Add space for canaries at start and end
        let total_size = size + (2 * CANARY_SIZE);
        let layout = Layout::from_size_align(total_size, 8)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Memory allocation failed"));
        }

        let non_null = NonNull::new(ptr)
            .ok_or_else(|| CursedError::runtime_error("Null pointer from allocator"))?;

        let mut region = Self {
            ptr: non_null,
            size: total_size,
            layout,
            guard_magic: GUARD_MAGIC,
        };

        // Initialize canaries
        region.write_canaries()?;
        Ok(region)
    }

    /// Write guard canaries to detect buffer overflows
    fn write_canaries(&mut self) -> Result<(), CursedError> {
        let ptr = self.ptr.as_ptr();
        
        // Start canary
        unsafe {
            std::ptr::write(ptr as *mut u64, GUARD_MAGIC);
        }
        
        // End canary
        unsafe {
            let end_ptr = ptr.add(self.size - CANARY_SIZE) as *mut u64;
            std::ptr::write(end_ptr, GUARD_MAGIC);
        }
        
        Ok(())
    }

    /// Verify canaries are intact
    pub fn check_canaries(&self) -> Result<(), CursedError> {
        let ptr = self.ptr.as_ptr();
        
        // Check start canary
        let start_canary = unsafe { std::ptr::read(ptr as *const u64) };
        if start_canary != GUARD_MAGIC {
            return Err(CursedError::runtime_error("Memory corruption detected: start canary"));
        }
        
        // Check end canary
        let end_canary = unsafe { 
            let end_ptr = ptr.add(self.size - CANARY_SIZE) as *const u64;
            std::ptr::read(end_ptr)
        };
        if end_canary != GUARD_MAGIC {
            return Err(CursedError::runtime_error("Memory corruption detected: end canary"));
        }
        
        Ok(())
    }

    /// Get usable memory slice (between canaries)
    pub fn as_slice_mut(&mut self) -> Result<&mut [u8], CursedError> {
        self.check_canaries()?;
        
        let ptr = self.ptr.as_ptr();
        let start = unsafe { ptr.add(CANARY_SIZE) };
        let len = self.size - (2 * CANARY_SIZE);
        
        Ok(unsafe { std::slice::from_raw_parts_mut(start, len) })
    }

    /// Secure zero memory on drop
    pub fn secure_zero(&mut self) -> Result<(), CursedError> {
        self.check_canaries()?;
        
        let slice = self.as_slice_mut()?;
        // Use volatile writes to prevent optimization
        for byte in slice.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
        
        Ok(())
    }
}

impl Drop for SecureMemoryRegion {
    fn drop(&mut self) {
        // Always check canaries on drop
        if let Err(e) = self.check_canaries() {
            eprintln!("SECURITY WARNING: {}", e);
            std::process::abort(); // Fail fast on corruption
        }
        
        // Secure zero before deallocation
        let _ = self.secure_zero();
        
        unsafe {
            dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

/// Safe transmute alternative using type validation
pub fn safe_transmute<T, U>(value: T) -> Result<U, CursedError>
where
    T: Copy,
    U: Copy,
{
    // Validate size compatibility
    if size_of::<T>() != size_of::<U>() {
        return Err(CursedError::runtime_error(&format!(
            "Type size mismatch: {} vs {}", 
            size_of::<T>(), 
            size_of::<U>()
        )));
    }

    // Validate alignment compatibility
    if align_of::<T>() > align_of::<U>() {
        return Err(CursedError::runtime_error(&format!(
            "Alignment incompatible: {} > {}", 
            align_of::<T>(), 
            align_of::<U>()
        )));
    }

    let mut uninit: MaybeUninit<U> = MaybeUninit::uninit();
    
    unsafe {
        let src_ptr = &value as *const T as *const u8;
        let dst_ptr = uninit.as_mut_ptr() as *mut u8;
        std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, size_of::<T>());
    }
    
    Ok(unsafe { uninit.assume_init() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_memory_allocation() {
        let mut region = SecureMemoryRegion::allocate(1024).unwrap();
        
        // Should be able to get slice
        let slice = region.as_slice_mut().unwrap();
        assert_eq!(slice.len(), 1024);
        
        // Should detect canary integrity
        region.check_canaries().unwrap();
        
        // Writing to slice should not corrupt canaries
        slice[0] = 42;
        slice[1023] = 24;
        region.check_canaries().unwrap();
    }

    #[test]
    fn test_safe_transmute() {
        let x: u32 = 0x12345678;
        let y: i32 = safe_transmute(x).unwrap();
        assert_eq!(y, 0x12345678u32 as i32);
        
        // Should fail for different sizes
        let result: Result<u64, _> = safe_transmute(x);
        assert!(result.is_err());
    }
}
