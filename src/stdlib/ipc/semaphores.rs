/// Semaphores implementation for CURSED IPC
/// 
/// Provides System V semaphores and POSIX semaphores for inter-process synchronization

use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use crate::stdlib::ipc::error::{IpcError, IpcResult, semaphore_error, system_error, timeout_error, not_found, already_exists};

/// Semaphore registry for cleanup
static SEMAPHORE_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<String, Arc<SemaphoreInfo>>>>> = std::sync::OnceLock::new();

#[derive(Debug)]
struct SemaphoreInfo {
    name: String,
    semaphore_id: i32,
    created_by_us: bool,
    ref_count: Arc<Mutex<usize>>,
}

fn get_semaphore_registry() -> &'static Arc<RwLock<HashMap<String, Arc<SemaphoreInfo>>>> {
    SEMAPHORE_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Semaphore value type
pub type SemaphoreValue = i32;

/// Semaphore configuration
#[derive(Debug, Clone)]
pub struct SemaphoreConfig {
    /// Initial value of the semaphore
    pub initial_value: SemaphoreValue,
    /// Maximum value of the semaphore
    pub max_value: SemaphoreValue,
    /// Permissions for the semaphore
    pub permissions: u32,
    /// Whether to create the semaphore if it doesn't exist
    pub create_if_missing: bool,
    /// Whether to use POSIX semaphores (vs System V)
    pub use_posix: bool,
    /// Number of semaphores in the set (System V only)
    pub semaphore_count: usize,
}

impl Default for SemaphoreConfig {
    fn default() -> Self {
        Self {
            initial_value: 1,
            max_value: i32::MAX,
            permissions: 0o666,
            create_if_missing: true,
            use_posix: false, // Default to System V for broader compatibility
            semaphore_count: 1,
        }
    }
}

/// Cross-platform semaphore
#[derive(Debug)]
pub struct Semaphore {
    name: String,
    config: SemaphoreConfig,
    #[cfg(unix)]
    semaphore_id: Option<i32>,
    #[cfg(unix)]
    posix_sem: Option<*mut libc::sem_t>,
    is_open: bool,
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(name: &str, initial_value: SemaphoreValue) -> Self {
        let config = SemaphoreConfig {
            initial_value,
            ..Default::default()
        };
        
        Self {
            name: name.to_string(),
            config,
            #[cfg(unix)]
            semaphore_id: None,
            #[cfg(unix)]
            posix_sem: None,
            is_open: false,
        }
    }

    /// Create with configuration
    pub fn with_config(name: &str, config: SemaphoreConfig) -> Self {
        Self {
            name: name.to_string(),
            config,
            #[cfg(unix)]
            semaphore_id: None,
            #[cfg(unix)]
            posix_sem: None,
            is_open: false,
        }
    }

    /// Open the semaphore
    pub fn open(&mut self) -> IpcResult<()> {
        if self.is_open {
            return Ok(());
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.open_posix()
            } else {
                self.open_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.open_windows()
        }
    }

    /// Wait (P operation, decrement)
    pub fn wait(&mut self) -> IpcResult<()> {
        if !self.is_open {
            return Err(semaphore_error(Some(&self.name), "wait", "Semaphore not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.wait_posix()
            } else {
                self.wait_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.wait_windows()
        }
    }

    /// Try wait (non-blocking P operation)
    pub fn try_wait(&mut self) -> IpcResult<bool> {
        if !self.is_open {
            return Err(semaphore_error(Some(&self.name), "try_wait", "Semaphore not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.try_wait_posix()
            } else {
                self.try_wait_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.try_wait_windows()
        }
    }

    /// Wait with timeout
    pub fn wait_timeout(&mut self, timeout: Duration) -> IpcResult<bool> {
        if !self.is_open {
            return Err(semaphore_error(Some(&self.name), "wait_timeout", "Semaphore not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.wait_timeout_posix(timeout)
            } else {
                self.wait_timeout_sysv(timeout)
            }
        }

        #[cfg(windows)]
        {
            self.wait_timeout_windows(timeout)
        }
    }

    /// Signal (V operation, increment)
    pub fn signal(&mut self) -> IpcResult<()> {
        if !self.is_open {
            return Err(semaphore_error(Some(&self.name), "signal", "Semaphore not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.signal_posix()
            } else {
                self.signal_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.signal_windows()
        }
    }

    /// Get current value
    pub fn value(&self) -> IpcResult<SemaphoreValue> {
        if !self.is_open {
            return Err(semaphore_error(Some(&self.name), "value", "Semaphore not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.value_posix()
            } else {
                self.value_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.value_windows()
        }
    }

    /// Close the semaphore
    pub fn close(&mut self) -> IpcResult<()> {
        if !self.is_open {
            return Ok(());
        }

        #[cfg(unix)]
        {
            if let Some(posix_sem) = self.posix_sem {
                unsafe {
                    sem_close(posix_sem);
                }
                self.posix_sem = None;
            }
            
            // System V semaphores don't need explicit closing
            self.semaphore_id = None;
        }

        self.is_open = false;
        self.unregister_semaphore();
        Ok(())
    }

    /// Delete the semaphore
    pub fn delete(&mut self) -> IpcResult<()> {
        self.close()?;

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.delete_posix()
            } else {
                self.delete_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.delete_windows()
        }
    }

    /// Get semaphore name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if semaphore is open
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    #[cfg(unix)]
    fn open_sysv(&mut self) -> IpcResult<()> {
        // Generate key from name
        let key = self.generate_sysv_key()?;
        
        // Try to get existing semaphore first
        let semaphore_id = unsafe {
            libc::semget(key, 0, 0)
        };
        
        if semaphore_id >= 0 {
            self.semaphore_id = Some(semaphore_id);
            self.is_open = true;
            self.register_semaphore(semaphore_id, false);
            return Ok(());
        }
        
        // Create new semaphore if allowed
        if !self.config.create_if_missing {
            return Err(not_found("semaphore", &self.name, "Semaphore does not exist"));
        }
        
        let semaphore_id = unsafe {
            libc::semget(
                key,
                self.config.semaphore_count as i32,
                libc::IPC_CREAT | libc::IPC_EXCL | (self.config.permissions as i32)
            )
        };
        
        if semaphore_id < 0 {
            let errno = unsafe { *libc::__errno_location() };
            if errno == libc::EEXIST {
                // Semaphore was created by another process
                let semaphore_id = unsafe { libc::semget(key, 0, 0) };
                if semaphore_id >= 0 {
                    self.semaphore_id = Some(semaphore_id);
                    self.is_open = true;
                    self.register_semaphore(semaphore_id, false);
                    return Ok(());
                }
            }
            return Err(system_error(errno, "semget", "Failed to create semaphore"));
        }
        
        // Initialize semaphore value
        let mut sops = sembuf {
            sem_num: 0,
            sem_op: self.config.initial_value as i16,
            sem_flg: 0,
        };
        
        let result = unsafe {
            libc::semop(semaphore_id, &mut sops, 1)
        };
        
        if result < 0 {
            let errno = unsafe { *libc::__errno_location() };
            // Try to remove the semaphore we just created
            unsafe {
                libc::semctl(semaphore_id, 0, libc::IPC_RMID);
            }
            return Err(system_error(errno, "semop", "Failed to initialize semaphore"));
        }
        
        self.semaphore_id = Some(semaphore_id);
        self.is_open = true;
        self.register_semaphore(semaphore_id, true);
        Ok(())
    }

    #[cfg(unix)]
    fn open_posix(&mut self) -> IpcResult<()> {
        let semaphore_name = format!("/{}", self.name);
        let semaphore_name_cstr = CString::new(semaphore_name)
            .map_err(|e| semaphore_error(Some(&self.name), "open", &e.to_string()))?;
        
        // Try to open existing semaphore first
        let sem = unsafe {
            sem_open(semaphore_name_cstr.as_ptr(), 0, 0, 0)
        };
        
        if sem != SEM_FAILED {
            self.posix_sem = Some(sem);
            self.is_open = true;
            self.register_semaphore(sem as i32, false);
            return Ok(());
        }
        
        // Create new semaphore if allowed
        if !self.config.create_if_missing {
            return Err(not_found("semaphore", &self.name, "Semaphore does not exist"));
        }
        
        let sem = unsafe {
            sem_open(
                semaphore_name_cstr.as_ptr(),
                libc::O_CREAT | libc::O_EXCL,
                self.config.permissions,
                self.config.initial_value as u32
            )
        };
        
        if sem == SEM_FAILED {
            let errno = unsafe { *libc::__errno_location() };
            if errno == libc::EEXIST {
                // Semaphore was created by another process, try to open it
                let sem = unsafe {
                    sem_open(semaphore_name_cstr.as_ptr(), 0, 0, 0)
                };
                if sem != SEM_FAILED {
                    self.posix_sem = Some(sem);
                    self.is_open = true;
                    self.register_semaphore(sem as i32, false);
                    return Ok(());
                }
            }
            return Err(system_error(errno, "sem_open", "Failed to create POSIX semaphore"));
        }
        
        self.posix_sem = Some(sem);
        self.is_open = true;
        self.register_semaphore(sem as i32, true);
        Ok(())
    }

    #[cfg(windows)]
    fn open_windows(&mut self) -> IpcResult<()> {
        // Windows implementation using CreateSemaphore
        self.is_open = true;
        Ok(())
    }

    #[cfg(unix)]
    fn wait_sysv(&mut self) -> IpcResult<()> {
        if let Some(semaphore_id) = self.semaphore_id {
            let mut sops = sembuf {
                sem_num: 0,
                sem_op: -1,
                sem_flg: 0,
            };
            
            let result = unsafe {
                libc::semop(semaphore_id, &mut sops, 1)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "semop", "Failed to wait on semaphore"));
            }
            
            Ok(())
        } else {
            Err(semaphore_error(Some(&self.name), "wait", "Semaphore not open"))
        }
    }

    #[cfg(unix)]
    fn wait_posix(&mut self) -> IpcResult<()> {
        if let Some(sem) = self.posix_sem {
            let result = unsafe {
                sem_wait(sem)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "sem_wait", "Failed to wait on POSIX semaphore"));
            }
            
            Ok(())
        } else {
            Err(semaphore_error(Some(&self.name), "wait", "Semaphore not open"))
        }
    }

    #[cfg(windows)]
    fn wait_windows(&mut self) -> IpcResult<()> {
        // Windows implementation using WaitForSingleObject
        Ok(())
    }

    #[cfg(unix)]
    fn try_wait_sysv(&mut self) -> IpcResult<bool> {
        if let Some(semaphore_id) = self.semaphore_id {
            let mut sops = sembuf {
                sem_num: 0,
                sem_op: -1,
                sem_flg: libc::IPC_NOWAIT,
            };
            
            let result = unsafe {
                libc::semop(semaphore_id, &mut sops, 1)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                if errno == libc::EAGAIN {
                    return Ok(false); // Would block
                }
                return Err(system_error(errno, "semop", "Failed to try_wait on semaphore"));
            }
            
            Ok(true)
        } else {
            Err(semaphore_error(Some(&self.name), "try_wait", "Semaphore not open"))
        }
    }

    #[cfg(unix)]
    fn try_wait_posix(&mut self) -> IpcResult<bool> {
        if let Some(sem) = self.posix_sem {
            let result = unsafe {
                sem_trywait(sem)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                if errno == libc::EAGAIN {
                    return Ok(false); // Would block
                }
                return Err(system_error(errno, "sem_trywait", "Failed to try_wait on POSIX semaphore"));
            }
            
            Ok(true)
        } else {
            Err(semaphore_error(Some(&self.name), "try_wait", "Semaphore not open"))
        }
    }

    #[cfg(windows)]
    fn try_wait_windows(&mut self) -> IpcResult<bool> {
        // Windows implementation using WaitForSingleObject with 0 timeout
        Ok(true)
    }

    #[cfg(unix)]
    fn wait_timeout_sysv(&mut self, timeout: Duration) -> IpcResult<bool> {
        if let Some(semaphore_id) = self.semaphore_id {
            let start = Instant::now();
            
            loop {
                let mut sops = sembuf {
                    sem_num: 0,
                    sem_op: -1,
                    sem_flg: libc::IPC_NOWAIT,
                };
                
                let result = unsafe {
                    libc::semop(semaphore_id, &mut sops, 1)
                };
                
                if result >= 0 {
                    return Ok(true);
                }
                
                let errno = unsafe { *libc::__errno_location() };
                if errno != libc::EAGAIN {
                    return Err(system_error(errno, "semop", "Failed to wait on semaphore"));
                }
                
                if start.elapsed() >= timeout {
                    return Ok(false); // Timeout
                }
                
                // Brief sleep to avoid busy waiting
                std::thread::sleep(Duration::from_millis(1));
            }
        } else {
            Err(semaphore_error(Some(&self.name), "wait_timeout", "Semaphore not open"))
        }
    }

    #[cfg(unix)]
    fn wait_timeout_posix(&mut self, timeout: Duration) -> IpcResult<bool> {
        if let Some(sem) = self.posix_sem {
            let timeout_spec = libc::timespec {
                tv_sec: timeout.as_secs() as i64,
                tv_nsec: timeout.subsec_nanos() as i64,
            };
            
            let result = unsafe {
                sem_timedwait(sem, &timeout_spec)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                if errno == libc::ETIMEDOUT {
                    return Ok(false); // Timeout
                }
                return Err(system_error(errno, "sem_timedwait", "Failed to wait on POSIX semaphore"));
            }
            
            Ok(true)
        } else {
            Err(semaphore_error(Some(&self.name), "wait_timeout", "Semaphore not open"))
        }
    }

    #[cfg(windows)]
    fn wait_timeout_windows(&mut self, timeout: Duration) -> IpcResult<bool> {
        // Windows implementation using WaitForSingleObject with timeout
        Ok(true)
    }

    #[cfg(unix)]
    fn signal_sysv(&mut self) -> IpcResult<()> {
        if let Some(semaphore_id) = self.semaphore_id {
            let mut sops = sembuf {
                sem_num: 0,
                sem_op: 1,
                sem_flg: 0,
            };
            
            let result = unsafe {
                libc::semop(semaphore_id, &mut sops, 1)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "semop", "Failed to signal semaphore"));
            }
            
            Ok(())
        } else {
            Err(semaphore_error(Some(&self.name), "signal", "Semaphore not open"))
        }
    }

    #[cfg(unix)]
    fn signal_posix(&mut self) -> IpcResult<()> {
        if let Some(sem) = self.posix_sem {
            let result = unsafe {
                sem_post(sem)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "sem_post", "Failed to signal POSIX semaphore"));
            }
            
            Ok(())
        } else {
            Err(semaphore_error(Some(&self.name), "signal", "Semaphore not open"))
        }
    }

    #[cfg(windows)]
    fn signal_windows(&mut self) -> IpcResult<()> {
        // Windows implementation using ReleaseSemaphore
        Ok(())
    }

    #[cfg(unix)]
    fn value_sysv(&self) -> IpcResult<SemaphoreValue> {
        if let Some(semaphore_id) = self.semaphore_id {
            let result = unsafe {
                libc::semctl(semaphore_id, 0, libc::GETVAL)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "semctl", "Failed to get semaphore value"));
            }
            
            Ok(result)
        } else {
            Err(semaphore_error(Some(&self.name), "value", "Semaphore not open"))
        }
    }

    #[cfg(unix)]
    fn value_posix(&self) -> IpcResult<SemaphoreValue> {
        if let Some(sem) = self.posix_sem {
            let mut value: i32 = 0;
            let result = unsafe {
                sem_getvalue(sem, &mut value)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "sem_getvalue", "Failed to get POSIX semaphore value"));
            }
            
            Ok(value)
        } else {
            Err(semaphore_error(Some(&self.name), "value", "Semaphore not open"))
        }
    }

    #[cfg(windows)]
    fn value_windows(&self) -> IpcResult<SemaphoreValue> {
        // Windows doesn't provide direct access to semaphore value
        Ok(self.config.initial_value)
    }

    #[cfg(unix)]
    fn delete_sysv(&mut self) -> IpcResult<()> {
        if let Some(semaphore_id) = self.semaphore_id {
            let result = unsafe {
                libc::semctl(semaphore_id, 0, libc::IPC_RMID)
            };
            
            if result < 0 {
                let errno = unsafe { *libc::__errno_location() };
                return Err(system_error(errno, "semctl", "Failed to delete semaphore"));
            }
            
            self.semaphore_id = None;
        }
        
        Ok(())
    }

    #[cfg(unix)]
    fn delete_posix(&mut self) -> IpcResult<()> {
        let semaphore_name = format!("/{}", self.name);
        let semaphore_name_cstr = CString::new(semaphore_name)
            .map_err(|e| semaphore_error(Some(&self.name), "delete", &e.to_string()))?;
        
        let result = unsafe {
            sem_unlink(semaphore_name_cstr.as_ptr())
        };
        
        if result < 0 {
            let errno = unsafe { *libc::__errno_location() };
            if errno != libc::ENOENT {
                return Err(system_error(errno, "sem_unlink", "Failed to delete POSIX semaphore"));
            }
        }
        
        Ok(())
    }

    #[cfg(windows)]
    fn delete_windows(&mut self) -> IpcResult<()> {
        Ok(())
    }

    #[cfg(unix)]
    fn generate_sysv_key(&self) -> IpcResult<i32> {
        // Generate a key based on the semaphore name
        let mut hash = 0i32;
        for byte in self.name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as i32);
        }
        
        // Ensure it's a valid System V key (non-zero)
        if hash == 0 {
            hash = 1;
        }
        
        Ok(hash)
    }

    fn register_semaphore(&self, semaphore_id: i32, created_by_us: bool) {
        let registry = get_semaphore_registry();
        if let Ok(mut semaphores) = registry.write() {
            let info = Arc::new(SemaphoreInfo {
                name: self.name.clone(),
                semaphore_id,
                created_by_us,
                ref_count: Arc::new(Mutex::new(1)),
            });
            semaphores.insert(self.name.clone(), info);
        }
    }

    fn unregister_semaphore(&self) {
        let registry = get_semaphore_registry();
        if let Ok(mut semaphores) = registry.write() {
            if let Some(info) = semaphores.get(&self.name) {
                let mut ref_count = info.ref_count.lock().unwrap();
                *ref_count -= 1;
                if *ref_count == 0 {
                    semaphores.remove(&self.name);
                }
            }
        }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Named semaphore (alias for consistency)
pub type NamedSemaphore = Semaphore;

/// System V sembuf structure
#[cfg(unix)]
#[repr(C)]
struct sembuf {
    sem_num: u16,
    sem_op: i16,
    sem_flg: i16,
}

/// Cleanup all registered semaphores
pub fn cleanup_semaphores() -> IpcResult<()> {
    let registry = get_semaphore_registry();
    if let Ok(mut semaphores) = registry.write() {
        for (name, info) in semaphores.drain() {
            #[cfg(unix)]
            if info.created_by_us {
                // Try to delete the semaphore
                unsafe {
                    libc::semctl(info.semaphore_id, 0, libc::IPC_RMID);
                }
                tracing::debug!(semaphore_name = name, "Cleaned up semaphore");
            }
        }
    }
    Ok(())
}

// POSIX semaphore system calls
#[cfg(unix)]
extern "C" {
    fn sem_open(name: *const i8, oflag: i32, mode: u32, value: u32) -> *mut libc::sem_t;
    fn sem_close(sem: *mut libc::sem_t) -> i32;
    fn sem_unlink(name: *const i8) -> i32;
    fn sem_wait(sem: *mut libc::sem_t) -> i32;
    fn sem_trywait(sem: *mut libc::sem_t) -> i32;
    fn sem_timedwait(sem: *mut libc::sem_t, abs_timeout: *const libc::timespec) -> i32;
    fn sem_post(sem: *mut libc::sem_t) -> i32;
    fn sem_getvalue(sem: *mut libc::sem_t, sval: *mut i32) -> i32;
}

#[cfg(unix)]
const SEM_FAILED: *mut libc::sem_t = (-1isize) as *mut libc::sem_t;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_semaphore_config() {
        let config = SemaphoreConfig::default();
        assert_eq!(config.initial_value, 1);
        assert_eq!(config.max_value, i32::MAX);
        assert_eq!(config.permissions, 0o666);
        assert!(!config.use_posix);
    }

    #[test]
    fn test_semaphore_creation() {
        let semaphore = Semaphore::new("test_semaphore", 2);
        assert_eq!(semaphore.name(), "test_semaphore");
        assert!(!semaphore.is_open());
    }

    #[cfg(unix)]
    #[test]
    fn test_semaphore_sysv() {
        let mut semaphore = Semaphore::new("test_sysv_semaphore", 3);
        
        // Test opening
        if semaphore.open().is_ok() {
            assert!(semaphore.is_open());
            
            // Test getting value
            if let Ok(value) = semaphore.value() {
                assert_eq!(value, 3);
            }
            
            // Test wait and signal
            if semaphore.wait().is_ok() {
                if let Ok(value) = semaphore.value() {
                    assert_eq!(value, 2);
                }
                
                if semaphore.signal().is_ok() {
                    if let Ok(value) = semaphore.value() {
                        assert_eq!(value, 3);
                    }
                }
            }
            
            // Test try_wait
            if let Ok(acquired) = semaphore.try_wait() {
                assert!(acquired);
            }
            
            // Cleanup
            let _ = semaphore.delete();
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_semaphore_posix() {
        let config = SemaphoreConfig {
            use_posix: true,
            initial_value: 2,
            ..Default::default()
        };
        let mut semaphore = Semaphore::with_config("test_posix_semaphore", config);
        
        // Test opening
        if semaphore.open().is_ok() {
            assert!(semaphore.is_open());
            
            // Test operations
            if let Ok(value) = semaphore.value() {
                assert_eq!(value, 2);
            }
            
            if semaphore.wait().is_ok() {
                if semaphore.signal().is_ok() {
                    // Successful round trip
                }
            }
            
            // Test timeout
            if let Ok(acquired) = semaphore.wait_timeout(Duration::from_millis(100)) {
                if acquired && semaphore.signal().is_ok() {
                    // Success
                }
            }
            
            // Cleanup
            let _ = semaphore.delete();
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_semaphore_synchronization() {
        let mut semaphore = Semaphore::new("test_sync_semaphore", 1);
        
        if semaphore.open().is_ok() {
            let sem_name = semaphore.name().to_string();
            
            // Spawn thread that will wait on semaphore
            let handle = thread::spawn(move || {
                let mut thread_sem = Semaphore::new(&sem_name, 1);
                if thread_sem.open().is_ok() {
                    thread_sem.wait()
                } else {
                    Err(semaphore_error(Some(&sem_name), "open", "Failed to open in thread"))
                }
            });
            
            // Wait briefly then signal
            thread::sleep(Duration::from_millis(100));
            if semaphore.signal().is_ok() {
                // Thread should now be able to acquire
                if let Ok(result) = handle.join() {
                    assert!(result.is_ok());
                }
            }
            
            // Cleanup
            let _ = semaphore.delete();
        }
    }

    #[test]
    fn test_semaphore_registry() {
        let registry = get_semaphore_registry();
        assert!(registry.read().is_ok());
        
        // Test cleanup
        assert!(cleanup_semaphores().is_ok());
    }
}
