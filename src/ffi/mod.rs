//! Advanced Foreign Function Interface (FFI) system for CURSED
//!
//! This module provides comprehensive FFI capabilities including:
//! - Automatic C header parsing and binding generation
//! - Advanced type mapping and struct marshalling
//! - Multi-language support (C/C++, Python, Go, JavaScript/WASM)
//! - Memory safety and error handling
//! - Performance optimizations and developer tools

pub mod binding_generator;
pub mod c_bridge;
pub mod callback_manager;
pub mod debug_tools;
pub mod error_handling;
pub mod header_parser;
pub mod memory_safety;
pub mod multi_language;
pub mod performance;
pub mod profiling;
pub mod safety_checks;
pub mod threads;
pub mod type_mapping;
pub mod wasm_bridge;

use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::{Arc, Mutex, RwLock};
use crate::error::CursedError;

pub use binding_generator::*;
pub use c_bridge::*;
pub use callback_manager::*;
pub use debug_tools::*;
pub use error_handling::*;
pub use header_parser::*;
pub use memory_safety::*;
pub use multi_language::*;
pub use performance::*;
pub use profiling::*;
pub use safety_checks::*;
pub use threads::*;
pub use type_mapping::*;
pub use wasm_bridge::*;

/// Main FFI system manager
pub struct FfiSystem {
    /// Type mapping system
    type_mapper: Arc<RwLock<TypeMapper>>,
    
    /// Memory safety manager
    memory_safety: Arc<Mutex<MemorySafetyManager>>,
    
    /// Callback manager
    callback_manager: Arc<CallbackManager>,
    
    /// Performance optimizer
    performance_optimizer: Arc<PerformanceOptimizer>,
    
    /// Debug tools
    debug_tools: Arc<DebugTools>,
    
    /// Multi-language bridges
    language_bridges: HashMap<String, Box<dyn LanguageBridge>>,
    
    /// Active function registry
    functions: Arc<RwLock<HashMap<String, Box<dyn FfiFunctionWrapper>>>>,
    
    /// Type registry
    types: Arc<RwLock<HashMap<String, FfiType>>>,
    
    /// Performance profiler
    profiler: Arc<Mutex<FfiProfiler>>,
    
    /// Safety checker
    safety_checker: Arc<SafetyChecker>,
}

impl FfiSystem {
    /// Create a new FFI system with default configuration
    pub fn new() -> Result<Self, CursedError> {
        let type_mapper = Arc::new(RwLock::new(TypeMapper::new()));
        let memory_safety = Arc::new(Mutex::new(MemorySafetyManager::new()));
        let callback_manager = Arc::new(CallbackManager::new());
        let performance_optimizer = Arc::new(PerformanceOptimizer::new());
        let debug_tools = Arc::new(DebugTools::new());
        let profiler = Arc::new(Mutex::new(FfiProfiler::new()));
        let safety_checker = Arc::new(SafetyChecker::new());
        
        let mut system = Self {
            type_mapper,
            memory_safety,
            callback_manager,
            performance_optimizer,
            debug_tools,
            language_bridges: HashMap::new(),
            functions: Arc::new(RwLock::new(HashMap::new())),
            types: Arc::new(RwLock::new(HashMap::new())),
            profiler,
            safety_checker,
        };
        
        // Initialize language bridges
        system.initialize_language_bridges()?;
        
        // Register built-in types
        system.register_builtin_types()?;
        
        // Initialize performance optimizations
        system.initialize_performance_optimizations()?;
        
        Ok(system)
    }
    
    /// Initialize language bridges for multi-language support
    fn initialize_language_bridges(&mut self) -> Result<(), CursedError> {
        // C/C++ bridge
        let c_bridge = Box::new(CBridge::new()?);
        self.language_bridges.insert("c".to_string(), c_bridge);
        
        // Python bridge
        let python_bridge = Box::new(PythonBridge::new()?);
        self.language_bridges.insert("python".to_string(), python_bridge);
        
        // Go bridge
        let go_bridge = Box::new(GoBridge::new()?);
        self.language_bridges.insert("go".to_string(), go_bridge);
        
        // JavaScript/WASM bridge
        let wasm_bridge = Box::new(WasmBridge::new()?);
        self.language_bridges.insert("wasm".to_string(), wasm_bridge);
        
        Ok(())
    }
    
    /// Register built-in types
    fn register_builtin_types(&mut self) -> Result<(), CursedError> {
        let mut types = self.types.write().unwrap();
        
        // Basic types
        types.insert("i8".to_string(), FfiType::SignedInteger(8));
        types.insert("i16".to_string(), FfiType::SignedInteger(16));
        types.insert("i32".to_string(), FfiType::SignedInteger(32));
        types.insert("i64".to_string(), FfiType::SignedInteger(64));
        types.insert("u8".to_string(), FfiType::UnsignedInteger(8));
        types.insert("u16".to_string(), FfiType::UnsignedInteger(16));
        types.insert("u32".to_string(), FfiType::UnsignedInteger(32));
        types.insert("u64".to_string(), FfiType::UnsignedInteger(64));
        types.insert("f32".to_string(), FfiType::Float(32));
        types.insert("f64".to_string(), FfiType::Float(64));
        types.insert("bool".to_string(), FfiType::Boolean);
        types.insert("char".to_string(), FfiType::Character);
        types.insert("*void".to_string(), FfiType::Pointer(Box::new(FfiType::Void)));
        
        // String types
        types.insert("*char".to_string(), FfiType::CString);
        types.insert("string".to_string(), FfiType::String);
        
        Ok(())
    }
    
    /// Initialize performance optimizations
    fn initialize_performance_optimizations(&mut self) -> Result<(), CursedError> {
        // Set up memory pools
        self.performance_optimizer.initialize_memory_pools()?;
        
        // Configure call optimization
        self.performance_optimizer.configure_call_optimization()?;
        
        // Set up zero-copy mechanisms
        self.performance_optimizer.initialize_zero_copy_transfers()?;
        
        Ok(())
    }
    
    /// Parse C header file and generate bindings
    pub fn parse_header_file(&self, header_path: &str) -> Result<HeaderInfo, CursedError> {
        let parser = HeaderParser::new();
        let header_info = parser.parse_file(header_path)?;
        
        // Generate bindings
        let binding_generator = BindingGenerator::new();
        let bindings = binding_generator.generate_bindings(&header_info)?;
        
        // Register functions and types
        self.register_header_bindings(&bindings)?;
        
        Ok(header_info)
    }
    
    /// Register bindings from parsed header
    fn register_header_bindings(&self, bindings: &GeneratedBindings) -> Result<(), CursedError> {
        let mut functions = self.functions.write().unwrap();
        let mut types = self.types.write().unwrap();
        
        // Register functions
        for (name, function) in &bindings.functions {
            functions.insert(name.clone(), function.clone());
        }
        
        // Register types
        for (name, ffi_type) in &bindings.types {
            types.insert(name.clone(), ffi_type.clone());
        }
        
        Ok(())
    }
    
    /// Call a foreign function with automatic type conversion
    pub fn call_function(
        &self,
        function_name: &str,
        args: &[FfiValue],
        language: &str,
    ) -> Result<FfiValue, CursedError> {
        // Get the appropriate language bridge
        let bridge = self.language_bridges.get(language)
            .ok_or_else(|| CursedError::General(format!("Unsupported language: {}", language)))?;
        
        // Start profiling
        let _profile_guard = self.profiler.lock().unwrap().start_call(function_name);
        
        // Perform safety checks
        self.safety_checker.validate_function_call(function_name, args)?;
        
        // Call the function through the bridge
        let result = bridge.call_function(function_name, args)?;
        
        // Validate the result
        self.safety_checker.validate_result(&result)?;
        
        Ok(result)
    }
    
    /// Create a callback that can be called from foreign code
    pub fn create_callback<F>(
        &self,
        callback: F,
        signature: &FunctionSignature,
    ) -> Result<CallbackHandle, CursedError>
    where
        F: Fn(&[FfiValue]) -> Result<FfiValue, CursedError> + Send + Sync + 'static,
    {
        self.callback_manager.create_callback(callback, signature)
    }
    
    /// Register a custom type mapping
    pub fn register_type_mapping(
        &self,
        cursed_type: &str,
        foreign_type: &str,
        language: &str,
    ) -> Result<(), CursedError> {
        let mut type_mapper = self.type_mapper.write().unwrap();
        type_mapper.register_mapping(cursed_type, foreign_type, language)
    }
    
    /// Get performance statistics
    pub fn get_performance_stats(&self) -> Result<PerformanceStats, CursedError> {
        let profiler = self.profiler.lock().unwrap();
        Ok(profiler.get_stats().clone())
    }
    
    /// Enable debug mode for FFI calls
    pub fn enable_debug_mode(&self) -> Result<(), CursedError> {
        self.debug_tools.enable_debug_mode()
    }
    
    /// Get debug information for a function call
    pub fn get_debug_info(&self, function_name: &str) -> Result<DebugInfo, CursedError> {
        self.debug_tools.get_function_debug_info(function_name)
    }
    
    /// Cleanup FFI resources
    pub fn cleanup(&self) -> Result<(), CursedError> {
        // Clean up callbacks
        self.callback_manager.cleanup()?;
        
        // Clean up memory
        self.memory_safety.lock().unwrap().cleanup()?;
        
        // Clean up language bridges
        for (_, bridge) in &self.language_bridges {
            bridge.cleanup()?;
        }
        
        Ok(())
    }
    
    /// Generate bindings for a library
    pub fn generate_library_bindings(
        &self,
        library_path: &str,
        output_path: &str,
        language: &str,
    ) -> Result<(), CursedError> {
        let bridge = self.language_bridges.get(language)
            .ok_or_else(|| CursedError::General(format!("Unsupported language: {}", language)))?;
        
        bridge.generate_bindings(library_path, output_path)
    }
    
    /// Load a dynamic library
    pub fn load_library(&self, library_path: &str) -> Result<LibraryHandle, CursedError> {
        let bridge = self.language_bridges.get("c")
            .ok_or_else(|| CursedError::General("C bridge not available".to_string()))?;
        
        bridge.load_library(library_path)
    }
    
    /// Marshal a CURSED value to foreign representation
    pub fn marshal_to_foreign(
        &self,
        value: &FfiValue,
        target_type: &FfiType,
        language: &str,
    ) -> Result<ForeignValue, CursedError> {
        let type_mapper = self.type_mapper.read().unwrap();
        type_mapper.marshal_to_foreign(value, target_type, language)
    }
    
    /// Unmarshal a foreign value to CURSED representation
    pub fn unmarshal_from_foreign(
        &self,
        foreign_value: &ForeignValue,
        source_type: &FfiType,
        language: &str,
    ) -> Result<FfiValue, CursedError> {
        let type_mapper = self.type_mapper.read().unwrap();
        type_mapper.unmarshal_from_foreign(foreign_value, source_type, language)
    }
    
    /// Get available functions for a language
    pub fn get_available_functions(&self, language: &str) -> Result<Vec<String>, CursedError> {
        let bridge = self.language_bridges.get(language)
            .ok_or_else(|| CursedError::General(format!("Unsupported language: {}", language)))?;
        
        bridge.get_available_functions()
    }
    
    /// Get supported languages
    pub fn get_supported_languages(&self) -> Vec<String> {
        self.language_bridges.keys().cloned().collect()
    }
}

impl Default for FfiSystem {
    fn default() -> Self {
        Self::new().expect("Failed to create FFI system")
    }
}

/// FFI value that can be passed across language boundaries
#[derive(Debug, Clone)]
pub enum FfiValue {
    Void,
    SignedInteger(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
    CString(std::ffi::CString),
    Pointer(*mut std::ffi::c_void),
    Array(Vec<FfiValue>),
    Struct(HashMap<String, FfiValue>),
    Function(CallbackHandle),
}

/// FFI type information
#[derive(Debug, Clone)]
pub enum FfiType {
    Void,
    SignedInteger(u8),   // bit width
    UnsignedInteger(u8), // bit width
    Float(u8),          // bit width
    Boolean,
    Character,
    String,
    CString,
    Pointer(Box<FfiType>),
    Array(Box<FfiType>, Option<usize>), // element type, optional size
    Struct(HashMap<String, FfiType>),
    Function(FunctionSignature),
}

/// Function signature for FFI calls
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: FfiType,
    pub parameters: Vec<Parameter>,
    pub is_variadic: bool,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: FfiType,
    pub is_const: bool,
    pub is_nullable: bool,
}

/// Foreign value representation
#[derive(Debug)]
pub struct ForeignValue {
    pub data: *mut std::ffi::c_void,
    pub size: usize,
    pub type_info: FfiType,
}

/// Library handle for loaded dynamic libraries
#[derive(Debug)]
pub struct LibraryHandle {
    pub handle: *mut std::ffi::c_void,
    pub path: String,
}

/// Callback handle for foreign-callable functions
#[derive(Debug, Clone)]
pub struct CallbackHandle {
    pub id: usize,
    pub function_ptr: *mut std::ffi::c_void,
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_calls: u64,
    pub average_call_time: f64,
    pub memory_usage: usize,
    pub cache_hit_rate: f64,
}

/// Debug information for FFI calls
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub function_name: String,
    pub call_count: u64,
    pub last_call_time: f64,
    pub error_count: u64,
    pub type_conversions: Vec<TypeConversion>,
}

/// Type conversion information
#[derive(Debug, Clone)]
pub struct TypeConversion {
    pub from_type: String,
    pub to_type: String,
    pub conversion_time: f64,
}

// Safety: These implementations are safe because the FFI system manages
// memory safety internally and provides proper cleanup mechanisms
unsafe impl Send for FfiSystem {}
unsafe impl Sync for FfiSystem {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_system_creation() {
        let ffi_system = FfiSystem::new().unwrap();
        let languages = ffi_system.get_supported_languages();
        assert!(languages.contains(&"c".to_string()));
        assert!(languages.contains(&"python".to_string()));
        assert!(languages.contains(&"go".to_string()));
        assert!(languages.contains(&"wasm".to_string()));
    }
    
    #[test]
    fn test_type_registration() {
        let ffi_system = FfiSystem::new().unwrap();
        
        let result = ffi_system.register_type_mapping("i32", "int", "c");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_callback_creation() {
        let ffi_system = FfiSystem::new().unwrap();
        
        let signature = FunctionSignature {
            name: "test_callback".to_string(),
            return_type: FfiType::SignedInteger(32),
            parameters: vec![
                Parameter {
                    name: "x".to_string(),
                    param_type: FfiType::SignedInteger(32),
                    is_const: false,
                    is_nullable: false,
                }
            ],
            is_variadic: false,
        };
        
        let callback = |args: &[FfiValue]| -> Result<FfiValue, CursedError> {
            if let [FfiValue::SignedInteger(x)] = args {
                Ok(FfiValue::SignedInteger(x * 2))
            } else {
                Err(CursedError::General("Invalid arguments".to_string()))
            }
        };
        
        let handle = ffi_system.create_callback(callback, &signature);
        assert!(handle.is_ok());
    }
    
    #[test]
    fn test_performance_stats() {
        let ffi_system = FfiSystem::new().unwrap();
        let stats = ffi_system.get_performance_stats().unwrap();
        
        assert_eq!(stats.total_calls, 0);
        assert_eq!(stats.average_call_time, 0.0);
    }
}
