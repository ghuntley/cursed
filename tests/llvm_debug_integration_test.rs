//! Comprehensive LLVM Debug Integration Tests
//! 
//! This test suite validates the complete LLVM debug information generation
//! system including DWARF metadata, source location mapping, and debugger
//! integration for the CURSED programming language.

use cursed::codegen::llvm::debug_metadata::{LlvmDebugMetadata, DebugStats};
use cursed::debug::{DebugConfig, SourceLocation};
use cursed::ast::{
    AST, Expression, Statement, FunctionDeclaration, VariableDeclaration,
    Parameter, Literal, BinaryOp, BinaryOperator, Type
};
use cursed::error::Error as CursedError;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum};
use inkwell::types::{BasicTypeEnum, FunctionType};
use inkwell::{AddressSpace, IntPredicate};

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Helper function to create test debug configuration
fn create_test_debug_config() -> DebugConfig {
    DebugConfig {
        generate_debug_info: true,
        optimized_debug: false,
        include_source_text: true,
        generate_line_tables: true,
        ..Default::default()
    }
}

/// Helper function to create test source location
fn create_test_location(line: u32, column: u32) -> SourceLocation {
    SourceLocation::new(PathBuf::from("test.csd"), line, column)
}

/// Helper function to create test function declaration
fn create_test_function_declaration(name: &str, line: u32) -> FunctionDeclaration {
    FunctionDeclaration {
        name: name.to_string(),
        parameters: vec![
            Parameter {
                name: "param1".to_string(),
                param_type: "sus".to_string(),
            },
            Parameter {
                name: "param2".to_string(),
                param_type: "vibes".to_string(),
            },
        ],
        return_type: "sus".to_string(),
        body: vec![],
        location: create_test_location(line, 1),
    }
}

/// Helper function to create test variable declaration
fn create_test_variable_declaration(name: &str, type_name: &str, line: u32) -> VariableDeclaration {
    VariableDeclaration {
        name: name.to_string(),
        var_type: type_name.to_string(),
        value: Some(Expression::Literal {
            value: Literal::Integer(42),
            location: create_test_location(line, 10),
        }),
        is_mutable: true,
        location: create_test_location(line, 5),
    }
}

/// Helper function to create test expression with location
fn create_test_expression(line: u32) -> Expression {
    Expression::BinaryOp {
        left: Box::new(Expression::Variable {
            name: "x".to_string(),
            location: create_test_location(line, 5),
        }),
        operator: BinaryOperator::Add,
        right: Box::new(Expression::Literal {
            value: Literal::Integer(10),
            location: create_test_location(line, 9),
        }),
        location: create_test_location(line, 7),
    }
}

/// Helper function to create test LLVM context and module
fn create_test_llvm_context() -> (Context, Module<'static>, Builder<'static>) {
    let context = Context::create();
    let module = context.create_module("test_debug_module");
    let builder = context.create_builder();
    (context, module, builder)
}

#[test]
fn test_debug_metadata_creation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let result = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    );
    
    assert!(result.is_ok(), "Debug metadata creation should succeed");
    
    if let Ok(metadata) = result {
        assert!(metadata.is_enabled(), "Debug should be enabled");
        assert_eq!(metadata.statistics().files_processed, 1);
        assert_eq!(metadata.statistics().metadata_entries_generated, 1);
    }
}

#[test]
fn test_debug_config_integration() {
    let (context, module, builder) = create_test_llvm_context();
    
    // Test with debug enabled
    let config_enabled = DebugConfig {
        generate_debug_info: true,
        optimized_debug: false,
        include_source_text: true,
        ..Default::default()
    };
    
    let result_enabled = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config_enabled,
    );
    
    assert!(result_enabled.is_ok());
    if let Ok(metadata) = result_enabled {
        assert!(metadata.is_enabled());
    }
    
    // Test with debug disabled
    let config_disabled = DebugConfig {
        generate_debug_info: false,
        ..Default::default()
    };
    
    let result_disabled = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config_disabled,
    );
    
    assert!(result_disabled.is_ok());
    if let Ok(metadata) = result_disabled {
        assert!(!metadata.is_enabled());
    }
}

#[test]
fn test_cursed_type_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Test all CURSED basic types
    let test_types = ["sus", "facts", "vibes", "tea", "void"];
    
    for type_name in &test_types {
        let result = metadata.get_or_create_cursed_type(type_name);
        assert!(result.is_ok(), "Type creation should succeed for {}", type_name);
    }
    
    // Check statistics
    assert_eq!(metadata.statistics().types_processed, test_types.len());
}

#[test]
fn test_complex_type_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Test struct types
    let struct_result = metadata.get_or_create_cursed_type("squad Person");
    assert!(struct_result.is_ok(), "Struct type creation should succeed");
    
    // Test interface types
    let interface_result = metadata.get_or_create_cursed_type("collab Drawable");
    assert!(interface_result.is_ok(), "Interface type creation should succeed");
    
    // Test unknown types (should default to pointer)
    let unknown_result = metadata.get_or_create_cursed_type("CustomType");
    assert!(unknown_result.is_ok(), "Unknown type creation should succeed");
    
    assert_eq!(metadata.statistics().types_processed, 3);
}

#[test]
fn test_function_debug_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create a test LLVM function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), context.f64_type().into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Create function declaration
    let func_decl = create_test_function_declaration("test_function", 10);
    
    // Generate debug information
    let result = metadata.generate_function_debug(function, "test_function", &func_decl);
    assert!(result.is_ok(), "Function debug generation should succeed");
    
    // Check statistics
    assert_eq!(metadata.statistics().functions_processed, 1);
    assert!(metadata.statistics().variables_processed >= 2); // Parameters
}

#[test]
fn test_variable_debug_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create a variable allocation
    let i32_type = context.i32_type();
    let alloca = builder.build_alloca(i32_type, "test_var")
        .expect("Alloca should succeed");
    
    // Create variable declaration
    let var_decl = create_test_variable_declaration("test_var", "sus", 15);
    
    // Generate debug information
    let result = metadata.generate_variable_debug("test_var", alloca, &var_decl);
    assert!(result.is_ok(), "Variable debug generation should succeed");
    
    // Check statistics
    assert_eq!(metadata.statistics().variables_processed, 1);
}

#[test]
fn test_expression_debug_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create test expression
    let expr = create_test_expression(20);
    
    // Generate debug information
    let result = metadata.generate_expression_debug(&expr, None);
    assert!(result.is_ok(), "Expression debug generation should succeed");
    
    // Check that debug location was created
    assert!(metadata.statistics().debug_locations_created > 0);
}

#[test]
fn test_source_location_mapping() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Test setting debug location
    let location = create_test_location(25, 15);
    let result = metadata.set_debug_location_from_source(&location);
    assert!(result.is_ok(), "Setting debug location should succeed");
    
    // Test creating debug location
    let file = metadata.get_or_create_file(&PathBuf::from("test.csd"));
    let debug_location = metadata.create_debug_location(30, 20, file);
    
    // Verify debug location was created (basic check)
    assert_eq!(metadata.statistics().debug_locations_created, 1);
}

#[test]
fn test_lexical_scope_management() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    let file = metadata.get_or_create_file(&PathBuf::from("test.csd"));
    
    // Enter lexical scope
    let scope_result = metadata.enter_lexical_scope(file, 10, 5);
    assert!(scope_result.is_ok(), "Entering lexical scope should succeed");
    
    // Enter nested scope
    let nested_scope_result = metadata.enter_lexical_scope(file, 15, 10);
    assert!(nested_scope_result.is_ok(), "Entering nested scope should succeed");
    
    // Exit scopes
    metadata.exit_lexical_scope();
    metadata.exit_lexical_scope();
    
    // Try to exit root scope (should warn but not fail)
    metadata.exit_lexical_scope();
}

#[test]
fn test_statement_debug_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Test variable declaration statement
    let var_stmt = Statement::VariableDeclaration {
        declaration: create_test_variable_declaration("x", "sus", 10),
        location: create_test_location(10, 1),
    };
    
    let result = metadata.generate_statement_debug(&var_stmt);
    assert!(result.is_ok(), "Statement debug generation should succeed");
    
    // Test expression statement
    let expr_stmt = Statement::Expression {
        expression: create_test_expression(15),
        location: create_test_location(15, 1),
    };
    
    let result2 = metadata.generate_statement_debug(&expr_stmt);
    assert!(result2.is_ok(), "Expression statement debug should succeed");
}

#[test]
fn test_block_statement_debug() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create block statement with nested statements
    let block_stmt = Statement::Block {
        statements: vec![
            Statement::VariableDeclaration {
                declaration: create_test_variable_declaration("x", "sus", 12),
                location: create_test_location(12, 5),
            },
            Statement::Expression {
                expression: create_test_expression(13),
                location: create_test_location(13, 5),
            },
        ],
        location: create_test_location(11, 1),
    };
    
    let result = metadata.generate_statement_debug(&block_stmt);
    assert!(result.is_ok(), "Block statement debug should succeed");
    
    // Check that debug locations were created for nested statements
    assert!(metadata.statistics().debug_locations_created > 0);
}

#[test]
fn test_control_flow_debug() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Test if statement debug
    let condition = Expression::Variable {
        name: "condition".to_string(),
        location: create_test_location(20, 10),
    };
    
    let then_branch = Box::new(Statement::Expression {
        expression: create_test_expression(21),
        location: create_test_location(21, 5),
    });
    
    let if_stmt = Statement::If {
        condition,
        then_branch,
        else_branch: None,
        location: create_test_location(20, 1),
    };
    
    let result = metadata.generate_statement_debug(&if_stmt);
    assert!(result.is_ok(), "If statement debug should succeed");
    
    // Test while statement debug
    let loop_condition = Expression::Literal {
        value: Literal::Boolean(true),
        location: create_test_location(25, 10),
    };
    
    let loop_body = Box::new(Statement::Expression {
        expression: create_test_expression(26),
        location: create_test_location(26, 5),
    });
    
    let while_stmt = Statement::While {
        condition: loop_condition,
        body: loop_body,
        location: create_test_location(25, 1),
    };
    
    let result2 = metadata.generate_statement_debug(&while_stmt);
    assert!(result2.is_ok(), "While statement debug should succeed");
}

#[test]
fn test_multiple_file_support() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("main.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create files for different source files
    let file1 = metadata.get_or_create_file(&PathBuf::from("file1.csd"));
    let file2 = metadata.get_or_create_file(&PathBuf::from("file2.csd"));
    let file3 = metadata.get_or_create_file(&PathBuf::from("file3.csd"));
    
    // Files should be cached and different
    let file1_again = metadata.get_or_create_file(&PathBuf::from("file1.csd"));
    
    // Check that file caching works
    assert_eq!(metadata.statistics().files_processed, 4); // main.csd + 3 new files
}

#[test]
fn test_debug_statistics() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    let stats = metadata.statistics();
    
    // Test initial statistics
    assert_eq!(stats.functions_processed, 0);
    assert_eq!(stats.variables_processed, 0);
    assert_eq!(stats.types_processed, 0);
    assert_eq!(stats.files_processed, 1); // The initial file
    assert_eq!(stats.debug_locations_created, 0);
    assert_eq!(stats.metadata_entries_generated, 1); // Compile unit
    
    // Test statistics display
    let display = format!("{}", stats);
    assert!(display.contains("0 functions"));
    assert!(display.contains("1 files"));
    assert!(display.contains("1 metadata entries"));
}

#[test]
fn test_line_table_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Set multiple debug locations
    let locations = vec![
        create_test_location(10, 5),
        create_test_location(15, 10),
        create_test_location(20, 15),
    ];
    
    for location in &locations {
        let _ = metadata.set_debug_location_from_source(location);
    }
    
    // Generate line table
    let line_table = metadata.generate_line_table();
    
    // Should have entries for each location set
    assert!(!line_table.is_empty());
    assert!(line_table.len() <= locations.len());
    
    // Check that line numbers are preserved
    for (line, file_path) in &line_table {
        assert!(locations.iter().any(|loc| loc.line == *line));
        assert!(file_path.contains("test.csd"));
    }
}

#[test]
fn test_stack_unwind_info_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Generate stack unwinding info
    let result = metadata.generate_stack_unwind_info("test_function");
    assert!(result.is_ok(), "Stack unwind info generation should succeed");
    
    // Check that metadata was generated
    assert!(metadata.statistics().metadata_entries_generated > 1);
}

#[test]
fn test_debug_location_management() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Set debug location
    let location = create_test_location(30, 25);
    let result = metadata.set_debug_location_from_source(&location);
    assert!(result.is_ok());
    
    // Clear debug location
    metadata.clear_debug_location();
    
    // Location should be cleared
    assert_eq!(metadata.statistics().debug_locations_created, 1);
}

#[test]
fn test_finalization() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Add some debug information
    let _ = metadata.get_or_create_cursed_type("sus");
    let _ = metadata.get_or_create_cursed_type("facts");
    
    // Finalize and check statistics
    let final_stats = metadata.finalize();
    assert!(final_stats.is_ok(), "Finalization should succeed");
    
    if let Ok(stats) = final_stats {
        assert_eq!(stats.types_processed, 2);
        assert_eq!(stats.files_processed, 1);
        assert!(stats.metadata_entries_generated > 0);
    }
}

#[test]
fn test_disabled_debug_generation() {
    let (context, module, builder) = create_test_llvm_context();
    let config = DebugConfig {
        generate_debug_info: false,
        ..Default::default()
    };
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("test.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let func_decl = create_test_function_declaration("test_function", 10);
    
    // Try to generate debug info (should fail gracefully)
    let result = metadata.generate_function_debug(function, "test_function", &func_decl);
    assert!(result.is_err(), "Debug generation should fail when disabled");
    
    // Variable debug should also fail
    let i32_type = context.i32_type();
    let alloca = builder.build_alloca(i32_type, "test_var")
        .expect("Alloca should succeed");
    let var_decl = create_test_variable_declaration("test_var", "sus", 15);
    
    let var_result = metadata.generate_variable_debug("test_var", alloca, &var_decl);
    assert!(var_result.is_err(), "Variable debug should fail when disabled");
    
    // Expression debug should succeed but do nothing
    let expr = create_test_expression(20);
    let expr_result = metadata.generate_expression_debug(&expr, None);
    assert!(expr_result.is_ok(), "Expression debug should succeed but do nothing");
}

#[test]
fn test_comprehensive_debug_workflow() {
    let (context, module, builder) = create_test_llvm_context();
    let config = create_test_debug_config();
    
    let mut metadata = LlvmDebugMetadata::new(
        &context,
        &module,
        &builder,
        Path::new("main.csd"),
        config,
    ).expect("Debug metadata creation should succeed");
    
    // Create function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("main", fn_type, None);
    let func_decl = create_test_function_declaration("main", 5);
    
    // Generate function debug
    let _ = metadata.generate_function_debug(function, "main", &func_decl);
    
    // Create basic block
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create variable
    let alloca = builder.build_alloca(i32_type, "x")
        .expect("Alloca should succeed");
    let var_decl = create_test_variable_declaration("x", "sus", 10);
    
    // Generate variable debug
    let _ = metadata.generate_variable_debug("x", alloca, &var_decl);
    
    // Set debug location and generate expression
    let location = create_test_location(15, 10);
    let _ = metadata.set_debug_location_from_source(&location);
    
    let expr = create_test_expression(15);
    let _ = metadata.generate_expression_debug(&expr, None);
    
    // Generate return
    let return_value = context.i32_type().const_int(0, false);
    let return_inst = builder.build_return(Some(&return_value))
        .expect("Return should succeed");
    
    // Finalize
    let final_stats = metadata.finalize();
    assert!(final_stats.is_ok());
    
    if let Ok(stats) = final_stats {
        assert!(stats.functions_processed >= 1);
        assert!(stats.variables_processed >= 1);
        assert!(stats.debug_locations_created >= 1);
        assert!(stats.metadata_entries_generated > 1);
    }
}
