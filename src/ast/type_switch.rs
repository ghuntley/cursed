/// Type switch AST structures for CURSED language
/// Handles `vibe_check variable := expression.(type)` syntax and
/// `vibe_check interface_expr { mood Type var: ... }` syntax

use crate::ast::traits::{Expression, Node, Statement};
use crate::ast::block::BlockStatement;
use std::any::Any;

/// Type switch statement for runtime type checking and variable binding
/// Syntax: vibe_check variable := expression.(type) { mood Type: ... }
#[derive(Debug, Clone)]
pub struct TypeSwitchStatement {
    pub token: String,
    pub expression: Option<Box<dyn Expression>>,
    pub cases: Vec<TypeSwitchCase>,
    pub default_case: Option<BlockStatement>,
    pub variable_name: Option<String>,
}

impl TypeSwitchStatement {
    pub fn new(
        token: String,
        expression: Box<dyn Expression>,
        cases: Vec<TypeSwitchCase>,
        default_case: Option<BlockStatement>,
        variable_name: Option<String>,
    ) -> Self {
        Self {
            token,
            expression: Some(expression),
            cases,
            default_case,
            variable_name,
        }
    }

    pub fn with_variable(
        expression: Box<dyn Expression>,
        variable_name: String,
        cases: Vec<TypeSwitchCase>,
    ) -> Self {
        Self {
            token: "vibe_check".to_string(),
            expression: Some(expression),
            cases,
            default_case: None,
            variable_name: Some(variable_name),
        }
    }
}

impl Node for TypeSwitchStatement {
    fn string(&self) -> String {
        let mut result = String::from("vibe_check");
        
        if let Some(var_name) = &self.variable_name {
            if let Some(expr) = &self.expression {
                result.push_str(&format!(" {} := {}.(type)", var_name, expr.string()));
            }
        } else if let Some(expr) = &self.expression {
            result.push_str(&format!(" {}.(type)", expr.string()));
        }
        
        result.push_str(" {\n");
        
        for case in &self.cases {
            result.push_str(&format!("  {}\n", case.string()));
        }
        
        if let Some(default) = &self.default_case {
            result.push_str(&format!("  basic: {}\n", default.string()));
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TypeSwitchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(TypeSwitchStatement {
            token: self.token.clone(),
            expression: self.expression.as_ref().map(|e| e.clone_box()),
            cases: self.cases.clone(),
            default_case: self.default_case.clone(),
            variable_name: self.variable_name.clone(),
        })
    }
}

/// Individual case within a type switch statement
/// Represents: mood Type1, Type2 var: { ... }
#[derive(Debug, Clone)]
pub struct TypeSwitchCase {
    pub types: Vec<String>,
    pub body: BlockStatement,
    pub variable_name: Option<String>,
    /// Bound variable for each type (for LLVM compilation)
    pub bound_variables: Vec<Option<String>>,
}

impl TypeSwitchCase {
    pub fn new(types: Vec<String>, body: BlockStatement, variable_name: Option<String>) -> Self {
        let bound_variables = vec![variable_name.clone(); types.len()];
        Self {
            types,
            body,
            variable_name,
            bound_variables,
        }
    }

    pub fn single_type(type_name: String, body: BlockStatement, variable_name: Option<String>) -> Self {
        Self {
            types: vec![type_name],
            body,
            variable_name: variable_name.clone(),
            bound_variables: vec![variable_name],
        }
    }

    pub fn with_specific_bindings(types: Vec<String>, bound_variables: Vec<Option<String>>, body: BlockStatement) -> Self {
        Self {
            types,
            body,
            variable_name: bound_variables.get(0).cloned().flatten(),
            bound_variables,
        }
    }
}

impl Node for TypeSwitchCase {
    fn string(&self) -> String {
        let types_str = self.types.join(", ");
        format!("mood {}:\n{}", types_str, self.body.string())
    }

    fn token_literal(&self) -> String {
        "mood".to_string()
    }
}

/// Type assertion used within type switches
/// Represents the .(type) syntax for runtime type checking
#[derive(Debug, Clone)]
pub struct TypeSwitchAssertion {
    pub expression: Box<dyn Expression>,
    pub token: String, // "type" keyword
}

impl TypeSwitchAssertion {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression,
            token: "type".to_string(),
        }
    }
}

impl Node for TypeSwitchAssertion {
    fn string(&self) -> String {
        format!("{}.(type)", self.expression.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeSwitchAssertion {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeSwitchAssertion {
            expression: self.expression.clone_box(),
            token: self.token.clone(),
        })
    }
}

/// Utility functions for type switch analysis and compilation
pub struct TypeSwitchAnalyzer;

impl TypeSwitchAnalyzer {
    /// Extract interface expression and target type from type assertion
    /// Parses expressions like "value.(Type)" 
    pub fn parse_type_assertion(expr: &dyn Expression) -> Option<(Box<dyn Expression>, String)> {
        let expr_str = expr.string();
        if let Some(dot_pos) = expr_str.find(".(") {
            if let Some(end_pos) = expr_str.rfind(')') {
                let type_name = expr_str[dot_pos + 2..end_pos].trim();
                return Some((expr.clone_box(), type_name.to_string()));
            }
        }
        None
    }

    /// Check if an expression is a type assertion
    pub fn is_type_assertion(expr: &dyn Expression) -> bool {
        let expr_str = expr.string();
        expr_str.contains(".(") && expr_str.ends_with(')')
    }

    /// Get all types referenced in a type switch
    pub fn get_referenced_types(stmt: &TypeSwitchStatement) -> Vec<String> {
        let mut types = Vec::new();
        
        for case in &stmt.cases {
            types.extend(case.types.clone());
        }
        
        types.sort();
        types.dedup();
        types
    }

    /// Check if type switch has variable bindings
    pub fn has_variable_bindings(stmt: &TypeSwitchStatement) -> bool {
        stmt.variable_name.is_some() || 
        stmt.cases.iter().any(|case| case.variable_name.is_some())
    }

    /// Get variable bindings for each case
    pub fn get_case_bindings(stmt: &TypeSwitchStatement) -> Vec<(Vec<String>, Option<String>)> {
        stmt.cases.iter().map(|case| {
            (case.types.clone(), case.variable_name.clone())
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::literals::StringLiteral;

    #[test]
    fn test_type_switch_case_creation() {
        let body = BlockStatement::new("{}".to_string(), vec![]);
        let case = TypeSwitchCase::single_type("String".to_string(), body, Some("s".to_string()));
        
        assert_eq!(case.types.len(), 1);
        assert_eq!(case.types[0], "String");
        assert_eq!(case.variable_name, Some("s".to_string()));
        assert_eq!(case.bound_variables.len(), 1);
        assert_eq!(case.bound_variables[0], Some("s".to_string()));
    }

    #[test]
    fn test_type_switch_with_specific_bindings() {
        let body = BlockStatement::new("{}".to_string(), vec![]);
        let types = vec!["String".to_string(), "Number".to_string()];
        let bindings = vec![Some("s".to_string()), Some("n".to_string())];
        
        let case = TypeSwitchCase::with_specific_bindings(types, bindings, body);
        
        assert_eq!(case.types.len(), 2);
        assert_eq!(case.bound_variables.len(), 2);
        assert_eq!(case.bound_variables[0], Some("s".to_string()));
        assert_eq!(case.bound_variables[1], Some("n".to_string()));
    }

    #[test]
    fn test_type_switch_analyzer() {
        let expr = StringLiteral::new("test".to_string(), "value".to_string());
        
        // This would need a proper type assertion expression for full testing
        assert!(!TypeSwitchAnalyzer::is_type_assertion(&expr));
        
        let types = vec!["String".to_string(), "Number".to_string()];
        let body = BlockStatement::new("{}".to_string(), vec![]);
        let case = TypeSwitchCase::new(types.clone(), body, None);
        let type_switch = TypeSwitchStatement::new(
            "vibe_check".to_string(),
            Box::new(expr),
            vec![case],
            None,
            None,
        );
        
        let referenced_types = TypeSwitchAnalyzer::get_referenced_types(&type_switch);
        assert_eq!(referenced_types, types);
    }
}
