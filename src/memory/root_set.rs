/// Root Set Management for Garbage Collection
/// 
/// This module manages the root set - the collection of objects that serve as
/// starting points for garbage collection reachability analysis. Root set
/// management is critical for memory safety because:
/// 
/// 1. **Reachability Analysis**: Roots define which objects are always reachable
/// 2. **Collection Safety**: Prevents collection of objects still in use
/// 3. **Performance**: Efficient root enumeration improves GC performance
/// 4. **Thread Safety**: Supports concurrent root set modifications
/// 5. **Debugging Support**: Provides visibility into object relationships
/// 
/// The root set includes stack variables, global variables, static objects,
/// and explicitly pinned objects that should never be collected.

use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};
use std::thread::{self, ThreadId};
use tracing::{instrument, debug, info, warn};

use crate::memory::object_id::ObjectId;
use crate::error_types::Error;

/// Types of roots in the garbage collection system
/// 
/// Different root types have different lifetime characteristics and
/// require different management strategies.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RootType {
    /// Global/static variables that exist for program lifetime
    Global,
    /// Stack variables for the current thread
    Stack,
    /// Pinned objects that should never be collected
    Pinned,
    /// JIT-compiled code references
    JitCode,
    /// Goroutine local roots (for concurrent execution)
    Goroutine(u64),
    /// External references from C code or other languages
    External,
}

impl std::fmt::Display for RootType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootType::Global => write!(f, "Global"),
            RootType::Stack => write!(f, "Stack"),
            RootType::Pinned => write!(f, "Pinned"),
            RootType::JitCode => write!(f, "JitCode"),
            RootType::Goroutine(id) => write!(f, "Goroutine({})", id),
            RootType::External => write!(f, "External"),
        }
    }
}

/// Information about a root object
#[derive(Debug, Clone)]
pub struct RootInfo {
    /// The object ID being rooted
    pub object_id: ObjectId,
    /// Type of root
    pub root_type: RootType,
    /// Optional description for debugging
    pub description: Option<String>,
    /// Thread that created this root
    pub thread_id: ThreadId,
    /// Timestamp when root was created
    pub created_at: std::time::SystemTime,
}

impl RootInfo {
    /// Create new root info
    pub fn new(object_id: ObjectId, root_type: RootType, description: Option<String>) -> Self {
        Self {
            object_id,
            root_type,
            description,
            thread_id: thread::current().id(),
            created_at: std::time::SystemTime::now(),
        }
    }
}

/// Thread-local root set for stack variables
/// 
/// Each thread maintains its own set of stack roots that are automatically
/// managed during function calls and returns.
pub struct ThreadRootSet {
    /// Thread ID this root set belongs to
    thread_id: ThreadId,
    /// Stack roots for this thread
    stack_roots: HashSet<ObjectId>,
    /// Pinned roots for this thread
    pinned_roots: HashSet<ObjectId>,
    /// Root information for debugging
    root_info: HashMap<ObjectId, RootInfo>,
}

impl ThreadRootSet {
    /// Create a new thread root set
    pub fn new() -> Self {
        let thread_id = thread::current().id();
        debug!("Creating thread root set for {:?}", thread_id);
        
        Self {
            thread_id,
            stack_roots: HashSet::new(),
            pinned_roots: HashSet::new(),
            root_info: HashMap::new(),
        }
    }
    
    /// Add a stack root
    #[instrument(skip(self))]
    pub fn add_stack_root(&mut self, object_id: ObjectId, description: Option<String>) {
        if self.stack_roots.insert(object_id) {
            let root_info = RootInfo::new(object_id, RootType::Stack, description);
            self.root_info.insert(object_id, root_info);
            debug!("Added stack root {} for thread {:?}", object_id, self.thread_id);
        }
    }
    
    /// Remove a stack root
    #[instrument(skip(self))]
    pub fn remove_stack_root(&mut self, object_id: ObjectId) -> bool {
        if self.stack_roots.remove(&object_id) {
            self.root_info.remove(&object_id);
            debug!("Removed stack root {} for thread {:?}", object_id, self.thread_id);
            true
        } else {
            false
        }
    }
    
    /// Add a pinned root
    #[instrument(skip(self))]
    pub fn add_pinned_root(&mut self, object_id: ObjectId, description: Option<String>) {
        if self.pinned_roots.insert(object_id) {
            let root_info = RootInfo::new(object_id, RootType::Pinned, description);
            self.root_info.insert(object_id, root_info);
            debug!("Added pinned root {} for thread {:?}", object_id, self.thread_id);
        }
    }
    
    /// Remove a pinned root
    #[instrument(skip(self))]
    pub fn remove_pinned_root(&mut self, object_id: ObjectId) -> bool {
        if self.pinned_roots.remove(&object_id) {
            self.root_info.remove(&object_id);
            debug!("Removed pinned root {} for thread {:?}", object_id, self.thread_id);
            true
        } else {
            false
        }
    }
    
    /// Get all roots for this thread
    pub fn get_all_roots(&self) -> Vec<ObjectId> {
        let mut roots = Vec::new();
        roots.extend(&self.stack_roots);
        roots.extend(&self.pinned_roots);
        roots
    }
    
    /// Get root info for an object
    pub fn get_root_info(&self, object_id: ObjectId) -> Option<&RootInfo> {
        self.root_info.get(&object_id)
    }
    
    /// Check if an object is rooted in this thread
    pub fn is_root(&self, object_id: ObjectId) -> bool {
        self.stack_roots.contains(&object_id) || self.pinned_roots.contains(&object_id)
    }
    
    /// Get thread ID
    pub fn thread_id(&self) -> ThreadId {
        self.thread_id
    }
    
    /// Clear all stack roots (called when thread exits)
    pub fn clear_stack_roots(&mut self) {
        let count = self.stack_roots.len();
        for object_id in &self.stack_roots {
            self.root_info.remove(object_id);
        }
        self.stack_roots.clear();
        debug!("Cleared {} stack roots for thread {:?}", count, self.thread_id);
    }
}

impl Default for ThreadRootSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Global root set manager
/// 
/// Coordinates root sets across all threads and manages global roots.
/// This is the main interface for root set management in the GC system.
pub struct RootSetManager {
    /// Global roots (exist for program lifetime)
    global_roots: RwLock<HashSet<ObjectId>>,
    /// JIT code roots
    jit_roots: RwLock<HashSet<ObjectId>>,
    /// External roots (from C code, etc.)
    external_roots: RwLock<HashSet<ObjectId>>,
    /// Goroutine roots (indexed by goroutine ID)
    goroutine_roots: RwLock<HashMap<u64, HashSet<ObjectId>>>,
    /// Thread-local root sets
    thread_roots: RwLock<HashMap<ThreadId, ThreadRootSet>>,
    /// Root information for debugging
    global_root_info: RwLock<HashMap<ObjectId, RootInfo>>,
}

impl RootSetManager {
    /// Create a new root set manager
    #[instrument]
    pub fn new() -> Self {
        info!("Creating root set manager");
        
        Self {
            global_roots: RwLock::new(HashSet::new()),
            jit_roots: RwLock::new(HashSet::new()),
            external_roots: RwLock::new(HashSet::new()),
            goroutine_roots: RwLock::new(HashMap::new()),
            thread_roots: RwLock::new(HashMap::new()),
            global_root_info: RwLock::new(HashMap::new()),
        }
    }
    
    /// Add a global root
    #[instrument(skip(self))]
    pub fn add_global_root(&self, object_id: ObjectId, description: Option<String>) -> Result<(), String> {
        let mut globals = self.global_roots.write()
            .map_err(|_| "Failed to acquire write lock on global roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        if globals.insert(object_id) {
            let root_info = RootInfo::new(object_id, RootType::Global, description);
            info.insert(object_id, root_info);
            debug!("Added global root {}", object_id);
        }
        
        Ok(())
    }
    
    /// Remove a global root
    #[instrument(skip(self))]
    pub fn remove_global_root(&self, object_id: ObjectId) -> Result<bool, String> {
        let mut globals = self.global_roots.write()
            .map_err(|_| "Failed to acquire write lock on global roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        let removed = globals.remove(&object_id);
        if removed {
            info.remove(&object_id);
            debug!("Removed global root {}", object_id);
        }
        
        Ok(removed)
    }
    
    /// Add a stack root for the current thread
    #[instrument(skip(self))]
    pub fn add_stack_root(&self, object_id: ObjectId, description: Option<String>) -> Result<(), String> {
        let thread_id = thread::current().id();
        let mut threads = self.thread_roots.write()
            .map_err(|_| "Failed to acquire write lock on thread roots")?;
        
        let thread_roots = threads.entry(thread_id).or_insert_with(ThreadRootSet::new);
        thread_roots.add_stack_root(object_id, description);
        
        Ok(())
    }
    
    /// Remove a stack root for the current thread
    #[instrument(skip(self))]
    pub fn remove_stack_root(&self, object_id: ObjectId) -> Result<bool, String> {
        let thread_id = thread::current().id();
        let mut threads = self.thread_roots.write()
            .map_err(|_| "Failed to acquire write lock on thread roots")?;
        
        if let Some(thread_roots) = threads.get_mut(&thread_id) {
            Ok(thread_roots.remove_stack_root(object_id))
        } else {
            Ok(false)
        }
    }
    
    /// Add a pinned root
    #[instrument(skip(self))]
    pub fn add_pinned_root(&self, object_id: ObjectId, description: Option<String>) -> Result<(), String> {
        let thread_id = thread::current().id();
        let mut threads = self.thread_roots.write()
            .map_err(|_| "Failed to acquire write lock on thread roots")?;
        
        let thread_roots = threads.entry(thread_id).or_insert_with(ThreadRootSet::new);
        thread_roots.add_pinned_root(object_id, description);
        
        Ok(())
    }
    
    /// Remove a pinned root
    #[instrument(skip(self))]
    pub fn remove_pinned_root(&self, object_id: ObjectId) -> Result<bool, String> {
        let thread_id = thread::current().id();
        let mut threads = self.thread_roots.write()
            .map_err(|_| "Failed to acquire write lock on thread roots")?;
        
        if let Some(thread_roots) = threads.get_mut(&thread_id) {
            Ok(thread_roots.remove_pinned_root(object_id))
        } else {
            Ok(false)
        }
    }
    
    /// Add a JIT code root
    #[instrument(skip(self))]
    pub fn add_jit_root(&self, object_id: ObjectId, description: Option<String>) -> Result<(), String> {
        let mut jit_roots = self.jit_roots.write()
            .map_err(|_| "Failed to acquire write lock on JIT roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        if jit_roots.insert(object_id) {
            let root_info = RootInfo::new(object_id, RootType::JitCode, description);
            info.insert(object_id, root_info);
            debug!("Added JIT root {}", object_id);
        }
        
        Ok(())
    }
    
    /// Remove a JIT code root
    #[instrument(skip(self))]
    pub fn remove_jit_root(&self, object_id: ObjectId) -> Result<bool, String> {
        let mut jit_roots = self.jit_roots.write()
            .map_err(|_| "Failed to acquire write lock on JIT roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        let removed = jit_roots.remove(&object_id);
        if removed {
            info.remove(&object_id);
            debug!("Removed JIT root {}", object_id);
        }
        
        Ok(removed)
    }
    
    /// Add an external root
    #[instrument(skip(self))]
    pub fn add_external_root(&self, object_id: ObjectId, description: Option<String>) -> Result<(), String> {
        let mut external_roots = self.external_roots.write()
            .map_err(|_| "Failed to acquire write lock on external roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        if external_roots.insert(object_id) {
            let root_info = RootInfo::new(object_id, RootType::External, description);
            info.insert(object_id, root_info);
            debug!("Added external root {}", object_id);
        }
        
        Ok(())
    }
    
    /// Remove an external root
    #[instrument(skip(self))]
    pub fn remove_external_root(&self, object_id: ObjectId) -> Result<bool, String> {
        let mut external_roots = self.external_roots.write()
            .map_err(|_| "Failed to acquire write lock on external roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        let removed = external_roots.remove(&object_id);
        if removed {
            info.remove(&object_id);
            debug!("Removed external root {}", object_id);
        }
        
        Ok(removed)
    }
    
    /// Add a goroutine root
    #[instrument(skip(self))]
    pub fn add_goroutine_root(&self, goroutine_id: u64, object_id: ObjectId, description: Option<String>) -> Result<(), String> {
        let mut goroutine_roots = self.goroutine_roots.write()
            .map_err(|_| "Failed to acquire write lock on goroutine roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        let roots = goroutine_roots.entry(goroutine_id).or_insert_with(HashSet::new);
        if roots.insert(object_id) {
            let root_info = RootInfo::new(object_id, RootType::Goroutine(goroutine_id), description);
            info.insert(object_id, root_info);
            debug!("Added goroutine root {} for goroutine {}", object_id, goroutine_id);
        }
        
        Ok(())
    }
    
    /// Remove a goroutine root
    #[instrument(skip(self))]
    pub fn remove_goroutine_root(&self, goroutine_id: u64, object_id: ObjectId) -> Result<bool, String> {
        let mut goroutine_roots = self.goroutine_roots.write()
            .map_err(|_| "Failed to acquire write lock on goroutine roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        let mut removed = false;
        if let Some(roots) = goroutine_roots.get_mut(&goroutine_id) {
            removed = roots.remove(&object_id);
            if removed {
                info.remove(&object_id);
                debug!("Removed goroutine root {} for goroutine {}", object_id, goroutine_id);
                
                // Clean up empty goroutine root sets
                if roots.is_empty() {
                    goroutine_roots.remove(&goroutine_id);
                    debug!("Removed empty goroutine root set for goroutine {}", goroutine_id);
                }
            }
        }
        
        Ok(removed)
    }
    
    /// Clear all roots for a goroutine (called when goroutine exits)
    #[instrument(skip(self))]
    pub fn clear_goroutine_roots(&self, goroutine_id: u64) -> Result<usize, String> {
        let mut goroutine_roots = self.goroutine_roots.write()
            .map_err(|_| "Failed to acquire write lock on goroutine roots")?;
        let mut info = self.global_root_info.write()
            .map_err(|_| "Failed to acquire write lock on global root info")?;
        
        if let Some(roots) = goroutine_roots.remove(&goroutine_id) {
            let count = roots.len();
            for object_id in &roots {
                info.remove(object_id);
            }
            debug!("Cleared {} goroutine roots for goroutine {}", count, goroutine_id);
            Ok(count)
        } else {
            Ok(0)
        }
    }
    
    /// Clear all stack roots for a thread (called when thread exits)
    #[instrument(skip(self))]
    pub fn clear_thread_roots(&self, thread_id: ThreadId) -> Result<usize, String> {
        let mut threads = self.thread_roots.write()
            .map_err(|_| "Failed to acquire write lock on thread roots")?;
        
        if let Some(mut thread_roots) = threads.remove(&thread_id) {
            let count = thread_roots.get_all_roots().len();
            thread_roots.clear_stack_roots();
            debug!("Cleared {} thread roots for thread {:?}", count, thread_id);
            Ok(count)
        } else {
            Ok(0)
        }
    }
    
    /// Get all root objects across all categories
    #[instrument(skip(self))]
    pub fn get_all_roots(&self) -> Result<Vec<ObjectId>, String> {
        let mut all_roots = Vec::new();
        
        // Global roots
        {
            let globals = self.global_roots.read()
                .map_err(|_| "Failed to acquire read lock on global roots")?;
            all_roots.extend(globals.iter());
        }
        
        // JIT roots
        {
            let jit_roots = self.jit_roots.read()
                .map_err(|_| "Failed to acquire read lock on JIT roots")?;
            all_roots.extend(jit_roots.iter());
        }
        
        // External roots
        {
            let external_roots = self.external_roots.read()
                .map_err(|_| "Failed to acquire read lock on external roots")?;
            all_roots.extend(external_roots.iter());
        }
        
        // Goroutine roots
        {
            let goroutine_roots = self.goroutine_roots.read()
                .map_err(|_| "Failed to acquire read lock on goroutine roots")?;
            for roots in goroutine_roots.values() {
                all_roots.extend(roots.iter());
            }
        }
        
        // Thread roots
        {
            let threads = self.thread_roots.read()
                .map_err(|_| "Failed to acquire read lock on thread roots")?;
            for thread_root_set in threads.values() {
                all_roots.extend(thread_root_set.get_all_roots());
            }
        }
        
        debug!("Found {} total root objects", all_roots.len());
        Ok(all_roots)
    }
    
    /// Get root statistics
    pub fn get_stats(&self) -> Result<RootSetStats, String> {
        let globals = self.global_roots.read()
            .map_err(|_| "Failed to acquire read lock on global roots")?;
        let jit_roots = self.jit_roots.read()
            .map_err(|_| "Failed to acquire read lock on JIT roots")?;
        let external_roots = self.external_roots.read()
            .map_err(|_| "Failed to acquire read lock on external roots")?;
        let goroutine_roots = self.goroutine_roots.read()
            .map_err(|_| "Failed to acquire read lock on goroutine roots")?;
        let threads = self.thread_roots.read()
            .map_err(|_| "Failed to acquire read lock on thread roots")?;
        
        let global_count = globals.len();
        let jit_count = jit_roots.len();
        let external_count = external_roots.len();
        let goroutine_count: usize = goroutine_roots.values().map(|s| s.len()).sum();
        let active_goroutines = goroutine_roots.len();
        let thread_count: usize = threads.values().map(|t| t.get_all_roots().len()).sum();
        let active_threads = threads.len();
        
        Ok(RootSetStats {
            global_roots: global_count,
            jit_roots: jit_count,
            external_roots: external_count,
            goroutine_roots: goroutine_count,
            thread_roots: thread_count,
            active_goroutines,
            active_threads,
            total_roots: global_count + jit_count + external_count + goroutine_count + thread_count,
        })
    }
    
    /// Check if an object is rooted anywhere
    pub fn is_root(&self, object_id: ObjectId) -> bool {
        // Check global roots
        if let Ok(globals) = self.global_roots.read() {
            if globals.contains(&object_id) {
                return true;
            }
        }
        
        // Check JIT roots
        if let Ok(jit_roots) = self.jit_roots.read() {
            if jit_roots.contains(&object_id) {
                return true;
            }
        }
        
        // Check external roots
        if let Ok(external_roots) = self.external_roots.read() {
            if external_roots.contains(&object_id) {
                return true;
            }
        }
        
        // Check goroutine roots
        if let Ok(goroutine_roots) = self.goroutine_roots.read() {
            for roots in goroutine_roots.values() {
                if roots.contains(&object_id) {
                    return true;
                }
            }
        }
        
        // Check thread roots
        if let Ok(threads) = self.thread_roots.read() {
            for thread_root_set in threads.values() {
                if thread_root_set.is_root(object_id) {
                    return true;
                }
            }
        }
        
        false
    }
}

impl Default for RootSetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Root set statistics for monitoring and debugging
#[derive(Debug, Clone)]
pub struct RootSetStats {
    pub global_roots: usize,
    pub jit_roots: usize,
    pub external_roots: usize,
    pub goroutine_roots: usize,
    pub thread_roots: usize,
    pub active_goroutines: usize,
    pub active_threads: usize,
    pub total_roots: usize,
}

impl std::fmt::Display for RootSetStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Root Set Stats:\n\
             - Global Roots: {}\n\
             - JIT Roots: {}\n\
             - External Roots: {}\n\
             - Goroutine Roots: {} (across {} goroutines)\n\
             - Thread Roots: {} (across {} threads)\n\
             - Total Roots: {}",
            self.global_roots,
            self.jit_roots,
            self.external_roots,
            self.goroutine_roots,
            self.active_goroutines,
            self.thread_roots,
            self.active_threads,
            self.total_roots
        )
    }
}

/// Shared root set manager for use across the GC system
pub type SharedRootSetManager = Arc<RootSetManager>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectId;
    use std::thread;
    use std::sync::Arc;
    
    #[test]
    fn test_thread_root_set() {
        let mut roots = ThreadRootSet::new();
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        // Add stack roots
        roots.add_stack_root(obj1, Some("test object 1".to_string()));
        roots.add_stack_root(obj2, None);
        
        assert!(roots.is_root(obj1));
        assert!(roots.is_root(obj2));
        assert_eq!(roots.get_all_roots().len(), 2);
        
        // Remove a root
        assert!(roots.remove_stack_root(obj1));
        assert!(!roots.is_root(obj1));
        assert!(roots.is_root(obj2));
        assert_eq!(roots.get_all_roots().len(), 1);
        
        // Add pinned root
        roots.add_pinned_root(obj1, Some("pinned object".to_string()));
        assert!(roots.is_root(obj1));
        assert_eq!(roots.get_all_roots().len(), 2);
    }
    
    #[test]
    fn test_root_set_manager() {
        let manager = RootSetManager::new();
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        let obj3 = ObjectId::new(3);
        
        // Add global root
        manager.add_global_root(obj1, Some("global object".to_string())).unwrap();
        assert!(manager.is_root(obj1));
        
        // Add stack root
        manager.add_stack_root(obj2, Some("stack object".to_string())).unwrap();
        assert!(manager.is_root(obj2));
        
        // Add JIT root
        manager.add_jit_root(obj3, Some("JIT object".to_string())).unwrap();
        assert!(manager.is_root(obj3));
        
        // Check all roots
        let all_roots = manager.get_all_roots().unwrap();
        assert_eq!(all_roots.len(), 3);
        assert!(all_roots.contains(&obj1));
        assert!(all_roots.contains(&obj2));
        assert!(all_roots.contains(&obj3));
        
        // Remove roots
        assert!(manager.remove_global_root(obj1).unwrap());
        assert!(!manager.is_root(obj1));
        
        assert!(manager.remove_stack_root(obj2).unwrap());
        assert!(!manager.is_root(obj2));
        
        assert!(manager.remove_jit_root(obj3).unwrap());
        assert!(!manager.is_root(obj3));
    }
    
    #[test]
    fn test_goroutine_roots() {
        let manager = RootSetManager::new();
        let goroutine_id = 42;
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        // Add goroutine roots
        manager.add_goroutine_root(goroutine_id, obj1, Some("goroutine obj 1".to_string())).unwrap();
        manager.add_goroutine_root(goroutine_id, obj2, Some("goroutine obj 2".to_string())).unwrap();
        
        assert!(manager.is_root(obj1));
        assert!(manager.is_root(obj2));
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.goroutine_roots, 2);
        assert_eq!(stats.active_goroutines, 1);
        
        // Clear goroutine roots
        let cleared = manager.clear_goroutine_roots(goroutine_id).unwrap();
        assert_eq!(cleared, 2);
        assert!(!manager.is_root(obj1));
        assert!(!manager.is_root(obj2));
    }
    
    #[test]
    fn test_concurrent_root_management() {
        let manager = Arc::new(RootSetManager::new());
        let mut handles = vec![];
        
        // Spawn threads that add/remove roots concurrently
        for i in 0..5 {
            let mgr = manager.clone();
            handles.push(thread::spawn(move || {
                for j in 0..10 {
                    let obj_id = ObjectId::new((i * 10 + j) as u64 + 1);
                    
                    // Add stack root
                    mgr.add_stack_root(obj_id, Some(format!("thread {} obj {}", i, j))).unwrap();
                    
                    // Verify it's rooted
                    assert!(mgr.is_root(obj_id));
                    
                    // Remove it
                    assert!(mgr.remove_stack_root(obj_id).unwrap());
                    assert!(!mgr.is_root(obj_id));
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // All thread-local roots should be cleaned up
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.thread_roots, 0);
    }
    
    #[test]
    fn test_root_set_stats() {
        let manager = RootSetManager::new();
        
        let stats_empty = manager.get_stats().unwrap();
        assert_eq!(stats_empty.total_roots, 0);
        
        // Add various types of roots
        manager.add_global_root(ObjectId::new(1), None).unwrap();
        manager.add_jit_root(ObjectId::new(2), None).unwrap();
        manager.add_external_root(ObjectId::new(3), None).unwrap();
        manager.add_stack_root(ObjectId::new(4), None).unwrap();
        manager.add_goroutine_root(100, ObjectId::new(5), None).unwrap();
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.global_roots, 1);
        assert_eq!(stats.jit_roots, 1);
        assert_eq!(stats.external_roots, 1);
        assert_eq!(stats.thread_roots, 1);
        assert_eq!(stats.goroutine_roots, 1);
        assert_eq!(stats.total_roots, 5);
        assert_eq!(stats.active_goroutines, 1);
        assert_eq!(stats.active_threads, 1);
    }
}
