//! Root set management for CURSED garbage collection
//!
//! Manages the root set of objects that are reachable from:
//! - Stack frames and local variables
//! - Global variables and static data
//! - Thread-local storage
//! - External references (JIT, FFI, etc.)

use crate::error::CursedError;
use crate::memory::{Traceable, Visitor};
use std::sync::{Arc, RwLock, Mutex};
use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;
use std::thread::ThreadId;
use std::time::{Duration, Instant};

/// Root set manager for garbage collection
pub struct RootSetManager {
    /// Global roots (static variables, etc.)
    global_roots: RwLock<HashSet<RootRef>>,
    /// Thread-local roots per thread
    thread_roots: RwLock<HashMap<ThreadId, ThreadRoots>>,
    /// Stack frame roots
    stack_roots: RwLock<HashMap<ThreadId, Vec<StackFrame>>>,
    /// External roots (JIT, FFI, etc.)
    external_roots: RwLock<HashSet<ExternalRoot>>,
    /// Root set statistics
    stats: Mutex<RootSetStats>,
    /// Configuration
    config: RwLock<RootSetConfig>,
}

/// Root reference to an object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootRef {
    /// Pointer to the object
    pub ptr: NonNull<u8>,
    /// Size of the object
    pub size: usize,
    /// Type information
    pub type_info: String,
    /// Creation timestamp
    pub created_at: Instant,
}

/// Thread-local roots
#[derive(Debug, Clone)]
struct ThreadRoots {
    thread_id: ThreadId,
    local_roots: HashSet<RootRef>,
    last_updated: Instant,
}

/// Stack frame representation
#[derive(Debug, Clone)]
struct StackFrame {
    /// Frame pointer
    frame_ptr: NonNull<u8>,
    /// Local variables in this frame
    locals: Vec<RootRef>,
    /// Function name for debugging
    function_name: String,
    /// Frame creation time
    created_at: Instant,
}

/// External root from JIT, FFI, or other systems
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ExternalRoot {
    ptr: NonNull<u8>,
    source: ExternalSource,
    metadata: String,
    created_at: Instant,
}

/// Source of external root
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ExternalSource {
    JIT,
    FFI,
    Runtime,
    Compiler,
    Other(String),
}

/// Root set configuration
#[derive(Debug, Clone)]
pub struct RootSetConfig {
    /// Maximum number of roots per thread
    pub max_roots_per_thread: usize,
    /// Maximum stack depth to track
    pub max_stack_depth: usize,
    /// Enable automatic root cleanup
    pub auto_cleanup: bool,
    /// Root cleanup interval
    pub cleanup_interval: Duration,
    /// Enable root validation
    pub validate_roots: bool,
}

impl Default for RootSetConfig {
    fn default() -> Self {
        Self {
            max_roots_per_thread: 10000,
            max_stack_depth: 1000,
            auto_cleanup: true,
            cleanup_interval: Duration::from_secs(10),
            validate_roots: true,
        }
    }
}

/// Root set statistics
#[derive(Debug, Clone, Default)]
pub struct RootSetStats {
    pub total_roots: usize,
    pub global_roots: usize,
    pub thread_roots: usize,
    pub stack_roots: usize,
    pub external_roots: usize,
    pub root_additions: u64,
    pub root_removals: u64,
    pub cleanup_runs: u64,
    pub validation_failures: u64,
    pub last_cleanup: Option<Instant>,
}

impl RootSetManager {
    /// Create new root set manager
    pub fn new() -> Self {
        Self {
            global_roots: RwLock::new(HashSet::new()),
            thread_roots: RwLock::new(HashMap::new()),
            stack_roots: RwLock::new(HashMap::new()),
            external_roots: RwLock::new(HashSet::new()),
            stats: Mutex::new(RootSetStats::default()),
            config: RwLock::new(RootSetConfig::default()),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: RootSetConfig) -> Self {
        Self {
            global_roots: RwLock::new(HashSet::new()),
            thread_roots: RwLock::new(HashMap::new()),
            stack_roots: RwLock::new(HashMap::new()),
            external_roots: RwLock::new(HashSet::new()),
            stats: Mutex::new(RootSetStats::default()),
            config: RwLock::new(config),
        }
    }

    /// Add global root
    pub fn add_global_root(&self, ptr: NonNull<u8>, size: usize, type_info: String) -> Result<(), CursedError> {
        let root = RootRef {
            ptr,
            size,
            type_info,
            created_at: Instant::now(),
        };

        let mut global_roots = self.global_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire global roots write lock".to_string(),
            context: Default::default(),
        })?;

        global_roots.insert(root);

        // Update statistics
        self.update_stats(|stats| {
            stats.global_roots = global_roots.len();
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
            stats.root_additions += 1;
        })?;

        Ok(())
    }

    /// Remove global root
    pub fn remove_global_root(&self, ptr: NonNull<u8>) -> Result<bool, CursedError> {
        let mut global_roots = self.global_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire global roots write lock".to_string(),
            context: Default::default(),
        })?;

        let removed = global_roots.retain(|root| root.ptr != ptr);
        let was_removed = global_roots.len() != global_roots.len();

        // Update statistics
        self.update_stats(|stats| {
            stats.global_roots = global_roots.len();
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
            if was_removed {
                stats.root_removals += 1;
            }
        })?;

        Ok(was_removed)
    }

    /// Add thread-local root
    pub fn add_thread_root(&self, thread_id: ThreadId, ptr: NonNull<u8>, size: usize, type_info: String) -> Result<(), CursedError> {
        let root = RootRef {
            ptr,
            size,
            type_info,
            created_at: Instant::now(),
        };

        let mut thread_roots = self.thread_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire thread roots write lock".to_string(),
            context: Default::default(),
        })?;

        let thread_data = thread_roots.entry(thread_id).or_insert_with(|| ThreadRoots {
            thread_id,
            local_roots: HashSet::new(),
            last_updated: Instant::now(),
        });

        // Check limits
        let config = self.config.read().unwrap();
        if thread_data.local_roots.len() >= config.max_roots_per_thread {
            return Err(CursedError::RuntimeError {
                message: format!("Thread root limit exceeded: {}", config.max_roots_per_thread),
                context: Default::default(),
            });
        }

        thread_data.local_roots.insert(root);
        thread_data.last_updated = Instant::now();

        // Update statistics
        let total_thread_roots: usize = thread_roots.values().map(|t| t.local_roots.len()).sum();
        self.update_stats(|stats| {
            stats.thread_roots = total_thread_roots;
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
            stats.root_additions += 1;
        })?;

        Ok(())
    }

    /// Remove thread-local root
    pub fn remove_thread_root(&self, thread_id: ThreadId, ptr: NonNull<u8>) -> Result<bool, CursedError> {
        let mut thread_roots = self.thread_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire thread roots write lock".to_string(),
            context: Default::default(),
        })?;

        let mut was_removed = false;
        if let Some(thread_data) = thread_roots.get_mut(&thread_id) {
            let original_len = thread_data.local_roots.len();
            thread_data.local_roots.retain(|root| root.ptr != ptr);
            was_removed = thread_data.local_roots.len() != original_len;
            thread_data.last_updated = Instant::now();
        }

        // Update statistics
        let total_thread_roots: usize = thread_roots.values().map(|t| t.local_roots.len()).sum();
        self.update_stats(|stats| {
            stats.thread_roots = total_thread_roots;
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
            if was_removed {
                stats.root_removals += 1;
            }
        })?;

        Ok(was_removed)
    }

    /// Add stack frame
    pub fn push_stack_frame(&self, thread_id: ThreadId, frame_ptr: NonNull<u8>, function_name: String) -> Result<(), CursedError> {
        let frame = StackFrame {
            frame_ptr,
            locals: Vec::new(),
            function_name,
            created_at: Instant::now(),
        };

        let mut stack_roots = self.stack_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire stack roots write lock".to_string(),
            context: Default::default(),
        })?;

        let stack = stack_roots.entry(thread_id).or_insert_with(Vec::new);
        
        // Check stack depth limit
        let config = self.config.read().unwrap();
        if stack.len() >= config.max_stack_depth {
            return Err(CursedError::RuntimeError {
                message: format!("Stack depth limit exceeded: {}", config.max_stack_depth),
                context: Default::default(),
            });
        }

        stack.push(frame);

        // Update statistics
        let total_stack_frames: usize = stack_roots.values().map(|s| s.len()).sum();
        self.update_stats(|stats| {
            stats.stack_roots = total_stack_frames;
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
        })?;

        Ok(())
    }

    /// Remove stack frame
    pub fn pop_stack_frame(&self, thread_id: ThreadId) -> Result<Option<StackFrame>, CursedError> {
        let mut stack_roots = self.stack_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire stack roots write lock".to_string(),
            context: Default::default(),
        })?;

        let frame = if let Some(stack) = stack_roots.get_mut(&thread_id) {
            stack.pop()
        } else {
            None
        };

        // Update statistics
        let total_stack_frames: usize = stack_roots.values().map(|s| s.len()).sum();
        self.update_stats(|stats| {
            stats.stack_roots = total_stack_frames;
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
        })?;

        Ok(frame)
    }

    /// Add local variable to current stack frame
    pub fn add_stack_local(&self, thread_id: ThreadId, ptr: NonNull<u8>, size: usize, type_info: String) -> Result<(), CursedError> {
        let root = RootRef {
            ptr,
            size,
            type_info,
            created_at: Instant::now(),
        };

        let mut stack_roots = self.stack_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire stack roots write lock".to_string(),
            context: Default::default(),
        })?;

        if let Some(stack) = stack_roots.get_mut(&thread_id) {
            if let Some(frame) = stack.last_mut() {
                frame.locals.push(root);
            }
        }

        Ok(())
    }

    /// Add external root
    pub fn add_external_root(&self, ptr: NonNull<u8>, source: &str, metadata: String) -> Result<(), CursedError> {
        let external_source = match source {
            "jit" => ExternalSource::JIT,
            "ffi" => ExternalSource::FFI,
            "runtime" => ExternalSource::Runtime,
            "compiler" => ExternalSource::Compiler,
            other => ExternalSource::Other(other.to_string()),
        };

        let root = ExternalRoot {
            ptr,
            source: external_source,
            metadata,
            created_at: Instant::now(),
        };

        let mut external_roots = self.external_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire external roots write lock".to_string(),
            context: Default::default(),
        })?;

        external_roots.insert(root);

        // Update statistics
        self.update_stats(|stats| {
            stats.external_roots = external_roots.len();
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
            stats.root_additions += 1;
        })?;

        Ok(())
    }

    /// Remove external root
    pub fn remove_external_root(&self, ptr: NonNull<u8>) -> Result<bool, CursedError> {
        let mut external_roots = self.external_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire external roots write lock".to_string(),
            context: Default::default(),
        })?;

        let original_len = external_roots.len();
        external_roots.retain(|root| root.ptr != ptr);
        let was_removed = external_roots.len() != original_len;

        // Update statistics
        self.update_stats(|stats| {
            stats.external_roots = external_roots.len();
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
            if was_removed {
                stats.root_removals += 1;
            }
        })?;

        Ok(was_removed)
    }

    /// Visit all roots with a visitor function
    pub fn visit_roots<F>(&self, mut visitor: F) -> Result<(), CursedError> 
    where
        F: FnMut(NonNull<u8>, usize, &str) -> Result<(), CursedError>,
    {
        // Visit global roots
        let global_roots = self.global_roots.read().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire global roots read lock".to_string(),
            context: Default::default(),
        })?;

        for root in global_roots.iter() {
            visitor(root.ptr, root.size, &root.type_info)?;
        }

        // Visit thread-local roots
        let thread_roots = self.thread_roots.read().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire thread roots read lock".to_string(),
            context: Default::default(),
        })?;

        for thread_data in thread_roots.values() {
            for root in &thread_data.local_roots {
                visitor(root.ptr, root.size, &root.type_info)?;
            }
        }

        // Visit stack roots
        let stack_roots = self.stack_roots.read().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire stack roots read lock".to_string(),
            context: Default::default(),
        })?;

        for stack in stack_roots.values() {
            for frame in stack {
                for local in &frame.locals {
                    visitor(local.ptr, local.size, &local.type_info)?;
                }
            }
        }

        // Visit external roots
        let external_roots = self.external_roots.read().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire external roots read lock".to_string(),
            context: Default::default(),
        })?;

        for root in external_roots.iter() {
            visitor(root.ptr, 0, &root.metadata)?; // External roots don't have size info
        }

        Ok(())
    }

    /// Get root set statistics
    pub fn get_stats(&self) -> Result<RootSetStats, CursedError> {
        let stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire stats lock".to_string(),
            context: Default::default(),
        })?;

        Ok(stats.clone())
    }

    /// Cleanup stale roots
    pub fn cleanup_stale_roots(&self) -> Result<usize, CursedError> {
        let config = self.config.read().unwrap();
        let cleanup_threshold = Instant::now() - config.cleanup_interval;
        let mut cleaned_count = 0;

        // Clean up thread roots for dead threads
        let mut thread_roots = self.thread_roots.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire thread roots write lock".to_string(),
            context: Default::default(),
        })?;

        thread_roots.retain(|_, thread_data| {
            if thread_data.last_updated < cleanup_threshold {
                cleaned_count += thread_data.local_roots.len();
                false
            } else {
                true
            }
        });

        // Update statistics
        self.update_stats(|stats| {
            stats.cleanup_runs += 1;
            stats.last_cleanup = Some(Instant::now());
            stats.thread_roots = thread_roots.values().map(|t| t.local_roots.len()).sum();
            stats.total_roots = stats.global_roots + stats.thread_roots + stats.stack_roots + stats.external_roots;
        })?;

        Ok(cleaned_count)
    }

    /// Update configuration
    pub fn update_config(&self, new_config: RootSetConfig) -> Result<(), CursedError> {
        let mut config = self.config.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire config write lock".to_string(),
            context: Default::default(),
        })?;

        *config = new_config;
        Ok(())
    }

    /// Helper function to update statistics
    fn update_stats<F>(&self, update_fn: F) -> Result<(), CursedError>
    where
        F: FnOnce(&mut RootSetStats),
    {
        let mut stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire stats lock".to_string(),
            context: Default::default(),
        })?;

        update_fn(&mut *stats);
        Ok(())
    }
}

/// Global root set manager instance
static GLOBAL_ROOT_SET: std::sync::OnceLock<Arc<RootSetManager>> = std::sync::OnceLock::new();

/// Get global root set manager
pub fn get_global_root_set() -> Arc<RootSetManager> {
    GLOBAL_ROOT_SET.get_or_init(|| {
        Arc::new(RootSetManager::new())
    }).clone()
}

/// Initialize root set management system
pub fn initialize_root_set() -> Result<String, CursedError> {
    let root_set = get_global_root_set();
    let config = root_set.config.read().unwrap();
    
    Ok(format!(
        "Root set manager initialized: max {} roots per thread, stack depth {}",
        config.max_roots_per_thread,
        config.max_stack_depth
    ))
}

/// Legacy compatibility structure
pub struct MinimalImplementation {
    root_set: Arc<RootSetManager>,
}

impl MinimalImplementation {
    pub fn new() -> Self {
        Self {
            root_set: get_global_root_set(),
        }
    }
    
    pub fn get_root_set(&self) -> Arc<RootSetManager> {
        self.root_set.clone()
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    initialize_root_set()
}
