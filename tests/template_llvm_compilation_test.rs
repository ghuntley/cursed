/// Template LLVM Compilation Test
/// 
/// Tests the LLVM code generation for the CURSED template system.
/// This test validates that templates can be compiled to LLVM IR correctly.

use std::collections::HashMap;
use std::sync::Arc;
use tracing_test::traced_test;

use cursed::ast::expressions::Expression;
use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmType, LlvmValue};
use cursed::codegen::llvm::template::{
    LlvmTemplateCompiler, TemplateCompilationContext, TemplateCompiler,
    TemplateOptimizationLevel, declare_template_runtime_functions,
    register_standard_filters
};
use cursed::object::Object as CursedObject;
use cursed::stdlib::template::{
    TemplateAst, TemplateNode, TemplateExpression, TemplateConfig,
    SecurityLevel, OutputFormat, FilterCall
};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[traced_test]
#[test]
fn test_template_literal_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let context = TemplateCompilationContext::default();
    
    // Test simple literal compilation
    let result = compiler.compile_template_literal("Hello, World!", &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    assert!(llvm_value.is_constant);
}

#[traced_test]
#[test] 
fn test_template_expression_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let mut context = TemplateCompilationContext::default();
    
    // Add a variable to the context
    context.add_variable("name".to_string(), LlvmType::String);
    
    // Test variable expression compilation
    let expr = TemplateExpression::Variable("name".to_string());
    let result = compiler.compile_template_expression(&expr, &context);
    assert!(result.is_ok());
    
    let llvm_value = result.unwrap();
    assert_eq!(llvm_value.value_type, LlvmType::String);
    assert!(!llvm_value.is_constant);
}

#[traced_test]
#[test]
fn test_template_literal_expression_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let context = TemplateCompilationContext::default();
    
    // Test string literal
    let string_expr = TemplateExpression::Literal(CursedObject::String("test".to_string()));
    let result = compiler.compile_template_expression(&string_expr, &context);
    assert!(result.is_ok());
    
    // Test integer literal
    let int_expr = TemplateExpression::Literal(CursedObject::Integer(42));
    let result = compiler.compile_template_expression(&int_expr, &context);
    assert!(result.is_ok());
    
    // Test boolean literal
    let bool_expr = TemplateExpression::Literal(CursedObject::Boolean(true));
    let result = compiler.compile_template_expression(&bool_expr, &context);
    assert!(result.is_ok());
}

#[traced_test]
#[test]
fn test_template_filter_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let mut context = TemplateCompilationContext::default();
    
    // Register a filter
    context.register_filter("upper".to_string(), "cursed_filter_upper".to_string());
    
    // Create a filter call
    let filter = FilterCall {
        name: "upper".to_string(),
        arguments: vec![],
    };
    
    let input_value = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%test_input".to_string(),
        is_constant: false,
    };
    
    let result = compiler.compile_template_filter(&filter, input_value, &context);
    assert!(result.is_ok());
}

#[traced_test]
#[test]
fn test_template_conditional_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let mut context = TemplateCompilationContext::default();
    
    // Add a boolean variable
    context.add_variable("show_content".to_string(), LlvmType::Boolean);
    
    // Create conditional expression
    let condition = TemplateExpression::Variable("show_content".to_string());
    let then_branch = vec![TemplateNode::Text("Content is shown".to_string())];
    let else_branch = vec![TemplateNode::Text("Content is hidden".to_string())];
    
    let result = compiler.compile_template_conditional(&condition, &then_branch, Some(&else_branch), &context);
    assert!(result.is_ok());
}

#[traced_test]
#[test]
fn test_template_loop_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let mut context = TemplateCompilationContext::default();
    
    // Add array variable
    context.add_variable("items".to_string(), LlvmType::Array);
    
    // Create loop
    let iterator = TemplateExpression::Variable("items".to_string());
    let body = vec![TemplateNode::Text("Item content".to_string())];
    
    let result = compiler.compile_template_loop("item", &iterator, &body, &context);
    assert!(result.is_ok());
}

#[traced_test]
#[test]
fn test_complete_template_compilation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    let mut context = TemplateCompilationContext::new("test_template".to_string(), TemplateConfig::default());
    
    // Add variables
    context.add_variable("title".to_string(), LlvmType::String);
    context.add_variable("items".to_string(), LlvmType::Array);
    
    // Register filters
    register_standard_filters(&mut context);
    
    // Create a simple template AST
    let ast = TemplateAst {
        nodes: vec![
            TemplateNode::Text("<h1>".to_string()),
            TemplateNode::Variable {
                expression: TemplateExpression::Variable("title".to_string()),
                filters: vec![],
                location: None,
            },
            TemplateNode::Text("</h1>".to_string()),
        ],
    };
    
    let result = compiler.compile_template(&ast, &context);
    assert!(result.is_ok());
    
    let compiled = result.unwrap();
    assert_eq!(compiled.name, "test_template");
    assert!(!compiled.metadata.required_variables.is_empty());
}

#[traced_test]
#[test]
fn test_template_compilation_context() {
    init_tracing!();
    
    let mut context = TemplateCompilationContext::new("test".to_string(), TemplateConfig::default());
    context.add_variable("user".to_string(), LlvmType::String);
    context.register_filter("escape".to_string(), "cursed_filter_escape".to_string());
    
    assert_eq!(context.template_name, "test");
    assert_eq!(context.scope_depth, 0);
    assert!(context.get_variable_type("user").is_some());
    assert!(context.get_variable_type("unknown").is_none());
    assert!(context.get_filter_function("escape").is_some());
    assert!(context.get_filter_function("unknown").is_none());
}

#[traced_test]
#[test]
fn test_child_scope_creation() {
    init_tracing!();
    
    let parent = TemplateCompilationContext::default();
    let child = parent.create_child_scope();
    
    assert_eq!(child.scope_depth, parent.scope_depth + 1);
    assert_eq!(child.template_name, parent.template_name);
    assert_eq!(child.optimization_level, parent.optimization_level);
}

#[traced_test]
#[test]
fn test_template_hash_calculation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let compiler = LlvmTemplateCompiler::new(generator);
    
    let mut context1 = TemplateCompilationContext::new("template1".to_string(), TemplateConfig::default());
    let mut context2 = TemplateCompilationContext::new("template2".to_string(), TemplateConfig::default());
    
    context1.add_variable("var1".to_string(), LlvmType::String);
    context2.add_variable("var2".to_string(), LlvmType::String);
    
    let ast1 = TemplateAst { nodes: vec![TemplateNode::Text("test".to_string())] };
    let ast2 = TemplateAst { nodes: vec![TemplateNode::Text("test2".to_string())] };
    
    let hash1 = compiler.calculate_template_hash(&ast1, &context1);
    let hash2 = compiler.calculate_template_hash(&ast2, &context2);
    
    assert_ne!(hash1, hash2); // Different templates should have different hashes
}

#[traced_test]
#[test]
fn test_performance_hints_generation() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let compiler = LlvmTemplateCompiler::new(generator);
    
    let mut context = TemplateCompilationContext::default();
    context.optimization_level = TemplateOptimizationLevel::None;
    context.security_level = SecurityLevel::Relaxed;
    context.output_format = OutputFormat::Html;
    
    // Add many variables to trigger hints
    for i in 0..25 {
        context.add_variable(format!("var{}", i), LlvmType::String);
    }
    
    let hints = compiler.generate_performance_hints(&context);
    assert!(!hints.is_empty());
    assert!(hints.iter().any(|h| h.contains("reducing the number of template variables")));
    assert!(hints.iter().any(|h| h.contains("Enable optimization")));
    assert!(hints.iter().any(|h| h.contains("strict security level")));
}

#[traced_test]
#[test]
fn test_template_cache_functionality() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    
    assert_eq!(compiler.cache_size(), 0);
    
    let context = TemplateCompilationContext::new("cached_template".to_string(), TemplateConfig::default());
    let ast = TemplateAst { nodes: vec![TemplateNode::Text("cached content".to_string())] };
    
    // First compilation - cache miss
    let result1 = compiler.compile_template(&ast, &context);
    assert!(result1.is_ok());
    assert_eq!(compiler.cache_size(), 1);
    assert_eq!(compiler.get_stats().cache_misses, 1);
    assert_eq!(compiler.get_stats().cache_hits, 0);
    
    // Second compilation - cache hit
    let result2 = compiler.compile_template(&ast, &context);
    assert!(result2.is_ok());
    assert_eq!(compiler.cache_size(), 1);
    assert_eq!(compiler.get_stats().cache_hits, 1);
    
    // Clear cache
    compiler.clear_cache();
    assert_eq!(compiler.cache_size(), 0);
}

#[traced_test]
#[test]
fn test_runtime_function_declarations() {
    init_tracing!();
    
    use cursed::codegen::llvm::{DummyModule, DummyType};
    
    let module = DummyModule::new();
    let result = declare_template_runtime_functions(&module);
    assert!(result.is_ok());
    
    let functions = result.unwrap();
    assert!(!functions.is_empty());
    
    // Check that essential functions are declared
    assert!(functions.contains_key("escape_html"));
    assert!(functions.contains_key("concat_strings"));
    assert!(functions.contains_key("get_variable"));
    assert!(functions.contains_key("set_variable"));
    assert!(functions.contains_key("values_equal"));
}

#[traced_test]
#[test]
fn test_standard_filter_registration() {
    init_tracing!();
    
    let mut context = TemplateCompilationContext::default();
    register_standard_filters(&mut context);
    
    assert!(!context.filters.is_empty());
    assert!(context.get_filter_function("upper").is_some());
    assert!(context.get_filter_function("lower").is_some());
    assert!(context.get_filter_function("escape").is_some());
    assert!(context.get_filter_function("join").is_some());
}

#[traced_test]
#[test]
fn test_template_compilation_stats() {
    init_tracing!();
    
    let generator = Arc::new(LlvmCodeGenerator::new());
    let mut compiler = LlvmTemplateCompiler::new(generator);
    
    let stats = compiler.get_stats();
    assert_eq!(stats.templates_compiled, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
    
    let context = TemplateCompilationContext::new("stats_test".to_string(), TemplateConfig::default());
    let ast = TemplateAst { nodes: vec![TemplateNode::Text("test".to_string())] };
    
    let _ = compiler.compile_template(&ast, &context);
    
    let updated_stats = compiler.get_stats();
    assert_eq!(updated_stats.templates_compiled, 1);
    assert_eq!(updated_stats.cache_misses, 1);
}
