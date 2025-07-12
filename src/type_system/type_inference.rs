//! Type inference engine for CURSED

use crate::error::CursedError;
use crate::core::Type;
use crate::ast::{Expression, Statement, Program};
use super::{TypeExpression, TypeSubstitution, ConstraintResolver};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeInference {
    type_constraints: Vec<TypeConstraint>,
    type_variables: HashMap<String, Type>,
    substitutions: HashMap<String, Type>,
    next_var_id: usize,
    inference_context: InferenceState,
}

#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub left: Type,
    pub right: Type,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct InferenceState {
    pub type_vars: HashMap<String, TypeExpression>,
    pub constraints: Vec<UnificationConstraint>,
    pub substitutions: TypeSubstitution,
}

#[derive(Debug, Clone)]
pub struct UnificationConstraint {
    pub left: TypeExpression,
    pub right: TypeExpression,
    pub source: String,
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            type_constraints: Vec::new(),
            type_variables: HashMap::new(),
            substitutions: HashMap::new(),
            next_var_id: 0,
            inference_context: InferenceState {
                type_vars: HashMap::new(),
                constraints: Vec::new(),
                substitutions: TypeSubstitution::new(),
            },
        }
    }

    pub fn add_constraint(&mut self, left: Type, right: Type, location: String) {
        self.type_constraints.push(TypeConstraint { left, right, location });
    }

    /// Infer types for expressions with advanced constraint solving
    pub fn infer_expression_type(&mut self, expr: &Expression) -> Result<TypeExpression, CursedError> {
        match expr {
            Expression::Integer(_) => Ok(TypeExpression::named("normie")),
            Expression::String(_) => Ok(TypeExpression::named("tea")),
            Expression::Boolean(_) => Ok(TypeExpression::named("lit")),
            Expression::Identifier(name) => {
                // Look up or create fresh type variable
                if let Some(type_expr) = self.inference_context.type_vars.get(name) {
                    Ok(type_expr.clone())
                } else {
                    let fresh_var = self.fresh_type_variable();
                    self.inference_context.type_vars.insert(name.clone(), fresh_var.clone());
                    Ok(fresh_var)
                }
            }
            Expression::Binary(binary) => {
                let left_type = self.infer_expression_type(&binary.left)?;
                let right_type = self.infer_expression_type(&binary.right)?;
                
                match binary.operator.as_str() {
                    "+" | "-" | "*" | "/" => {
                        // For simple arithmetic with concrete types, return concrete type
                        if left_type.name == Some("normie".to_string()) && right_type.name == Some("normie".to_string()) {
                            Ok(TypeExpression::named("normie"))
                        } else {
                            // Create constraints for complex cases
                            let result_type = self.fresh_type_variable();
                            self.add_unification_constraint(left_type.clone(), TypeExpression::named("normie"), "arithmetic left operand".to_string());
                            self.add_unification_constraint(right_type.clone(), TypeExpression::named("normie"), "arithmetic right operand".to_string());
                            self.add_unification_constraint(result_type.clone(), TypeExpression::named("normie"), "arithmetic result".to_string());
                            Ok(result_type)
                        }
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        // For simple comparisons with concrete types, return concrete type
                        if left_type.name.is_some() && right_type.name.is_some() {
                            Ok(TypeExpression::named("lit"))
                        } else {
                            // Create constraints for complex cases
                            let result_type = self.fresh_type_variable();
                            self.add_unification_constraint(left_type, right_type, "comparison operands".to_string());
                            self.add_unification_constraint(result_type.clone(), TypeExpression::named("lit"), "comparison result".to_string());
                            Ok(result_type)
                        }
                    }
                    "&&" | "||" => {
                        // For simple logical with concrete types, return concrete type
                        if left_type.name == Some("lit".to_string()) && right_type.name == Some("lit".to_string()) {
                            Ok(TypeExpression::named("lit"))
                        } else {
                            // Create constraints for complex cases
                            let result_type = self.fresh_type_variable();
                            self.add_unification_constraint(left_type, TypeExpression::named("lit"), "logical left operand".to_string());
                            self.add_unification_constraint(right_type, TypeExpression::named("lit"), "logical right operand".to_string());
                            self.add_unification_constraint(result_type.clone(), TypeExpression::named("lit"), "logical result".to_string());
                            Ok(result_type)
                        }
                    }
                    _ => return Err(CursedError::type_error(&format!("Unknown binary operator: {}", binary.operator))),
                }
            }
            Expression::Array(elements) => {
                if elements.is_empty() {
                    // Empty array - create fresh type variable for element type
                    let element_type = self.fresh_type_variable();
                    Ok(TypeExpression::array(element_type))
                } else {
                    // Infer type from first element and unify with others
                    let first_type = self.infer_expression_type(&elements[0])?;
                    
                    for (i, element) in elements.iter().enumerate().skip(1) {
                        let element_type = self.infer_expression_type(element)?;
                        self.add_unification_constraint(
                            first_type.clone(), 
                            element_type, 
                            format!("array element {}", i)
                        );
                    }
                    
                    Ok(TypeExpression::array(first_type))
                }
            }
            Expression::Call(call) => {
                self.infer_call_type(call)
            }
            Expression::ArrayAccess(array_access) => {
                let array_type = self.infer_expression_type(&array_access.array)?;
                let index_type = self.infer_expression_type(&array_access.index)?;
                
                // Ensure index is integer
                self.add_unification_constraint(
                    index_type,
                    TypeExpression::named("normie"),
                    "array index must be integer".to_string()
                );
                
                // Extract element type from array type
                match array_type.name.as_ref().map(|s| s.as_str()) {
                    Some(name) if name.starts_with('[') && name.ends_with(']') => {
                        // Extract element type from [ElementType] syntax
                        let element_type_name = &name[1..name.len()-1];
                        Ok(TypeExpression::named(element_type_name))
                    }
                    _ => {
                        Err(CursedError::TypeError(format!("Cannot index non-array type: {:?}", array_type)))
                    }
                }
            }
            Expression::SliceAccess(slice_access) => {
                let array_type = self.infer_expression_type(&slice_access.array)?;
                
                // Check start index type if present
                if let Some(ref start_expr) = slice_access.start {
                    let start_type = self.infer_expression_type(start_expr)?;
                    self.add_unification_constraint(
                        start_type,
                        TypeExpression::named("normie"),
                        "slice start index must be integer".to_string()
                    );
                }
                
                // Check end index type if present
                if let Some(ref end_expr) = slice_access.end {
                    let end_type = self.infer_expression_type(end_expr)?;
                    self.add_unification_constraint(
                        end_type,
                        TypeExpression::named("normie"),
                        "slice end index must be integer".to_string()
                    );
                }
                
                // Slice of an array returns the same array type (slice)
                match array_type.name.as_ref().map(|s| s.as_str()) {
                    Some(name) if name.starts_with('[') && name.ends_with(']') => {
                        // Return the same array type for slices
                        Ok(array_type)
                    }
                    _ => {
                        Err(CursedError::TypeError(format!("Cannot slice non-array type: {:?}", array_type)))
                    }
                }
            }
            Expression::Variable(name) => {
                // Look up the variable type in the inference context
                match self.inference_context.type_vars.get(name) {
                    Some(var_type) => Ok(var_type.clone()),
                    None => {
                        // For now, we'll assume 'normie' type for undefined variables
                        // A full implementation would report an error
                        log::warn!("Undefined variable: {}. Assuming 'normie' type.", name);
                        Ok(TypeExpression::named("normie"))
                    }
                }
            }
            _ => {
                // For other expressions, create fresh type variables
                Ok(self.fresh_type_variable())
            }
        }
    }
    
    fn infer_call_type(&mut self, call: &crate::ast::CallExpression) -> Result<TypeExpression, CursedError> {
        let func_type = self.infer_expression_type(&call.function)?;
        
        // Create fresh type variables for parameters and return type
        let param_types: Vec<TypeExpression> = call.arguments.iter()
            .map(|_| self.fresh_type_variable())
            .collect();
        
        let return_type = self.fresh_type_variable();
        
        // Infer argument types
        for (i, arg) in call.arguments.iter().enumerate() {
            let arg_type = self.infer_expression_type(arg)?;
            self.add_unification_constraint(
                arg_type, 
                param_types[i].clone(), 
                format!("function argument {}", i)
            );
        }
        
        // Create function type constraint
        let expected_func_type = TypeExpression::function(param_types, return_type.clone());
        self.add_unification_constraint(
            func_type, 
            expected_func_type, 
            "function call".to_string()
        );
        
        Ok(return_type)
    }
    
    /// Infer types for statements
    pub fn infer_statement_types(&mut self, stmt: &Statement) -> Result<(), CursedError> {
        match stmt {
            Statement::Let(let_stmt) => {
                let value_type = self.infer_expression_type(&let_stmt.value)?;
                self.inference_context.type_vars.insert(let_stmt.target.primary_name(), value_type);
                Ok(())
            }
            Statement::Function(func_stmt) => {
                // Create type variables for parameters
                let param_types: Vec<TypeExpression> = func_stmt.parameters.iter()
                    .map(|param| {
                        let param_type = self.fresh_type_variable();
                        self.inference_context.type_vars.insert(param.name.clone(), param_type.clone());
                        param_type
                    })
                    .collect();
                
                // Infer return type from function body
                let return_type = self.infer_function_return_type(&func_stmt.body)?;
                
                // Create function type
                let func_type = TypeExpression::function(param_types, return_type);
                self.inference_context.type_vars.insert(func_stmt.name.clone(), func_type);
                
                Ok(())
            }
            Statement::If(if_stmt) => {
                // Condition must be bool
                let condition_type = self.infer_expression_type(&if_stmt.condition)?;
                self.add_unification_constraint(
                    condition_type, 
                    TypeExpression::named("bool"), 
                    "if condition".to_string()
                );
                
                // Process branches
                for stmt in &if_stmt.then_branch {
                    self.infer_statement_types(stmt)?;
                }
                
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.infer_statement_types(stmt)?;
                    }
                }
                
                Ok(())
            }
            _ => Ok(()),
        }
    }
    
    fn infer_function_return_type(&mut self, body: &[Statement]) -> Result<TypeExpression, CursedError> {
        let mut return_types = Vec::new();
        
        // Collect all return statements
        for stmt in body {
            self.collect_return_types(stmt, &mut return_types)?;
        }
        
        if return_types.is_empty() {
            Ok(TypeExpression::named("void"))
        } else {
            // Unify all return types
            let mut unified_type = return_types[0].clone();
            for return_type in return_types.iter().skip(1) {
                self.add_unification_constraint(
                    unified_type.clone(), 
                    return_type.clone(), 
                    "function return type".to_string()
                );
            }
            Ok(unified_type)
        }
    }
    
    fn collect_return_types(&mut self, stmt: &Statement, return_types: &mut Vec<TypeExpression>) -> Result<(), CursedError> {
        match stmt {
            Statement::Return(return_stmt) => {
                if let Some(value) = &return_stmt.value {
                    let return_type = self.infer_expression_type(value)?;
                    return_types.push(return_type);
                } else {
                    return_types.push(TypeExpression::named("void"));
                }
            }
            Statement::If(if_stmt) => {
                for stmt in &if_stmt.then_branch {
                    self.collect_return_types(stmt, return_types)?;
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.collect_return_types(stmt, return_types)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn infer_types(&mut self) -> Result<HashMap<String, Type>, CursedError> {
        // Solve all unification constraints
        self.solve_constraints()?;
        
        // Convert TypeExpression results back to Type for compatibility
        let mut result = HashMap::new();
        for (name, type_expr) in &self.inference_context.type_vars {
            let resolved_type = self.inference_context.substitutions.apply(type_expr);
            if let Some(type_name) = &resolved_type.name {
                let core_type = match type_name.as_str() {
                    "int" => Type::Int,
                    "float" => Type::Float,
                    "string" => Type::String,
                    "bool" => Type::Bool,
                    "void" => Type::Void,
                    _ => Type::Custom(type_name.clone()),
                };
                result.insert(name.clone(), core_type);
            }
        }
        
        Ok(result)
    }
    
    fn solve_constraints(&mut self) -> Result<(), CursedError> {
        let resolver = ConstraintResolver::new();
        
        // Process each unification constraint
        for constraint in &self.inference_context.constraints.clone() {
            match self.inference_context.substitutions.unify(&constraint.left, &constraint.right) {
                Ok(()) => {
                    // Constraint solved successfully
                }
                Err(e) => {
                    return Err(CursedError::type_error(&format!(
                        "Type unification failed for {}: {}", 
                        constraint.source, e
                    )));
                }
            }
        }
        
        Ok(())
    }
    
    fn add_unification_constraint(&mut self, left: TypeExpression, right: TypeExpression, source: String) {
        self.inference_context.constraints.push(UnificationConstraint {
            left,
            right,
            source,
        });
    }
    
    fn fresh_type_variable(&mut self) -> TypeExpression {
        let var_name = format!("T{}", self.next_var_id);
        self.next_var_id += 1;
        TypeExpression::named(&var_name)
    }

    fn unify(&mut self, left: &Type, right: &Type) -> Result<(), CursedError> {
        match (left, right) {
            (Type::Unknown, t) | (t, Type::Unknown) => {
                // Handle type variables
                Ok(())
            },
            (Type::Int, Type::Int) => Ok(()),
            (Type::Float, Type::Float) => Ok(()),
            (Type::String, Type::String) => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Array(t1), Type::Array(t2)) => {
                self.unify(t1, t2)
            }
            (Type::Function(params1, ret1), Type::Function(params2, ret2)) => {
                if params1.len() != params2.len() {
                    return Err(CursedError::type_error("Function parameter count mismatch"));
                }
                
                for (p1, p2) in params1.iter().zip(params2.iter()) {
                    self.unify(p1, p2)?;
                }
                
                self.unify(ret1, ret2)
            }
            _ => Err(CursedError::type_error(&format!("Cannot unify {:?} with {:?}", left, right))),
        }
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}
