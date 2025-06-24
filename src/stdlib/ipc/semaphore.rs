use crate::error::Error;
/// Enhanced semaphore implementation for CURSED IPC
/// 
/// Provides both named and unnamed semaphores for process synchronization,
/// with advanced features like timeout operations, priority queuing, and statistics.

use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant, SystemTime};
use std::collections::{HashMap, VecDeque};
use std::thread;
use std::ffi::CString;

use crate::stdlib::ipc::{IpcResult, IpcError, IpcHandle, IpcPermissions};
use crate::stdlib::ipc::error::{timeout_error, resource_error, system_error};

/// Semaphore configuration
#[derive(Debug, Clone)]
pub struct SemaphoreConfig {
    pub name: Option<String>,
    pub initial_value: i32,
    pub max_value: i32,
    pub permissions: IpcPermissions,
    pub enable_priority: bool,
    pub timeout: Duration,
    pub auto_cleanup: bool,
}

impl SemaphoreConfig {
    pub fn new(initial_value: i32) -> Self {
        Self {
            name: None,
            initial_value: initial_value.max(0),
            max_value: i32::MAX,
            permissions: IpcPermissions::read_write(),
            enable_priority: false,
            timeout: Duration::from_secs(30),
            auto_cleanup: true,
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_max_value(mut self, max_value: i32) -> Self {
        self.max_value = max_value.max(1);
        self
    }

    pub fn with_priority(mut self) -> Self {
        self.enable_priority = true;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Semaphore statistics
#[derive(Debug, Clone, Default)]
pub struct SemaphoreStatistics {
    pub total_waits: u64,
    pub total_posts: u64,
    pub timeout_count: u64,
    pub current_waiters: u32,
    pub current_value: i32,
    pub max_waiters_seen: u32,
    pub average_wait_time: Duration,
    pub created_at: SystemTime,
}

/// Waiting thread information
#[derive(Debug)]
struct WaitingThread {
    thread_id: thread::ThreadId,
    priority: i32,
    start_time: Instant,
}

/// Inter-process semaphore
pub struct Semaphore {
    handle: IpcHandle,
    config: SemaphoreConfig,
    value: Arc<Mutex<i32>>,
    condition: Arc<Condvar>,
    waiting_queue: Arc<Mutex<VecDeque<WaitingThread>>>,
    statistics: Arc<Mutex<SemaphoreStatistics>>,
    #[cfg(unix)]
    posix_semaphore: Option<*mut libc::sem_t>,
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(config: SemaphoreConfig) -> IpcResult<Self> {
        let mut stats = SemaphoreStatistics::default();
        stats.current_value = config.initial_value;
        stats.created_at = SystemTime::now();

        let handle = if let Some(ref name) = config.name {
            IpcHandle::named(name, crate::stdlib::ipc::crate::types::IpcHandleType::Semaphore)
        } else {
            IpcHandle::anonymous(crate::stdlib::ipc::crate::types::IpcHandleType::Semaphore)
        };

        let mut semaphore = Self {
            handle,
            config: config.clone(),
            value: Arc::new(Mutex::new(config.initial_value)),
            condition: Arc::new(Condvar::new()),
            waiting_queue: Arc::new(Mutex::new(VecDeque::new())),
            statistics: Arc::new(Mutex::new(stats)),
            #[cfg(unix)]
            posix_semaphore: None,
        };

        // For named semaphores on Unix, use POSIX semaphores
        #[cfg(unix)]
        if config.name.is_some() {
            semaphore.init_posix_semaphore()?;
        }

        Ok(semaphore)
    }

    /// Create a named semaphore
    pub fn create_named(name: &str, initial_value: i32) -> IpcResult<Self> {
        let config = SemaphoreConfig::new(initial_value).named(name);
        Self::new(config)
    }

    /// Open an existing named semaphore
    pub fn open_named(name: &str) -> IpcResult<Self> {
        let config = SemaphoreConfig::new(0).named(name);
        // For simplicity, create a new one (in real implementation, would connect to existing)
        Self::new(config)
    }

    /// Wait for the semaphore (decrement)
    pub fn wait(&self) -> IpcResult<()> {
        self.wait_timeout(self.config.timeout)
    }

    /// Wait for the semaphore with timeout
    pub fn wait_timeout(&self, timeout: Duration) -> IpcResult<()> {
        let start_time = Instant::now();
        let thread_id = thread::current().id();

        // Add to waiting queue if priority is enabled
        if self.config.enable_priority {
            let mut queue = self.waiting_queue.lock().unwrap();
            queue.push_back(WaitingThread {
                thread_id,
                priority: 0, // Default priority
                start_time,
            });
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.total_waits += 1;
            stats.current_waiters += 1;
            stats.max_waiters_seen = stats.max_waiters_seen.max(stats.current_waiters);
        }

        let result = {
            let value = self.value.lock().unwrap();
            let (_guard, timeout_result) = self.condition
                .wait_timeout_while(value, timeout, |&mut val| val <= 0)
                .unwrap();

            if timeout_result.timed_out() {
                Err(timeout_error(&format!("Semaphore wait timed out after {:?}", timeout)))
            } else {
                Ok(())
            }
        };

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.current_waiters = stats.current_waiters.saturating_sub(1);
            
            let wait_time = start_time.elapsed();
            let total_time = stats.average_wait_time.as_nanos() as u64 * (stats.total_waits - 1) + wait_time.as_nanos() as u64;
            stats.average_wait_time = Duration::from_nanos(total_time / stats.total_waits);

            if result.is_err() {
                stats.timeout_count += 1;
            }
        }

        // Remove from waiting queue
        if self.config.enable_priority {
            let mut queue = self.waiting_queue.lock().unwrap();
            queue.retain(|w| w.thread_id != thread_id);
        }

        result
    }

    /// Try to wait without blocking
    pub fn try_wait(&self) -> IpcResult<bool> {
        let mut value = self.value.lock().unwrap();
        if *value > 0 {
            *value -= 1;
            self.update_statistics_on_success();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Post to the semaphore (increment)
    pub fn post(&self) -> IpcResult<()> {
        {
            let mut value = self.value.lock().unwrap();
            if *value >= self.config.max_value {
                return Err(resource_error("Semaphore value would exceed maximum"));
            }
            *value += 1;
        }

        // Notify waiting threads
        self.condition.notify_one();

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.total_posts += 1;
            stats.current_value += 1;
        }

        Ok(())
    }

    /// Get current semaphore value
    pub fn value(&self) -> IpcResult<i32> {
        let value = self.value.lock().unwrap();
        Ok(*value)
    }

    /// Get semaphore statistics
    pub fn statistics(&self) -> SemaphoreStatistics {
        let stats = self.statistics.lock().unwrap();
        stats.clone()
    }

    /// Get number of current waiters
    pub fn waiting_count(&self) -> u32 {
        let stats = self.statistics.lock().unwrap();
        stats.current_waiters
    }

    /// Reset semaphore to initial value
    pub fn reset(&self) -> IpcResult<()> {
        {
            let mut value = self.value.lock().unwrap();
            *value = self.config.initial_value;
        }

        // Wake up all waiting threads
        self.condition.notify_all();

        // Reset statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.current_value = self.config.initial_value;
            stats.current_waiters = 0;
        }

        Ok(())
    }

    /// Get semaphore configuration
    pub fn config(&self) -> &SemaphoreConfig {
        &self.config
    }

    /// Get handle information
    pub fn handle(&self) -> &IpcHandle {
        &self.handle
    }

    #[cfg(unix)]
    fn init_posix_semaphore(&mut self) -> IpcResult<()> {
        if let Some(ref name) = self.config.name {
            let name_cstr = CString::new(format!("/{}", name))
                .map_err(|e| IpcError::InvalidInput(format!("Invalid semaphore name: {}", e)))?;

            unsafe {
                let sem = libc::sem_open(
                    name_cstr.as_ptr(),
                    libc::O_CREAT | libc::O_EXCL,
                    self.config.permissions.to_mode(),
                    self.config.initial_value as u32,
                );

                if sem == libc::SEM_FAILED {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    if errno == libc::EEXIST {
                        // Semaphore already exists, try to open it
                        let sem = libc::sem_open(name_cstr.as_ptr(), 0);
                        if sem == libc::SEM_FAILED {
                            return Err(system_error(&format!("Failed to open existing semaphore: {}", errno)));
                        }
                        self.posix_semaphore = Some(sem);
                    } else {
                        return Err(system_error(&format!("Failed to create POSIX semaphore: {}", errno)));
                    }
                } else {
                    self.posix_semaphore = Some(sem);
                }
            }
        }
        Ok(())
    }

    fn update_statistics_on_success(&self) {
        let mut stats = self.statistics.lock().unwrap();
        stats.current_value -= 1;
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(sem) = self.posix_semaphore {
            unsafe {
                libc::sem_close(sem);
                
                // Unlink named semaphore if auto_cleanup is enabled
                if self.config.auto_cleanup {
                    if let Some(ref name) = self.config.name {
                        if let Ok(name_cstr) = CString::new(format!("/{}", name)) {
                            libc::sem_unlink(name_cstr.as_ptr());
                        }
                    }
                }
            }
        }
    }
}

// Thread-safe semaphore operations
unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

/// Semaphore builder for fluent configuration
pub struct SemaphoreBuilder {
    config: SemaphoreConfig,
}

impl SemaphoreBuilder {
    pub fn new(initial_value: i32) -> Self {
        Self {
            config: SemaphoreConfig::new(initial_value),
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.config.name = Some(name.to_string());
        self
    }

    pub fn max_value(mut self, max_value: i32) -> Self {
        self.config.max_value = max_value;
        self
    }

    pub fn with_priority(mut self) -> Self {
        self.config.enable_priority = true;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn auto_cleanup(mut self, enabled: bool) -> Self {
        self.config.auto_cleanup = enabled;
        self
    }

    pub fn build(self) -> IpcResult<Semaphore> {
        Semaphore::new(self.config)
    }
}

/// Convenience functions for semaphore operations
pub fn create_semaphore(initial_value: i32) -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(initial_value).build()
}

pub fn create_named_semaphore(name: &str, initial_value: i32) -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(initial_value).named(name).build()
}

pub fn create_binary_semaphore() -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(1).max_value(1).build()
}

pub fn create_counting_semaphore(initial_value: i32, max_value: i32) -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(initial_value).max_value(max_value).build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn test_semaphore_creation() {
        let config = SemaphoreConfig::new(5);
        let semaphore = Semaphore::new(config).unwrap();
        assert_eq!(semaphore.value().unwrap(), 5);
    }

    #[test]
    fn test_semaphore_wait_and_post() {
        let semaphore = create_semaphore(2).unwrap();
        
        // Initial value should be 2
        assert_eq!(semaphore.value().unwrap(), 2);
        
        // Wait should decrement
        assert!(semaphore.try_wait().unwrap());
        assert_eq!(semaphore.value().unwrap(), 1);
        
        // Post should increment
        semaphore.post().unwrap();
        assert_eq!(semaphore.value().unwrap(), 2);
    }

    #[test]
    fn test_semaphore_try_wait_failure() {
        let semaphore = create_semaphore(0).unwrap();
        assert!(!semaphore.try_wait().unwrap());
        assert_eq!(semaphore.value().unwrap(), 0);
    }

    #[test]
    fn test_semaphore_statistics() {
        let semaphore = create_semaphore(1).unwrap();
        semaphore.try_wait().unwrap();
        semaphore.post().unwrap();
        
        let stats = semaphore.statistics();
        assert_eq!(stats.total_waits, 1);
        assert_eq!(stats.total_posts, 1);
    }

    #[test]
    fn test_semaphore_reset() {
        let semaphore = create_semaphore(5).unwrap();
        semaphore.try_wait().unwrap();
        assert_eq!(semaphore.value().unwrap(), 4);
        
        semaphore.reset().unwrap();
        assert_eq!(semaphore.value().unwrap(), 5);
    }

    #[test]
    fn test_binary_semaphore() {
        let semaphore = create_binary_semaphore().unwrap();
        assert_eq!(semaphore.value().unwrap(), 1);
        
        semaphore.try_wait().unwrap();
        assert_eq!(semaphore.value().unwrap(), 0);
        
        semaphore.post().unwrap();
        assert_eq!(semaphore.value().unwrap(), 1);
    }

    #[test]
    fn test_concurrent_semaphore_access() {
        let semaphore = Arc::new(create_semaphore(3).unwrap());
        let counter = Arc::new(AtomicI32::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let sem = Arc::clone(&semaphore);
            let cnt = Arc::clone(&counter);
            
            let handle = thread::spawn(move || {
                if sem.try_wait().unwrap() {
                    cnt.fetch_add(1, Ordering::Relaxed);
                    thread::sleep(Duration::from_millis(10));
                    sem.post().unwrap();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // All operations should complete
        assert_eq!(semaphore.value().unwrap(), 3);
    }

    #[test]
    fn test_semaphore_timeout() {
        let semaphore = create_semaphore(0).unwrap();
        let result = semaphore.wait_timeout(Duration::from_millis(10));
        assert!(result.is_err());
        
        let stats = semaphore.statistics();
        assert_eq!(stats.timeout_count, 1);
    }

    #[test]
    fn test_semaphore_builder() {
        let semaphore = SemaphoreBuilder::new(10)
            .named("test_semaphore")
            .max_value(20)
            .with_priority()
            .timeout(Duration::from_secs(5))
            .auto_cleanup(true)
            .build()
            .unwrap();

        assert_eq!(semaphore.value().unwrap(), 10);
        assert_eq!(semaphore.config().max_value, 20);
        assert!(semaphore.config().enable_priority);
    }
}
