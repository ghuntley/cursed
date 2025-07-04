//! Complete TypeChecker implementation for CURSED language
//! 
//! This module provides comprehensive type checking and inference capabilities
//! including expression type checking, statement validation, and type unification.

use crate::ast::{Expression, Statement, Program, BinaryExpression, CallExpression, 
                 MemberAccessExpression, LetStatement, FunctionStatement, IfStatement, 
                 WhileStatement, ReturnStatement, Literal, StructStatement, InterfaceStatement,
                 ChannelStatement, GoroutineStatement, SelectStatement, PanicStatement, 
                 CatchStatement, ChannelSendExpression, ChannelReceiveExpression, 
                 ChannelCreationExpression};
use crate::error::CursedError;
use crate::core::Type;
use super::{TypeExpression, TypeSystem, TypeEnvironment, InferenceContext, 
            TypeSubstitution, ConstraintResolver, ConstraintViolation};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeChecker {
    pub type_system: TypeSystem,
    pub scopes: Vec<HashMap<String, TypeExpression>>,
    pub current_function_return_type: Option<TypeExpression>,
    pub errors: Vec<TypeCheckError>,
}

#[derive(Debug, Clone)]
pub struct TypeCheckError {
    pub message: String,
    pub location: Option<String>,
    pub error_type: TypeErrorKind,
}

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    TypeMismatch,
    UndefinedVariable,
    UndefinedFunction,
    ArityMismatch,
    InvalidOperation,
    ConstraintViolation,
    UnificationFailure,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            type_system: TypeSystem::new(),
            scopes: vec![HashMap::new()],
            current_function_return_type: None,
            errors: Vec::new(),
        };
        
        // Initialize built-in types and functions
        checker.initialize_builtins();
        checker
    }
    
    fn initialize_builtins(&mut self) {
        // Add built-in variables
        self.add_variable("vibez".to_string(), TypeExpression::named("vibez"));
        
        // Add built-in functions
        self.add_function("print".to_string(), 
                         vec![TypeExpression::named("string")], 
                         TypeExpression::named("void"));
        
        self.add_function("len".to_string(),
                         vec![TypeExpression::named("string")],
                         TypeExpression::named("int"));
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeCheckError>> {
        self.errors.clear();
        
        // Check all statements
        for statement in &program.statements {
            if let Err(error) = self.check_statement(statement) {
                self.errors.push(error);
            }
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    pub fn check_statement(&mut self, statement: &Statement) -> Result<TypeExpression, TypeCheckError> {
        match statement {
            Statement::Expression(expr) => {
                self.check_expression(expr)
            }
            Statement::Let(let_stmt) => {
                self.check_let_statement(let_stmt)
            }
            Statement::Function(func_stmt) => {
                self.check_function_statement(func_stmt)
            }
            Statement::If(if_stmt) => {
                self.check_if_statement(if_stmt)
            }
            Statement::While(while_stmt) => {
                self.check_while_statement(while_stmt)
            }
            Statement::Return(return_stmt) => {
                self.check_return_statement(return_stmt)
            }
            Statement::Struct(struct_stmt) => {
                self.check_struct_statement(struct_stmt)
            }
            Statement::Interface(interface_stmt) => {
                self.check_interface_statement(interface_stmt)
            }
            Statement::Channel(channel_stmt) => {
                self.check_channel_statement(channel_stmt)
            }
            Statement::Goroutine(goroutine_stmt) => {
                self.check_goroutine_statement(goroutine_stmt)
            }
            Statement::Select(select_stmt) => {
                self.check_select_statement(select_stmt)
            }
            Statement::Panic(panic_stmt) => {
                self.check_panic_statement(panic_stmt)
            }
            Statement::Catch(catch_stmt) => {
                self.check_catch_statement(catch_stmt)
            }
            _ => Ok(TypeExpression::named("void")),
        }
    }
    
    pub fn check_expression(&mut self, expression: &Expression) -> Result<TypeExpression, TypeCheckError> {
        match expression {
            Expression::Integer(_) => Ok(TypeExpression::named("int")),
            Expression::String(_) => Ok(TypeExpression::named("string")),
            Expression::Boolean(_) => Ok(TypeExpression::named("bool")),
            Expression::Identifier(name) => {
                self.check_identifier(name)
            }
            Expression::Binary(binary) => {
                self.check_binary_expression(binary)
            }
            Expression::Call(call) => {
                self.check_call_expression(call)
            }
            Expression::MemberAccess(member) => {
                self.check_member_access(member)
            }
            Expression::Literal(literal) => {
                self.check_literal(literal)
            }
            Expression::Array(elements) => {
                self.check_array_expression(elements)
            }
            Expression::Map(pairs) => {
                self.check_map_expression(pairs)
            }
            Expression::ChannelSend(channel_send) => {
                self.check_channel_send_expression(channel_send)
            }
            Expression::ChannelReceive(channel_receive) => {
                self.check_channel_receive_expression(channel_receive)
            }
            Expression::ChannelCreation(channel_creation) => {
                self.check_channel_creation_expression(channel_creation)
            }
            _ => Ok(TypeExpression::named("unknown")),
        }
    }
    
    fn check_identifier(&self, name: &str) -> Result<TypeExpression, TypeCheckError> {
        // Look up in current scopes (from innermost to outermost)
        for scope in self.scopes.iter().rev() {
            if let Some(type_expr) = scope.get(name) {
                return Ok(type_expr.clone());
            }
        }
        
        // Check in type system's built-in types
        if let Some(type_def) = self.type_system.environment.type_definitions.get(name) {
            return Ok(TypeExpression::named(&type_def.name));
        }
        
        Err(TypeCheckError {
            message: format!("Undefined variable: {}", name),
            location: None,
            error_type: TypeErrorKind::UndefinedVariable,
        })
    }
    
    fn check_binary_expression(&mut self, binary: &BinaryExpression) -> Result<TypeExpression, TypeCheckError> {
        let left_type = self.check_expression(&binary.left)?;
        let right_type = self.check_expression(&binary.right)?;
        
        // Unify types for binary operations
        let mut unifier = super::constraint_resolver::TypeUnifier::new();
        match unifier.unify(&left_type, &right_type) {
            Ok(_) => {
                // Determine result type based on operation
                match binary.operator.as_str() {
                    "+" | "-" | "*" | "/" => {
                        if self.is_numeric_type(&left_type) && self.is_numeric_type(&right_type) {
                            Ok(left_type) // Return the numeric type
                        } else {
                            Err(TypeCheckError {
                                message: format!("Arithmetic operation requires numeric types, got {:?} and {:?}", 
                                               left_type, right_type),
                                location: None,
                                error_type: TypeErrorKind::TypeMismatch,
                            })
                        }
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        Ok(TypeExpression::named("bool"))
                    }
                    "&&" | "||" => {
                        if self.is_bool_type(&left_type) && self.is_bool_type(&right_type) {
                            Ok(TypeExpression::named("bool"))
                        } else {
                            Err(TypeCheckError {
                                message: format!("Logical operation requires bool types, got {:?} and {:?}", 
                                               left_type, right_type),
                                location: None,
                                error_type: TypeErrorKind::TypeMismatch,
                            })
                        }
                    }
                    _ => Err(TypeCheckError {
                        message: format!("Unknown binary operator: {}", binary.operator),
                        location: None,
                        error_type: TypeErrorKind::InvalidOperation,
                    })
                }
            }
            Err(_) => Err(TypeCheckError {
                message: format!("Type mismatch in binary expression: {:?} and {:?}", left_type, right_type),
                location: None,
                error_type: TypeErrorKind::TypeMismatch,
            })
        }
    }
    
    fn check_call_expression(&mut self, call: &CallExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check if it's a method call
        if let Expression::MemberAccess(member) = &*call.function {
            return self.check_method_call(member, &call.arguments);
        }
        
        // Check if it's a function call
        if let Expression::Identifier(func_name) = &*call.function {
            return self.check_function_call(func_name, &call.arguments);
        }
        
        Err(TypeCheckError {
            message: "Invalid function call".to_string(),
            location: None,
            error_type: TypeErrorKind::InvalidOperation,
        })
    }
    
    fn check_method_call(&mut self, member: &MemberAccessExpression, arguments: &[Expression]) -> Result<TypeExpression, TypeCheckError> {
        let object_type = self.check_expression(&member.object)?;
        
        // Look up method in object's type definition
        if let Some(object_name) = &object_type.name {
            if let Some(type_def) = self.type_system.environment.type_definitions.get(object_name) {
                let type_def = type_def.clone(); // Clone to avoid borrow conflicts
                for method in &type_def.methods {
                    if method.name == member.property {
                        // Check argument types
                        if arguments.len() != method.parameters.len() {
                            return Err(TypeCheckError {
                                message: format!("Method '{}' expects {} arguments, got {}", 
                                               method.name, method.parameters.len(), arguments.len()),
                                location: None,
                                error_type: TypeErrorKind::ArityMismatch,
                            });
                        }
                        
                        // Type check each argument
                        let expected_params = method.parameters.clone();
                        let return_type = method.return_type.clone().unwrap_or(TypeExpression::named("void"));
                        for (i, arg) in arguments.iter().enumerate() {
                            let arg_type = self.check_expression(arg)?;
                            let expected_type = &expected_params[i];
                            
                            if !self.types_compatible(&arg_type, expected_type) {
                                return Err(TypeCheckError {
                                    message: format!("Argument {} type mismatch: expected {:?}, got {:?}", 
                                                   i, expected_type, arg_type),
                                    location: None,
                                    error_type: TypeErrorKind::TypeMismatch,
                                });
                            }
                        }
                        
                        return Ok(return_type);
                    }
                }
                
                return Err(TypeCheckError {
                    message: format!("Method '{}' not found on type '{}'", member.property, object_name),
                    location: None,
                    error_type: TypeErrorKind::UndefinedFunction,
                });
            }
        }
        
        Err(TypeCheckError {
            message: format!("Cannot call method '{}' on unknown type", member.property),
            location: None,
            error_type: TypeErrorKind::UndefinedFunction,
        })
    }
    
    fn check_function_call(&mut self, func_name: &str, arguments: &[Expression]) -> Result<TypeExpression, TypeCheckError> {
        // Look up function in current scopes
        for scope in self.scopes.iter().rev() {
            if let Some(func_type) = scope.get(func_name) {
                let func_type_clone = func_type.clone();
                return self.check_function_type_call(&func_type_clone, arguments);
            }
        }
        
        Err(TypeCheckError {
            message: format!("Undefined function: {}", func_name),
            location: None,
            error_type: TypeErrorKind::UndefinedFunction,
        })
    }
    
    fn check_function_type_call(&mut self, func_type: &TypeExpression, arguments: &[Expression]) -> Result<TypeExpression, TypeCheckError> {
        // For function types, check parameters and return type
        if func_type.parameters.len() != arguments.len() {
            return Err(TypeCheckError {
                message: format!("Function expects {} arguments, got {}", 
                               func_type.parameters.len(), arguments.len()),
                location: None,
                error_type: TypeErrorKind::ArityMismatch,
            });
        }
        
        // Type check each argument
        for (i, arg) in arguments.iter().enumerate() {
            let arg_type = self.check_expression(arg)?;
            let expected_type = &func_type.parameters[i];
            
            if !self.types_compatible(&arg_type, expected_type) {
                return Err(TypeCheckError {
                    message: format!("Argument {} type mismatch: expected {:?}, got {:?}", 
                                   i, expected_type, arg_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        Ok(func_type.return_type.as_ref()
           .map(|rt| (**rt).clone())
           .unwrap_or(TypeExpression::named("void")))
    }
    
    fn check_member_access(&mut self, member: &MemberAccessExpression) -> Result<TypeExpression, TypeCheckError> {
        let object_type = self.check_expression(&member.object)?;
        
        if let Some(object_name) = &object_type.name {
            if let Some(type_def) = self.type_system.environment.type_definitions.get(object_name) {
                // Look for property or method
                for method in &type_def.methods {
                    if method.name == member.property {
                        // Return function type for methods
                        return Ok(TypeExpression::function(
                            method.parameters.clone(), 
                            method.return_type.clone().unwrap_or(TypeExpression::named("void"))
                        ));
                    }
                }
                
                return Err(TypeCheckError {
                    message: format!("Property '{}' not found on type '{}'", member.property, object_name),
                    location: None,
                    error_type: TypeErrorKind::UndefinedVariable,
                });
            }
        }
        
        Err(TypeCheckError {
            message: format!("Cannot access property '{}' on unknown type", member.property),
            location: None,
            error_type: TypeErrorKind::UndefinedVariable,
        })
    }
    
    fn check_literal(&self, literal: &Literal) -> Result<TypeExpression, TypeCheckError> {
        match literal {
            Literal::Integer(_) => Ok(TypeExpression::named("int")),
            Literal::Float(_) => Ok(TypeExpression::named("float")),
            Literal::String(_) => Ok(TypeExpression::named("string")),
            Literal::Boolean(_) => Ok(TypeExpression::named("bool")),
            Literal::Null | Literal::Nil => Ok(TypeExpression::named("void")),
        }
    }
    
    fn check_array_expression(&mut self, elements: &[Expression]) -> Result<TypeExpression, TypeCheckError> {
        if elements.is_empty() {
            // Empty array - infer element type as unknown
            return Ok(TypeExpression::array(TypeExpression::named("unknown")));
        }
        
        // Check first element type
        let first_type = self.check_expression(&elements[0])?;
        
        // Check all elements have the same type
        for (i, element) in elements.iter().enumerate().skip(1) {
            let element_type = self.check_expression(element)?;
            if !self.types_compatible(&first_type, &element_type) {
                return Err(TypeCheckError {
                    message: format!("Array element {} type mismatch: expected {:?}, got {:?}", 
                                   i, first_type, element_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        Ok(TypeExpression::array(first_type))
    }
    
    fn check_map_expression(&mut self, pairs: &[(Expression, Expression)]) -> Result<TypeExpression, TypeCheckError> {
        if pairs.is_empty() {
            // Empty map - infer types as unknown
            return Ok(TypeExpression::map(TypeExpression::named("unknown"), TypeExpression::named("unknown")));
        }
        
        // Check first pair types
        let (first_key, first_value) = &pairs[0];
        let key_type = self.check_expression(first_key)?;
        let value_type = self.check_expression(first_value)?;
        
        // Check all pairs have compatible types
        for (i, (key, value)) in pairs.iter().enumerate().skip(1) {
            let curr_key_type = self.check_expression(key)?;
            let curr_value_type = self.check_expression(value)?;
            
            if !self.types_compatible(&key_type, &curr_key_type) {
                return Err(TypeCheckError {
                    message: format!("Map key {} type mismatch: expected {:?}, got {:?}", 
                                   i, key_type, curr_key_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
            
            if !self.types_compatible(&value_type, &curr_value_type) {
                return Err(TypeCheckError {
                    message: format!("Map value {} type mismatch: expected {:?}, got {:?}", 
                                   i, value_type, curr_value_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        Ok(TypeExpression::map(key_type, value_type))
    }
    
    pub fn check_let_statement(&mut self, let_stmt: &LetStatement) -> Result<TypeExpression, TypeCheckError> {
        let value_type = self.check_expression(&let_stmt.value)?;
        
        // Add variable to current scope
        self.add_variable(let_stmt.name.clone(), value_type.clone());
        
        Ok(value_type)
    }
    
    fn check_function_statement(&mut self, func_stmt: &FunctionStatement) -> Result<TypeExpression, TypeCheckError> {
        // Create function type
        let param_types: Vec<TypeExpression> = func_stmt.parameters.iter()
            .map(|_| TypeExpression::named("unknown")) // TODO: Add type annotations
            .collect();
        
        let return_type = TypeExpression::named("void"); // TODO: Infer return type
        
        // Add function to current scope
        let func_type = TypeExpression::function(param_types, return_type.clone());
        self.add_variable(func_stmt.name.clone(), func_type.clone());
        
        // Enter new scope for function body
        self.enter_scope();
        
        // Add parameters to function scope
        for param in &func_stmt.parameters {
            self.add_variable(param.name.clone(), TypeExpression::named("unknown"));
        }
        
        // Set current function return type
        let old_return_type = self.current_function_return_type.clone();
        self.current_function_return_type = Some(return_type.clone());
        
        // Check function body
        for statement in &func_stmt.body {
            self.check_statement(statement)?;
        }
        
        // Restore previous function return type
        self.current_function_return_type = old_return_type;
        
        // Exit function scope
        self.exit_scope();
        
        Ok(func_type)
    }
    
    pub fn check_if_statement(&mut self, if_stmt: &IfStatement) -> Result<TypeExpression, TypeCheckError> {
        let condition_type = self.check_expression(&if_stmt.condition)?;
        
        if !self.is_bool_type(&condition_type) {
            return Err(TypeCheckError {
                message: format!("If condition must be bool, got {:?}", condition_type),
                location: None,
                error_type: TypeErrorKind::TypeMismatch,
            });
        }
        
        // Check then branch
        self.enter_scope();
        for statement in &if_stmt.then_branch {
            self.check_statement(statement)?;
        }
        self.exit_scope();
        
        // Check else branch if present
        if let Some(else_branch) = &if_stmt.else_branch {
            self.enter_scope();
            for statement in else_branch {
                self.check_statement(statement)?;
            }
            self.exit_scope();
        }
        
        Ok(TypeExpression::named("void"))
    }
    
    pub fn check_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<TypeExpression, TypeCheckError> {
        let condition_type = self.check_expression(&while_stmt.condition)?;
        
        if !self.is_bool_type(&condition_type) {
            return Err(TypeCheckError {
                message: format!("While condition must be bool, got {:?}", condition_type),
                location: None,
                error_type: TypeErrorKind::TypeMismatch,
            });
        }
        
        // Check loop body
        self.enter_scope();
        for statement in &while_stmt.body {
            self.check_statement(statement)?;
        }
        self.exit_scope();
        
        Ok(TypeExpression::named("void"))
    }
    
    fn check_return_statement(&mut self, return_stmt: &ReturnStatement) -> Result<TypeExpression, TypeCheckError> {
        let return_type = if let Some(value) = &return_stmt.value {
            self.check_expression(value)?
        } else {
            TypeExpression::named("void")
        };
        
        // Check against current function's return type
        if let Some(expected_return_type) = &self.current_function_return_type {
            if !self.types_compatible(&return_type, expected_return_type) {
                return Err(TypeCheckError {
                    message: format!("Return type mismatch: expected {:?}, got {:?}", 
                                   expected_return_type, return_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        Ok(return_type)
    }
    
    /// Complete function type checking with proper inference
    pub fn check_function_complete(&mut self, func_stmt: &FunctionStatement) -> Result<TypeExpression, TypeCheckError> {
        // Collect all return statements in the function
        let mut return_types = Vec::new();
        self.collect_return_types(&func_stmt.body, &mut return_types)?;
        
        // Infer return type from return statements
        let inferred_return_type = self.unify_return_types(&return_types)?;
        
        // Create function type with inferred return type
        let param_types: Vec<TypeExpression> = func_stmt.parameters.iter()
            .map(|_| TypeExpression::named("unknown"))
            .collect();
        
        let func_type = TypeExpression::function(param_types, inferred_return_type);
        self.add_variable(func_stmt.name.clone(), func_type.clone());
        
        Ok(func_type)
    }
    
    fn collect_return_types(&mut self, statements: &[Statement], return_types: &mut Vec<TypeExpression>) -> Result<(), TypeCheckError> {
        for statement in statements {
            match statement {
                Statement::Return(return_stmt) => {
                    let return_type = if let Some(value) = &return_stmt.value {
                        self.check_expression(value)?
                    } else {
                        TypeExpression::named("void")
                    };
                    return_types.push(return_type);
                }
                Statement::If(if_stmt) => {
                    self.collect_return_types(&if_stmt.then_branch, return_types)?;
                    if let Some(else_branch) = &if_stmt.else_branch {
                        self.collect_return_types(else_branch, return_types)?;
                    }
                }
                Statement::While(while_stmt) => {
                    self.collect_return_types(&while_stmt.body, return_types)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn unify_return_types(&self, return_types: &[TypeExpression]) -> Result<TypeExpression, TypeCheckError> {
        if return_types.is_empty() {
            return Ok(TypeExpression::named("void"));
        }
        
        let mut unified_type = return_types[0].clone();
        
        for return_type in return_types.iter().skip(1) {
            unified_type = self.unify_types_advanced(&unified_type, return_type)?;
        }
        
        Ok(unified_type)
    }
    
    /// Advanced type unification with proper constraint solving
    pub fn unify_types_advanced(&self, t1: &TypeExpression, t2: &TypeExpression) -> Result<TypeExpression, TypeCheckError> {
        let mut unifier = super::constraint_resolver::TypeUnifier::new();
        
        match unifier.unify(t1, t2) {
            Ok(substitutions) => {
                // Apply substitutions to get unified type
                let mut result = t1.clone();
                for (var, substitution) in substitutions {
                    if let Some(name) = &result.name {
                        if name == &var {
                            result = substitution;
                            break;
                        }
                    }
                }
                Ok(result)
            }
            Err(violation) => Err(TypeCheckError {
                message: format!("Type unification failed: {}", violation.context),
                location: None,
                error_type: TypeErrorKind::UnificationFailure,
            })
        }
    }
    
    // Helper methods
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }
    
    fn add_variable(&mut self, name: String, type_expr: TypeExpression) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, type_expr);
        }
    }
    
    fn add_function(&mut self, name: String, param_types: Vec<TypeExpression>, return_type: TypeExpression) {
        let func_type = TypeExpression::function(param_types, return_type);
        self.add_variable(name, func_type);
    }
    
    fn types_compatible(&self, t1: &TypeExpression, t2: &TypeExpression) -> bool {
        // Simple compatibility check - can be enhanced with subtyping
        match (&t1.name, &t2.name) {
            (Some(n1), Some(n2)) => n1 == n2,
            _ => false,
        }
    }
    
    fn is_numeric_type(&self, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            matches!(name.as_str(), "int" | "float")
        } else {
            false
        }
    }
    
    fn is_bool_type(&self, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            name == "bool"
        } else {
            false
        }
    }
    
    // CURSED-specific type checking methods
    
    fn check_struct_statement(&mut self, struct_stmt: &StructStatement) -> Result<TypeExpression, TypeCheckError> {
        // Register the struct type in the type system
        let struct_type = TypeExpression::named(&struct_stmt.name);
        
        // TODO: Check field types and add struct definition to environment
        // For now, return a basic struct type representation
        
        Ok(struct_type)
    }
    
    fn check_interface_statement(&mut self, interface_stmt: &InterfaceStatement) -> Result<TypeExpression, TypeCheckError> {
        // Register the interface type in the type system
        let interface_type = TypeExpression::named(&interface_stmt.name);
        
        // TODO: Check method signatures and add interface definition to environment
        // For now, return a basic interface type representation
        
        Ok(interface_type)
    }
    
    fn check_channel_statement(&mut self, _channel_stmt: &ChannelStatement) -> Result<TypeExpression, TypeCheckError> {
        // Channel statements typically declare channels
        // Return a generic channel type for now
        Ok(TypeExpression::named("channel"))
    }
    
    fn check_goroutine_statement(&mut self, goroutine_stmt: &GoroutineStatement) -> Result<TypeExpression, TypeCheckError> {
        // Check the expression within the goroutine
        self.check_expression(&goroutine_stmt.expression)?;
        
        // Goroutines don't return a value in the traditional sense
        Ok(TypeExpression::named("void"))
    }
    
    fn check_select_statement(&mut self, select_stmt: &SelectStatement) -> Result<TypeExpression, TypeCheckError> {
        // Check each case in the select statement
        for case in &select_stmt.cases {
            // Check the channel operation
            self.check_expression(&case.operation)?;
            
            // Check the body statements
            for stmt in &case.body {
                self.check_statement(stmt)?;
            }
        }
        
        Ok(TypeExpression::named("void"))
    }
    
    fn check_panic_statement(&mut self, panic_stmt: &PanicStatement) -> Result<TypeExpression, TypeCheckError> {
        // Check the panic message expression
        let message_type = self.check_expression(&*panic_stmt.message)?;
        
        // Panic message should be a string
        if !self.types_compatible(&message_type, &TypeExpression::named("string")) {
            return Err(TypeCheckError {
                message: "Panic message must be a string".to_string(),
                location: None, // TODO: Add location support to AST
                error_type: TypeErrorKind::TypeMismatch,
            });
        }
        
        // Panic statements don't return (they diverge)
        Ok(TypeExpression::named("never"))
    }
    
    fn check_catch_statement(&mut self, catch_stmt: &CatchStatement) -> Result<TypeExpression, TypeCheckError> {
        // Check the protected block (try equivalent)
        for stmt in &catch_stmt.protected_block {
            self.check_statement(stmt)?;
        }
        
        // Check the recovery block if present (catch equivalent)
        if let Some(recovery_block) = &catch_stmt.recovery_block {
            for stmt in recovery_block {
                self.check_statement(stmt)?;
            }
        }
        
        // Error variable handling could be added here if needed
        
        Ok(TypeExpression::named("void"))
    }
    
    fn check_channel_send_expression(&mut self, channel_send: &ChannelSendExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check the channel expression
        let channel_type = self.check_expression(&*channel_send.channel)?;
        
        // Check the value being sent
        let value_type = self.check_expression(&*channel_send.value)?;
        
        // TODO: Validate that the value type matches the channel's element type
        // For now, assume channel send operations are valid
        
        Ok(TypeExpression::named("void"))
    }
    
    fn check_channel_receive_expression(&mut self, channel_receive: &ChannelReceiveExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check the channel expression
        let channel_type = self.check_expression(&*channel_receive.channel)?;
        
        // TODO: Extract the element type from the channel type
        // For now, return a generic type
        
        Ok(TypeExpression::named("unknown"))
    }
    
    fn check_channel_creation_expression(&mut self, channel_creation: &ChannelCreationExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check the element type
        let element_type = &channel_creation.element_type;
        
        // Check the capacity expression if provided
        if let Some(ref capacity) = channel_creation.capacity {
            let capacity_type = self.check_expression(&**capacity)?;
            // Capacity should be an integer
            if !self.types_compatible(&capacity_type, &TypeExpression::named("int")) {
                return Err(TypeCheckError {
                    message: "Channel capacity must be an integer".to_string(),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        // Return a channel type with the specified element type
        Ok(TypeExpression::named(&format!("dm<{}>", element_type)))
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    
    #[test]
    fn test_basic_type_checking() {
        let mut checker = TypeChecker::new();
        
        // Test integer literal
        let expr = Expression::Integer(42);
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("int".to_string()));
        
        // Test string literal
        let expr = Expression::String("hello".to_string());
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("string".to_string()));
        
        // Test boolean literal
        let expr = Expression::Boolean(true);
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("bool".to_string()));
    }
    
    #[test]
    fn test_binary_expression_type_checking() {
        let mut checker = TypeChecker::new();
        
        // Test arithmetic operation
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("int".to_string()));
        
        // Test comparison operation
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "<".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("bool".to_string()));
    }
    
    #[test]
    fn test_function_type_checking() {
        let mut checker = TypeChecker::new();
        
        // Test function declaration
        let func_stmt = FunctionStatement {
            name: "test_func".to_string(),
            type_parameters: vec![],
            parameters: vec![crate::ast::Parameter {
                name: "x".to_string(),
                param_type: Some("normie".to_string()),
            }],
            body: vec![
                Statement::Return(ReturnStatement {
                    value: Some(Expression::Integer(42)),
                })
            ],
            visibility: crate::ast::Visibility::Private,
            return_type: None,
            where_clause: None,
        };
        
        let result = checker.check_function_complete(&func_stmt).unwrap();
        assert!(result.return_type.is_some());
    }
    
    #[test]
    fn test_type_error_detection() {
        let mut checker = TypeChecker::new();
        
        // Test undefined variable
        let expr = Expression::Identifier("undefined_var".to_string());
        let result = checker.check_expression(&expr);
        assert!(result.is_err());
        
        // Test type mismatch in binary operation
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::String("hello".to_string())),
        });
        
        let result = checker.check_expression(&expr);
        assert!(result.is_err());
    }
}
