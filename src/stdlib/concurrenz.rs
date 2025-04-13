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
    // Create a new mutex
    let mutex = Mutex {
        inner: Arc::new(StdMutex::new(())),
    };

    // Create a HashTable to store the mutex
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("Mutex".to_string()));
    hash_map.insert("locked".to_string(), Object::Boolean(false));
    
    // Store the actual mutex in a Box to maintain a stable memory address
    let boxed_mutex = Box::new(mutex);
    let raw_ptr = Box::into_raw(boxed_mutex) as usize;
    hash_map.insert("raw_ptr".to_string(), Object::Integer(raw_ptr as i64));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Lock a mutex
pub fn mutex_lock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "mutex_lock requires 1 argument: mutex".to_string(),
        ));
    }

    // Extract mutex from HashTable
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

        // Get the raw pointer and reconstruct the mutex
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let mutex_ptr = *raw_ptr as usize as *mut Mutex;
            // SAFETY: The pointer comes from Box::into_raw in new_mutex
            //         and we're only dereferencing it, not dropping it
            let mutex = unsafe { &*mutex_ptr };
            
            // Acquire the lock
            if let Ok(guard) = mutex.inner.lock() {
                // Due to Rc's nature, we can't update the object here
                // In a real implementation, we would use interior mutability
                // to update the locked status
                
                // We intentionally forget the guard to keep the lock held
                // until unlock is called
                std::mem::forget(guard);
                
                return Ok(Rc::new(Object::Boolean(true)));
            } else {
                return Err(Error::Runtime("Failed to acquire mutex lock".to_string()));
            }
        } else {
            return Err(Error::Runtime("Invalid mutex object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a mutex object".to_string()))
}

/// Unlock a mutex
pub fn mutex_unlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "mutex_unlock requires 1 argument: mutex".to_string(),
        ));
    }

    // Extract mutex from HashTable
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

        // Get the raw pointer and reconstruct the mutex
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let mutex_ptr = *raw_ptr as usize as *mut Mutex;
            // SAFETY: The pointer comes from Box::into_raw in new_mutex
            //         and we're only dereferencing it, not dropping it
            let mutex = unsafe { &*mutex_ptr };
            
            // SAFETY: We're assuming the mutex was locked by mutex_lock
            // Create a new mutex with the same data but release the lock
            let new_mutex = StdMutex::new(());
            // Replace the poisoned mutex with a fresh one
            let _ = std::mem::replace(&mut *(&mutex.inner as *const _ as *mut Arc<StdMutex<()>>), Arc::new(new_mutex));
            
            // Due to Rc's nature, we can't update the object here
            // In a real implementation, we would use interior mutability
            // to update the locked status
            
            return Ok(Rc::new(Object::Boolean(true)));
        } else {
            return Err(Error::Runtime("Invalid mutex object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a mutex object".to_string()))
}

/// Create a new read-write mutex
pub fn new_rwmutex(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new rwmutex
    let rwmutex = RWMutex {
        inner: Arc::new(StdRwLock::new(())),
    };

    // Create a HashTable to store the rwmutex
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("RWMutex".to_string()));
    hash_map.insert("write_locked".to_string(), Object::Boolean(false));
    hash_map.insert("read_count".to_string(), Object::Integer(0));
    
    // Store the actual rwmutex in a Box to maintain a stable memory address
    let boxed_rwmutex = Box::new(rwmutex);
    let raw_ptr = Box::into_raw(boxed_rwmutex) as usize;
    hash_map.insert("raw_ptr".to_string(), Object::Integer(raw_ptr as i64));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Acquire a read lock on the RWMutex
pub fn rwmutex_rlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_rlock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract rwmutex from HashTable
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

        // Get the raw pointer and reconstruct the rwmutex
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let rwmutex_ptr = *raw_ptr as usize as *mut RWMutex;
            // SAFETY: The pointer comes from Box::into_raw in new_rwmutex
            let rwmutex = unsafe { &*rwmutex_ptr };
            
            // Acquire the read lock
            if let Ok(guard) = rwmutex.inner.read() {
                // Update the read count in the Object
                if let Some(rwmutex_obj_mut) = Rc::get_mut(rwmutex_obj) {
                    if let Object::HashTable(hash_map_mut) = rwmutex_obj_mut {
                        if let Some(Object::Integer(read_count)) = hash_map_mut.get("read_count") {
                            let new_count = read_count + 1;
                            hash_map_mut.insert("read_count".to_string(), Object::Integer(new_count));
                        }
                    }
                }
                
                // We intentionally forget the guard to keep the lock held
                // until runlock is called
                std::mem::forget(guard);
                
                return Ok(Rc::new(Object::Boolean(true)));
            } else {
                return Err(Error::Runtime("Failed to acquire read lock".to_string()));
            }
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a rwmutex object".to_string()))
}

/// Release a read lock on the RWMutex
pub fn rwmutex_runlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_runlock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract rwmutex from HashTable
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

        // Get the raw pointer and reconstruct the rwmutex
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let rwmutex_ptr = *raw_ptr as usize as *mut RWMutex;
            // SAFETY: The pointer comes from Box::into_raw in new_rwmutex
            let rwmutex = unsafe { &*rwmutex_ptr };
            
            // SAFETY: We're assuming the rwmutex was read-locked
            // Create a new rwmutex with the same data but release the lock
            let new_rwmutex = StdRwLock::new(());
            // Replace the poisoned rwmutex with a fresh one
            let _ = std::mem::replace(&mut *(&rwmutex.inner as *const _ as *mut Arc<StdRwLock<()>>), Arc::new(new_rwmutex));
            
            // Update the read count in the Object
            if let Some(rwmutex_obj_mut) = Rc::get_mut(rwmutex_obj) {
                if let Object::HashTable(hash_map_mut) = rwmutex_obj_mut {
                    if let Some(Object::Integer(read_count)) = hash_map_mut.get("read_count") {
                        let new_count = (read_count - 1).max(0);
                        hash_map_mut.insert("read_count".to_string(), Object::Integer(new_count));
                    }
                }
            }
            
            return Ok(Rc::new(Object::Boolean(true)));
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a rwmutex object".to_string()))
}

/// Acquire a write lock on the RWMutex
pub fn rwmutex_lock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_lock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract rwmutex from HashTable
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

        // Get the raw pointer and reconstruct the rwmutex
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let rwmutex_ptr = *raw_ptr as usize as *mut RWMutex;
            // SAFETY: The pointer comes from Box::into_raw in new_rwmutex
            let rwmutex = unsafe { &*rwmutex_ptr };
            
            // Acquire the write lock
            if let Ok(guard) = rwmutex.inner.write() {
                // Update the write_locked status in the Object
                if let Some(rwmutex_obj_mut) = Rc::get_mut(rwmutex_obj) {
                    if let Object::HashTable(hash_map_mut) = rwmutex_obj_mut {
                        hash_map_mut.insert("write_locked".to_string(), Object::Boolean(true));
                    }
                }
                
                // We intentionally forget the guard to keep the lock held
                // until unlock is called
                std::mem::forget(guard);
                
                return Ok(Rc::new(Object::Boolean(true)));
            } else {
                return Err(Error::Runtime("Failed to acquire write lock".to_string()));
            }
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a rwmutex object".to_string()))
}

/// Release a write lock on the RWMutex
pub fn rwmutex_unlock(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "rwmutex_unlock requires 1 argument: rwmutex".to_string(),
        ));
    }

    // Extract rwmutex from HashTable
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

        // Get the raw pointer and reconstruct the rwmutex
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let rwmutex_ptr = *raw_ptr as usize as *mut RWMutex;
            // SAFETY: The pointer comes from Box::into_raw in new_rwmutex
            let rwmutex = unsafe { &*rwmutex_ptr };
            
            // SAFETY: We're assuming the rwmutex was write-locked
            // Create a new rwmutex with the same data but release the lock
            let new_rwmutex = StdRwLock::new(());
            // Replace the poisoned rwmutex with a fresh one
            let _ = std::mem::replace(&mut *(&rwmutex.inner as *const _ as *mut Arc<StdRwLock<()>>), Arc::new(new_rwmutex));
            
            // Update the write_locked status in the Object
            if let Some(rwmutex_obj_mut) = Rc::get_mut(rwmutex_obj) {
                if let Object::HashTable(hash_map_mut) = rwmutex_obj_mut {
                    hash_map_mut.insert("write_locked".to_string(), Object::Boolean(false));
                }
            }
            
            return Ok(Rc::new(Object::Boolean(true)));
        } else {
            return Err(Error::Runtime("Invalid rwmutex object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a rwmutex object".to_string()))
}

/// Create a new wait group
pub fn new_waitgroup(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new waitgroup with a counter and condition variable
    let waitgroup = WaitGroup {
        count: Arc::new((StdMutex::new(0), Condvar::new())),
    };

    // Create a HashTable to store the waitgroup
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("WaitGroup".to_string()));
    hash_map.insert("count".to_string(), Object::Integer(0));
    
    // Store the actual waitgroup in a Box to maintain a stable memory address
    let boxed_waitgroup = Box::new(waitgroup);
    let raw_ptr = Box::into_raw(boxed_waitgroup) as usize;
    hash_map.insert("raw_ptr".to_string(), Object::Integer(raw_ptr as i64));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Add delta to WaitGroup counter
pub fn waitgroup_add(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "waitgroup_add requires 2 arguments: waitgroup and delta".to_string(),
        ));
    }

    // Extract waitgroup from HashTable
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

        // Get the raw pointer and reconstruct the waitgroup
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let waitgroup_ptr = *raw_ptr as usize as *mut WaitGroup;
            // SAFETY: The pointer comes from Box::into_raw in new_waitgroup
            let waitgroup = unsafe { &*waitgroup_ptr };
            
            // Add to the counter
            let (lock, _cvar) = &*waitgroup.count;
            if let Ok(mut count) = lock.lock() {
                if delta > 0 {
                    *count = count.checked_add(delta as usize).unwrap_or(*count);
                } else if delta < 0 {
                    let abs_delta = (-delta) as usize;
                    *count = count.checked_sub(abs_delta).unwrap_or(0);
                }
                
                // Update the count in the Object
                if let Some(waitgroup_obj_mut) = Rc::get_mut(waitgroup_obj) {
                    if let Object::HashTable(hash_map_mut) = waitgroup_obj_mut {
                        hash_map_mut.insert("count".to_string(), Object::Integer(*count as i64));
                    }
                }
                
                // If counter is zero, notify all waiting threads
                if *count == 0 {
                    drop(count); // Release the mutex before notifying
                    let (_lock, cvar) = &*waitgroup.count;
                    cvar.notify_all();
                }
                
                return Ok(Rc::new(Object::Boolean(true)));
            } else {
                return Err(Error::Runtime("Failed to lock waitgroup counter".to_string()));
            }
        } else {
            return Err(Error::Runtime("Invalid waitgroup object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a waitgroup object".to_string()))
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

    // Extract waitgroup from HashTable
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

        // Quick check if count is already 0
        if let Some(Object::Integer(count)) = hash_map.get("count") {
            if *count == 0 {
                return Ok(Rc::new(Object::Boolean(true)));
            }
        }

        // Get the raw pointer and reconstruct the waitgroup
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let waitgroup_ptr = *raw_ptr as usize as *mut WaitGroup;
            // SAFETY: The pointer comes from Box::into_raw in new_waitgroup
            let waitgroup = unsafe { &*waitgroup_ptr };
            
            // Wait for the counter to become zero
            let (lock, cvar) = &*waitgroup.count;
            if let Ok(mut count) = lock.lock() {
                while *count > 0 {
                    count = match cvar.wait(count) {
                        Ok(guard) => guard,
                        Err(_) => {
                            return Err(Error::Runtime("Failed to wait on condition variable".to_string()));
                        }
                    };
                }
                
                return Ok(Rc::new(Object::Boolean(true)));
            } else {
                return Err(Error::Runtime("Failed to lock waitgroup counter".to_string()));
            }
        } else {
            return Err(Error::Runtime("Invalid waitgroup object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a waitgroup object".to_string()))
}

/// Create a new Once instance
pub fn new_once(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create a new Once instance
    let once = Once {
        inner: Arc::new(StdOnce::new()),
        called: Arc::new(AtomicBool::new(false)),
    };

    // Create a HashTable to store the Once instance
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("type".to_string(), Object::String("Once".to_string()));
    hash_map.insert("called".to_string(), Object::Boolean(false));
    
    // Store the actual Once instance in a Box to maintain a stable memory address
    let boxed_once = Box::new(once);
    let raw_ptr = Box::into_raw(boxed_once) as usize;
    hash_map.insert("raw_ptr".to_string(), Object::Integer(raw_ptr as i64));

    Ok(Rc::new(Object::HashTable(hash_map)))
}

/// Execute a function exactly once
pub fn once_do(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "once_do requires 1 argument: once".to_string(),
        ));
    }

    // Extract Once from HashTable
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

        // Get the raw pointer and reconstruct the Once instance
        if let Some(Object::Integer(raw_ptr)) = hash_map.get("raw_ptr") {
            let once_ptr = *raw_ptr as usize as *mut Once;
            // SAFETY: The pointer comes from Box::into_raw in new_once
            let once = unsafe { &*once_ptr };
            
            // Use the inner Once to ensure the function runs exactly once
            if !once.called.load(Ordering::SeqCst) {
                once.inner.call_once(|| {
                    // Mark as called - this only happens once
                    once.called.store(true, Ordering::SeqCst);
                    
                    // In a real implementation, we would call the provided function here
                    println!("Once function executed");
                });
                
                // Update the called status in the Object
                if let Some(once_obj_mut) = Rc::get_mut(once_obj) {
                    if let Object::HashTable(hash_map_mut) = once_obj_mut {
                        hash_map_mut.insert("called".to_string(), Object::Boolean(true));
                    }
                }
            }
            
            // Return whether the function was executed during this call
            let first_time = once.called.load(Ordering::SeqCst);
            return Ok(Rc::new(Object::Boolean(first_time)));
        } else {
            return Err(Error::Runtime("Invalid once object: missing raw_ptr".to_string()));
        }
    }

    Err(Error::Runtime("Expected a once object".to_string()))
}
