use crate::error::CursedError;
/// Threading and synchronization primitives for CURSED
/// 
/// This module provides comprehensive threading support including:
/// - Thread spawning and management
/// - Synchronization primitives (Mutex, RwLock, Semaphore, etc.)
/// - Concurrent collections
/// - Parallel processing utilities
/// - Atomic operations

pub mod error;
pub mod primitives;
pub mod collections;
pub mod parallel;
pub mod thread_local;

// Re-export all public APIs for easy access
pub use error::{SyncError, SyncResult, thread_error, lock_error, timeout_error, deadlock_error};

// Thread management
pub use primitives::{
    // Core threading
    Thread, ThreadId, ThreadBuilder, JoinHandle,
    spawn, spawn_named, current_thread_id, current_thread_name, 
    sleep, yield_now, park, unpark,
    
    // Synchronization primitives
    Mutex, RwLock, Semaphore, Barrier, CondVar,
    MutexGuard, RwLockReadGuard, RwLockWriteGuard,
    
    // Atomic operations
    AtomicBool, AtomicI32, AtomicI64, AtomicUsize, AtomicPtr,
    Ordering, memory_fence, compiler_fence,
    
    // Once and lazy initialization
    Once, OnceCell, Lazy,
};

// Concurrent collections
pub use collections::{
    ConcurrentHashMap, ConcurrentVec, ConcurrentQueue, ConcurrentStack,
    ChannelSender, ChannelReceiver, channel, bounded_channel, unbounded_channel,
    select_channel, try_select_channel, ChannelError,
    LockFreeStack, LockFreeQueue, AtomicCounter,
};

// Parallel processing
pub use parallel::{
    ThreadPool, ThreadPoolBuilder, ThreadPoolConfig,
    WorkStealingPool, TaskQueue, Task, TaskResult,
    ParallelIterator, par_map, par_filter, par_reduce, par_for_each,
    RayonCompat, parallel_sort, parallel_search,
    SchedulerPolicy, LoadBalancer,
    init_global_thread_pool, shutdown_global_thread_pool, get_thread_pool_utilization,
};

// Thread-local storage
pub use thread_local::{
    ThreadLocal, ThreadLocalKey, ThreadLocalCell, ThreadLocalValue,
    with_thread_local, thread_local_get, thread_local_set, thread_local_remove,
    create_thread_local_key, cleanup_current_thread, cleanup_thread_local_storage,
    get_thread_local_statistics, ThreadLocalStatistics, TlsKey,
};

/// Initialize the sync module - sets up any global state needed
pub fn init_sync_module() -> SyncResult<()> {
    // Initialize global thread pool
    parallel::init_global_thread_pool()?;
    
    // Set up signal handlers for proper cleanup
    setup_sync_signal_handlers()?;
    
    Ok(())
}

/// Cleanup sync module resources
pub fn cleanup_sync_module() -> SyncResult<()> {
    parallel::shutdown_global_thread_pool()?;
    thread_local::cleanup_thread_local_storage()?;
    Ok(())
}

/// Set up signal handlers for proper cleanup of sync resources
fn setup_sync_signal_handlers() -> SyncResult<()> {
    // Set up handlers for SIGTERM, SIGINT to ensure proper cleanup
    // This is a placeholder - actual implementation would use platform-specific APIs
    Ok(())
}

/// Get comprehensive sync module statistics
pub fn get_sync_statistics() -> SyncStatistics {
    SyncStatistics {
        active_threads: primitives::get_active_thread_count(),
        thread_pool_utilization: parallel::get_thread_pool_utilization(),
        lock_contention_stats: primitives::get_lock_contention_stats(),
        channel_stats: collections::get_channel_statistics(),
        memory_usage: get_sync_memory_usage(),
    }
}

/// Statistics about the sync module's usage
#[derive(Debug, Clone)]
pub struct SyncStatistics {
    pub active_threads: usize,
    pub thread_pool_utilization: f64,
    pub lock_contention_stats: LockContentionStats,
    pub channel_stats: ChannelStatistics,
    pub memory_usage: usize,
}

#[derive(Debug, Clone)]
pub struct LockContentionStats {
    pub mutex_contentions: u64,
    pub rwlock_contentions: u64,
    pub average_wait_time_nanos: u64,
}

#[derive(Debug, Clone)]  
pub struct ChannelStatistics {
    pub active_channels: usize,
    pub messages_sent: u64,
    pub messages_received: u64, 
    pub blocked_senders: usize,
    pub blocked_receivers: usize,
}

fn get_sync_memory_usage() -> usize {
    // Placeholder - would calculate actual memory usage
    0
}

