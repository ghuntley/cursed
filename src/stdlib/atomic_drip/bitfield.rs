use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use crate::error::Result as CursedResult;
use super::{MemoryOrder, atomic_error};

/// Atomic 32-bit bitfield for efficient bit operations
#[derive(Debug)]
pub struct Bitfield32 {
    inner: AtomicU32,
}

impl Bitfield32 {
    /// Create a new bitfield with initial value
    pub fn new(initial_value: u32) -> Self {
        Self {
            inner: AtomicU32::new(initial_value),
        }
    }

    /// Load the current bitfield value
    pub fn load(&self) -> u32 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Load with specified memory ordering
    pub fn load_ordered(&self, order: MemoryOrder) -> u32 {
        self.inner.load(order.to_std_ordering())
    }

    /// Store a new bitfield value
    pub fn store(&self, value: u32) {
        self.store_ordered(value, MemoryOrder::SequentiallyConsistent)
    }

    /// Store with specified memory ordering
    pub fn store_ordered(&self, value: u32, order: MemoryOrder) {
        self.inner.store(value, order.to_std_ordering())
    }

    /// Set a specific bit (0-31)
    pub fn set_bit(&self, bit_position: u32) -> CursedResult<()> {
        if bit_position >= 32 {
            return Err(atomic_error("Bit position out of range for 32-bit bitfield"));
        }
        
        let mask = 1u32 << bit_position;
        self.inner.fetch_or(mask, Ordering::SeqCst);
        Ok(())
    }

    /// Set a specific bit with memory ordering
    pub fn set_bit_ordered(&self, bit_position: u32, order: MemoryOrder) -> CursedResult<()> {
        if bit_position >= 32 {
            return Err(atomic_error("Bit position out of range for 32-bit bitfield"));
        }
        
        let mask = 1u32 << bit_position;
        self.inner.fetch_or(mask, order.to_std_ordering());
        Ok(())
    }

    /// Clear a specific bit (0-31)
    pub fn clear_bit(&self, bit_position: u32) -> CursedResult<()> {
        if bit_position >= 32 {
            return Err(atomic_error("Bit position out of range for 32-bit bitfield"));
        }
        
        let mask = !(1u32 << bit_position);
        self.inner.fetch_and(mask, Ordering::SeqCst);
        Ok(())
    }

    /// Clear a specific bit with memory ordering
    pub fn clear_bit_ordered(&self, bit_position: u32, order: MemoryOrder) -> CursedResult<()> {
        if bit_position >= 32 {
            return Err(atomic_error("Bit position out of range for 32-bit bitfield"));
        }
        
        let mask = !(1u32 << bit_position);
        self.inner.fetch_and(mask, order.to_std_ordering());
        Ok(())
    }

    /// Test if a specific bit is set
    pub fn test_bit(&self, bit_position: u32) -> CursedResult<bool> {
        if bit_position >= 32 {
            return Err(atomic_error("Bit position out of range for 32-bit bitfield"));
        }
        
        let current = self.load();
        let mask = 1u32 << bit_position;
        Ok((current & mask) != 0)
    }

    /// Set multiple bits using a mask
    pub fn set_bits(&self, mask: u32) {
        self.set_bits_ordered(mask, MemoryOrder::SequentiallyConsistent)
    }

    /// Set multiple bits with memory ordering
    pub fn set_bits_ordered(&self, mask: u32, order: MemoryOrder) {
        self.inner.fetch_or(mask, order.to_std_ordering());
    }

    /// Clear multiple bits using a mask
    pub fn clear_bits(&self, mask: u32) {
        self.clear_bits_ordered(mask, MemoryOrder::SequentiallyConsistent)
    }

    /// Clear multiple bits with memory ordering
    pub fn clear_bits_ordered(&self, mask: u32, order: MemoryOrder) {
        self.inner.fetch_and(!mask, order.to_std_ordering());
    }

    /// Toggle a specific bit
    pub fn toggle_bit(&self, bit_position: u32) -> CursedResult<()> {
        if bit_position >= 32 {
            return Err(atomic_error("Bit position out of range for 32-bit bitfield"));
        }
        
        let mask = 1u32 << bit_position;
        self.inner.fetch_xor(mask, Ordering::SeqCst);
        Ok(())
    }

    /// Count the number of set bits (population count)
    pub fn count_set_bits(&self) -> u32 {
        self.load().count_ones()
    }

    /// Find the first set bit (least significant bit)
    /// Returns None if no bits are set
    pub fn find_first_set(&self) -> Option<u32> {
        let value = self.load();
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    /// Find the last set bit (most significant bit)
    /// Returns None if no bits are set
    pub fn find_last_set(&self) -> Option<u32> {
        let value = self.load();
        if value == 0 {
            None
        } else {
            Some(31 - value.leading_zeros())
        }
    }

    /// Check if all bits are clear
    pub fn is_empty(&self) -> bool {
        self.load() == 0
    }

    /// Check if any bit is set
    pub fn is_any_set(&self) -> bool {
        self.load() != 0
    }
}

impl Default for Bitfield32 {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Atomic 64-bit bitfield for efficient bit operations
#[derive(Debug)]
pub struct Bitfield64 {
    inner: AtomicU64,
}

impl Bitfield64 {
    /// Create a new bitfield with initial value
    pub fn new(initial_value: u64) -> Self {
        Self {
            inner: AtomicU64::new(initial_value),
        }
    }

    /// Load the current bitfield value
    pub fn load(&self) -> u64 {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Load with specified memory ordering
    pub fn load_ordered(&self, order: MemoryOrder) -> u64 {
        self.inner.load(order.to_std_ordering())
    }

    /// Store a new bitfield value
    pub fn store(&self, value: u64) {
        self.store_ordered(value, MemoryOrder::SequentiallyConsistent)
    }

    /// Store with specified memory ordering
    pub fn store_ordered(&self, value: u64, order: MemoryOrder) {
        self.inner.store(value, order.to_std_ordering())
    }

    /// Set a specific bit (0-63)
    pub fn set_bit(&self, bit_position: u32) -> CursedResult<()> {
        if bit_position >= 64 {
            return Err(atomic_error("Bit position out of range for 64-bit bitfield"));
        }
        
        let mask = 1u64 << bit_position;
        self.inner.fetch_or(mask, Ordering::SeqCst);
        Ok(())
    }

    /// Set a specific bit with memory ordering
    pub fn set_bit_ordered(&self, bit_position: u32, order: MemoryOrder) -> CursedResult<()> {
        if bit_position >= 64 {
            return Err(atomic_error("Bit position out of range for 64-bit bitfield"));
        }
        
        let mask = 1u64 << bit_position;
        self.inner.fetch_or(mask, order.to_std_ordering());
        Ok(())
    }

    /// Clear a specific bit (0-63)
    pub fn clear_bit(&self, bit_position: u32) -> CursedResult<()> {
        if bit_position >= 64 {
            return Err(atomic_error("Bit position out of range for 64-bit bitfield"));
        }
        
        let mask = !(1u64 << bit_position);
        self.inner.fetch_and(mask, Ordering::SeqCst);
        Ok(())
    }

    /// Clear a specific bit with memory ordering
    pub fn clear_bit_ordered(&self, bit_position: u32, order: MemoryOrder) -> CursedResult<()> {
        if bit_position >= 64 {
            return Err(atomic_error("Bit position out of range for 64-bit bitfield"));
        }
        
        let mask = !(1u64 << bit_position);
        self.inner.fetch_and(mask, order.to_std_ordering());
        Ok(())
    }

    /// Test if a specific bit is set
    pub fn test_bit(&self, bit_position: u32) -> CursedResult<bool> {
        if bit_position >= 64 {
            return Err(atomic_error("Bit position out of range for 64-bit bitfield"));
        }
        
        let current = self.load();
        let mask = 1u64 << bit_position;
        Ok((current & mask) != 0)
    }

    /// Set multiple bits using a mask
    pub fn set_bits(&self, mask: u64) {
        self.set_bits_ordered(mask, MemoryOrder::SequentiallyConsistent)
    }

    /// Set multiple bits with memory ordering
    pub fn set_bits_ordered(&self, mask: u64, order: MemoryOrder) {
        self.inner.fetch_or(mask, order.to_std_ordering());
    }

    /// Clear multiple bits using a mask
    pub fn clear_bits(&self, mask: u64) {
        self.clear_bits_ordered(mask, MemoryOrder::SequentiallyConsistent)
    }

    /// Clear multiple bits with memory ordering
    pub fn clear_bits_ordered(&self, mask: u64, order: MemoryOrder) {
        self.inner.fetch_and(!mask, order.to_std_ordering());
    }

    /// Toggle a specific bit
    pub fn toggle_bit(&self, bit_position: u32) -> CursedResult<()> {
        if bit_position >= 64 {
            return Err(atomic_error("Bit position out of range for 64-bit bitfield"));
        }
        
        let mask = 1u64 << bit_position;
        self.inner.fetch_xor(mask, Ordering::SeqCst);
        Ok(())
    }

    /// Count the number of set bits (population count)
    pub fn count_set_bits(&self) -> u32 {
        self.load().count_ones()
    }

    /// Find the first set bit (least significant bit)
    /// Returns None if no bits are set
    pub fn find_first_set(&self) -> Option<u32> {
        let value = self.load();
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    /// Find the last set bit (most significant bit)
    /// Returns None if no bits are set
    pub fn find_last_set(&self) -> Option<u32> {
        let value = self.load();
        if value == 0 {
            None
        } else {
            Some(63 - value.leading_zeros())
        }
    }

    /// Check if all bits are clear
    pub fn is_empty(&self) -> bool {
        self.load() == 0
    }

    /// Check if any bit is set
    pub fn is_any_set(&self) -> bool {
        self.load() != 0
    }
}

impl Default for Bitfield64 {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Create a new 32-bit bitfield
pub fn new_bitfield32(initial_value: u32) -> Bitfield32 {
    Bitfield32::new(initial_value)
}

/// Create a new 64-bit bitfield
pub fn new_bitfield64(initial_value: u64) -> Bitfield64 {
    Bitfield64::new(initial_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_bitfield32_basic() {
        let bf = Bitfield32::new(0);
        assert_eq!(bf.load(), 0);
        assert!(bf.is_empty());
        assert!(!bf.is_any_set());
        
        bf.set_bit(0).unwrap();
        assert_eq!(bf.load(), 1);
        assert!(bf.test_bit(0).unwrap());
        assert!(!bf.test_bit(1).unwrap());
        assert!(!bf.is_empty());
        assert!(bf.is_any_set());
        
        bf.set_bit(2).unwrap();
        assert_eq!(bf.load(), 5); // 0b101
        assert!(bf.test_bit(0).unwrap());
        assert!(!bf.test_bit(1).unwrap());
        assert!(bf.test_bit(2).unwrap());
        
        bf.clear_bit(0).unwrap();
        assert_eq!(bf.load(), 4); // 0b100
        assert!(!bf.test_bit(0).unwrap());
        assert!(bf.test_bit(2).unwrap());
    }

    #[test]
    fn test_bitfield32_set_bits() {
        let bf = Bitfield32::new(0);
        
        bf.set_bits(0x3); // Set bits 0 and 1
        assert_eq!(bf.load(), 3);
        assert!(bf.test_bit(0).unwrap());
        assert!(bf.test_bit(1).unwrap());
        assert!(!bf.test_bit(2).unwrap());
        
        bf.clear_bits(0x1); // Clear bit 0
        assert_eq!(bf.load(), 2);
        assert!(!bf.test_bit(0).unwrap());
        assert!(bf.test_bit(1).unwrap());
    }

    #[test]
    fn test_bitfield32_toggle() {
        let bf = Bitfield32::new(0);
        
        bf.toggle_bit(0).unwrap();
        assert!(bf.test_bit(0).unwrap());
        
        bf.toggle_bit(0).unwrap();
        assert!(!bf.test_bit(0).unwrap());
    }

    #[test]
    fn test_bitfield32_count_operations() {
        let bf = Bitfield32::new(0b1010101);
        assert_eq!(bf.count_set_bits(), 4);
        
        assert_eq!(bf.find_first_set(), Some(0));
        assert_eq!(bf.find_last_set(), Some(6));
        
        let empty_bf = Bitfield32::new(0);
        assert_eq!(empty_bf.find_first_set(), None);
        assert_eq!(empty_bf.find_last_set(), None);
    }

    #[test]
    fn test_bitfield32_out_of_range() {
        let bf = Bitfield32::new(0);
        
        assert!(bf.set_bit(32).is_err());
        assert!(bf.clear_bit(32).is_err());
        assert!(bf.test_bit(32).is_err());
        assert!(bf.toggle_bit(32).is_err());
    }

    #[test]
    fn test_bitfield64_basic() {
        let bf = Bitfield64::new(0);
        assert_eq!(bf.load(), 0);
        
        bf.set_bit(63).unwrap(); // Set highest bit
        assert_eq!(bf.load(), 1u64 << 63);
        assert!(bf.test_bit(63).unwrap());
        
        bf.clear_bit(63).unwrap();
        assert_eq!(bf.load(), 0);
        assert!(!bf.test_bit(63).unwrap());
    }

    #[test]
    fn test_bitfield64_out_of_range() {
        let bf = Bitfield64::new(0);
        
        assert!(bf.set_bit(64).is_err());
        assert!(bf.clear_bit(64).is_err());
        assert!(bf.test_bit(64).is_err());
        assert!(bf.toggle_bit(64).is_err());
    }

    #[test]
    fn test_bitfield_concurrent() {
        let bf = Arc::new(Bitfield32::new(0));
        let mut handles = vec![];
        
        // Each thread sets a different bit
        for i in 0..16 {
            let bf_clone = Arc::clone(&bf);
            let handle = thread::spawn(move || {
                bf_clone.set_bit(i).unwrap();
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Check that all bits are set
        for i in 0..16 {
            assert!(bf.test_bit(i).unwrap());
        }
        
        assert_eq!(bf.count_set_bits(), 16);
    }

    #[test]
    fn test_memory_ordering() {
        let bf = Bitfield32::new(0);
        
        bf.set_bit_ordered(5, MemoryOrder::Release).unwrap();
        let value = bf.load_ordered(MemoryOrder::Acquire);
        assert_eq!(value & (1 << 5), 1 << 5);
        
        bf.clear_bit_ordered(5, MemoryOrder::Relaxed).unwrap();
        assert!(!bf.test_bit(5).unwrap());
    }
}
