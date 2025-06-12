/// Enhanced Type Checker for CURSED Language
/// 
/// This module implements the main type checking interface that integrates
/// with the sophisticated type system infrastructure including constraint
/// resolution, type inference, and generic instantiation.

use crate::error::Error;
use crate::ast::traits::{Node, Expression, Statement};
use crate::ast::Program;
use crate::type_system::{
    TypeSystem, TypeExpression, TypeEnvironment, TypeDefinition, TypeKind,
    MethodSignature, InferenceContext
};
use std::collections::HashMap;

/// Compatibility layer between old Type enum and new TypeExpression system
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Basic types
    Lit,      // bool
    Normie,   // i32
    Thicc,    // i64
    Snack,    // f32
    Meal,     // f64
    Tea,      // string
    Smol,     // i8
    Mid,      // i16
    Sip,      // u32
    Cap,      // u64
    
    // Composite types
    Array(Box<Type>, usize),
    Slice(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Struct(String, Vec<Box<Type>>), // name, type parameters
    
    // Generic types
    TypeParam(String), // type parameter
    
    // Special types
    Nil,
    Unknown,
    Custom(String), // For backward compatibility
}

/// Enhanced type checker with full type system integration
pub struct TypeChecker {
    /// Core type system providing constraint resolution and inference
    type_system: TypeSystem,
    /// Cache for expression type checking results
    expression_cache: HashMap<String, TypeExpression>,
    /// Current type checking context
    current_context: Option<InferenceContext>,
}

impl TypeChecker {
    /// Create a new type checker with initialized type system
    pub fn new() -> Self {
        Self {
            type_system: TypeSystem::with_builtins(),
            expression_cache: HashMap::new(),
            current_context: None,
        }
    }

    /// Create a type checker with custom type system
    pub fn with_type_system(type_system: TypeSystem) -> Self {
        Self {
            type_system,
            expression_cache: HashMap::new(),
            current_context: None,
        }
    }

    /// Check type of an expression (legacy string interface)
    pub fn check_type(&self, expr: &str) -> Result<Type, Error> {
        // For backward compatibility, try to parse basic type names
        match expr {
            "facts" | "true" | "false" => Ok(Type::Lit),
            "normie" => Ok(Type::Normie),
            "thicc" => Ok(Type::Thicc),
            "snack" => Ok(Type::Snack),
            "meal" => Ok(Type::Meal),
            "tea" => Ok(Type::Tea),
            "smol" => Ok(Type::Smol),
            "mid" => Ok(Type::Mid),
            "sip" => Ok(Type::Sip),
            "cap" => Ok(Type::Cap),
            "nil" => Ok(Type::Nil),
            _ => {
                // Try to infer type using type system
                if let Some(type_def) = self.type_system.get_type_definition(expr) {
                    Ok(self.convert_type_definition_to_type(type_def))
                } else {
                    // Return unknown for compatibility but log the issue
                    tracing::warn!("Could not determine type for expression: {}", expr);
                    Ok(Type::Unknown)
                }
            }
        }
    }

    /// Check type of an AST expression (enhanced interface)
    pub fn check_expression_type(
        &mut self,
        expr: &dyn Expression,
    ) -> Result<TypeExpression, Error> {
        // Use inference context if available
        let default_context = InferenceContext::new();
        let context = self.current_context.as_ref().unwrap_or(&default_context);
        
        // Try cache first
        let expr_key = format!("{:?}", expr.string());
        if let Some(cached_type) = self.expression_cache.get(&expr_key) {
            return Ok(cached_type.clone());
        }

        // Infer type using type system
        let inferred_type = self.type_system.infer_expression_type(expr, context)?;
        
        // Cache result
        self.expression_cache.insert(expr_key, inferred_type.clone());
        
        Ok(inferred_type)
    }

    /// Check an entire program for type correctness
    pub fn check_program(&mut self, program: &Program) -> Result<(), Error> {
        tracing::info!("Starting type checking for program");
        
        // Create new inference context for the program
        let mut context = InferenceContext::new();
        self.current_context = Some(context.clone());

        // First pass: collect type definitions
        for statement in &program.statements {
            self.collect_type_definitions(statement)?;
        }

        // Second pass: type check all statements
        for statement in &program.statements {
            self.check_statement_types(statement, &mut context)?;
        }

        tracing::info!("Type checking completed successfully");
        Ok(())
    }

    /// Collect type definitions from a statement
    fn collect_type_definitions(&mut self, statement: &Box<dyn Statement>) -> Result<(), Error> {
        // This would analyze struct/interface declarations and register them
        // For now, we'll implement basic structure
        tracing::debug!("Collecting type definitions from statement: {}", statement.string());
        
        // Check if this is a type definition statement
        let statement_str = statement.string();
        if statement_str.contains("squad ") {
            // This is a struct definition - parse and register
            self.parse_and_register_struct_definition(&statement_str)?;
        } else if statement_str.contains("collab ") {
            // This is an interface definition - parse and register
            self.parse_and_register_interface_definition(&statement_str)?;
        }

        Ok(())
    }

    /// Parse and register a struct definition
    fn parse_and_register_struct_definition(&mut self, definition: &str) -> Result<(), Error> {
        // Basic parsing - in a real implementation this would use the AST
        let parts: Vec<&str> = definition.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "squad" {
            let struct_name = parts[1].to_string();
            
            let type_def = TypeDefinition {
                name: struct_name.clone(),
                kind: TypeKind::Struct,
                type_parameters: Vec::new(), // TODO: Parse generic parameters
                constraints: Vec::new(),
                methods: Vec::new(),
                is_builtin: false,
            };

            self.type_system.register_type(type_def)?;
            tracing::debug!("Registered struct type: {}", struct_name);
        }

        Ok(())
    }

    /// Parse and register an interface definition
    fn parse_and_register_interface_definition(&mut self, definition: &str) -> Result<(), Error> {
        // Basic parsing - in a real implementation this would use the AST
        let parts: Vec<&str> = definition.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "collab" {
            let interface_name = parts[1].to_string();
            
            let type_def = TypeDefinition {
                name: interface_name.clone(),
                kind: TypeKind::Interface,
                type_parameters: Vec::new(), // TODO: Parse generic parameters
                constraints: Vec::new(),
                methods: Vec::new(), // TODO: Parse method signatures
                is_builtin: false,
            };

            self.type_system.register_type(type_def)?;
            tracing::debug!("Registered interface type: {}", interface_name);
        }

        Ok(())
    }

    /// Check types within a statement
    fn check_statement_types(
        &mut self,
        statement: &Box<dyn Statement>,
        context: &mut InferenceContext,
    ) -> Result<(), Error> {
        tracing::debug!("Type checking statement: {}", statement.string());
        
        // This would recursively check all expressions within the statement
        // For now, we'll implement basic structure
        let statement_str = statement.string();
        
        // Check for variable declarations
        if statement_str.contains("sus ") || statement_str.contains("facts ") {
            self.check_variable_declaration(&statement_str, context)?;
        }
        
        // Check for function calls or other expressions
        // This would be expanded to handle all statement types
        
        Ok(())
    }

    /// Check a variable declaration
    fn check_variable_declaration(&mut self, declaration: &str, _context: &InferenceContext) -> Result<(), Error> {
        tracing::debug!("Checking variable declaration: {}", declaration);
        
        // Basic parsing - extract variable name and type/value
        let parts: Vec<&str> = declaration.split_whitespace().collect();
        if parts.len() >= 3 {
            let var_name = parts[1];
            
            // Try to infer type from assignment or explicit type annotation
            if declaration.contains(" = ") {
                // Has assignment - infer from value
                let assignment_parts: Vec<&str> = declaration.split(" = ").collect();
                if assignment_parts.len() == 2 {
                    let value = assignment_parts[1].trim_end_matches(';');
                    let inferred_type = self.infer_type_from_literal(value)?;
                    tracing::debug!("Inferred type for {}: {:?}", var_name, inferred_type);
                }
            }
        }

        Ok(())
    }

    /// Infer type from a literal value
    fn infer_type_from_literal(&self, literal: &str) -> Result<TypeExpression, Error> {
        match literal {
            "true" | "false" => Ok(TypeExpression::named("facts")),
            s if s.parse::<i32>().is_ok() => Ok(TypeExpression::named("normie")),
            s if s.parse::<i64>().is_ok() => Ok(TypeExpression::named("thicc")),
            s if s.parse::<f32>().is_ok() => Ok(TypeExpression::named("snack")),
            s if s.parse::<f64>().is_ok() => Ok(TypeExpression::named("meal")),
            s if s.starts_with('"') && s.ends_with('"') => Ok(TypeExpression::named("tea")),
            "nil" => Ok(TypeExpression::named("nil")),
            _ => {
                // Try to resolve as a known type
                if self.type_system.get_type_definition(literal).is_some() {
                    Ok(TypeExpression::named(literal))
                } else {
                    Err(Error::Type(format!("Cannot infer type for literal: {}", literal)))
                }
            }
        }
    }

    /// Convert TypeDefinition to legacy Type enum for compatibility
    fn convert_type_definition_to_type(&self, type_def: &TypeDefinition) -> Type {
        match type_def.name.as_str() {
            "facts" => Type::Lit,
            "normie" => Type::Normie,
            "thicc" => Type::Thicc,
            "snack" => Type::Snack,
            "meal" => Type::Meal,
            "tea" => Type::Tea,
            "smol" => Type::Smol,
            "mid" => Type::Mid,
            "sip" => Type::Sip,
            "cap" => Type::Cap,
            "nil" => Type::Nil,
            _ => Type::Custom(type_def.name.clone()),
        }
    }

    /// Convert legacy Type to TypeExpression
    pub fn convert_type_to_expression(&self, legacy_type: &Type) -> TypeExpression {
        match legacy_type {
            Type::Lit => TypeExpression::named("facts"),
            Type::Normie => TypeExpression::named("normie"),
            Type::Thicc => TypeExpression::named("thicc"),
            Type::Snack => TypeExpression::named("snack"),
            Type::Meal => TypeExpression::named("meal"),
            Type::Tea => TypeExpression::named("tea"),
            Type::Smol => TypeExpression::named("smol"),
            Type::Mid => TypeExpression::named("mid"),
            Type::Sip => TypeExpression::named("sip"),
            Type::Cap => TypeExpression::named("cap"),
            Type::Nil => TypeExpression::named("nil"),
            Type::Array(elem_type, _size) => {
                let elem_expr = self.convert_type_to_expression(elem_type);
                TypeExpression::array(elem_expr)
            }
            Type::Map(key_type, value_type) => {
                let key_expr = self.convert_type_to_expression(key_type);
                let value_expr = self.convert_type_to_expression(value_type);
                TypeExpression::map(key_expr, value_expr)
            }
            Type::TypeParam(name) => TypeExpression::parameter(name),
            Type::Custom(name) => TypeExpression::named(name),
            Type::Unknown => TypeExpression::named("unknown"),
            _ => TypeExpression::named("unknown"),
        }
    }

    /// Get access to the underlying type system
    pub fn type_system(&self) -> &TypeSystem {
        &self.type_system
    }

    /// Get mutable access to the underlying type system
    pub fn type_system_mut(&mut self) -> &mut TypeSystem {
        &mut self.type_system
    }

    /// Clear the expression cache
    pub fn clear_cache(&mut self) {
        self.expression_cache.clear();
    }

    /// Set the current inference context
    pub fn set_context(&mut self, context: InferenceContext) {
        self.current_context = Some(context);
    }

    /// Get the current inference context
    pub fn get_context(&self) -> Option<&InferenceContext> {
        self.current_context.as_ref()
    }

    /// Register a custom type
    pub fn register_type(&mut self, type_def: TypeDefinition) -> Result<(), Error> {
        self.type_system.register_type(type_def)
    }

    /// Check if constraints are satisfied for a type
    pub fn check_constraints(
        &self,
        type_expr: &TypeExpression,
        constraints: &[crate::ast::declarations::GenericConstraint],
    ) -> Result<bool, Error> {
        self.type_system.check_constraints(type_expr, constraints)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
