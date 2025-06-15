/// Runtime Functions for CURSED Execution
/// 
/// This module provides runtime function support for CURSED programs,
/// including built-in functions, standard library integration, and FFI support.

use crate::execution::{CursedValue, ValueType, ExecutionContext};
use crate::error::Error;
use std::collections::HashMap;

/// Registry for runtime functions
pub struct RuntimeFunctionRegistry {
    builtin_functions: HashMap<String, BuiltinFunction>,
    external_functions: HashMap<String, ExternalFunction>,
}

/// A built-in function that can be called from CURSED code
pub struct BuiltinFunction {
    pub name: String,
    pub parameter_types: Vec<ValueType>,
    pub return_type: ValueType,
    pub implementation: fn(&[CursedValue]) -> Result<CursedValue, Error>,
    pub description: String,
}

/// An external function (from C/Rust libraries)
pub struct ExternalFunction {
    pub name: String,
    pub parameter_types: Vec<ValueType>,
    pub return_type: ValueType,
    pub symbol_name: String,
    pub library_path: Option<String>,
}

impl RuntimeFunctionRegistry {
    /// Create a new runtime function registry
    pub fn new() -> Self {
        let mut registry = Self {
            builtin_functions: HashMap::new(),
            external_functions: HashMap::new(),
        };
        
        register_builtin_functions(&mut registry);
        registry
    }

    /// Register a built-in function
    pub fn register_builtin(&mut self, function: BuiltinFunction) {
        self.builtin_functions.insert(function.name.clone(), function);
    }

    /// Register an external function
    pub fn register_external(&mut self, function: ExternalFunction) {
        self.external_functions.insert(function.name.clone(), function);
    }

    /// Call a built-in function
    pub fn call_builtin(&self, name: &str, args: &[CursedValue]) -> Result<CursedValue, Error> {
        if let Some(function) = self.builtin_functions.get(name) {
            // Validate argument types
            if args.len() != function.parameter_types.len() {
                return Err(Error::RuntimeError(format!(
                    "Function '{}' expects {} arguments, got {}",
                    name, function.parameter_types.len(), args.len()
                )));
            }

            for (i, (arg, expected_type)) in args.iter().zip(&function.parameter_types).enumerate() {
                if arg.get_type() != *expected_type {
                    return Err(Error::RuntimeError(format!(
                        "Function '{}' argument {} expects type {:?}, got {:?}",
                        name, i, expected_type, arg.get_type()
                    )));
                }
            }

            // Call the function
            (function.implementation)(args)
        } else {
            Err(Error::RuntimeError(format!("Built-in function '{}' not found", name)))
        }
    }

    /// Check if a function is available
    pub fn has_function(&self, name: &str) -> bool {
        self.builtin_functions.contains_key(name) || self.external_functions.contains_key(name)
    }

    /// Get function signature
    pub fn get_function_signature(&self, name: &str) -> Option<(Vec<ValueType>, ValueType)> {
        if let Some(function) = self.builtin_functions.get(name) {
            Some((function.parameter_types.clone(), function.return_type.clone()))
        } else if let Some(function) = self.external_functions.get(name) {
            Some((function.parameter_types.clone(), function.return_type.clone()))
        } else {
            None
        }
    }

    /// Get all available function names
    pub fn get_function_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.builtin_functions.keys().cloned().collect();
        names.extend(self.external_functions.keys().cloned());
        names.sort();
        names
    }

    /// Get function help text
    pub fn get_function_help(&self, name: &str) -> Option<String> {
        if let Some(function) = self.builtin_functions.get(name) {
            Some(format!(
                "{}: {} -> {} - {}",
                function.name,
                function.parameter_types.iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(", "),
                format!("{:?}", function.return_type),
                function.description
            ))
        } else {
            None
        }
    }
}

/// Register all built-in functions
pub fn register_builtin_functions(registry: &mut RuntimeFunctionRegistry) {
    // Math functions
    registry.register_builtin(BuiltinFunction {
        name: "add".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_add,
        description: "Add two integers".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "subtract".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_subtract,
        description: "Subtract two integers".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "multiply".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_multiply,
        description: "Multiply two integers".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "divide".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_divide,
        description: "Divide two integers".to_string(),
    });

    // String functions
    registry.register_builtin(BuiltinFunction {
        name: "concat".to_string(),
        parameter_types: vec![ValueType::String, ValueType::String],
        return_type: ValueType::String,
        implementation: builtin_concat,
        description: "Concatenate two strings".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "length".to_string(),
        parameter_types: vec![ValueType::String],
        return_type: ValueType::Integer,
        implementation: builtin_length,
        description: "Get string length".to_string(),
    });

    // I/O functions
    registry.register_builtin(BuiltinFunction {
        name: "print".to_string(),
        parameter_types: vec![ValueType::String],
        return_type: ValueType::Nil,
        implementation: builtin_print,
        description: "Print string to stdout".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "println".to_string(),
        parameter_types: vec![ValueType::String],
        return_type: ValueType::Nil,
        implementation: builtin_println,
        description: "Print string to stdout with newline".to_string(),
    });

    // Type conversion functions
    registry.register_builtin(BuiltinFunction {
        name: "to_string".to_string(),
        parameter_types: vec![ValueType::Integer],
        return_type: ValueType::String,
        implementation: builtin_to_string,
        description: "Convert integer to string".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "to_int".to_string(),
        parameter_types: vec![ValueType::String],
        return_type: ValueType::Integer,
        implementation: builtin_to_int,
        description: "Convert string to integer".to_string(),
    });

    // Comparison functions
    registry.register_builtin(BuiltinFunction {
        name: "equals".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Boolean,
        implementation: builtin_equals,
        description: "Check if two integers are equal".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "less_than".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Boolean,
        implementation: builtin_less_than,
        description: "Check if first integer is less than second".to_string(),
    });

    // Utility functions
    registry.register_builtin(BuiltinFunction {
        name: "abs".to_string(),
        parameter_types: vec![ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_abs,
        description: "Get absolute value of integer".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "max".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_max,
        description: "Get maximum of two integers".to_string(),
    });

    registry.register_builtin(BuiltinFunction {
        name: "min".to_string(),
        parameter_types: vec![ValueType::Integer, ValueType::Integer],
        return_type: ValueType::Integer,
        implementation: builtin_min,
        description: "Get minimum of two integers".to_string(),
    });
}

// Built-in function implementations

fn builtin_add(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Integer(a + b))
    } else {
        Err(Error::RuntimeError("Invalid arguments for add function".to_string()))
    }
}

fn builtin_subtract(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Integer(a - b))
    } else {
        Err(Error::RuntimeError("Invalid arguments for subtract function".to_string()))
    }
}

fn builtin_multiply(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Integer(a * b))
    } else {
        Err(Error::RuntimeError("Invalid arguments for multiply function".to_string()))
    }
}

fn builtin_divide(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        if *b == 0 {
            Err(Error::RuntimeError("Division by zero".to_string()))
        } else {
            Ok(CursedValue::Integer(a / b))
        }
    } else {
        Err(Error::RuntimeError("Invalid arguments for divide function".to_string()))
    }
}

fn builtin_concat(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::String(a), CursedValue::String(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::String(format!("{}{}", a, b)))
    } else {
        Err(Error::RuntimeError("Invalid arguments for concat function".to_string()))
    }
}

fn builtin_length(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let CursedValue::String(s) = &args[0] {
        Ok(CursedValue::Integer(s.len() as i64))
    } else {
        Err(Error::RuntimeError("Invalid argument for length function".to_string()))
    }
}

fn builtin_print(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let CursedValue::String(s) = &args[0] {
        print!("{}", s);
        std::io::Write::flush(&mut std::io::stdout()).map_err(|e| {
            Error::RuntimeError(format!("Failed to flush stdout: {}", e))
        })?;
        Ok(CursedValue::Nil)
    } else {
        Err(Error::RuntimeError("Invalid argument for print function".to_string()))
    }
}

fn builtin_println(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let CursedValue::String(s) = &args[0] {
        println!("{}", s);
        Ok(CursedValue::Nil)
    } else {
        Err(Error::RuntimeError("Invalid argument for println function".to_string()))
    }
}

fn builtin_to_string(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let CursedValue::Integer(i) = &args[0] {
        Ok(CursedValue::String(i.to_string()))
    } else {
        Err(Error::RuntimeError("Invalid argument for to_string function".to_string()))
    }
}

fn builtin_to_int(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let CursedValue::String(s) = &args[0] {
        match s.parse::<i64>() {
            Ok(i) => Ok(CursedValue::Integer(i)),
            Err(_) => Err(Error::RuntimeError(format!("Cannot convert '{}' to integer", s))),
        }
    } else {
        Err(Error::RuntimeError("Invalid argument for to_int function".to_string()))
    }
}

fn builtin_equals(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Boolean(a == b))
    } else {
        Err(Error::RuntimeError("Invalid arguments for equals function".to_string()))
    }
}

fn builtin_less_than(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Boolean(a < b))
    } else {
        Err(Error::RuntimeError("Invalid arguments for less_than function".to_string()))
    }
}

fn builtin_abs(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let CursedValue::Integer(i) = &args[0] {
        Ok(CursedValue::Integer(i.abs()))
    } else {
        Err(Error::RuntimeError("Invalid argument for abs function".to_string()))
    }
}

fn builtin_max(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Integer(*a.max(b)))
    } else {
        Err(Error::RuntimeError("Invalid arguments for max function".to_string()))
    }
}

fn builtin_min(args: &[CursedValue]) -> Result<CursedValue, Error> {
    if let (CursedValue::Integer(a), CursedValue::Integer(b)) = (&args[0], &args[1]) {
        Ok(CursedValue::Integer(*a.min(b)))
    } else {
        Err(Error::RuntimeError("Invalid arguments for min function".to_string()))
    }
}

/// Register runtime functions with execution context
pub fn register_runtime_functions(context: &mut ExecutionContext) -> Result<(), Error> {
    use crate::execution::execution_context::{FunctionInfo, CompiledFunction};

    // Register built-in functions as available functions
    let registry = RuntimeFunctionRegistry::new();
    
    for (name, function) in &registry.builtin_functions {
        let function_info = FunctionInfo {
            name: name.clone(),
            parameter_types: function.parameter_types.clone(),
            return_type: function.return_type.clone(),
            is_compiled: true,
            source_location: None,
        };
        
        context.register_function(function_info);
        
        // Mark as compiled (they're implemented in Rust)
        let compiled_function = CompiledFunction {
            name: name.clone(),
            ir_code: format!("; Built-in function: {}", name),
            entry_point: format!("builtin_{}", name),
            compiled_at: std::time::SystemTime::now(),
        };
        
        context.add_compiled_function(compiled_function);
    }

    Ok(())
}

impl Default for RuntimeFunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_function_registry() {
        let registry = RuntimeFunctionRegistry::new();
        
        // Test that built-in functions are registered
        assert!(registry.has_function("add"));
        assert!(registry.has_function("subtract"));
        assert!(registry.has_function("print"));
        assert!(!registry.has_function("nonexistent"));
    }

    #[test]
    fn test_builtin_add() {
        let registry = RuntimeFunctionRegistry::new();
        
        let args = vec![CursedValue::Integer(5), CursedValue::Integer(3)];
        let result = registry.call_builtin("add", &args).unwrap();
        
        assert_eq!(result, CursedValue::Integer(8));
    }

    #[test]
    fn test_builtin_concat() {
        let registry = RuntimeFunctionRegistry::new();
        
        let args = vec![
            CursedValue::String("Hello".to_string()),
            CursedValue::String(" World".to_string())
        ];
        let result = registry.call_builtin("concat", &args).unwrap();
        
        assert_eq!(result, CursedValue::String("Hello World".to_string()));
    }

    #[test]
    fn test_builtin_divide_by_zero() {
        let registry = RuntimeFunctionRegistry::new();
        
        let args = vec![CursedValue::Integer(10), CursedValue::Integer(0)];
        let result = registry.call_builtin("divide", &args);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_function_signature() {
        let registry = RuntimeFunctionRegistry::new();
        
        let signature = registry.get_function_signature("add");
        assert_eq!(signature, Some((vec![ValueType::Integer, ValueType::Integer], ValueType::Integer)));
    }

    #[test]
    fn test_type_validation() {
        let registry = RuntimeFunctionRegistry::new();
        
        // Wrong argument types
        let args = vec![CursedValue::String("not a number".to_string()), CursedValue::Integer(5)];
        let result = registry.call_builtin("add", &args);
        
        assert!(result.is_err());
    }
}
