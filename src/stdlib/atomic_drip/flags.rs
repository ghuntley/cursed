use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use crate::error::Result as CursedResult;
use super::{MemoryOrder, atomic_error};

/// Specialized atomic flag with extended operations
/// Provides additional functionality beyond basic boolean operations
#[derive(Debug)]
pub struct Flag {
    inner: AtomicBool,
    set_count: AtomicU8, // Track how many times the flag has been set
}

impl Flag {
    /// Create a new flag with initial state
    pub fn new(initial: bool) -> Self {
        Self {
            inner: AtomicBool::new(initial),
            set_count: AtomicU8::new(if initial { 1 } else { 0 }),
        }
    }

    /// Load the current flag state
    pub fn load(&self) -> bool {
        self.load_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Load with specified memory ordering
    pub fn load_ordered(&self, order: MemoryOrder) -> bool {
        self.inner.load(order.to_std_ordering())
    }

    /// Store a new flag state
    pub fn store(&self, value: bool) {
        self.store_ordered(value, MemoryOrder::SequentiallyConsistent)
    }

    /// Store with specified memory ordering
    pub fn store_ordered(&self, value: bool, order: MemoryOrder) {
        self.inner.store(value, order.to_std_ordering());
        if value {
            self.set_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Set the flag to true
    pub fn set(&self) {
        self.store(true)
    }

    /// Set the flag with memory ordering
    pub fn set_ordered(&self, order: MemoryOrder) {
        self.store_ordered(true, order)
    }

    /// Clear the flag to false
    pub fn clear(&self) {
        self.store(false)
    }

    /// Clear the flag with memory ordering
    pub fn clear_ordered(&self, order: MemoryOrder) {
        self.store_ordered(false, order)
    }

    /// Set the flag only if it's currently unset
    /// Returns true if the flag was successfully set, false if it was already set
    pub fn set_if_unset(&self) -> bool {
        self.set_if_unset_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Set the flag only if unset, with memory ordering
    pub fn set_if_unset_ordered(&self, order: MemoryOrder) -> bool {
        let result = self.inner.compare_exchange(
            false, 
            true, 
            order.to_std_ordering(), 
            order.cas_failure_ordering().to_std_ordering()
        );
        
        if result.is_ok() {
            self.set_count.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Clear the flag only if it's currently set
    /// Returns true if the flag was successfully cleared, false if it was already clear
    pub fn clear_if_set(&self) -> bool {
        self.clear_if_set_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Clear the flag only if set, with memory ordering
    pub fn clear_if_set_ordered(&self, order: MemoryOrder) -> bool {
        self.inner.compare_exchange(
            true, 
            false, 
            order.to_std_ordering(), 
            order.cas_failure_ordering().to_std_ordering()
        ).is_ok()
    }

    /// Toggle the flag state and return the previous value
    pub fn toggle(&self) -> bool {
        self.toggle_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Toggle with memory ordering
    pub fn toggle_ordered(&self, order: MemoryOrder) -> bool {
        let old = self.inner.fetch_xor(true, order.to_std_ordering());
        if !old {
            // Was false, now true (just got set)
            self.set_count.fetch_add(1, Ordering::Relaxed);
        }
        old
    }

    /// Test and set: check current value and set to true atomically
    /// Returns the previous value
    pub fn test_and_set(&self) -> bool {
        self.test_and_set_ordered(MemoryOrder::SequentiallyConsistent)
    }

    /// Test and set with memory ordering
    pub fn test_and_set_ordered(&self, order: MemoryOrder) -> bool {
        let old = self.inner.swap(true, order.to_std_ordering());
        if !old {
            self.set_count.fetch_add(1, Ordering::Relaxed);
        }
        old
    }

    /// Get the number of times this flag has been set
    /// Note: This counter can overflow at 255
    pub fn set_count(&self) -> u8 {
        self.set_count.load(Ordering::Relaxed)
    }

    /// Reset the set counter
    pub fn reset_count(&self) {
        self.set_count.store(0, Ordering::Relaxed);
    }

    /// Check if the flag has ever been set
    pub fn has_been_set(&self) -> bool {
        self.set_count() > 0
    }

    /// Wait for the flag to be set (busy wait)
    /// Returns immediately if already set
    pub fn wait_for_set(&self) {
        while !self.load() {
            std::hint::spin_loop();
        }
    }

    /// Wait for the flag to be cleared (busy wait)
    /// Returns immediately if already clear
    pub fn wait_for_clear(&self) {
        while self.load() {
            std::hint::spin_loop();
        }
    }

    /// Compare and swap operation
    pub fn compare_and_swap(&self, expected: bool, new: bool) -> bool {
        self.compare_and_swap_ordered(expected, new, MemoryOrder::SequentiallyConsistent)
    }

    /// Compare and swap with memory ordering
    pub fn compare_and_swap_ordered(&self, expected: bool, new: bool, order: MemoryOrder) -> bool {
        let result = self.inner.compare_exchange(
            expected, 
            new, 
            order.to_std_ordering(), 
            order.cas_failure_ordering().to_std_ordering()
        );
        
        if result.is_ok() && new && !expected {
            // Successfully changed from false to true
            self.set_count.fetch_add(1, Ordering::Relaxed);
        }
        
        result.is_ok()
    }
}

impl Default for Flag {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Multi-state atomic flag that can hold more than just true/false
/// Useful for state machines or multi-phase operations
#[derive(Debug)]
pub struct StateFlag {
    state: AtomicU8,
    max_state: u8,
}

impl StateFlag {
    /// Create a new state flag with maximum number of states
    /// States are numbered 0 to max_states-1
    pub fn new(max_states: u8) -> CursedResult<Self> {
        if max_states == 0 {
            return Err(atomic_error("StateFlag must have at least 1 state"));
        }
        
        Ok(Self {
            state: AtomicU8::new(0),
            max_state: max_states - 1,
        })
    }

    /// Get the current state
    pub fn get(&self) -> u8 {
        self.state.load(Ordering::SeqCst)
    }

    /// Set the state
    pub fn set(&self, state: u8) -> CursedResult<()> {
        if state > self.max_state {
            return Err(atomic_error("State value exceeds maximum"));
        }
        
        self.state.store(state, Ordering::SeqCst);
        Ok(())
    }

    /// Try to advance to the next state
    /// Returns true if successfully advanced, false if already at max state
    pub fn advance(&self) -> bool {
        loop {
            let current = self.state.load(Ordering::SeqCst);
            if current >= self.max_state {
                return false; // Already at max state
            }
            
            if self.state.compare_exchange_weak(
                current,
                current + 1,
                Ordering::SeqCst,
                Ordering::Relaxed
            ).is_ok() {
                return true;
            }
        }
    }

    /// Try to go back to the previous state
    /// Returns true if successfully moved back, false if already at state 0
    pub fn retreat(&self) -> bool {
        loop {
            let current = self.state.load(Ordering::SeqCst);
            if current == 0 {
                return false; // Already at min state
            }
            
            if self.state.compare_exchange_weak(
                current,
                current - 1,
                Ordering::SeqCst,
                Ordering::Relaxed
            ).is_ok() {
                return true;
            }
        }
    }

    /// Reset to state 0
    pub fn reset(&self) {
        self.state.store(0, Ordering::SeqCst);
    }

    /// Check if in specific state
    pub fn is_state(&self, state: u8) -> bool {
        self.get() == state
    }

    /// Check if at maximum state
    pub fn is_max_state(&self) -> bool {
        self.get() == self.max_state
    }

    /// Check if at minimum state (0)
    pub fn is_min_state(&self) -> bool {
        self.get() == 0
    }

    /// Compare and swap states
    pub fn compare_and_swap(&self, expected: u8, new: u8) -> CursedResult<bool> {
        if new > self.max_state {
            return Err(atomic_error("New state value exceeds maximum"));
        }
        
        Ok(self.state.compare_exchange(
            expected,
            new,
            Ordering::SeqCst,
            Ordering::Relaxed
        ).is_ok())
    }

    /// Get the maximum allowed state value
    pub fn max_state(&self) -> u8 {
        self.max_state
    }

    /// Wait for a specific state (busy wait)
    pub fn wait_for_state(&self, target_state: u8) {
        while self.get() != target_state {
            std::hint::spin_loop();
        }
    }
}

/// Create a new atomic flag
pub fn new_flag(initial: bool) -> Flag {
    Flag::new(initial)
}

/// Create a new state flag with the specified number of states
pub fn new_state_flag(max_states: u8) -> CursedResult<StateFlag> {
    StateFlag::new(max_states)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_flag_basic() {
        let flag = Flag::new(false);
        assert!(!flag.load());
        assert_eq!(flag.set_count(), 0);
        assert!(!flag.has_been_set());
        
        flag.set();
        assert!(flag.load());
        assert_eq!(flag.set_count(), 1);
        assert!(flag.has_been_set());
        
        flag.clear();
        assert!(!flag.load());
        assert_eq!(flag.set_count(), 1); // Count doesn't reset on clear
        assert!(flag.has_been_set());
    }

    #[test]
    fn test_flag_set_if_unset() {
        let flag = Flag::new(false);
        
        // First attempt should succeed
        assert!(flag.set_if_unset());
        assert!(flag.load());
        
        // Second attempt should fail
        assert!(!flag.set_if_unset());
        assert!(flag.load());
    }

    #[test]
    fn test_flag_clear_if_set() {
        let flag = Flag::new(true);
        
        // First attempt should succeed
        assert!(flag.clear_if_set());
        assert!(!flag.load());
        
        // Second attempt should fail
        assert!(!flag.clear_if_set());
        assert!(!flag.load());
    }

    #[test]
    fn test_flag_toggle() {
        let flag = Flag::new(false);
        
        let old = flag.toggle();
        assert!(!old); // Previous value was false
        assert!(flag.load()); // Current value is true
        
        let old = flag.toggle();
        assert!(old); // Previous value was true
        assert!(!flag.load()); // Current value is false
    }

    #[test]
    fn test_flag_test_and_set() {
        let flag = Flag::new(false);
        
        let old = flag.test_and_set();
        assert!(!old); // Previous value was false
        assert!(flag.load()); // Current value is true
        
        let old = flag.test_and_set();
        assert!(old); // Previous value was true
        assert!(flag.load()); // Current value is still true
    }

    #[test]
    fn test_flag_concurrent_set_if_unset() {
        let flag = Arc::new(Flag::new(false));
        let mut handles = vec![];
        let mut winners = vec![];
        
        // Multiple threads trying to set the flag
        for i in 0..10 {
            let flag_clone = Arc::clone(&flag);
            let (tx, rx) = std::sync::mpsc::channel();
            winners.push(rx);
            
            let handle = thread::spawn(move || {
                let won = flag_clone.set_if_unset();
                tx.send((i, won)).unwrap();
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Exactly one thread should have won
        let mut winner_count = 0;
        for rx in winners {
            let (thread_id, won) = rx.recv().unwrap();
            if won {
                winner_count += 1;
                println!("Thread {} won the race", thread_id);
            }
        }
        
        assert_eq!(winner_count, 1);
        assert!(flag.load());
        assert_eq!(flag.set_count(), 1);
    }

    #[test]
    fn test_state_flag_basic() {
        let flag = new_state_flag(4).unwrap(); // States 0, 1, 2, 3
        assert_eq!(flag.get(), 0);
        assert!(flag.is_min_state());
        assert!(!flag.is_max_state());
        assert!(flag.is_state(0));
        
        assert!(flag.advance());
        assert_eq!(flag.get(), 1);
        assert!(!flag.is_min_state());
        assert!(!flag.is_max_state());
        
        flag.set(3).unwrap();
        assert_eq!(flag.get(), 3);
        assert!(flag.is_max_state());
        assert!(flag.is_state(3));
        
        // Can't advance past max
        assert!(!flag.advance());
        assert_eq!(flag.get(), 3);
    }

    #[test]
    fn test_state_flag_retreat() {
        let flag = new_state_flag(3).unwrap(); // States 0, 1, 2
        flag.set(2).unwrap();
        
        assert!(flag.retreat());
        assert_eq!(flag.get(), 1);
        
        assert!(flag.retreat());
        assert_eq!(flag.get(), 0);
        
        // Can't retreat past min
        assert!(!flag.retreat());
        assert_eq!(flag.get(), 0);
    }

    #[test]
    fn test_state_flag_invalid() {
        // Can't create with 0 states
        assert!(new_state_flag(0).is_err());
        
        let flag = new_state_flag(3).unwrap();
        
        // Can't set to invalid state
        assert!(flag.set(3).is_err());
        assert!(flag.compare_and_swap(0, 3).is_err());
    }

    #[test]
    fn test_state_flag_concurrent() {
        let flag = Arc::new(new_state_flag(10).unwrap());
        let mut handles = vec![];
        
        // Multiple threads advancing the state
        for _ in 0..5 {
            let flag_clone = Arc::clone(&flag);
            let handle = thread::spawn(move || {
                for _ in 0..3 {
                    flag_clone.advance();
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Should be at least some advancement
        assert!(flag.get() > 0);
        assert!(flag.get() <= 9); // Shouldn't exceed max
    }

    #[test]
    fn test_flag_memory_ordering() {
        let flag = Flag::new(false);
        
        flag.set_ordered(MemoryOrder::Release);
        assert!(flag.load_ordered(MemoryOrder::Acquire));
        
        flag.clear_ordered(MemoryOrder::Relaxed);
        assert!(!flag.load());
    }

    #[test]
    fn test_flag_compare_and_swap() {
        let flag = Flag::new(false);
        
        // Successful CAS
        assert!(flag.compare_and_swap(false, true));
        assert!(flag.load());
        
        // Failed CAS
        assert!(!flag.compare_and_swap(false, false));
        assert!(flag.load()); // Should still be true
        
        // Successful CAS back
        assert!(flag.compare_and_swap(true, false));
        assert!(!flag.load());
    }

    #[test]
    fn test_flag_reset_count() {
        let flag = Flag::new(false);
        
        flag.set();
        flag.clear();
        flag.set();
        
        assert_eq!(flag.set_count(), 2);
        
        flag.reset_count();
        assert_eq!(flag.set_count(), 0);
        assert!(!flag.has_been_set()); // Reset makes it appear never set
    }
}
