use crate::error::CursedError;
/// Enhanced semaphore implementation for CURSED IPC
/// 
/// Provides both named and unnamed semaphores for process synchronization,
/// with advanced features like timeout operations, priority queuing, and statistics.

use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant, SystemTime};
use std::collections::{HashMap, VecDeque};
use std::thread;
use std::ffi::CString;

// use crate::stdlib::ipc::{IpcResult, IpcError, IpcHandle, IpcPermissions};
// use crate::stdlib::ipc::error::{timeout_error, resource_error, system_error};

/// Semaphore configuration
#[derive(Debug, Clone)]
pub struct SemaphoreConfig {
impl SemaphoreConfig {
    pub fn new(initial_value: i32) -> Self {
        Self {
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    pub fn with_max_value(mut self, max_value: i32) -> Self {
        self.max_value = max_value.max(1);
        self
    pub fn with_priority(mut self) -> Self {
        self.enable_priority = true;
        self
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Semaphore statistics
#[derive(Debug, Clone, Default)]
pub struct SemaphoreStatistics {
/// Waiting thread information
#[derive(Debug)]
struct WaitingThread {
/// Inter-process semaphore
pub struct Semaphore {
    #[cfg(unix)]
impl Semaphore {
    /// Create a new semaphore
    pub fn new(config: SemaphoreConfig) -> IpcResult<Self> {
        let mut stats = SemaphoreStatistics::default();
        stats.current_value = config.initial_value;
        stats.created_at = SystemTime::now();

        let handle = if let Some(ref name) = config.name {
//             IpcHandle::named(name, crate::stdlib::ipc::types::IpcHandleType::Semaphore)
        } else {
//             IpcHandle::anonymous(crate::stdlib::ipc::types::IpcHandleType::Semaphore)

        let mut semaphore = Self {
            #[cfg(unix)]

        // For named semaphores on Unix, use POSIX semaphores
        #[cfg(unix)]
        if config.name.is_some() {
            semaphore.init_posix_semaphore()?;
        Ok(semaphore)
    /// Create a named semaphore
    pub fn create_named(name: &str, initial_value: i32) -> IpcResult<Self> {
        let config = SemaphoreConfig::new(initial_value).named(name);
        Self::new(config)
    /// Open an existing named semaphore
    pub fn open_named(name: &str) -> IpcResult<Self> {
        let config = SemaphoreConfig::new(0).named(name);
        // For simplicity, create a new one (in real implementation, would connect to existing)
        Self::new(config)
    /// Wait for the semaphore (decrement)
    pub fn wait(&self) -> IpcResult<()> {
        self.wait_timeout(self.config.timeout)
    /// Wait for the semaphore with timeout
    pub fn wait_timeout(&self, timeout: Duration) -> IpcResult<()> {
        let start_time = Instant::now();
        let thread_id = thread::current().id();

        // Add to waiting queue if priority is enabled
        if self.config.enable_priority {
            let mut queue = self.waiting_queue.lock().unwrap();
            queue.push_back(WaitingThread {
                priority: 0, // Default priority
            });
        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.total_waits += 1;
            stats.current_waiters += 1;
            stats.max_waiters_seen = stats.max_waiters_seen.max(stats.current_waiters);
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
        result
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
        // Notify waiting threads
        self.condition.notify_one();

        // Update statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.total_posts += 1;
            stats.current_value += 1;
        Ok(())
    /// Get current semaphore value
    pub fn value(&self) -> IpcResult<i32> {
        let value = self.value.lock().unwrap();
        Ok(*value)
    /// Get semaphore statistics
    pub fn statistics(&self) -> SemaphoreStatistics {
        let stats = self.statistics.lock().unwrap();
        stats.clone()
    /// Get number of current waiters
    pub fn waiting_count(&self) -> u32 {
        let stats = self.statistics.lock().unwrap();
        stats.current_waiters
    /// Reset semaphore to initial value
    pub fn reset(&self) -> IpcResult<()> {
        {
            let mut value = self.value.lock().unwrap();
            *value = self.config.initial_value;
        // Wake up all waiting threads
        self.condition.notify_all();

        // Reset statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.current_value = self.config.initial_value;
            stats.current_waiters = 0;
        Ok(())
    /// Get semaphore configuration
    pub fn config(&self) -> &SemaphoreConfig {
        &self.config
    /// Get handle information
    pub fn handle(&self) -> &IpcHandle {
        &self.handle
    #[cfg(unix)]
    fn init_posix_semaphore(&mut self) -> IpcResult<()> {
        if let Some(ref name) = self.config.name {
            let name_cstr = CString::new(format!("/{}", name))
                .map_err(|e| IpcError::InvalidInput(format!("Invalid semaphore name: {}", e)))?;

            unsafe {
                let sem = libc::sem_open(
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
// Thread-safe semaphore operations
unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

/// Semaphore builder for fluent configuration
pub struct SemaphoreBuilder {
impl SemaphoreBuilder {
    pub fn new(initial_value: i32) -> Self {
        Self {
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.config.name = Some(name.to_string());
        self
    pub fn max_value(mut self, max_value: i32) -> Self {
        self.config.max_value = max_value;
        self
    pub fn with_priority(mut self) -> Self {
        self.config.enable_priority = true;
        self
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    pub fn auto_cleanup(mut self, enabled: bool) -> Self {
        self.config.auto_cleanup = enabled;
        self
    pub fn build(self) -> IpcResult<Semaphore> {
        Semaphore::new(self.config)
    }
}

/// Convenience functions for semaphore operations
pub fn create_semaphore(initial_value: i32) -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(initial_value).build()
pub fn create_named_semaphore(name: &str, initial_value: i32) -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(initial_value).named(name).build()
pub fn create_binary_semaphore() -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(1).max_value(1).build()
pub fn create_counting_semaphore(initial_value: i32, max_value: i32) -> IpcResult<Semaphore> {
    SemaphoreBuilder::new(initial_value).max_value(max_value).build()
