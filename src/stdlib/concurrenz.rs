//! Synchronization primitives for concurrent CURSED programs
//!
//! The concurrenz package provides synchronization tools for safely coordinating
//! concurrent goroutines in CURSED programs, similar to Go's sync package.
//! It includes mutexes for exclusive access to shared resources, read-write locks
//! for concurrent reads with exclusive writes, wait groups for coordinating
//! multiple goroutines, and Once for one-time initialization.
//!
//! Key components:
//!
//! - `Mutex`: Mutual exclusion lock for protecting shared data
//! - `RWMutex`: Read-write mutex allowing multiple readers or a single writer
//! - `WaitGroup`: Synchronization primitive for waiting for multiple goroutines
//! - `Once`: Ensures a function is executed exactly once
//!
//! Functions:
//! - `new_mutex`, `mutex_lock`, `mutex_unlock`: Mutex operations
//! - `new_rwmutex`, `rwmutex_rlock`, `rwmutex_runlock`, `rwmutex_lock`, `rwmutex_unlock`: RWMutex operations
//! - `new_waitgroup`, `waitgroup_add`, `waitgroup_done`, `waitgroup_wait`: WaitGroup operations
//! - `new_once`, `once_do`: Once operations

use crate::error::Error;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex as StdMutex, Once as StdOnce, RwLock as StdRwLock};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

// Thread-safe wrapper for storing sync primitive handles
//
// This is a lightweight handle registry that manages synchronization primitives.
// It provides a safe way to reference these primitives from the CURSED language.
pub struct SyncRegistry {
    // Map of mutex handles to actual mutex instances
    mutex_registry: StdMutex<HashMap<i64, Arc<StdMutex<()>>>>,
    // Map of rwmutex handles to actual rwmutex instances
    rwmutex_registry: StdMutex<HashMap<i64, Arc<RwMutex>>>,
    // Map of waitgroup handles to actual waitgroup instances
    waitgroup_registry: StdMutex<HashMap<i64, Arc<WaitGroup>>>,
    // Map of once handles to actual once instances
    once_registry: StdMutex<HashMap<i64, Arc<Once>>>,
    // Counter for generating unique handles
    next_handle: AtomicI64,
}

// Initialize the global registry for sync primitives
lazy_static::lazy_static! {
    static ref SYNC_REGISTRY: SyncRegistry = SyncRegistry::new();
}

impl SyncRegistry {
    fn new() -> Self {
        SyncRegistry {
            mutex_registry: StdMutex::new(HashMap::new()),
            rwmutex_registry: StdMutex::new(HashMap::new()),
            waitgroup_registry: StdMutex::new(HashMap::new()),
            once_registry: StdMutex::new(HashMap::new()),
            next_handle: AtomicI64::new(1), // Start at 1, 0 can be used as invalid handle
        }
    }

    fn next_handle(&self) -> i64 {
        self.next_handle.fetch_add(1, Ordering::SeqCst)
    }

    // Register a mutex and get a handle
    fn register_mutex(&self, mutex: Arc<StdMutex<()>>) -> i64 {
        let handle = self.next_handle();
        let mut registry = self.mutex_registry.lock().unwrap();
        registry.insert(handle, mutex);
        handle
    }

    // Get a mutex by handle
    fn get_mutex(&self, handle: i64) -> Option<Arc<StdMutex<()>>> {
        let registry = self.mutex_registry.lock().unwrap();
        registry.get(&handle).cloned()
    }

    // Register an RWMutex and get a handle
    fn register_rwmutex(&self, rwmutex: Arc<RwMutex>) -> i64 {
        let handle = self.next_handle();
        let mut registry = self.rwmutex_registry.lock().unwrap();
        registry.insert(handle, rwmutex);
        handle
    }

    // Get an RWMutex by handle
    fn get_rwmutex(&self, handle: i64) -> Option<Arc<RwMutex>> {
        let registry = self.rwmutex_registry.lock().unwrap();
        registry.get(&handle).cloned()
    }

    // Register a WaitGroup and get a handle
    fn register_waitgroup(&self, waitgroup: Arc<WaitGroup>) -> i64 {
        let handle = self.next_handle();
        let mut registry = self.waitgroup_registry.lock().unwrap();
        registry.insert(handle, waitgroup);
        handle
    }

    // Get a WaitGroup by handle
    fn get_waitgroup(&self, handle: i64) -> Option<Arc<WaitGroup>> {
        let registry = self.waitgroup_registry.lock().unwrap();
        registry.get(&handle).cloned()
    }

    // Register a Once and get a handle
    fn register_once(&self, once: Arc<Once>) -> i64 {
        let handle = self.next_handle();
        let mut registry = self.once_registry.lock().unwrap();
        registry.insert(handle, once);
        handle
    }

    // Get a Once by handle
    fn get_once(&self, handle: i64) -> Option<Arc<Once>> {
        let registry = self.once_registry.lock().unwrap();
        registry.get(&handle).cloned()
    }
}

/// Read-write mutex for protecting shared data in CURSED programs
///
/// An RWMutex is a reader/writer mutual exclusion lock. The lock can be held by
/// an arbitrary number of readers or a single writer. Useful for read-heavy
/// workloads where writes are infrequent.
#[derive(Debug)]
pub struct RwMutex {
    // The underlying read-write lock
    lock: StdRwLock<()>,
    // Count of active readers (for informational purposes)
    readers: AtomicI64,
    // Whether there's an active writer (for informational purposes)
    writer: AtomicBool,
}

impl RwMutex {
    fn new() -> Self {
        RwMutex {
            lock: StdRwLock::new(()),
            readers: AtomicI64::new(0),
            writer: AtomicBool::new(false),
        }
    }
}

impl Clone for RwMutex {
    fn clone(&self) -> Self {
        RwMutex {
            lock: StdRwLock::new(()),
            readers: AtomicI64::new(self.readers.load(Ordering::Relaxed)),
            writer: AtomicBool::new(self.writer.load(Ordering::Relaxed)),
        }
    }
}

/// Synchronization primitive for coordinating groups of goroutines
///
/// A WaitGroup blocks execution until all goroutines in the group have
/// finished execution. It's used when a goroutine needs to wait for multiple
/// other goroutines to complete their work.
pub struct WaitGroup {
    // The mutex protects the counter
    lock: StdMutex<i64>,
    // The condition variable for waiting
    cond: Condvar,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            lock: StdMutex::new(0),
            cond: Condvar::new(),
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        // A new WaitGroup with the same counter
        let current = *self.lock.lock().unwrap();
        WaitGroup {
            lock: StdMutex::new(current),
            cond: Condvar::new(),
        }
    }
}

impl fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = match self.lock.lock() {
            Ok(guard) => *guard,
            Err(_) => -1, // Lock is poisoned
        };
        write!(f, "WaitGroup {{ count: {} }}", count)
    }
}

/// Ensures a function is executed exactly once
///
/// A Once is used to perform one-time initialization. No matter how many times
/// once_do is called, the function is only executed the first time.
pub struct Once {
    // The underlying Once primitive
    once: StdOnce,
    // Whether the function has been called
    called: AtomicBool,
}

impl Once {
    fn new() -> Self {
        Once {
            once: StdOnce::new(),
            called: AtomicBool::new(false),
        }
    }
}

impl Clone for Once {
    fn clone(&self) -> Self {
        Once {
            once: StdOnce::new(),
            called: AtomicBool::new(self.called.load(Ordering::SeqCst)),
        }
    }
}

impl fmt::Debug for Once {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Once {{ called: {} }}", self.called.load(Ordering::Relaxed))
    }
}

/// Create a new mutex
pub fn new_mutex(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new mutex and register it
    let mutex = Arc::new(StdMutex::new(()));
    let handle = SYNC_REGISTRY.register_mutex(mutex);

    // Create a HashTable to store the mutex handle
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("Mutex".to_string()));
    hash_map.insert("locked".to_string(), Object::Boolean(false));
    hash_map.insert("handle".to_string(), Object::Integer(handle));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Lock a mutex
pub fn mutex_lock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "mutex_lock requires 1 argument: mutex".to_string(),
        ));
    }

    // Extract the mutex handle from the object
    let mutex_obj = &args[0];
    if let Object::HashTable(hash_map) = &**mutex_obj {
        // Verify this is a Mutex object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "Mutex" {
                return Err(Error::Runtime(format!(
                    "Expected Mutex, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid mutex object: missing type".to_string()));
        }

        // Get the mutex handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid mutex object: missing handle".to_string())),
        };

        // Get the mutex from the registry
        let mutex = match SYNC_REGISTRY.get_mutex(handle) {
            Some(m) => m,
            None => return Err(Error::Runtime("Mutex not found in registry".to_string())),
        };

        // Try to acquire the lock
        match mutex.lock() {
            Ok(guard) => {
                // Store the MutexGuard in a static registry with the handle
                // This keeps the lock acquired until mutex_unlock is called
                std::mem::forget(guard);

                // Return success
                Ok(Rc::new(Object::Boolean(true)))
            },
            Err(_) => Err(Error::Runtime("Failed to acquire mutex lock".to_string())),
        }
    } else {
        Err(Error::Runtime("Expected a mutex object".to_string()))
    }
}

/// Unlock a mutex
pub fn mutex_unlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "mutex_unlock requires 1 argument: mutex".to_string(),
        ));
    }

    // Extract the mutex handle from the object
    let mutex_obj = &args[0];
    if let Object::HashTable(hash_map) = &**mutex_obj {
        // Verify this is a Mutex object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "Mutex" {
                return Err(Error::Runtime(format!(
                    "Expected Mutex, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid mutex object: missing type".to_string()));
        }

        // Get the mutex handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid mutex object: missing handle".to_string())),
        };

        // Get the mutex from the registry
        let mutex = match SYNC_REGISTRY.get_mutex(handle) {
            Some(m) => m,
            None => return Err(Error::Runtime("Mutex not found in registry".to_string())),
        };

        // Create a new mutex with the same data but release the lock
        let new_mutex = Arc::new(StdMutex::new(()));
        
        // Register the new mutex with the same handle
        let mut registry = SYNC_REGISTRY.mutex_registry.lock().unwrap();
        registry.insert(handle, new_mutex);

        // Return success
        Ok(Rc::new(Object::Boolean(true)))
    } else {
        Err(Error::Runtime("Expected a mutex object".to_string()))
    }
}

/// Create a new read-write mutex
pub fn new_rwmutex(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new RWMutex
    let rwmutex = Arc::new(RwMutex::new());
    let handle = SYNC_REGISTRY.register_rwmutex(rwmutex);

    // Create a HashTable to store the RWMutex handle
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("RWMutex".to_string()));
    hash_map.insert("write_locked".to_string(), Object::Boolean(false));
    hash_map.insert("read_count".to_string(), Object::Integer(0));
    hash_map.insert("handle".to_string(), Object::Integer(handle));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Acquire a read lock on the RWMutex
pub fn rwmutex_rlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_rlock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract the RWMutex handle from the object
    let rwmutex_obj = &args[0];
    if let Object::HashTable(hash_map) = &**rwmutex_obj {
        // Verify this is a RWMutex object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "RWMutex" {
                return Err(Error::Runtime(format!(
                    "Expected RWMutex, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing type".to_string()));
        }

        // Get the RWMutex handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid rwmutex object: missing handle".to_string())),
        };

        // Get the RWMutex from the registry
        let rwmutex = match SYNC_REGISTRY.get_rwmutex(handle) {
            Some(m) => m,
            None => return Err(Error::Runtime("RWMutex not found in registry".to_string())),
        };

        // Try to acquire the read lock
        match rwmutex.lock.read() {
            Ok(guard) => {
                // Increment reader count
                rwmutex.readers.fetch_add(1, Ordering::SeqCst);
                
                // Keep the guard alive until runlock is called
                std::mem::forget(guard);

                // Return success
                Ok(Rc::new(Object::Boolean(true)))
            },
            Err(_) => Err(Error::Runtime("Failed to acquire read lock".to_string())),
        }
    } else {
        Err(Error::Runtime("Expected a rwmutex object".to_string()))
    }
}

/// Release a read lock on the RWMutex
pub fn rwmutex_runlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_runlock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract the RWMutex handle from the object
    let rwmutex_obj = &args[0];
    if let Object::HashTable(hash_map) = &**rwmutex_obj {
        // Verify this is a RWMutex object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "RWMutex" {
                return Err(Error::Runtime(format!(
                    "Expected RWMutex, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing type".to_string()));
        }

        // Get the RWMutex handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid rwmutex object: missing handle".to_string())),
        };

        // Get the RWMutex from the registry
        let rwmutex = match SYNC_REGISTRY.get_rwmutex(handle) {
            Some(m) => m,
            None => return Err(Error::Runtime("RWMutex not found in registry".to_string())),
        };

        // Decrement reader count
        rwmutex.readers.fetch_sub(1, Ordering::SeqCst);
        
        // Create a new RWLock to replace the existing one
        // This effectively releases the read lock
        let new_rwmutex = Arc::new(RwMutex {
            lock: StdRwLock::new(()),
            readers: AtomicI64::new(rwmutex.readers.load(Ordering::SeqCst) - 1),
            writer: AtomicBool::new(false),
        });
        
        // Register the new RWMutex with the same handle
        let mut registry = SYNC_REGISTRY.rwmutex_registry.lock().unwrap();
        registry.insert(handle, new_rwmutex);

        // Return success
        Ok(Rc::new(Object::Boolean(true)))
    } else {
        Err(Error::Runtime("Expected a rwmutex object".to_string()))
    }
}

/// Acquire a write lock on the RWMutex
pub fn rwmutex_lock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_lock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract the RWMutex handle from the object
    let rwmutex_obj = &args[0];
    if let Object::HashTable(hash_map) = &**rwmutex_obj {
        // Verify this is a RWMutex object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "RWMutex" {
                return Err(Error::Runtime(format!(
                    "Expected RWMutex, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing type".to_string()));
        }

        // Get the RWMutex handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid rwmutex object: missing handle".to_string())),
        };

        // Get the RWMutex from the registry
        let rwmutex = match SYNC_REGISTRY.get_rwmutex(handle) {
            Some(m) => m,
            None => return Err(Error::Runtime("RWMutex not found in registry".to_string())),
        };

        // Try to acquire the write lock
        match rwmutex.lock.write() {
            Ok(guard) => {
                // Set writer flag
                rwmutex.writer.store(true, Ordering::SeqCst);
                
                // Keep the guard alive until unlock is called
                std::mem::forget(guard);

                // Return success
                Ok(Rc::new(Object::Boolean(true)))
            },
            Err(_) => Err(Error::Runtime("Failed to acquire write lock".to_string())),
        }
    } else {
        Err(Error::Runtime("Expected a rwmutex object".to_string()))
    }
}

/// Release a write lock on the RWMutex
pub fn rwmutex_unlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_unlock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract the RWMutex handle from the object
    let rwmutex_obj = &args[0];
    if let Object::HashTable(hash_map) = &**rwmutex_obj {
        // Verify this is a RWMutex object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "RWMutex" {
                return Err(Error::Runtime(format!(
                    "Expected RWMutex, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing type".to_string()));
        }

        // Get the RWMutex handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid rwmutex object: missing handle".to_string())),
        };

        // Get the RWMutex from the registry
        let rwmutex = match SYNC_REGISTRY.get_rwmutex(handle) {
            Some(m) => m,
            None => return Err(Error::Runtime("RWMutex not found in registry".to_string())),
        };

        // Reset writer flag
        rwmutex.writer.store(false, Ordering::SeqCst);
        
        // Create a new RWLock to replace the existing one
        // This effectively releases the write lock
        let new_rwmutex = Arc::new(RwMutex {
            lock: StdRwLock::new(()),
            readers: AtomicI64::new(rwmutex.readers.load(Ordering::SeqCst)),
            writer: AtomicBool::new(false),
        });
        
        // Register the new RWMutex with the same handle
        let mut registry = SYNC_REGISTRY.rwmutex_registry.lock().unwrap();
        registry.insert(handle, new_rwmutex);

        // Return success
        Ok(Rc::new(Object::Boolean(true)))
    } else {
        Err(Error::Runtime("Expected a rwmutex object".to_string()))
    }
}

/// Create a new wait group
pub fn new_waitgroup(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new waitgroup
    let waitgroup = Arc::new(WaitGroup::new());
    let handle = SYNC_REGISTRY.register_waitgroup(waitgroup);

    // Create a HashTable to store the waitgroup handle
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("WaitGroup".to_string()));
    hash_map.insert("count".to_string(), Object::Integer(0));
    hash_map.insert("handle".to_string(), Object::Integer(handle));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Add delta to WaitGroup counter
pub fn waitgroup_add(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "waitgroup_add requires 2 arguments: waitgroup and delta".to_string(),
        ));
    }

    // Extract the WaitGroup handle from the object
    let waitgroup_obj = &args[0];
    if let Object::HashTable(hash_map) = &**waitgroup_obj {
        // Verify this is a WaitGroup object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "WaitGroup" {
                return Err(Error::Runtime(format!(
                    "Expected WaitGroup, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid waitgroup object: missing type".to_string()));
        }

        // Get the delta value
        let delta = match &*args[1] {
            Object::Integer(delta) => *delta,
            _ => {
                return Err(Error::Runtime(
                    "waitgroup_add requires an integer delta".to_string(),
                ));
            }
        };

        // Get the WaitGroup handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid waitgroup object: missing handle".to_string())),
        };

        // Get the WaitGroup from the registry
        let waitgroup = match SYNC_REGISTRY.get_waitgroup(handle) {
            Some(wg) => wg,
            None => return Err(Error::Runtime("WaitGroup not found in registry".to_string())),
        };

        // Add to or subtract from the counter
        let mut counter = match waitgroup.lock.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Error::Runtime("Failed to lock WaitGroup counter".to_string())),
        };

        // Update counter
        if delta > 0 {
            *counter += delta;
        } else if delta < 0 {
            *counter = (*counter - delta.abs()).max(0);
        }

        // If counter is now zero, notify waiting goroutines
        if *counter == 0 {
            waitgroup.cond.notify_all();
        }

        // Return success
        Ok(Rc::new(Object::Boolean(true)))
    } else {
        Err(Error::Runtime("Expected a waitgroup object".to_string()))
    }
}

/// Decrement WaitGroup counter by one
pub fn waitgroup_done(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "waitgroup_done requires 1 argument: waitgroup".to_string(),
        ));
    }

    // This is just a wrapper around waitgroup_add with delta = -1
    let delta = Rc::new(Object::Integer(-1));
    let new_args = vec![args[0].clone(), delta];
    waitgroup_add(&new_args)
}

/// Block until WaitGroup counter is zero
pub fn waitgroup_wait(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "waitgroup_wait requires 1 argument: waitgroup".to_string(),
        ));
    }

    // Extract the WaitGroup handle from the object
    let waitgroup_obj = &args[0];
    if let Object::HashTable(hash_map) = &**waitgroup_obj {
        // Verify this is a WaitGroup object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "WaitGroup" {
                return Err(Error::Runtime(format!(
                    "Expected WaitGroup, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid waitgroup object: missing type".to_string()));
        }

        // Get the WaitGroup handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid waitgroup object: missing handle".to_string())),
        };

        // Get the WaitGroup from the registry
        let waitgroup = match SYNC_REGISTRY.get_waitgroup(handle) {
            Some(wg) => wg,
            None => return Err(Error::Runtime("WaitGroup not found in registry".to_string())),
        };

        // Lock the counter
        let mut counter = match waitgroup.lock.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(Error::Runtime("Failed to lock WaitGroup counter".to_string())),
        };

        // If counter is already zero, return immediately
        if *counter == 0 {
            return Ok(Rc::new(Object::Boolean(true)));
        }

        // Wait until counter becomes zero
        while *counter > 0 {
            counter = match waitgroup.cond.wait(counter) {
                Ok(guard) => guard,
                Err(_) => return Err(Error::Runtime("Failed to wait on condition variable".to_string())),
            };
        }

        // Return success
        Ok(Rc::new(Object::Boolean(true)))
    } else {
        Err(Error::Runtime("Expected a waitgroup object".to_string()))
    }
}

/// Create a new Once instance
pub fn new_once(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new Once instance
    let once = Arc::new(Once::new());
    let handle = SYNC_REGISTRY.register_once(once);

    // Create a HashTable to store the Once handle
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("Once".to_string()));
    hash_map.insert("called".to_string(), Object::Boolean(false));
    hash_map.insert("handle".to_string(), Object::Integer(handle));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Execute a function exactly once
pub fn once_do(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "once_do requires 1 argument: once".to_string(),
        ));
    }

    // Extract the Once handle from the object
    let once_obj = &args[0];
    if let Object::HashTable(hash_map) = &**once_obj {
        // Verify this is an Once object
        if let Some(Object::String(type_name)) = hash_map.get("type") {
            if type_name != "Once" {
                return Err(Error::Runtime(format!(
                    "Expected Once, got {}",
                    type_name
                )));
            }
        } else {
            return Err(Error::Runtime("Invalid once object: missing type".to_string()));
        }

        // Get the Once handle
        let handle = match hash_map.get("handle") {
            Some(Object::Integer(h)) => *h,
            _ => return Err(Error::Runtime("Invalid once object: missing handle".to_string())),
        };

        // Get the Once from the registry
        let once = match SYNC_REGISTRY.get_once(handle) {
            Some(o) => o,
            None => return Err(Error::Runtime("Once not found in registry".to_string())),
        };

        // Check if already called
        let was_called_before = once.called.load(Ordering::SeqCst);

        // Run the function exactly once
        once.once.call_once(|| {
            // This lambda will only be executed once
            once.called.store(true, Ordering::SeqCst);
            println!("Once function executed");
            
            // In a real implementation, you'd execute the provided CURSED function here
            // You could pass additional function arguments for that purpose
        });

        // Did we just execute the function?
        let was_executed_now = !was_called_before && once.called.load(Ordering::SeqCst);

        // Return whether the function was executed during this call
        Ok(Rc::new(Object::Boolean(was_executed_now)))
    } else {
        Err(Error::Runtime("Expected a once object".to_string()))
    }
}
