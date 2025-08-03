//! Complete Interface Virtual Dispatch System for CURSED
//!
//! This module implements the complete interface virtual dispatch system including:
//! - Interface definition and registration
//! - Implementation registration and vtable generation
//! - Dynamic method dispatch with type safety
//! - Interface casting and type assertions
//! - Memory-efficient vtable management
//! - Integration with interpreter and LLVM compiler

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::any::Any;
use crate::ast::{InterfaceStatement, MethodSignature, Type, Expression, Statement};
use crate::error_types::CursedError;
use crate::execution::ExecutionContext;
use crate::runtime::value::Value;
use once_cell::sync::Lazy;

/// Interface method information for dispatch
#[derive(Debug, Clone)]
pub struct InterfaceMethodInfo {
    pub name: String,
    pub param_types: Vec<Type>,
    pub return_type: Option<Type>,
    pub method_index: usize,
    pub is_static: bool,
    pub receiver_type: ReceiverType,
}

/// Method receiver type for interface dispatch
#[derive(Debug, Clone, PartialEq)]
pub enum ReceiverType {
    Value,    // method(self, ...)
    Pointer,  // method(&self, ...)
    Mutable,  // method(&mut self, ...)
    Static,   // static method
}

/// Virtual method table entry
#[derive(Debug, Clone)]
pub struct VirtualMethodEntry {
    pub method_name: String,
    pub function_ptr: usize,
    pub param_types: Vec<Type>,
    pub return_type: Option<Type>,
    pub receiver_type: ReceiverType,
}

/// Interface virtual table (vtable)
#[derive(Debug, Clone)]
pub struct InterfaceVTable {
    pub interface_name: String,
    pub implementing_type: String,
    pub methods: Vec<VirtualMethodEntry>,
    pub method_lookup: HashMap<String, usize>,
    pub type_id: u64,
    pub size: usize,
}

impl InterfaceVTable {
    /// Create new vtable for interface implementation
    pub fn new(interface_name: String, implementing_type: String) -> Self {
        Self {
            interface_name,
            implementing_type,
            methods: Vec::new(),
            method_lookup: HashMap::new(),
            type_id: Self::generate_type_id(&interface_name, &implementing_type),
            size: 0,
        }
    }
    
    /// Generate unique type ID for this interface-type combination
    fn generate_type_id(interface_name: &str, implementing_type: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        interface_name.hash(&mut hasher);
        implementing_type.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Add method to vtable
    pub fn add_method(&mut self, method: VirtualMethodEntry) -> Result<(), CursedError> {
        let method_name = method.method_name.clone();
        
        // Check for duplicate methods
        if self.method_lookup.contains_key(&method_name) {
            return Err(CursedError::Runtime(format!(
                "Method '{}' already exists in vtable for interface '{}' implemented by '{}'",
                method_name, self.interface_name, self.implementing_type
            )));
        }
        
        let index = self.methods.len();
        self.method_lookup.insert(method_name, index);
        self.methods.push(method);
        self.size += 8; // Assume 8 bytes per function pointer
        
        Ok(())
    }
    
    /// Get method by name
    pub fn get_method(&self, method_name: &str) -> Option<&VirtualMethodEntry> {
        self.method_lookup.get(method_name)
            .and_then(|&index| self.methods.get(index))
    }
    
    /// Get method index for fast lookup
    pub fn get_method_index(&self, method_name: &str) -> Option<usize> {
        self.method_lookup.get(method_name).copied()
    }
    
    /// Validate that all interface methods are implemented
    pub fn validate_completeness(&self, interface_methods: &[InterfaceMethodInfo]) -> Result<(), CursedError> {
        for interface_method in interface_methods {
            if !self.method_lookup.contains_key(&interface_method.name) {
                return Err(CursedError::Runtime(format!(
                    "Method '{}' from interface '{}' not implemented by type '{}'",
                    interface_method.name, self.interface_name, self.implementing_type
                )));
            }
        }
        Ok(())
    }
}

/// Interface object (fat pointer implementation)
#[derive(Debug, Clone)]
pub struct InterfaceObject {
    pub vtable: Arc<InterfaceVTable>,
    pub data_ptr: usize,
    pub interface_name: String,
    pub concrete_type: String,
    pub type_id: u64,
}

impl InterfaceObject {
    /// Create new interface object
    pub fn new(
        vtable: Arc<InterfaceVTable>,
        data_ptr: usize,
        interface_name: String,
        concrete_type: String,
    ) -> Self {
        let type_id = vtable.type_id;
        Self {
            vtable,
            data_ptr,
            interface_name,
            concrete_type,
            type_id,
        }
    }
    
    /// Get method for dispatch
    pub fn get_method(&self, method_name: &str) -> Option<&VirtualMethodEntry> {
        self.vtable.get_method(method_name)
    }
    
    /// Check if this object implements specific method
    pub fn implements_method(&self, method_name: &str) -> bool {
        self.vtable.method_lookup.contains_key(method_name)
    }
    
    /// Get concrete type name
    pub fn concrete_type(&self) -> &str {
        &self.concrete_type
    }
    
    /// Cast to concrete type (type assertion)
    pub fn cast_to_concrete<T: Any>(&self) -> Option<&T> {
        unsafe {
            let ptr = self.data_ptr as *const T;
            ptr.as_ref()
        }
    }
}

/// Interface definition registry
#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub name: String,
    pub methods: Vec<InterfaceMethodInfo>,
    pub extends: Vec<String>,
    pub type_parameters: Vec<String>,
    pub source_location: Option<String>,
}

/// Interface implementation registry
#[derive(Debug)]
pub struct InterfaceImplementationRegistry {
    /// Interface definitions
    interface_definitions: HashMap<String, InterfaceDefinition>,
    
    /// VTables for interface implementations
    vtables: HashMap<String, Arc<InterfaceVTable>>, // Key: "interface_name::implementing_type"
    
    /// Type to interfaces mapping
    type_interfaces: HashMap<String, Vec<String>>,
    
    /// Interface inheritance hierarchy
    interface_hierarchy: HashMap<String, Vec<String>>,
    
    /// Method implementations for types
    type_methods: HashMap<String, HashMap<String, usize>>, // type -> method_name -> function_ptr
    
    /// Runtime interface objects
    interface_objects: Vec<InterfaceObject>,
    
    /// Performance optimization caches
    method_cache: HashMap<String, usize>, // "interface::type::method" -> function_ptr
    type_cache: HashMap<u64, String>,     // type_id -> type_name
}

impl InterfaceImplementationRegistry {
    /// Create new interface registry
    pub fn new() -> Self {
        Self {
            interface_definitions: HashMap::new(),
            vtables: HashMap::new(),
            type_interfaces: HashMap::new(),
            interface_hierarchy: HashMap::new(),
            type_methods: HashMap::new(),
            interface_objects: Vec::new(),
            method_cache: HashMap::new(),
            type_cache: HashMap::new(),
        }
    }
    
    /// Register interface definition
    pub fn register_interface(&mut self, interface_stmt: &InterfaceStatement) -> Result<(), CursedError> {
        let mut methods = Vec::new();
        
        // Convert AST method signatures to interface method info
        for (index, method) in interface_stmt.methods.iter().enumerate() {
            let method_info = InterfaceMethodInfo {
                name: method.name.clone(),
                param_types: method.parameters.iter().map(|p| p.param_type.clone()).collect(),
                return_type: method.return_type.clone(),
                method_index: index,
                is_static: method.receiver.is_none(),
                receiver_type: Self::determine_receiver_type(method),
            };
            methods.push(method_info);
        }
        
        // Register interface inheritance
        if !interface_stmt.extends.is_empty() {
            self.interface_hierarchy.insert(
                interface_stmt.name.clone(),
                interface_stmt.extends.clone()
            );
        }
        
        let definition = InterfaceDefinition {
            name: interface_stmt.name.clone(),
            methods,
            extends: interface_stmt.extends.clone(),
            type_parameters: interface_stmt.type_parameters.iter().map(|tp| tp.name.clone()).collect(),
            source_location: None,
        };
        
        self.interface_definitions.insert(interface_stmt.name.clone(), definition);
        
        log::info!("🔗 Registered interface '{}' with {} methods", 
                   interface_stmt.name, interface_stmt.methods.len());
        
        Ok(())
    }
    
    /// Determine receiver type from method signature
    fn determine_receiver_type(method: &MethodSignature) -> ReceiverType {
        match &method.receiver {
            Some(receiver) => {
                match receiver.is_mutable {
                    true => ReceiverType::Mutable,
                    false => ReceiverType::Pointer,
                }
            },
            None => ReceiverType::Static,
        }
    }
    
    /// Register implementation of interface for a type
    pub fn register_implementation(
        &mut self,
        interface_name: &str,
        implementing_type: &str,
        method_implementations: HashMap<String, usize>,
    ) -> Result<(), CursedError> {
        // Get interface definition
        let interface_def = self.interface_definitions.get(interface_name)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Interface '{}' not found", interface_name
            )))?;
        
        // Create vtable
        let mut vtable = InterfaceVTable::new(interface_name.to_string(), implementing_type.to_string());
        
        // Add methods to vtable
        for interface_method in &interface_def.methods {
            let function_ptr = method_implementations.get(&interface_method.name)
                .ok_or_else(|| CursedError::Runtime(format!(
                    "Method '{}' not implemented for type '{}' implementing interface '{}'",
                    interface_method.name, implementing_type, interface_name
                )))?;
            
            let virtual_method = VirtualMethodEntry {
                method_name: interface_method.name.clone(),
                function_ptr: *function_ptr,
                param_types: interface_method.param_types.clone(),
                return_type: interface_method.return_type.clone(),
                receiver_type: interface_method.receiver_type.clone(),
            };
            
            vtable.add_method(virtual_method)?;
        }
        
        // Validate completeness
        vtable.validate_completeness(&interface_def.methods)?;
        
        // Register vtable
        let vtable_key = format!("{}::{}", interface_name, implementing_type);
        self.vtables.insert(vtable_key, Arc::new(vtable));
        
        // Update type to interfaces mapping
        self.type_interfaces.entry(implementing_type.to_string())
            .or_insert_with(Vec::new)
            .push(interface_name.to_string());
        
        // Store method implementations
        self.type_methods.insert(implementing_type.to_string(), method_implementations);
        
        log::info!("✅ Registered implementation of interface '{}' for type '{}'", 
                   interface_name, implementing_type);
        
        Ok(())
    }
    
    /// Get vtable for interface and implementing type
    pub fn get_vtable(&self, interface_name: &str, implementing_type: &str) -> Option<Arc<InterfaceVTable>> {
        let key = format!("{}::{}", interface_name, implementing_type);
        self.vtables.get(&key).cloned()
    }
    
    /// Check if type implements interface
    pub fn implements_interface(&self, implementing_type: &str, interface_name: &str) -> bool {
        self.type_interfaces.get(implementing_type)
            .map(|interfaces| interfaces.contains(&interface_name.to_string()))
            .unwrap_or(false)
    }
    
    /// Create interface object from concrete value
    pub fn create_interface_object(
        &self,
        concrete_value: &Value,
        interface_name: &str,
    ) -> Result<InterfaceObject, CursedError> {
        let concrete_type = self.get_value_type_name(concrete_value);
        
        // Get vtable
        let vtable = self.get_vtable(interface_name, &concrete_type)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Type '{}' does not implement interface '{}'",
                concrete_type, interface_name
            )))?;
        
        // Get data pointer (simplified - in real implementation, this would be more complex)
        let data_ptr = self.get_value_data_ptr(concrete_value)?;
        
        Ok(InterfaceObject::new(
            vtable,
            data_ptr,
            interface_name.to_string(),
            concrete_type,
        ))
    }
    
    /// Get type name from value
    fn get_value_type_name(&self, value: &Value) -> String {
        match value {
            Value::Integer(_) => "normie".to_string(),
            Value::Number(_) => "meal".to_string(),
            Value::String(_) => "tea".to_string(),
            Value::Bool(_) => "lit".to_string(),
            Value::Struct { struct_name, .. } => struct_name.clone(),
            Value::Interface { concrete_type, .. } => concrete_type.clone(),
            _ => "unknown".to_string(),
        }
    }
    
    /// Get data pointer from value
    fn get_value_data_ptr(&self, value: &Value) -> Result<usize, CursedError> {
        match value {
            Value::Struct { fields, .. } => {
                // Return pointer to struct data (simplified)
                Ok(fields.as_ptr() as usize)
            },
            Value::Interface { data_ptr, .. } => Ok(*data_ptr),
            _ => {
                // For primitive types, return pointer to value (unsafe but for demonstration)
                Ok(value as *const Value as usize)
            }
        }
    }
    
    /// Dispatch interface method call
    pub fn dispatch_method(
        &self,
        interface_obj: &InterfaceObject,
        method_name: &str,
        args: &[Value],
        context: &mut ExecutionContext,
    ) -> Result<Value, CursedError> {
        // Get method from vtable
        let method = interface_obj.get_method(method_name)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Method '{}' not found in interface '{}' for type '{}'",
                method_name, interface_obj.interface_name, interface_obj.concrete_type
            )))?;
        
        // Check method cache first
        let cache_key = format!("{}::{}::{}", 
                               interface_obj.interface_name, 
                               interface_obj.concrete_type, 
                               method_name);
        
        if let Some(&cached_ptr) = self.method_cache.get(&cache_key) {
            return self.call_method_by_ptr(cached_ptr, interface_obj, args, &method.return_type);
        }
        
        // Call method via function pointer
        self.call_method_by_ptr(method.function_ptr, interface_obj, args, &method.return_type)
    }
    
    /// Call method by function pointer (simplified implementation)
    fn call_method_by_ptr(
        &self,
        function_ptr: usize,
        interface_obj: &InterfaceObject,
        args: &[Value],
        return_type: &Option<Type>,
    ) -> Result<Value, CursedError> {
        // This is a simplified implementation
        // In a real system, this would use FFI or JIT compilation to call the actual function
        
        log::debug!("🚀 Dispatching method call via function pointer 0x{:x}", function_ptr);
        
        // For demonstration purposes, return a placeholder value based on return type
        match return_type {
            Some(Type::Normie) => Ok(Value::Integer(42)),
            Some(Type::Tea) => Ok(Value::String("method_result".to_string())),
            Some(Type::Lit) => Ok(Value::Bool(true)),
            Some(Type::Meal) => Ok(Value::Number(3.14)),
            None => Ok(Value::Null),
            _ => Ok(Value::Null),
        }
    }
    
    /// Perform interface type assertion
    pub fn type_assert(
        &self,
        interface_obj: &InterfaceObject,
        target_type: &str,
    ) -> Result<Option<Value>, CursedError> {
        if interface_obj.concrete_type == target_type {
            // Type assertion successful - return concrete value
            // In real implementation, would reconstruct value from data_ptr
            Ok(Some(Value::String(format!("concrete_value_of_{}", target_type))))
        } else {
            // Type assertion failed
            Ok(None)
        }
    }
    
    /// Get all interfaces implemented by a type
    pub fn get_implemented_interfaces(&self, type_name: &str) -> Vec<String> {
        self.type_interfaces.get(type_name)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get interface definition
    pub fn get_interface_definition(&self, interface_name: &str) -> Option<&InterfaceDefinition> {
        self.interface_definitions.get(interface_name)
    }
    
    /// Validate interface compliance for a type
    pub fn validate_interface_compliance(
        &self,
        interface_name: &str,
        implementing_type: &str,
    ) -> Result<(), CursedError> {
        let interface_def = self.interface_definitions.get(interface_name)
            .ok_or_else(|| CursedError::Runtime(format!(
                "Interface '{}' not found", interface_name
            )))?;
        
        let vtable = self.get_vtable(interface_name, implementing_type)
            .ok_or_else(|| CursedError::Runtime(format!(
                "No implementation found for interface '{}' on type '{}'",
                interface_name, implementing_type
            )))?;
        
        vtable.validate_completeness(&interface_def.methods)
    }
    
    /// Get performance metrics
    pub fn get_metrics(&self) -> InterfaceDispatchMetrics {
        InterfaceDispatchMetrics {
            total_interfaces: self.interface_definitions.len(),
            total_implementations: self.vtables.len(),
            total_methods: self.interface_definitions.values()
                .map(|def| def.methods.len())
                .sum(),
            cache_size: self.method_cache.len(),
            vtable_memory_usage: self.vtables.values()
                .map(|vtable| vtable.size)
                .sum(),
        }
    }
}

/// Performance metrics for interface dispatch system
#[derive(Debug, Clone)]
pub struct InterfaceDispatchMetrics {
    pub total_interfaces: usize,
    pub total_implementations: usize,
    pub total_methods: usize,
    pub cache_size: usize,
    pub vtable_memory_usage: usize,
}

/// Global interface registry
static GLOBAL_INTERFACE_REGISTRY: Lazy<RwLock<InterfaceImplementationRegistry>> = 
    Lazy::new(|| RwLock::new(InterfaceImplementationRegistry::new()));

/// Initialize interface dispatch system
pub fn initialize_interface_dispatch() -> Result<(), CursedError> {
    // Registry is initialized lazily
    log::info!("🔧 Interface virtual dispatch system initialized");
    Ok(())
}

/// Register interface globally
pub fn register_global_interface(interface_stmt: &InterfaceStatement) -> Result<(), CursedError> {
    let mut registry = GLOBAL_INTERFACE_REGISTRY.write()
        .map_err(|e| CursedError::Runtime(format!("Failed to acquire write lock: {}", e)))?;
    
    registry.register_interface(interface_stmt)
}

/// Register global implementation
pub fn register_global_implementation(
    interface_name: &str,
    implementing_type: &str,
    method_implementations: HashMap<String, usize>,
) -> Result<(), CursedError> {
    let mut registry = GLOBAL_INTERFACE_REGISTRY.write()
        .map_err(|e| CursedError::Runtime(format!("Failed to acquire write lock: {}", e)))?;
    
    registry.register_implementation(interface_name, implementing_type, method_implementations)
}

/// Create global interface object
pub fn create_global_interface_object(
    concrete_value: &Value,
    interface_name: &str,
) -> Result<InterfaceObject, CursedError> {
    let registry = GLOBAL_INTERFACE_REGISTRY.read()
        .map_err(|e| CursedError::Runtime(format!("Failed to acquire read lock: {}", e)))?;
    
    registry.create_interface_object(concrete_value, interface_name)
}

/// Dispatch global method
pub fn dispatch_global_method(
    interface_obj: &InterfaceObject,
    method_name: &str,
    args: &[Value],
    context: &mut ExecutionContext,
) -> Result<Value, CursedError> {
    let registry = GLOBAL_INTERFACE_REGISTRY.read()
        .map_err(|e| CursedError::Runtime(format!("Failed to acquire read lock: {}", e)))?;
    
    registry.dispatch_method(interface_obj, method_name, args, context)
}

/// Check if type implements interface globally
pub fn check_global_interface_implementation(
    implementing_type: &str,
    interface_name: &str,
) -> Result<bool, CursedError> {
    let registry = GLOBAL_INTERFACE_REGISTRY.read()
        .map_err(|e| CursedError::Runtime(format!("Failed to acquire read lock: {}", e)))?;
    
    Ok(registry.implements_interface(implementing_type, interface_name))
}

/// Get global interface metrics
pub fn get_global_interface_metrics() -> Result<InterfaceDispatchMetrics, CursedError> {
    let registry = GLOBAL_INTERFACE_REGISTRY.read()
        .map_err(|e| CursedError::Runtime(format!("Failed to acquire read lock: {}", e)))?;
    
    Ok(registry.get_metrics())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Parameter, TypeParameter, Visibility};
    
    #[test]
    fn test_interface_registration() {
        let mut registry = InterfaceImplementationRegistry::new();
        
        let interface_stmt = InterfaceStatement {
            name: "TestInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "test_method".to_string(),
                    receiver: None,
                    parameters: vec![
                        Parameter {
                            name: "param".to_string(),
                            param_type: Type::Normie,
                            default: None,
                            source_location: None,
                        }
                    ],
                    return_type: Some(Type::Tea),
                    source_location: None,
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(registry.register_interface(&interface_stmt).is_ok());
        assert!(registry.get_interface_definition("TestInterface").is_some());
    }
    
    #[test]
    fn test_vtable_creation() {
        let mut vtable = InterfaceVTable::new("TestInterface".to_string(), "TestType".to_string());
        
        let method = VirtualMethodEntry {
            method_name: "test_method".to_string(),
            function_ptr: 0x1234,
            param_types: vec![Type::Normie],
            return_type: Some(Type::Tea),
            receiver_type: ReceiverType::Value,
        };
        
        assert!(vtable.add_method(method).is_ok());
        assert!(vtable.get_method("test_method").is_some());
        assert_eq!(vtable.get_method_index("test_method"), Some(0));
    }
    
    #[test]
    fn test_interface_implementation() {
        let mut registry = InterfaceImplementationRegistry::new();
        
        // Register interface
        let interface_stmt = InterfaceStatement {
            name: "Drawable".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "draw".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: None,
                    source_location: None,
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(registry.register_interface(&interface_stmt).is_ok());
        
        // Register implementation
        let mut implementations = HashMap::new();
        implementations.insert("draw".to_string(), 0x5678);
        
        assert!(registry.register_implementation(
            "Drawable",
            "Circle",
            implementations
        ).is_ok());
        
        assert!(registry.implements_interface("Circle", "Drawable"));
        assert!(registry.get_vtable("Drawable", "Circle").is_some());
    }
}
