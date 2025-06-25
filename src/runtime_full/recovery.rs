/// Recovery mechanism for CURSED panic handling
///
/// Provides panic catching, recovery scopes, and panic-to-error conversion
/// utilities for handling recoverable errors in CURSED programs.

use crate::error::CursedError;
use crate::runtime::panic::{
    get_panic_runtime
// };
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
    /// Thread where scope is active
    /// Recovery configuration
    /// Start time for timeout tracking
    /// Whether scope is currently active
    /// Nested recovery depth
/// Configuration for recovery operations
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// Maximum time to wait for recovery
    /// Whether to convert panics to errors
    /// Whether to log recovery attempts
    /// Maximum recovery attempts
    /// Whether to propagate unrecoverable panics
    /// Categories of panics that can be recovered
impl Default for RecoveryConfig {
    fn default() -> Self {
        RecoveryConfig {
            recoverable_categories: vec![
        }
    }
impl RecoveryScope {
    /// Create a new recovery scope
    pub fn new(scope_id: String, config: RecoveryConfig) -> Self {
        RecoveryScope {
        }
    }

    /// Activate the recovery scope
    pub fn activate(&mut self) {
        self.active = true;
        self.start_time = Instant::now();
    /// Deactivate the recovery scope
    pub fn deactivate(&mut self) {
        self.active = false;
    /// Check if the scope has timed out
    pub fn has_timed_out(&self) -> bool {
        self.start_time.elapsed() > self.config.timeout
    /// Check if a panic category is recoverable in this scope
    pub fn is_recoverable(&self, category: &PanicCategory) -> bool {
        self.config.recoverable_categories.contains(category)
    /// Increment the recovery depth
    pub fn enter_nested(&mut self) {
        self.depth += 1;
    /// Decrement the recovery depth
    pub fn exit_nested(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
/// Recovery manager for handling panic recovery operations
#[derive(Debug)]
pub struct RecoveryManager {
    /// Active recovery scopes by thread
    /// Recovery statistics
    /// Default recovery configuration
/// Statistics for recovery operations
#[derive(Debug, Default, Clone)]
pub struct RecoveryStatistics {
    /// Total recovery attempts
    /// Successful recoveries
    /// Failed recoveries
    /// Recoveries by category
    /// Average recovery time
    /// Timeouts during recovery
impl RecoveryManager {
    /// Create a new recovery manager
    pub fn new() -> Self {
        RecoveryManager {
        }
    }

    /// Create recovery manager with custom default config
    pub fn with_config(config: RecoveryConfig) -> Self {
        RecoveryManager {
        }
    }

    /// Enter a new recovery scope
    pub fn enter_scope(&self, scope_id: String, config: Option<RecoveryConfig>) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        let config = config.unwrap_or_else(|| self.default_config.clone());
        
        let mut scope = RecoveryScope::new(scope_id, config);
        scope.activate();

        let mut scopes = self.scopes.lock()
            .map_err(|_| CursedError::Runtime("Failed to acquire recovery scopes lock".to_string()))?;
        
        let thread_scopes = scopes.entry(thread_id).or_insert_with(Vec::new);
        
        // Set nested depth
        scope.depth = thread_scopes.len();
        thread_scopes.push(scope);

        Ok(())
    /// Exit the current recovery scope
    pub fn exit_scope(&self) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        
        let mut scopes = self.scopes.lock()
            .map_err(|_| CursedError::Runtime("Failed to acquire recovery scopes lock".to_string()))?;
        
        if let Some(thread_scopes) = scopes.get_mut(&thread_id) {
            if let Some(mut scope) = thread_scopes.pop() {
                scope.deactivate();
                return Ok(Some(scope.scope_id));
            }
        }
        
        Ok(None)
    /// Get the current recovery scope
    pub fn current_scope(&self) -> crate::error::Result<()> {
        let thread_id = thread::current().id();
        
        let scopes = self.scopes.lock()
            .map_err(|_| CursedError::Runtime("Failed to acquire recovery scopes lock".to_string()))?;
        
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
    /// Get recovery statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        self.stats.read()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access recovery statistics".to_string()))
    /// Update recovery statistics
    fn update_stats<F>(&self, updater: F) -> crate::error::Result<()>
    where
    {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire recovery statistics lock".to_string()))?;
        
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
pub fn initialize_recovery_manager() -> crate::error::Result<()> {
    let manager = Arc::new(RecoveryManager::new());
    
    RECOVERY_MANAGER.set(manager)
        .map_err(|_| CursedError::Runtime("Failed to initialize recovery manager".to_string()))?;
    
    Ok(())
/// Get the global recovery manager
pub fn get_recovery_manager() -> Option<&'static Arc<RecoveryManager>> {
    RECOVERY_MANAGER.get()
/// Catch panic and attempt recovery
pub fn catch_panic<T, F>(operation: F) -> crate::error::Result<()>
where
{
    catch_panic_with_config(operation, None)
/// Catch panic with custom recovery configuration
pub fn catch_panic_with_config<T, F>(
) -> crate::error::Result<()>
where
{
    let start_time = Instant::now();
    let scope_id = format!("recovery_{}", start_time.elapsed().as_nanos());
    
    // Enter recovery scope
    if let Some(manager) = get_recovery_manager() {
        manager.enter_scope(scope_id.clone(), config.clone())?;
    // Get panic runtime for integrated recovery
    let panic_runtime = get_panic_runtime();
    
    let result = if let Some(runtime) = panic_runtime {
        // Use integrated panic runtime recovery
        runtime.recover(operation)
    } else {
        // Fallback to standard panic catching
        match panic::catch_unwind(AssertUnwindSafe(operation)) {
            Err(panic_payload) => {
                let message = extract_panic_message(&panic_payload);
                Err(CursedError::Recovery {
                })
            }
        }
    
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
    result
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
pub fn panic_to_error(panic_info: &CursedPanicInfo) -> CursedError {
    CursedError::Panic {
    }
}

/// Convert error to recovery action
pub fn error_to_recovery_action(error: CursedError) -> RecoveryAction {
    match error {
        CursedError::Panic { recoverable: true, .. } => {
            RecoveryAction::Continue(error)
        }
        CursedError::Panic { recoverable: false, .. } => {
            RecoveryAction::TerminateGoroutine
        }
    }
}

/// Check if an error is recoverable
pub fn is_recoverable_error(error: &CursedError) -> bool {
    match error {
    }
}

/// Recovery scope guard for RAII-style scope management
pub struct RecoveryScopeGuard {
impl RecoveryScopeGuard {
    /// Create a new recovery scope guard
    pub fn new(scope_id: String, config: Option<RecoveryConfig>) -> crate::error::Result<()> {
        let manager = get_recovery_manager().cloned();
        
        if let Some(ref mgr) = manager {
            mgr.enter_scope(scope_id.clone(), config)?;
        Ok(RecoveryScopeGuard {
        })
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
/// Macro for easy recovery scope creation
#[macro_export]
macro_rules! with_recovery {
    ($scope_id:expr, $code:block) => {
        {
            let _guard = $crate::runtime::recovery::RecoveryScopeGuard::new(
                None
            )?;
            $code
        }
    
    ($scope_id:expr, $config:expr, $code:block) => {
        {
            let _guard = $crate::runtime::recovery::RecoveryScopeGuard::new(
                Some($config)
            )?;
            $code
        }
// FFI functions for LLVM integration

/// Enter a recovery scope from compiled code
#[no_mangle]
pub extern "C" fn cursed_enter_recovery_scope(
) -> u8 {
    if scope_id_ptr.is_null() || scope_id_len == 0 {
        return 0; // Failed
    let scope_id = unsafe {
        let slice = std::slice::from_raw_parts(scope_id_ptr, scope_id_len);
        String::from_utf8_lossy(slice).to_string()
    
    let config = RecoveryConfig {
        ..RecoveryConfig::default()
    
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

