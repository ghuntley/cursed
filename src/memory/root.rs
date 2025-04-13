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
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] RootScope[{:p}]::new Creating new root scope for GC[{:p}] (thread: {:?})", 
                 now, &gc, Arc::as_ptr(&gc), std::thread::current().id());
        Self {
            roots: HashSet::new(),
            gc,
        }
    }

    /// Register an object as a root in this scope
    pub fn add_root(&mut self, addr: usize) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] RootScope[{:p}]::add_root Adding root 0x{:x} to scope (thread: {:?})", 
                 now, self, addr, std::thread::current().id());
                 
        let inserted = self.roots.insert(addr);
        if inserted {
            // Only add to GC if it's new to this scope
            println!("[{}ms] RootScope[{:p}]::add_root Successfully added to scope, forwarding to GC[{:p}]", 
                    now, self, Arc::as_ptr(&self.gc));
            // Use a timeout when adding to the GC to avoid deadlocks
            let lock_context = format!("RootScope::add_root(0x{:x})", addr);
            let timeout = std::time::Duration::from_secs(1); // Use a shorter timeout to fail fast
            match crate::memory::deadlock_detector::try_write_with_timeout(&self.gc.inner, timeout, &lock_context) {
                Some(mut state) => {
                    let _ = state.roots.insert(addr);
                    println!("[{}ms] RootScope[{:p}]::add_root Successfully added to GC roots", now, self);
                },
                None => {
                    println!("[{}ms] RootScope[{:p}]::add_root WARNING: Failed to add to GC roots due to lock timeout", now, self);
                    // Don't panic here, just log the issue
                }
            }
        } else {
            println!("[{}ms] RootScope[{:p}]::add_root Root already in this scope, not adding to GC", 
                    now, self);
        }
        
        println!("[{}ms] RootScope[{:p}]::add_root completed", now, self);
    }

    /// Unregister an object from this scope's roots
    pub fn remove_root(&mut self, addr: usize) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] RootScope[{:p}]::remove_root Removing root 0x{:x} from scope (thread: {:?})", 
                 now, self, addr, std::thread::current().id());
                 
        let removed = self.roots.remove(&addr);
        if removed {
            // Only remove from GC if it was in this scope
            println!("[{}ms] RootScope[{:p}]::remove_root Successfully removed from scope, forwarding to GC[{:p}]", 
                    now, self, Arc::as_ptr(&self.gc));
            // Use a timeout when removing from the GC to avoid deadlocks
            let lock_context = format!("RootScope::remove_root(0x{:x})", addr);
            let timeout = std::time::Duration::from_secs(1); // Use a shorter timeout to fail fast
            match crate::memory::deadlock_detector::try_write_with_timeout(&self.gc.inner, timeout, &lock_context) {
                Some(mut state) => {
                    let _ = state.roots.remove(&addr);
                    println!("[{}ms] RootScope[{:p}]::remove_root Successfully removed from GC roots", now, self);
                },
                None => {
                    println!("[{}ms] RootScope[{:p}]::remove_root WARNING: Failed to remove from GC roots due to lock timeout", now, self);
                    // Don't panic here, just log the issue
                }
            }
        } else {
            println!("[{}ms] RootScope[{:p}]::remove_root Root not in this scope, not removing from GC", 
                    now, self);
        }
        
        println!("[{}ms] RootScope[{:p}]::remove_root completed", now, self);
    }
}

impl Drop for RootScope {
    fn drop(&mut self) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] RootScope[{:p}]::drop Dropping root scope with {} roots (thread: {:?})", 
                 now, self, self.roots.len(), std::thread::current().id());
        
        // Make a copy of roots to avoid potential borrowing issues during iteration
        let roots = self.roots.iter().cloned().collect::<Vec<_>>();
        println!("[{}ms] RootScope[{:p}]::drop Created copy of roots for cleanup: {:?}", 
                 now, self, roots);
                 
        // Try to acquire lock on GC once to remove all roots
        let lock_context = format!("RootScope::drop ({} roots)", roots.len());
        let timeout = std::time::Duration::from_secs(1);
        match crate::memory::deadlock_detector::try_write_with_timeout(&self.gc.inner, timeout, &lock_context) {
            Some(mut state) => {
                // Ensure all roots are unregistered when scope ends
                for addr in &roots {
                    println!("[{}ms] RootScope[{:p}]::drop Auto-removing root 0x{:x}", now, self, addr);
                    state.roots.remove(addr);
                }
                println!("[{}ms] RootScope[{:p}]::drop Successfully removed all roots from GC", now, self);
            },
            None => {
                println!("[{}ms] RootScope[{:p}]::drop WARNING: Failed to remove roots from GC due to lock timeout", now, self);
                // Continue with cleanup despite lock failure
            }
        }
        
        // Clear remaining roots (if any weren't removed for some reason)
        self.roots.clear();
        println!("[{}ms] RootScope[{:p}]::drop Completed cleanup", now, self);
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
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] ThreadLocalRootManager::new Creating new local manager (thread: {:?})", 
                 now, std::thread::current().id());
        Self {
            current_scope: RefCell::new(None),
        }
    }

    /// Set the current root scope
    pub fn set_current_scope(&self, scope: RootScope) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] ThreadLocalRootManager[{:p}]::set_current_scope Setting scope[{:p}] (thread: {:?})", 
                 now, self, &scope, std::thread::current().id());
                 
        let lock_start = std::time::Instant::now();
        let mut current = self.current_scope.borrow_mut();
        let elapsed = lock_start.elapsed().as_millis();
        
        println!("[{}ms] ThreadLocalRootManager[{:p}]::set_current_scope Acquired RefCell lock after {}ms", 
                now, self, elapsed);
                
        // Check if there's already a scope that needs cleanup
        if current.is_some() {
            println!("[{}ms] ThreadLocalRootManager[{:p}]::set_current_scope Warning: Replacing existing scope", 
                    now, self);
        }
        
        *current = Some(scope);
        println!("[{}ms] ThreadLocalRootManager[{:p}]::set_current_scope Scope set successfully", 
                now, self);
    }

    /// Clear the current root scope
    pub fn clear_current_scope(&self) -> Option<RootScope> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] ThreadLocalRootManager[{:p}]::clear_current_scope Clearing current scope (thread: {:?})", 
                 now, self, std::thread::current().id());
                 
        let lock_start = std::time::Instant::now();
        let mut current = self.current_scope.borrow_mut();
        let elapsed = lock_start.elapsed().as_millis();
        
        println!("[{}ms] ThreadLocalRootManager[{:p}]::clear_current_scope Acquired RefCell lock after {}ms", 
                now, self, elapsed);
                
        let result = current.take();
        println!("[{}ms] ThreadLocalRootManager[{:p}]::clear_current_scope Current scope {}", 
                now, self, if result.is_some() { "was cleared" } else { "was already None" });
        result
    }

    /// Add a root to the current scope
    pub fn add_root(&self, addr: usize) -> bool {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] ThreadLocalRootManager[{:p}]::add_root Adding root 0x{:x} (thread: {:?})", 
                 now, self, addr, std::thread::current().id());
                 
        let lock_start = std::time::Instant::now();
        let mut current = self.current_scope.borrow_mut();
        let elapsed = lock_start.elapsed().as_millis();
        
        println!("[{}ms] ThreadLocalRootManager[{:p}]::add_root Acquired RefCell lock after {}ms", 
                now, self, elapsed);
        
        if let Some(ref mut scope) = *current {
            println!("[{}ms] ThreadLocalRootManager[{:p}]::add_root Forwarding to scope[{:p}]", 
                    now, self, &*scope);
            scope.add_root(addr);
            println!("[{}ms] ThreadLocalRootManager[{:p}]::add_root Completed successfully", 
                    now, self);
            true
        } else {
            println!("[{}ms] ThreadLocalRootManager[{:p}]::add_root WARNING - no active scope when adding root 0x{:x}", 
                    now, self, addr);
            println!("[{}ms] ThreadLocalRootManager[{:p}]::add_root Stack trace: {:?}", 
                    now, self, std::backtrace::Backtrace::capture());
            false
        }
    }

    /// Remove a root from the current scope
    pub fn remove_root(&self, addr: usize) -> bool {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] ThreadLocalRootManager[{:p}]::remove_root Removing root 0x{:x} (thread: {:?})", 
                 now, self, addr, std::thread::current().id());
                 
        let lock_start = std::time::Instant::now();
        let mut current = self.current_scope.borrow_mut();
        let elapsed = lock_start.elapsed().as_millis();
        
        println!("[{}ms] ThreadLocalRootManager[{:p}]::remove_root Acquired RefCell lock after {}ms", 
                now, self, elapsed);
                
        if let Some(ref mut scope) = *current {
            println!("[{}ms] ThreadLocalRootManager[{:p}]::remove_root Forwarding to scope[{:p}]", 
                    now, self, &*scope);
            scope.remove_root(addr);
            println!("[{}ms] ThreadLocalRootManager[{:p}]::remove_root Completed successfully", 
                    now, self);
            true
        } else {
            println!("[{}ms] ThreadLocalRootManager[{:p}]::remove_root WARNING - no active scope when removing root 0x{:x}", 
                    now, self, addr);
            println!("[{}ms] ThreadLocalRootManager[{:p}]::remove_root Stack trace: {:?}", 
                    now, self, std::backtrace::Backtrace::capture());
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
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] RootScopeGuard::new Creating new guard (thread: {:?})", 
                 now, std::thread::current().id());
                 
        let scope = RootScope::new(gc);
        
        // Use timeout when acquiring the lock
        let lock_context = format!("RootScopeGuard::new");
        let timeout = std::time::Duration::from_secs(1);
        
        match crate::memory::deadlock_detector::try_mutex_with_timeout(&ROOT_MANAGER, timeout, &lock_context) {
            Some(mut manager) => {
                println!("[{}ms] RootScopeGuard::new Successfully acquired ROOT_MANAGER lock", now);
                manager.push_scope(scope);
                println!("[{}ms] RootScopeGuard::new Successfully pushed scope", now);
            },
            None => {
                println!("[{}ms] RootScopeGuard::new WARNING: Failed to acquire ROOT_MANAGER lock due to timeout", now);
                println!("[{}ms] RootScopeGuard::new WARNING: Scope will not be properly registered, memory leaks may occur", now);
                // Continue despite lock failure, but the scope won't be registered
            }
        }
        
        Self {
            _non_send: PhantomData,
        }
    }
}

impl Drop for RootScopeGuard {
    fn drop(&mut self) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] RootScopeGuard::drop Dropping guard (thread: {:?})", 
                 now, std::thread::current().id());
        
        // Use timeout when acquiring the lock
        let lock_context = format!("RootScopeGuard::drop");
        let timeout = std::time::Duration::from_secs(1);
        
        match crate::memory::deadlock_detector::try_mutex_with_timeout(&ROOT_MANAGER, timeout, &lock_context) {
            Some(mut manager) => {
                if let Some(scope) = manager.pop_scope() {
                    // The scope will clean up its roots when dropped
                    println!("[{}ms] RootScopeGuard::drop Successfully popped and will clean up scope", now);
                } else {
                    println!("[{}ms] RootScopeGuard::drop No scope found to pop", now);
                }
            },
            None => {
                println!("[{}ms] RootScopeGuard::drop WARNING: Failed to acquire ROOT_MANAGER lock due to timeout", now);
                // Continue with drop despite lock failure
            }
        }
        
        println!("[{}ms] RootScopeGuard::drop Completed", now);
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
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] GlobalRootManager::new Creating new global manager", now);
        Self {
            thread_local: thread_local::ThreadLocal::new(),
        }
    }

    /// Push a new root scope
    pub fn push_scope(&self, scope: RootScope) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] GlobalRootManager[{:p}]::push_scope Pushing scope[{:p}] (thread: {:?})", 
                 now, self, &scope, std::thread::current().id());
        
        let local_manager = self.get_or_create_local();
        println!("[{}ms] GlobalRootManager[{:p}]::push_scope Got local manager[{:p}]", 
                 now, self, local_manager);
                 
        local_manager.set_current_scope(scope);
        println!("[{}ms] GlobalRootManager[{:p}]::push_scope Scope pushed successfully", now, self);
    }

    /// Pop the current root scope
    pub fn pop_scope(&self) -> Option<RootScope> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] GlobalRootManager[{:p}]::pop_scope Popping current scope (thread: {:?})", 
                 now, self, std::thread::current().id());
        
        let result = self.get_local().and_then(|local| {
            println!("[{}ms] GlobalRootManager[{:p}]::pop_scope Using local manager[{:p}]", 
                     now, self, local);
            let scope = local.clear_current_scope();
            println!("[{}ms] GlobalRootManager[{:p}]::pop_scope Got scope: {}", 
                     now, self, if scope.is_some() { "Some" } else { "None" });
            scope
        });
        
        if result.is_none() {
            println!("[{}ms] GlobalRootManager[{:p}]::pop_scope No scope to pop (thread: {:?})", 
                     now, self, std::thread::current().id());
        }
        
        result
    }

    /// Add a root to the current scope
    pub fn add_root(&self, addr: usize) -> bool {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] GlobalRootManager[{:p}]::add_root Adding root 0x{:x} (thread: {:?})", 
                 now, self, addr, std::thread::current().id());
        
        let result = self.get_local()
            .map(|local| {
                println!("[{}ms] GlobalRootManager[{:p}]::add_root Using local manager[{:p}]", 
                         now, self, local);
                local.add_root(addr)
            })
            .unwrap_or_else(|| {
                println!("[{}ms] GlobalRootManager[{:p}]::add_root No active scope for root 0x{:x}", 
                         now, self, addr);
                println!("[{}ms] GlobalRootManager[{:p}]::add_root Stack trace: {:?}", 
                         now, self, std::backtrace::Backtrace::capture());
                false
            });
            
        println!("[{}ms] GlobalRootManager[{:p}]::add_root Completed with result: {}", 
                 now, self, result);
        result
    }

    /// Remove a root from the current scope
    pub fn remove_root(&self, addr: usize) -> bool {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] GlobalRootManager[{:p}]::remove_root Removing root 0x{:x} (thread: {:?})", 
                 now, self, addr, std::thread::current().id());
        
        let result = self.get_local()
            .map(|local| {
                println!("[{}ms] GlobalRootManager[{:p}]::remove_root Using local manager[{:p}]", 
                         now, self, local);
                local.remove_root(addr)
            })
            .unwrap_or_else(|| {
                println!("[{}ms] GlobalRootManager[{:p}]::remove_root No active scope for root 0x{:x}", 
                         now, self, addr);
                println!("[{}ms] GlobalRootManager[{:p}]::remove_root Stack trace: {:?}", 
                         now, self, std::backtrace::Backtrace::capture());
                false
            });
            
        println!("[{}ms] GlobalRootManager[{:p}]::remove_root Completed with result: {}", 
                 now, self, result);
        result
    }

    /// Get the thread-local manager
    fn get_local(&self) -> Option<&ThreadLocalRootManager> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let result = self.thread_local.get();
        println!("[{}ms] GlobalRootManager[{:p}]::get_local Result: {} (thread: {:?})", 
                 now, self, 
                 if result.is_some() { "Some" } else { "None" },
                 std::thread::current().id());
        result
    }

    /// Get or create the thread-local manager
    fn get_or_create_local(&self) -> &ThreadLocalRootManager {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("[{}ms] GlobalRootManager[{:p}]::get_or_create_local Called (thread: {:?})", 
                 now, self, std::thread::current().id());
        
        let result = self.thread_local.get_or(|| {
            println!("[{}ms] GlobalRootManager[{:p}]::get_or_create_local Creating new thread-local manager", 
                    now, self);
            ThreadLocalRootManager::new()
        });
        
        println!("[{}ms] GlobalRootManager[{:p}]::get_or_create_local Returning manager[{:p}]", 
                now, self, result);
        result
    }
}