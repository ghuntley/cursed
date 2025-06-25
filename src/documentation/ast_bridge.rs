// AST Bridge for Documentation System
// 
// This module provides unified conversion methods and traits to bridge type gaps
// between different AST representations used throughout the documentation system.
// It resolves E0308 type mismatches by providing safe conversion methods.

use crate::ast::*;
use crate::error::{CursedError, SourceLocation};
use crate::documentation::extractors::ast_node_support;

use std::collections::HashMap;
use tracing::{debug, instrument, warn};

/// Trait for converting between AST node types
pub trait AstConverter {
    /// The target type this converter produces
    type Target;
    
    /// Convert from source to target type
    fn convert(&self) -> crate::error::Result<()>;
/// Trait for converting to documentation-specific AST nodes
pub trait ToDocumentationNode {
    /// Convert to documentation node
    fn to_doc_node(&self) -> crate::error::Result<()>;
/// Bridge between core AST and documentation AST
pub struct AstBridge {
    /// Cache for type conversions
impl AstBridge {
    /// Create a new AST bridge
    pub fn new() -> Self {
        Self {
        }
    }

    /// Convert AstNode to documentation-compatible format
    #[instrument(skip(self, node))]
    pub fn convert_ast_node(&mut self, node: &AstNode) -> crate::error::Result<()> {
        match &node.node_type {
            AstNodeType::Program(program) => {
                self.convert_program_node(program)
            }
            AstNodeType::Expression(expr) => {
                self.convert_expression_node(expr)
            }
            AstNodeType::Statement(stmt) => {
                self.convert_statement_to_expression(stmt)
            }
            AstNodeType::FunctionDeclaration(func) => {
                self.convert_function_declaration(func)
            }
            AstNodeType::StructDeclaration(struct_decl) => {
                self.convert_struct_declaration(struct_decl)
            }
            AstNodeType::InterfaceDeclaration(interface) => {
                self.convert_interface_declaration(interface)
            }
            AstNodeType::VariableDeclaration(var) => {
                self.convert_variable_declaration(var)
            }
            _ => {
                // Generic fallback for unknown node types
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
        }
    /// Convert Program to documentation expression
    fn convert_program_node(&self, program: &Program) -> crate::error::Result<()> {
        // For program nodes, create an identifier representing the module
        Ok(ast_node_support::Expression {
            expr_type: ast_node_support::ExpressionType::Identifier(
                ast_node_support::IdentifierExpression {
                }
        })
    /// Convert expression node to documentation expression
    fn convert_expression_node(&self, expr: &Box<dyn Expression>) -> crate::error::Result<()> {
        // Use string representation as fallback
        let expr_str = format!("{}", expr);
        
        // Try to determine expression type from string representation
        if expr_str.contains("(") && expr_str.contains(")") {
            // Likely a function call
            Ok(ast_node_support::Expression {
                expr_type: ast_node_support::ExpressionType::FunctionCall(
                    ast_node_support::FunctionCallExpression {
                        function: Box::new(ast_node_support::Expression {
                            expr_type: ast_node_support::ExpressionType::Identifier(
                                ast_node_support::IdentifierExpression {
                                }
                        arguments: Vec::new(), // Would need more sophisticated parsing
                    }
            })
        } else if expr_str.chars().all(|c| c.is_alphanumeric() || c == '_') {
            // Likely an identifier
            Ok(ast_node_support::Expression {
                expr_type: ast_node_support::ExpressionType::Identifier(
                    ast_node_support::IdentifierExpression {
                    }
            })
        } else {
            // Generic literal
            Ok(ast_node_support::Expression {
                expr_type: ast_node_support::ExpressionType::Literal(
                    ast_node_support::Literal::String(expr_str)
            })
        }
    }

    /// Convert statement to expression (for compatibility)
    fn convert_statement_to_expression(&self, stmt: &Box<dyn Statement>) -> crate::error::Result<()> {
        let stmt_str = format!("{}", stmt);
        
        // Create an identifier expression from the statement string
        Ok(ast_node_support::Expression {
            expr_type: ast_node_support::ExpressionType::Identifier(
                ast_node_support::IdentifierExpression {
                }
        })
    /// Convert function declaration to documentation format
    fn convert_function_declaration(&self, func: &FunctionDeclaration) -> crate::error::Result<()> {
        Ok(ast_node_support::Expression {
            expr_type: ast_node_support::ExpressionType::Identifier(
                ast_node_support::IdentifierExpression {
                }
        })
    /// Convert struct declaration to documentation format
    fn convert_struct_declaration(&self, struct_decl: &StructDeclaration) -> crate::error::Result<()> {
        Ok(ast_node_support::Expression {
            expr_type: ast_node_support::ExpressionType::Identifier(
                ast_node_support::IdentifierExpression {
                }
        })
    /// Convert interface declaration to documentation format
    fn convert_interface_declaration(&self, interface: &InterfaceDeclaration) -> crate::error::Result<()> {
        Ok(ast_node_support::Expression {
            expr_type: ast_node_support::ExpressionType::Identifier(
                ast_node_support::IdentifierExpression {
                }
        })
    /// Convert variable declaration to documentation format
    fn convert_variable_declaration(&self, var: &VariableDeclaration) -> crate::error::Result<()> {
        Ok(ast_node_support::Expression {
            expr_type: ast_node_support::ExpressionType::Identifier(
                ast_node_support::IdentifierExpression {
                }
        })
    /// Convert Program statements to documentation nodes
    #[instrument(skip(self, statements))]
    pub fn convert_statements(&mut self, statements: &[Box<dyn Statement>]) -> crate::error::Result<()> {
        let mut converted = Vec::new();
        
        for stmt in statements {
            let expr = self.convert_statement_to_expression(stmt)?;
            converted.push(expr);
        Ok(converted)
    /// Extract function declaration information from statements
    #[instrument(skip(self, statements))]
    pub fn extract_function_declarations(&self, statements: &[Box<dyn Statement>]) -> crate::error::Result<()> {
        let mut functions = Vec::new();
        
        for stmt in statements {
            if let Some(func_decl) = self.try_extract_function_declaration(stmt)? {
                functions.push(func_decl);
            }
        }
        
        Ok(functions)
    /// Try to extract function declaration from a statement
    fn try_extract_function_declaration(&self, stmt: &Box<dyn Statement>) -> crate::error::Result<()> {
        // Use string representation to extract function information
        let stmt_str = format!("{}", stmt);
        
        // Simple pattern matching - would need more sophisticated parsing in production
        if stmt_str.starts_with("slay ") || stmt_str.contains("function") {
            // Extract function name from statement string
            let parts: Vec<&str> = stmt_str.split_whitespace().collect();
            let name = if parts.len() > 1 {
                parts[1].trim_end_matches('(').to_string()
            } else {
                "unknown_function".to_string()
            
            Ok(Some(ast_node_support::FunctionDeclaration {
                parameters: Vec::new(), // Would need more sophisticated extraction
                is_public: true, // Default assumption
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract struct declarations from statements
    #[instrument(skip(self, statements))]
    pub fn extract_struct_declarations(&self, statements: &[Box<dyn Statement>]) -> crate::error::Result<()> {
        let mut structs = Vec::new();
        
        for stmt in statements {
            if let Some(struct_decl) = self.try_extract_struct_declaration(stmt)? {
                structs.push(struct_decl);
            }
        }
        
        Ok(structs)
    /// Try to extract struct declaration from a statement
    fn try_extract_struct_declaration(&self, stmt: &Box<dyn Statement>) -> crate::error::Result<()> {
        let stmt_str = format!("{}", stmt);
        
        if stmt_str.starts_with("squad ") || stmt_str.contains("struct") {
            let parts: Vec<&str> = stmt_str.split_whitespace().collect();
            let name = if parts.len() > 1 {
                parts[1].trim_end_matches('{').to_string()
            } else {
                "unknown_struct".to_string()
            
            Ok(Some(ast_node_support::StructDeclaration {
                fields: Vec::new(), // Would need more sophisticated extraction
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract interface declarations from statements
    #[instrument(skip(self, statements))]
    pub fn extract_interface_declarations(&self, statements: &[Box<dyn Statement>]) -> crate::error::Result<()> {
        let mut interfaces = Vec::new();
        
        for stmt in statements {
            if let Some(interface_decl) = self.try_extract_interface_declaration(stmt)? {
                interfaces.push(interface_decl);
            }
        }
        
        Ok(interfaces)
    /// Try to extract interface declaration from a statement
    fn try_extract_interface_declaration(&self, stmt: &Box<dyn Statement>) -> crate::error::Result<()> {
        let stmt_str = format!("{}", stmt);
        
        if stmt_str.starts_with("collab ") || stmt_str.contains("interface") {
            let parts: Vec<&str> = stmt_str.split_whitespace().collect();
            let name = if parts.len() > 1 {
                parts[1].trim_end_matches('{').to_string()
            } else {
                "unknown_interface".to_string()
            
            Ok(Some(ast_node_support::InterfaceDeclaration {
                methods: Vec::new(), // Would need more sophisticated extraction
            }))
        } else {
            Ok(None)
        }
    }

    /// Convert type information from expressions
    #[instrument(skip(self, expr))]
    pub fn extract_type_info(&self, expr: &Box<dyn Expression>) -> crate::error::Result<()> {
        // For now, use the expression's string representation as type info
        Ok(format!("{}", expr))
    /// Convert AST node types to compatible documentation types
    #[instrument(skip(self, node_type))]
    pub fn convert_ast_node_type(&self, node_type: &AstNodeType) -> crate::error::Result<()> {
        match node_type {
            AstNodeType::Program(program) => {
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
            AstNodeType::FunctionDeclaration(func) => {
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
            AstNodeType::StructDeclaration(struct_stmt) => {
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
            AstNodeType::InterfaceDeclaration(interface_stmt) => {
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
            AstNodeType::VariableDeclaration(var_stmt) => {
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
            _ => {
                // Generic fallback
                Ok(ast_node_support::Expression {
                    expr_type: ast_node_support::ExpressionType::Identifier(
                        ast_node_support::IdentifierExpression {
                        }
                })
            }
        }
    /// Create a safe wrapper for handling Vec<Box<dyn Statement>> to Option conversions
    #[instrument(skip(self, statements))]
    pub fn statements_to_optional_body(&self, statements: &[Box<dyn Statement>]) -> Option<AstNode> {
        if statements.is_empty() {
            None
        } else {
            // Create a block AST node containing all statements
            Some(AstNode::new_block(
                statements.iter()
                    .map(|stmt| AstNode::new_statement(stmt.clone()))
                    .collect()
            ))
        }
    }
impl Default for AstBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for converting core AST types to documentation types
pub trait ToDocumentationAst {
    /// Convert to documentation-compatible AST representation
    fn to_doc_ast(&self) -> crate::error::Result<()>;
impl ToDocumentationAst for AstNode {
    fn to_doc_ast(&self) -> crate::error::Result<()> {
        let mut bridge = AstBridge::new();
        bridge.convert_ast_node(self)
    }
}

impl ToDocumentationAst for Program {
    fn to_doc_ast(&self) -> crate::error::Result<()> {
        let bridge = AstBridge::new();
        bridge.convert_program_node(self)
    }
}

impl ToDocumentationAst for Box<dyn Statement> {
    fn to_doc_ast(&self) -> crate::error::Result<()> {
        let bridge = AstBridge::new();
        bridge.convert_statement_to_expression(self)
    }
}

impl ToDocumentationAst for Box<dyn Expression> {
    fn to_doc_ast(&self) -> crate::error::Result<()> {
        let bridge = AstBridge::new();
        bridge.convert_expression_node(self)
    }
}

/// Safe conversion utilities for common type mismatches
pub struct SafeConverter;

impl SafeConverter {
    /// Safely convert AstNode reference to Statement vector
    pub fn ast_node_to_statements(node: &AstNode) -> Vec<Box<dyn Statement>> {
        match &node.node_type {
            _ => {
                // Create a wrapper statement for non-statement nodes
                vec![Box::new(crate::ast::statements::ExpressionStatement::new(
                    Box::new(crate::ast::literals::StringLiteral::new("converted_node".to_string()))
                ))]
            }
        }
    /// Safely convert Statement vector to Program
    pub fn statements_to_program(statements: Vec<Box<dyn Statement>>) -> Program {
        Program {
        }
    }

    /// Safely convert Option to Vec
    pub fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
        match opt {
        }
    }

    /// Safely convert Vec to Option (takes first element)
    pub fn vec_to_option<T>(mut vec: Vec<T>) -> Option<T> {
        if vec.is_empty() {
            None
        } else {
            Some(vec.remove(0))
        }
    }
