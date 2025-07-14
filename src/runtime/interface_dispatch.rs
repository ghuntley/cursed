// Interface Method Dispatch System for CURSED Runtime
//
// This module implements the runtime support for interface method dispatch,
// including vtable management and method resolution.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::runtime::value::Value;
use crate::error_types::CursedError;

/// Interface method signature for dispatch
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub param_types: Vec<String>,
    pub return_type: Option<String>,
    pub method_index: usize,
}

/// Virtual table entry for interface methods
#[derive(Debug, Clone)]
pub struct VTableEntry {
    pub method_name: String,
    pub function_ptr: usize, // Function pointer address
    pub param_types: Vec<String>,
    pub return_type: Option<String>,
}

/// Interface virtual table
#[derive(Debug, Clone)]
pub struct InterfaceVTable {
    pub interface_name: String,
    pub concrete_type: String,
    pub methods: Vec<VTableEntry>,
    pub method_indices: HashMap<String, usize>,
}

impl InterfaceVTable {
    /// Create a new interface vtable
    pub fn new(interface_name: String, concrete_type: String) -> Self {
        Self {
            interface_name,
            concrete_type,
            methods: Vec::new(),
            method_indices: HashMap::new(),
        }
    }
    
    /// Add a method to the vtable
    pub fn add_method(&mut self, method: VTableEntry) -> Result<(), CursedError> {
        let method_name = method.method_name.clone();
        let index = self.methods.len();
        
        // Check for duplicate methods
        if self.method_indices.contains_key(&method_name) {
            return Err(CursedError::Runtime(format!(
                "Method '{}' already exists in vtable for interface '{}'",
                method_name, self.interface_name
            )));
        }
        
        self.method_indices.insert(method_name, index);
        self.methods.push(method);
        Ok(())
    }
    
    /// Get method by name
    pub fn get_method(&self, method_name: &str) -> Option<&VTableEntry> {
        self.method_indices.get(method_name)
            .and_then(|&index| self.methods.get(index))
    }
    
    /// Get method index by name
    pub fn get_method_index(&self, method_name: &str) -> Option<usize> {
        self.method_indices.get(method_name).copied()
    }
}

/// Interface object representation (fat pointer)
#[derive(Debug, Clone)]
pub struct InterfaceValue {
    pub vtable: Arc<InterfaceVTable>,
    pub data_ptr: usize, // Pointer to concrete object
    pub interface_name: String,
    pub concrete_type: String,
}

impl InterfaceValue {
    /// Create a new interface value
    pub fn new(
        vtable: Arc<InterfaceVTable>,
        data_ptr: usize,
        interface_name: String,
        concrete_type: String,
    ) -> Self {
        Self {
            vtable,
            data_ptr,
            interface_name,
            concrete_type,
        }
    }
    
    /// Get method for dispatch
    pub fn get_method(&self, method_name: &str) -> Option<&VTableEntry> {
        self.vtable.get_method(method_name)
    }
}

/// Interface dispatch registry
pub struct InterfaceDispatchRegistry {
    /// Map from (interface_name, concrete_type) to vtable
    vtables: HashMap<(String, String), Arc<InterfaceVTable>>,
    /// Map from interface_name to list of methods
    pub interface_methods: HashMap<String, Vec<InterfaceMethod>>,
    /// Map from (concrete_type, interface_name) to implementation status
    implementations: HashMap<(String, String), bool>,
    /// Runtime dispatch table for optimized method resolution
    dispatch_table: HashMap<String, HashMap<String, usize>>, // interface_name -> method_name -> method_index
    /// Global method registry for fast lookup
    global_methods: HashMap<String, HashMap<String, usize>>, // interface_name -> method_name -> function_ptr
}

impl InterfaceDispatchRegistry {
    /// Create a new dispatch registry
    pub fn new() -> Self {
        Self {
            vtables: HashMap::new(),
            interface_methods: HashMap::new(),
            implementations: HashMap::new(),
            dispatch_table: HashMap::new(),
            global_methods: HashMap::new(),
        }
    }
    
    /// Register an interface definition
    pub fn register_interface(&mut self, interface_name: String, methods: Vec<InterfaceMethod>) -> Result<(), CursedError> {
        // Validate method signatures
        for method in &methods {
            if method.name.is_empty() {
                return Err(CursedError::Runtime(format!(
                    "Empty method name in interface '{}'", interface_name
                )));
            }
        }
        
        // Create dispatch table for this interface
        let mut dispatch_map = HashMap::new();
        let mut global_method_map = HashMap::new();
        
        for method in &methods {
            dispatch_map.insert(method.name.clone(), method.method_index);
            global_method_map.insert(method.name.clone(), 0); // Will be updated during implementation registration
        }
        
        self.dispatch_table.insert(interface_name.clone(), dispatch_map);
        self.global_methods.insert(interface_name.clone(), global_method_map);
        self.interface_methods.insert(interface_name, methods);
        
        Ok(())
    }
    
    /// Register a concrete type implementation of an interface
    pub fn register_implementation(
        &mut self,
        interface_name: String,
        concrete_type: String,
        method_implementations: HashMap<String, usize>, // method_name -> function_ptr
    ) -> Result<(), CursedError> {
        // Get interface methods
        let interface_methods = self.interface_methods.get(&interface_name)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Interface '{}' not found", interface_name
            )))?;
        
        // Create vtable
        let mut vtable = InterfaceVTable::new(interface_name.clone(), concrete_type.clone());
        
        // Add methods to vtable
        for method in interface_methods {
            let function_ptr = method_implementations.get(&method.name)
                .ok_or_else(|| CursedError::Runtime(format!(
                    "Method '{}' not implemented for type '{}' implementing interface '{}'",
                    method.name, concrete_type, interface_name
                )))?;
            
            let vtable_entry = VTableEntry {
                method_name: method.name.clone(),
                function_ptr: *function_ptr,
                param_types: method.param_types.clone(),
                return_type: method.return_type.clone(),
            };
            
            vtable.add_method(vtable_entry)?;
        }
        
        // Register vtable and implementation
        let key = (interface_name.clone(), concrete_type.clone());
        self.vtables.insert(key, Arc::new(vtable));
        self.implementations.insert((concrete_type, interface_name), true);
        
        Ok(())
    }
    
    /// Get vtable for interface and concrete type
    pub fn get_vtable(&self, interface_name: &str, concrete_type: &str) -> Option<Arc<InterfaceVTable>> {
        let key = (interface_name.to_string(), concrete_type.to_string());
        self.vtables.get(&key).cloned()
    }
    
    /// Check if a concrete type implements an interface
    pub fn implements_interface(&self, concrete_type: &str, interface_name: &str) -> bool {
        let key = (concrete_type.to_string(), interface_name.to_string());
        self.implementations.get(&key).copied().unwrap_or(false)
    }
    
    /// Get method index for interface method (optimized lookup)
    pub fn get_method_index(&self, interface_name: &str, method_name: &str) -> Option<usize> {
        self.dispatch_table.get(interface_name)
            .and_then(|methods| methods.get(method_name))
            .copied()
    }
    
    /// Get function pointer for interface method implementation
    pub fn get_method_function_ptr(&self, interface_name: &str, method_name: &str) -> Option<usize> {
        self.global_methods.get(interface_name)
            .and_then(|methods| methods.get(method_name))
            .copied()
    }
    
    /// Validate interface compliance for a concrete type
    pub fn validate_interface_compliance(&self, interface_name: &str, concrete_type: &str) -> Result<(), CursedError> {
        let interface_methods = self.interface_methods.get(interface_name)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Interface '{}' not found", interface_name
            )))?;
        
        let vtable = self.get_vtable(interface_name, concrete_type)
            .ok_or_else(|| CursedError::Runtime(format!(
                "No implementation found for interface '{}' on type '{}'", 
                interface_name, concrete_type
            )))?;
        
        // Check that all interface methods are implemented
        for method in interface_methods {
            if vtable.get_method(&method.name).is_none() {
                return Err(CursedError::Runtime(format!(
                    "Method '{}' from interface '{}' not implemented by type '{}'",
                    method.name, interface_name, concrete_type
                )));
            }
        }
        
        Ok(())
    }
    
    /// Create interface value from concrete object
    pub fn create_interface_value(
        &self,
        concrete_type: &str,
        interface_name: &str,
        data_ptr: usize,
    ) -> Result<InterfaceValue, CursedError> {
        let vtable = self.get_vtable(interface_name, concrete_type)
            .ok_or_else(|| CursedError::Runtime(format!(
                "No vtable found for type '{}' implementing interface '{}'",
                concrete_type, interface_name
            )))?;
        
        Ok(InterfaceValue::new(
            vtable,
            data_ptr,
            interface_name.to_string(),
            concrete_type.to_string(),
        ))
    }
    
    /// Dispatch interface method call
    pub fn dispatch_method(
        &self,
        interface_value: &InterfaceValue,
        method_name: &str,
        args: &[Value],
    ) -> Result<Value, CursedError> {
        // Get method from vtable
        let method = interface_value.get_method(method_name)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Method '{}' not found in interface '{}' for type '{}'",
                method_name, interface_value.interface_name, interface_value.concrete_type
            )))?;
        
        // TODO: Implement actual method dispatch via function pointer
        // For now, return a placeholder error
        Err(CursedError::Runtime(format!(
            "Method dispatch not yet implemented for {}.{} (function_ptr: {})",
            interface_value.interface_name, method_name, method.function_ptr
        )))
    }
}

/// Global interface dispatch registry
static mut GLOBAL_DISPATCH_REGISTRY: Option<Mutex<InterfaceDispatchRegistry>> = None;

/// Initialize global dispatch registry
pub fn initialize_interface_dispatch() -> Result<(), CursedError> {
    unsafe {
        if GLOBAL_DISPATCH_REGISTRY.is_some() {
            return Ok(()); // Already initialized
        }
        
        GLOBAL_DISPATCH_REGISTRY = Some(Mutex::new(InterfaceDispatchRegistry::new()));
        Ok(())
    }
}

/// Get global dispatch registry
pub fn get_global_dispatch_registry() -> Result<&'static Mutex<InterfaceDispatchRegistry>, CursedError> {
    unsafe {
        GLOBAL_DISPATCH_REGISTRY.as_ref()
            .ok_or_else(|| CursedError::Runtime(
                "Interface dispatch registry not initialized".to_string()
            ))
    }
}

/// Register interface with global registry
pub fn register_global_interface(interface_name: String, methods: Vec<InterfaceMethod>) -> Result<(), CursedError> {
    let registry = get_global_dispatch_registry()?;
    let mut registry = registry.lock().map_err(|e| CursedError::Runtime(format!(
        "Failed to lock dispatch registry: {}", e
    )))?;
    
    registry.register_interface(interface_name, methods)
}

/// Register implementation with global registry
pub fn register_global_implementation(
    interface_name: String,
    concrete_type: String,
    method_implementations: HashMap<String, usize>,
) -> Result<(), CursedError> {
    let registry = get_global_dispatch_registry()?;
    let mut registry = registry.lock().map_err(|e| CursedError::Runtime(format!(
        "Failed to lock dispatch registry: {}", e
    )))?;
    
    registry.register_implementation(interface_name, concrete_type, method_implementations)
}

/// Create interface value using global registry
pub fn create_global_interface_value(
    concrete_type: &str,
    interface_name: &str,
    data_ptr: usize,
) -> Result<InterfaceValue, CursedError> {
    let registry = get_global_dispatch_registry()?;
    let registry = registry.lock().map_err(|e| CursedError::Runtime(format!(
        "Failed to lock dispatch registry: {}", e
    )))?;
    
    registry.create_interface_value(concrete_type, interface_name, data_ptr)
}

/// Dispatch method using global registry
pub fn dispatch_global_method(
    interface_value: &InterfaceValue,
    method_name: &str,
    args: &[Value],
) -> Result<Value, CursedError> {
    let registry = get_global_dispatch_registry()?;
    let registry = registry.lock().map_err(|e| CursedError::Runtime(format!(
        "Failed to lock dispatch registry: {}", e
    )))?;
    
    registry.dispatch_method(interface_value, method_name, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interface_vtable_creation() {
        let mut vtable = InterfaceVTable::new("TestInterface".to_string(), "TestType".to_string());
        
        let method = VTableEntry {
            method_name: "test_method".to_string(),
            function_ptr: 0x1234,
            param_types: vec!["i32".to_string()],
            return_type: Some("i32".to_string()),
        };
        
        assert!(vtable.add_method(method).is_ok());
        assert!(vtable.get_method("test_method").is_some());
        assert_eq!(vtable.get_method_index("test_method"), Some(0));
    }
    
    #[test]
    fn test_interface_registry() {
        let mut registry = InterfaceDispatchRegistry::new();
        
        let methods = vec![
            InterfaceMethod {
                name: "test_method".to_string(),
                param_types: vec!["i32".to_string()],
                return_type: Some("i32".to_string()),
                method_index: 0,
            }
        ];
        
        assert!(registry.register_interface("TestInterface".to_string(), methods).is_ok());
        
        let mut implementations = HashMap::new();
        implementations.insert("test_method".to_string(), 0x1234);
        
        assert!(registry.register_implementation(
            "TestInterface".to_string(),
            "TestType".to_string(),
            implementations
        ).is_ok());
        
        assert!(registry.implements_interface("TestType", "TestInterface"));
        assert!(registry.get_vtable("TestInterface", "TestType").is_some());
    }
}
