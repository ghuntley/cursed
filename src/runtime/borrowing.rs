//! Borrowing system for CURSED runtime
//!
//! This module implements a borrowing checker and reference management system
//! that integrates with the garbage collector to ensure memory safety and
//! prevent data races in mutable reference operations.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::runtime_value::RuntimeValue;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::time::Instant;
use std::fmt;

/// Reference borrowing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorrowMode {
    /// Shared/immutable borrow
    Shared,
    /// Exclusive/mutable borrow
    Mutable,
}

/// Borrow state for a value
#[derive(Debug, Clone)]
pub struct BorrowState {
    /// Current borrow mode
    pub mode: Option<BorrowMode>,
    /// Number of shared borrows
    pub shared_count: usize,
    /// Whether there's an active mutable borrow
    pub has_mutable: bool,
    /// Timestamps for borrow tracking
    pub borrow_times: Vec<Instant>,
    /// Source locations for debugging
    pub borrow_locations: Vec<String>,
}

impl Default for BorrowState {
    fn default() -> Self {
        Self {
            mode: None,
            shared_count: 0,
            has_mutable: false,
            borrow_times: Vec::new(),
            borrow_locations: Vec::new(),
        }
    }
}

/// Unique identifier for runtime values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u64);

/// Global borrow checker state
pub struct BorrowChecker {
    /// Value borrow states
    borrows: RwLock<HashMap<ValueId, BorrowState>>,
    /// Value ID counter
    next_id: Mutex<u64>,
    /// Weak references to tracked values
    tracked_values: RwLock<HashMap<ValueId, Weak<Mutex<RuntimeValue>>>>,
    /// GC integration callbacks
    gc_callbacks: RwLock<Vec<Box<dyn Fn(&BorrowState) + Send + Sync>>>,
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowChecker {
    /// Create a new borrow checker
    pub fn new() -> Self {
        Self {
            borrows: RwLock::new(HashMap::new()),
            next_id: Mutex::new(1),
            tracked_values: RwLock::new(HashMap::new()),
            gc_callbacks: RwLock::new(Vec::new()),
        }
    }

    /// Generate a new unique value ID
    pub fn new_value_id(&self) -> ValueId {
        let mut counter = self.next_id.lock().unwrap();
        let id = ValueId(*counter);
        *counter += 1;
        id
    }

    /// Register a value for borrow tracking
    pub fn register_value(&self, value: Arc<Mutex<RuntimeValue>>) -> ValueId {
        let id = self.new_value_id();
        let weak_ref = Arc::downgrade(&value);
        
        if let Ok(mut tracked) = self.tracked_values.write() {
            tracked.insert(id, weak_ref);
        }
        
        if let Ok(mut borrows) = self.borrows.write() {
            borrows.insert(id, BorrowState::default());
        }
        
        id
    }

    /// Attempt to acquire a shared borrow
    pub fn try_borrow_shared(&self, id: ValueId, location: String) -> CursedResult<SharedBorrow> {
        let mut borrows = self.borrows.write()
            .map_err(|_| Error::Runtime("Failed to acquire borrow lock".to_string()))?;
        
        let state = borrows.entry(id).or_insert_with(BorrowState::default);
        
        // Check if mutable borrow is active
        if state.has_mutable {
            return Err(Error::Runtime(format!(
                "Cannot create shared borrow: value {} has active mutable borrow at {}",
                id.0,
                state.borrow_locations.last().unwrap_or(&"unknown".to_string())
            )));
        }
        
        // Allow shared borrows
        state.shared_count += 1;
        state.mode = Some(BorrowMode::Shared);
        state.borrow_times.push(Instant::now());
        state.borrow_locations.push(location);
        
        Ok(SharedBorrow {
            id,
            checker: self,
        })
    }

    /// Attempt to acquire a mutable borrow
    pub fn try_borrow_mutable(&self, id: ValueId, location: String) -> CursedResult<MutableBorrow> {
        let mut borrows = self.borrows.write()
            .map_err(|_| Error::Runtime("Failed to acquire borrow lock".to_string()))?;
        
        let state = borrows.entry(id).or_insert_with(BorrowState::default);
        
        // Check if any borrow is active
        if state.shared_count > 0 {
            return Err(Error::Runtime(format!(
                "Cannot create mutable borrow: value {} has {} active shared borrows",
                id.0, state.shared_count
            )));
        }
        
        if state.has_mutable {
            return Err(Error::Runtime(format!(
                "Cannot create mutable borrow: value {} already has active mutable borrow at {}",
                id.0,
                state.borrow_locations.last().unwrap_or(&"unknown".to_string())
            )));
        }
        
        // Allow mutable borrow
        state.has_mutable = true;
        state.mode = Some(BorrowMode::Mutable);
        state.borrow_times.push(Instant::now());
        state.borrow_locations.push(location);
        
        Ok(MutableBorrow {
            id,
            checker: self,
        })
    }

    /// Release a shared borrow
    fn release_shared(&self, id: ValueId) {
        if let Ok(mut borrows) = self.borrows.write() {
            if let Some(state) = borrows.get_mut(&id) {
                if state.shared_count > 0 {
                    state.shared_count -= 1;
                    if state.shared_count == 0 {
                        state.mode = None;
                    }
                }
            }
        }
    }

    /// Release a mutable borrow
    fn release_mutable(&self, id: ValueId) {
        if let Ok(mut borrows) = self.borrows.write() {
            if let Some(state) = borrows.get_mut(&id) {
                state.has_mutable = false;
                state.mode = None;
            }
        }
    }

    /// Get the current borrow state for a value
    pub fn get_borrow_state(&self, id: ValueId) -> Option<BorrowState> {
        self.borrows.read().ok()?.get(&id).cloned()
    }

    /// Check if a value can be safely accessed
    pub fn can_access(&self, id: ValueId, mode: BorrowMode) -> bool {
        if let Some(state) = self.get_borrow_state(id) {
            match mode {
                BorrowMode::Shared => !state.has_mutable,
                BorrowMode::Mutable => state.shared_count == 0 && !state.has_mutable,
            }
        } else {
            true // No tracking = can access
        }
    }

    /// Clean up expired weak references
    pub fn cleanup_expired_references(&self) {
        if let Ok(mut tracked) = self.tracked_values.write() {
            if let Ok(mut borrows) = self.borrows.write() {
                let mut to_remove = Vec::new();
                
                for (id, weak_ref) in tracked.iter() {
                    if weak_ref.strong_count() == 0 {
                        to_remove.push(*id);
                    }
                }
                
                for id in to_remove {
                    tracked.remove(&id);
                    borrows.remove(&id);
                }
            }
        }
    }

    /// Add GC integration callback
    pub fn add_gc_callback<F>(&self, callback: F) 
    where 
        F: Fn(&BorrowState) + Send + Sync + 'static,
    {
        if let Ok(mut callbacks) = self.gc_callbacks.write() {
            callbacks.push(Box::new(callback));
        }
    }

    /// Notify GC of borrow state changes
    fn notify_gc(&self, state: &BorrowState) {
        if let Ok(callbacks) = self.gc_callbacks.read() {
            for callback in callbacks.iter() {
                callback(state);
            }
        }
    }
}

/// RAII guard for shared borrows
pub struct SharedBorrow<'a> {
    id: ValueId,
    checker: &'a BorrowChecker,
}

impl<'a> Drop for SharedBorrow<'a> {
    fn drop(&mut self) {
        self.checker.release_shared(self.id);
    }
}

/// RAII guard for mutable borrows
pub struct MutableBorrow<'a> {
    id: ValueId,
    checker: &'a BorrowChecker,
}

impl<'a> Drop for MutableBorrow<'a> {
    fn drop(&mut self) {
        self.checker.release_mutable(self.id);
    }
}

/// Global borrow checker instance
static GLOBAL_BORROW_CHECKER: std::sync::OnceLock<BorrowChecker> = std::sync::OnceLock::new();

/// Get the global borrow checker
pub fn get_global_borrow_checker() -> &'static BorrowChecker {
    GLOBAL_BORROW_CHECKER.get_or_init(BorrowChecker::new)
}

/// Mutable reference wrapper that integrates with borrow checker
pub struct MutableRef<T> {
    value: Arc<Mutex<T>>,
    borrow_id: ValueId,
    _guard: Option<MutableBorrow<'static>>,
}

impl<T> MutableRef<T> {
    /// Create a new mutable reference
    pub fn new(value: T, location: String) -> CursedResult<Self> {
        let wrapped = Arc::new(Mutex::new(value));
        let checker = get_global_borrow_checker();
        let borrow_id = checker.new_value_id();
        
        let guard = checker.try_borrow_mutable(borrow_id, location)?;
        
        Ok(Self {
            value: wrapped,
            borrow_id,
            _guard: Some(unsafe { std::mem::transmute(guard) }),
        })
    }

    /// Get exclusive access to the value
    pub fn get_mut(&mut self) -> std::sync::MutexGuard<T> {
        self.value.lock().unwrap()
    }

    /// Get the value ID for borrow tracking
    pub fn value_id(&self) -> ValueId {
        self.borrow_id
    }
}

impl<T> Clone for MutableRef<T> 
where 
    T: Clone,
{
    fn clone(&self) -> Self {
        let checker = get_global_borrow_checker();
        let new_id = checker.new_value_id();
        
        // Clone the underlying value
        let cloned_value = {
            let guard = self.value.lock().unwrap();
            guard.clone()
        };
        
        Self {
            value: Arc::new(Mutex::new(cloned_value)),
            borrow_id: new_id,
            _guard: None,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for MutableRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutableRef")
            .field("borrow_id", &self.borrow_id)
            .field("value", &"<locked>")
            .finish()
    }
}

/// Shared reference wrapper that integrates with borrow checker
pub struct SharedRef<T> {
    value: Arc<T>,
    borrow_id: ValueId,
    _guard: Option<SharedBorrow<'static>>,
}

impl<T> SharedRef<T> {
    /// Create a new shared reference
    pub fn new(value: T, location: String) -> CursedResult<Self> {
        let wrapped = Arc::new(value);
        let checker = get_global_borrow_checker();
        let borrow_id = checker.new_value_id();
        
        let guard = checker.try_borrow_shared(borrow_id, location)?;
        
        Ok(Self {
            value: wrapped,
            borrow_id,
            _guard: Some(unsafe { std::mem::transmute(guard) }),
        })
    }

    /// Get shared access to the value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Get the value ID for borrow tracking
    pub fn value_id(&self) -> ValueId {
        self.borrow_id
    }
}

impl<T> Clone for SharedRef<T> {
    fn clone(&self) -> Self {
        let checker = get_global_borrow_checker();
        let new_id = checker.new_value_id();
        
        Self {
            value: Arc::clone(&self.value),
            borrow_id: new_id,
            _guard: None,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SharedRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedRef")
            .field("borrow_id", &self.borrow_id)
            .field("value", &self.value)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_borrow() {
        let checker = BorrowChecker::new();
        let id = checker.new_value_id();
        
        // Multiple shared borrows should succeed
        let _borrow1 = checker.try_borrow_shared(id, "test1".to_string()).unwrap();
        let _borrow2 = checker.try_borrow_shared(id, "test2".to_string()).unwrap();
        
        // Mutable borrow should fail
        assert!(checker.try_borrow_mutable(id, "test3".to_string()).is_err());
    }

    #[test]
    fn test_mutable_borrow() {
        let checker = BorrowChecker::new();
        let id = checker.new_value_id();
        
        // Mutable borrow should succeed
        let _borrow1 = checker.try_borrow_mutable(id, "test1".to_string()).unwrap();
        
        // Additional borrows should fail
        assert!(checker.try_borrow_shared(id, "test2".to_string()).is_err());
        assert!(checker.try_borrow_mutable(id, "test3".to_string()).is_err());
    }

    #[test]
    fn test_borrow_release() {
        let checker = BorrowChecker::new();
        let id = checker.new_value_id();
        
        {
            let _borrow = checker.try_borrow_mutable(id, "test".to_string()).unwrap();
            assert!(!checker.can_access(id, BorrowMode::Shared));
        }
        
        // After drop, should be accessible again
        assert!(checker.can_access(id, BorrowMode::Shared));
        assert!(checker.can_access(id, BorrowMode::Mutable));
    }

    #[test]
    fn test_mutable_ref() {
        let mut_ref = MutableRef::new(42i32, "test".to_string()).unwrap();
        let id = mut_ref.value_id();
        let checker = get_global_borrow_checker();
        
        // Should not be able to create another mutable borrow
        assert!(!checker.can_access(id, BorrowMode::Mutable));
    }
}
