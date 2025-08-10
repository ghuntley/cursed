/// Thread-safe write barrier implementation for concurrent GC
/// Fixes race conditions in write barrier operations for M:N scheduler

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, AtomicUsize, AtomicPtr, Ordering}};
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::VecDeque;
use std::thread;
use std::time::Instant;
use crossbeam::atomic::AtomicCell;

use crate::runtime::gc::{GarbageCollector, HeapObject};
use crate::error::CursedError;
use crate::memory::Tag;

/// Thread-safe write barrier entry
#[derive(Debug, Clone)]
pub struct WriteBarrierEntry {
    /// Source object address
    pub source_addr: usize,
    /// Target object address  
    pub target_addr: usize,
    /// Field offset
    pub field_offset: usize,
    /// Thread ID for debugging
    pub thread_id: thread::ThreadId,
    /// Timestamp for debugging
    pub timestamp: Instant,
    /// Barrier sequence number for ordering
    pub sequence: u64,
}

/// Write barrier log with thread-safe operations
pub struct ThreadSafeWriteBarrierLog {
    /// Log entries (lock-free using channels)
    log_sender: Sender<WriteBarrierEntry>,
    log_receiver: Arc<Mutex<Receiver<WriteBarrierEntry>>>,
    /// Processing queue
    processing_queue: Arc<Mutex<VecDeque<WriteBarrierEntry>>>,
    /// Sequence counter for ordering
    sequence_counter: AtomicUsize,
    /// Active flag
    active: AtomicBool,
    /// Worker thread handle
    worker_handle: Option<thread::JoinHandle<()>>,
}

impl ThreadSafeWriteBarrierLog {
    /// Create new thread-safe write barrier log
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver_arc = Arc::new(Mutex::new(receiver));
        let processing_queue = Arc::new(Mutex::new(VecDeque::new()));
        
        Self {
            log_sender: sender,
            log_receiver: receiver_arc,
            processing_queue,
            sequence_counter: AtomicUsize::new(0),
            active: AtomicBool::new(true),
            worker_handle: None,
        }
    }
    
    /// Start background processing worker
    pub fn start_worker(&mut self) -> Result<(), CursedError> {
        let receiver = Arc::clone(&self.log_receiver);
        let queue = Arc::clone(&self.processing_queue);
        let active = Arc::new(AtomicBool::new(true));
        let worker_active = Arc::clone(&active);
        
        let handle = thread::Builder::new()
            .name("write-barrier-worker".to_string())
            .spawn(move || {
                while worker_active.load(Ordering::Acquire) {
                    // Process incoming entries with timeout
                    if let Ok(receiver_guard) = receiver.lock() {
                        match receiver_guard.recv_timeout(std::time::Duration::from_millis(100)) {
                            Ok(entry) => {
                                // Add to processing queue
                                if let Ok(mut queue_guard) = queue.lock() {
                                    queue_guard.push_back(entry);
                                }
                            }
                            Err(mpsc::RecvTimeoutError::Timeout) => {
                                // Timeout is normal, continue
                                continue;
                            }
                            Err(mpsc::RecvTimeoutError::Disconnected) => {
                                // Channel disconnected, exit
                                break;
                            }
                        }
                    }
                    
                    // Process queued entries in batches
                    if let Ok(mut queue_guard) = queue.lock() {
                        let batch_size = std::cmp::min(100, queue_guard.len());
                        for _ in 0..batch_size {
                            if let Some(entry) = queue_guard.pop_front() {
                                // Process the write barrier entry
                                Self::process_write_barrier_entry(entry);
                            }
                        }
                    }
                }
            })
            .map_err(|e| CursedError::runtime_error(&format!("Failed to start write barrier worker: {}", e)))?;
        
        self.worker_handle = Some(handle);
        Ok(())
    }
    
    /// Record write barrier (thread-safe)
    pub fn record_write_barrier(
        &self,
        source_addr: usize,
        target_addr: usize,
        field_offset: usize,
    ) -> Result<(), CursedError> {
        if !self.active.load(Ordering::Acquire) {
            return Ok(());
        }
        
        let sequence = self.sequence_counter.fetch_add(1, Ordering::AcqRel);
        
        let entry = WriteBarrierEntry {
            source_addr,
            target_addr,
            field_offset,
            thread_id: thread::current().id(),
            timestamp: Instant::now(),
            sequence: sequence as u64,
        };
        
        // Send to worker thread (lock-free)
        self.log_sender.send(entry)
            .map_err(|_| CursedError::runtime_error("Write barrier log channel disconnected"))?;
        
        Ok(())
    }
    
    /// Process write barrier entry
    fn process_write_barrier_entry(entry: WriteBarrierEntry) {
        // Mark target object if not already marked
        // This is a simplified implementation - in practice would integrate with tri-color marking
        
        // Update remembered set for inter-generational references
        if Self::is_inter_generational_reference(entry.source_addr, entry.target_addr) {
            Self::add_to_remembered_set(entry.source_addr);
        }
        
        // Update card table
        Self::mark_card_dirty(entry.source_addr);
    }
    
    /// Check if reference is inter-generational
    fn is_inter_generational_reference(source_addr: usize, target_addr: usize) -> bool {
        // Simplified check - would use actual generation information
        let source_gen = (source_addr >> 20) & 0x3; // Extract generation bits
        let target_gen = (target_addr >> 20) & 0x3;
        source_gen != target_gen
    }
    
    /// Add to remembered set
    fn add_to_remembered_set(source_addr: usize) {
        // Would add to actual remembered set implementation
    }
    
    /// Mark card as dirty in card table
    fn mark_card_dirty(source_addr: usize) {
        // Would mark card in actual card table implementation
    }
    
    /// Stop processing and cleanup
    pub fn stop(&mut self) -> Result<(), CursedError> {
        self.active.store(false, Ordering::Release);
        
        if let Some(handle) = self.worker_handle.take() {
            handle.join()
                .map_err(|_| CursedError::runtime_error("Failed to join write barrier worker thread"))?;
        }
        
        Ok(())
    }
}

/// Thread-safe memory barrier for GC operations
pub struct MemoryBarrier {
    /// Barrier generation counter
    generation: AtomicUsize,
    /// Thread synchronization
    sync_points: Arc<RwLock<Vec<AtomicBool>>>,
}

impl MemoryBarrier {
    /// Create new memory barrier
    pub fn new() -> Self {
        Self {
            generation: AtomicUsize::new(0),
            sync_points: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Acquire read barrier
    pub fn acquire_read_barrier(&self) -> BarrierGuard {
        let generation = self.generation.load(Ordering::Acquire);
        BarrierGuard::new(generation, BarrierType::Read)
    }
    
    /// Acquire write barrier
    pub fn acquire_write_barrier(&self) -> BarrierGuard {
        let generation = self.generation.fetch_add(1, Ordering::AcqRel);
        BarrierGuard::new(generation, BarrierType::Write)
    }
    
    /// Global memory fence
    pub fn memory_fence() {
        std::sync::atomic::fence(Ordering::SeqCst);
    }
}

/// Memory barrier guard
pub struct BarrierGuard {
    generation: usize,
    barrier_type: BarrierType,
}

impl BarrierGuard {
    fn new(generation: usize, barrier_type: BarrierType) -> Self {
        Self { generation, barrier_type }
    }
}

impl Drop for BarrierGuard {
    fn drop(&mut self) {
        // Release barrier on drop
        match self.barrier_type {
            BarrierType::Read => {
                std::sync::atomic::fence(Ordering::Acquire);
            }
            BarrierType::Write => {
                std::sync::atomic::fence(Ordering::Release);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BarrierType {
    Read,
    Write,
}

/// Safe pointer wrapper for Send/Sync compliance
#[derive(Debug)]
pub struct SafeGcPointer {
    ptr: AtomicPtr<u8>,
    tag: AtomicCell<Tag>,
    generation: AtomicUsize,
}

impl SafeGcPointer {
    /// Create new safe pointer
    pub fn new(ptr: *mut u8, tag: Tag) -> Self {
        Self {
            ptr: AtomicPtr::new(ptr),
            tag: AtomicCell::new(tag),
            generation: AtomicUsize::new(0),
        }
    }
    
    /// Load pointer with proper ordering
    pub fn load(&self, ordering: Ordering) -> *mut u8 {
        self.ptr.load(ordering)
    }
    
    /// Store pointer with proper ordering
    pub fn store(&self, ptr: *mut u8, ordering: Ordering) {
        self.ptr.store(ptr, ordering);
        self.generation.fetch_add(1, ordering);
    }
    
    /// Compare and swap with proper ordering
    pub fn compare_exchange(
        &self,
        current: *mut u8,
        new: *mut u8,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut u8, *mut u8> {
        match self.ptr.compare_exchange(current, new, success, failure) {
            Ok(old) => {
                self.generation.fetch_add(1, success);
                Ok(old)
            }
            Err(current) => Err(current),
        }
    }
    
    /// Get tag
    pub fn get_tag(&self) -> Tag {
        self.tag.load()
    }
    
    /// Set tag
    pub fn set_tag(&self, tag: Tag) {
        self.tag.store(tag);
    }
    
    /// Get generation
    pub fn get_generation(&self) -> usize {
        self.generation.load(Ordering::Acquire)
    }
}

// Safe Send/Sync implementations
unsafe impl Send for SafeGcPointer {}
unsafe impl Sync for SafeGcPointer {}

// Thread-safe global write barrier log
use std::sync::OnceLock;
static GLOBAL_WRITE_BARRIER_LOG: OnceLock<Arc<Mutex<ThreadSafeWriteBarrierLog>>> = OnceLock::new();

/// Initialize global write barrier log
pub fn initialize_write_barrier_log() -> Result<(), CursedError> {
    let mut log = ThreadSafeWriteBarrierLog::new();
    log.start_worker()?;
    
    GLOBAL_WRITE_BARRIER_LOG.set(Arc::new(Mutex::new(log)))
        .map_err(|_| CursedError::runtime_error("Write barrier log already initialized"))?;
    
    Ok(())
}

/// Get global write barrier log
pub fn get_write_barrier_log() -> Option<Arc<Mutex<ThreadSafeWriteBarrierLog>>> {
    GLOBAL_WRITE_BARRIER_LOG.get().cloned()
}

/// Record write barrier globally (thread-safe)
pub fn record_write_barrier(
    source_addr: usize,
    target_addr: usize,
    field_offset: usize,
) -> Result<(), CursedError> {
    if let Some(log_arc) = get_write_barrier_log() {
        if let Ok(log) = log_arc.lock() {
            log.record_write_barrier(source_addr, target_addr, field_offset)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_safe_write_barrier_log() {
        let mut log = ThreadSafeWriteBarrierLog::new();
        log.start_worker().unwrap();
        
        // Record write barriers from multiple threads
        let handles: Vec<_> = (0..4).map(|i| {
            let log_sender = log.log_sender.clone();
            thread::spawn(move || {
                for j in 0..100 {
                    let entry = WriteBarrierEntry {
                        source_addr: (i * 1000 + j) as usize,
                        target_addr: (i * 1000 + j + 1) as usize,
                        field_offset: 0,
                        thread_id: thread::current().id(),
                        timestamp: Instant::now(),
                        sequence: (i * 100 + j) as u64,
                    };
                    log_sender.send(entry).unwrap();
                }
            })
        }).collect();
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Allow processing time
        thread::sleep(Duration::from_millis(200));
        
        log.stop().unwrap();
    }
    
    #[test]
    fn test_safe_gc_pointer() {
        let ptr = SafeGcPointer::new(std::ptr::null_mut(), Tag::Object);
        
        let test_ptr = 0x1000 as *mut u8;
        ptr.store(test_ptr, Ordering::Release);
        
        assert_eq!(ptr.load(Ordering::Acquire), test_ptr);
        assert_eq!(ptr.get_generation(), 1);
        
        let new_ptr = 0x2000 as *mut u8;
        let result = ptr.compare_exchange(test_ptr, new_ptr, Ordering::AcqRel, Ordering::Acquire);
        
        assert!(result.is_ok());
        assert_eq!(ptr.load(Ordering::Acquire), new_ptr);
        assert_eq!(ptr.get_generation(), 2);
    }
    
    #[test]
    fn test_memory_barrier() {
        let barrier = MemoryBarrier::new();
        
        let _read_guard = barrier.acquire_read_barrier();
        let _write_guard = barrier.acquire_write_barrier();
        
        MemoryBarrier::memory_fence();
    }
    
    #[test] 
    fn test_concurrent_write_barriers() {
        initialize_write_barrier_log().unwrap();
        
        let handles: Vec<_> = (0..8).map(|i| {
            thread::spawn(move || {
                for j in 0..50 {
                    record_write_barrier(
                        (i * 1000 + j) as usize,
                        (i * 1000 + j + 1) as usize,
                        0
                    ).unwrap();
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Cleanup
        if let Some(log_arc) = get_write_barrier_log() {
            if let Ok(mut log) = log_arc.lock() {
                log.stop().unwrap();
            }
        }
    }
}
