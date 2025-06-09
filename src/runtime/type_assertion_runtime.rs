//! Runtime system for type assertion operations
//!
//! This module provides runtime support for type assertions, including panic mechanisms,
//! runtime type safety checks, and integration with the enhanced error system.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::panic;

use tracing::{instrument, error, warn, debug, info};

use crate::error::type_assertion_error::{TypeAssertionError, helpers};
use crate::error::{Error, SourceLocation};
use crate::error_enhanced::{CursedError, ErrorKind};

/// Runtime type information for interface values
#[derive(Debug, Clone)]
pub struct RuntimeTypeInfo {
    /// Unique type identifier
    pub type_id: u64,
    /// Human-readable type name
    pub type_name: String,
    /// Size of the type in bytes
    pub size: usize,
    /// Whether the type implements specific interfaces
    pub implements: Vec<String>,
    /// Optional debug information
    pub debug_info: Option<String>,
}

/// Runtime context for type assertion operations
#[derive(Debug)]
pub struct TypeAssertionRuntime {
    /// Registry of known types
    type_registry: Arc<RwLock<HashMap<u64, RuntimeTypeInfo>>>,
    /// Name to type ID mapping
    name_to_id: Arc<RwLock<HashMap<String, u64>>>,
    /// Statistics for debugging and performance monitoring
    stats: Arc<Mutex<AssertionStatistics>>,
    /// Panic behavior configuration
    panic_config: PanicConfiguration,
}

/// Statistics for type assertion operations
#[derive(Debug, Default)]
pub struct AssertionStatistics {
    /// Total number of type assertions performed
    pub total_assertions: u64,
    /// Number of successful assertions
    pub successful_assertions: u64,
    /// Number of failed assertions
    pub failed_assertions: u64,
    /// Number of panics triggered
    pub panic_count: u64,
    /// Type mismatches by type pair
    pub type_mismatches: HashMap<(String, String), u64>,
}

/// Configuration for panic behavior in type assertions
#[derive(Debug, Clone)]
pub struct PanicConfiguration {
    /// Whether to panic on type assertion failures
    pub panic_on_failure: bool,
    /// Whether to panic on nil interface assertions
    pub panic_on_nil: bool,
    /// Whether to include detailed type information in panic messages
    pub detailed_panic_messages: bool,
    /// Maximum stack trace depth for panic messages
    pub max_stack_trace_depth: usize,
}

impl Default for PanicConfiguration {
    fn default() -> Self {
        Self {
            panic_on_failure: false, // Default to safe behavior
            panic_on_nil: false,
            detailed_panic_messages: true,
            max_stack_trace_depth: 10,
        }
    }
}

impl TypeAssertionRuntime {
    /// Create a new runtime with default configuration
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new TypeAssertionRuntime");
        Self {
            type_registry: Arc::new(RwLock::new(HashMap::new())),
            name_to_id: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(AssertionStatistics::default())),
            panic_config: PanicConfiguration::default(),
        }
    }

    /// Create a new runtime with custom panic configuration
    #[instrument(skip(panic_config))]
    pub fn with_panic_config(panic_config: PanicConfiguration) -> Self {
        info!("Creating TypeAssertionRuntime with custom panic configuration");
        Self {
            type_registry: Arc::new(RwLock::new(HashMap::new())),
            name_to_id: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Mutex::new(AssertionStatistics::default())),
            panic_config,
        }
    }

    /// Register a new type in the runtime system
    #[instrument(skip(self), fields(type_id = type_info.type_id))]
    pub fn register_type(&mut self, type_info: RuntimeTypeInfo) -> Result<(), Error> {
        debug!("Registering type: {} (ID: 0x{:016x})", type_info.type_name, type_info.type_id);
        
        {
            let mut registry = self.type_registry.write()
                .map_err(|_| Error::Runtime("Failed to acquire type registry write lock".to_string()))?;
            registry.insert(type_info.type_id, type_info.clone());
        }
        
        {
            let mut name_map = self.name_to_id.write()
                .map_err(|_| Error::Runtime("Failed to acquire name mapping write lock".to_string()))?;
            name_map.insert(type_info.type_name.clone(), type_info.type_id);
        }
        
        info!("Successfully registered type: {}", type_info.type_name);
        Ok(())
    }

    /// Perform a runtime type assertion with comprehensive error handling
    #[instrument(skip(self), fields(interface_type_id = %interface_type_id, target_type_name = %target_type_name))]
    pub fn assert_type(
        &self,
        interface_type_id: u64,
        target_type_name: &str,
        source_location: Option<SourceLocation>
    ) -> Result<bool, CursedError> {
        debug!("Performing type assertion: 0x{:016x} -> {}", interface_type_id, target_type_name);
        
        // Update statistics
        self.increment_total_assertions();
        
        // Handle nil interface special case
        if interface_type_id == 0 {
            return self.handle_nil_assertion(target_type_name, source_location);
        }
        
        // Get type information
        let interface_type_info = self.get_type_info(interface_type_id);
        let target_type_id = self.get_type_id_by_name(target_type_name);
        
        match (interface_type_info, target_type_id) {
            (Some(interface_info), Some(target_id)) => {
                if interface_type_id == target_id {
                    self.increment_successful_assertions();
                    debug!("Type assertion successful: {} is {}", interface_info.type_name, target_type_name);
                    Ok(true)
                } else {
                    self.handle_type_mismatch(
                        &interface_info,
                        target_type_name,
                        target_id,
                        source_location
                    )
                }
            },
            (interface_info, target_id) => {
                self.handle_incomplete_type_info(
                    interface_type_id,
                    interface_info,
                    target_type_name,
                    target_id,
                    source_location
                )
            }
        }
    }

    /// Handle nil interface assertions
    #[instrument(skip(self))]
    fn handle_nil_assertion(
        &self,
        target_type_name: &str,
        source_location: Option<SourceLocation>
    ) -> Result<bool, CursedError> {
        warn!("Attempting to assert nil interface to type: {}", target_type_name);
        
        if self.panic_config.panic_on_nil {
            self.increment_panic_count();
            let panic_message = self.create_panic_message(
                "nil interface",
                target_type_name,
                source_location.as_ref(),
                "Cannot assert nil interface value to concrete type"
            );
            error!("Panicking on nil assertion: {}", panic_message);
            panic!("{}", panic_message);
        }
        
        self.increment_failed_assertions();
        
        let error = TypeAssertionError::new("nil", target_type_name)
            .with_message("Cannot assert nil interface value to concrete type");
        
        let error = if let Some(loc) = source_location {
            error.with_location(loc)
        } else {
            error
        };
        
        Err(error.into())
    }

    /// Handle type mismatch scenarios
    #[instrument(skip(self, interface_info), fields(interface_type = %interface_info.type_name))]
    fn handle_type_mismatch(
        &self,
        interface_info: &RuntimeTypeInfo,
        target_type_name: &str,
        target_type_id: u64,
        source_location: Option<SourceLocation>
    ) -> Result<bool, CursedError> {
        warn!("Type mismatch: {} is not {}", interface_info.type_name, target_type_name);
        
        // Record type mismatch statistics
        self.record_type_mismatch(&interface_info.type_name, target_type_name);
        
        if self.panic_config.panic_on_failure {
            self.increment_panic_count();
            let panic_message = self.create_panic_message(
                &interface_info.type_name,
                target_type_name,
                source_location.as_ref(),
                &format!("Type assertion failed: {} is not {}", interface_info.type_name, target_type_name)
            );
            error!("Panicking on assertion failure: {}", panic_message);
            panic!("{}", panic_message);
        }
        
        self.increment_failed_assertions();
        
        let error = helpers::create_detailed_assertion_error(
            &interface_info.type_name,
            target_type_name,
            Some(interface_info.type_id),
            Some(target_type_id),
            Some(interface_info.type_name.clone()),
            Some(interface_info.type_id),
            source_location
        );
        
        Err(error.into())
    }

    /// Handle cases where type information is incomplete
    #[instrument(skip(self))]
    fn handle_incomplete_type_info(
        &self,
        interface_type_id: u64,
        interface_info: Option<RuntimeTypeInfo>,
        target_type_name: &str,
        target_type_id: Option<u64>,
        source_location: Option<SourceLocation>
    ) -> Result<bool, CursedError> {
        warn!("Incomplete type information for assertion: 0x{:016x} -> {}", 
              interface_type_id, target_type_name);
        
        // Try to perform hash-based comparison as fallback
        if let Some(target_id) = target_type_id {
            if interface_type_id == target_id {
                self.increment_successful_assertions();
                debug!("Hash-based type assertion successful");
                return Ok(true);
            }
        }
        
        // Check if we should panic
        if self.panic_config.panic_on_failure {
            self.increment_panic_count();
            let interface_name = interface_info
                .as_ref()
                .map(|info| info.type_name.as_str())
                .unwrap_or("unknown");
            let panic_message = self.create_panic_message(
                interface_name,
                target_type_name,
                source_location.as_ref(),
                "Incomplete type information for assertion"
            );
            error!("Panicking on assertion failure (incomplete type info): {}", panic_message);
            panic!("{}", panic_message);
        }
        
        self.increment_failed_assertions();
        
        let interface_name = interface_info
            .map(|info| info.type_name)
            .unwrap_or_else(|| format!("unknown(0x{:016x})", interface_type_id));
        
        let error = TypeAssertionError::new(&interface_name, target_type_name)
            .with_interface_type_id(interface_type_id)
            .with_message("Incomplete type information available for assertion");
        
        let error = if let Some(loc) = source_location {
            error.with_location(loc)
        } else {
            error
        };
        
        if let Some(target_id) = target_type_id {
            Err(error.with_target_type_id(target_id).into())
        } else {
            Err(error.into())
        }
    }

    /// Create a detailed panic message
    #[instrument(skip(self))]
    fn create_panic_message(
        &self,
        interface_type: &str,
        target_type: &str,
        source_location: Option<&SourceLocation>,
        base_message: &str
    ) -> String {
        let mut message = format!("CURSED RUNTIME PANIC: {}", base_message);
        
        if self.panic_config.detailed_panic_messages {
            message.push_str(&format!(
                "\n  Interface Type: {}\n  Target Type: {}",
                interface_type, target_type
            ));
            
            if let Some(loc) = source_location {
                message.push_str(&format!("\n  Location: {}", loc));
            }
            
            // Add runtime statistics
            if let Ok(stats) = self.stats.lock() {
                message.push_str(&format!(
                    "\n  Runtime Stats: {} total, {} successful, {} failed",
                    stats.total_assertions, stats.successful_assertions, stats.failed_assertions
                ));
            }
            
            // Add stack trace information if available
            message.push_str(&format!("\n  Stack trace limited to {} frames", 
                                    self.panic_config.max_stack_trace_depth));
        }
        
        message
    }

    /// Get type information by type ID
    #[instrument(skip(self))]
    pub fn get_type_info(&self, type_id: u64) -> Option<RuntimeTypeInfo> {
        self.type_registry.read().ok()?.get(&type_id).cloned()
    }

    /// Get type ID by name
    #[instrument(skip(self))]
    pub fn get_type_id_by_name(&self, type_name: &str) -> Option<u64> {
        self.name_to_id.read().ok()?.get(type_name).copied()
    }

    /// Get current runtime statistics
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<AssertionStatistics, Error> {
        self.stats.lock()
            .map(|stats| AssertionStatistics {
                total_assertions: stats.total_assertions,
                successful_assertions: stats.successful_assertions,
                failed_assertions: stats.failed_assertions,
                panic_count: stats.panic_count,
                type_mismatches: stats.type_mismatches.clone(),
            })
            .map_err(|_| Error::Runtime("Failed to acquire statistics lock".to_string()))
    }

    /// Update panic configuration
    #[instrument(skip(self, config))]
    pub fn set_panic_config(&mut self, config: PanicConfiguration) {
        info!("Updating panic configuration");
        self.panic_config = config;
    }

    // Private helper methods for statistics

    fn increment_total_assertions(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_assertions += 1;
        }
    }

    fn increment_successful_assertions(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.successful_assertions += 1;
        }
    }

    fn increment_failed_assertions(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.failed_assertions += 1;
        }
    }

    fn increment_panic_count(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.panic_count += 1;
        }
    }

    fn record_type_mismatch(&self, interface_type: &str, target_type: &str) {
        if let Ok(mut stats) = self.stats.lock() {
            let key = (interface_type.to_string(), target_type.to_string());
            *stats.type_mismatches.entry(key).or_insert(0) += 1;
        }
    }
}

impl Default for TypeAssertionRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Safe wrapper for type assertion operations with automatic error handling
pub struct SafeTypeAssertion {
    runtime: Arc<TypeAssertionRuntime>,
}

impl SafeTypeAssertion {
    /// Create a new safe type assertion wrapper
    pub fn new(runtime: Arc<TypeAssertionRuntime>) -> Self {
        Self { runtime }
    }

    /// Perform a type assertion with automatic panic handling
    #[instrument(skip(self, recovery_fn))]
    pub fn assert_with_recovery<F, R>(
        &self,
        interface_type_id: u64,
        target_type_name: &str,
        source_location: Option<SourceLocation>,
        recovery_fn: F
    ) -> Result<R, CursedError>
    where
        F: FnOnce() -> R + panic::UnwindSafe,
    {
        // Set up panic hook to catch panics and convert them to errors
        let result = panic::catch_unwind(|| {
            self.runtime.assert_type(interface_type_id, target_type_name, source_location.clone())
        });

        match result {
            Ok(assertion_result) => {
                match assertion_result {
                    Ok(true) => {
                        // Assertion successful, execute the recovery function
                        match panic::catch_unwind(recovery_fn) {
                            Ok(value) => Ok(value),
                            Err(_) => {
                                error!("Panic occurred in recovery function after successful type assertion");
                                Err(CursedError::new(
                                    ErrorKind::Runtime,
                                    "Panic in recovery function after successful type assertion"
                                ))
                            }
                        }
                    },
                    Ok(false) => {
                        // Assertion failed but didn't panic
                        Err(CursedError::new(
                            ErrorKind::TypeAssertion,
                            format!("Type assertion failed: cannot convert to {}", target_type_name)
                        ))
                    },
                    Err(error) => Err(error),
                }
            },
            Err(_) => {
                error!("Panic occurred during type assertion");
                Err(CursedError::new(
                    ErrorKind::Runtime,
                    "Type assertion caused a panic - this indicates a serious runtime error"
                ).with_code("ASSERT-PANIC-001"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tracing_test::traced_test;

    // // #[traced_test]
    #[test]
    fn test_runtime_creation() {
        let runtime = TypeAssertionRuntime::new();
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_assertions, 0);
    }

    // #[traced_test]
    #[test]
    fn test_type_registration() {
        let mut runtime = TypeAssertionRuntime::new();
        
        let type_info = RuntimeTypeInfo {
            type_id: 0x1234567890ABCDEF,
            type_name: "Person".to_string(),
            size: 64,
            implements: vec!["Stringer".to_string()],
            debug_info: Some("Test person type".to_string()),
        };
        
        assert!(runtime.register_type(type_info).is_ok());
        
        let retrieved = runtime.get_type_info(0x1234567890ABCDEF);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().type_name, "Person");
    }

    // #[traced_test]
    #[test]
    fn test_successful_assertion() {
        let mut runtime = TypeAssertionRuntime::new();
        
        let type_info = RuntimeTypeInfo {
            type_id: 0x1111222233334444,
            type_name: "Dog".to_string(),
            size: 32,
            implements: vec![],
            debug_info: None,
        };
        
        runtime.register_type(type_info).unwrap();
        
        let result = runtime.assert_type(
            0x1111222233334444,
            "Dog",
            None
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.successful_assertions, 1);
    }

    // #[traced_test]
    #[test]
    fn test_failed_assertion() {
        let mut runtime = TypeAssertionRuntime::new();
        
        let person_info = RuntimeTypeInfo {
            type_id: 0x1111222233334444,
            type_name: "Person".to_string(),
            size: 64,
            implements: vec![],
            debug_info: None,
        };
        
        let dog_info = RuntimeTypeInfo {
            type_id: 0x5555666677778888,
            type_name: "Dog".to_string(),
            size: 32,
            implements: vec![],
            debug_info: None,
        };
        
        runtime.register_type(person_info).unwrap();
        runtime.register_type(dog_info).unwrap();
        
        let result = runtime.assert_type(
            0x1111222233334444, // Person type ID
            "Dog",               // Trying to assert to Dog
            None
        );
        
        assert!(result.is_err());
        
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.failed_assertions, 1);
    }

    // #[traced_test]
    #[test]
    fn test_nil_assertion_handling() {
        let runtime = TypeAssertionRuntime::new();
        
        let result = runtime.assert_type(0, "Person", None);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.kind(), &ErrorKind::TypeAssertion);
    }

    // #[traced_test]
    #[test]
    #[should_panic(expected = "CURSED RUNTIME PANIC")]
    fn test_panic_on_failure() {
        let mut runtime = TypeAssertionRuntime::with_panic_config(
            PanicConfiguration {
                panic_on_failure: true,
                ..Default::default()
            }
        );
        
        let person_info = RuntimeTypeInfo {
            type_id: 0x1111222233334444,
            type_name: "Person".to_string(),
            size: 64,
            implements: vec![],
            debug_info: None,
        };
        
        runtime.register_type(person_info).unwrap();
        
        // This should panic
        let _ = runtime.assert_type(0x1111222233334444, "Dog", None);
    }

    // #[traced_test]
    #[test]
    fn test_safe_assertion_with_recovery() {
        let mut runtime = TypeAssertionRuntime::new();
        
        let type_info = RuntimeTypeInfo {
            type_id: 0x1111222233334444,
            type_name: "Person".to_string(),
            size: 64,
            implements: vec![],
            debug_info: None,
        };
        
        runtime.register_type(type_info).unwrap();
        
        let safe_wrapper = SafeTypeAssertion::new(Arc::new(runtime));
        
        let result = safe_wrapper.assert_with_recovery(
            0x1111222233334444,
            "Person",
            None,
            || "Success!".to_string()
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success!");
    }
}
