use std::sync::atomic::Ordering;

/// Memory ordering constraints for atomic operations
/// Controls how memory operations are ordered around atomic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryOrder {
    /// No ordering constraints, only atomicity is guaranteed
    Relaxed,
    /// Acquire ordering for loads - no memory operations after this load can be reordered before it
    Acquire,
    /// Release ordering for stores - no memory operations before this store can be reordered after it
    Release,
    /// Acquire-release ordering - combines acquire and release semantics
    AcquireRelease,
    /// Sequential consistency - strongest ordering, all operations appear in some global order
    SequentiallyConsistent,
}

impl MemoryOrder {
    /// Convert CURSED MemoryOrder to std::sync::atomic::Ordering
    pub fn to_std_ordering(self) -> Ordering {
        match self {
            MemoryOrder::Relaxed => Ordering::Relaxed,
            MemoryOrder::Acquire => Ordering::Acquire,
            MemoryOrder::Release => Ordering::Release,
            MemoryOrder::AcquireRelease => Ordering::AcqRel,
            MemoryOrder::SequentiallyConsistent => Ordering::SeqCst,
        }
    }

    /// Check if this ordering is valid for load operations
    pub fn is_valid_for_load(self) -> bool {
        matches!(self, 
            MemoryOrder::Relaxed | 
            MemoryOrder::Acquire | 
            MemoryOrder::SequentiallyConsistent
        )
    }

    /// Check if this ordering is valid for store operations
    pub fn is_valid_for_store(self) -> bool {
        matches!(self, 
            MemoryOrder::Relaxed | 
            MemoryOrder::Release | 
            MemoryOrder::SequentiallyConsistent
        )
    }

    /// Check if this ordering is valid for compare-and-swap operations
    pub fn is_valid_for_cas(self) -> bool {
        true // All orderings are valid for CAS
    }

    /// Get the appropriate ordering for CAS failure case
    pub fn cas_failure_ordering(self) -> MemoryOrder {
        match self {
            MemoryOrder::Release => MemoryOrder::Relaxed,
            MemoryOrder::AcquireRelease => MemoryOrder::Acquire,
            other => other,
        }
    }
}

impl Default for MemoryOrder {
    fn default() -> Self {
        MemoryOrder::SequentiallyConsistent
    }
}

/// Memory ordering constants for convenience
pub mod memory_order {
    use super::MemoryOrder;

    pub const MEMORY_ORDER_RELAXED: MemoryOrder = MemoryOrder::Relaxed;
    pub const MEMORY_ORDER_ACQUIRE: MemoryOrder = MemoryOrder::Acquire;
    pub const MEMORY_ORDER_RELEASE: MemoryOrder = MemoryOrder::Release;
    pub const MEMORY_ORDER_ACQUIRE_RELEASE: MemoryOrder = MemoryOrder::AcquireRelease;
    pub const MEMORY_ORDER_SEQUENTIALLY_CONSISTENT: MemoryOrder = MemoryOrder::SequentiallyConsistent;
}

// Re-export constants at module level for easy access
pub use memory_order::*;

/// Memory fence operations
pub mod fence {
    use super::MemoryOrder;
    use std::sync::atomic;

    /// Insert a memory fence with the specified ordering
    pub fn memory_fence(order: MemoryOrder) {
        atomic::fence(order.to_std_ordering());
    }

    /// Insert a compiler fence (prevents compiler reordering but not CPU reordering)
    pub fn compiler_fence(order: MemoryOrder) {
        atomic::compiler_fence(order.to_std_ordering());
    }

    /// Full memory barrier - ensures all memory operations are complete
    pub fn full_barrier() {
        memory_fence(MemoryOrder::SequentiallyConsistent);
    }

    /// Acquire barrier - prevents reordering of subsequent reads
    pub fn acquire_barrier() {
        memory_fence(MemoryOrder::Acquire);
    }

    /// Release barrier - prevents reordering of previous writes
    pub fn release_barrier() {
        memory_fence(MemoryOrder::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_order_conversion() {
        assert_eq!(MemoryOrder::Relaxed.to_std_ordering(), Ordering::Relaxed);
        assert_eq!(MemoryOrder::Acquire.to_std_ordering(), Ordering::Acquire);
        assert_eq!(MemoryOrder::Release.to_std_ordering(), Ordering::Release);
        assert_eq!(MemoryOrder::AcquireRelease.to_std_ordering(), Ordering::AcqRel);
        assert_eq!(MemoryOrder::SequentiallyConsistent.to_std_ordering(), Ordering::SeqCst);
    }

    #[test]
    fn test_load_validity() {
        assert!(MemoryOrder::Relaxed.is_valid_for_load());
        assert!(MemoryOrder::Acquire.is_valid_for_load());
        assert!(!MemoryOrder::Release.is_valid_for_load());
        assert!(!MemoryOrder::AcquireRelease.is_valid_for_load());
        assert!(MemoryOrder::SequentiallyConsistent.is_valid_for_load());
    }

    #[test]
    fn test_store_validity() {
        assert!(MemoryOrder::Relaxed.is_valid_for_store());
        assert!(!MemoryOrder::Acquire.is_valid_for_store());
        assert!(MemoryOrder::Release.is_valid_for_store());
        assert!(!MemoryOrder::AcquireRelease.is_valid_for_store());
        assert!(MemoryOrder::SequentiallyConsistent.is_valid_for_store());
    }

    #[test]
    fn test_cas_validity() {
        assert!(MemoryOrder::Relaxed.is_valid_for_cas());
        assert!(MemoryOrder::Acquire.is_valid_for_cas());
        assert!(MemoryOrder::Release.is_valid_for_cas());
        assert!(MemoryOrder::AcquireRelease.is_valid_for_cas());
        assert!(MemoryOrder::SequentiallyConsistent.is_valid_for_cas());
    }

    #[test]
    fn test_cas_failure_ordering() {
        assert_eq!(MemoryOrder::Release.cas_failure_ordering(), MemoryOrder::Relaxed);
        assert_eq!(MemoryOrder::AcquireRelease.cas_failure_ordering(), MemoryOrder::Acquire);
        assert_eq!(MemoryOrder::Relaxed.cas_failure_ordering(), MemoryOrder::Relaxed);
    }

    #[test]
    fn test_default_ordering() {
        assert_eq!(MemoryOrder::default(), MemoryOrder::SequentiallyConsistent);
    }

    #[test]
    fn test_fence_operations() {
        // These should not panic
        fence::memory_fence(MemoryOrder::Relaxed);
        fence::compiler_fence(MemoryOrder::Acquire);
        fence::full_barrier();
        fence::acquire_barrier();
        fence::release_barrier();
    }
}
