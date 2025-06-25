/// Recovery mechanism for CURSED panic handling
///
/// Provides panic catching, recovery scopes, and panic-to-error conversion
/// utilities for handling recoverable errors in CURSED programs.

use crate::error_types::Error;
use crate::runtime::panic::{
    PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction,
    get_panic_runtime
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};
use std::panic::{self, AssertUnwindSafe};
use std::any::Any;

/// Recovery scope for managing panic boundaries
#[derive(Debug, Clone)]
pub struct RecoveryScope {
    /// Scope ID for tracking
    pub scope_id: String,
    /// Thread where scope is active
    pub thread_id: ThreadId,
    /// Recovery configuration
    pub config: RecoveryConfig,
    /// Start time for timeout tracking
    pub start_time: Instant,
    /// Whether scope is currently active
    pub active: bool,
    /// Nested recovery depth
    pub depth: usize,
}

/// Configuration for recovery operations
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// Maximum time to wait for recovery
    pub timeout: Duration,
    /// Whether to convert panics to errors
    pub convert_to_error: bool,
    /// Whether to log recovery attempts
    pub log_recovery: bool,
    /// Maximum recovery attempts
    pub max_attempts: u32,
    /// Whether to propagate unrecoverable panics
    pub propagate_unrecoverable: bool,
    /// Categories of panics that can be recovered
    pub recoverable_categories: Vec<PanicCategory>,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        RecoveryConfig {
            timeout: Duration::from_secs(30),
            convert_to_error: true,
            log_recovery: false,
            max_attempts: 3,
            propagate_unrecoverable: true,
            recoverable_categories: vec![
                PanicCategory::User,
                PanicCategory::TypeAssertion,
                PanicCategory::BoundsCheck,
                PanicCategory::Arithmetic,
                PanicCategory::Channel,
            ],
        }
    }
}

impl RecoveryScope {
    /// Create a new recovery scope
    pub fn new(scope_id: String, config: RecoveryConfig) -> Self {
        RecoveryScope {
            scope_id,
            thread_id: thread::current().id(),
            config,
            start_time: Instant::now(),
            active: false,
            depth: 0,
        }
    }

    /// Activate the recovery scope
    pub fn activate(&mut self) {
        self.active = true;
        self.start_time = Instant::now();
    }

    /// Deactivate the recovery scope
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Check if the scope has timed out
    pub fn has_timed_out(&self) -> bool {
        self.start_time.elapsed() > self.config.timeout
    }

    /// Check if a panic category is recoverable in this scope
    pub fn is_recoverable(&self, category: &PanicCategory) -> bool {
        self.config.recoverable_categories.contains(category)
    }

    /// Increment the recovery depth
    pub fn enter_nested(&mut self) {
        self.depth += 1;
    }

    /// Decrement the recovery depth
    pub fn exit_nested(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
}

/// Recovery manager for handling panic recovery operations
#[derive(Debug)]
pub struct RecoveryManager {
    /// Active recovery scopes by thread
    scopes: Arc<Mutex<HashMap<ThreadId, Vec<RecoveryScope>>>>,
    /// Recovery statistics
    stats: Arc<RwLock<RecoveryStatistics>>,
    /// Default recovery configuration
    default_config: RecoveryConfig,
}

/// Statistics for recovery operations
#[derive(Debug, Default, Clone)]
pub struct RecoveryStatistics {
    /// Total recovery attempts
    pub total_attempts: u64,
    /// Successful recoveries
    pub successful_recoveries: u64,
    /// Failed recoveries
    pub failed_recoveries: u64,
    /// Recoveries by category
    pub recoveries_by_category: HashMap<PanicCategory, u64>,
    /// Average recovery time
    pub average_recovery_time: Duration,
    /// Timeouts during recovery
    pub recovery_timeouts: u64,
}

impl RecoveryManager {
    /// Create a new recovery manager
    pub fn new() -> Self {
        RecoveryManager {
            scopes: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RecoveryStatistics::default())),
            default_config: RecoveryConfig::default(),
        }
    }

    /// Create recovery manager with custom default config
    pub fn with_config(config: RecoveryConfig) -> Self {
        RecoveryManager {
            scopes: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RecoveryStatistics::default())),
            default_config: config,
        }
    }

    /// Enter a new recovery scope
    pub fn enter_scope(&self, scope_id: String, config: Option<RecoveryConfig>) -> Result<(), Error> {
        let thread_id = thread::current().id();
        let config = config.unwrap_or_else(|| self.default_config.clone());
        
        let mut scope = RecoveryScope::new(scope_id, config);
        scope.activate();

        let mut scopes = self.scopes.lock()
            .map_err(|_| Error::Runtime("Failed to acquire recovery scopes lock".to_string()))?;
        
        let thread_scopes = scopes.entry(thread_id).or_insert_with(Vec::new);
        
        // Set nested depth
        scope.depth = thread_scopes.len();
        thread_scopes.push(scope);

        Ok(())
    }

    /// Exit the current recovery scope
    pub fn exit_scope(&self) -> Result<(), Error> {
        let thread_id = thread::current().id();
        
        let mut scopes = self.scopes.lock()
            .map_err(|_| Error::Runtime("Failed to acquire recovery scopes lock".to_string()))?;
        
        if let Some(thread_scopes) = scopes.get_mut(&thread_id) {
            if let Some(mut scope) = thread_scopes.pop() {
                scope.deactivate();
                return Ok(Some(scope.scope_id));
            }
        }
        
        Ok(None)
    }

    /// Get the current recovery scope
    pub fn current_scope(&self) -> Result<(), Error> {
        let thread_id = thread::current().id();
        
        let scopes = self.scopes.lock()
            .map_err(|_| Error::Runtime("Failed to acquire recovery scopes lock".to_string()))?;
        
        if let Some(thread_scopes) = scopes.get(&thread_id) {
            Ok(thread_scopes.last().cloned())
        } else {
            Ok(None)
        }
    }

    /// Check if currently in a recovery scope
    pub fn in_recovery_scope(&self) -> bool {
        let thread_id = thread::current().id();
        
        if let Ok(scopes) = self.scopes.lock() {
            if let Some(thread_scopes) = scopes.get(&thread_id) {
                return !thread_scopes.is_empty() && 
                       thread_scopes.last().map(|s| s.active).unwrap_or(false);
            }
        }
        
        false
    }

    /// Get recovery statistics
    pub fn get_statistics(&self) -> Result<(), Error> {
        self.stats.read()
            .map(|stats| stats.clone())
            .map_err(|_| Error::Runtime("Failed to access recovery statistics".to_string()))
    }

    /// Update recovery statistics
    fn update_stats<F>(&self, updater: F) -> Result<(), Error>
    where
        F: FnOnce(&mut RecoveryStatistics),
    {
        let mut stats = self.stats.write()
            .map_err(|_| Error::Runtime("Failed to acquire recovery statistics lock".to_string()))?;
        
        updater(&mut *stats);
        Ok(())
    }
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global recovery manager instance
static RECOVERY_MANAGER: std::sync::OnceLock<Arc<RecoveryManager>> = std::sync::OnceLock::new();

/// Initialize the global recovery manager
pub fn initialize_recovery_manager() -> Result<(), Error> {
    let manager = Arc::new(RecoveryManager::new());
    
    RECOVERY_MANAGER.set(manager)
        .map_err(|_| Error::Runtime("Failed to initialize recovery manager".to_string()))?;
    
    Ok(())
}

/// Get the global recovery manager
pub fn get_recovery_manager() -> Option<&'static Arc<RecoveryManager>> {
    RECOVERY_MANAGER.get()
}

/// Catch panic and attempt recovery
pub fn catch_panic<T, F>(operation: F) -> Result<(), Error>
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
{
    catch_panic_with_config(operation, None)
}

/// Catch panic with custom recovery configuration
pub fn catch_panic_with_config<T, F>(
    operation: F,
    config: Option<RecoveryConfig>,
) -> Result<(), Error>
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
{
    let start_time = Instant::now();
    let scope_id = format!("recovery_{}", start_time.elapsed().as_nanos());
    
    // Enter recovery scope
    if let Some(manager) = get_recovery_manager() {
        manager.enter_scope(scope_id.clone(), config.clone())?;
    }
    
    // Get panic runtime for integrated recovery
    let panic_runtime = get_panic_runtime();
    
    let result = if let Some(runtime) = panic_runtime {
        // Use integrated panic runtime recovery
        runtime.recover(operation)
    } else {
        // Fallback to standard panic catching
        match panic::catch_unwind(AssertUnwindSafe(operation)) {
            Ok(value) => Ok(value),
            Err(panic_payload) => {
                let message = extract_panic_message(&panic_payload);
                Err(Error::Recovery {
                    message: format!("Panic caught: {}", message),
                    recovery_attempts: 1,
                    source_location: None,
                })
            }
        }
    };
    
    let recovery_time = start_time.elapsed();
    
    // Update statistics
    if let Some(manager) = get_recovery_manager() {
        let _ = manager.update_stats(|stats| {
            stats.total_attempts += 1;
            
            match &result {
                Ok(_) => {
                    stats.successful_recoveries += 1;
                    // Update average recovery time
                    let total_time = stats.average_recovery_time.as_nanos() as u64 * 
                                   stats.successful_recoveries.saturating_sub(1) +
                                   recovery_time.as_nanos() as u64;
                    stats.average_recovery_time = Duration::from_nanos(
                        total_time / stats.successful_recoveries
                    );
                }
                Err(_) => {
                    stats.failed_recoveries += 1;
                }
            }
        });
        
        // Exit recovery scope
        let _ = manager.exit_scope();
    }
    
    result
}

/// Extract panic message from panic payload
fn extract_panic_message(panic_payload: &Box<dyn Any + Send>) -> String {
    if let Some(s) = panic_payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic message".to_string()
    }
}

/// Convert panic to error
pub fn panic_to_error(panic_info: &CursedPanicInfo) -> Error {
    Error::Panic {
        message: panic_info.message.clone(),
        panic_id: Some(panic_info.panic_id),
        recoverable: panic_info.severity != PanicSeverity::Fatal,
        source_location: panic_info.source_location.clone(),
    }
}

/// Convert error to recovery action
pub fn error_to_recovery_action(error: Error) -> RecoveryAction {
    match error {
        Error::Panic { recoverable: true, .. } => {
            RecoveryAction::Continue(error)
        }
        Error::Panic { recoverable: false, .. } => {
            RecoveryAction::TerminateGoroutine
        }
        _ => RecoveryAction::Continue(error),
    }
}

/// Check if an error is recoverable
pub fn is_recoverable_error(error: &Error) -> bool {
    match error {
        Error::Panic { recoverable, .. } => *recoverable,
        Error::Recovery { .. } => true,
        Error::Runtime(_) => true,
        Error::Parse(_) => false,
        Error::Compile(_) => false,
        Error::Io(_) => true,
        _ => false,
    }
}

/// Recovery scope guard for RAII-style scope management
pub struct RecoveryScopeGuard {
    scope_id: Option<String>,
    manager: Option<Arc<RecoveryManager>>,
}

impl RecoveryScopeGuard {
    /// Create a new recovery scope guard
    pub fn new(scope_id: String, config: Option<RecoveryConfig>) -> Result<(), Error> {
        let manager = get_recovery_manager().cloned();
        
        if let Some(ref mgr) = manager {
            mgr.enter_scope(scope_id.clone(), config)?;
        }
        
        Ok(RecoveryScopeGuard {
            scope_id: Some(scope_id),
            manager,
        })
    }
    
    /// Check if currently in recovery scope
    pub fn in_scope(&self) -> bool {
        self.manager.as_ref()
            .map(|mgr| mgr.in_recovery_scope())
            .unwrap_or(false)
    }
}

impl Drop for RecoveryScopeGuard {
    fn drop(&mut self) {
        if let (Some(_), Some(ref manager)) = (&self.scope_id, &self.manager) {
            let _ = manager.exit_scope();
        }
    }
}

/// Macro for easy recovery scope creation
#[macro_export]
macro_rules! with_recovery {
    ($scope_id:expr, $code:block) => {
        {
            let _guard = $crate::runtime::recovery::RecoveryScopeGuard::new(
                $scope_id.to_string(), 
                None
            )?;
            $code
        }
    };
    
    ($scope_id:expr, $config:expr, $code:block) => {
        {
            let _guard = $crate::runtime::recovery::RecoveryScopeGuard::new(
                $scope_id.to_string(), 
                Some($config)
            )?;
            $code
        }
    };
}

// FFI functions for LLVM integration

/// Enter a recovery scope from compiled code
#[no_mangle]
pub extern "C" fn cursed_enter_recovery_scope(
    scope_id_ptr: *const u8,
    scope_id_len: usize,
    timeout_secs: u32,
) -> u8 {
    if scope_id_ptr.is_null() || scope_id_len == 0 {
        return 0; // Failed
    }
    
    let scope_id = unsafe {
        let slice = std::slice::from_raw_parts(scope_id_ptr, scope_id_len);
        String::from_utf8_lossy(slice).to_string()
    };
    
    let config = RecoveryConfig {
        timeout: Duration::from_secs(timeout_secs as u64),
        ..RecoveryConfig::default()
    };
    
    if let Some(manager) = get_recovery_manager() {
        if manager.enter_scope(scope_id, Some(config)).is_ok() {
            1 // Success
        } else {
            0 // Failed
        }
    } else {
        0 // Manager not available
    }
}

/// Exit current recovery scope from compiled code
#[no_mangle]
pub extern "C" fn cursed_exit_recovery_scope() -> u8 {
    if let Some(manager) = get_recovery_manager() {
        if manager.exit_scope().is_ok() {
            1 // Success
        } else {
            0 // Failed
        }
    } else {
        0 // Manager not available
    }
}

/// Check if in recovery scope from compiled code
#[no_mangle]
pub extern "C" fn cursed_in_recovery_scope() -> u8 {
    if let Some(manager) = get_recovery_manager() {
        if manager.in_recovery_scope() {
            1 // In scope
        } else {
            0 // Not in scope
        }
    } else {
        0 // Manager not available
    }
}

/// Attempt to recover from panic in compiled code
#[no_mangle]
pub extern "C" fn cursed_attempt_recovery() -> u8 {
    if let Some(panic_runtime) = get_panic_runtime() {
        if panic_runtime.is_in_recovery() {
            1 // Recovery possible
        } else {
            0 // No recovery needed/possible
        }
    } else {
        0 // Runtime not available
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_recovery_scope_creation() {
        let config = RecoveryConfig::default();
        let scope = RecoveryScope::new("test_scope".to_string(), config);
        
        assert_eq!(scope.scope_id, "test_scope");
        assert!(!scope.active);
        assert_eq!(scope.depth, 0);
    }

    #[test]
    fn test_recovery_scope_activation() {
        let config = RecoveryConfig::default();
        let mut scope = RecoveryScope::new("test_scope".to_string(), config);
        
        scope.activate();
        assert!(scope.active);
        
        scope.deactivate();
        assert!(!scope.active);
    }

    #[test]
    fn test_recovery_manager_creation() {
        let manager = RecoveryManager::new();
        assert!(!manager.in_recovery_scope());
    }

    #[test]
    fn test_recovery_scope_management() {
        let manager = RecoveryManager::new();
        
        assert!(manager.enter_scope("test1".to_string(), None).is_ok());
        assert!(manager.in_recovery_scope());
        
        assert!(manager.enter_scope("test2".to_string(), None).is_ok());
        assert!(manager.in_recovery_scope());
        
        let exited = manager.exit_scope().unwrap();
        assert_eq!(exited, Some("test2".to_string()));
        assert!(manager.in_recovery_scope());
        
        let exited = manager.exit_scope().unwrap();
        assert_eq!(exited, Some("test1".to_string()));
        assert!(!manager.in_recovery_scope());
    }

    #[test]
    fn test_catch_panic_success() {
        let result = catch_panic(|| {
            42
        });
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_catch_panic_failure() {
        let result = catch_panic(|| -> i32 {
            panic!("Test panic");
        });
        
        assert!(result.is_err());
        if let Err(Error::Recovery { message, .. }) = result {
            assert!(message.contains("Test panic"));
        }
    }

    #[test]
    fn test_recovery_scope_guard() {
        let guard = RecoveryScopeGuard::new("test_guard".to_string(), None);
        
        // Note: Without global manager initialized, this won't enter scope
        // but it shouldn't crash
        assert!(guard.is_ok());
    }

    #[test]
    fn test_panic_to_error_conversion() {
        let panic_info = CursedPanicInfo::new(
            "Test panic".to_string(),
            PanicSeverity::Recoverable,
            PanicCategory::User,
        );
        
        let error = panic_to_error(&panic_info);
        
        if let Error::Panic { message, recoverable, .. } = error {
            assert_eq!(message, "Test panic");
            assert!(recoverable);
        } else {
            panic!("Wrong error type");
        }
    }

    #[test]
    fn test_error_recoverability() {
        let recoverable_error = Error::Panic {
            message: "Recoverable".to_string(),
            panic_id: Some(1),
            recoverable: true,
            source_location: None,
        };
        
        let unrecoverable_error = Error::Panic {
            message: "Fatal".to_string(),
            panic_id: Some(2),
            recoverable: false,
            source_location: None,
        };
        
        assert!(is_recoverable_error(&recoverable_error));
        assert!(!is_recoverable_error(&unrecoverable_error));
        
        let compile_error = Error::Compile("Compile error".to_string());
        assert!(!is_recoverable_error(&compile_error));
        
        let runtime_error = Error::Runtime("Runtime error".to_string());
        assert!(is_recoverable_error(&runtime_error));
    }

    #[test]
    fn test_error_to_recovery_action() {
        let recoverable_panic = Error::Panic {
            message: "Recoverable".to_string(),
            panic_id: Some(1),
            recoverable: true,
            source_location: None,
        };
        
        let action = error_to_recovery_action(recoverable_panic);
        match action {
            RecoveryAction::Continue(_) => (), // Expected
            _ => panic!("Wrong recovery action"),
        }
        
        let fatal_panic = Error::Panic {
            message: "Fatal".to_string(),
            panic_id: Some(2),
            recoverable: false,
            source_location: None,
        };
        
        let action = error_to_recovery_action(fatal_panic);
        match action {
            RecoveryAction::TerminateGoroutine => (), // Expected
            _ => panic!("Wrong recovery action"),
        }
    }

    #[test]
    fn test_recovery_config_defaults() {
        let config = RecoveryConfig::default();
        
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(config.convert_to_error);
        assert!(!config.log_recovery);
        assert_eq!(config.max_attempts, 3);
        assert!(config.propagate_unrecoverable);
        assert!(!config.recoverable_categories.is_empty());
    }

    #[test]
    fn test_recovery_statistics() {
        let manager = RecoveryManager::new();
        let stats = manager.get_statistics().unwrap();
        
        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.successful_recoveries, 0);
        assert_eq!(stats.failed_recoveries, 0);
        assert_eq!(stats.recovery_timeouts, 0);
    }
}
