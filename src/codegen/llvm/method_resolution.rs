//! Method resolution and interface satisfaction checking for CURSED
//!
//! This module handles the process of resolving method calls and ensuring
//! that types properly implement interface methods.

use crate::ast::declarations::{MethodDeclaration, MethodSignature, CollabStatement, SquadStatement};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use std::collections::HashMap;
use tracing::{debug, info, instrument};

/// Information about a method implementation
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub receiver_type: String,
    pub parameter_types: Vec<String>,
    pub return_type: Option<String>,
    pub is_pointer_receiver: bool,
}

/// Trait for method resolution and interface satisfaction
pub trait MethodResolution<'ctx> {
    /// Register a method implementation for a type
    fn register_type_method(&mut self, type_name: &str, method: &MethodInfo) -> Result<(), Error>;
    
    /// Check if a type implements all methods required by an interface
    fn check_interface_satisfaction(&self, type_name: &str, interface: &CollabStatement) -> Result<bool, Error>;
    
    /// Get all methods implemented by a type
    fn get_type_methods(&self, type_name: &str) -> Vec<&MethodInfo>;
    
    /// Resolve a method call to find the implementation
    fn resolve_method_call(&self, receiver_type: &str, method_name: &str) -> Option<&MethodInfo>;
    
    /// Register interface methods from a CollabStatement
    fn register_interface_methods(&mut self, interface: &CollabStatement) -> Result<(), Error>;
    
    /// Register struct methods from method declarations
    fn register_struct_methods_from_declarations(&mut self, struct_name: &str, methods: Vec<&MethodDeclaration>) -> Result<(), Error>;
}

impl<'ctx> MethodResolution<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn register_type_method(&mut self, type_name: &str, method: &MethodInfo) -> Result<(), Error> {
        debug!("Registering method {} for type {}", method.name, type_name);
        
        // Initialize method info registry if needed
        if self.method_info_registry.is_none() {
            self.method_info_registry = Some(HashMap::new());
        }
        
        if let Some(registry) = &mut self.method_info_registry {
            let type_methods = registry.entry(type_name.to_string())
                .or_insert_with(Vec::new);
            type_methods.push(method.clone());
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, interface), level = "debug")]
    fn check_interface_satisfaction(&self, type_name: &str, interface: &CollabStatement) -> Result<bool, Error> {
        info!("Checking if type {} satisfies interface {}", type_name, interface.name.value);
        
        let type_methods = match &self.method_info_registry {
            Some(registry) => registry.get(type_name).map(|v| v.as_slice()).unwrap_or(&[]),
            None => &[],
        };
        
        // Check each required interface method
        for interface_method in &interface.methods {
            let mut found = false;
            
            for type_method in type_methods {
                if self.methods_match(&interface_method, type_method)? {
                    found = true;
                    break;
                }
            }
            
            if !found {
                debug!("Type {} missing method {} required by interface {}", 
                      type_name, interface_method.name.value, interface.name.value);
                return Ok(false);
            }
        }
        
        info!("Type {} successfully satisfies interface {}", type_name, interface.name.value);
        Ok(true)
    }
    
    fn get_type_methods(&self, type_name: &str) -> Vec<&MethodInfo> {
        match &self.method_info_registry {
            Some(registry) => {
                registry.get(type_name)
                    .map(|methods| methods.iter().collect())
                    .unwrap_or_else(Vec::new)
            },
            None => Vec::new(),
        }
    }
    
    fn resolve_method_call(&self, receiver_type: &str, method_name: &str) -> Option<&MethodInfo> {
        if let Some(registry) = &self.method_info_registry {
            if let Some(type_methods) = registry.get(receiver_type) {
                return type_methods.iter().find(|method| method.name == method_name);
            }
        }
        None
    }
    
    #[instrument(skip(self, interface), level = "debug")]
    fn register_interface_methods(&mut self, interface: &CollabStatement) -> Result<(), Error> {
        info!("Registering interface methods for {}", interface.name.value);
        
        // Initialize interface method registry if needed
        if self.interface_method_registry.is_none() {
            self.interface_method_registry = Some(HashMap::new());
        }
        
        if let Some(registry) = &mut self.interface_method_registry {
            let mut interface_methods = Vec::new();
            
            for method_sig in &interface.methods {
                let method_info = MethodInfo {
                    name: method_sig.name.value.clone(),
                    receiver_type: interface.name.value.clone(),
                    parameter_types: method_sig.parameters.iter()
                        .map(|p| p.name.value.clone()) // Simplified for now
                        .collect(),
                    return_type: method_sig.return_type.as_ref()
                        .map(|rt| rt.string()),
                    is_pointer_receiver: false, // Interfaces don't specify receiver type
                };
                interface_methods.push(method_info);
            }
            
            registry.insert(interface.name.value.clone(), interface_methods);
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, methods), level = "debug")]
    fn register_struct_methods_from_declarations(&mut self, struct_name: &str, methods: Vec<&MethodDeclaration>) -> Result<(), Error> {
        info!("Registering {} methods for struct {}", methods.len(), struct_name);
        
        for method in methods {
            let method_info = MethodInfo {
                name: method.name.value.clone(),
                receiver_type: struct_name.to_string(),
                parameter_types: method.parameters.iter()
                    .map(|p| format!("param_{}", p.name.value)) // Simplified for now
                    .collect(),
                return_type: method.return_type.as_ref()
                    .map(|rt| rt.string()),
                is_pointer_receiver: method.receiver.is_pointer,
            };
            
            self.register_type_method(struct_name, &method_info)?;
        }
        
        Ok(())
    }
}

/// Helper methods for method resolution
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if an interface method signature matches a type method implementation
    fn methods_match(&self, interface_method: &MethodSignature, type_method: &MethodInfo) -> Result<bool, Error> {
        // Check method name
        if interface_method.name.value != type_method.name {
            return Ok(false);
        }
        
        // Check parameter count (simplified check for now)
        if interface_method.parameters.len() != type_method.parameter_types.len() {
            return Ok(false);
        }
        
        // Check return type presence
        let interface_has_return = interface_method.return_type.is_some();
        let type_has_return = type_method.return_type.is_some();
        
        if interface_has_return != type_has_return {
            return Ok(false);
        }
        
        // TODO: Add more sophisticated type checking
        // For now, we'll accept any match of name and parameter count
        
        Ok(true)
    }
}
