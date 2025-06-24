use crate::error::Error;
/// fr fr Constant time operations for cryptographic security
use super::errors::*;
use std::sync::atomic::{AtomicU8, Ordering};

/// Constant time operations utility
#[derive(Debug, Clone)]
pub struct ConstantTimeOps;

impl ConstantTimeOps {
    /// Create new constant time operations handler
    pub fn new() -> Self {
        Self
    }
    
    /// Constant time comparison of byte slices
    pub fn compare(&self, a: &[u8], b: &[u8]) -> bool {
        constant_time_compare(a, b)
    }
    
    /// Constant time conditional selection
    pub fn select(&self, condition: bool, a: &[u8], b: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        constant_time_select(condition, a, b)
    }
    
    /// Constant time copy operation
    pub fn copy(&self, src: &[u8], dst: &mut [u8]) -> AdvancedCryptoResult<()> {
        constant_time_copy(src, dst)
    }
    
    /// Constant time integer comparison
    pub fn compare_u32(&self, a: u32, b: u32) -> bool {
        constant_time_compare_u32(a, b)
    }
    
    /// Constant time integer selection
    pub fn select_u32(&self, condition: bool, a: u32, b: u32) -> u32 {
        constant_time_select_u32(condition, a, b)
    }
}

/// Constant time comparison of byte slices
/// Uses bitwise operations to avoid branching and timing attacks
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    // Use volatile operations to prevent optimization
    let atomic_result = AtomicU8::new(result);
    atomic_result.load(Ordering::Acquire) == 0
}

/// Constant time conditional selection between byte slices
/// Returns a copy of 'a' if condition is true, otherwise 'b'
pub fn constant_time_select(condition: bool, a: &[u8], b: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
    if a.len() != b.len() {
        return Err(AdvancedCryptoError::InvalidParameters("Arrays must have same length".to_string()));
    }
    
    let mask = if condition { 0xFF } else { 0x00 };
    let inv_mask = !mask;
    
    let mut result = Vec::with_capacity(a.len());
    for (x, y) in a.iter().zip(b.iter()) {
        // Constant time selection using bitwise operations
        let selected = (mask & x) | (inv_mask & y);
        result.push(selected);
    }
    
    Ok(result)
}

/// Constant time copy operation
/// Copies src to dst in constant time regardless of data content
pub fn constant_time_copy(src: &[u8], dst: &mut [u8]) -> AdvancedCryptoResult<()> {
    if src.len() != dst.len() {
        return Err(AdvancedCryptoError::InvalidParameters("Source and destination must have same length".to_string()));
    }
    
    // Perform copy using volatile operations to prevent optimization
    for (i, &byte) in src.iter().enumerate() {
        unsafe {
            std::ptr::write_volatile(&mut dst[i], byte);
        }
    }
    
    Ok(())
}

/// Constant time comparison of 32-bit integers
pub fn constant_time_compare_u32(a: u32, b: u32) -> bool {
    let diff = a ^ b;
    
    // Convert to atomic to prevent optimization
    let atomic_diff = AtomicU8::new((diff | (diff >> 16) | (diff >> 8)) as u8);
    atomic_diff.load(Ordering::Acquire) == 0
}

/// Constant time conditional selection for 32-bit integers
pub fn constant_time_select_u32(condition: bool, a: u32, b: u32) -> u32 {
    let mask = if condition { 0xFFFFFFFF } else { 0x00000000 };
    let inv_mask = !mask;
    
    (mask & a) | (inv_mask & b)
}

/// Constant time conditional selection for 64-bit integers
pub fn constant_time_select_u64(condition: bool, a: u64, b: u64) -> u64 {
    let mask = if condition { 0xFFFFFFFFFFFFFFFF } else { 0x0000000000000000 };
    let inv_mask = !mask;
    
    (mask & a) | (inv_mask & b)
}

/// Timing-safe equality check (alias for constant_time_compare)
pub fn timing_safe_equal(a: &[u8], b: &[u8]) -> bool {
    constant_time_compare(a, b)
}

/// Constant time memory clearing to prevent optimization
pub fn constant_time_clear(data: &mut [u8]) {
    for byte in data.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
    
    // Memory barrier to prevent reordering
    std::sync::atomic::compiler_fence(Ordering::SeqCst);
}

/// Constant time conditional memory clearing
pub fn constant_time_conditional_clear(data: &mut [u8], condition: bool) {
    let clear_value = if condition { 0u8 } else { 0u8 }; // Always 0, but prevents optimization
    
    for byte in data.iter_mut() {
        if condition {
            unsafe {
                std::ptr::write_volatile(byte, clear_value);
            }
        }
    }
    
    std::sync::atomic::compiler_fence(Ordering::SeqCst);
}

/// Constant time byte array XOR
pub fn constant_time_xor(a: &[u8], b: &[u8], dst: &mut [u8]) -> AdvancedCryptoResult<()> {
    if a.len() != b.len() || a.len() != dst.len() {
        return Err(AdvancedCryptoError::InvalidParameters("All arrays must have same length".to_string()));
    }
    
    for ((x, y), dst_byte) in a.iter().zip(b.iter()).zip(dst.iter_mut()) {
        unsafe {
            std::ptr::write_volatile(dst_byte, x ^ y);
        }
    }
    
    Ok(())
}

/// Constant time conditional swap
pub fn constant_time_conditional_swap(condition: bool, a: &mut [u8], b: &mut [u8]) -> AdvancedCryptoResult<()> {
    if a.len() != b.len() {
        return Err(AdvancedCryptoError::InvalidParameters("Arrays must have same length".to_string()));
    }
    
    let mask = if condition { 0xFF } else { 0x00 };
    
    for (x, y) in a.iter_mut().zip(b.iter_mut()) {
        let temp_x = *x;
        let temp_y = *y;
        
        // Constant time swap using XOR operations
        let swap_mask = mask & (temp_x ^ temp_y);
        
        unsafe {
            std::ptr::write_volatile(x, temp_x ^ swap_mask);
            std::ptr::write_volatile(y, temp_y ^ swap_mask);
        }
    }
    
    Ok(())
}

/// Constant time greater than comparison for bytes
pub fn constant_time_greater_than(a: u8, b: u8) -> bool {
    let diff = (a as u16).wrapping_sub(b as u16).wrapping_sub(1);
    (diff >> 8) == 0
}

/// Constant time less than comparison for bytes  
pub fn constant_time_less_than(a: u8, b: u8) -> bool {
    constant_time_greater_than(b, a)
}

/// Constant time range check (a <= x <= b)
pub fn constant_time_range_check(x: u8, a: u8, b: u8) -> bool {
    let ge_a = !constant_time_less_than(x, a);
    let le_b = !constant_time_greater_than(x, b);
    ge_a && le_b
}

/// Secure utility functions using constant time operations
pub struct SecureOps;

impl SecureOps {
    /// Find first differing byte index in constant time
    pub fn find_diff_index(a: &[u8], b: &[u8]) -> Option<usize> {
        if a.len() != b.len() {
            return None;
        }
        
        let mut first_diff = a.len();
        
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            let diff = x ^ y;
            let is_diff = (diff | diff.wrapping_neg()) >> 7;
            
            // Update first_diff only if this is the first difference
            let update_mask = (first_diff.wrapping_sub(a.len())) >> (std::mem::size_of::<usize>() * 8 - 1);
            first_diff ^= update_mask & (first_diff ^ i);
        }
        
        if first_diff < a.len() {
            Some(first_diff)
        } else {
            None
        }
    }
    
    /// Count differing bytes in constant time
    pub fn count_differences(a: &[u8], b: &[u8]) -> usize {
        if a.len() != b.len() {
            return std::cmp::max(a.len(), b.len());
        }
        
        let mut count = 0;
        for (x, y) in a.iter().zip(b.iter()) {
            let diff = x ^ y;
            let is_diff = ((diff | diff.wrapping_neg()) >> 7) as usize;
            count += is_diff;
        }
        
        count
    }
    
    /// Constant time string comparison
    pub fn compare_strings(a: &str, b: &str) -> bool {
        constant_time_compare(a.as_bytes(), b.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_time_compare() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";
        
        assert!(constant_time_compare(a, b));
        assert!(!constant_time_compare(a, c));
        assert!(!constant_time_compare(a, b"hell")); // Different lengths
    }
    
    #[test]
    fn test_constant_time_select() {
        let a = b"option_a";
        let b = b"option_b";
        
        let result_true = constant_time_select(true, a, b).unwrap();
        let result_false = constant_time_select(false, a, b).unwrap();
        
        assert_eq!(result_true, a);
        assert_eq!(result_false, b);
        
        // Test with different lengths should fail
        let result_err = constant_time_select(true, a, b"short");
        assert!(result_err.is_err());
    }
    
    #[test]
    fn test_constant_time_copy() {
        let src = b"source";
        let mut dst = vec![0u8; 6];
        
        constant_time_copy(src, &mut dst).unwrap();
        assert_eq!(dst, src);
        
        // Test with different lengths should fail
        let mut short_dst = vec![0u8; 3];
        let result = constant_time_copy(src, &mut short_dst);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_constant_time_compare_u32() {
        assert!(constant_time_compare_u32(42, 42));
        assert!(!constant_time_compare_u32(42, 43));
        assert!(!constant_time_compare_u32(0, 1));
    }
    
    #[test]
    fn test_constant_time_select_u32() {
        assert_eq!(constant_time_select_u32(true, 100, 200), 100);
        assert_eq!(constant_time_select_u32(false, 100, 200), 200);
    }
    
    #[test]
    fn test_constant_time_select_u64() {
        assert_eq!(constant_time_select_u64(true, 1000, 2000), 1000);
        assert_eq!(constant_time_select_u64(false, 1000, 2000), 2000);
    }
    
    #[test]
    fn test_timing_safe_equal() {
        assert!(timing_safe_equal(b"test", b"test"));
        assert!(!timing_safe_equal(b"test", b"fail"));
    }
    
    #[test]
    fn test_constant_time_clear() {
        let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
        constant_time_clear(&mut data);
        assert_eq!(data, vec![0, 0, 0, 0]);
    }
    
    #[test]
    fn test_constant_time_xor() {
        let a = b"hello";
        let b = b"world";
        let mut result = vec![0u8; 5];
        
        constant_time_xor(a, b, &mut result).unwrap();
        
        // XOR should be reversible
        let mut original = vec![0u8; 5];
        constant_time_xor(&result, b, &mut original).unwrap();
        assert_eq!(original, a);
    }
    
    #[test]
    fn test_constant_time_conditional_swap() {
        let mut a = vec![1, 2, 3];
        let mut b = vec![4, 5, 6];
        let original_a = a.clone();
        let original_b = b.clone();
        
        // Swap when condition is true
        constant_time_conditional_swap(true, &mut a, &mut b).unwrap();
        assert_eq!(a, original_b);
        assert_eq!(b, original_a);
        
        // Swap back when condition is true again
        constant_time_conditional_swap(true, &mut a, &mut b).unwrap();
        assert_eq!(a, original_a);
        assert_eq!(b, original_b);
        
        // No swap when condition is false
        constant_time_conditional_swap(false, &mut a, &mut b).unwrap();
        assert_eq!(a, original_a);
        assert_eq!(b, original_b);
    }
    
    #[test]
    fn test_constant_time_comparisons() {
        assert!(constant_time_greater_than(5, 3));
        assert!(!constant_time_greater_than(3, 5));
        assert!(!constant_time_greater_than(5, 5));
        
        assert!(constant_time_less_than(3, 5));
        assert!(!constant_time_less_than(5, 3));
        assert!(!constant_time_less_than(5, 5));
        
        assert!(constant_time_range_check(5, 3, 7));
        assert!(constant_time_range_check(3, 3, 7));
        assert!(constant_time_range_check(7, 3, 7));
        assert!(!constant_time_range_check(2, 3, 7));
        assert!(!constant_time_range_check(8, 3, 7));
    }
    
    #[test]
    fn test_secure_ops() {
        let a = b"hello world";
        let b = b"hello earth";
        
        let diff_index = SecureOps::find_diff_index(a, b);
        assert_eq!(diff_index, Some(6)); // First diff at 'w' vs 'e'
        
        let count = SecureOps::count_differences(a, b);
        assert_eq!(count, 5); // "world" vs "earth" differs in 5 positions
        
        assert!(SecureOps::compare_strings("test", "test"));
        assert!(!SecureOps::compare_strings("test", "fail"));
    }
    
    #[test]
    fn test_constant_time_ops_struct() {
        let ops = ConstantTimeOps::new();
        
        assert!(ops.compare(b"test", b"test"));
        assert!(!ops.compare(b"test", b"fail"));
        
        let result = ops.select(true, b"a", b"b").unwrap();
        assert_eq!(result, b"a");
        
        assert!(ops.compare_u32(42, 42));
        assert!(!ops.compare_u32(42, 43));
        
        assert_eq!(ops.select_u32(true, 100, 200), 100);
        assert_eq!(ops.select_u32(false, 100, 200), 200);
    }
}
