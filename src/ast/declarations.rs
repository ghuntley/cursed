use std::any::Any;
use crate::ast::{Node, Statement, Expression};
use super::expressions::Identifier;
use super::statements::fields::FieldStatement;

/// SquadStatement represents a struct definition
pub struct SquadStatement {
    pub token: String, // Token::Squad
    pub name: Identifier,
    pub type_parameters: Vec<Identifier>, // Generic type parameters [T], [A, B], etc.
    pub fields: Vec<FieldStatement>,
}

impl Node for SquadStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        
        // Format the struct name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self.type_parameters.iter()
                .map(|param| param.string())
                .collect();
            format!("[{}]", params.join(", "))
        } else {
            String::new()
        };
        
        out.push_str(&format!("be_like {}{} squad {{\n", self.name.string(), type_params_str));
        
        for field in &self.fields {
            out.push_str(&format!("    {}\n", field.string()));
        }
        
        out.push_str("}\n");
        out
    }
}

impl Statement for SquadStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// CollabStatement represents an interface definition
pub struct CollabStatement {
    pub token: String, // Token::Collab
    pub name: Identifier,
    pub type_parameters: Vec<Identifier>, // Generic type parameters [T], [A, B], etc.
    pub methods: Vec<MethodSignature>,
}

impl Node for CollabStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        
        // Format the type name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self.type_parameters.iter()
                .map(|param| param.string())
                .collect();
            format!("[{}]", params.join(", "))
        } else {
            String::new()
        };
        
        out.push_str(&format!("be_like {}{} collab {{\n", self.name.string(), type_params_str));
        
        for method in &self.methods {
            out.push_str(&format!("    {}\n", method.string()));
        }
        
        out.push_str("}\n");
        out
    }
}

impl Statement for CollabStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// MethodSignature represents a method signature in an interface
pub struct MethodSignature {
    pub token: String, // Usually the method name token
    pub name: Identifier,
    pub parameters: Vec<ParameterStatement>,
    pub return_type: Option<Box<dyn Expression>>,
    pub type_parameters: Vec<Identifier>, // Generic type parameters for method
}

impl Node for MethodSignature {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        
        // Format the method name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self.type_parameters.iter()
                .map(|param| param.string())
                .collect();
            format!("[{}] ", params.join(", "))
        } else {
            String::new()
        };
        
        out.push_str(&format!("{}{}", self.name.string(), type_params_str));
        
        // Format parameters
        out.push_str("(");
        let params: Vec<String> = self.parameters.iter()
            .map(|param| param.string())
            .collect();
        out.push_str(&params.join(", "));
        out.push_str(")");
        
        // Format return type if any
        if let Some(ret_type) = &self.return_type {
            out.push_str(&format!(": {}", ret_type.string()));
        }
        
        out
    }
}

impl Statement for MethodSignature {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// FunctionStatement represents a function definition
pub struct FunctionStatement {
    pub token: String, // Token::Function
    pub name: Identifier,
    pub parameters: Vec<ParameterStatement>,
    pub body: super::statements::block::BlockStatement,
    pub return_type: Option<Box<dyn Expression>>,
    pub type_parameters: Vec<Identifier>, // Generic type parameters for function [T], [A, B], etc.
}

impl Node for FunctionStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        
        // Format the function name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self.type_parameters.iter()
                .map(|param| param.string())
                .collect();
            format!("[{}] ", params.join(", "))
        } else {
            String::new()
        };
        
        out.push_str(&format!("{} {}{}", self.token_literal(), self.name.string(), type_params_str));
        
        // Format parameters
        out.push_str("(");
        let params: Vec<String> = self.parameters.iter()
            .map(|param| param.string())
            .collect();
        out.push_str(&params.join(", "));
        out.push_str(")");
        
        // Format return type if any
        if let Some(ret_type) = &self.return_type {
            out.push_str(&format!(": {}", ret_type.string()));
        }
        
        // Format body
        out.push_str(" ");
        out.push_str(&self.body.string());
        
        out
    }
}

impl Statement for FunctionStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ParameterStatement represents a function parameter
pub struct ParameterStatement {
    pub token: String, // Usually the parameter name token
    pub name: Identifier,
    pub type_name: Box<dyn Expression>,
}

impl Node for ParameterStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.name.string(), self.type_name.string())
    }
}

impl Statement for ParameterStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}