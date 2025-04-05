// Core module for the CURSED language
// Contains essential types needed by the runtime that were migrated from compiler

use std::fmt;

/// Symbol table module for tracking variables and scopes
pub mod symbol_table;

/// Character type (sip) methods
pub mod char;
#[cfg(test)]
mod char_test;

/// Compiled function representation for the runtime
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    /// The LLVM IR representation of the function
    pub ir_representation: String,
    /// Number of local variables
    pub num_locals: usize,
    /// Number of parameters
    pub num_parameters: usize,
    /// Names of free variables (used in closures)
    pub free_variables: Vec<String>,
    /// Function name
    pub name: Option<String>,
    /// Whether the function accepts variable arguments
    pub is_variadic: bool,
}

impl CompiledFunction {
    /// Create a new compiled function
    pub fn new(
        ir_representation: String,
        num_locals: usize,
        num_parameters: usize,
        name: Option<String>,
        is_variadic: bool,
    ) -> Self {
        Self {
            ir_representation,
            num_locals,
            num_parameters,
            free_variables: Vec::new(),
            name,
            is_variadic,
        }
    }
    
    /// Add a free variable reference
    pub fn add_free_variable(&mut self, name: &str) {
        self.free_variables.push(name.to_string());
    }
}

impl fmt::Display for CompiledFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "CompiledFunction[{}]", name)
        } else {
            write!(f, "CompiledFunction[anonymous]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiled_function_creation() {
        let func = CompiledFunction::new(
            "define i32 @add(i32 %a, i32 %b) { %result = add i32 %a, %b ret i32 %result }".to_string(),
            2,
            2,
            Some("add".to_string()),
            false,
        );

        assert_eq!(func.num_locals, 2);
        assert_eq!(func.num_parameters, 2);
        assert_eq!(func.name, Some("add".to_string()));
        assert_eq!(func.is_variadic, false);
        assert!(func.free_variables.is_empty());
    }

    #[test]
    fn test_add_free_variable() {
        let mut func = CompiledFunction::new(
            "define i32 @example() { ret i32 %x }".to_string(),
            1,
            0,
            Some("example".to_string()),
            false,
        );

        func.add_free_variable("x");
        func.add_free_variable("y");

        assert_eq!(func.free_variables.len(), 2);
        assert_eq!(func.free_variables[0], "x");
        assert_eq!(func.free_variables[1], "y");
    }

    #[test]
    fn test_display() {
        let func1 = CompiledFunction::new(
            "define void @named() {}".to_string(),
            0,
            0,
            Some("named".to_string()),
            false,
        );

        let func2 = CompiledFunction::new(
            "define void @lambda() {}".to_string(),
            0,
            0,
            None,
            false,
        );

        assert_eq!(format!("{}", func1), "CompiledFunction[named]");
        assert_eq!(format!("{}", func2), "CompiledFunction[anonymous]");
    }
} 