//! Core infrastructure for generics monomorphization
//! 
//! This module implements the missing core infrastructure identified in the analysis:
//! 1. TypeEnvironment integration - generic storage/retrieval functions
//! 2. AST-TypeExpression conversion - critical conversion functions
//! 3. Basic statement type substitution - handling core statement types
//! 4. Error handling for type substitution failures

use crate::error_types::Error as CursedError;
use crate::ast::{Expression, Statement, FunctionDeclaration, StructStatement as StructDeclaration};
use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint, GenericInfo};
use std::collections::HashMap;

/// Core monomorphization infrastructure
pub struct GenericsCore {
    pub type_env: TypeEnvironment,
}

impl GenericsCore {
    pub fn new(type_env: TypeEnvironment) -> Self {
        Self { type_env }
    }

    /// Convert AST Type to TypeExpression - Phase 1 requirement
    pub fn convert_ast_type_to_expression(&self, ast_type: &crate::ast::Type) -> TypeExpression {
        use crate::ast::Type;
        
        match ast_type {
            Type::Normie => TypeExpression::named("normie"),
            Type::Tea => TypeExpression::named("tea"),
            Type::Lit => TypeExpression::named("lit"),
            Type::Sip => TypeExpression::named("sip"),
            Type::Smol => TypeExpression::named("smol"),
            Type::Mid => TypeExpression::named("mid"),
            Type::Thicc => TypeExpression::named("thicc"),
            Type::Snack => TypeExpression::named("snack"),
            Type::Meal => TypeExpression::named("meal"),
            Type::Void => TypeExpression::named("void"),
            Type::Custom(name) => TypeExpression::named(name),
            Type::Array(element_type, _) => {
                TypeExpression::array(self.convert_ast_type_to_expression(element_type))
            }
            Type::Slice(element_type) => {
                TypeExpression::generic("slice", vec![self.convert_ast_type_to_expression(element_type)])
            }
            Type::Function(params, return_type) => {
                let param_types: Vec<TypeExpression> = params.iter()
                    .map(|p| self.convert_ast_type_to_expression(p))
                    .collect();
                TypeExpression::function(param_types, self.convert_ast_type_to_expression(return_type))
            }
            _ => TypeExpression::named("unknown"),
        }
    }

    /// Convert TypeExpression to AST Type - Phase 1 requirement
    pub fn convert_type_expression_to_ast(&self, type_expr: &TypeExpression) -> crate::ast::Type {
        use crate::ast::Type;
        
        if let Some(name) = &type_expr.name {
            match name.as_str() {
                "normie" => Type::Normie,
                "tea" => Type::Tea,
                "lit" => Type::Lit,
                "sip" => Type::Sip,
                "smol" => Type::Smol,
                "mid" => Type::Mid,
                "thicc" => Type::Thicc,
                "snack" => Type::Snack,
                "meal" => Type::Meal,
                "void" => Type::Void,
                "Array" if type_expr.parameters.len() == 1 => {
                    Type::Array(
                        Box::new(self.convert_type_expression_to_ast(&type_expr.parameters[0])),
                        None
                    )
                }
                "slice" if type_expr.parameters.len() == 1 => {
                    Type::Slice(Box::new(self.convert_type_expression_to_ast(&type_expr.parameters[0])))
                }
                _ => Type::Custom(name.clone()),
            }
        } else {
            Type::Custom("unknown".to_string())
        }
    }

    /// Basic type substitution for core statements - Phase 1 requirement
    pub fn substitute_types_in_statement(&self, statement: &Statement, substitutions: &HashMap<String, TypeExpression>) -> Result<Statement, CursedError> {
        match statement {
            Statement::Let(let_stmt) => {
                let mut new_let_stmt = let_stmt.clone();
                if let Some(var_type) = &let_stmt.var_type {
                    let type_expr = self.convert_ast_type_to_expression(var_type);
                    let substituted_type_expr = self.substitute_type_parameters(&type_expr, substitutions)?;
                    new_let_stmt.var_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                }
                new_let_stmt.value = self.substitute_types_in_expression(&let_stmt.value, substitutions)?;
                Ok(Statement::Let(new_let_stmt))
            }
            Statement::Expression(expr) => {
                Ok(Statement::Expression(self.substitute_types_in_expression(expr, substitutions)?))
            }
            Statement::Return(return_stmt) => {
                let mut new_return = return_stmt.clone();
                if let Some(value) = &return_stmt.value {
                    new_return.value = Some(self.substitute_types_in_expression(value, substitutions)?);
                }
                Ok(Statement::Return(new_return))
            }
            Statement::Function(func_stmt) => {
                let mut new_func_stmt = func_stmt.clone();
                // Substitute parameter types
                for param in &mut new_func_stmt.parameters {
                    if let Some(param_type) = &param.param_type {
                        let type_expr = self.convert_ast_type_to_expression(param_type);
                        let substituted_type_expr = self.substitute_type_parameters(&type_expr, substitutions)?;
                        param.param_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                    }
                }
                // Substitute return type
                if let Some(return_type) = &func_stmt.return_type {
                    let type_expr = self.convert_ast_type_to_expression(return_type);
                    let substituted_type_expr = self.substitute_type_parameters(&type_expr, substitutions)?;
                    new_func_stmt.return_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                }
                // Substitute function body
                new_func_stmt.body = self.substitute_types_in_statements(&func_stmt.body, substitutions)?;
                Ok(Statement::Function(new_func_stmt))
            }
            // For other statement types, just clone them for now
            _ => Ok(statement.clone()),
        }
    }

    /// Substitute types in statement list
    pub fn substitute_types_in_statements(&self, statements: &[Statement], substitutions: &HashMap<String, TypeExpression>) -> Result<Vec<Statement>, CursedError> {
        let mut substituted_statements = Vec::new();
        
        for statement in statements {
            substituted_statements.push(self.substitute_types_in_statement(statement, substitutions)?);
        }
        
        Ok(substituted_statements)
    }

    /// Basic expression type substitution - Phase 1 requirement
    pub fn substitute_types_in_expression(&self, expression: &Expression, substitutions: &HashMap<String, TypeExpression>) -> Result<Expression, CursedError> {
        match expression {
            Expression::Call(call_expr) => {
                let mut new_call = call_expr.clone();
                new_call.function = Box::new(self.substitute_types_in_expression(&call_expr.function, substitutions)?);
                
                let mut new_args = Vec::new();
                for arg in &call_expr.arguments {
                    new_args.push(self.substitute_types_in_expression(arg, substitutions)?);
                }
                new_call.arguments = new_args;
                
                Ok(Expression::Call(new_call))
            }
            Expression::Binary(binary_expr) => {
                Ok(Expression::Binary(crate::ast::BinaryExpression {
                    left: Box::new(self.substitute_types_in_expression(&binary_expr.left, substitutions)?),
                    operator: binary_expr.operator.clone(),
                    right: Box::new(self.substitute_types_in_expression(&binary_expr.right, substitutions)?),
                }))
            }
            // For other expressions, just clone them for now
            _ => Ok(expression.clone()),
        }
    }

    /// Substitute type parameters with concrete types - Phase 1 requirement
    pub fn substitute_type_parameters(&self, type_expr: &TypeExpression, substitutions: &HashMap<String, TypeExpression>) -> Result<TypeExpression, CursedError> {
        if let Some(type_name) = &type_expr.name {
            // Check if this is a type parameter that needs substitution
            if let Some(substitution) = substitutions.get(type_name) {
                // Validate that substitution is valid - Phase 1 requirement
                self.validate_type_substitution(type_name, substitution)?;
                return Ok(substitution.clone());
            }
        }
        
        // Recursively substitute in type parameters
        let mut substituted_params = Vec::new();
        for param in &type_expr.parameters {
            match self.substitute_type_parameters(param, substitutions) {
                Ok(substituted_param) => substituted_params.push(substituted_param),
                Err(e) => return Err(CursedError::Type(format!("Failed to substitute type parameter in {}: {}", 
                                     type_expr.name.as_ref().unwrap_or(&"unknown".to_string()), 
                                     e))),
            }
        }
        
        let substituted_return_type = if let Some(return_type) = &type_expr.return_type {
            match self.substitute_type_parameters(return_type, substitutions) {
                Ok(substituted_rt) => Some(Box::new(substituted_rt)),
                Err(e) => return Err(CursedError::Type(format!("Failed to substitute return type in {}: {}", 
                                     type_expr.name.as_ref().unwrap_or(&"unknown".to_string()), 
                                     e))),
            }
        } else {
            None
        };
        
        Ok(TypeExpression {
            kind: type_expr.kind.clone(),
            name: type_expr.name.clone(),
            parameters: substituted_params,
            return_type: substituted_return_type,
        })
    }

    /// Validate that a type substitution is valid - Phase 1 requirement
    pub fn validate_type_substitution(&self, type_param: &str, substitution: &TypeExpression) -> Result<(), CursedError> {
        // Check if substitution creates circular dependencies
        if self.has_circular_type_dependency(type_param, substitution) {
            return Err(CursedError::Type(format!("Circular type dependency detected: {} -> {}", type_param, 
                                 substitution.name.as_ref().unwrap_or(&"unknown".to_string()))));
        }
        
        // Check if substitution is a valid type
        if !self.is_valid_type_expression(substitution) {
            return Err(CursedError::Type(format!("Invalid type substitution: {} cannot be substituted with {}", 
                                 type_param, 
                                 substitution.name.as_ref().unwrap_or(&"unknown".to_string()))));
        }
        
        Ok(())
    }

    /// Check if a type substitution creates circular dependencies
    fn has_circular_type_dependency(&self, type_param: &str, substitution: &TypeExpression) -> bool {
        if let Some(subst_name) = &substitution.name {
            if subst_name == type_param {
                return true;
            }
            
            // Check parameters recursively
            for param in &substitution.parameters {
                if self.has_circular_type_dependency(type_param, param) {
                    return true;
                }
            }
            
            // Check return type recursively
            if let Some(return_type) = &substitution.return_type {
                if self.has_circular_type_dependency(type_param, return_type) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Check if a type expression is valid
    fn is_valid_type_expression(&self, type_expr: &TypeExpression) -> bool {
        // Check if type name is valid
        if let Some(type_name) = &type_expr.name {
            // Check if it's a built-in type
            if self.is_builtin_type(type_name) {
                return true;
            }
            
            // Check if it's a defined type in the type environment
            if self.type_env.get_type(type_name).is_some() {
                return true;
            }
            
            // If type name is present but not built-in and not in type environment, it's invalid
            return false;
        }
        
        // Check parameters recursively
        for param in &type_expr.parameters {
            if !self.is_valid_type_expression(param) {
                return false;
            }
        }
        
        // Check return type recursively
        if let Some(return_type) = &type_expr.return_type {
            if !self.is_valid_type_expression(return_type) {
                return false;
            }
        }
        
        true
    }

    /// Check if a type is a built-in type
    fn is_builtin_type(&self, type_name: &str) -> bool {
        matches!(type_name, "normie" | "tea" | "lit" | "sip" | "smol" | "mid" | "thicc" | "snack" | "meal" | "void")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_system::TypeKind;
    use std::collections::HashMap;

    #[test]
    fn test_ast_type_conversion() {
        let type_env = TypeEnvironment::new();
        let core = GenericsCore::new(type_env);
        
        // Test basic type conversion
        let ast_type = crate::ast::Type::Normie;
        let type_expr = core.convert_ast_type_to_expression(&ast_type);
        assert_eq!(type_expr.name, Some("normie".to_string()));
        
        // Test conversion back
        let converted_back = core.convert_type_expression_to_ast(&type_expr);
        assert!(matches!(converted_back, crate::ast::Type::Normie));
    }

    #[test]
    fn test_type_substitution() {
        let type_env = TypeEnvironment::new();
        let core = GenericsCore::new(type_env);
        
        // Create a type expression with a parameter
        let type_expr = TypeExpression {
            kind: TypeKind::Named("T".to_string()),
            name: Some("T".to_string()),
            parameters: vec![],
            return_type: None,
        };
        
        // Create substitution map
        let mut substitutions = HashMap::new();
        substitutions.insert("T".to_string(), TypeExpression::named("normie"));
        
        // Test substitution
        let result = core.substitute_type_parameters(&type_expr, &substitutions).unwrap();
        assert_eq!(result.name, Some("normie".to_string()));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let type_env = TypeEnvironment::new();
        let core = GenericsCore::new(type_env);
        
        // Create a circular dependency: T -> T
        let type_expr = TypeExpression::named("T");
        let result = core.has_circular_type_dependency("T", &type_expr);
        assert!(result);
    }

    #[test]
    fn test_valid_type_expression() {
        let type_env = TypeEnvironment::new();
        let core = GenericsCore::new(type_env);
        
        // Test built-in type
        let builtin_type = TypeExpression::named("normie");
        assert!(core.is_valid_type_expression(&builtin_type));
        
        // Test unknown type
        let unknown_type = TypeExpression::named("UnknownType");
        assert!(!core.is_valid_type_expression(&unknown_type));
    }
}
