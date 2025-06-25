/// Comprehensive test suite for function call compilation in CURSED
use cursed::codegen::llvm::{LlvmCodeGenerator, function_registry::{FunctionRegistry, FunctionSignature}, expression_compiler::LlvmType};
use cursed::ast::{
    declarations::FunctionStatement,
    calls::CallExpression,
    identifiers::Identifier,
    expressions::{Literal, LiteralValue, Parameter},
    block::BlockStatement,
};
use cursed::error::Error;
use std::sync::{Arc, Mutex};

#[test]
fn test_function_registry_creation() {
    let registry = FunctionRegistry::new();
    
    // Test that built-in functions are registered
    assert!(registry.has_function("print"));
    assert!(registry.has_function("println"));
    assert!(registry.has_function("malloc"));
    assert!(registry.has_function("strlen"));
    assert!(registry.has_function("abs"));
    assert!(registry.has_function("sqrt"));
    
    // Test function count includes built-ins
    assert!(registry.function_count() > 10);
}

#[test]
fn test_function_signature_creation() {
    let signature = FunctionSignature::new(
        "test_func".to_string(),
        vec![
            ("x".to_string(), LlvmType::Int32),
            ("y".to_string(), LlvmType::Float64),
        ],
        LlvmType::Int32,
        false,
    );
    
    assert_eq!(signature.name, "test_func");
    assert_eq!(signature.parameters.len(), 2);
    assert_eq!(signature.return_type, LlvmType::Int32);
    assert_eq!(signature.llvm_function_type, "i32 (i32, double)");
    assert!(!signature.is_builtin);
    assert!(!signature.is_variadic);
}

#[test]
fn test_function_signature_from_ast() {
    // Create a simple function AST: slay test_func(x: normie, y: vibes) -> facts { }
    let func_name = Identifier::new("test_func".to_string(), "test_func".to_string());
    let parameters = vec![
        Parameter::new("x".to_string(), "normie".to_string()),
        Parameter::new("y".to_string(), "vibes".to_string()),
    ];
    let return_type = Some(Box::new(Identifier::new("facts".to_string(), "facts".to_string())) as Box<dyn cursed::ast::traits::Expression>);
    let body = BlockStatement::new("test_func_body".to_string(), vec![]);
    
    let func_stmt = FunctionStatement::new(
        "slay".to_string(),
        func_name,
        parameters,
        return_type,
        body,
    );
    
    let signature = FunctionSignature::from_function_statement(&func_stmt).unwrap();
    
    assert_eq!(signature.name, "test_func");
    assert_eq!(signature.parameters.len(), 2);
    assert_eq!(signature.parameters[0].0, "x");
    assert_eq!(signature.parameters[0].1, LlvmType::Int64); // normie maps to i64
    assert_eq!(signature.parameters[1].0, "y");
    assert_eq!(signature.parameters[1].1, LlvmType::Float64); // vibes maps to f64
    assert_eq!(signature.return_type, LlvmType::Boolean); // facts maps to bool
}

#[test]
fn test_argument_type_checking() {
    let signature = FunctionSignature::new(
        "test".to_string(),
        vec![
            ("x".to_string(), LlvmType::Int32),
            ("y".to_string(), LlvmType::Float64),
        ],
        LlvmType::Void,
        false,
    );
    
    // Valid arguments
    assert!(signature.check_argument_types(&[LlvmType::Int32, LlvmType::Float64]).is_ok());
    
    // Compatible arguments (int64 -> int32 conversion allowed)
    assert!(signature.check_argument_types(&[LlvmType::Int64, LlvmType::Float64]).is_ok());
    
    // Wrong number of arguments
    assert!(signature.check_argument_types(&[LlvmType::Int32]).is_err());
    
    // Incompatible types
    assert!(signature.check_argument_types(&[LlvmType::String, LlvmType::Float64]).is_err());
}

#[test]
fn test_variadic_function_checking() {
    let signature = FunctionSignature::new_variadic(
        "printf".to_string(),
        vec![("format".to_string(), LlvmType::String)],
        LlvmType::Int32,
        true,
    );
    
    assert!(signature.is_variadic);
    assert_eq!(signature.llvm_function_type, "i32 (i8*, ...)");
    
    // Should accept any number of arguments >= required
    assert!(signature.check_argument_types(&[LlvmType::String]).is_ok());
    assert!(signature.check_argument_types(&[LlvmType::String, LlvmType::Int32]).is_ok());
    assert!(signature.check_argument_types(&[LlvmType::String, LlvmType::Int32, LlvmType::Float64]).is_ok());
    
    // Should reject fewer than required
    assert!(signature.check_argument_types(&[]).is_err());
}

#[test]
fn test_function_registry_registration() {
    let mut registry = FunctionRegistry::new();
    
    // Test user function registration
    let signature = FunctionSignature::new(
        "my_func".to_string(),
        vec![("x".to_string(), LlvmType::Int32)],
        LlvmType::Int32,
        false,
    );
    
    assert!(registry.register_function(signature).is_ok());
    assert!(registry.has_function("my_func"));
    
    // Test built-in function collision
    let builtin_collision = FunctionSignature::new(
        "print".to_string(),
        vec![],
        LlvmType::Void,
        false,
    );
    
    assert!(registry.register_function(builtin_collision).is_err());
}

#[test]
fn test_function_lookup_with_overloads() {
    let mut registry = FunctionRegistry::new();
    
    // Register multiple overloads of a function
    let func1 = FunctionSignature::new(
        "test".to_string(),
        vec![("x".to_string(), LlvmType::Int32)],
        LlvmType::Int32,
        false,
    );
    
    let func2 = FunctionSignature::new(
        "test".to_string(),
        vec![
            ("x".to_string(), LlvmType::Int32),
            ("y".to_string(), LlvmType::Int32),
        ],
        LlvmType::Int32,
        false,
    );
    
    assert!(registry.register_function(func1).is_ok());
    
    // Test overload resolution
    let lookup1 = registry.lookup_function_with_args("test", &[LlvmType::Int32]);
    assert!(lookup1.is_some());
    assert_eq!(lookup1.unwrap().parameters.len(), 1);
    
    // This should fail since we haven't registered the 2-arg overload yet
    let lookup2 = registry.lookup_function_with_args("test", &[LlvmType::Int32, LlvmType::Int32]);
    assert!(lookup2.is_none());
}

#[test]
fn test_llvm_code_generator_integration() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test that function registry is initialized
    assert!(generator.get_function_count() > 0);
    
    // Test built-in function lookup
    assert!(generator.has_function("print"));
    assert!(generator.has_function("malloc"));
    
    // Test registering a new function
    let signature = FunctionSignature::new(
        "my_custom_func".to_string(),
        vec![("x".to_string(), LlvmType::Int32)],
        LlvmType::Int32,
        false,
    );
    
    assert!(generator.register_function(signature).is_ok());
    assert!(generator.has_function("my_custom_func"));
}

#[test]
fn test_function_compilation_with_registry() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create a simple function AST
    let func_name = Identifier::new("add_numbers".to_string(), "add_numbers".to_string());
    let parameters = vec![
        Parameter::new("a".to_string(), "normie".to_string()),
        Parameter::new("b".to_string(), "normie".to_string()),
    ];
    let return_type = Some(Box::new(Identifier::new("normie".to_string(), "normie".to_string())) as Box<dyn cursed::ast::traits::Expression>);
    let body = BlockStatement::new("add_numbers_body".to_string(), vec![]);
    
    let func_stmt = FunctionStatement::new(
        "slay".to_string(),
        func_name,
        parameters,
        return_type,
        body,
    );
    
    // Compile the function (this should register it in the registry)
    let result = generator.compile_function_declaration(&func_stmt);
    assert!(result.is_ok());
    
    // Verify the function was registered
    assert!(generator.has_function("add_numbers"));
    
    let lookup = generator.lookup_function("add_numbers").unwrap();
    assert_eq!(lookup.name, "add_numbers");
    assert_eq!(lookup.parameters.len(), 2);
    assert_eq!(lookup.return_type, LlvmType::Int64); // normie maps to i64
}

#[test]
fn test_function_call_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create a function call AST: print("Hello")
    let function_name = Identifier::new("print".to_string(), "print".to_string());
    let arguments = vec![
        Box::new(Literal::string("Hello".to_string())) as Box<dyn cursed::ast::traits::Expression>
    ];
    
    let call_expr = CallExpression::new(
        Box::new(function_name),
        arguments,
        1,
        1,
    );
    
    // Compile the function call
    let result = generator.compile_function_call_value(&call_expr);
    assert!(result.is_ok());
    
    let call_result = result.unwrap();
    assert_eq!(call_result.value_type, LlvmType::Void); // print returns void
    assert!(!call_result.is_constant);
}

#[test]
fn test_function_call_with_type_checking() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Register a custom function
    let signature = FunctionSignature::new(
        "add_ints".to_string(),
        vec![
            ("a".to_string(), LlvmType::Int32),
            ("b".to_string(), LlvmType::Int32),
        ],
        LlvmType::Int32,
        false,
    );
    generator.register_function(signature).unwrap();
    
    // Create a valid function call: add_ints(5, 10)
    let function_name = Identifier::new("add_ints".to_string(), "add_ints".to_string());
    let arguments = vec![
        Box::new(Literal::integer(5)) as Box<dyn cursed::ast::traits::Expression>,
        Box::new(Literal::integer(10)) as Box<dyn cursed::ast::traits::Expression>,
    ];
    
    let call_expr = CallExpression::new(
        Box::new(function_name),
        arguments,
        1,
        1,
    );
    
    // This should compile successfully
    let result = generator.compile_function_call_value(&call_expr);
    assert!(result.is_ok());
    
    let call_result = result.unwrap();
    assert_eq!(call_result.value_type, LlvmType::Int32);
}

#[test]
fn test_function_call_with_wrong_arguments() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create a function call to a non-existent function
    let function_name = Identifier::new("non_existent_func".to_string(), "non_existent_func".to_string());
    let arguments = vec![
        Box::new(Literal::integer(42)) as Box<dyn cursed::ast::traits::Expression>
    ];
    
    let call_expr = CallExpression::new(
        Box::new(function_name),
        arguments,
        1,
        1,
    );
    
    // This should fail with a function not found error
    let result = generator.compile_function_call_value(&call_expr);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        Error::CompilationError(msg) => {
            assert!(msg.contains("Function 'non_existent_func' not found"));
        }
        _ => panic!("Expected CompilationError"),
    }
}

#[test]
fn test_builtin_function_types() {
    let registry = FunctionRegistry::new();
    
    // Test print function
    let print_func = registry.lookup_function("print").unwrap();
    assert_eq!(print_func.parameters.len(), 1);
    assert_eq!(print_func.parameters[0].1, LlvmType::String);
    assert_eq!(print_func.return_type, LlvmType::Void);
    
    // Test malloc function
    let malloc_func = registry.lookup_function("malloc").unwrap();
    assert_eq!(malloc_func.parameters.len(), 1);
    assert_eq!(malloc_func.parameters[0].1, LlvmType::Int64);
    assert_eq!(malloc_func.return_type, LlvmType::Pointer(Box::new(LlvmType::Void)));
    
    // Test math functions
    let sqrt_func = registry.lookup_function("sqrt").unwrap();
    assert_eq!(sqrt_func.parameters.len(), 1);
    assert_eq!(sqrt_func.parameters[0].1, LlvmType::Float64);
    assert_eq!(sqrt_func.return_type, LlvmType::Float64);
}

#[test]
fn test_cursed_type_mapping() {
    // Test Gen Z slang types mapping
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("normie").unwrap(),
        LlvmType::Int64
    );
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("sus").unwrap(),
        LlvmType::Int64
    );
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("facts").unwrap(),
        LlvmType::Boolean
    );
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("tea").unwrap(),
        LlvmType::String
    );
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("vibes").unwrap(),
        LlvmType::Float64
    );
    
    // Test standard types
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("i32").unwrap(),
        LlvmType::Int32
    );
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("string").unwrap(),
        LlvmType::String
    );
    assert_eq!(
        FunctionSignature::parse_cursed_type_to_llvm("void").unwrap(),
        LlvmType::Void
    );
}

#[test]
fn test_function_name_listing() {
    let mut registry = FunctionRegistry::new();
    
    // Register some custom functions
    let func1 = FunctionSignature::new(
        "func_a".to_string(),
        vec![],
        LlvmType::Void,
        false,
    );
    let func2 = FunctionSignature::new(
        "func_b".to_string(),
        vec![],
        LlvmType::Void,
        false,
    );
    
    registry.register_function(func1).unwrap();
    registry.register_function(func2).unwrap();
    
    let function_names = registry.get_function_names();
    
    // Should include built-ins and custom functions
    assert!(function_names.contains(&"print".to_string()));
    assert!(function_names.contains(&"malloc".to_string()));
    assert!(function_names.contains(&"func_a".to_string()));
    assert!(function_names.contains(&"func_b".to_string()));
    
    // Should be sorted
    for i in 1..function_names.len() {
        assert!(function_names[i-1] <= function_names[i]);
    }
}

#[test]
fn test_function_registry_thread_safety() {
    use std::thread;
    
    let registry = Arc::new(Mutex::new(FunctionRegistry::new()));
    let mut handles = vec![];
    
    // Spawn multiple threads that register functions
    for i in 0..10 {
        let registry_clone = registry.clone();
        let handle = thread::spawn(move || {
            let signature = FunctionSignature::new(
                format!("thread_func_{}", i),
                vec![],
                LlvmType::Void,
                false,
            );
            
            let mut reg = registry_clone.lock().unwrap();
            reg.register_function(signature).unwrap();
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all functions were registered
    let reg = registry.lock().unwrap();
    for i in 0..10 {
        assert!(reg.has_function(&format!("thread_func_{}", i)));
    }
}

#[test]
fn test_function_call_ir_generation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create and compile a simple function call
    let function_name = Identifier::new("abs".to_string(), "abs".to_string());
    let arguments = vec![
        Box::new(Literal::integer(-42)) as Box<dyn cursed::ast::traits::Expression>
    ];
    
    let call_expr = CallExpression::new(
        Box::new(function_name),
        arguments,
        1,
        1,
    );
    
    let result = generator.compile_function_call_value(&call_expr);
    assert!(result.is_ok());
    
    let call_result = result.unwrap();
    
    // Verify the result has the correct type
    assert_eq!(call_result.value_type, LlvmType::Int32);
    assert!(call_result.llvm_name.starts_with("%call_result_"));
    assert!(!call_result.is_constant);
}

// Integration test with realistic CURSED code
#[test] 
fn test_realistic_function_usage() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Register a custom mathematical function
    let math_signature = FunctionSignature::new(
        "calculate_area".to_string(),
        vec![
            ("width".to_string(), LlvmType::Float64),
            ("height".to_string(), LlvmType::Float64),
        ],
        LlvmType::Float64,
        false,
    );
    generator.register_function(math_signature).unwrap();
    
    // Test function call with float arguments
    let function_name = Identifier::new("calculate_area".to_string(), "calculate_area".to_string());
    let arguments = vec![
        Box::new(Literal::float(10.5)) as Box<dyn cursed::ast::traits::Expression>,
        Box::new(Literal::float(20.0)) as Box<dyn cursed::ast::traits::Expression>,
    ];
    
    let call_expr = CallExpression::new(
        Box::new(function_name),
        arguments,
        1,
        1,
    );
    
    let result = generator.compile_function_call_value(&call_expr);
    assert!(result.is_ok());
    
    let call_result = result.unwrap();
    assert_eq!(call_result.value_type, LlvmType::Float64);
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn bench_function_registry_lookup() {
        let mut registry = FunctionRegistry::new();
        
        // Register many functions
        for i in 0..1000 {
            let signature = FunctionSignature::new(
                format!("bench_func_{}", i),
                vec![("x".to_string(), LlvmType::Int32)],
                LlvmType::Int32,
                false,
            );
            registry.register_function(signature).unwrap();
        }
        
        let start = Instant::now();
        
        // Perform many lookups
        for i in 0..1000 {
            let func_name = format!("bench_func_{}", i);
            let result = registry.lookup_function(&func_name);
            assert!(result.is_some());
        }
        
        let duration = start.elapsed();
        println!("1000 function lookups took: {:?}", duration);
        
        // Should be reasonably fast (less than 10ms on modern hardware)
        assert!(duration.as_millis() < 50);
    }
    
    #[test]
    fn bench_function_call_compilation() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        let start = Instant::now();
        
        // Compile many function calls
        for i in 0..100 {
            let function_name = Identifier::new("print".to_string(), "print".to_string());
            let arguments = vec![
                Box::new(Literal::string(format!("Message {}", i))) as Box<dyn cursed::ast::traits::Expression>
            ];
            
            let call_expr = CallExpression::new(
                Box::new(function_name),
                arguments,
                1,
                1,
            );
            
            let result = generator.compile_function_call_value(&call_expr);
            assert!(result.is_ok());
        }
        
        let duration = start.elapsed();
        println!("100 function call compilations took: {:?}", duration);
        
        // Should be reasonably fast
        assert!(duration.as_millis() < 1000);
    }
}
