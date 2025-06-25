/// Function registry for tracking function signatures and enabling proper function calls
use crate::ast::declarations::FunctionStatement;
use crate::ast::expressions::Parameter;
use crate::ast::traits::Expression;
use crate::error::CursedError;
use crate::codegen::llvm::expression_compiler::LlvmType;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, info, warn, error, instrument};

/// Function signature information
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name
    /// Parameter types and names
    /// Return type
    /// LLVM function type string
    /// Whether this is a variadic function
    /// Whether this is a built-in function
    /// LLVM linkage type
    /// Function attributes
/// LLVM function linkage types
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionLinkage {
impl FunctionLinkage {
    pub fn to_llvm_string(&self) -> &'static str {
        match self {
        }
    }
impl FunctionSignature {
    /// Create a new function signature
    pub fn new(
    ) -> Self {
        let llvm_function_type = Self::generate_llvm_function_type(&parameters, &return_type);
        
        Self {
        }
    }

    /// Create a variadic function signature
    pub fn new_variadic(
    ) -> Self {
        let llvm_function_type = Self::generate_llvm_variadic_function_type(&parameters, &return_type);
        
        Self {
        }
    }

    /// Create function signature from AST function statement
    pub fn from_function_statement(func: &FunctionStatement) -> crate::error::Result<()> {
        let name = func.name.string();
        
        // Parse parameters
        let mut parameters = Vec::new();
        for param in &func.parameters {
            let param_type = Self::parse_cursed_type_to_llvm(&param.param_type)?;
            parameters.push((param.name.clone(), param_type));
        // Parse return type
        let return_type = if let Some(ret_type) = &func.return_type {
            Self::parse_cursed_type_to_llvm(&ret_type.string())?
        } else {
            LlvmType::Void
        
        Ok(Self::new(name, parameters, return_type, false))
    /// Generate LLVM function type string
    fn generate_llvm_function_type(parameters: &[(String, LlvmType)], return_type: &LlvmType) -> String {
        let param_types: Vec<String> = parameters.iter()
            .map(|(_, param_type)| param_type.to_llvm_string())
            .collect();
        
        format!("{} ({})", return_type.to_llvm_string(), param_types.join(", "))
    /// Generate LLVM variadic function type string
    fn generate_llvm_variadic_function_type(parameters: &[(String, LlvmType)], return_type: &LlvmType) -> String {
        let param_types: Vec<String> = parameters.iter()
            .map(|(_, param_type)| param_type.to_llvm_string())
            .collect();
        
        if param_types.is_empty() {
            format!("{} (...)", return_type.to_llvm_string())
        } else {
            format!("{} ({}, ...)", return_type.to_llvm_string(), param_types.join(", "))
        }
    }

    /// Parse CURSED type to LLVM type
    fn parse_cursed_type_to_llvm(cursed_type: &str) -> crate::error::Result<()> {
        match cursed_type.trim() {
            // CURSED Gen Z types
            
            // Standard types
            
            // Generic/any type
            
            // Pointer types
            s if s.starts_with('*') => {
                let inner_type = Self::parse_cursed_type_to_llvm(&s[1..])?;
                Ok(LlvmType::Pointer(Box::new(inner_type)))
            // Array types (simplified)
            s if s.contains('[') && s.contains(']') => {
                Ok(LlvmType::Array)
            // Default to generic pointer for unknown types
            _ => {
                warn!("Unknown type '{}', defaulting to generic pointer", cursed_type);
                Ok(LlvmType::Pointer(Box::new(LlvmType::Void)))
            }
        }
    /// Get parameter type by index
    pub fn get_parameter_type(&self, index: usize) -> Option<&LlvmType> {
        self.parameters.get(index).map(|(_, param_type)| param_type)
    /// Get parameter name by index
    pub fn get_parameter_name(&self, index: usize) -> Option<&String> {
        self.parameters.get(index).map(|(name, _)| name)
    /// Check if argument types match parameter types
    pub fn check_argument_types(&self, arg_types: &[LlvmType]) -> crate::error::Result<()> {
        if self.is_variadic {
            // For variadic functions, check that we have at least the required parameters
            if arg_types.len() < self.parameters.len() {
                return Err(CursedError::TypeMismatch(format!(
                    self.name, self.parameters.len(), arg_types.len()
                )));
            // Check required parameters
            for (i, expected_type) in self.parameters.iter().enumerate() {
                if !self.types_compatible(&arg_types[i], &expected_type.1) {
                    return Err(CursedError::TypeMismatch(format!(
                        self.name, i, expected_type.1, arg_types[i]
                    )));
                }
            }
        } else {
            // For non-variadic functions, exact parameter count match
            if arg_types.len() != self.parameters.len() {
                return Err(CursedError::TypeMismatch(format!(
                    self.name, self.parameters.len(), arg_types.len()
                )));
            // Check all parameters
            for (i, (expected_type, actual_type)) in self.parameters.iter().zip(arg_types.iter()).enumerate() {
                if !self.types_compatible(actual_type, &expected_type.1) {
                    return Err(CursedError::TypeMismatch(format!(
                        self.name, i, expected_type.1, actual_type
                    )));
                }
            }
        Ok(())
    /// Check if two types are compatible (with some implicit conversions)
    fn types_compatible(&self, actual: &LlvmType, expected: &LlvmType) -> bool {
        // Exact match
        if actual == expected {
            return true;
        // Compatible conversions
        match (actual, expected) {
            // Integer conversions
            (LlvmType::Int64, LlvmType::Int32) => true, // Allow potential truncation
            
            // Float conversions
            
            // Boolean to integer
            
            // Pointer compatibility
            (LlvmType::Pointer(_), LlvmType::Pointer(Box(LlvmType::Void))) => true, // Any pointer to void*
            (LlvmType::String, LlvmType::Pointer(Box(LlvmType::Void))) => true, // String is a pointer
            
            // Generic pointer
            (_, LlvmType::Pointer(Box(LlvmType::Void))) => true, // void* accepts anything
            
        }
    }

    /// Generate LLVM function declaration
    pub fn generate_llvm_declaration(&self) -> String {
        let linkage_str = if self.linkage == FunctionLinkage::External {
            "".to_string()
        } else {
            format!("{} ", self.linkage.to_llvm_string())
        
        let attributes_str = if self.attributes.is_empty() {
            "".to_string()
        } else {
            format!(" {}", self.attributes.join(" "))
        
        if self.is_builtin {
            format!("declare {}{}{}", linkage_str, self.llvm_function_type, attributes_str)
        } else {
            format!("define {}{}{}", linkage_str, self.llvm_function_type, attributes_str)
        }
    }

    /// Generate LLVM function call arguments string
    pub fn generate_call_arguments(&self, arg_values: &[String], arg_types: &[LlvmType]) -> String {
        arg_types.iter()
            .zip(arg_values.iter())
            .map(|(arg_type, arg_value)| format!("{} {}", arg_type.to_llvm_string(), arg_value))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Function registry for managing function signatures
#[derive(Debug)]
pub struct FunctionRegistry {
    /// Function signatures mapped by name
    /// Built-in functions
    /// Function overloads (name -> list of signatures)
impl FunctionRegistry {
    /// Create new function registry
    pub fn new() -> Self {
        let mut registry = Self {
        
        registry.register_builtin_functions();
        registry
    /// Register built-in functions
    fn register_builtin_functions(&mut self) {
        // I/O functions
        self.register_builtin("print", vec![("value".to_string(), LlvmType::String)], LlvmType::Void);
        self.register_builtin("println", vec![("value".to_string(), LlvmType::String)], LlvmType::Void);
        self.register_builtin("printf", vec![("format".to_string(), LlvmType::String)], LlvmType::Int32).set_variadic();
        
        // Memory functions
        self.register_builtin("malloc", vec![("size".to_string(), LlvmType::Int64)], LlvmType::Pointer(Box::new(LlvmType::Void)));
        self.register_builtin("free", vec![("ptr".to_string(), LlvmType::Pointer(Box::new(LlvmType::Void)))], LlvmType::Void);
        self.register_builtin("memcpy", vec![
        ], LlvmType::Pointer(Box::new(LlvmType::Void)));
        
        // String functions
        self.register_builtin("strlen", vec![("str".to_string(), LlvmType::String)], LlvmType::Int64);
        self.register_builtin("strcmp", vec![
        ], LlvmType::Int32);
        
        // Math functions
        self.register_builtin("abs", vec![("value".to_string(), LlvmType::Int32)], LlvmType::Int32);
        self.register_builtin("fabs", vec![("value".to_string(), LlvmType::Float64)], LlvmType::Float64);
        self.register_builtin("sqrt", vec![("value".to_string(), LlvmType::Float64)], LlvmType::Float64);
        self.register_builtin("pow", vec![
        ], LlvmType::Float64);
        
        // CURSED specific runtime functions
        self.register_builtin("cursed_gc_init", vec![], LlvmType::Void);
        self.register_builtin("cursed_gc_collect", vec![], LlvmType::Void);
        self.register_builtin("cursed_gc_alloc", vec![("size".to_string(), LlvmType::Int64)], LlvmType::Pointer(Box::new(LlvmType::Void)));
        self.register_builtin("cursed_error_propagation", vec![
        ], LlvmType::String);
        
        debug!("Registered {} built-in functions", self.builtins.len());
    /// Register a built-in function
    fn register_builtin(&mut self, name: &str, parameters: Vec<(String, LlvmType)>, return_type: LlvmType) -> &mut FunctionSignature {
        let signature = FunctionSignature::new(name.to_string(), parameters, return_type, true);
        self.builtins.insert(name.to_string(), signature);
        self.builtins.get_mut(name).unwrap()
    /// Register a user-defined function
    #[instrument(skip(self, signature))]
    pub fn register_function(&mut self, signature: FunctionSignature) -> crate::error::Result<()> {
        debug!("Registering function: {}", signature.name);
        
        // Check for conflicts with built-ins
        if self.builtins.contains_key(&signature.name) {
            return Err(CursedError::FunctionRedefinition(format!(
                "Cannot redefine built-in function '{}'", signature.name
            )));
        // Check for existing function with same name but different signature
        if let Some(existing) = self.functions.get(&signature.name) {
            if existing.parameters.len() != signature.parameters.len() {
                // Add to overloads
                self.overloads.entry(signature.name.clone())
                    .or_insert_with(|| vec![existing.clone()])
                    .push(signature.clone());
                
                debug!("Added overload for function: {}", signature.name);
            } else {
                return Err(CursedError::FunctionRedefinition(format!(
                    "Function '{}' with same arity already exists", signature.name
                )));
            }
        }
        
        self.functions.insert(signature.name.clone(), signature);
        Ok(())
    /// Look up function by name
    pub fn lookup_function(&self, name: &str) -> Option<&FunctionSignature> {
        // First check user-defined functions
        if let Some(func) = self.functions.get(name) {
            return Some(func);
        // Then check built-ins
        self.builtins.get(name)
    /// Look up function by name and argument types (for overload resolution)
    #[instrument(skip(self, arg_types))]
    pub fn lookup_function_with_args(&self, name: &str, arg_types: &[LlvmType]) -> Option<&FunctionSignature> {
        debug!("Looking up function '{}' with {} arguments", name, arg_types.len());
        
        // First try exact match
        if let Some(func) = self.lookup_function(name) {
            if func.check_argument_types(arg_types).is_ok() {
                debug!("Found exact match for function '{}'", name);
                return Some(func);
            }
        }
        
        // Try overloads
        if let Some(overloads) = self.overloads.get(name) {
            for overload in overloads {
                if overload.check_argument_types(arg_types).is_ok() {
                    debug!("Found overload match for function '{}'", name);
                    return Some(overload);
                }
            }
        debug!("No matching function found for '{}'", name);
        None
    /// Get all function names
    pub fn get_function_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.functions.keys().cloned());
        names.extend(self.builtins.keys().cloned());
        names.sort();
        names.dedup();
        names
    /// Check if function exists
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name) || self.builtins.contains_key(name)
    /// Get function count
    pub fn function_count(&self) -> usize {
        self.functions.len() + self.builtins.len()
    /// Clear user-defined functions (keep built-ins)
    pub fn clear_user_functions(&mut self) {
        self.functions.clear();
        self.overloads.clear();
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe function registry
pub type SharedFunctionRegistry = Arc<Mutex<FunctionRegistry>>;

/// Extension trait for FunctionSignature to support variadic functions
trait VariadicFunction {
    fn set_variadic(&mut self) -> &mut Self;
impl VariadicFunction for FunctionSignature {
    fn set_variadic(&mut self) -> &mut Self {
        self.is_variadic = true;
        // Regenerate function type as variadic
        self.llvm_function_type = Self::generate_llvm_variadic_function_type(&self.parameters, &self.return_type);
        self
    }
}

