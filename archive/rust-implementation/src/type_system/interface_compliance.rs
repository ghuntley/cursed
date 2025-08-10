// Interface Compliance Checking for CURSED Type System
//
// This module implements interface compliance checking including:
// - Method set comparison
// - Parameter and return type compatibility
// - Pointer vs value receiver handling
// - Auto-dereference rules for interfaces

use crate::ast::{InterfaceStatement, MethodSignature, Parameter, Type as AstType, StructStatement, TypeParameter as AstTypeParameter};
use once_cell::sync::Lazy;
use crate::error::SourceLocation;
use crate::core::Type;
use crate::error_types::CursedError;
use std::collections::HashMap;

/// Interface method requirement
#[derive(Debug, Clone)]
pub struct InterfaceMethodRequirement {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<AstType>,
    pub receiver_type: ReceiverType,
    pub source_location: Option<SourceLocation>,
}

/// Generic interface definition
#[derive(Debug, Clone)]
pub struct GenericInterfaceDefinition {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub extends: Vec<String>,
    pub methods: Vec<InterfaceMethodRequirement>,
}

/// Type parameter for generics
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub bounds: Vec<String>,
}

/// Receiver type for interface methods
#[derive(Debug, Clone, PartialEq)]
pub enum ReceiverType {
    Value,
    Pointer,
    Any, // Can be satisfied by both value and pointer receivers
}

/// Concrete type method implementation
#[derive(Debug, Clone)]
pub struct ConcreteMethodImplementation {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<AstType>,
    pub receiver_type: ReceiverType,
    pub source_location: Option<SourceLocation>,
}

/// Interface compliance checker
#[derive(Debug, Clone)]
pub struct InterfaceComplianceChecker {
    /// Map from interface name to method requirements
    interface_methods: HashMap<String, Vec<InterfaceMethodRequirement>>,
    /// Map from type name to method implementations
    type_implementations: HashMap<String, Vec<ConcreteMethodImplementation>>,
    /// Map from interface name to generic interface definitions
    generic_interfaces: HashMap<String, GenericInterfaceDefinition>,
    /// Interface inheritance hierarchy
    interface_hierarchy: HashMap<String, Vec<String>>,
}

impl InterfaceComplianceChecker {
    /// Create a new interface compliance checker
    pub fn new() -> Self {
        Self {
            interface_methods: HashMap::new(),
            type_implementations: HashMap::new(),
            generic_interfaces: HashMap::new(),
            interface_hierarchy: HashMap::new(),
        }
    }
    
    /// Register an interface definition
    pub fn register_interface(&mut self, interface: &InterfaceStatement) -> Result<(), CursedError> {
        let mut method_requirements = Vec::new();
        
        for method in &interface.methods {
            // Determine receiver type based on method signature
            let receiver_type = self.determine_receiver_type(method)?;
            
            let requirement = InterfaceMethodRequirement {
                name: method.name.clone(),
                parameters: method.parameters.clone(),
                return_type: method.return_type.clone(),
                receiver_type,
                source_location: method.source_location.clone(),
            };
            
            method_requirements.push(requirement);
        }
        
        // Register interface hierarchy if it extends other interfaces
        if !interface.extends.is_empty() {
            self.interface_hierarchy.insert(interface.name.clone(), interface.extends.clone());
        }
        
        // If interface has type parameters, register it as a generic interface
        if !interface.type_parameters.is_empty() {
            let generic_def = GenericInterfaceDefinition {
                name: interface.name.clone(),
                type_parameters: interface.type_parameters.iter().map(|tp| TypeParameter {
                    name: tp.name.clone(),
                    bounds: tp.bounds.clone(),
                }).collect(),
                extends: interface.extends.clone(),
                methods: method_requirements.clone(),
            };
            self.generic_interfaces.insert(interface.name.clone(), generic_def);
        }
        
        self.interface_methods.insert(interface.name.clone(), method_requirements);
        Ok(())
    }
    
    /// Register a concrete type's method implementations
    pub fn register_type_methods(&mut self, type_name: &str, methods: Vec<ConcreteMethodImplementation>) -> Result<(), CursedError> {
        self.type_implementations.insert(type_name.to_string(), methods);
        Ok(())
    }
    
    /// Check if a concrete type implements an interface
    pub fn check_interface_compliance(&self, type_name: &str, interface_name: &str) -> Result<bool, CursedError> {
        // Get all interface requirements (including inherited ones)
        let all_requirements = self.get_all_interface_requirements(interface_name)?;
        
        // Get type implementations
        let type_methods = self.type_implementations.get(type_name)
            .ok_or_else(|| CursedError::Runtime(format!("Type '{}' not found", type_name)))?;
        
        // Check each interface method is implemented
        for requirement in &all_requirements {
            if !self.is_method_implemented(requirement, type_methods)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Get all interface requirements including inherited ones
    pub fn get_all_interface_requirements(&self, interface_name: &str) -> Result<Vec<InterfaceMethodRequirement>, CursedError> {
        let mut all_requirements = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        self.collect_interface_requirements(interface_name, &mut all_requirements, &mut visited)?;
        
        // Remove duplicate methods (derived interface methods override base interface methods)
        self.deduplicate_methods(&mut all_requirements);
        
        Ok(all_requirements)
    }
    
    /// Remove duplicate methods, keeping derived interface methods over base interface methods
    fn deduplicate_methods(&self, requirements: &mut Vec<InterfaceMethodRequirement>) {
        let mut seen_methods = std::collections::HashMap::new();
        let mut to_remove = Vec::new();
        
        for (i, req) in requirements.iter().enumerate() {
            if seen_methods.contains_key(&req.name) {
                to_remove.push(i);
            } else {
                seen_methods.insert(req.name.clone(), i);
            }
        }
        
        // Remove duplicates in reverse order to preserve indices
        for &i in to_remove.iter().rev() {
            requirements.remove(i);
        }
    }

    /// Recursively collect interface requirements including inheritance
    fn collect_interface_requirements(
        &self,
        interface_name: &str,
        requirements: &mut Vec<InterfaceMethodRequirement>,
        visited: &mut std::collections::HashSet<String>
    ) -> Result<(), CursedError> {
        if visited.contains(interface_name) {
            return Err(CursedError::Runtime(format!("Circular interface inheritance detected: {}", interface_name)));
        }
        
        visited.insert(interface_name.to_string());
        
        // Add this interface's methods
        if let Some(interface_methods) = self.interface_methods.get(interface_name) {
            requirements.extend(interface_methods.clone());
        }
        
        // Add inherited interface methods
        if let Some(extends) = self.interface_hierarchy.get(interface_name) {
            for parent_interface in extends {
                self.collect_interface_requirements(parent_interface, requirements, visited)?;
            }
        }
        
        visited.remove(interface_name);
        Ok(())
    }
    
    /// Check if a specific method requirement is satisfied
    fn is_method_implemented(&self, requirement: &InterfaceMethodRequirement, implementations: &[ConcreteMethodImplementation]) -> Result<bool, CursedError> {
        // Find method with matching name
        let implementation = implementations.iter()
            .find(|impl_method| impl_method.name == requirement.name);
        
        let implementation = match implementation {
            Some(impl_method) => impl_method,
            None => return Ok(false), // Method not found
        };
        
        // Check parameter compatibility
        if !self.check_parameter_compatibility(&requirement.parameters, &implementation.parameters)? {
            return Ok(false);
        }
        
        // Check return type compatibility
        if !self.check_return_type_compatibility(&requirement.return_type, &implementation.return_type)? {
            return Ok(false);
        }
        
        // Check receiver type compatibility
        if !self.check_receiver_compatibility(&requirement.receiver_type, &implementation.receiver_type)? {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Check parameter list compatibility
    fn check_parameter_compatibility(&self, required: &[Parameter], provided: &[Parameter]) -> Result<bool, CursedError> {
        if required.len() != provided.len() {
            return Ok(false);
        }
        
        for (req_param, prov_param) in required.iter().zip(provided.iter()) {
            if !self.check_type_compatibility(&req_param.param_type, &prov_param.param_type)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check return type compatibility
    fn check_return_type_compatibility(&self, required: &Option<AstType>, provided: &Option<AstType>) -> Result<bool, CursedError> {
        match (required, provided) {
            (None, None) => Ok(true),
            (Some(req), Some(prov)) => self.check_type_compatibility(&Some(req.clone()), &Some(prov.clone())),
            _ => Ok(false), // One has return type, other doesn't
        }
    }
    
    /// Check receiver type compatibility
    fn check_receiver_compatibility(&self, required: &ReceiverType, provided: &ReceiverType) -> Result<bool, CursedError> {
        match (required, provided) {
            (ReceiverType::Any, _) => Ok(true),
            (ReceiverType::Value, ReceiverType::Value) => Ok(true),
            (ReceiverType::Pointer, ReceiverType::Pointer) => Ok(true),
            (ReceiverType::Value, ReceiverType::Pointer) => Ok(true), // Auto-dereference
            (ReceiverType::Pointer, ReceiverType::Value) => Ok(false), // Can't take address of value
            (ReceiverType::Value, ReceiverType::Any) => Ok(true), // Value receiver can accept any
            (ReceiverType::Pointer, ReceiverType::Any) => Ok(true), // Pointer receiver can accept any
        }
    }
    
    /// Check type compatibility
    fn check_type_compatibility(&self, required: &Option<AstType>, provided: &Option<AstType>) -> Result<bool, CursedError> {
        match (required, provided) {
            (None, None) => Ok(true),
            (Some(req), Some(prov)) => {
                // Enhanced type compatibility checking
                self.check_ast_type_compatibility(req, prov)
            },
            _ => Ok(false),
        }
    }
    
    /// Check compatibility between two AST types
    fn check_ast_type_compatibility(&self, required: &AstType, provided: &AstType) -> Result<bool, CursedError> {
        match (required, provided) {
            // Exact type match
            (AstType::Normie, AstType::Normie) => Ok(true),
            (AstType::Thicc, AstType::Thicc) => Ok(true),
            (AstType::Smol, AstType::Smol) => Ok(true),
            (AstType::Mid, AstType::Mid) => Ok(true),
            (AstType::Meal, AstType::Meal) => Ok(true),
            (AstType::Snack, AstType::Snack) => Ok(true),
            (AstType::Tea, AstType::Tea) => Ok(true),
            (AstType::Lit, AstType::Lit) => Ok(true),
            (AstType::Sip, AstType::Sip) => Ok(true),
            (AstType::Byte, AstType::Byte) => Ok(true),
            (AstType::Rune, AstType::Rune) => Ok(true),
            (AstType::Extra, AstType::Extra) => Ok(true),
            (AstType::Integer, AstType::Integer) => Ok(true),
            (AstType::Float, AstType::Float) => Ok(true),
            (AstType::String, AstType::String) => Ok(true),
            (AstType::Boolean, AstType::Boolean) => Ok(true),
            (AstType::Void, AstType::Void) => Ok(true),
            
            // Custom types (named types)
            (AstType::Custom(req_name), AstType::Custom(prov_name)) => {
                Ok(req_name == prov_name)
            },
            
            // Interface types (collab)
            (AstType::Collab(req_name), AstType::Collab(prov_name)) => {
                Ok(req_name == prov_name)
            },
            
            // Pointer types
            (AstType::Pointer(req_inner), AstType::Pointer(prov_inner)) => {
                self.check_ast_type_compatibility(req_inner, prov_inner)
            },
            
            // Array types
            (AstType::Array(req_inner, req_size), AstType::Array(prov_inner, prov_size)) => {
                // Compare size expressions (simplified for now)
                let sizes_match = match (req_size, prov_size) {
                    (Some(_), Some(_)) => true, // Simplified - should compare expressions
                    (None, None) => true,
                    _ => false,
                };
                Ok(sizes_match && self.check_ast_type_compatibility(req_inner, prov_inner)?)
            },
            
            // Slice types
            (AstType::Slice(req_inner), AstType::Slice(prov_inner)) => {
                self.check_ast_type_compatibility(req_inner, prov_inner)
            },
            
            // Channel types (dm)
            (AstType::Dm(req_inner), AstType::Dm(prov_inner)) => {
                self.check_ast_type_compatibility(req_inner, prov_inner)
            },
            
            // Function types
            (AstType::Function(req_params, req_return), AstType::Function(prov_params, prov_return)) => {
                // Check parameter count
                if req_params.len() != prov_params.len() {
                    return Ok(false);
                }
                
                // Check parameter compatibility
                for (req_param, prov_param) in req_params.iter().zip(prov_params.iter()) {
                    if !self.check_ast_type_compatibility(req_param, prov_param)? {
                        return Ok(false);
                    }
                }
                
                // Check return type compatibility
                self.check_ast_type_compatibility(req_return, prov_return)
            },
            
            // Tuple types
            (AstType::Tuple(req_types), AstType::Tuple(prov_types)) => {
                if req_types.len() != prov_types.len() {
                    return Ok(false);
                }
                
                for (req_type, prov_type) in req_types.iter().zip(prov_types.iter()) {
                    if !self.check_ast_type_compatibility(req_type, prov_type)? {
                        return Ok(false);
                    }
                }
                
                Ok(true)
            },
            
            // Generic types
            (AstType::Generic(req_name, req_params), AstType::Generic(prov_name, prov_params)) => {
                if req_name != prov_name || req_params.len() != prov_params.len() {
                    return Ok(false);
                }
                
                for (req_param, prov_param) in req_params.iter().zip(prov_params.iter()) {
                    if !self.check_ast_type_compatibility(req_param, prov_param)? {
                        return Ok(false);
                    }
                }
                
                Ok(true)
            },
            
            // Interface types - allow any concrete type that implements the interface
            (AstType::Collab(interface_name), _) => {
                // This would need to be checked against the actual type system
                // For now, allow interface to match any named type
                Ok(true)
            },
            
            // Numeric type coercion
            (AstType::Normie, AstType::Smol) => Ok(true),
            (AstType::Normie, AstType::Mid) => Ok(true),
            (AstType::Thicc, AstType::Normie) => Ok(true),
            (AstType::Thicc, AstType::Smol) => Ok(true),
            (AstType::Thicc, AstType::Mid) => Ok(true),
            (AstType::Meal, AstType::Snack) => Ok(true),
            
            // Everything else is incompatible
            _ => Ok(false),
        }
    }
    
    /// Determine receiver type from method signature
    fn determine_receiver_type(&self, method: &MethodSignature) -> Result<ReceiverType, CursedError> {
        // Check if the method has a receiver parameter
        if let Some(first_param) = method.parameters.first() {
            // Check if first parameter is a receiver (self-like)
            if first_param.name == "self" || first_param.name.starts_with("self") {
                // Determine receiver type based on parameter type
                match &first_param.param_type {
                    Some(param_type) => {
                        // Check for pointer/reference types indicating pointer receiver
                        match param_type {
                            crate::ast::Type::Pointer(_) => {
                                Ok(ReceiverType::Pointer)
                            }
                            _ => Ok(ReceiverType::Value)
                        }
                    }
                    None => {
                        // No explicit type, check parameter name for hints
                        if first_param.name.contains("ptr") || first_param.name.contains("ref") {
                            Ok(ReceiverType::Pointer)
                        } else {
                            Ok(ReceiverType::Value)
                        }
                    }
                }
            } else {
                // Check if first parameter type suggests it's a receiver
                if let Some(param_type) = &first_param.param_type {
                    match param_type {
                        crate::ast::Type::Pointer(_) => {
                            Ok(ReceiverType::Pointer)
                        }
                        _ => Ok(ReceiverType::Value)
                    }
                } else {
                    // Default to value receiver
                    Ok(ReceiverType::Value)
                }
            }
        } else {
            // No parameters, assume value receiver (functions without receivers)
            Ok(ReceiverType::Value)
        }
    }
    
    /// Get interface method requirements
    pub fn get_interface_methods(&self, interface_name: &str) -> Option<&Vec<InterfaceMethodRequirement>> {
        self.interface_methods.get(interface_name)
    }
    
    /// Get type method implementations
    pub fn get_type_methods(&self, type_name: &str) -> Option<&Vec<ConcreteMethodImplementation>> {
        self.type_implementations.get(type_name)
    }
    
    /// Check if one interface extends another (inheritance check)
    pub fn interface_extends(&self, derived_interface: &str, base_interface: &str) -> bool {
        if derived_interface == base_interface {
            return true;
        }
        
        // Check direct inheritance
        if let Some(parents) = self.interface_hierarchy.get(derived_interface) {
            if parents.contains(&base_interface.to_string()) {
                return true;
            }
            
            // Check transitive inheritance
            for parent in parents {
                if self.interface_extends(parent, base_interface) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get all parent interfaces (including transitive)
    pub fn get_all_parent_interfaces(&self, interface_name: &str) -> Result<Vec<String>, CursedError> {
        let mut all_parents = Vec::new();
        let mut visited = std::collections::HashSet::new();
        
        self.collect_parent_interfaces(interface_name, &mut all_parents, &mut visited)?;
        
        Ok(all_parents)
    }
    
    /// Recursively collect parent interfaces
    fn collect_parent_interfaces(
        &self,
        interface_name: &str,
        parents: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>
    ) -> Result<(), CursedError> {
        if visited.contains(interface_name) {
            return Err(CursedError::Runtime(format!("Circular interface inheritance detected: {}", interface_name)));
        }
        
        visited.insert(interface_name.to_string());
        
        // Add direct parents
        if let Some(direct_parents) = self.interface_hierarchy.get(interface_name) {
            for parent in direct_parents {
                if !parents.contains(parent) {
                    parents.push(parent.clone());
                }
                
                // Recursively collect transitive parents
                self.collect_parent_interfaces(parent, parents, visited)?;
            }
        }
        
        visited.remove(interface_name);
        Ok(())
    }
    
    /// Generate interface compliance report
    pub fn generate_compliance_report(&self, type_name: &str, interface_name: &str) -> Result<InterfaceComplianceReport, CursedError> {
        let interface_methods = self.interface_methods.get(interface_name)
            .ok_or_else(|| CursedError::Runtime(format!("Interface '{}' not found", interface_name)))?;
        
        let type_methods = self.type_implementations.get(type_name)
            .ok_or_else(|| CursedError::Runtime(format!("Type '{}' not found", type_name)))?;
        
        let mut report = InterfaceComplianceReport {
            type_name: type_name.to_string(),
            interface_name: interface_name.to_string(),
            compliant: true,
            missing_methods: Vec::new(),
            incompatible_methods: Vec::new(),
        };
        
        // Check each interface method
        for requirement in interface_methods {
            match type_methods.iter().find(|impl_method| impl_method.name == requirement.name) {
                Some(implementation) => {
                    // Method exists, check compatibility
                    if let Some(incompatibility_reason) = self.check_method_incompatibility(requirement, implementation)? {
                        report.compliant = false;
                        report.incompatible_methods.push(IncompatibleMethod {
                            method_name: requirement.name.clone(),
                            reason: incompatibility_reason,
                        });
                    }
                },
                None => {
                    // Method missing
                    report.compliant = false;
                    report.missing_methods.push(requirement.name.clone());
                },
            }
        }
        
        Ok(report)
    }
    
    /// Check for method incompatibility and return detailed reason
    fn check_method_incompatibility(&self, requirement: &InterfaceMethodRequirement, implementation: &ConcreteMethodImplementation) -> Result<Option<String>, CursedError> {
        // Check parameter count
        if requirement.parameters.len() != implementation.parameters.len() {
            return Ok(Some(format!(
                "Parameter count mismatch: interface requires {} parameters, implementation has {}",
                requirement.parameters.len(),
                implementation.parameters.len()
            )));
        }
        
        // Check individual parameter types
        for (i, (req_param, impl_param)) in requirement.parameters.iter().zip(implementation.parameters.iter()).enumerate() {
            if !self.check_parameter_compatibility(&requirement.parameters, &implementation.parameters)? {
                return Ok(Some(format!(
                    "Parameter {} type mismatch: interface requires {:?}, implementation has {:?}",
                    i + 1,
                    req_param.param_type,
                    impl_param.param_type
                )));
            }
        }
        
        // Check return type compatibility
        if !self.check_return_type_compatibility(&requirement.return_type, &implementation.return_type)? {
            return Ok(Some(format!(
                "Return type mismatch: interface requires {:?}, implementation returns {:?}",
                requirement.return_type,
                implementation.return_type
            )));
        }
        
        // Check receiver type compatibility
        if !self.check_receiver_compatibility(&requirement.receiver_type, &implementation.receiver_type)? {
            return Ok(Some(format!(
                "Receiver type mismatch: interface requires {:?}, implementation has {:?}",
                requirement.receiver_type,
                implementation.receiver_type
            )));
        }
        
        Ok(None)
    }
}

/// Interface compliance report
#[derive(Debug, Clone)]
pub struct InterfaceComplianceReport {
    pub type_name: String,
    pub interface_name: String,
    pub compliant: bool,
    pub missing_methods: Vec<String>,
    pub incompatible_methods: Vec<IncompatibleMethod>,
}

/// Incompatible method information
#[derive(Debug, Clone)]
pub struct IncompatibleMethod {
    pub method_name: String,
    pub reason: String,
}

/// Global interface compliance checker
static GLOBAL_COMPLIANCE_CHECKER: Lazy<std::sync::Mutex<InterfaceComplianceChecker>> = Lazy::new(|| std::sync::Mutex::new(InterfaceComplianceChecker::new()));

/// Initialize global compliance checker
pub fn initialize_interface_compliance_checker() {
    unsafe {
        // Initialization handled by Lazy);
    }
}

/// Get global compliance checker
pub fn get_global_compliance_checker() -> Result<std::sync::MutexGuard<'static, InterfaceComplianceChecker>, CursedError> {
    Ok(GLOBAL_COMPLIANCE_CHECKER.lock().unwrap())
}

/// Check interface compliance using global checker
pub fn check_global_interface_compliance(type_name: &str, interface_name: &str) -> Result<bool, CursedError> {
    let checker = get_global_compliance_checker()?;
    checker.check_interface_compliance(type_name, interface_name)
}

/// Generate compliance report using global checker
pub fn generate_global_compliance_report(type_name: &str, interface_name: &str) -> Result<InterfaceComplianceReport, CursedError> {
    let checker = get_global_compliance_checker()?;
    checker.generate_compliance_report(type_name, interface_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    
    #[test]
    fn test_interface_registration() {
        let mut checker = InterfaceComplianceChecker::new();
        
        let interface = InterfaceStatement {
            name: "TestInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "test_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                    source_location: None,
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(checker.register_interface(&interface).is_ok());
        assert!(checker.get_interface_methods("TestInterface").is_some());
    }
    
    #[test]
    fn test_compliance_checking() {
        let mut checker = InterfaceComplianceChecker::new();
        
        // Register interface
        let interface = InterfaceStatement {
            name: "TestInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "test_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                    source_location: None,
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(checker.register_interface(&interface).is_ok());
        
        // Register type with implementation
        let implementation = ConcreteMethodImplementation {
            name: "test_method".to_string(),
            parameters: vec![],
            return_type: Some(AstType::Normie),
            receiver_type: ReceiverType::Value,
            source_location: None,
        };
        
        assert!(checker.register_type_methods("TestType", vec![implementation]).is_ok());
        
        // Check compliance
        assert!(checker.check_interface_compliance("TestType", "TestInterface").unwrap());
    }
    
    #[test]
    fn test_interface_inheritance() {
        let mut checker = InterfaceComplianceChecker::new();
        
        // Register base interface
        let base_interface = InterfaceStatement {
            name: "BaseInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "base_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                    source_location: None,
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(checker.register_interface(&base_interface).is_ok());
        
        // Register derived interface
        let derived_interface = InterfaceStatement {
            name: "DerivedInterface".to_string(),
            type_parameters: vec![],
            extends: vec!["BaseInterface".to_string()],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "derived_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                    source_location: None,
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(checker.register_interface(&derived_interface).is_ok());
        
        // Register type with implementations
        let implementations = vec![
            ConcreteMethodImplementation {
                name: "base_method".to_string(),
                parameters: vec![],
                return_type: Some(AstType::Normie),
                receiver_type: ReceiverType::Value,
                source_location: None,
            },
            ConcreteMethodImplementation {
                name: "derived_method".to_string(),
                parameters: vec![],
                return_type: Some(AstType::Normie),
                receiver_type: ReceiverType::Value,
                source_location: None,
            }
        ];
        
        assert!(checker.register_type_methods("TestType", implementations).is_ok());
        
        // Check compliance with base interface
        assert!(checker.check_interface_compliance("TestType", "BaseInterface").unwrap());
        
        // Check compliance with derived interface (should require both methods)
        assert!(checker.check_interface_compliance("TestType", "DerivedInterface").unwrap());
        
        // Test interface inheritance check
        assert!(checker.interface_extends("DerivedInterface", "BaseInterface"));
        assert!(!checker.interface_extends("BaseInterface", "DerivedInterface"));
        
        // Test getting all requirements for derived interface
        let all_requirements = checker.get_all_interface_requirements("DerivedInterface").unwrap();
        assert_eq!(all_requirements.len(), 2); // base_method + derived_method
        
        let method_names: Vec<&str> = all_requirements.iter().map(|r| r.name.as_str()).collect();
        assert!(method_names.contains(&"base_method"));
        assert!(method_names.contains(&"derived_method"));
    }
}
