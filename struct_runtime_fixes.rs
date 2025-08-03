// CURSED Struct Runtime Implementation Fixes
// This file documents the fixes needed for complete struct support

use crate::error::CursedError;
use crate::execution::ExecutionContext;
use crate::ast::{Expression, MemberAccessExpression, StructStatement, StructField};
use std::collections::HashMap;

/// Enhanced CursedValue with proper struct support
#[derive(Debug, Clone)]
pub enum CursedValue {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<CursedValue>),
    Struct(HashMap<String, CursedValue>),  // Already exists
    // ... other variants
}

/// Enhanced struct runtime support
pub struct StructRuntime {
    /// Registered struct types and their field definitions
    struct_types: HashMap<String, StructDefinition>,
    /// Memory allocator for struct instances
    allocator: StructAllocator,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub methods: Vec<MethodDefinition>,
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: String,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct MethodDefinition {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
}

/// Memory-safe struct allocator
pub struct StructAllocator {
    /// Track allocated struct instances for GC
    allocated_structs: HashMap<usize, (String, HashMap<String, CursedValue>)>,
    next_id: usize,
}

impl StructRuntime {
    pub fn new() -> Self {
        Self {
            struct_types: HashMap::new(),
            allocator: StructAllocator {
                allocated_structs: HashMap::new(),
                next_id: 0,
            },
        }
    }

    /// Register a struct type definition
    pub fn register_struct_type(&mut self, struct_stmt: &StructStatement) -> Result<(), CursedError> {
        let mut fields = Vec::new();
        let mut offset = 0;

        for field in &struct_stmt.fields {
            let field_size = self.calculate_field_size(&field.field_type)?;
            fields.push(FieldDefinition {
                name: field.name.clone(),
                field_type: format!("{:?}", field.field_type), // Convert Type to string
                offset,
                size: field_size,
            });
            offset += field_size;
        }

        let definition = StructDefinition {
            name: struct_stmt.name.clone(),
            fields,
            methods: Vec::new(), // Methods will be added separately
        };

        self.struct_types.insert(struct_stmt.name.clone(), definition);
        Ok(())
    }

    /// Create a new struct instance
    pub fn create_struct_instance(
        &mut self, 
        struct_name: &str, 
        field_values: HashMap<String, CursedValue>
    ) -> Result<CursedValue, CursedError> {
        let struct_def = self.struct_types.get(struct_name)
            .ok_or_else(|| CursedError::RuntimeError(format!("Unknown struct type: {}", struct_name)))?;

        // Validate field names and types
        for field_def in &struct_def.fields {
            if !field_values.contains_key(&field_def.name) {
                return Err(CursedError::RuntimeError(
                    format!("Missing field '{}' in struct '{}'", field_def.name, struct_name)
                ));
            }
        }

        // Check for extra fields
        for field_name in field_values.keys() {
            if !struct_def.fields.iter().any(|f| f.name == *field_name) {
                return Err(CursedError::RuntimeError(
                    format!("Unknown field '{}' in struct '{}'", field_name, struct_name)
                ));
            }
        }

        // Allocate struct instance
        let instance_id = self.allocator.next_id;
        self.allocator.next_id += 1;
        self.allocator.allocated_structs.insert(instance_id, (struct_name.to_string(), field_values.clone()));

        Ok(CursedValue::Struct(field_values))
    }

    /// Access a field from a struct
    pub fn access_struct_field(
        &self,
        struct_value: &CursedValue,
        field_name: &str
    ) -> Result<CursedValue, CursedError> {
        match struct_value {
            CursedValue::Struct(fields) => {
                fields.get(field_name)
                    .cloned()
                    .ok_or_else(|| CursedError::RuntimeError(
                        format!("Field '{}' not found in struct", field_name)
                    ))
            }
            _ => Err(CursedError::RuntimeError(
                "Attempted to access field on non-struct value".to_string()
            ))
        }
    }

    /// Modify a field in a struct (for mutable structs)
    pub fn modify_struct_field(
        &mut self,
        struct_value: &mut CursedValue,
        field_name: &str,
        new_value: CursedValue
    ) -> Result<(), CursedError> {
        match struct_value {
            CursedValue::Struct(fields) => {
                if fields.contains_key(field_name) {
                    fields.insert(field_name.to_string(), new_value);
                    Ok(())
                } else {
                    Err(CursedError::RuntimeError(
                        format!("Field '{}' not found in struct", field_name)
                    ))
                }
            }
            _ => Err(CursedError::RuntimeError(
                "Attempted to modify field on non-struct value".to_string()
            ))
        }
    }

    /// Calculate field size for memory layout
    fn calculate_field_size(&self, field_type: &Option<crate::ast::Type>) -> Result<usize, CursedError> {
        match field_type {
            Some(crate::ast::Type::Integer) => Ok(4),
            Some(crate::ast::Type::Float) => Ok(8),
            Some(crate::ast::Type::String) => Ok(8), // Pointer size
            Some(crate::ast::Type::Boolean) => Ok(1),
            Some(_) => Ok(8), // Default pointer size for complex types
            None => Ok(8), // Unknown type, assume pointer
        }
    }

    /// Cleanup allocated structs (for GC integration)
    pub fn cleanup_structs(&mut self) {
        self.allocator.allocated_structs.clear();
    }
}

/// Enhanced member access evaluation that handles structs
pub fn enhanced_evaluate_member_access(
    struct_runtime: &StructRuntime,
    member_expr: &MemberAccessExpression, 
    context: &mut ExecutionContext
) -> Result<CursedValue, CursedError> {
    // Evaluate the object being accessed
    let object_value = evaluate_expression_with_context(&member_expr.object, context)?;
    
    // Access the field
    struct_runtime.access_struct_field(&object_value, &member_expr.property)
}

/// Helper function to evaluate expressions (placeholder)
fn evaluate_expression_with_context(
    expr: &Expression, 
    _context: &mut ExecutionContext
) -> Result<CursedValue, CursedError> {
    // This would be the actual expression evaluation
    // For now, return a placeholder
    Ok(CursedValue::String("placeholder".to_string()))
}

/// Enhanced struct literal parsing that handles proper field assignments
pub fn enhanced_parse_struct_literal(
    lexer: &mut crate::lexer::Lexer,
    struct_name: String
) -> Result<Expression, CursedError> {
    // Expect '{'
    expect_token(lexer, crate::lexer::TokenKind::LeftBrace)?;
    
    let mut fields = Vec::new();
    
    while !matches!(lexer.peek()?.kind, crate::lexer::TokenKind::RightBrace) {
        // Parse field assignment: field_name: value
        let field_name = expect_identifier(lexer)?;
        expect_token(lexer, crate::lexer::TokenKind::Colon)?;
        let field_value = parse_expression(lexer)?;
        
        fields.push(crate::ast::StructFieldAssignment {
            field_name,
            value: field_value,
        });
        
        // Optional comma
        if matches!(lexer.peek()?.kind, crate::lexer::TokenKind::Comma) {
            lexer.advance()?;
        }
    }
    
    expect_token(lexer, crate::lexer::TokenKind::RightBrace)?;
    
    Ok(Expression::StructLiteral(crate::ast::StructLiteralExpression {
        struct_name,
        fields,
    }))
}

// Helper functions (placeholders for actual parser functions)
fn expect_token(lexer: &mut crate::lexer::Lexer, expected: crate::lexer::TokenKind) -> Result<(), CursedError> {
    // Implementation would check and consume expected token
    Ok(())
}

fn expect_identifier(lexer: &mut crate::lexer::Lexer) -> Result<String, CursedError> {
    // Implementation would parse and return identifier
    Ok("identifier".to_string())
}

fn parse_expression(lexer: &mut crate::lexer::Lexer) -> Result<Expression, CursedError> {
    // Implementation would parse full expression
    Ok(Expression::Integer(42))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_runtime_registration() {
        let mut runtime = StructRuntime::new();
        
        let struct_stmt = StructStatement {
            name: "Person".to_string(),
            fields: vec![
                StructField {
                    name: "name".to_string(),
                    field_type: Some(crate::ast::Type::String),
                    visibility: crate::ast::Visibility::Public,
                },
                StructField {
                    name: "age".to_string(),
                    field_type: Some(crate::ast::Type::Integer),
                    visibility: crate::ast::Visibility::Public,
                },
            ],
            visibility: crate::ast::Visibility::Public,
        };

        assert!(runtime.register_struct_type(&struct_stmt).is_ok());
        assert!(runtime.struct_types.contains_key("Person"));
    }

    #[test]
    fn test_struct_instance_creation() {
        let mut runtime = StructRuntime::new();
        
        // Register struct type first
        let struct_stmt = StructStatement {
            name: "Point".to_string(),
            fields: vec![
                StructField {
                    name: "x".to_string(),
                    field_type: Some(crate::ast::Type::Integer),
                    visibility: crate::ast::Visibility::Public,
                },
                StructField {
                    name: "y".to_string(),
                    field_type: Some(crate::ast::Type::Integer),
                    visibility: crate::ast::Visibility::Public,
                },
            ],
            visibility: crate::ast::Visibility::Public,
        };
        
        runtime.register_struct_type(&struct_stmt).unwrap();
        
        // Create instance
        let mut field_values = HashMap::new();
        field_values.insert("x".to_string(), CursedValue::Integer(10));
        field_values.insert("y".to_string(), CursedValue::Integer(20));
        
        let instance = runtime.create_struct_instance("Point", field_values).unwrap();
        
        // Test field access
        let x_value = runtime.access_struct_field(&instance, "x").unwrap();
        match x_value {
            CursedValue::Integer(val) => assert_eq!(val, 10),
            _ => panic!("Expected integer value"),
        }
    }

    #[test]
    fn test_struct_field_modification() {
        let mut runtime = StructRuntime::new();
        
        let mut field_values = HashMap::new();
        field_values.insert("x".to_string(), CursedValue::Integer(5));
        field_values.insert("y".to_string(), CursedValue::Integer(10));
        
        let mut struct_value = CursedValue::Struct(field_values);
        
        // Modify field
        runtime.modify_struct_field(&mut struct_value, "x", CursedValue::Integer(15)).unwrap();
        
        // Verify modification
        let x_value = runtime.access_struct_field(&struct_value, "x").unwrap();
        match x_value {
            CursedValue::Integer(val) => assert_eq!(val, 15),
            _ => panic!("Expected modified integer value"),
        }
    }
}

/// Integration point for existing CURSED execution engine
pub fn integrate_struct_runtime_with_execution_engine(
    engine: &mut crate::execution::CursedExecutionEngine,
    struct_runtime: StructRuntime
) -> Result<(), CursedError> {
    // This would integrate the struct runtime with the existing execution engine
    // The implementation would involve:
    // 1. Adding struct_runtime to the execution context
    // 2. Modifying evaluate_struct_literal to use the new runtime
    // 3. Updating member access evaluation
    // 4. Integrating with the memory management system
    
    Ok(())
}
