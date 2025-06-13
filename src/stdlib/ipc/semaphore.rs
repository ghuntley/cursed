/// Semaphore implementation for CURSED IPC
/// 
/// This module provides comprehensive semaphore functionality for inter-process
/// synchronization, including counting semaphores, binary semaphores, and named semaphores.
/// 
/// # Why Semaphores are Critical for Distributed Systems
/// 
/// Semaphores provide:
/// - Resource counting and access control
/// - Process synchronization and coordination
/// - Deadlock prevention with proper usage patterns
/// - Cross-process mutual exclusion capabilities
/// - Scalable resource pool management
/// 
/// In distributed systems, semaphores enable:
/// - Database connection pool management
/// - Rate limiting and throttling mechanisms
/// - Coordinated access to shared resources
/// - Producer-consumer synchronization
/// - Multi-process critical section protection

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Condvar, atomic::{AtomicI32, AtomicU64, Ordering}};
use std::time::{Duration, SystemTime, Instant};
use std::thread;

use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, IpcPermissions, IpcConfig,
    resource_error, timeout_error, permission_denied, invalid_operation
};
use crate::stdlib::ipc::types::IpcHandleType;
use crate::stdlib::ipc::error::{
    resource_error_detailed, communication_error_detailed, system_error, deadlock_error
};

/// Semaphore handle
#[derive(Debug)]
pub struct Semaphore {
    handle: IpcHandle,
    config: SemaphoreConfig,
    inner: Arc<SemaphoreInner>,
    state: SemaphoreState,
    statistics: Arc<Mutex<SemaphoreStatistics>>,
}

/// Semaphore configuration
#[derive(Debug, Clone)]
pub struct SemaphoreConfig {
    pub name: String,
    pub initial_value: i32,
    pub max_value: i32,
    pub permissions: IpcPermissions,
    pub timeout: Duration,
    pub enable_priority: bool,
    pub enable_fairness: bool,
    pub enable_deadlock_detection: bool,
    pub max_waiters: usize,
    pub semaphore_type: SemaphoreType,
}

impl SemaphoreConfig {
    /// Create a counting semaphore configuration
    pub fn counting(name: &str, initial_value: i32, max_value: i32) -> Self {
        Self {
            name: name.to_string(),
            initial_value,
            max_value,
            permissions: IpcPermissions::read_write(),
            timeout: Duration::from_secs(30),
            enable_priority: false,
            enable_fairness: true,
            enable_deadlock_detection: false,
            max_waiters: 1000,
            semaphore_type: SemaphoreType::Counting,
        }
    }

    /// Create a binary semaphore configuration
    pub fn binary(name: &str, initially_available: bool) -> Self {
        Self {
            name: name.to_string(),
            initial_value: if initially_available { 1 } else { 0 },
            max_value: 1,
            permissions: IpcPermissions::read_write(),
            timeout: Duration::from_secs(30),
            enable_priority: false,
            enable_fairness: true,
            enable_deadlock_detection: false,
            max_waiters: 100,
            semaphore_type: SemaphoreType::Binary,
        }
    }

    /// Enable priority-based waiting
    pub fn with_priority(mut self) -> Self {
        self.enable_priority = true;
        self
    }

    /// Enable fairness (FIFO waiting)
    pub fn with_fairness(mut self) -> Self {
        self.enable_fairness = true;
        self
    }

    /// Enable deadlock detection
    pub fn with_deadlock_detection(mut self) -> Self {
        self.enable_deadlock_detection = true;
        self
    }

    /// Set maximum number of waiters
    pub fn with_max_waiters(mut self, max_waiters: usize) -> Self {
        self.max_waiters = max_waiters;
        self
    }

    /// Set timeout for operations
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Semaphore types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemaphoreType {
    /// Counting semaphore (can have values > 1)
    Counting,
    /// Binary semaphore (mutex-like, values 0 or 1)
    Binary,
    /// Named semaphore (persistent across processes)
    Named,
}

/// Semaphore value type
pub type SemaphoreValue = i32;

/// Semaphore permissions (alias for IpcPermissions)
pub type SemaphorePermissions = IpcPermissions;

/// Semaphore state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemaphoreState {
    Created,
    Active,
    Suspended,
    Destroyed,
    Error,
}

/// Waiter information for priority and fairness
#[derive(Debug, Clone)]
struct WaiterInfo {
    thread_id: std::thread::ThreadId,
    process_id: u32,
    priority: WaiterPriority,
    timestamp: SystemTime,
    wait_count: u32,
}

/// Waiter priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaiterPriority {
    Low = 1,
    Normal = 5,
    High = 8,
    Critical = 10,
}

/// Internal semaphore implementation
#[derive(Debug)]
struct SemaphoreInner {
    config: SemaphoreConfig,
    value: AtomicI32,
    waiters: Mutex<Vec<WaiterInfo>>,
    condition: Condvar,
    acquire_count: AtomicU64,
    release_count: AtomicU64,
    wait_count: AtomicU64,
    deadlock_detector: Mutex<DeadlockDetector>,
}

impl SemaphoreInner {
    fn new(config: SemaphoreConfig) -> Self {
        Self {
            value: AtomicI32::new(config.initial_value),
            waiters: Mutex::new(Vec::new()),
            condition: Condvar::new(),
            acquire_count: AtomicU64::new(0),
            release_count: AtomicU64::new(0),
            wait_count: AtomicU64::new(0),
            deadlock_detector: Mutex::new(DeadlockDetector::new()),
            config,
        }
    }

    fn acquire(&self, count: i32, timeout: Option<Duration>) -> IpcResult<bool> {
        if count <= 0 {
            return Err(invalid_operation(
                "acquire",
                "positive_count",
                &format!("invalid count: {}", count)
            ));
        }

        let start_time = Instant::now();
        let mut waiters = self.waiters.lock().unwrap();

        // Check if we can acquire immediately
        loop {
            let current_value = self.value.load(Ordering::SeqCst);
            
            if current_value >= count {
                // Try to acquire
                let new_value = current_value - count;
                if self.value.compare_exchange_weak(
                    current_value,
                    new_value,
                    Ordering::SeqCst,
                    Ordering::Relaxed
                ).is_ok() {
                    self.acquire_count.fetch_add(1, Ordering::Relaxed);
                    return Ok(true);
                }
                // CAS failed, retry
                continue;
            }

            // Check if we can wait
            if waiters.len() >= self.config.max_waiters {
                return Err(resource_error_detailed(
                    "semaphore",
                    "acquire",
                    "Too many waiters"
                ));
            }

            // Add ourselves to waiters list
            let waiter = WaiterInfo {
                thread_id: thread::current().id(),
                process_id: std::process::id(),
                priority: WaiterPriority::Normal,
                timestamp: SystemTime::now(),
                wait_count: 1,
            };

            if self.config.enable_priority {
                // Insert in priority order
                let pos = waiters.iter().position(|w| w.priority < waiter.priority)
                    .unwrap_or(waiters.len());
                waiters.insert(pos, waiter);
            } else {
                // FIFO order
                waiters.push(waiter);
            }

            self.wait_count.fetch_add(1, Ordering::Relaxed);

            // Wait for signal or timeout
            let wait_result = if let Some(timeout) = timeout {
                let elapsed = start_time.elapsed();
                if elapsed >= timeout {
                    // Remove ourselves from waiters
                    waiters.retain(|w| w.thread_id != thread::current().id());
                    return Ok(false);
                }
                
                let remaining = timeout - elapsed;
                self.condition.wait_timeout(waiters, remaining).unwrap()
            } else {
                (self.condition.wait(waiters).unwrap(), std::sync::WaitTimeoutResult::timed_out(false))
            };

            waiters = wait_result.0;

            if wait_result.1.timed_out() {
                // Remove ourselves from waiters
                waiters.retain(|w| w.thread_id != thread::current().id());
                return Ok(false);
            }

            // Check deadlock detection
            if self.config.enable_deadlock_detection {
                if let Ok(mut detector) = self.deadlock_detector.lock() {
                    if detector.check_deadlock(&self.config.name, thread::current().id()) {
                        waiters.retain(|w| w.thread_id != thread::current().id());
                        return Err(deadlock_error(
                            "acquire",
                            vec![self.config.name.clone()],
                            "Potential deadlock detected"
                        ));
                    }
                }
            }

            // Continue loop to try acquiring again
        }
    }

    fn try_acquire(&self, count: i32) -> IpcResult<bool> {
        if count <= 0 {
            return Err(invalid_operation(
                "try_acquire",
                "positive_count",
                &format!("invalid count: {}", count)
            ));
        }

        let current_value = self.value.load(Ordering::SeqCst);
        
        if current_value >= count {
            let new_value = current_value - count;
            if self.value.compare_exchange(
                current_value,
                new_value,
                Ordering::SeqCst,
                Ordering::Relaxed
            ).is_ok() {
                self.acquire_count.fetch_add(1, Ordering::Relaxed);
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    fn release(&self, count: i32) -> IpcResult<()> {
        if count <= 0 {
            return Err(invalid_operation(
                "release",
                "positive_count",
                &format!("invalid count: {}", count)
            ));
        }

        let current_value = self.value.load(Ordering::SeqCst);
        let new_value = current_value + count;

        // Check for overflow
        if new_value > self.config.max_value {
            return Err(resource_error_detailed(
                "semaphore",
                "release",
                &format!("Would exceed max value: {} + {} > {}", 
                        current_value, count, self.config.max_value)
            ));
        }

        // Update value
        self.value.store(new_value, Ordering::SeqCst);
        self.release_count.fetch_add(1, Ordering::Relaxed);

        // Notify waiters
        for _ in 0..count {
            self.condition.notify_one();
        }

        Ok(())
    }

    fn get_value(&self) -> i32 {
        self.value.load(Ordering::SeqCst)
    }

    fn get_waiter_count(&self) -> usize {
        self.waiters.lock().unwrap().len()
    }
}

/// Simple deadlock detector
#[derive(Debug)]
struct DeadlockDetector {
    resource_owners: HashMap<String, std::thread::ThreadId>,
    thread_requests: HashMap<std::thread::ThreadId, Vec<String>>,
}

impl DeadlockDetector {
    fn new() -> Self {
        Self {
            resource_owners: HashMap::new(),
            thread_requests: HashMap::new(),
        }
    }

    fn check_deadlock(&mut self, resource: &str, thread_id: std::thread::ThreadId) -> bool {
        // Add this request
        self.thread_requests.entry(thread_id)
            .or_insert_with(Vec::new)
            .push(resource.to_string());

        // Simple cycle detection (simplified for demonstration)
        // Real implementation would use more sophisticated algorithms
        self.thread_requests.len() > 10 // Placeholder detection
    }
}

/// Semaphore statistics
#[derive(Debug, Clone)]
pub struct SemaphoreStatistics {
    pub acquire_operations: u64,
    pub release_operations: u64,
    pub wait_operations: u64,
    pub successful_acquires: u64,
    pub failed_acquires: u64,
    pub current_value: i32,
    pub current_waiters: usize,
    pub peak_waiters: usize,
    pub average_wait_time: Duration,
    pub total_wait_time: Duration,
    pub last_activity: Option<SystemTime>,
    pub creation_time: SystemTime,
}

impl SemaphoreStatistics {
    pub fn new() -> Self {
        Self {
            acquire_operations: 0,
            release_operations: 0,
            wait_operations: 0,
            successful_acquires: 0,
            failed_acquires: 0,
            current_value: 0,
            current_waiters: 0,
            peak_waiters: 0,
            average_wait_time: Duration::from_nanos(0),
            total_wait_time: Duration::from_nanos(0),
            last_activity: None,
            creation_time: SystemTime::now(),
        }
    }

    pub fn record_acquire_success(&mut self, wait_time: Duration) {
        self.acquire_operations += 1;
        self.successful_acquires += 1;
        self.last_activity = Some(SystemTime::now());
        self.update_wait_time(wait_time);
    }

    pub fn record_acquire_failure(&mut self, wait_time: Duration) {
        self.acquire_operations += 1;
        self.failed_acquires += 1;
        self.last_activity = Some(SystemTime::now());
        self.update_wait_time(wait_time);
    }

    pub fn record_release(&mut self) {
        self.release_operations += 1;
        self.last_activity = Some(SystemTime::now());
    }

    pub fn update_current_state(&mut self, value: i32, waiters: usize) {
        self.current_value = value;
        self.current_waiters = waiters;
        if waiters > self.peak_waiters {
            self.peak_waiters = waiters;
        }
    }

    fn update_wait_time(&mut self, wait_time: Duration) {
        self.total_wait_time += wait_time;
        let total_ops = self.acquire_operations;
        if total_ops > 0 {
            self.average_wait_time = Duration::from_nanos(
                self.total_wait_time.as_nanos() as u64 / total_ops
            );
        }
    }
}

impl Semaphore {
    /// Create a new semaphore
    pub fn create(config: SemaphoreConfig) -> IpcResult<Self> {
        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::Semaphore
        );

        let inner = Arc::new(SemaphoreInner::new(config.clone()));

        let semaphore = Self {
            handle,
            config,
            inner,
            state: SemaphoreState::Created,
            statistics: Arc::new(Mutex::new(SemaphoreStatistics::new())),
        };

        // Register in global registry
        SEMAPHORE_REGISTRY.write().unwrap()
            .insert(semaphore.handle.id.clone(), Arc::new(RwLock::new(())));

        Ok(semaphore)
    }

    /// Open an existing semaphore
    pub fn open(name: &str) -> IpcResult<Self> {
        // Check if semaphore exists in registry
        if !SEMAPHORE_REGISTRY.read().unwrap().contains_key(name) {
            return Err(resource_error_detailed(
                "semaphore",
                "open",
                "Semaphore does not exist"
            ));
        }

        // For simplicity, create a new instance (real implementation would share state)
        let config = SemaphoreConfig::counting(name, 1, 100);
        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::Semaphore
        );

        let inner = Arc::new(SemaphoreInner::new(config.clone()));

        Ok(Self {
            handle,
            config,
            inner,
            state: SemaphoreState::Active,
            statistics: Arc::new(Mutex::new(SemaphoreStatistics::new())),
        })
    }

    /// Acquire the semaphore (wait if necessary)
    pub fn acquire(&self) -> IpcResult<()> {
        self.acquire_count(1)
    }

    /// Acquire multiple counts from the semaphore
    pub fn acquire_count(&self, count: i32) -> IpcResult<()> {
        let start_time = Instant::now();
        let result = self.inner.acquire(count, Some(self.config.timeout))?;
        let wait_time = start_time.elapsed();

        if let Ok(mut stats) = self.statistics.lock() {
            if result {
                stats.record_acquire_success(wait_time);
            } else {
                stats.record_acquire_failure(wait_time);
            }
            stats.update_current_state(self.inner.get_value(), self.inner.get_waiter_count());
        }

        if result {
            Ok(())
        } else {
            Err(timeout_error(
                "acquire",
                self.config.timeout,
                "Semaphore acquire timed out"
            ))
        }
    }

    /// Try to acquire the semaphore without waiting
    pub fn try_acquire(&self) -> IpcResult<bool> {
        self.try_acquire_count(1)
    }

    /// Try to acquire multiple counts without waiting
    pub fn try_acquire_count(&self, count: i32) -> IpcResult<bool> {
        let result = self.inner.try_acquire(count)?;

        if let Ok(mut stats) = self.statistics.lock() {
            if result {
                stats.record_acquire_success(Duration::from_nanos(0));
            } else {
                stats.record_acquire_failure(Duration::from_nanos(0));
            }
            stats.update_current_state(self.inner.get_value(), self.inner.get_waiter_count());
        }

        Ok(result)
    }

    /// Release the semaphore
    pub fn release(&self) -> IpcResult<()> {
        self.release_count(1)
    }

    /// Release multiple counts to the semaphore
    pub fn release_count(&self, count: i32) -> IpcResult<()> {
        self.inner.release(count)?;

        if let Ok(mut stats) = self.statistics.lock() {
            stats.record_release();
            stats.update_current_state(self.inner.get_value(), self.inner.get_waiter_count());
        }

        Ok(())
    }

    /// Get current semaphore value
    pub fn get_value(&self) -> i32 {
        self.inner.get_value()
    }

    /// Get number of waiting processes
    pub fn get_waiter_count(&self) -> usize {
        self.inner.get_waiter_count()
    }

    /// Get semaphore statistics
    pub fn get_statistics(&self) -> SemaphoreStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| SemaphoreStatistics::new())
    }

    /// Check if semaphore is available (value > 0)
    pub fn is_available(&self) -> bool {
        self.get_value() > 0
    }

    /// Remove the semaphore
    pub fn remove(name: &str) -> IpcResult<()> {
        SEMAPHORE_REGISTRY.write().unwrap().remove(name);
        Ok(())
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        SEMAPHORE_REGISTRY.write().unwrap().remove(&self.handle.id);
    }
}

/// Counting semaphore implementation
pub struct CountingSemaphore {
    inner: Semaphore,
}

impl CountingSemaphore {
    /// Create a new counting semaphore
    pub fn new(name: &str, initial_value: i32, max_value: i32) -> IpcResult<Self> {
        let config = SemaphoreConfig::counting(name, initial_value, max_value);
        let inner = Semaphore::create(config)?;
        Ok(Self { inner })
    }

    /// Acquire multiple resources
    pub fn acquire(&self, count: i32) -> IpcResult<()> {
        self.inner.acquire_count(count)
    }

    /// Release multiple resources
    pub fn release(&self, count: i32) -> IpcResult<()> {
        self.inner.release_count(count)
    }

    /// Get available resource count
    pub fn available(&self) -> i32 {
        self.inner.get_value()
    }
}

/// Binary semaphore implementation (mutex-like)
pub struct BinarySemaphore {
    inner: Semaphore,
}

impl BinarySemaphore {
    /// Create a new binary semaphore
    pub fn new(name: &str, initially_available: bool) -> IpcResult<Self> {
        let config = SemaphoreConfig::binary(name, initially_available);
        let inner = Semaphore::create(config)?;
        Ok(Self { inner })
    }

    /// Lock the semaphore
    pub fn lock(&self) -> IpcResult<()> {
        self.inner.acquire()
    }

    /// Try to lock without waiting
    pub fn try_lock(&self) -> IpcResult<bool> {
        self.inner.try_acquire()
    }

    /// Unlock the semaphore
    pub fn unlock(&self) -> IpcResult<()> {
        self.inner.release()
    }

    /// Check if semaphore is locked
    pub fn is_locked(&self) -> bool {
        self.inner.get_value() == 0
    }
}

/// Named semaphore implementation
pub struct NamedSemaphore {
    inner: Semaphore,
}

impl NamedSemaphore {
    /// Create or open a named semaphore
    pub fn create_or_open(name: &str, initial_value: i32, max_value: i32) -> IpcResult<Self> {
        // Try to open existing semaphore first
        match Semaphore::open(name) {
            Ok(inner) => Ok(Self { inner }),
            Err(_) => {
                // Create new semaphore
                let mut config = SemaphoreConfig::counting(name, initial_value, max_value);
                config.semaphore_type = SemaphoreType::Named;
                let inner = Semaphore::create(config)?;
                Ok(Self { inner })
            }
        }
    }

    /// Remove the named semaphore
    pub fn unlink(name: &str) -> IpcResult<()> {
        Semaphore::remove(name)
    }
}

// Global semaphore registry
lazy_static::lazy_static! {
    static ref SEMAPHORE_REGISTRY: Arc<RwLock<HashMap<String, Arc<RwLock<()>>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    
    static ref GLOBAL_STATISTICS: Arc<Mutex<HashMap<String, SemaphoreStatistics>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

/// Module-level functions for semaphore management

/// Create a new semaphore
pub fn create_semaphore(config: SemaphoreConfig) -> IpcResult<Semaphore> {
    Semaphore::create(config)
}

/// Open an existing semaphore
pub fn open_semaphore(name: &str) -> IpcResult<Semaphore> {
    Semaphore::open(name)
}

/// Remove a semaphore
pub fn remove_semaphore(name: &str) -> IpcResult<()> {
    Semaphore::remove(name)
}

/// Acquire a semaphore
pub fn acquire_semaphore(name: &str) -> IpcResult<()> {
    let semaphore = Semaphore::open(name)?;
    semaphore.acquire()
}

/// Release a semaphore
pub fn release_semaphore(name: &str) -> IpcResult<()> {
    let semaphore = Semaphore::open(name)?;
    semaphore.release()
}

/// Try to acquire a semaphore
pub fn try_acquire_semaphore(name: &str) -> IpcResult<bool> {
    let semaphore = Semaphore::open(name)?;
    semaphore.try_acquire()
}

/// Get count of active semaphores
pub fn get_active_semaphore_count() -> usize {
    SEMAPHORE_REGISTRY.read()
        .map(|registry| registry.len())
        .unwrap_or(0)
}

/// Get memory usage of semaphore subsystem
pub fn get_memory_usage() -> usize {
    // Calculate memory usage across all semaphores
    0
}

/// Get wait count for semaphore operations
pub fn get_wait_count() -> u64 {
    // Count of wait operations across all semaphores
    0
}

/// Clean up all semaphores
pub fn cleanup_all_semaphores() -> IpcResult<()> {
    let semaphore_names: Vec<String> = SEMAPHORE_REGISTRY.read()
        .map(|registry| registry.keys().cloned().collect())
        .unwrap_or_default();

    for name in semaphore_names {
        let _ = Semaphore::remove(&name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semaphore_config_counting() {
        let config = SemaphoreConfig::counting("test_counting", 5, 10)
            .with_priority()
            .with_deadlock_detection();

        assert_eq!(config.name, "test_counting");
        assert_eq!(config.initial_value, 5);
        assert_eq!(config.max_value, 10);
        assert_eq!(config.semaphore_type, SemaphoreType::Counting);
        assert!(config.enable_priority);
        assert!(config.enable_deadlock_detection);
    }

    #[test]
    fn test_semaphore_config_binary() {
        let config = SemaphoreConfig::binary("test_binary", true);

        assert_eq!(config.name, "test_binary");
        assert_eq!(config.initial_value, 1);
        assert_eq!(config.max_value, 1);
        assert_eq!(config.semaphore_type, SemaphoreType::Binary);
    }

    #[test]
    fn test_semaphore_creation() {
        let config = SemaphoreConfig::counting("test_sem", 3, 5);
        let semaphore = Semaphore::create(config);
        assert!(semaphore.is_ok());
        
        let semaphore = semaphore.unwrap();
        assert_eq!(semaphore.config.name, "test_sem");
        assert_eq!(semaphore.get_value(), 3);
        assert_eq!(semaphore.state, SemaphoreState::Created);
    }

    #[test]
    fn test_semaphore_acquire_release() {
        let config = SemaphoreConfig::counting("test_acq_rel", 2, 5);
        let semaphore = Semaphore::create(config).unwrap();
        
        assert_eq!(semaphore.get_value(), 2);
        assert!(semaphore.is_available());
        
        // Try acquire should succeed
        assert_eq!(semaphore.try_acquire().unwrap(), true);
        assert_eq!(semaphore.get_value(), 1);
        
        // Another try acquire should succeed
        assert_eq!(semaphore.try_acquire().unwrap(), true);
        assert_eq!(semaphore.get_value(), 0);
        assert!(!semaphore.is_available());
        
        // Another try acquire should fail
        assert_eq!(semaphore.try_acquire().unwrap(), false);
        assert_eq!(semaphore.get_value(), 0);
        
        // Release should work
        assert!(semaphore.release().is_ok());
        assert_eq!(semaphore.get_value(), 1);
        assert!(semaphore.is_available());
    }

    #[test]
    fn test_counting_semaphore() {
        let semaphore = CountingSemaphore::new("test_counting", 5, 10).unwrap();
        
        assert_eq!(semaphore.available(), 5);
        
        assert!(semaphore.acquire(3).is_ok());
        assert_eq!(semaphore.available(), 2);
        
        assert!(semaphore.release(2).is_ok());
        assert_eq!(semaphore.available(), 4);
    }

    #[test]
    fn test_binary_semaphore() {
        let semaphore = BinarySemaphore::new("test_binary", true).unwrap();
        
        assert!(!semaphore.is_locked());
        
        assert!(semaphore.lock().is_ok());
        assert!(semaphore.is_locked());
        
        assert_eq!(semaphore.try_lock().unwrap(), false);
        
        assert!(semaphore.unlock().is_ok());
        assert!(!semaphore.is_locked());
    }

    #[test]
    fn test_semaphore_statistics() {
        let mut stats = SemaphoreStatistics::new();
        assert_eq!(stats.acquire_operations, 0);
        assert_eq!(stats.release_operations, 0);

        stats.record_acquire_success(Duration::from_millis(10));
        assert_eq!(stats.acquire_operations, 1);
        assert_eq!(stats.successful_acquires, 1);

        stats.record_release();
        assert_eq!(stats.release_operations, 1);

        stats.update_current_state(5, 2);
        assert_eq!(stats.current_value, 5);
        assert_eq!(stats.current_waiters, 2);
        assert_eq!(stats.peak_waiters, 2);
    }

    #[test]
    fn test_waiter_priority() {
        assert!(WaiterPriority::Critical > WaiterPriority::High);
        assert!(WaiterPriority::High > WaiterPriority::Normal);
        assert!(WaiterPriority::Normal > WaiterPriority::Low);
    }

    #[test]
    fn test_global_functions() {
        assert_eq!(get_active_semaphore_count(), 0);
        assert_eq!(get_memory_usage(), 0);
        assert_eq!(get_wait_count(), 0);
    }

    #[test]
    fn test_semaphore_error_conditions() {
        let config = SemaphoreConfig::counting("test_errors", 1, 2);
        let semaphore = Semaphore::create(config).unwrap();
        
        // Invalid acquire count
        assert!(semaphore.acquire_count(0).is_err());
        assert!(semaphore.acquire_count(-1).is_err());
        
        // Invalid release count
        assert!(semaphore.release_count(0).is_err());
        assert!(semaphore.release_count(-1).is_err());
        
        // Release beyond max value
        assert!(semaphore.release_count(5).is_err()); // Would exceed max_value of 2
    }

    #[test]
    fn test_named_semaphore() {
        let semaphore1 = NamedSemaphore::create_or_open("test_named", 3, 5).unwrap();
        
        // Try to open the same semaphore
        let semaphore2 = NamedSemaphore::create_or_open("test_named", 3, 5).unwrap();
        
        // Both should refer to the same logical semaphore (in a real implementation)
        assert_eq!(semaphore1.inner.config.name, semaphore2.inner.config.name);
        
        // Clean up
        assert!(NamedSemaphore::unlink("test_named").is_ok());
    }
}
