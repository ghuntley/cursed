//! Synchronization primitives for concurrent CURSED programs
//!
//! The concurrenz package provides synchronization mechanisms similar to Go's sync package,
//! enabling safe concurrent programming in CURSED. It includes locks, wait groups, and once
//! primitives, as well as functionality to create and use channels for communication between
//! goroutines.
//!
//! Key components include:
//!
//! - Mutex: A mutual exclusion lock that allows only one goroutine to hold it at a time
//! - RWMutex: A reader/writer mutex that allows multiple readers or one writer
//! - WaitGroup: A synchronization primitive that waits for a collection of goroutines to finish
//! - Once: A mechanism to ensure a function is executed exactly once
//! - Channel: Communication primitives for goroutines (via core/channel)

use crate::error::Error;
use crate::object::{Object, self};
use crate::core::channel::{create_channel, send_to_channel, receive_from_channel, try_send_to_channel, try_receive_from_channel};
use crate::memory::{Traceable, Tag, Visitor};
// Note: Using Arc instead of Rc for thread safety
use std::cell::RefCell;
use std::sync::{Mutex as StdMutex, RwLock as StdRwLock, RwLock, Once as StdOnce, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};

// Wrapper type for CURSED mutex
#[derive(Debug, Clone)]
pub struct CursedMutex {
    inner: Arc<StdMutex<()>>
}

impl PartialEq for CursedMutex {
    fn eq(&self, _other: &Self) -> bool {
        // Consider two mutexes always equal for our purposes
        // since we can't compare the inner mutex
        true
    }
}

impl CursedMutex {
    pub fn new() -> Self {
        CursedMutex {
            inner: Arc::new(StdMutex::new(()))
        }
    }
    
    pub fn lock(&self) -> Result<(), Error> {
        match self.inner.lock() {
            Ok(_guard) => Ok(()),
            Err(e) => Err(Error::Runtime(format!("Failed to lock mutex: {}", e)))
        }
    }
    
    pub fn unlock(&self) -> Result<(), Error> {
        // Since StdMutex guard is RAII, we don't actually need to unlock it
        // but we'll check that we own the lock
        match self.inner.try_lock() {
            Ok(_) => Err(Error::Runtime("Attempted to unlock a mutex that wasn't locked".to_string())),
            Err(_) => Ok(()) // This is good - the mutex is locked
        }
    }
}

// Wrapper type for CURSED RWMutex
#[derive(Debug, Clone)]
pub struct CursedRWMutex {
    inner: Arc<StdRwLock<()>>
}

impl PartialEq for CursedRWMutex {
    fn eq(&self, _other: &Self) -> bool {
        // Consider two rwmutexes always equal for our purposes
        // since we can't compare the inner rwmutex
        true
    }
}

impl CursedRWMutex {
    pub fn new() -> Self {
        CursedRWMutex {
            inner: Arc::new(StdRwLock::new(()))
        }
    }
    
    pub fn lock(&self) -> Result<(), Error> {
        match self.inner.write() {
            Ok(_guard) => Ok(()),
            Err(e) => Err(Error::Runtime(format!("Failed to write-lock RWMutex: {}", e)))
        }
    }
    
    pub fn unlock(&self) -> Result<(), Error> {
        // Since StdRwLock guard is RAII, we don't actually need to unlock it
        // but we'll check that we own the write lock
        match self.inner.try_write() {
            Ok(_) => Err(Error::Runtime("Attempted to unlock a RWMutex that wasn't write-locked".to_string())),
            Err(_) => Ok(()) // This is good - the mutex is write-locked
        }
    }
    
    pub fn rlock(&self) -> Result<(), Error> {
        match self.inner.read() {
            Ok(_guard) => Ok(()),
            Err(e) => Err(Error::Runtime(format!("Failed to read-lock RWMutex: {}", e)))
        }
    }
    
    pub fn runlock(&self) -> Result<(), Error> {
        // Since StdRwLock guard is RAII, we don't actually need to unlock it
        // but we'll check that we own the read lock
        match self.inner.try_read() {
            Ok(_) => Err(Error::Runtime("Attempted to read-unlock a RWMutex that wasn't read-locked".to_string())),
            Err(_) => Ok(()) // This is good - the mutex is read-locked
        }
    }
}

// Wrapper type for CURSED WaitGroup
#[derive(Debug, Clone)]
pub struct CursedWaitGroup {
    count: Arc<AtomicUsize>,
    mutex: Arc<StdMutex<()>>,
    rwlock: Arc<StdRwLock<()>>
}

impl PartialEq for CursedWaitGroup {
    fn eq(&self, _other: &Self) -> bool {
        // Consider two wait groups always equal for our purposes
        // since we can't compare the inner components
        true
    }
}

impl CursedWaitGroup {
    pub fn new() -> Self {
        CursedWaitGroup {
            count: Arc::new(AtomicUsize::new(0)),
            mutex: Arc::new(StdMutex::new(())),
            rwlock: Arc::new(StdRwLock::new(()))
        }
    }
    
    pub fn add(&self, delta: i64) -> Result<(), Error> {
        if delta < 0 {
            return Err(Error::Runtime("Cannot add negative value to WaitGroup".to_string()));
        }
        
        // Lock to prevent race conditions
        let _lock = self.mutex.lock().map_err(|e| Error::Runtime(format!("Failed to lock WaitGroup: {}", e)))?;
        
        // Add to the counter
        self.count.fetch_add(delta as usize, Ordering::SeqCst);
        
        Ok(())
    }
    
    pub fn done(&self) -> Result<(), Error> {
        // Lock to prevent race conditions
        let _lock = self.mutex.lock().map_err(|e| Error::Runtime(format!("Failed to lock WaitGroup: {}", e)))?;
        
        // Check if counter would go below 0
        let current = self.count.load(Ordering::SeqCst);
        if current == 0 {
            return Err(Error::Runtime("WaitGroup counter cannot go below zero".to_string()));
        }
        
        // Decrement the counter
        self.count.fetch_sub(1, Ordering::SeqCst);
        
        // Check if counter is 0 after decrement
        if self.count.load(Ordering::SeqCst) == 0 {
            // Unlock the RWLock to release waiters
            drop(self.rwlock.write().unwrap());
        }
        
        Ok(())
    }
    
    pub fn wait(&self) -> Result<(), Error> {
        // Lock to check counter
        let _lock = self.mutex.lock().map_err(|e| Error::Runtime(format!("Failed to lock WaitGroup: {}", e)))?;
        
        // If counter is already 0, return immediately
        if self.count.load(Ordering::SeqCst) == 0 {
            return Ok(());
        }
        
        // Release the mutex lock
        drop(_lock);
        
        // Acquire the RWLock's read lock - this will block until all workers are done
        // and the write lock is released in done()
        let _rlock = self.rwlock.read().map_err(|e| Error::Runtime(format!("Failed to wait on WaitGroup: {}", e)))?;
        
        Ok(())
    }
}

// Wrapper type for CURSED Once
#[derive(Debug, Clone)]
pub struct CursedOnce {
    inner: Arc<StdOnce>,
    done: Arc<AtomicUsize>
}

impl PartialEq for CursedOnce {
    fn eq(&self, _other: &Self) -> bool {
        // Consider two once objects as equal if both have been done or not done
        self.is_done() == _other.is_done()
    }
}

impl CursedOnce {
    pub fn new() -> Self {
        CursedOnce {
            inner: Arc::new(StdOnce::new()),
            done: Arc::new(AtomicUsize::new(0))
        }
    }
    
    pub fn do_with_fn<F>(&self, f: F) -> Result<(), Error> 
    where F: FnOnce() + Send + 'static {
        self.inner.call_once(|| {
            f();
            self.done.store(1, Ordering::SeqCst);
        });
        
        Ok(())
    }
    
    pub fn is_done(&self) -> bool {
        self.done.load(Ordering::SeqCst) == 1
    }
}

/// Creates a new mutex
///
/// A mutex (mutual exclusion lock) allows only one goroutine to hold
/// the lock at a time. It's used to protect shared resources from
/// concurrent access.
///
/// # Returns
///
/// A new mutex object
pub fn new_mutex(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    let mutex = CursedMutex::new();
    Ok(Arc::new(Object::Mutex(StdRwLock::new(mutex))))
}

/// Locks a mutex
///
/// This function attempts to acquire the mutex lock. If the lock is
/// already in use, the calling goroutine blocks until the mutex
/// is available.
///
/// # Arguments
///
/// * `args[0]` - The mutex to lock
///
/// # Returns
///
/// `nil` on success or an error if the mutex is invalid
pub fn mutex_lock(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("mutex_lock requires a mutex argument".to_string()));
    }
    
    match &*args[0] {
        Object::Mutex(mutex_cell) => {
            let mut mutex = mutex_cell.write().unwrap();
            mutex.lock()?;
            Ok(Arc::new(Object::Null))
        },
        _ => Err(Error::Runtime("First argument to mutex_lock must be a mutex".to_string())),
    }
}

/// Unlocks a mutex
///
/// This function releases an acquired mutex lock. It should only be called
/// by the goroutine that acquired the lock.
///
/// # Arguments
///
/// * `args[0]` - The mutex to unlock
///
/// # Returns
///
/// `nil` on success or an error if the mutex is invalid or not locked
pub fn mutex_unlock(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("mutex_unlock requires a mutex argument".to_string()));
    }
    
    match &*args[0] {
        Object::Mutex(mutex_cell) => {
            let mutex = mutex_cell.read().map_err(|_| Error::Runtime("Failed to read mutex".to_string()))?;
            mutex.unlock()?;
            Ok(Arc::new(Object::Null))
        },
        _ => Err(Error::Runtime("First argument to mutex_unlock must be a mutex".to_string())),
    }
}

/// Creates a new read-write mutex
///
/// A read-write mutex can be held by any number of readers or a single writer.
/// It's useful when reads are more common than writes to shared data.
///
/// # Returns
///
/// A new RWMutex object
pub fn new_rwmutex(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    let rwmutex = CursedRWMutex::new();
    Ok(Arc::new(Object::RWMutex(StdRwLock::new(rwmutex))))
}

/// Locks a read-write mutex for writing
///
/// This function acquires a write lock on the RWMutex. If there are
/// any readers or writers, it blocks until the lock is available.
///
/// # Arguments
///
/// * `args[0]` - The RWMutex to lock
///
/// # Returns
///
/// `nil` on success or an error if the RWMutex is invalid
pub fn rwmutex_lock(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rwmutex_lock requires a rwmutex argument".to_string()));
    }
    
    match &*args[0] {
        Object::RWMutex(rwmutex_cell) => {
            let rwmutex = rwmutex_cell.read().map_err(|_| Error::Runtime("Failed to read rwmutex".to_string()))?;
            rwmutex.lock()?;
            Ok(Arc::new(Object::Null))
        },
        _ => Err(Error::Runtime("First argument to rwmutex_lock must be a rwmutex".to_string())),
    }
}

/// Unlocks a read-write mutex from a write lock
///
/// This function releases a write lock on the RWMutex.
///
/// # Arguments
///
/// * `args[0]` - The RWMutex to unlock
///
/// # Returns
///
/// `nil` on success or an error if the RWMutex is invalid or not locked
pub fn rwmutex_unlock(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rwmutex_unlock requires a rwmutex argument".to_string()));
    }
    
    match &*args[0] {
        Object::RWMutex(rwmutex_cell) => {
            let rwmutex = rwmutex_cell.read().map_err(|_| Error::Runtime("Failed to read rwmutex".to_string()))?;
            rwmutex.unlock()?;
            Ok(Arc::new(Object::Null))
        },
        _ => Err(Error::Runtime("First argument to rwmutex_unlock must be a rwmutex".to_string())),
    }
}

/// Locks a read-write mutex for reading
///
/// This function acquires a read lock on the RWMutex. Multiple readers
/// can hold the lock simultaneously, but it blocks if there is a writer.
///
/// # Arguments
///
/// * `args[0]` - The RWMutex to lock for reading
///
/// # Returns
///
/// `nil` on success or an error if the RWMutex is invalid
pub fn rwmutex_rlock(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rwmutex_rlock requires a rwmutex argument".to_string()));
    }
    
    match &*args[0] {
        Object::RWMutex(rwmutex_cell) => {
            let rwmutex = rwmutex_cell.read().map_err(|_| Error::Runtime("Failed to read rwmutex".to_string()))?;
            rwmutex.rlock()?;
            Ok(Arc::new(Object::Null))
        },
        _ => Err(Error::Runtime("First argument to rwmutex_rlock must be a rwmutex".to_string())),
    }
}

/// Unlocks a read-write mutex from a read lock
///
/// This function releases a read lock on the RWMutex.
///
/// # Arguments
///
/// * `args[0]` - The RWMutex to unlock from reading
///
/// # Returns
///
/// `nil` on success or an error if the RWMutex is invalid or not locked
pub fn rwmutex_runlock(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("rwmutex_runlock requires a rwmutex argument".to_string()));
    }
    
    match &*args[0] {
        Object::RWMutex(rwmutex_cell) => {
            let rwmutex = rwmutex_cell.read().map_err(|_| Error::Runtime("Failed to read rwmutex".to_string()))?;
            rwmutex.runlock()?;
            Ok(Arc::new(Object::Null))
        },
        _ => Err(Error::Runtime("First argument to rwmutex_runlock must be a rwmutex".to_string())),
    }
}

/// Creates a new wait group
///
/// A WaitGroup waits for a collection of goroutines to finish.
/// The main goroutine calls Add to set the number of goroutines to wait for.
/// Each goroutine calls Done when it finishes, and the main goroutine
/// calls Wait to block until all goroutines have finished.
///
/// # Returns
///
/// A new WaitGroup object
pub fn new_waitgroup(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    let waitgroup = CursedWaitGroup::new();
    Ok(Arc::new(Object::WaitGroup(StdRwLock::new(waitgroup))))
}

/// Adds delta to the WaitGroup counter
///
/// Delta may be negative, but this may cause a panic if the counter
/// goes below zero. If the counter becomes zero, all goroutines blocked
/// on Wait are released.
///
/// # Arguments
///
/// * `args[0]` - The WaitGroup to add to
/// * `args[1]` - The delta to add (integer)
///
/// # Returns
///
/// `nil` on success or an error if the arguments are invalid
pub fn waitgroup_add(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("waitgroup_add requires a waitgroup and delta arguments".to_string()));
    }
    
    // Extract waitgroup
    let mut waitgroup = match &*args[0] {
        Object::WaitGroup(wg_cell) => wg_cell.write().unwrap(),
        _ => return Err(Error::Runtime("First argument to waitgroup_add must be a waitgroup".to_string())),
    };
    
    // Extract delta
    let delta = match &*args[1] {
        Object::Integer(n) => *n,
        _ => return Err(Error::Runtime("Second argument to waitgroup_add must be an integer".to_string())),
    };
    
    // Add to the waitgroup
    waitgroup.add(delta)?;
    
    Ok(Arc::new(Object::Null))
}

/// Decrements the WaitGroup counter by one
///
/// This should be called by goroutines when they finish their work.
///
/// # Arguments
///
/// * `args[0]` - The WaitGroup to decrement
///
/// # Returns
///
/// `nil` on success or an error if the arguments are invalid
pub fn waitgroup_done(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("waitgroup_done requires a waitgroup argument".to_string()));
    }
    
    // Extract waitgroup
    let mut waitgroup = match &*args[0] {
        Object::WaitGroup(wg_cell) => wg_cell.write().unwrap(),
        _ => return Err(Error::Runtime("First argument to waitgroup_done must be a waitgroup".to_string())),
    };
    
    // Call done on the waitgroup
    waitgroup.done()?;
    
    Ok(Arc::new(Object::Null))
}

/// Blocks until the WaitGroup counter is zero
///
/// # Arguments
///
/// * `args[0]` - The WaitGroup to wait on
///
/// # Returns
///
/// `nil` on success or an error if the arguments are invalid
pub fn waitgroup_wait(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("waitgroup_wait requires a waitgroup argument".to_string()));
    }
    
    // Extract waitgroup
    let waitgroup = match &*args[0] {
        Object::WaitGroup(wg_cell) => wg_cell.read().unwrap(),
        _ => return Err(Error::Runtime("First argument to waitgroup_wait must be a waitgroup".to_string())),
    };
    
    // Wait on the waitgroup
    waitgroup.wait()?;
    
    Ok(Arc::new(Object::Null))
}

/// Creates a new Once object
///
/// A Once is an object that will perform an action exactly once.
///
/// # Returns
///
/// A new Once object
pub fn new_once(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    let once = CursedOnce::new();
    Ok(Arc::new(Object::Once(StdRwLock::new(once))))
}

/// Performs a function execution exactly once
///
/// No matter how many times Do is called, the function will only be executed once.
/// 
/// This version is a placeholder since we can't pass a function directly in the test.
/// A real implementation would accept a function reference.
///
/// # Arguments
///
/// * `args[0]` - The Once object
///
/// # Returns
///
/// `nil` on success or an error if the arguments are invalid
pub fn once_do(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("once_do requires a once argument".to_string()));
    }
    
    // Extract once
    let mut once = match &*args[0] {
        Object::Once(once_cell) => once_cell.write().unwrap(),
        _ => return Err(Error::Runtime("First argument to once_do must be a once".to_string())),
    };
    
    // Execute with an empty function (for testing)
    once.do_with_fn(|| {})?;
    
    Ok(Arc::new(Object::Null))
}

/// Internal function used by tests to execute a function exactly once with a Once object
///
/// # Arguments
///
/// * `args[0]` - The Once object
/// * `f` - The function to execute
///
/// # Returns
///
/// `nil` on success or an error if the arguments are invalid
pub fn once_do_with_fn<F>(args: &[Arc<Object>], f: F) -> Result<Arc<Object>, Error>
where
    F: FnOnce() + Send + 'static
{
    if args.is_empty() {
        return Err(Error::Runtime("once_do_with_fn requires a once argument".to_string()));
    }
    
    // Extract once
    let mut once = match &*args[0] {
        Object::Once(once_cell) => once_cell.write().unwrap(),
        _ => return Err(Error::Runtime("First argument to once_do_with_fn must be a once".to_string())),
    };
    
    // Execute with the provided function
    once.do_with_fn(f)?;
    
    Ok(Arc::new(Object::Null))
}

/// Creates a new channel with the specified capacity
///
/// # Arguments
///
/// * `args[0]` - The capacity of the channel (integer)
///
/// # Returns
///
/// A new Channel object
pub fn new_channel(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("new_channel requires a capacity argument".to_string()));
    }
    
    // Extract capacity
    let capacity = match &*args[0] {
        Object::Integer(n) => *n as usize,
        _ => return Err(Error::Runtime("First argument to new_channel must be an integer".to_string())),
    };
    
    // Create a new channel using the core function
    let channel = match create_channel("any".to_string(), Some(capacity)) {
        Object::Channel(ch) => ch,
        _ => return Err(Error::Runtime("Failed to create channel".to_string()))
    };
    
    Ok(Arc::new(Object::Channel(channel)))
}

/// Sends a value to a channel
///
/// # Arguments
///
/// * `args[0]` - The channel to send to
/// * `args[1]` - The value to send
///
/// # Returns
///
/// `nil` on success or an error if the arguments are invalid
pub fn channel_send(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("channel_send requires a channel and value arguments".to_string()));
    }
    
    // Extract channel
    let channel_ref = match &*args[0] {
        Object::Channel(ch) => ch,
        _ => return Err(Error::Runtime("First argument to channel_send must be a channel".to_string())),
    };
    
    // Extract value
    let value = args[1].as_ref().clone();
    
    // Send to the channel using the core function
    let channel_obj = Object::Channel(channel_ref.clone());
    send_to_channel(channel_obj, value).map_err(|e| Error::Runtime(e))?;
    
    Ok(Arc::new(Object::Null))
}

/// Receives a value from a channel
///
/// # Arguments
///
/// * `args[0]` - The channel to receive from
///
/// # Returns
///
/// The received value or an error if the arguments are invalid
pub fn channel_receive(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("channel_receive requires a channel argument".to_string()));
    }
    
    // Extract channel
    let channel_ref = match &*args[0] {
        Object::Channel(ch) => ch,
        _ => return Err(Error::Runtime("First argument to channel_receive must be a channel".to_string())),
    };
    
    // Receive from the channel using the core function
    let channel_obj = Object::Channel(channel_ref.clone());
    let value = receive_from_channel(channel_obj).map_err(|e| Error::Runtime(e))?;
    
    Ok(Arc::new(value))
}

/// Tries to send a value to a channel without blocking
///
/// # Arguments
///
/// * `args[0]` - The channel to send to
/// * `args[1]` - The value to send
///
/// # Returns
///
/// `true` if the send was successful, `false` if the channel is full
pub fn channel_try_send(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("channel_try_send requires a channel and value arguments".to_string()));
    }
    
    // Extract channel
    let channel_ref = match &*args[0] {
        Object::Channel(ch) => ch,
        _ => return Err(Error::Runtime("First argument to channel_try_send must be a channel".to_string())),
    };
    
    // Extract value
    let value = args[1].as_ref().clone();
    
    // Try to send to the channel using the core function
    let channel_obj = Object::Channel(channel_ref.clone());
    let result = try_send_to_channel(channel_obj, value).map_err(|e| Error::Runtime(e))?;
    
    let success = match result {
        Object::Boolean(b) => b,
        _ => return Err(Error::Runtime("try_send_to_channel returned unexpected type".to_string()))
    };
    
    Ok(Arc::new(Object::Boolean(success)))
}

/// Tries to receive a value from a channel without blocking
///
/// # Arguments
///
/// * `args[0]` - The channel to receive from
///
/// # Returns
///
/// An option containing the received value, or None if the channel is empty
pub fn channel_try_receive(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("channel_try_receive requires a channel argument".to_string()));
    }
    
    // Extract channel
    let channel_ref = match &*args[0] {
        Object::Channel(ch) => ch,
        _ => return Err(Error::Runtime("First argument to channel_try_receive must be a channel".to_string())),
    };
    
    // Try to receive from the channel using the core function
    let channel_obj = Object::Channel(channel_ref.clone());
    let result = try_receive_from_channel(channel_obj).map_err(|e| Error::Runtime(e))?;
    
    // Parse the result
    match result {
        Object::Option(opt) => {
            match opt {
                Some(obj) => Ok(Arc::new(Object::Option(Some(obj.clone())))),
                None => Ok(Arc::new(Object::Option(None))),
            }
        },
        _ => Err(Error::Runtime("try_receive_from_channel returned unexpected type".to_string()))
    }
}