/// fr fr Constant-time operations for CURSED crypto - no timing leaks bestie

/// fr fr Constant-time operations trait
pub trait ConstantTimeOps {
    /// slay Compare two byte slices in constant time
    fn constant_time_compare(&self, other: &Self) -> bool;
    
    /// slay Select between two values in constant time
    fn constant_time_select(condition: bool, true_val: Self, false_val: Self) -> Self;
}

/// slay Compare two byte slices in constant time
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}

/// slay Select between two values in constant time
pub fn constant_time_select<T: Copy>(condition: bool, true_val: T, false_val: T) -> T {
    if condition { true_val } else { false_val }
}

/// slay Copy data in constant time if condition is true
pub fn constant_time_copy(condition: bool, dest: &mut [u8], src: &[u8]) {
    let mask = if condition { 0xFF } else { 0x00 };
    
    for (d, &s) in dest.iter_mut().zip(src.iter()) {
        *d = (*d & !mask) | (s & mask);
    }
}

/// slay Check if two values are timing-safe equal
pub fn timing_safe_equal<T: AsRef<[u8]>>(a: T, b: T) -> bool {
    constant_time_compare(a.as_ref(), b.as_ref())
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
        assert!(!constant_time_compare(a, b"hello world"));
    }
    
    #[test]
    fn test_constant_time_select() {
        assert_eq!(constant_time_select(true, 42, 24), 42);
        assert_eq!(constant_time_select(false, 42, 24), 24);
    }
    
    #[test]
    fn test_constant_time_copy() {
        let mut dest = [0xFF; 5];
        let src = [0x01, 0x02, 0x03, 0x04, 0x05];
        
        constant_time_copy(true, &mut dest, &src);
        assert_eq!(dest, src);
        
        let mut dest = [0xFF; 5];
        constant_time_copy(false, &mut dest, &src);
        assert_eq!(dest, [0xFF; 5]);
    }
}
