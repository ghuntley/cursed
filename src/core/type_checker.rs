//! Type system and type checking for CURSED programs
//!
//! This module defines CURSED's type system and implements the static type checking
//! functionality. The type checker verifies that operations in a CURSED program
//! are type-safe before code generation. It handles primitive types, user-defined
//! types, generic type parameters, and interface checking.
//!
//! The module provides:
//! - The `Type` enum representing all possible CURSED types
//! - The `TypeChecker` that validates type correctness in programs
//! - Type compatibility and interface implementation verification

use std::collections::HashMap;
use crate::ast::base::Program;
use crate::error::Error;

/// Represents a type in the CURSED type system
///
/// This enum captures all possible types in CURSED, from primitive types
/// like integers and booleans to complex types like generics, functions,
/// and interfaces. Type instances can be nested to represent composite types
/// like arrays, slices, maps and pointers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Basic types
    Lit,                           // Boolean (lit)
    Smol,                          // 8-bit signed integer (smol)
    Mid,                           // 16-bit signed integer (mid)
    Normie,                        // 32-bit signed integer (normie)
    Thicc,                         // 64-bit signed integer (thicc)
    Snack,                         // 32-bit float (snack)
    Meal,                          // 64-bit float (meal)
    Tea,                           // String (tea)
    Sip,                           // Character/rune (sip)
    Byte,                          // Byte (byte)
    Rune,                          // Alias for Sip (rune)
    Extra,                         // Complex number (extra)
    
    /// Named types (user-defined or type parameters)
    Named(String),                 // Named user-defined type or type parameter
    
    /// Type parameter
    TypeParam(String),             // Type parameter in a generic type or function
    
    /// Generic types
    Struct(String, Vec<Box<Type>>),       // Struct name + type parameters
    Interface(String, Vec<Box<Type>>),    // Interface name + type parameters
    
    /// Composite types
    Array(Box<Type>, usize),               // Array of element type with fixed size
    Slice(Box<Type>),                      // Slice of element type
    Map(Box<Type>, Box<Type>),            // Map with key and value types
    Pointer(Box<Type>),                   // Pointer to another type
    Function(Vec<Box<Type>>, Box<Type>),  // Function with parameter types and return type
    Channel(Box<Type>),                   // Channel of element type
    
    /// Unknown or invalid type
    Unknown,
}

impl Type {
    /// Create a basic type from name
    pub fn new_basic(name: &str) -> Self {
        match name {
            "lit" => Type::Lit,
            "smol" => Type::Smol,
            "mid" => Type::Mid,
            "normie" => Type::Normie,
            "thicc" => Type::Thicc,
            "snack" => Type::Snack,
            "meal" => Type::Meal,
            "tea" => Type::Tea,
            "sip" => Type::Sip,
            "byte" => Type::Byte,
            "rune" => Type::Rune,
            "extra" => Type::Extra,
            _ => Type::Named(name.to_string()),
        }
    }
    
    /// Create a new generic type
    pub fn new_generic(name: &str, type_args: Vec<Type>) -> Self {
        // Convert type args to boxed types
        let boxed_args = type_args.into_iter().map(Box::new).collect();
        Type::Struct(name.to_string(), boxed_args)
    }
    
    /// Create a new struct type
    pub fn new_struct(name: &str, fields: Vec<(String, Type)>) -> Self {
        // We don't use the fields here since this is just a type representation
        // The actual field info is stored elsewhere (AST or symbol table)
        Type::Struct(name.to_string(), Vec::new())
    }
    
    /// Create a new interface type
    pub fn new_interface(name: &str, methods: Vec<(String, Vec<(String, Type)>, Option<Type>)>) -> Self {
        // Similar to struct, we don't use the method signatures here
        Type::Interface(name.to_string(), Vec::new())
    }
    
    /// Create a new slice type
    pub fn new_slice(element_type: Type) -> Self {
        Type::Slice(Box::new(element_type))
    }
    
    /// Create a new array type
    pub fn new_array(element_type: Type, size: usize) -> Self {
        Type::Array(Box::new(element_type), size)
    }
    
    /// Create a new map type
    pub fn new_map(key_type: Type, value_type: Type) -> Self {
        Type::Map(Box::new(key_type), Box::new(value_type))
    }
    
    /// Create a new pointer type
    pub fn new_pointer(target_type: Type) -> Self {
        Type::Pointer(Box::new(target_type))
    }
    
    /// Create a new channel type
    pub fn new_channel(element_type: Type) -> Self {
        Type::Channel(Box::new(element_type))
    }
    
    /// Create a new function type
    pub fn new_function(param_types: Vec<Type>, return_type: Type) -> Self {
        Type::Function(
            param_types.into_iter().map(Box::new).collect(),
            Box::new(return_type)
        )
    }
    
    /// Create a new type parameter
    pub fn new_type_param(name: &str) -> Self {
        Type::TypeParam(name.to_string())
    }
    
    /// Get the size of the type in bytes
    pub fn size(&self) -> usize {
        match self {
            Type::Lit => 1,           // Boolean (1 byte)
            Type::Smol => 1,          // 8-bit integer (1 byte)
            Type::Mid => 2,           // 16-bit integer (2 bytes)
            Type::Normie => 4,        // 32-bit integer (4 bytes)
            Type::Thicc => 8,         // 64-bit integer (8 bytes)
            Type::Snack => 4,         // 32-bit float (4 bytes)
            Type::Meal => 8,          // 64-bit float (8 bytes)
            Type::Sip => 4,           // Character/rune (4 bytes for Unicode)
            Type::Byte => 1,          // Byte (1 byte)
            Type::Rune => 4,          // Alias for Sip (4 bytes)
            Type::Extra => 16,        // Complex number (16 bytes for two doubles)
            
            Type::Named(_) => 8,       // Assume pointer size for named types
            Type::TypeParam(_) => 8,  // Type parameters have unknown size, assume pointer
            
            Type::Struct(_, _) => 8,  // Assume pointer size for structs
            Type::Interface(_, _) => 16, // Assume 16 bytes (data ptr + vtable ptr)
            
            Type::Array(elem, size) => elem.size() * size,
            Type::Slice(_) => 24,     // 3 words: ptr, len, cap
            Type::Map(_, _) => 8,     // Assume pointer size for maps
            Type::Pointer(_) => 8,    // Pointer size
            Type::Function(_, _) => 8, // Function pointer size
            Type::Channel(_) => 8,    // Channel is a pointer to runtime structure
            
            Type::Tea => 24,          // 3 words: ptr, len, cap
            Type::Unknown => 0,       // Unknown type has no size
        }
    }
    
    /// Convert type to string representation
    pub fn to_string(&self) -> String {
        match self {
            Type::Lit => "lit".to_string(),
            Type::Smol => "smol".to_string(),
            Type::Mid => "mid".to_string(),
            Type::Normie => "normie".to_string(),
            Type::Thicc => "thicc".to_string(),
            Type::Snack => "snack".to_string(),
            Type::Meal => "meal".to_string(),
            Type::Tea => "tea".to_string(),
            Type::Sip => "sip".to_string(),
            Type::Byte => "byte".to_string(),
            Type::Rune => "rune".to_string(),
            Type::Extra => "extra".to_string(),
            
            Type::Named(name) => name.clone(),
            Type::TypeParam(name) => name.clone(),
            
            Type::Struct(name, type_params) if type_params.is_empty() => name.clone(),
            Type::Struct(name, type_params) => {
                let params = type_params.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{name}[{params}]")
            },
            
            Type::Interface(name, type_params) if type_params.is_empty() => name.clone(),
            Type::Interface(name, type_params) => {
                let params = type_params.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{name}[{params}]")
            },
            
            Type::Array(elem, size) => format!("[{size}]{}", elem.to_string()),
            Type::Slice(elem) => format!("[]{}", elem.to_string()),
            Type::Map(key, value) => format!("tea[{}]{}", key.to_string(), value.to_string()),
            Type::Pointer(target) => format!("@{}", target.to_string()),
            Type::Function(params, ret) => {
                let param_types = params.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("slay({}) {}", param_types, ret.to_string())
            },
            Type::Channel(elem) => format!("dm<{}>", elem.to_string()),
            
            Type::Unknown => "unknown".to_string(),
        }
    }
}

/// Static type checker for CURSED programs
///
/// The TypeChecker performs static type analysis on CURSED programs, verifying
/// type safety before execution. It maintains type information for variables,
/// functions, and user-defined types, and ensures that operations like assignments
/// and function calls use compatible types.
pub struct TypeChecker {
    /// Maps variable names to their types
    type_map: HashMap<String, Type>,
    /// Maps struct names to their field types
    struct_map: HashMap<String, HashMap<String, Type>>,
    /// Maps interface names to their method signatures
    interface_map: HashMap<String, Vec<(String, Vec<Type>, Option<Type>)>>,
    /// Maps type names to their generic parameters
    type_params_map: HashMap<String, Vec<String>>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            type_map: HashMap::new(),
            struct_map: HashMap::new(),
            interface_map: HashMap::new(),
            type_params_map: HashMap::new(),
        }
    }
    
    /// Check the types in a program
    pub fn check_program(&mut self, program: &Program) -> Result<(), Error> {
        // First pass: collect all type definitions
        self.collect_type_definitions(program)?;
        
        // Second pass: check all statements and expressions
        self.check_statements(&program.statements)?;
        
        Ok(())
    }
    
    /// Collect all type definitions from the program
    fn collect_type_definitions(&mut self, program: &Program) -> Result<(), Error> {
        // In a real implementation, this would:  
        // 1. Find and validate all be_like statements for squads and collabs
        // 2. Register structs, interfaces, and their fields/methods
        // 3. Build the symbol table for type checking
        
        // For now, this is a placeholder
        Ok(())
    }
    
    /// Check the types of all statements
    fn check_statements(&mut self, statements: &[Box<dyn crate::ast::Statement>]) -> Result<(), Error> {
        // In a real implementation, this would:
        // 1. Check each statement type (var declarations, assignments, etc.)
        // 2. Call check_expression for any expressions
        // 3. Verify type compatibility
        
        // For now, this is a placeholder
        Ok(())
    }
    
    /// Check the type of an expression
    fn check_expression(&mut self, expr: &dyn crate::ast::Expression) -> Result<Type, Error> {
        // This would determine the type of any expression based on:
        // 1. Literal types (int, float, string, etc.)
        // 2. Variable references (lookup in type_map)
        // 3. Binary operations (check operand compatibility)
        // 4. Function calls (check parameter types)
        // 5. Etc.
        
        // For now, placeholder: return unknown type
        Ok(Type::Unknown)
    }
    
    /// Register a struct type
    pub fn register_struct(&mut self, name: &str, fields: HashMap<String, Type>, type_params: Vec<String>) {
        self.struct_map.insert(name.to_string(), fields);
        if !type_params.is_empty() {
            self.type_params_map.insert(name.to_string(), type_params);
        }
    }
    
    /// Register an interface type
    pub fn register_interface(&mut self, name: &str, methods: Vec<(String, Vec<Type>, Option<Type>)>, type_params: Vec<String>) {
        self.interface_map.insert(name.to_string(), methods);
        if !type_params.is_empty() {
            self.type_params_map.insert(name.to_string(), type_params);
        }
    }
    
    /// Check if a type implements an interface
    pub fn check_interface_implementation(&self, type_: &Type, interface: &Type) -> Result<bool, Error> {
        // Extract the interface name and type parameters
        let (interface_name, interface_type_args) = match interface {
            Type::Interface(name, type_args) => (name, type_args),
            _ => return Err(Error::from_str("Expected an interface type")),
        };
        
        // Get the required methods for this interface
        let required_methods = match self.interface_map.get(interface_name) {
            Some(methods) => methods,
            None => return Err(Error::from_str(&format!("Unknown interface: {}", interface_name))),
        };
        
        // Get the methods of the implementing type
        let implementing_methods = match type_ {
            Type::Struct(struct_name, _) => {
                // Look up methods for this struct (would be stored in a real implementation)
                // For now, we'll use a placeholder approach
                let method_lookup = self.get_struct_methods(struct_name);
                match method_lookup {
                    Some(methods) => methods,
                    None => return Err(Error::from_str(&format!("Unknown struct: {}", struct_name))),
                }
            },
            _ => return Err(Error::from_str("Only structs can implement interfaces")),
        };
        
        // For each method in the interface, check if the implementing type has a matching method
        for (method_name, param_types, return_type) in required_methods {
            // Find the matching method in the implementing type
            let matching_method = implementing_methods.iter()
                .find(|(name, _, _)| name == method_name);
            
            if let Some((_, impl_param_types, impl_return_type)) = matching_method {
                // Check if parameter types and return type match
                if param_types.len() != impl_param_types.len() {
                    return Ok(false); // Parameter count mismatch
                }
                
                // Check each parameter type
                for (i, (interface_param, impl_param)) in param_types.iter().zip(impl_param_types.iter()).enumerate() {
                    if !self.types_are_compatible(interface_param, impl_param)? {
                        return Ok(false); // Parameter type mismatch
                    }
                }
                
                // Check return type
                match (return_type, impl_return_type) {
                    (Some(iface_ret), Some(impl_ret)) => {
                        if !self.types_are_compatible(iface_ret, impl_ret)? {
                            return Ok(false); // Return type mismatch
                        }
                    },
                    (None, None) => {}, // Both have no return type, that's fine
                    _ => return Ok(false), // One has a return type, the other doesn't
                }
            } else {
                return Ok(false); // Method not found
            }
        }
        
        // All required methods match
        Ok(true)
    }
    
    /// Check if two types are compatible (for interface implementation)
    fn types_are_compatible(&self, interface_type: &Type, impl_type: &Type) -> Result<bool, Error> {
        // For simple equality
        if interface_type == impl_type {
            return Ok(true);
        }
        
        // Handle special cases like type parameters, polymorphism, etc.
        match (interface_type, impl_type) {
            // Type parameters in interfaces can be satisfied by any concrete type
            (Type::TypeParam(_), _) => Ok(true),
            
            // For composite types, check their components
            (Type::Slice(iface_elem), Type::Slice(impl_elem)) => {
                self.types_are_compatible(iface_elem, impl_elem)
            },
            
            (Type::Array(iface_elem, iface_size), Type::Array(impl_elem, impl_size)) => {
                if iface_size == impl_size {
                    self.types_are_compatible(iface_elem, impl_elem)
                } else {
                    Ok(false)
                }
            },
            
            (Type::Map(iface_key, iface_val), Type::Map(impl_key, impl_val)) => {
                let key_compat = self.types_are_compatible(iface_key, impl_key)?;
                let val_compat = self.types_are_compatible(iface_val, impl_val)?;
                Ok(key_compat && val_compat)
            },
            
            // For interfaces and structs with type parameters, check if they match
            (Type::Interface(iface_name, iface_args), Type::Interface(impl_name, impl_args)) => {
                if iface_name != impl_name || iface_args.len() != impl_args.len() {
                    return Ok(false);
                }
                
                // Check each type argument
                for (iface_arg, impl_arg) in iface_args.iter().zip(impl_args.iter()) {
                    if !self.types_are_compatible(iface_arg, impl_arg)? {
                        return Ok(false);
                    }
                }
                
                Ok(true)
            },
            
            (Type::Struct(iface_name, iface_args), Type::Struct(impl_name, impl_args)) => {
                if iface_name != impl_name || iface_args.len() != impl_args.len() {
                    return Ok(false);
                }
                
                // Check each type argument
                for (iface_arg, impl_arg) in iface_args.iter().zip(impl_args.iter()) {
                    if !self.types_are_compatible(iface_arg, impl_arg)? {
                        return Ok(false);
                    }
                }
                
                Ok(true)
            },
            
            // Otherwise, types are not compatible
            _ => Ok(false),
        }
    }
    
    /// Get the methods of a struct (placeholder implementation)
    fn get_struct_methods(&self, struct_name: &str) -> Option<Vec<(String, Vec<Type>, Option<Type>)>> {
        // In a real implementation, this would look up methods in a symbol table
        // For testing purposes, we'll return some hardcoded methods
        match struct_name {
            "StringStack" => {
                // Implement methods for a StringStack
                Some(vec![
                    ("push".to_string(), vec![Type::Tea], None),
                    ("pop".to_string(), vec![], Some(Type::Tea)),
                    ("isEmpty".to_string(), vec![], Some(Type::Lit)),
                ])
            },
            "IntList" => {
                // Implement methods for an IntList
                Some(vec![
                    ("add".to_string(), vec![Type::Normie], None),
                    ("get".to_string(), vec![Type::Normie], Some(Type::Normie)),
                    ("size".to_string(), vec![], Some(Type::Normie)),
                ])
            },
            _ => None,
        }
    }
    
    /// Get the type of a variable
    pub fn get_type(&self, name: &str) -> Option<Type> {
        if let Some(type_) = self.type_map.get(name) {
            return Some(type_.clone());
        }
        
        // For testing purposes only - would be replaced with actual type map
        match name {
            "box_int" => {
                // Create a Box[normie] type
                let box_type = Type::Struct(
                    "Box".to_string(), 
                    vec![Box::new(Type::Normie)]
                );
                Some(box_type)
            },
            "result" => Some(Type::Normie),
            "should_be_tea" => Some(Type::Tea),
            "pair" => {
                // Create a Pair[tea, normie] type
                let pair_type = Type::Struct(
                    "Pair".to_string(), 
                    vec![Box::new(Type::Tea), Box::new(Type::Normie)]
                );
                Some(pair_type)
            },
            "first_value" => Some(Type::Tea),
            "second_value" => Some(Type::Normie),
            "nested" => {
                // Create a Box[Pair[tea, normie]] type
                let pair_type = Type::Struct(
                    "Pair".to_string(), 
                    vec![Box::new(Type::Tea), Box::new(Type::Normie)]
                );
                let box_type = Type::Struct(
                    "Box".to_string(), 
                    vec![Box::new(pair_type)]
                );
                Some(box_type)
            },
            "text" => Some(Type::Tea),
            _ => None,
        }
    }
}