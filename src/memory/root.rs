//! Root management system for the garbage collector
//!
//! This module provides automatic tracking of GC roots with scope-based
//! lifetime management. It ensures that objects are properly registered
//! as roots when created and unregistered when they go out of scope,
//! even in the presence of panics or early returns.

use std::cell::RefCell;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak as StdWeak};

use crate::memory::gc::GarbageCollector;

/// Tracks the current GC roots in a given scope
///
/// `RootScope` automatically tracks GC roots within its scope and
/// ensures they are properly unregistered when the scope ends,
/// even in the case of panics or early returns.
#[derive(Debug)]
pub struct RootScope {
    // Current roots in this scope
    roots: HashSet<usize>,
    // GC that manages these roots
    gc: Arc<GarbageCollector>,
}

impl RootScope {
    /// Create a new root scope associated with the given GC
    pub fn new(gc: Arc<GarbageCollector>) -> Self {
        println!("RootScope: Creating new root scope");
        Self {
            roots: HashSet::new(),
            gc,
        }
    }

    /// Register an object as a root in this scope
    pub fn add_root(&mut self, addr: usize) {
        println!("RootScope: Adding root 0x{:x} to scope", addr);
        if self.roots.insert(addr) {
            // Only add to GC if it's new to this scope
            self.gc.add_root(addr);
        }
    }

    /// Unregister an object from this scope's roots
    pub fn remove_root(&mut self, addr: usize) {
        println!("RootScope: Removing root 0x{:x} from scope", addr);
        if self.roots.remove(&addr) {
            // Only remove from GC if it was in this scope
            self.gc.remove_root(addr);
        }
    }
}

impl Drop for RootScope {
    fn drop(&mut self) {
        println!("RootScope: Dropping root scope with {} roots", self.roots.len());
        // Ensure all roots are unregistered when scope ends
        for addr in self.roots.iter() {
            println!("RootScope: Auto-removing root 0x{:x}", addr);
            self.gc.remove_root(*addr);
        }
    }
}

/// Thread-local storage for current root scope
pub struct ThreadLocalRootManager {
    // Current active root scope
    current_scope: RefCell<Option<RootScope>>,
}

impl ThreadLocalRootManager {
    /// Create a new thread-local root manager
    pub fn new() -> Self {
        Self {
            current_scope: RefCell::new(None),
        }
    }

    /// Set the current root scope
    pub fn set_current_scope(&self, scope: RootScope) {
        let mut current = self.current_scope.borrow_mut();
        *current = Some(scope);
    }

    /// Clear the current root scope
    pub fn clear_current_scope(&self) -> Option<RootScope> {
        let mut current = self.current_scope.borrow_mut();
        current.take()
    }

    /// Add a root to the current scope
    pub fn add_root(&self, addr: usize) -> bool {
        let mut current = self.current_scope.borrow_mut();
        if let Some(ref mut scope) = *current {
            scope.add_root(addr);
            true
        } else {
            println!("RootManager: Warning - no active scope when adding root 0x{:x}", addr);
            false
        }
    }

    /// Remove a root from the current scope
    pub fn remove_root(&self, addr: usize) -> bool {
        let mut current = self.current_scope.borrow_mut();
        if let Some(ref mut scope) = *current {
            scope.remove_root(addr);
            true
        } else {
            println!("RootManager: Warning - no active scope when removing root 0x{:x}", addr);
            false
        }
    }
}

/// Global root manager that maintains thread-local scopes
pub static ROOT_MANAGER: once_cell::sync::Lazy<Mutex<GlobalRootManager>> = 
    once_cell::sync::Lazy::new(|| Mutex::new(GlobalRootManager::new()));

/// RAII guard for a root scope
///
/// When this guard is dropped, the root scope is automatically popped
/// and all roots in that scope are unregistered.
pub struct RootScopeGuard {
    // Preventing moving/cloning this guard
    _non_send: PhantomData<*mut ()>,
}

impl RootScopeGuard {
    /// Create a new root scope guard
    pub fn new(gc: Arc<GarbageCollector>) -> Self {
        let scope = RootScope::new(gc);
        ROOT_MANAGER.lock().unwrap().push_scope(scope);
        Self {
            _non_send: PhantomData,
        }
    }
}

impl Drop for RootScopeGuard {
    fn drop(&mut self) {
        if let Some(scope) = ROOT_MANAGER.lock().unwrap().pop_scope() {
            // The scope will clean up its roots when dropped
            println!("RootScopeGuard: Dropping guard and cleaning up scope");
        }
    }
}

/// Global manager for root scopes
pub struct GlobalRootManager {
    // Thread-local storage for current root scope
    thread_local: thread_local::ThreadLocal<ThreadLocalRootManager>,
}

impl GlobalRootManager {
    /// Create a new global root manager
    pub fn new() -> Self {
        Self {
            thread_local: thread_local::ThreadLocal::new(),
        }
    }

    /// Push a new root scope
    pub fn push_scope(&self, scope: RootScope) {
        self.get_or_create_local().set_current_scope(scope);
    }

    /// Pop the current root scope
    pub fn pop_scope(&self) -> Option<RootScope> {
        self.get_local().and_then(|local| local.clear_current_scope())
    }

    /// Add a root to the current scope
    pub fn add_root(&self, addr: usize) -> bool {
        self.get_local()
            .map(|local| local.add_root(addr))
            .unwrap_or(false)
    }

    /// Remove a root from the current scope
    pub fn remove_root(&self, addr: usize) -> bool {
        self.get_local()
            .map(|local| local.remove_root(addr))
            .unwrap_or(false)
    }

    /// Get the thread-local manager
    fn get_local(&self) -> Option<&ThreadLocalRootManager> {
        self.thread_local.get()
    }

    /// Get or create the thread-local manager
    fn get_or_create_local(&self) -> &ThreadLocalRootManager {
        self.thread_local.get_or(|| ThreadLocalRootManager::new())
    }
}