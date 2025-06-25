/// LLVM Type Switch Compilation Tests
/// 
/// Tests for type switch LLVM code generation functionality including
/// runtime type checking, variable binding, and interface type assertions.

use cursed::codegen::llvm::{
    TypeSwitchCompilation, TypeSwitchContext, LlvmTypeSwitchCompiler, 
    TypeSwitchUtils, LlvmTypeRegistry
};
use cursed::ast::type_switch::{TypeSwitchStatement, TypeSwitchCase, TypeSwitchAnalyzer};
use cursed::ast::expressions::literals::StringLiteral;
use cursed::ast::block::BlockStatement;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::FunctionValue;

#[test]
fn test_type_switch_utils_parsing() {
    // Test parsing type assertion syntax
    let result = TypeSwitchUtils::parse_type_switch_expr("value.(String)");
    assert!(result.is_ok());
    
    let (variable, type_name) = result.unwrap();
    assert_eq!(variable, "value");
    assert_eq!(type_name, "String");
}

#[test]
fn test_type_switch_utils_invalid_syntax() {
    // Test invalid syntax handling
    let result = TypeSwitchUtils::parse_type_switch_expr("invalid_syntax");
    assert!(result.is_err());
    
    let result = TypeSwitchUtils::parse_type_switch_expr("value.(");
    assert!(result.is_err());
}

#[test] 
fn test_type_switch_context_creation() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a test function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Create type registry
    let type_registry = LlvmTypeRegistry::new();
    
    // Create type switch context
    let ctx = TypeSwitchContext::new(function, &type_registry);
    
    assert_eq!(ctx.case_variables.len(), 0);
    assert_eq!(ctx.variable_scopes.len(), 1);
    assert_eq!(ctx.current_function, function);
}

#[test]
fn test_type_switch_context_scoping() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let type_registry = LlvmTypeRegistry::new();
    
    let mut ctx = TypeSwitchContext::new(function, &type_registry);
    
    // Test scope operations
    ctx.push_scope();
    assert_eq!(ctx.variable_scopes.len(), 2);
    
    ctx.pop_scope();
    assert_eq!(ctx.variable_scopes.len(), 1);
}

#[test]
fn test_llvm_runtime_declarations() {
    let context = Context::create();
    let module = context.create_module("test_module");
    
    // Test generating runtime function declarations
    let result = TypeSwitchUtils::generate_runtime_declarations(&context, &module);
    assert!(result.is_ok());
    
    // Check that functions were declared
    assert!(module.get_function("cursed_type_switch_check").is_some());
    assert!(module.get_function("cursed_type_switch_extract").is_some());
    assert!(module.get_function("cursed_type_switch_get_type_id").is_some());
}

#[test]
fn test_type_id_calculation() {
    let compiler = LlvmTypeSwitchCompiler;
    
    // Test type ID calculation consistency
    let id1 = compiler.calculate_type_id("String");
    let id2 = compiler.calculate_type_id("String");
    let id3 = compiler.calculate_type_id("Number");
    
    assert_eq!(id1, id2); // Same type should have same ID
    assert_ne!(id1, id3); // Different types should have different IDs
}

#[test]
fn test_type_mapping_to_llvm() {
    let compiler = LlvmTypeSwitchCompiler;
    let registry = LlvmTypeRegistry::new();
    
    // Test primitive type mappings
    let (llvm_type, size) = compiler.map_type_to_llvm("normie", &registry).unwrap();
    assert_eq!(llvm_type, "i64");
    assert_eq!(size, 8);
    
    let (llvm_type, size) = compiler.map_type_to_llvm("facts", &registry).unwrap();
    assert_eq!(llvm_type, "i1");
    assert_eq!(size, 1);
    
    let (llvm_type, size) = compiler.map_type_to_llvm("tea", &registry).unwrap();
    assert_eq!(llvm_type, "i8*");
    assert_eq!(size, 8);
    
    let (llvm_type, size) = compiler.map_type_to_llvm("sus", &registry).unwrap();
    assert_eq!(llvm_type, "i8*");
    assert_eq!(size, 8);
}

#[test]
fn test_expected_type_id_primitive_types() {
    let compiler = LlvmTypeSwitchCompiler;
    let registry = LlvmTypeRegistry::new();
    
    // Test predefined primitive type IDs
    assert_eq!(compiler.get_expected_type_id("normie", &registry).unwrap(), 1);
    assert_eq!(compiler.get_expected_type_id("facts", &registry).unwrap(), 2);
    assert_eq!(compiler.get_expected_type_id("tea", &registry).unwrap(), 3);
    assert_eq!(compiler.get_expected_type_id("sus", &registry).unwrap(), 4);
}

#[test]
fn test_expected_type_id_unknown_type() {
    let compiler = LlvmTypeSwitchCompiler;
    let registry = LlvmTypeRegistry::new();
    
    // Test unknown type handling
    let result = compiler.get_expected_type_id("UnknownType", &registry);
    assert!(result.is_err());
}

#[test]
fn test_llvm_type_parsing() {
    let compiler = LlvmTypeSwitchCompiler;
    let context = Context::create();
    
    // Test basic type parsing
    let bool_type = compiler.parse_llvm_type(&context, "i1").unwrap();
    assert!(bool_type.is_int_type());
    
    let i64_type = compiler.parse_llvm_type(&context, "i64").unwrap();
    assert!(i64_type.is_int_type());
    
    let ptr_type = compiler.parse_llvm_type(&context, "i8*").unwrap();
    assert!(ptr_type.is_pointer_type());
    
    let interface_type = compiler.parse_llvm_type(&context, "{i8*, i8*}").unwrap();
    assert!(interface_type.is_struct_type());
}

#[test]
fn test_llvm_type_parsing_invalid() {
    let compiler = LlvmTypeSwitchCompiler;
    let context = Context::create();
    
    // Test invalid type string handling
    let result = compiler.parse_llvm_type(&context, "invalid_type");
    assert!(result.is_err());
}

#[test]
fn test_type_switch_analyzer_functions() {
    let expr = StringLiteral::new("test".to_string(), "value".to_string());
    
    // Test type assertion detection (should be false for string literal)
    assert!(!TypeSwitchAnalyzer::is_type_assertion(&expr));
    
    // Create a type switch for testing
    let body = BlockStatement::new("{}".to_string(), vec![]);
    let case1 = TypeSwitchCase::single_type("String".to_string(), body.clone(), Some("s".to_string()));
    let case2 = TypeSwitchCase::single_type("Number".to_string(), body.clone(), Some("n".to_string()));
    
    let type_switch = TypeSwitchStatement::new(
        "vibe_check".to_string(),
        Box::new(expr),
        vec![case1, case2],
        None,
        Some("x".to_string()),
    );
    
    // Test referenced types extraction
    let referenced_types = TypeSwitchAnalyzer::get_referenced_types(&type_switch);
    assert_eq!(referenced_types.len(), 2);
    assert!(referenced_types.contains(&"String".to_string()));
    assert!(referenced_types.contains(&"Number".to_string()));
    
    // Test variable binding detection
    assert!(TypeSwitchAnalyzer::has_variable_bindings(&type_switch));
    
    // Test case bindings extraction
    let case_bindings = TypeSwitchAnalyzer::get_case_bindings(&type_switch);
    assert_eq!(case_bindings.len(), 2);
    assert_eq!(case_bindings[0].0, vec!["String".to_string()]);
    assert_eq!(case_bindings[0].1, Some("s".to_string()));
    assert_eq!(case_bindings[1].0, vec!["Number".to_string()]);
    assert_eq!(case_bindings[1].1, Some("n".to_string()));
}

#[test]
fn test_type_switch_case_with_multiple_types() {
    let body = BlockStatement::new("{}".to_string(), vec![]);
    let types = vec!["String".to_string(), "Number".to_string()];
    let case = TypeSwitchCase::new(types.clone(), body, Some("value".to_string()));
    
    assert_eq!(case.types, types);
    assert_eq!(case.variable_name, Some("value".to_string()));
    assert_eq!(case.bound_variables.len(), 2);
    assert_eq!(case.bound_variables[0], Some("value".to_string()));
    assert_eq!(case.bound_variables[1], Some("value".to_string()));
}

#[test]
fn test_type_switch_case_with_specific_bindings() {
    let body = BlockStatement::new("{}".to_string(), vec![]);
    let types = vec!["String".to_string(), "Number".to_string()];
    let bindings = vec![Some("str".to_string()), Some("num".to_string())];
    
    let case = TypeSwitchCase::with_specific_bindings(types.clone(), bindings.clone(), body);
    
    assert_eq!(case.types, types);
    assert_eq!(case.bound_variables, bindings);
    assert_eq!(case.variable_name, Some("str".to_string())); // First binding
}

#[test]
fn test_type_switch_statement_string_representation() {
    let expr = StringLiteral::new("test".to_string(), "interface_value".to_string());
    let body = BlockStatement::new("{}".to_string(), vec![]);
    let case = TypeSwitchCase::single_type("String".to_string(), body, Some("s".to_string()));
    
    let type_switch = TypeSwitchStatement::new(
        "vibe_check".to_string(),
        Box::new(expr),
        vec![case],
        None,
        Some("x".to_string()),
    );
    
    let string_repr = type_switch.string();
    assert!(string_repr.contains("vibe_check"));
    assert!(string_repr.contains("x := interface_value.(type)"));
    assert!(string_repr.contains("mood String:"));
}

/// Integration test for complete type switch compilation workflow
#[test]
fn test_type_switch_compilation_integration() {
    let context = Context::create();
    let module = context.create_module("type_switch_test");
    let builder = context.create_builder();
    
    // Setup LLVM function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_type_switch", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create type registry and context
    let type_registry = LlvmTypeRegistry::new();
    let mut ctx = TypeSwitchContext::new(function, &type_registry);
    
    // Generate runtime declarations
    TypeSwitchUtils::generate_runtime_declarations(&context, &module).unwrap();
    
    // Create a simple type switch AST
    let interface_expr = StringLiteral::new("test".to_string(), "interface_value".to_string());
    let body = BlockStatement::new("{}".to_string(), vec![]);
    let case = TypeSwitchCase::single_type("String".to_string(), body, Some("s".to_string()));
    
    // Test compilation (would need full integration with expression compiler)
    let compiler = LlvmTypeSwitchCompiler;
    
    // For now, just test that the structures are created correctly
    assert_eq!(ctx.current_function, function);
    assert!(module.get_function("cursed_type_switch_check").is_some());
    
    // Test type checking code generation
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
    let interface_type = context.struct_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    let null_ptr = i8_ptr_type.const_null();
    let interface_value = interface_type.const_named_struct(&[null_ptr.into(), null_ptr.into()]);
    
    let type_check_result = compiler.generate_type_check(
        &context,
        &module, 
        &builder,
        interface_value.into(),
        "String",
        &type_registry,
    );
    
    assert!(type_check_result.is_ok());
    
    println!("Type switch LLVM compilation integration test passed");
}
