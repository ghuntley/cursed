/// Thread-local storage for CURSED
/// 
/// This module provides thread-local storage capabilities allowing
/// data to be stored per-thread with automatic cleanup.

// use crate::stdlib::sync::error::{SyncError, SyncResult, thread_local_error};
use crate::error::CursedError;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread::{self, ThreadId};
use std::any::{Any, TypeId};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::{RefCell, Cell};
use std::sync::OnceLock;

// Global thread-local storage registry
static NEXT_KEY_ID: AtomicUsize = AtomicUsize::new(0);
static mut GLOBAL_STORAGE: Option<Arc<Mutex<ThreadLocalRegistry>>> = None;
static INIT_ONCE: std::sync::Once = std::sync::Once::new();

//==============================================================================
// Thread Local Storage Registry
//==============================================================================

/// Registry for managing thread-local storage across all threads
struct ThreadLocalRegistry {
    /// Storage for each thread
    thread_storage: HashMap<ThreadId, HashMap<usize, Box<dyn Any + Send>>>,
    /// Cleanup functions for each key
    cleanup_functions: HashMap<usize, Box<dyn Fn(&mut dyn Any) + Send + Sync>>,
    /// Key metadata
    key_metadata: HashMap<usize, KeyMetadata>,
}

#[derive(Debug, Clone)]
struct KeyMetadata {
    name: Option<String>,
    type_name: String,
    created_at: std::time::Instant,
}

impl ThreadLocalRegistry {
    fn new() -> Self {
        Self {
            thread_storage: HashMap::new(),
            cleanup_functions: HashMap::new(),
            key_metadata: HashMap::new(),
        }
    }

    fn get_value<T: 'static>(&mut self, thread_id: ThreadId, key: usize) -> Option<&T> {
        self.thread_storage
            .get(&thread_id)?
            .get(&key)?
            .downcast_ref::<T>()
    }

    fn set_value<T: 'static + Send>(&mut self, thread_id: ThreadId, key: usize, value: T) {
        self.thread_storage
            .entry(thread_id)
            .or_insert_with(HashMap::new)
            .insert(key, Box::new(value));
    }

    fn remove_value(&mut self, thread_id: ThreadId, key: usize) -> Option<Box<dyn Any + Send>> {
        self.thread_storage
            .get_mut(&thread_id)?
            .remove(&key)
    }

    fn cleanup_thread(&mut self, thread_id: ThreadId) {
        if let Some(thread_data) = self.thread_storage.remove(&thread_id) {
            for (key, mut value) in thread_data {
                if let Some(cleanup_fn) = self.cleanup_functions.get(&key) {
                    cleanup_fn(value.as_mut());
                }
            }
        }
    }

    fn register_key<T: 'static>(&mut self, key: usize, name: Option<String>) {
        let metadata = KeyMetadata {
            name,
            type_name: std::any::type_name::<T>().to_string(),
            created_at: std::time::Instant::now(),
        };
        self.key_metadata.insert(key, metadata);
    }

    fn register_cleanup<T: 'static, F>(&mut self, key: usize, cleanup: F)
    where
        F: Fn(&mut T) + Send + Sync + 'static,
    {
        let boxed_cleanup: Box<dyn Fn(&mut dyn Any) + Send + Sync> = Box::new(move |any_value| {
            if let Some(typed_value) = any_value.downcast_mut::<T>() {
                cleanup(typed_value);
            }
        });
        self.cleanup_functions.insert(key, boxed_cleanup);
    }

    fn get_statistics(&self) -> ThreadLocalStatistics {
        let mut total_values = 0;
        let mut max_values_per_thread = 0;

        for thread_data in self.thread_storage.values() {
            total_values += thread_data.len();
            max_values_per_thread = max_values_per_thread.max(thread_data.len());
        }

        ThreadLocalStatistics {
            active_threads: self.thread_storage.len(),
            total_keys: self.key_metadata.len(),
            total_values,
            max_values_per_thread,
            memory_usage_estimate: total_values * 64, // Rough estimate
        }
    }
}

/// Statistics about thread-local storage usage
#[derive(Debug, Clone)]
pub struct ThreadLocalStatistics {
    pub active_threads: usize,
    pub total_keys: usize,
    pub total_values: usize,
    pub max_values_per_thread: usize,
    pub memory_usage_estimate: usize,
}

fn get_global_registry() -> &'static Arc<Mutex<ThreadLocalRegistry>> {
    INIT_ONCE.call_once(|| {
        unsafe {
            GLOBAL_STORAGE = Some(Arc::new(Mutex::new(ThreadLocalRegistry::new())));
        }
    });

    unsafe { GLOBAL_STORAGE.as_ref().unwrap() }
}

//==============================================================================
// Thread Local Key
//==============================================================================

/// A key for thread-local storage
pub struct ThreadLocalKey<T> {
    key: usize,
    name: Option<String>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ThreadLocalKey<T>
where
    T: 'static + Send,
{
    /// Create a new thread-local key
    pub fn new() -> Self {
        let key = NEXT_KEY_ID.fetch_add(1, Ordering::Relaxed);
        
        let mut registry = get_global_registry().lock().unwrap();
        registry.register_key::<T>(key, None);
        
        Self {
            key,
            name: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new named thread-local key
    pub fn named(name: &str) -> Self {
        let key = NEXT_KEY_ID.fetch_add(1, Ordering::Relaxed);
        let name_string = name.to_string();
        
        let mut registry = get_global_registry().lock().unwrap();
        registry.register_key::<T>(key, Some(name_string.clone()));
        
        Self {
            key,
            name: Some(name_string),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the value for the current thread
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        let thread_id = thread::current().id();
        let mut registry = get_global_registry().lock().unwrap();
        registry.get_value::<T>(thread_id, self.key).cloned()
    }

    /// Set the value for the current thread
    pub fn set(&self, value: T) -> SyncResult<()> {
        let thread_id = thread::current().id();
        let mut registry = get_global_registry().lock().unwrap();
        registry.set_value(thread_id, self.key, value);
        Ok(())
    }

    /// Remove the value for the current thread
    pub fn remove(&self) -> SyncResult<Option<T>> {
        let thread_id = thread::current().id();
        let mut registry = get_global_registry().lock().unwrap();
        
        if let Some(boxed_value) = registry.remove_value(thread_id, self.key) {
            if let Ok(value) = boxed_value.downcast::<T>() {
                return Ok(Some(*value));
            }
        }
        
        Ok(None)
    }

    /// Execute a function with access to the thread-local value
    pub fn with<F, R>(&self, f: F) -> SyncResult<Option<R>>
    where
        F: FnOnce(&T) -> R,
        T: Clone,
    {
        if let Some(value) = self.get() {
            Ok(Some(f(&value)))
        } else {
            Ok(None)
        }
    }

    /// Execute a function with mutable access to the thread-local value
    pub fn with_mut<F, R>(&self, f: F) -> SyncResult<Option<R>>
    where
        F: FnOnce(&mut T) -> R,
        T: Clone,
    {
        if let Some(mut value) = self.get() {
            let result = f(&mut value);
            self.set(value)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Get or insert a default value
    pub fn get_or_insert<F>(&self, default: F) -> SyncResult<T>
    where
        F: FnOnce() -> T,
        T: Clone,
    {
        if let Some(value) = self.get() {
            Ok(value)
        } else {
            let value = default();
            self.set(value.clone())?;
            Ok(value)
        }
    }

    /// Register a cleanup function for this key
    pub fn register_cleanup<F>(&self, cleanup: F) -> SyncResult<()>
    where
        F: Fn(&mut T) + Send + Sync + 'static,
    {
        let mut registry = get_global_registry().lock().unwrap();
        registry.register_cleanup(self.key, cleanup);
        Ok(())
    }

    /// Get the key ID
    pub fn key_id(&self) -> usize {
        self.key
    }

    /// Get the key name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<T> Clone for ThreadLocalKey<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            name: self.name.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

//==============================================================================
// Thread Local Storage
//==============================================================================

/// A high-level thread-local storage container
pub struct ThreadLocal<T> {
    key: ThreadLocalKey<T>,
    initializer: Option<Box<dyn Fn() -> T + Send + Sync>>,
}

impl<T> ThreadLocal<T>
where
    T: 'static + Send + Clone,
{
    /// Create a new thread-local storage
    pub fn new() -> Self {
        Self {
            key: ThreadLocalKey::new(),
            initializer: None,
        }
    }

    /// Create a new thread-local storage with an initializer
    pub fn with_initializer<F>(initializer: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            key: ThreadLocalKey::new(),
            initializer: Some(Box::new(initializer)),
        }
    }

    /// Create a new named thread-local storage
    pub fn named(name: &str) -> Self {
        Self {
            key: ThreadLocalKey::named(name),
            initializer: None,
        }
    }

    /// Get the value, initializing if necessary
    pub fn get(&self) -> SyncResult<T> {
        if let Some(value) = self.key.get() {
            Ok(value)
        } else if let Some(ref initializer) = self.initializer {
            let value = initializer();
            self.key.set(value.clone())?;
            Ok(value)
        } else {
            Err(thread_local_error(&format!("key_{}", self.key.key_id()), "No value set and no initializer provided"))
        }
    }

    /// Set the value
    pub fn set(&self, value: T) -> SyncResult<()> {
        self.key.set(value)
    }

    /// Execute a function with access to the value
    pub fn with<F, R>(&self, f: F) -> SyncResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        let value = self.get()?;
        Ok(f(&value))
    }

    /// Execute a function with mutable access to the value
    pub fn with_mut<F, R>(&self, f: F) -> SyncResult<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut value = self.get()?;
        let result = f(&mut value);
        self.set(value)?;
        Ok(result)
    }

    /// Reset the value to the initial state
    pub fn reset(&self) -> SyncResult<()> {
        self.key.remove()?;
        Ok(())
    }

    /// Check if a value is set for the current thread
    pub fn is_set(&self) -> bool {
        self.key.get().is_some()
    }

    /// Get the underlying key
    pub fn key(&self) -> &ThreadLocalKey<T> {
        &self.key
    }
}

impl<T> Default for ThreadLocal<T>
where
    T: 'static + Send + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

//==============================================================================
// High-Level API Functions
//==============================================================================

/// Type alias for thread-local storage key
pub type TlsKey<T> = ThreadLocalKey<T>;

/// Create a new thread-local storage key
pub fn create_thread_local_key<T>() -> ThreadLocalKey<T>
where
    T: 'static + Send,
{
    ThreadLocalKey::new()
}

/// Get a value from thread-local storage
pub fn thread_local_get<T>(key: &ThreadLocalKey<T>) -> Option<T>
where
    T: 'static + Send + Clone,
{
    key.get()
}

/// Set a value in thread-local storage
pub fn thread_local_set<T>(key: &ThreadLocalKey<T>, value: T) -> SyncResult<()>
where
    T: 'static + Send,
{
    key.set(value)
}

/// Remove a value from thread-local storage
pub fn thread_local_remove<T>(key: &ThreadLocalKey<T>) -> SyncResult<Option<T>>
where
    T: 'static + Send,
{
    key.remove()
}

/// Execute a function with thread-local value
pub fn with_thread_local<T, F, R>(key: &ThreadLocalKey<T>, f: F) -> SyncResult<Option<R>>
where
    T: 'static + Send + Clone,
    F: FnOnce(&T) -> R,
{
    key.with(f)
}

/// Cleanup thread-local storage for the current thread
pub fn cleanup_current_thread() -> SyncResult<()> {
    let thread_id = thread::current().id();
    let mut registry = get_global_registry().lock().unwrap();
    registry.cleanup_thread(thread_id);
    Ok(())
}

/// Cleanup all thread-local storage
pub fn cleanup_thread_local_storage() -> SyncResult<()> {
    let mut registry = get_global_registry().lock().unwrap();
    let thread_ids: Vec<ThreadId> = registry.thread_storage.keys().cloned().collect();
    
    for thread_id in thread_ids {
        registry.cleanup_thread(thread_id);
    }
    
    Ok(())
}

/// Get thread-local storage statistics
pub fn get_thread_local_statistics() -> SyncResult<ThreadLocalStatistics> {
    let registry = get_global_registry().lock().unwrap();
    Ok(registry.get_statistics())
}

//==============================================================================
// Cell-based Thread Local Storage
//==============================================================================

/// Thread-local storage using built-in thread_local! macro for interior mutability
pub struct ThreadLocalCell<T> {
    key: ThreadLocalKey<RefCell<Option<T>>>,
    name: Option<String>,
}

impl<T> ThreadLocalCell<T>
where
    T: 'static + Send + Clone,
{
    /// Create a new thread-local cell
    pub fn new() -> Self {
        Self {
            key: ThreadLocalKey::new(),
            name: None,
        }
    }

    /// Create a new named thread-local cell
    pub fn named(name: &str) -> Self {
        Self {
            key: ThreadLocalKey::named(name),
            name: Some(name.to_string()),
        }
    }

    /// Get the value for the current thread
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        if let Some(cell) = self.key.get() {
            cell.borrow().clone()
        } else {
            // Initialize with empty RefCell
            let cell = RefCell::new(None);
            let _ = self.key.set(cell);
            None
        }
    }

    /// Set the value for the current thread
    pub fn set(&self, value: T) -> SyncResult<()>
    where
        T: Clone,
    {
        if let Some(cell) = self.key.get() {
            *cell.borrow_mut() = Some(value);
            Ok(())
        } else {
            // Initialize with the value
            let cell = RefCell::new(Some(value));
            self.key.set(cell)
        }
    }

    /// Execute a function with access to the value
    pub fn with<F, R>(&self, f: F) -> SyncResult<Option<R>>
    where
        F: FnOnce(&T) -> R,
        T: Clone,
    {
        if let Some(value) = self.get() {
            Ok(Some(f(&value)))
        } else {
            Ok(None)
        }
    }

    /// Execute a function with mutable access to the value
    pub fn with_mut<F, R>(&self, f: F) -> SyncResult<Option<R>>
    where
        F: FnOnce(&mut T) -> R,
        T: Clone,
    {
        if let Some(cell) = self.key.get() {
            if let Some(ref mut value) = *cell.borrow_mut() {
                Ok(Some(f(value)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get or insert a default value
    pub fn get_or_insert_with<F>(&self, default: F) -> SyncResult<T>
    where
        F: FnOnce() -> T,
        T: Clone,
    {
        if let Some(value) = self.get() {
            Ok(value)
        } else {
            let value = default();
            self.set(value.clone())?;
            Ok(value)
        }
    }

    /// Remove the value
    pub fn remove(&self) -> SyncResult<Option<T>>
    where
        T: Clone,
    {
        if let Some(cell) = self.key.get() {
            Ok(cell.borrow_mut().take())
        } else {
            Ok(None)
        }
    }

    /// Check if a value is set
    pub fn is_set(&self) -> bool {
        self.get().is_some()
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<T> Default for ThreadLocalCell<T>
where
    T: 'static + Send + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-local storage using Cell for Copy types
pub struct ThreadLocalValue<T> {
    key: ThreadLocalKey<Cell<Option<T>>>,
    name: Option<String>,
}

impl<T> ThreadLocalValue<T>
where
    T: Copy + 'static + Send,
{
    /// Create a new thread-local value
    pub fn new() -> Self {
        Self {
            key: ThreadLocalKey::new(),
            name: None,
        }
    }

    /// Create a new named thread-local value
    pub fn named(name: &str) -> Self {
        Self {
            key: ThreadLocalKey::named(name),
            name: Some(name.to_string()),
        }
    }

    /// Get the value
    pub fn get(&self) -> Option<T> {
        if let Some(cell) = self.key.get() {
            cell.get()
        } else {
            // Initialize with empty Cell
            let cell = Cell::new(None);
            let _ = self.key.set(cell);
            None
        }
    }

    /// Set the value
    pub fn set(&self, value: T) -> SyncResult<()> {
        if let Some(cell) = self.key.get() {
            cell.set(Some(value));
            Ok(())
        } else {
            // Initialize with the value
            let cell = Cell::new(Some(value));
            self.key.set(cell)
        }
    }

    /// Get or insert a default value
    pub fn get_or_insert(&self, default: T) -> SyncResult<T> {
        if let Some(value) = self.get() {
            Ok(value)
        } else {
            self.set(default)?;
            Ok(default)
        }
    }

    /// Remove the value
    pub fn remove(&self) -> SyncResult<Option<T>> {
        if let Some(cell) = self.key.get() {
            let value = cell.get();
            cell.set(None);
            Ok(value)
        } else {
            Ok(None)
        }
    }

    /// Check if a value is set
    pub fn is_set(&self) -> bool {
        self.get().is_some()
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<T> Default for ThreadLocalValue<T>
where
    T: Copy + 'static + Send,
{
    fn default() -> Self {
        Self::new()
    }
}

