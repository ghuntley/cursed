//! Integration between type system and compilation pipeline
//! 
//! This module ensures type checking happens before LLVM compilation
//! and provides proper error reporting with source locations.

use crate::ast::Program;
use crate::error::CursedError;
use super::{TypeChecker, TypeCheckError, TypeSystem};
use crate::error::SourceLocation as ErrorSourceLocation;
use std::collections::HashMap;

/// Compilation pipeline that integrates type checking
#[derive(Debug)]
pub struct TypedCompilationPipeline {
    pub type_checker: TypeChecker,
    pub source_map: SourceLocationMap,
    pub compilation_context: CompilationContext,
}

/// Maps AST nodes to source locations for error reporting
#[derive(Debug, Clone)]
pub struct SourceLocationMap {
    pub locations: HashMap<String, SourceLocation>,
}

/// Source location information
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub span: (usize, usize), // Start and end byte offsets
}

/// Compilation context with type information
#[derive(Debug, Clone)]
pub struct CompilationContext {
    pub typed_program: Option<TypedProgram>,
    pub type_environment: HashMap<String, super::TypeExpression>,
    pub error_recovery_enabled: bool,
}

/// Program with complete type information
#[derive(Debug, Clone)]
pub struct TypedProgram {
    pub original_program: Program,
    pub type_annotations: HashMap<String, super::TypeExpression>,
    pub resolved_functions: HashMap<String, FunctionTypeInfo>,
    pub resolved_variables: HashMap<String, VariableTypeInfo>,
}

/// Function type information
#[derive(Debug, Clone)]
pub struct FunctionTypeInfo {
    pub name: String,
    pub parameter_types: Vec<super::TypeExpression>,
    pub return_type: super::TypeExpression,
    pub is_generic: bool,
    pub type_parameters: Vec<String>,
}

/// Variable type information
#[derive(Debug, Clone)]
pub struct VariableTypeInfo {
    pub name: String,
    pub type_expr: super::TypeExpression,
    pub is_mutable: bool,
    pub scope_level: usize,
}

impl TypedCompilationPipeline {
    pub fn new() -> Self {
        Self {
            type_checker: TypeChecker::new(),
            source_map: SourceLocationMap::new(),
            compilation_context: CompilationContext::new(),
        }
    }
    
    /// Main entry point for typed compilation
    pub fn compile_with_types(&mut self, program: &Program, source_file: &str) -> Result<TypedProgram, CompilationError> {
        // Phase 1: Type checking
        self.run_type_checking_phase(program, source_file)?;
        
        // Phase 2: Type resolution
        let typed_program = self.create_typed_program(program)?;
        
        // Phase 3: Pre-compilation validation
        self.validate_for_compilation(&typed_program)?;
        
        Ok(typed_program)
    }
    
    /// Run comprehensive type checking
    fn run_type_checking_phase(&mut self, program: &Program, source_file: &str) -> Result<(), CompilationError> {
        // Clear previous state
        self.type_checker.errors.clear();
        
        // Run type checking
        match self.type_checker.check_program(program) {
            Ok(()) => {
                println!("✓ Type checking completed successfully");
                Ok(())
            }
            Err(type_errors) => {
                // Convert type errors to compilation errors with source locations
                let compilation_errors = self.convert_type_errors_to_compilation_errors(type_errors, source_file);
                Err(CompilationError::TypeCheckingFailed(compilation_errors))
            }
        }
    }
    
    /// Create typed program with resolved type information
    fn create_typed_program(&mut self, program: &Program) -> Result<TypedProgram, CompilationError> {
        let mut type_annotations = HashMap::new();
        let mut resolved_functions = HashMap::new();
        let mut resolved_variables = HashMap::new();
        
        // Extract type information from type checker
        for scope in &self.type_checker.scopes {
            for (name, var_info) in scope {
                type_annotations.insert(name.clone(), var_info.symbol_type.clone());
                
                // Categorize as function or variable
                if self.is_function_type(&var_info.symbol_type) {
                    let func_info = self.extract_function_type_info(name, &var_info.symbol_type);
                    resolved_functions.insert(name.clone(), func_info);
                } else {
                    // Extract mutability from VariableInfo
                    let var_type_info = VariableTypeInfo {
                        name: name.clone(),
                        type_expr: var_info.symbol_type.clone(),
                        is_mutable: var_info.is_mutable,
                        scope_level: self.type_checker.scopes.len() - 1,
                    };
                    resolved_variables.insert(name.clone(), var_type_info);
                }
            }
        }
        
        Ok(TypedProgram {
            original_program: program.clone(),
            type_annotations,
            resolved_functions,
            resolved_variables,
        })
    }
    
    /// Validate that the typed program is ready for LLVM compilation
    fn validate_for_compilation(&self, typed_program: &TypedProgram) -> Result<(), CompilationError> {
        // Check that all functions have resolved return types
        for (name, func_info) in &typed_program.resolved_functions {
            if func_info.return_type.name.is_none() {
                return Err(CompilationError::UnresolvedType {
                    item_name: name.clone(),
                    context: "Function return type could not be resolved".to_string(),
                });
            }
        }
        
        // Check that all variables have concrete types
        for (name, var_info) in &typed_program.resolved_variables {
            if var_info.type_expr.name.is_none() {
                return Err(CompilationError::UnresolvedType {
                    item_name: name.clone(),
                    context: "Variable type could not be resolved".to_string(),
                });
            }
        }
        
        // Validate no unresolved type variables remain
        for (name, type_expr) in &typed_program.type_annotations {
            if self.has_unresolved_type_variables(type_expr) {
                return Err(CompilationError::UnresolvedType {
                    item_name: name.clone(),
                    context: "Contains unresolved type variables".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Convert type checking errors to compilation errors with source locations
    fn convert_type_errors_to_compilation_errors(&self, type_errors: Vec<TypeCheckError>, source_file: &str) -> Vec<CompilationErrorDetails> {
        type_errors.into_iter().map(|type_error| {
            let location = type_error.location.as_ref()
                .map(|loc| SourceLocation {
                    file: loc.file.clone().unwrap_or_else(|| source_file.to_string()),
                    line: loc.line,
                    column: loc.column,
                    span: (loc.offset, loc.offset),
                })
                .unwrap_or_else(|| SourceLocation {
                    file: source_file.to_string(),
                    line: 0,
                    column: 0,
                    span: (0, 0),
                });
            let suggestions = self.generate_error_suggestions(&type_error);
            CompilationErrorDetails {
                message: type_error.message,
                location,
                error_type: CompilationErrorType::TypeError,
                suggestions,
            }
        }).collect()
    }
    
    /// Generate helpful suggestions for type errors
    fn generate_error_suggestions(&self, type_error: &TypeCheckError) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match type_error.error_type {
            super::TypeErrorKind::UndefinedVariable => {
                suggestions.push("Check variable name spelling".to_string());
                suggestions.push("Ensure variable is declared before use".to_string());
            }
            super::TypeErrorKind::TypeMismatch => {
                suggestions.push("Check that operand types are compatible".to_string());
                suggestions.push("Consider adding explicit type annotations".to_string());
            }
            super::TypeErrorKind::ArityMismatch => {
                suggestions.push("Check function parameter count".to_string());
                suggestions.push("Verify function signature".to_string());
            }
            _ => {
                suggestions.push("Review type annotations and declarations".to_string());
            }
        }
        
        suggestions
    }
    
    /// Check if a type expression is a function type
    fn is_function_type(&self, type_expr: &super::TypeExpression) -> bool {
        type_expr.return_type.is_some() && !type_expr.parameters.is_empty()
    }
    
    /// Extract function type information
    fn extract_function_type_info(&self, name: &str, type_expr: &super::TypeExpression) -> FunctionTypeInfo {
        FunctionTypeInfo {
            name: name.to_string(),
            parameter_types: type_expr.parameters.clone(),
            return_type: type_expr.return_type.as_ref()
                .map(|rt| (**rt).clone())
                .unwrap_or_else(|| super::TypeExpression::named("void")),
            is_generic: self.has_type_parameters(type_expr),
            type_parameters: self.extract_type_parameters(type_expr),
        }
    }
    
    /// Check if type expression has type parameters
    fn has_type_parameters(&self, type_expr: &super::TypeExpression) -> bool {
        // Simple heuristic: check for type variable names (start with uppercase)
        if let Some(name) = &type_expr.name {
            if name.chars().next().map_or(false, |c| c.is_uppercase() && c.is_alphabetic()) {
                return true;
            }
        }
        
        type_expr.parameters.iter().any(|p| self.has_type_parameters(p))
    }
    
    /// Extract type parameter names
    fn extract_type_parameters(&self, type_expr: &super::TypeExpression) -> Vec<String> {
        let mut params = Vec::new();
        
        if let Some(name) = &type_expr.name {
            if name.chars().next().map_or(false, |c| c.is_uppercase() && c.is_alphabetic()) {
                params.push(name.clone());
            }
        }
        
        for param in &type_expr.parameters {
            params.extend(self.extract_type_parameters(param));
        }
        
        params.sort();
        params.dedup();
        params
    }
    
    /// Check for unresolved type variables
    fn has_unresolved_type_variables(&self, type_expr: &super::TypeExpression) -> bool {
        // Type variables typically start with 'T' followed by digits
        if let Some(name) = &type_expr.name {
            if name.starts_with('T') && name[1..].chars().all(|c| c.is_ascii_digit()) {
                return true;
            }
        }
        
        type_expr.parameters.iter().any(|p| self.has_unresolved_type_variables(p))
    }
    
    /// Get type information for compilation targets
    pub fn get_compilation_types(&self) -> HashMap<String, CompilationTypeInfo> {
        let mut compilation_types = HashMap::new();
        
        if let Some(typed_program) = &self.compilation_context.typed_program {
            for (name, type_expr) in &typed_program.type_annotations {
                let compilation_info = CompilationTypeInfo {
                    name: name.clone(),
                    llvm_type: self.map_to_llvm_type(type_expr),
                    size_bytes: self.calculate_type_size(type_expr),
                    alignment: self.calculate_type_alignment(type_expr),
                    is_reference: self.is_reference_type(type_expr),
                };
                compilation_types.insert(name.clone(), compilation_info);
            }
        }
        
        compilation_types
    }
    
    /// Map CURSED types to LLVM types
    fn map_to_llvm_type(&self, type_expr: &super::TypeExpression) -> LLVMTypeMapping {
        if let Some(name) = &type_expr.name {
            match name.as_str() {
                "int" => LLVMTypeMapping::Integer(64),
                "float" => LLVMTypeMapping::Float(64),
                "bool" => LLVMTypeMapping::Integer(1),
                "string" => LLVMTypeMapping::Pointer,
                "void" => LLVMTypeMapping::Void,
                _ => LLVMTypeMapping::Struct(name.clone()),
            }
        } else {
            LLVMTypeMapping::Unknown
        }
    }
    
    /// Calculate type size in bytes
    fn calculate_type_size(&self, type_expr: &super::TypeExpression) -> usize {
        if let Some(name) = &type_expr.name {
            match name.as_str() {
                "int" => 8,
                "float" => 8,
                "bool" => 1,
                "string" => 8, // Pointer size
                "void" => 0,
                _ => 8, // Default pointer size for complex types
            }
        } else {
            8
        }
    }
    
    /// Calculate type alignment
    fn calculate_type_alignment(&self, type_expr: &super::TypeExpression) -> usize {
        if let Some(name) = &type_expr.name {
            match name.as_str() {
                "int" => 8,
                "float" => 8,
                "bool" => 1,
                "string" => 8,
                "void" => 1,
                _ => 8,
            }
        } else {
            8
        }
    }
    
    /// Check if type is a reference type
    fn is_reference_type(&self, type_expr: &super::TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            matches!(name.as_str(), "string" | "Array" | "Map")
        } else {
            false
        }
    }
}

/// Compilation type information for LLVM
#[derive(Debug, Clone)]
pub struct CompilationTypeInfo {
    pub name: String,
    pub llvm_type: LLVMTypeMapping,
    pub size_bytes: usize,
    pub alignment: usize,
    pub is_reference: bool,
}

/// LLVM type mapping
#[derive(Debug, Clone)]
pub enum LLVMTypeMapping {
    Integer(u32),  // Bit width
    Float(u32),    // Bit width
    Pointer,
    Void,
    Struct(String),
    Unknown,
}

/// Compilation errors
#[derive(Debug, Clone)]
pub enum CompilationError {
    TypeCheckingFailed(Vec<CompilationErrorDetails>),
    UnresolvedType {
        item_name: String,
        context: String,
    },
    CodeGenerationFailed(String),
}

/// Detailed compilation error information
#[derive(Debug, Clone)]
pub struct CompilationErrorDetails {
    pub message: String,
    pub location: SourceLocation,
    pub error_type: CompilationErrorType,
    pub suggestions: Vec<String>,
}

/// Types of compilation errors
#[derive(Debug, Clone)]
pub enum CompilationErrorType {
    TypeError,
    UndefinedSymbol,
    InvalidOperation,
    CodeGeneration,
}

impl SourceLocationMap {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
        }
    }
    
    pub fn add_location(&mut self, node_id: String, location: SourceLocation) {
        self.locations.insert(node_id, location);
    }
    
    pub fn get_location(&self, node_id: &str) -> Option<SourceLocation> {
        self.locations.get(node_id).cloned()
    }
}

impl CompilationContext {
    pub fn new() -> Self {
        Self {
            typed_program: None,
            type_environment: HashMap::new(),
            error_recovery_enabled: true,
        }
    }
}

impl Default for TypedCompilationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    
    #[test]
    fn test_compilation_pipeline() {
        let mut pipeline = TypedCompilationPipeline::new();
        
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    target: crate::ast::LetTarget::Single("x".to_string()),
                    value: Expression::Integer(42),
                    var_type: None,
                    visibility: crate::ast::Visibility::Private,
                }),
            ],
            imports: Vec::new(),
            package: None,
        };
        
        let result = pipeline.compile_with_types(&program, "test.csd");
        assert!(result.is_ok());
        
        let typed_program = result.unwrap();
        assert!(!typed_program.type_annotations.is_empty());
    }
    
    #[test]
    fn test_error_reporting() {
        let mut pipeline = TypedCompilationPipeline::new();
        
        // Program with type error
        let program = Program {
            statements: vec![
                Statement::Expression(Expression::Binary(BinaryExpression {
                    left: Box::new(Expression::Integer(1)),
                    operator: "+".to_string(),
                    right: Box::new(Expression::String("hello".to_string())),
                })),
            ],
            imports: Vec::new(),
            package: None,
        };
        
        let result = pipeline.compile_with_types(&program, "test.csd");
        assert!(result.is_err());
        
        if let Err(CompilationError::TypeCheckingFailed(errors)) = result {
            assert!(!errors.is_empty());
            assert!(errors[0].message.contains("Type mismatch") || errors[0].message.contains("Arithmetic operation"));
        }
    }
    
    #[test]
    fn test_llvm_type_mapping() {
        let pipeline = TypedCompilationPipeline::new();
        
        let int_type = super::super::TypeExpression::named("int");
        let mapping = pipeline.map_to_llvm_type(&int_type);
        assert!(matches!(mapping, LLVMTypeMapping::Integer(64)));
        
        let string_type = super::super::TypeExpression::named("string");
        let mapping = pipeline.map_to_llvm_type(&string_type);
        assert!(matches!(mapping, LLVMTypeMapping::Pointer));
    }
}
