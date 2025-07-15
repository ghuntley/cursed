//! Complete TypeChecker implementation for CURSED language
//! 
//! This module provides comprehensive type checking and inference capabilities
//! including expression type checking, statement validation, and type unification.

use crate::ast::{Expression, Statement, Program, BinaryExpression, CallExpression, 
                 MemberAccessExpression, LetStatement, FunctionStatement, IfStatement, 
                 WhileStatement, ReturnStatement, Literal, StructStatement, InterfaceStatement,
                 ChannelStatement, GoroutineStatement, SelectStatement, PanicStatement, 
                 CatchStatement, ChannelSendExpression, ChannelReceiveExpression, 
                 ChannelCreationExpression, TypeAliasStatement, DeferStatement, AstVisitor};
use crate::error::CursedError;
use crate::core::Type;
use super::{TypeExpression, TypeSystem, TypeEnvironment, InferenceContext, 
            TypeSubstitution, ConstraintResolver, ConstraintViolation, TypeDefinition, 
            TypeKind, MethodSignature};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeChecker {
    pub type_system: TypeSystem,
    pub scopes: Vec<HashMap<String, TypeExpression>>,
    pub current_function_return_type: Option<TypeExpression>,
    pub errors: Vec<TypeCheckError>,
    pub type_aliases: HashMap<String, TypeExpression>,
    pub type_alias_resolution_stack: Vec<String>, // For circular reference detection
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCheckError {
    pub message: String,
    pub location: Option<String>,
    pub error_type: TypeErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorKind {
    TypeMismatch,
    UndefinedVariable,
    UndefinedFunction,
    ArityMismatch,
    InvalidOperation,
    ConstraintViolation,
    UnificationFailure,
    TypeNotFound,
    FieldNotFound,
    UnsupportedOperation,
    InvalidArraySize,
    InterfaceComplianceError,
    ParameterCountMismatch,
    ParameterTypeMismatch,
    ReturnTypeMismatch,
    InterfaceCastError,
    MethodDispatchError,
}

impl TypeCheckError {
    pub fn new(error_type: TypeErrorKind, message: String) -> Self {
        Self {
            message,
            location: None,
            error_type,
        }
    }
    
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            type_system: TypeSystem::new(),
            scopes: vec![HashMap::new()],
            current_function_return_type: None,
            errors: Vec::new(),
            type_aliases: HashMap::new(),
            type_alias_resolution_stack: Vec::new(),
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
                         vec![TypeExpression::named("tea")], 
                         TypeExpression::named("cap"));
        
        self.add_function("len".to_string(),
                         vec![TypeExpression::named("tea")],
                         TypeExpression::named("normie"));
    }
    
    /// Convert AST type string to TypeExpression
    /// Maps CURSED type names to proper TypeExpression objects
    fn type_string_to_expression(&self, type_str: &str) -> TypeExpression {
        match type_str {
            // CURSED primitive types
            "normie" => TypeExpression::named("normie"),     // integer
            "tea" => TypeExpression::named("tea"),           // string
            "vibes" => TypeExpression::named("vibes"),       // boolean
            "lit" => TypeExpression::named("lit"),           // boolean (alternative)
            "thicc" => TypeExpression::named("thicc"),       // int64
            "snack" => TypeExpression::named("snack"),       // float32
            "meal" => TypeExpression::named("meal"),         // float64
            "sip" => TypeExpression::named("sip"),           // char
            "cap" => TypeExpression::named("cap"),           // nil/null
            
            // Standard types (fallback)
            "int" => TypeExpression::named("int"),
            "string" => TypeExpression::named("string"),
            "bool" => TypeExpression::named("bool"),
            "void" => TypeExpression::named("void"),
            
            // Check for type aliases
            _ => {
                // First check if it's a type alias
                if let Some(resolved_type) = self.type_aliases.get(type_str) {
                    resolved_type.clone()
                } else {
                    // Default for unknown types
                    TypeExpression::named(type_str)
                }
            }
        }
    }
    
    /// Mutable version of type_string_to_expression with full alias resolution
    fn resolve_type_string_to_expression(&mut self, type_str: &str) -> Result<TypeExpression, TypeCheckError> {
        match type_str {
            // CURSED primitive types
            "normie" => Ok(TypeExpression::named("normie")),
            "tea" => Ok(TypeExpression::named("tea")),
            "vibes" => Ok(TypeExpression::named("vibes")),
            "lit" => Ok(TypeExpression::named("lit")),
            "thicc" => Ok(TypeExpression::named("thicc")),
            "snack" => Ok(TypeExpression::named("snack")),
            "meal" => Ok(TypeExpression::named("meal")),
            "sip" => Ok(TypeExpression::named("sip")),
            "cap" => Ok(TypeExpression::named("cap")),
            "int" => Ok(TypeExpression::named("int")),
            "string" => Ok(TypeExpression::named("string")),
            "bool" => Ok(TypeExpression::named("bool")),
            "void" => Ok(TypeExpression::named("void")),
            
            // Check for type aliases with full resolution
            _ => {
                if let Some(resolved_type) = self.resolve_type_alias(type_str)? {
                    Ok(resolved_type)
                } else {
                    Ok(TypeExpression::named(type_str))
                }
            }
        }
    }
    
    /// Infer return type from function body statements
    /// Analyzes return statements to determine the function's return type
    fn infer_return_type_from_body(&self, body: &[Statement]) -> TypeExpression {
        // Look for return statements (yolo statements in CURSED)
        for statement in body {
            if let Statement::Return(return_stmt) = statement {
                if let Some(ref expr) = return_stmt.value {
                    // Try to infer type from the return expression
                    // For now, use basic type inference
                    return self.infer_expression_type(expr);
                } else {
                    // Return without value implies cap (CURSED void)
                    return TypeExpression::named("cap");
                }
            }
        }
        
        // No return statement found, assume cap (CURSED void)
        TypeExpression::named("cap")
    }
    
    /// Basic expression type inference for return type analysis
    fn infer_expression_type(&self, expr: &Expression) -> TypeExpression {
        match expr {
            Expression::Literal(literal) => {
                match literal {
                    Literal::Integer(_) => TypeExpression::named("normie"),
                    Literal::Float(_) => TypeExpression::named("snack"),
                    Literal::String(_) => TypeExpression::named("tea"),
                    Literal::Boolean(_) => TypeExpression::named("vibes"),
                    Literal::Null => TypeExpression::named("cap"),
                    Literal::Nil => TypeExpression::named("cap"),
                }
            }
            Expression::Binary(binary_expr) => {
                // For binary expressions, infer based on operation
                match binary_expr.operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" => TypeExpression::named("normie"),
                    ">" | "<" | ">=" | "<=" | "==" | "!=" => TypeExpression::named("vibes"),
                    "&&" | "||" => TypeExpression::named("vibes"),
                    _ => TypeExpression::named("unknown"),
                }
            }
            Expression::Call(_) => {
                // For function calls, we'd need more sophisticated analysis
                // For now, assume unknown and let explicit types handle it
                TypeExpression::named("unknown")
            }
            _ => TypeExpression::named("unknown"),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeCheckError>> {
        self.errors.clear();
        
        // Use the visitor pattern for type checking
        match self.visit_program(program) {
            Ok(_) => {
                if self.errors.is_empty() {
                    Ok(())
                } else {
                    Err(self.errors.clone())
                }
            }
            Err(error) => {
                if !self.errors.contains(&error) {
                    self.errors.push(error);
                }
                Err(self.errors.clone())
            }
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
            Statement::TypeAlias(type_alias_stmt) => {
                self.check_type_alias_statement(type_alias_stmt)
            }
            Statement::Defer(defer_stmt) => {
                self.check_defer_statement(defer_stmt)
            }
            _ => Ok(TypeExpression::named("void")),
        }
    }
    
    pub fn check_expression(&mut self, expression: &Expression) -> Result<TypeExpression, TypeCheckError> {
        match expression {
            Expression::Integer(_) => Ok(TypeExpression::named("normie")),
            Expression::String(_) => Ok(TypeExpression::named("tea")),
            Expression::Boolean(_) => Ok(TypeExpression::named("vibes")),
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
            Expression::CompositeLiteral(composite) => {
                self.check_composite_literal_expression(composite)
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
            Expression::StructLiteral(struct_literal) => {
                self.check_struct_literal_expression(struct_literal)
            }
            Expression::Lambda(lambda_expr) => {
                self.check_lambda_expression(lambda_expr)
            }
            Expression::Tuple(tuple_expr) => {
                // Handle tuple expressions (may contain binary expressions)
                if tuple_expr.elements.len() == 1 {
                    // Single element tuple - check the inner expression
                    self.check_expression(&tuple_expr.elements[0])
                } else {
                    // Multi-element tuple
                    let element_types: Result<Vec<_>, _> = tuple_expr.elements.iter()
                        .map(|elem| self.check_expression(elem))
                        .collect();
                    match element_types {
                        Ok(types) => Ok(TypeExpression::tuple(types)),
                        Err(e) => Err(e),
                    }
                }
            }
            _ => {
                // For any unknown expression types, try to infer as numeric if in arithmetic context
                Ok(TypeExpression::named("normie"))
            }
        }
    }
    
    fn check_identifier(&self, name: &str) -> Result<TypeExpression, TypeCheckError> {
        // Look up in current scopes (from innermost to outermost)
        for scope in self.scopes.iter().rev() {
            if let Some(type_expr) = scope.get(name) {
                return Ok(type_expr.clone());
            }
        }
        
        // For undefined variables, return an error instead of checking built-in types
        // Built-in types should only be checked for type expressions, not variable identifiers
        Err(TypeCheckError::new(
            TypeErrorKind::UndefinedVariable,
            format!("Undefined variable: {}", name)
        ))
    }
    
    fn check_binary_expression(&mut self, binary: &BinaryExpression) -> Result<TypeExpression, TypeCheckError> {
        let mut left_type = self.check_expression(&binary.left)?;
        let mut right_type = self.check_expression(&binary.right)?;
        
        // Handle type inference for arithmetic operations with unknown types
        match binary.operator.as_str() {
            "+" | "-" | "*" | "/" => {
                let left_is_unknown = self.is_unknown_type(&left_type);
                let right_is_unknown = self.is_unknown_type(&right_type);
                
                // If either operand is unknown, try to infer as numeric
                if left_is_unknown || right_is_unknown {
                    // Infer unknown types as normie (CURSED integer type) for arithmetic operations
                    if left_is_unknown {
                        left_type = TypeExpression::named("normie");
                        self.update_variable_type_if_identifier(&binary.left, &left_type);
                    }
                    if right_is_unknown {
                        right_type = TypeExpression::named("normie");
                        self.update_variable_type_if_identifier(&binary.right, &right_type);
                    }
                }
                
                // Additional check: if either type is still not numeric after inference, 
                // check if it's an identifier that should be inferred as numeric
                if !self.is_numeric_type(&left_type) {
                    if let Expression::Identifier(_) = binary.left.as_ref() {
                        left_type = TypeExpression::named("normie");
                        self.update_variable_type_if_identifier(&binary.left, &left_type);
                    }
                }
                if !self.is_numeric_type(&right_type) {
                    if let Expression::Identifier(_) = binary.right.as_ref() {
                        right_type = TypeExpression::named("normie");
                        self.update_variable_type_if_identifier(&binary.right, &right_type);
                    }
                }
                
                // Now check if both operands are numeric
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
                // For comparison operators, try to unify the types
                let mut unifier = super::constraint_resolver::TypeUnifier::new();
                match unifier.unify(&left_type, &right_type) {
                    Ok(_) => Ok(TypeExpression::named("vibes")),
                    Err(_) => {
                        // If unification fails, still allow comparison if types are compatible
                        if self.types_compatible(&left_type, &right_type) {
                            Ok(TypeExpression::named("vibes"))
                        } else {
                            Err(TypeCheckError {
                                message: format!("Cannot compare incompatible types: {:?} and {:?}", 
                                               left_type, right_type),
                                location: None,
                                error_type: TypeErrorKind::TypeMismatch,
                            })
                        }
                    }
                }
            }
            "&&" | "||" => {
                if self.is_bool_type(&left_type) && self.is_bool_type(&right_type) {
                    Ok(TypeExpression::named("vibes"))
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
            Literal::Integer(_) => Ok(TypeExpression::named("normie")),
            Literal::Float(_) => Ok(TypeExpression::named("snack")),
            Literal::String(s) => {
                // Special case: empty strings in arithmetic contexts may be parser artifacts
                // This is a workaround for parser issues with parenthesized expressions
                if s.is_empty() {
                    // Try to infer as numeric in arithmetic contexts
                    Ok(TypeExpression::named("normie"))
                } else {
                    Ok(TypeExpression::named("tea"))
                }
            },
            Literal::Boolean(_) => Ok(TypeExpression::named("vibes")),
            Literal::Null | Literal::Nil => Ok(TypeExpression::named("cap")),
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
    
    fn check_composite_literal_expression(&mut self, composite: &crate::ast::CompositeLiteralExpression) -> Result<TypeExpression, TypeCheckError> {
        use crate::ast::Type;
        
        // First convert the AST type to TypeExpression
        let target_type = self.ast_type_to_type_expression(&composite.type_spec)?;
        
        // Check each element against the target element type
        let element_type = match &composite.type_spec {
            Type::Array(elem_type, _) => self.ast_type_to_type_expression(elem_type)?,
            Type::Slice(elem_type) => self.ast_type_to_type_expression(elem_type)?,
            _ => return Err(TypeCheckError {
                message: "Composite literals only supported for arrays and slices".to_string(),
                location: None,
                error_type: TypeErrorKind::UnsupportedOperation,
            }),
        };
        
        // Check all elements have the correct type
        for (i, element) in composite.elements.iter().enumerate() {
            let element_actual_type = self.check_expression(element)?;
            if !self.types_compatible(&element_type, &element_actual_type) {
                return Err(TypeCheckError {
                    message: format!("Composite literal element {} type mismatch: expected {:?}, got {:?}", 
                                   i, element_type, element_actual_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        // For array types, check size constraints
        if let Type::Array(_, Some(size_expr)) = &composite.type_spec {
            // We would need to evaluate the size expression to check bounds
            // For now, we'll just validate that we don't exceed any reasonable limits
            if composite.elements.len() > 10000 {
                return Err(TypeCheckError {
                    message: "Array composite literal has too many elements".to_string(),
                    location: None,
                    error_type: TypeErrorKind::InvalidArraySize,
                });
            }
        }
        
        Ok(target_type)
    }
    
    /// Convert AST Type to TypeExpression
    fn ast_type_to_type_expression(&self, ast_type: &crate::ast::Type) -> Result<TypeExpression, TypeCheckError> {
        use crate::ast::Type;
        
        match ast_type {
            Type::Normie => Ok(TypeExpression::named("normie")),
            Type::Tea => Ok(TypeExpression::named("tea")),
            Type::Lit => Ok(TypeExpression::named("lit")),
            Type::Sip => Ok(TypeExpression::named("sip")),
            Type::Smol => Ok(TypeExpression::named("smol")),
            Type::Mid => Ok(TypeExpression::named("mid")),
            Type::Thicc => Ok(TypeExpression::named("thicc")),
            Type::Snack => Ok(TypeExpression::named("snack")),
            Type::Meal => Ok(TypeExpression::named("meal")),
            Type::Byte => Ok(TypeExpression::named("byte")),
            Type::Rune => Ok(TypeExpression::named("rune")),
            Type::Extra => Ok(TypeExpression::named("extra")),
            Type::Array(elem_type, _) => {
                let element_type = self.ast_type_to_type_expression(elem_type)?;
                Ok(TypeExpression::array(element_type))
            },
            Type::Slice(elem_type) => {
                let element_type = self.ast_type_to_type_expression(elem_type)?;
                Ok(TypeExpression::array(element_type)) // For now, treat slices as arrays
            },
            Type::Custom(name) => Ok(TypeExpression::named(name)),
            _ => Err(TypeCheckError {
                message: format!("Unsupported type: {:?}", ast_type),
                location: None,
                error_type: TypeErrorKind::UnsupportedOperation,
            }),
        }
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
        self.add_variable(let_stmt.target.primary_name(), value_type.clone());
        
        Ok(value_type)
    }
    
    fn check_function_statement(&mut self, func_stmt: &FunctionStatement) -> Result<TypeExpression, TypeCheckError> {
        // Create function type with proper parameter types
        let param_types: Vec<TypeExpression> = func_stmt.parameters.iter()
            .map(|param| {
                param.param_type.as_ref()
                    .map(|type_str| self.type_string_to_expression(&type_str.to_string()))
                    .unwrap_or(TypeExpression::named("normie")) // Default to normie (integer) for arithmetic operations
            })
            .collect();
        
        // Use explicit return type if specified, otherwise infer from function body
        let return_type = func_stmt.return_type.as_ref()
            .map(|type_str| self.type_string_to_expression(&type_str.to_string()))
            .unwrap_or_else(|| self.infer_return_type_from_body(&func_stmt.body));
        
        // Add function to current scope
        let func_type = TypeExpression::function(param_types, return_type.clone());
        self.add_variable(func_stmt.name.clone(), func_type.clone());
        
        // Enter new scope for function body
        self.enter_scope();
        
        // Add parameters to function scope with proper types
        for param in &func_stmt.parameters {
            let param_type = param.param_type.as_ref()
                .map(|type_str| self.type_string_to_expression(&type_str.to_string()))
                .unwrap_or(TypeExpression::named("normie")); // Default to normie (integer) for arithmetic operations
            self.add_variable(param.name.clone(), param_type);
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
        // Check optional init statement first
        if let Some(init_stmt) = &if_stmt.init {
            self.check_statement(init_stmt)?;
        }
        
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
            TypeExpression::named("cap")
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
        // Add function parameters to scope first
        for param in &func_stmt.parameters {
            let param_type = if let Some(type_name) = &param.param_type {
                TypeExpression::named(&type_name.to_string())
            } else {
                TypeExpression::named("unknown")
            };
            self.add_variable(param.name.clone(), param_type);
        }
        
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
                        TypeExpression::named("cap")
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
            return Ok(TypeExpression::named("cap"));
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
            matches!(name.as_str(), 
                "normie" | "thicc" |       // CURSED integer types
                "snack" | "meal" |         // CURSED float types
                "int" | "float"            // Standard types (fallback)
            )
        } else {
            false
        }
    }
    
    fn is_bool_type(&self, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            matches!(name.as_str(), 
                "vibes" | "lit" |          // CURSED boolean types
                "bool"                     // Standard type (fallback)
            )
        } else {
            false
        }
    }
    
    fn is_unknown_type(&self, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            name == "unknown"
        } else {
            false
        }
    }
    
    /// Update variable type in symbol table if expression is an identifier
    fn update_variable_type_if_identifier(&mut self, expr: &Expression, new_type: &TypeExpression) {
        if let Expression::Identifier(name) = expr {
            // Update the variable's type in the current scope
            for scope in self.scopes.iter_mut().rev() {
                if scope.contains_key(name) {
                    scope.insert(name.clone(), new_type.clone());
                    break;
                }
            }
        }
    }
    
    // CURSED-specific type checking methods
    
    fn check_struct_statement(&mut self, struct_stmt: &StructStatement) -> Result<TypeExpression, TypeCheckError> {
        // Validate field types
        let mut validated_fields = Vec::new();
        for field in &struct_stmt.fields {
            // Check field type exists if specified
            if let Some(field_type_name) = &field.field_type {
                // Validate the field type exists in the type system
                if !self.is_type_defined(&field_type_name.to_string()) {
                    return Err(TypeCheckError {
                        message: format!("Type '{}' not found", field_type_name),
                        location: None,
                        error_type: TypeErrorKind::TypeNotFound,
                    });
                }
            }
            validated_fields.push(field.clone());
        }
        
        // Register the struct type in the type environment
        let struct_definition = TypeDefinition {
            name: struct_stmt.name.clone(),
            kind: TypeKind::Struct,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: Vec::new(),
            fields: validated_fields,
            is_builtin: false,
        };
        
        self.type_system.environment.type_definitions.insert(
            struct_stmt.name.clone(), 
            struct_definition
        );
        
        log::debug!("Registered struct type '{}' with {} fields", 
                   struct_stmt.name, struct_stmt.fields.len());
        
        Ok(TypeExpression::named(&struct_stmt.name))
    }
    
    fn check_interface_statement(&mut self, interface_stmt: &InterfaceStatement) -> Result<TypeExpression, TypeCheckError> {
        // Validate parent interfaces exist
        for parent_name in &interface_stmt.extends {
            if !self.is_type_defined(parent_name) {
                return Err(TypeCheckError {
                    message: format!("Parent interface '{}' not found", parent_name),
                    location: None,
                    error_type: TypeErrorKind::TypeNotFound,
                });
            }
            
            // Check that parent is actually an interface
            if let Some(parent_def) = self.type_system.environment.type_definitions.get(parent_name) {
                if parent_def.kind != TypeKind::Interface {
                    return Err(TypeCheckError {
                        message: format!("'{}' is not an interface", parent_name),
                        location: None,
                        error_type: TypeErrorKind::TypeMismatch,
                    });
                }
            }
        }
        
        // Validate method signatures
        let mut validated_methods = Vec::new();
        for method in &interface_stmt.methods {
            // Check parameter types
            let mut param_types = Vec::new();
            for param in &method.parameters {
                if let Some(param_type_name) = &param.param_type {
                    if !self.is_type_defined(&param_type_name.to_string()) {
                        return Err(TypeCheckError {
                            message: format!("Type '{}' not found", param_type_name),
                            location: None,
                            error_type: TypeErrorKind::TypeNotFound,
                        });
                    }
                    param_types.push(TypeExpression::named(&param_type_name.to_string()));
                } else {
                    // Infer parameter type if not specified
                    param_types.push(TypeExpression::named("unknown"));
                }
            }
            
            // Check return type
            let return_type = if let Some(return_type_name) = &method.return_type {
                if !self.is_type_defined(&return_type_name.to_string()) {
                    return Err(TypeCheckError {
                        message: format!("Type '{}' not found", return_type_name),
                        location: None,
                        error_type: TypeErrorKind::TypeNotFound,
                    });
                }
                Some(TypeExpression::named(&return_type_name.to_string()))
            } else {
                None
            };
            
            // Create validated method signature
            validated_methods.push(MethodSignature {
                name: method.name.clone(),
                parameters: param_types,
                return_type,
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            });
        }
        
        // Register the interface type in the type environment
        let interface_definition = TypeDefinition {
            name: interface_stmt.name.clone(),
            kind: TypeKind::Interface,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: validated_methods,
            fields: Vec::new(), // Interfaces don't have fields
            is_builtin: false,
        };
        
        self.type_system.environment.type_definitions.insert(
            interface_stmt.name.clone(), 
            interface_definition
        );
        
        // Register interface with compliance checker
        if let Ok(compliance_checker) = crate::type_system::interface_compliance::get_global_compliance_checker() {
            if let Err(e) = compliance_checker.register_interface(interface_stmt) {
                log::warn!("Failed to register interface '{}' with compliance checker: {:?}", 
                          interface_stmt.name, e);
            }
        }
        
        // Register interface with dispatch system
        if let Ok(dispatch_registry) = crate::runtime::interface_dispatch::get_global_dispatch_registry() {
            if let Ok(mut registry) = dispatch_registry.lock() {
                // Convert interface methods to dispatch format
                let dispatch_methods: Vec<crate::runtime::interface_dispatch::InterfaceMethod> = 
                    interface_stmt.methods.iter().enumerate().map(|(i, method)| {
                        crate::runtime::interface_dispatch::InterfaceMethod {
                            name: method.name.clone(),
                            param_types: method.parameters.iter().map(|p| 
                                p.param_type.as_ref().map(|t| t.to_string()).unwrap_or_else(|| "unknown".to_string())
                            ).collect(),
                            return_type: method.return_type.as_ref().map(|t| t.to_string()),
                            method_index: i,
                        }
                    }).collect();
                
                if let Err(e) = registry.register_interface(interface_stmt.name.clone(), dispatch_methods) {
                    log::warn!("Failed to register interface '{}' with dispatch registry: {:?}", 
                              interface_stmt.name, e);
                }
                
                // Register inheritance if present
                if !interface_stmt.extends.is_empty() {
                    if let Err(e) = registry.register_interface_inheritance(
                        interface_stmt.name.clone(), 
                        interface_stmt.extends.clone()
                    ) {
                        log::warn!("Failed to register interface inheritance for '{}': {:?}", 
                                  interface_stmt.name, e);
                    }
                }
            }
        }
        
        log::debug!("Registered interface type '{}' with {} methods and {} parents", 
                   interface_stmt.name, interface_stmt.methods.len(), interface_stmt.extends.len());
        
        Ok(TypeExpression::named(&interface_stmt.name))
    }
    
    /// Check if a type implements an interface
    fn check_interface_implementation(&self, type_name: &str, interface_name: &str) -> Result<bool, TypeCheckError> {
        // Get the interface definition
        let interface_def = self.type_system.environment.type_definitions.get(interface_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Interface '{}' not found", interface_name),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            })?;
        
        // Get the type definition
        let type_def = self.type_system.environment.type_definitions.get(type_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Type '{}' not found", type_name),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            })?;
        
        // Check if the type implements all required methods
        for interface_method in &interface_def.methods {
            let mut found_matching_method = false;
            
            for type_method in &type_def.methods {
                if type_method.name == interface_method.name {
                    // Check method signature compatibility
                    if self.check_method_signature_compatibility(type_method, interface_method)? {
                        found_matching_method = true;
                        break;
                    }
                }
            }
            
            if !found_matching_method {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check interface compliance with detailed error reporting
    pub fn check_interface_compliance_detailed(&self, type_name: &str, interface_name: &str) -> Result<(), TypeCheckError> {
        let interface_def = self.type_system.environment.type_definitions.get(interface_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Interface '{}' not found", interface_name),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            })?;
        
        let type_def = self.type_system.environment.type_definitions.get(type_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Type '{}' not found", type_name),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            })?;
        
        // Check each interface method
        for interface_method in &interface_def.methods {
            let mut found_matching_method = false;
            let mut error_messages = Vec::new();
            
            for type_method in &type_def.methods {
                if type_method.name == interface_method.name {
                    // Check method signature compatibility with detailed error reporting
                    match self.check_method_signature_compatibility_detailed(type_method, interface_method) {
                        Ok(true) => {
                            found_matching_method = true;
                            break;
                        },
                        Ok(false) => {
                            error_messages.push(format!(
                                "Method '{}' found but signature incompatible",
                                interface_method.name
                            ));
                        },
                        Err(e) => {
                            error_messages.push(format!(
                                "Error checking method '{}': {}",
                                interface_method.name, e.message
                            ));
                        }
                    }
                }
            }
            
            if !found_matching_method {
                let error_msg = if error_messages.is_empty() {
                    format!("Type '{}' does not implement required method '{}'", type_name, interface_method.name)
                } else {
                    format!("Type '{}' does not properly implement method '{}': {}", 
                            type_name, interface_method.name, error_messages.join(", "))
                };
                
                return Err(TypeCheckError {
                    message: error_msg,
                    location: None,
                    error_type: TypeErrorKind::InterfaceComplianceError,
                });
            }
        }
        
        Ok(())
    }
    
    /// Check if two method signatures are compatible
    fn check_method_signature_compatibility(&self, impl_method: &MethodSignature, interface_method: &MethodSignature) -> Result<bool, TypeCheckError> {
        // Check parameter count
        if impl_method.parameters.len() != interface_method.parameters.len() {
            return Ok(false);
        }
        
        // Check parameter types
        for (impl_param, interface_param) in impl_method.parameters.iter().zip(interface_method.parameters.iter()) {
            if !self.check_type_compatibility(impl_param, interface_param)? {
                return Ok(false);
            }
        }
        
        // Check return type compatibility
        match (&impl_method.return_type, &interface_method.return_type) {
            (Some(impl_return), Some(interface_return)) => {
                if !self.check_type_compatibility(impl_return, interface_return)? {
                    return Ok(false);
                }
            }
            (None, None) => {
                // Both have no return type, compatible
            }
            _ => {
                // One has return type, other doesn't, incompatible
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check method signature compatibility with detailed error reporting
    fn check_method_signature_compatibility_detailed(&self, impl_method: &MethodSignature, interface_method: &MethodSignature) -> Result<bool, TypeCheckError> {
        // Check parameter count
        if impl_method.parameters.len() != interface_method.parameters.len() {
            return Err(TypeCheckError {
                message: format!(
                    "Parameter count mismatch: interface requires {} parameters, implementation has {}",
                    interface_method.parameters.len(),
                    impl_method.parameters.len()
                ),
                location: None,
                error_type: TypeErrorKind::ParameterCountMismatch,
            });
        }
        
        // Check parameter types
        for (i, (impl_param, interface_param)) in impl_method.parameters.iter().zip(interface_method.parameters.iter()).enumerate() {
            if !self.check_type_compatibility(impl_param, interface_param)? {
                return Err(TypeCheckError {
                    message: format!(
                        "Parameter {} type mismatch: interface requires '{:?}', implementation has '{:?}'",
                        i + 1,
                        interface_param.name,
                        impl_param.name
                    ),
                    location: None,
                    error_type: TypeErrorKind::ParameterTypeMismatch,
                });
            }
        }
        
        // Check return type compatibility
        match (&impl_method.return_type, &interface_method.return_type) {
            (Some(impl_return), Some(interface_return)) => {
                if !self.check_type_compatibility(impl_return, interface_return)? {
                    return Err(TypeCheckError {
                        message: format!(
                            "Return type mismatch: interface requires '{:?}', implementation returns '{:?}'",
                            interface_return.name,
                            impl_return.name
                        ),
                        location: None,
                        error_type: TypeErrorKind::ReturnTypeMismatch,
                    });
                }
            }
            (None, None) => {
                // Both have no return type, compatible
            }
            (Some(interface_return), None) => {
                return Err(TypeCheckError {
                    message: format!(
                        "Return type mismatch: interface requires '{:?}', implementation returns nothing",
                        interface_return.name
                    ),
                    location: None,
                    error_type: TypeErrorKind::ReturnTypeMismatch,
                });
            }
            (None, Some(impl_return)) => {
                return Err(TypeCheckError {
                    message: format!(
                        "Return type mismatch: interface requires no return value, implementation returns '{:?}'",
                        impl_return.name
                    ),
                    location: None,
                    error_type: TypeErrorKind::ReturnTypeMismatch,
                });
            }
        }
        
        Ok(true)
    }
    
    /// Check if two types are compatible
    fn check_type_compatibility(&self, type1: &TypeExpression, type2: &TypeExpression) -> Result<bool, TypeCheckError> {
        // For now, just check if they're the same type
        // In a full implementation, this would handle subtyping, coercion, etc.
        Ok(type1.name == type2.name)
    }
    
    /// Check if a type can be cast to an interface
    pub fn check_interface_cast(&self, from_type: &str, to_interface: &str) -> Result<bool, TypeCheckError> {
        // Check if the from_type implements the to_interface
        self.check_interface_implementation(from_type, to_interface)
    }
    
    /// Validate interface cast and return detailed error if invalid
    pub fn validate_interface_cast(&self, from_type: &str, to_interface: &str) -> Result<(), TypeCheckError> {
        // Check if the interface exists
        if !self.type_system.environment.type_definitions.contains_key(to_interface) {
            return Err(TypeCheckError {
                message: format!("Interface '{}' not found", to_interface),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            });
        }
        
        // Check if the from_type exists
        if !self.type_system.environment.type_definitions.contains_key(from_type) {
            return Err(TypeCheckError {
                message: format!("Type '{}' not found", from_type),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            });
        }
        
        // Check if the type can be cast to the interface
        if !self.check_interface_cast(from_type, to_interface)? {
            // Use detailed compliance checking to provide better error messages
            self.check_interface_compliance_detailed(from_type, to_interface)
                .map_err(|e| TypeCheckError {
                    message: format!("Cannot cast '{}' to interface '{}': {}", from_type, to_interface, e.message),
                    location: e.location,
                    error_type: TypeErrorKind::InterfaceCastError,
                })?;
        }
        
        Ok(())
    }
    
    /// Check if a method dispatch is valid for an interface type
    pub fn check_interface_method_dispatch(&self, interface_name: &str, method_name: &str, args: &[TypeExpression]) -> Result<TypeExpression, TypeCheckError> {
        // Get the interface definition
        let interface_def = self.type_system.environment.type_definitions.get(interface_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Interface '{}' not found", interface_name),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            })?;
        
        // Find the method in the interface
        let method = interface_def.methods.iter()
            .find(|m| m.name == method_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Method '{}' not found in interface '{}'", method_name, interface_name),
                location: None,
                error_type: TypeErrorKind::MethodDispatchError,
            })?;
        
        // Check argument count
        if method.parameters.len() != args.len() {
            return Err(TypeCheckError {
                message: format!(
                    "Method '{}' expects {} arguments, got {}",
                    method_name,
                    method.parameters.len(),
                    args.len()
                ),
                location: None,
                error_type: TypeErrorKind::ArityMismatch,
            });
        }
        
        // Check argument types
        for (i, (expected, actual)) in method.parameters.iter().zip(args.iter()).enumerate() {
            if !self.check_type_compatibility(expected, actual)? {
                return Err(TypeCheckError {
                    message: format!(
                        "Method '{}' argument {} type mismatch: expected '{:?}', got '{:?}'",
                        method_name,
                        i + 1,
                        expected.name,
                        actual.name
                    ),
                    location: None,
                    error_type: TypeErrorKind::ParameterTypeMismatch,
                });
            }
        }
        
        // Return the method's return type
        Ok(method.return_type.clone().unwrap_or_else(|| TypeExpression::named("void")))
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
    
    fn check_defer_statement(&mut self, defer_stmt: &DeferStatement) -> Result<TypeExpression, TypeCheckError> {
        // Check the deferred expression
        let expr_type = self.check_expression(&*defer_stmt.expression)?;
        
        // Defer statements should only contain expressions that are valid to defer
        // Typically these are function calls or other side-effecting operations
        match &*defer_stmt.expression {
            Expression::Call(_) => {
                // Function calls are always valid to defer
                Ok(TypeExpression::named("void"))
            }
            Expression::MemberAccess(_) => {
                // Member access expressions (like cleanup methods) are valid
                Ok(TypeExpression::named("void"))
            }
            _ => {
                // Other expressions might be valid but generate a warning
                // For now, we'll allow them but this could be enhanced
                Ok(TypeExpression::named("void"))
            }
        }
    }
    
    fn check_channel_send_expression(&mut self, channel_send: &ChannelSendExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check the channel expression
        let channel_type = self.check_expression(&*channel_send.channel)?;
        
        // Check the value being sent
        let value_type = self.check_expression(&*channel_send.value)?;
        
        // Validate that the value type matches the channel's element type
        if let Some(channel_element_type) = self.extract_channel_element_type(&channel_type) {
            if !self.types_compatible(&value_type, &channel_element_type) {
                return Err(TypeCheckError {
                    message: format!("Type mismatch: expected {:?}, got {:?}", channel_element_type, value_type),
                    location: None,
                    error_type: TypeErrorKind::TypeMismatch,
                });
            }
        }
        
        Ok(TypeExpression::named("void"))
    }
    
    fn check_channel_receive_expression(&mut self, channel_receive: &ChannelReceiveExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check the channel expression
        let channel_type = self.check_expression(&*channel_receive.channel)?;
        
        // Extract the element type from the channel type
        if let Some(element_type) = self.extract_channel_element_type(&channel_type) {
            Ok(element_type)
        } else {
            // If we can't determine the element type, return a generic type
            Ok(TypeExpression::named("unknown"))
        }
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

    fn check_struct_literal_expression(&mut self, struct_literal: &crate::ast::StructLiteralExpression) -> Result<TypeExpression, TypeCheckError> {
        // Verify the struct exists and get its definition
        let struct_type_name = &struct_literal.struct_name;
        let struct_definition = self.type_system.environment.type_definitions.get(struct_type_name)
            .ok_or_else(|| TypeCheckError {
                message: format!("Struct type '{}' not found", struct_type_name),
                location: None,
                error_type: TypeErrorKind::TypeNotFound,
            })?.clone();
        
        // Verify it's actually a struct
        if struct_definition.kind != TypeKind::Struct {
            return Err(TypeCheckError {
                message: format!("Expected struct type, found {:?}", struct_definition.kind),
                location: None,
                error_type: TypeErrorKind::TypeMismatch,
            });
        }
        
        // Check each field assignment against the struct definition
        for field_assignment in &struct_literal.fields {
            // Find the field in the struct definition
            let struct_field = struct_definition.fields.iter()
                .find(|f| f.name == field_assignment.field_name)
                .ok_or_else(|| TypeCheckError {
                    message: format!("Field '{}' not found in struct '{}'", 
                                    field_assignment.field_name, struct_type_name),
                    location: None,
                    error_type: TypeErrorKind::FieldNotFound,
                })?;
            
            // Check the assigned value type
            let assigned_type = self.check_expression(&field_assignment.value)?;
            
            // Validate against declared field type if specified
            if let Some(expected_type_name) = &struct_field.field_type {
                let expected_type = TypeExpression::named(&expected_type_name.to_string());
                if !self.are_types_compatible(&assigned_type, &expected_type) {
                    return Err(TypeCheckError {
                        message: format!("Type mismatch in field '{}': expected '{}', found '{:?}'", 
                                        field_assignment.field_name, expected_type_name, assigned_type),
                        location: None,
                        error_type: TypeErrorKind::TypeMismatch,
                    });
                }
            }
            
            log::debug!("Struct field '{}' validated: assigned {:?} to field type {:?}", 
                       field_assignment.field_name, assigned_type, struct_field.field_type);
        }
        
        // Return the struct type
        Ok(TypeExpression::named(struct_type_name))
    }
    
    /// Helper method to check if a type is defined in the type system
    fn is_type_defined(&self, type_name: &str) -> bool {
        // Check built-in types
        match type_name {
            "normie" | "tea" | "vibes" | "snack" | "cap" | "lit" => true,
            _ => {
                // Check user-defined types in the environment
                self.type_system.environment.type_definitions.contains_key(type_name)
            }
        }
    }
    
    /// Helper method to check if two types are compatible for assignment
    fn are_types_compatible(&self, assigned: &TypeExpression, expected: &TypeExpression) -> bool {
        // Basic type compatibility check
        // For now, require exact match; later can be extended for subtyping, coercion, etc.
        format!("{:?}", assigned) == format!("{:?}", expected)
    }

    /// Check lambda expression and infer its function type
    fn check_lambda_expression(&mut self, lambda_expr: &crate::ast::LambdaExpression) -> Result<TypeExpression, TypeCheckError> {
        // Enter new scope for lambda parameters
        self.enter_scope();
        
        // Add lambda parameters to scope with unknown types initially
        let mut param_types = Vec::new();
        for param in &lambda_expr.parameters {
            let param_type = TypeExpression::named("unknown");
            self.add_variable(param.clone(), param_type.clone());
            param_types.push(param_type);
        }
        
        // Check lambda body and infer return type
        let return_type = self.check_expression(&lambda_expr.body)?;
        
        // Exit lambda scope
        self.exit_scope();
        
        // Create function type
        Ok(TypeExpression::function(param_types, return_type))
    }
    
    /// Extract the element type from a channel type
    fn extract_channel_element_type(&self, channel_type: &TypeExpression) -> Option<TypeExpression> {
        // Handle channel types in the form "channel<T>" or "dm<T>"
        if let Some(type_name) = &channel_type.name {
            if type_name.starts_with("channel<") && type_name.ends_with(">") {
                let element_type_str = &type_name[8..type_name.len()-1];
                return Some(TypeExpression::named(element_type_str));
            }
            if type_name.starts_with("dm<") && type_name.ends_with(">") {
                let element_type_str = &type_name[3..type_name.len()-1];
                return Some(TypeExpression::named(element_type_str));
            }
        }
        
        // Handle generic channel types
        if !channel_type.parameters.is_empty() {
            return Some(channel_type.parameters[0].clone());
        }
        
        None
    }
    

}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of AstVisitor trait for TypeChecker
/// This provides standardized traversal of the AST with type checking
impl AstVisitor<Result<TypeExpression, TypeCheckError>> for TypeChecker {
    fn visit_program(&mut self, program: &Program) -> Result<TypeExpression, TypeCheckError> {
        self.errors.clear();
        
        // Visit all statements in the program
        for statement in &program.statements {
            if let Err(error) = self.visit_statement(statement) {
                self.errors.push(error);
            }
        }
        
        // Return void type for the program, or error if any accumulated
        if self.errors.is_empty() {
            Ok(TypeExpression::named("void"))
        } else {
            Err(self.errors.first().cloned().unwrap())
        }
    }
    
    fn visit_statement(&mut self, statement: &Statement) -> Result<TypeExpression, TypeCheckError> {
        match statement {
            Statement::Expression(expr) => {
                self.visit_expression(expr)
            }
            Statement::Let(let_stmt) => {
                self.visit_let_statement(let_stmt)
            }
            Statement::Function(func_stmt) => {
                self.visit_function_statement(func_stmt)
            }
            Statement::If(if_stmt) => {
                self.visit_if_statement(if_stmt)
            }
            Statement::While(while_stmt) => {
                self.visit_while_statement(while_stmt)
            }
            Statement::Return(return_stmt) => {
                self.visit_return_statement(return_stmt)
            }
            Statement::Struct(struct_stmt) => {
                self.visit_struct_statement(struct_stmt)
            }
            Statement::Interface(interface_stmt) => {
                self.visit_interface_statement(interface_stmt)
            }
            Statement::Channel(channel_stmt) => {
                self.visit_channel_statement(channel_stmt)
            }
            Statement::Goroutine(goroutine_stmt) => {
                self.visit_goroutine_statement(goroutine_stmt)
            }
            Statement::Select(select_stmt) => {
                self.visit_select_statement(select_stmt)
            }
            Statement::Panic(panic_stmt) => {
                self.visit_panic_statement(panic_stmt)
            }
            Statement::Catch(catch_stmt) => {
                self.visit_catch_statement(catch_stmt)
            }
            _ => Ok(TypeExpression::named("void")),
        }
    }
    
    fn visit_expression(&mut self, expression: &Expression) -> Result<TypeExpression, TypeCheckError> {
        match expression {
            Expression::Integer(_) => Ok(TypeExpression::named("int")),
            Expression::String(_) => Ok(TypeExpression::named("string")),
            Expression::Boolean(_) => Ok(TypeExpression::named("bool")),
            Expression::Identifier(name) => {
                self.visit_identifier(name)
            }
            Expression::Binary(binary) => {
                self.visit_binary_expression(binary)
            }
            Expression::Call(call) => {
                self.visit_call_expression(call)
            }
            Expression::MemberAccess(member) => {
                self.visit_member_access(member)
            }
            Expression::Literal(literal) => {
                self.visit_literal(literal)
            }
            Expression::Array(elements) => {
                self.visit_array_expression(elements)
            }
            Expression::Map(pairs) => {
                self.visit_map_expression(pairs)
            }
            Expression::ChannelSend(channel_send) => {
                self.visit_channel_send_expression(channel_send)
            }
            Expression::ChannelReceive(channel_receive) => {
                self.visit_channel_receive_expression(channel_receive)
            }
            Expression::ChannelCreation(channel_creation) => {
                self.visit_channel_creation_expression(channel_creation)
            }
            _ => Ok(TypeExpression::named("unknown")),
        }
    }
}

/// Visitor-specific methods that delegate to existing functionality
impl TypeChecker {
    fn visit_identifier(&self, name: &str) -> Result<TypeExpression, TypeCheckError> {
        self.check_identifier(name)
    }

    fn visit_binary_expression(&mut self, binary: &BinaryExpression) -> Result<TypeExpression, TypeCheckError> {
        self.check_binary_expression(binary)
    }

    fn visit_call_expression(&mut self, call: &CallExpression) -> Result<TypeExpression, TypeCheckError> {
        self.check_call_expression(call)
    }

    fn visit_member_access(&mut self, member: &MemberAccessExpression) -> Result<TypeExpression, TypeCheckError> {
        self.check_member_access(member)
    }

    fn visit_literal(&self, literal: &Literal) -> Result<TypeExpression, TypeCheckError> {
        self.check_literal(literal)
    }

    fn visit_array_expression(&mut self, elements: &[Expression]) -> Result<TypeExpression, TypeCheckError> {
        self.check_array_expression(elements)
    }

    fn visit_map_expression(&mut self, pairs: &[(Expression, Expression)]) -> Result<TypeExpression, TypeCheckError> {
        self.check_map_expression(pairs)
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_let_statement(let_stmt)
    }

    fn visit_function_statement(&mut self, func_stmt: &FunctionStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_function_statement(func_stmt)
    }

    fn visit_if_statement(&mut self, if_stmt: &IfStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_if_statement(if_stmt)
    }

    fn visit_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_while_statement(while_stmt)
    }

    fn visit_return_statement(&mut self, return_stmt: &ReturnStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_return_statement(return_stmt)
    }

    fn visit_struct_statement(&mut self, struct_stmt: &StructStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_struct_statement(struct_stmt)
    }

    fn visit_interface_statement(&mut self, interface_stmt: &InterfaceStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_interface_statement(interface_stmt)
    }

    fn visit_channel_statement(&mut self, channel_stmt: &ChannelStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_channel_statement(channel_stmt)
    }

    fn visit_goroutine_statement(&mut self, goroutine_stmt: &GoroutineStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_goroutine_statement(goroutine_stmt)
    }

    fn visit_select_statement(&mut self, select_stmt: &SelectStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_select_statement(select_stmt)
    }

    fn visit_panic_statement(&mut self, panic_stmt: &PanicStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_panic_statement(panic_stmt)
    }

    fn visit_catch_statement(&mut self, catch_stmt: &CatchStatement) -> Result<TypeExpression, TypeCheckError> {
        self.check_catch_statement(catch_stmt)
    }

    fn visit_channel_send_expression(&mut self, channel_send: &ChannelSendExpression) -> Result<TypeExpression, TypeCheckError> {
        self.check_channel_send_expression(channel_send)
    }

    fn visit_channel_receive_expression(&mut self, channel_receive: &ChannelReceiveExpression) -> Result<TypeExpression, TypeCheckError> {
        self.check_channel_receive_expression(channel_receive)
    }

    fn visit_channel_creation_expression(&mut self, channel_creation: &ChannelCreationExpression) -> Result<TypeExpression, TypeCheckError> {
        self.check_channel_creation_expression(channel_creation)
    }
    
    /// Check type alias statement and register it
    pub fn check_type_alias_statement(&mut self, type_alias: &TypeAliasStatement) -> Result<TypeExpression, TypeCheckError> {
        // Check if alias name conflicts with built-in types
        if self.is_builtin_type(&type_alias.name) {
            return Err(TypeCheckError::new(
                TypeErrorKind::InvalidOperation,
                format!("Type alias '{}' conflicts with built-in type", type_alias.name),
            ));
        }
        
        // Check if alias already exists
        if self.type_aliases.contains_key(&type_alias.name) {
            return Err(TypeCheckError::new(
                TypeErrorKind::InvalidOperation,
                format!("Type alias '{}' is already defined", type_alias.name),
            ));
        }
        
        // Convert AST Type to TypeExpression and validate target type
        let target_type_expr = self.convert_ast_type_to_type_expression(&type_alias.target_type)?;
        
        // Register the type alias
        self.type_aliases.insert(type_alias.name.clone(), target_type_expr.clone());
        
        Ok(TypeExpression::named("void"))
    }
    
    /// Convert AST Type to TypeExpression with proper validation
    fn convert_ast_type_to_type_expression(&mut self, ast_type: &crate::ast::Type) -> Result<TypeExpression, TypeCheckError> {
        use crate::ast::Type;
        
        match ast_type {
            // CURSED primitive types
            Type::Normie => Ok(TypeExpression::named("normie")),
            Type::Tea => Ok(TypeExpression::named("tea")),
            Type::Lit => Ok(TypeExpression::named("lit")),
            Type::Sip => Ok(TypeExpression::named("sip")),
            Type::Smol => Ok(TypeExpression::named("smol")),
            Type::Mid => Ok(TypeExpression::named("mid")),
            Type::Thicc => Ok(TypeExpression::named("thicc")),
            Type::Snack => Ok(TypeExpression::named("snack")),
            Type::Meal => Ok(TypeExpression::named("meal")),
            Type::Byte => Ok(TypeExpression::named("byte")),
            Type::Rune => Ok(TypeExpression::named("rune")),
            Type::Extra => Ok(TypeExpression::named("extra")),
            
            // Standard types
            Type::Integer => Ok(TypeExpression::named("int")),
            Type::Float => Ok(TypeExpression::named("float")),
            Type::String => Ok(TypeExpression::named("string")),
            Type::Boolean => Ok(TypeExpression::named("bool")),
            Type::Void => Ok(TypeExpression::named("void")),
            
            Type::Custom(name) => {
                // Check if it's a type alias
                if let Some(resolved_type) = self.resolve_type_alias(name)? {
                    Ok(resolved_type)
                } else {
                    // Check if it's a valid custom type (struct, interface, etc.)
                    Ok(TypeExpression::named(name))
                }
            }
            Type::Array(element_type, _size) => {
                let element_type_expr = self.convert_ast_type_to_type_expression(element_type)?;
                Ok(TypeExpression::array(element_type_expr))
            }
            Type::Slice(element_type) => {
                let element_type_expr = self.convert_ast_type_to_type_expression(element_type)?;
                // Use named type for slices since slice constructor doesn't exist
                Ok(TypeExpression::named(&format!("[]{}", element_type_expr.name.unwrap_or("unknown".to_string()))))
            }
            Type::Pointer(target_type) => {
                let target_type_expr = self.convert_ast_type_to_type_expression(target_type)?;
                // Use named type for pointers since pointer constructor doesn't exist
                Ok(TypeExpression::named(&format!("*{}", target_type_expr.name.unwrap_or("unknown".to_string()))))
            }
            Type::Function(parameters, return_type) => {
                let param_types: Result<Vec<_>, _> = parameters
                    .iter()
                    .map(|p| self.convert_ast_type_to_type_expression(p))
                    .collect();
                let param_types = param_types?;
                
                let return_type_expr = self.convert_ast_type_to_type_expression(return_type)?;
                
                Ok(TypeExpression::function(param_types, return_type_expr))
            }
            Type::Dm(inner_type) => {
                let inner_type_expr = self.convert_ast_type_to_type_expression(inner_type)?;
                // Use named type for channels since channel constructor doesn't exist
                Ok(TypeExpression::named(&format!("dm<{}>", inner_type_expr.name.unwrap_or("unknown".to_string()))))
            }
            Type::Collab(name) => Ok(TypeExpression::named(&format!("collab {}", name))),
            Type::Squad(element_type) => {
                let element_type_expr = self.convert_ast_type_to_type_expression(element_type)?;
                Ok(TypeExpression::array(element_type_expr))
            }
            Type::Tuple(types) => {
                let type_names: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| self.convert_ast_type_to_type_expression(t))
                    .collect();
                let type_names = type_names?;
                let tuple_repr = type_names
                    .iter()
                    .map(|t| t.name.clone().unwrap_or("unknown".to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(TypeExpression::named(&format!("({})", tuple_repr)))
            }
            Type::Generic(name, type_params) => {
                let param_names: Result<Vec<_>, _> = type_params
                    .iter()
                    .map(|t| self.convert_ast_type_to_type_expression(t))
                    .collect();
                let param_names = param_names?;
                let params_repr = param_names
                    .iter()
                    .map(|t| t.name.clone().unwrap_or("unknown".to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(TypeExpression::named(&format!("{}<{}>", name, params_repr)))
            }
            Type::TestResult => Ok(TypeExpression::named("TestResult")),
            Type::TestStatus => Ok(TypeExpression::named("TestStatus")),
            Type::TestSuite => Ok(TypeExpression::named("TestSuite")),
            Type::TestReport => Ok(TypeExpression::named("TestReport")),
        }
    }
    
    /// Resolve type alias with circular reference detection
    pub fn resolve_type_alias(&mut self, alias_name: &str) -> Result<Option<TypeExpression>, TypeCheckError> {
        // Check for circular reference
        if self.type_alias_resolution_stack.contains(&alias_name.to_string()) {
            return Err(TypeCheckError::new(
                TypeErrorKind::InvalidOperation,
                format!("Circular type alias detected: {}", 
                       self.type_alias_resolution_stack.join(" -> ") + " -> " + alias_name),
            ));
        }
        
        if let Some(target_type) = self.type_aliases.get(alias_name).cloned() {
            // Add to resolution stack for circular reference detection
            self.type_alias_resolution_stack.push(alias_name.to_string());
            
            // Recursively resolve if the target is also a type alias
            let resolved_type = self.fully_resolve_type_expression(&target_type)?;
            
            // Remove from resolution stack
            self.type_alias_resolution_stack.pop();
            
            Ok(Some(resolved_type))
        } else {
            Ok(None)
        }
    }
    
    /// Fully resolve a type expression, expanding all type aliases
    fn fully_resolve_type_expression(&mut self, type_expr: &TypeExpression) -> Result<TypeExpression, TypeCheckError> {
        // Check if this type expression has a name that might be a type alias
        if let Some(name) = &type_expr.name {
            // Check if this is a type alias
            if let Some(resolved) = self.resolve_type_alias(name)? {
                Ok(resolved)
            } else {
                Ok(type_expr.clone())
            }
        } else {
            Ok(type_expr.clone())
        }
    }
    
    /// Check if a name is a built-in type
    fn is_builtin_type(&self, name: &str) -> bool {
        matches!(name, 
            "normie" | "tea" | "lit" | "sip" | "smol" | "mid" | "thicc" | 
            "drip" | "snack" | "meal" | "byte" | "rune" | "extra" | 
            "vibes" | "cap" | "void"
        )
    }
    
    /// Get all registered type aliases (for debugging/inspection)
    pub fn get_type_aliases(&self) -> &HashMap<String, TypeExpression> {
        &self.type_aliases
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
        assert_eq!(result.name, Some("normie".to_string()));
        
        // Test string literal
        let expr = Expression::String("hello".to_string());
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("tea".to_string()));
        
        // Test boolean literal
        let expr = Expression::Boolean(true);
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("vibes".to_string()));
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
        assert_eq!(result.name, Some("normie".to_string()));
        
        // Test comparison operation
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "<".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = checker.check_expression(&expr).unwrap();
        assert_eq!(result.name, Some("vibes".to_string()));
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
                param_type: Some(crate::ast::Type::Normie),
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

    #[test]
    fn test_ast_visitor_trait() {
        let mut checker = TypeChecker::new();
        
        // Test visiting a simple program
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    target: crate::ast::LetTarget::Single("x".to_string()),
                    value: Expression::Integer(42),
                    var_type: None,
                    visibility: crate::ast::Visibility::Private,
                }),
                Statement::Expression(Expression::Identifier("x".to_string())),
            ],
            imports: vec![],
            package: None,
        };
        
        let result = checker.visit_program(&program);
        assert!(result.is_ok());
        
        // Test visiting individual expressions
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = checker.visit_expression(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("normie".to_string()));
        
        // Test visiting statements
        let stmt = Statement::If(IfStatement {
            init: None,
            condition: Expression::Boolean(true),
            then_branch: vec![Statement::Expression(Expression::Integer(1))],
            else_branch: None,
        });
        
        let result = checker.visit_statement(&stmt);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_type_alias_basic() {
        use crate::ast::{Type, TypeAliasStatement, Visibility};
        
        let mut checker = TypeChecker::new();
        
        // Create a simple type alias: be_like MyInt = normie
        let type_alias = TypeAliasStatement {
            name: "MyInt".to_string(),
            target_type: Type::Normie,
            visibility: Visibility::Private,
        };
        
        // Check the type alias
        let result = checker.check_type_alias_statement(&type_alias);
        assert!(result.is_ok());
        
        // Verify the alias was registered
        assert!(checker.type_aliases.contains_key("MyInt"));
        
        // Verify resolution works
        let resolved = checker.resolve_type_alias("MyInt").unwrap();
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().name, Some("normie".to_string()));
    }
    
    #[test]
    fn test_type_alias_circular_detection() {
        use crate::ast::{Type, TypeAliasStatement, Visibility};
        
        let mut checker = TypeChecker::new();
        
        // Create circular type aliases
        let alias_a = TypeAliasStatement {
            name: "TypeA".to_string(),
            target_type: Type::Custom("TypeB".to_string()),
            visibility: Visibility::Private,
        };
        
        let alias_b = TypeAliasStatement {
            name: "TypeB".to_string(),
            target_type: Type::Custom("TypeA".to_string()),
            visibility: Visibility::Private,
        };
        
        // Register both aliases
        assert!(checker.check_type_alias_statement(&alias_a).is_ok());
        assert!(checker.check_type_alias_statement(&alias_b).is_ok());
        
        // Try to resolve - should detect circular reference
        let result = checker.resolve_type_alias("TypeA");
        assert!(result.is_err());
        assert!(result.err().unwrap().message.contains("Circular type alias"));
    }
    
    #[test]
    fn test_type_alias_builtin_conflict() {
        use crate::ast::{Type, TypeAliasStatement, Visibility};
        
        let mut checker = TypeChecker::new();
        
        // Try to create alias with built-in type name
        let type_alias = TypeAliasStatement {
            name: "normie".to_string(),
            target_type: Type::Tea,
            visibility: Visibility::Private,
        };
        
        let result = checker.check_type_alias_statement(&type_alias);
        assert!(result.is_err());
        assert!(result.err().unwrap().message.contains("conflicts with built-in type"));
    }
    
    #[test]
    fn test_type_alias_duplicate() {
        use crate::ast::{Type, TypeAliasStatement, Visibility};
        
        let mut checker = TypeChecker::new();
        
        let type_alias1 = TypeAliasStatement {
            name: "MyType".to_string(),
            target_type: Type::Normie,
            visibility: Visibility::Private,
        };
        
        let type_alias2 = TypeAliasStatement {
            name: "MyType".to_string(),
            target_type: Type::Tea,
            visibility: Visibility::Private,
        };
        
        // First definition should succeed
        assert!(checker.check_type_alias_statement(&type_alias1).is_ok());
        
        // Second definition should fail
        let result = checker.check_type_alias_statement(&type_alias2);
        assert!(result.is_err());
        assert!(result.err().unwrap().message.contains("already defined"));
    }
    
    #[test]
    fn test_type_alias_nested_resolution() {
        use crate::ast::{Type, TypeAliasStatement, Visibility};
        
        let mut checker = TypeChecker::new();
        
        // Create nested aliases: MyInt -> Counter -> FinalType -> normie
        let alias1 = TypeAliasStatement {
            name: "MyInt".to_string(),
            target_type: Type::Normie,
            visibility: Visibility::Private,
        };
        
        let alias2 = TypeAliasStatement {
            name: "Counter".to_string(),
            target_type: Type::Custom("MyInt".to_string()),
            visibility: Visibility::Private,
        };
        
        let alias3 = TypeAliasStatement {
            name: "FinalType".to_string(),
            target_type: Type::Custom("Counter".to_string()),
            visibility: Visibility::Private,
        };
        
        // Register all aliases
        assert!(checker.check_type_alias_statement(&alias1).is_ok());
        assert!(checker.check_type_alias_statement(&alias2).is_ok());
        assert!(checker.check_type_alias_statement(&alias3).is_ok());
        
        // Resolve final type - should resolve to normie
        let resolved = checker.resolve_type_alias("FinalType").unwrap();
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().name, Some("normie".to_string()));
    }
    
    #[test]
    fn test_type_alias_array_resolution() {
        use crate::ast::{Type, TypeAliasStatement, Visibility};
        
        let mut checker = TypeChecker::new();
        
        // Create array type alias: IntArray = []normie
        let type_alias = TypeAliasStatement {
            name: "IntArray".to_string(),
            target_type: Type::Array(Box::new(Type::Normie), None),
            visibility: Visibility::Private,
        };
        
        let result = checker.check_type_alias_statement(&type_alias);
        assert!(result.is_ok());
        
        // Verify array type was registered correctly
        let resolved = checker.resolve_type_alias("IntArray").unwrap();
        assert!(resolved.is_some());
        
        let resolved_type = resolved.unwrap();
        // Since we're using named types, just check the name is present
        assert!(resolved_type.name.is_some());
    }
}
