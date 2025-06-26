//! REPL Type Definitions

use crate::error::CursedError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ReplValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

pub trait ReplEvaluator {
    fn evaluate(&mut self, input: &str) -> Result<ReplValue, CursedError>;
    fn get_variables(&self) -> &HashMap<String, ReplValue>;
    fn set_variable(&mut self, name: String, value: ReplValue);
}

pub trait BuildIntegration {
    fn compile_snippet(&self, code: &str) -> Result<String, CursedError>;
    fn check_syntax(&self, code: &str) -> Result<bool, CursedError>;
    fn get_compilation_errors(&self) -> Vec<String>;
}

pub struct BasicReplEvaluator {
    variables: HashMap<String, ReplValue>,
}

impl BasicReplEvaluator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl ReplEvaluator for BasicReplEvaluator {
    fn evaluate(&mut self, input: &str) -> Result<ReplValue, CursedError> {
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(ReplValue::Nil);
        }

        // Handle variable assignment
        if input.starts_with("let ") && input.contains('=') {
            let parts: Vec<&str> = input.splitn(2, '=').collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim().strip_prefix("let ").unwrap_or("").trim();
                let value_str = parts[1].trim();
                
                let value = if value_str.starts_with('"') && value_str.ends_with('"') {
                    ReplValue::String(value_str[1..value_str.len()-1].to_string())
                } else if let Ok(num) = value_str.parse::<f64>() {
                    ReplValue::Number(num)
                } else if value_str == "true" {
                    ReplValue::Boolean(true)
                } else if value_str == "false" {
                    ReplValue::Boolean(false)
                } else if value_str == "nil" {
                    ReplValue::Nil
                } else {
                    ReplValue::String(value_str.to_string())
                };
                
                self.variables.insert(var_name.to_string(), value.clone());
                return Ok(value);
            }
        }

        // Handle variable lookup
        if let Some(value) = self.variables.get(input) {
            return Ok(value.clone());
        }

        // Default evaluation
        Ok(ReplValue::String(format!("Unknown: {}", input)))
    }

    fn get_variables(&self) -> &HashMap<String, ReplValue> {
        &self.variables
    }

    fn set_variable(&mut self, name: String, value: ReplValue) {
        self.variables.insert(name, value);
    }
}

pub struct BasicBuildIntegration {
    errors: Vec<String>,
}

impl BasicBuildIntegration {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
}

impl BuildIntegration for BasicBuildIntegration {
    fn compile_snippet(&self, code: &str) -> Result<String, CursedError> {
        // Basic syntax validation
        if code.trim().is_empty() {
            return Err(CursedError::syntax_error("Empty code snippet"));
        }
        
        // TODO: Integrate with actual CURSED compiler
        Ok("Compiled successfully".to_string())
    }

    fn check_syntax(&self, code: &str) -> Result<bool, CursedError> {
        // Basic syntax checks
        let balanced_parens = code.chars().fold(0, |acc, c| {
            match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            }
        }) == 0;

        Ok(balanced_parens && !code.trim().is_empty())
    }

    fn get_compilation_errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}
