//! Integration tests for error propagation mechanisms
//!
//! These tests validate the complete error propagation system including
//! runtime, parser, and LLVM integration components.

use cursed::error::{CursedError, SourceLocation};
use cursed::runtime::error_propagation::{
    ErrorPropagationOperator, PropagationError, NoneError, helpers,
};
use cursed::runtime::error_context::{
    ErrorContextManager, EnhancedErrorContext, FunctionCallContext,
};
use cursed::types::result::{Result as CursedResult, Option as CursedOption};
use std::time::Duration;

/// Initialize tracing for tests
#[macro_export]
macro_rules! init_test_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_basic_error_propagation_operator() {
    init_test_tracing!();
    
    let operator = ErrorPropagationOperator::new();
    
    // Test successful Result propagation
    let success_result = CursedResult::Ok(42);
    let location = SourceLocation::new(1, 5);
    
    let propagated = operator.apply_question_mark(
        success_result,
        location,
        Some("test_function".to_string()),
    );
    
    assert!(propagated.is_ok());
    assert_eq!(propagated.unwrap(), 42);
    
    // Test error Result propagation
    let error_result = CursedResult::Err("test error".to_string());
    let location = SourceLocation::new(2, 10);
    
    let propagated = operator.apply_question_mark(
        error_result,
        location,
        Some("test_function".to_string()),
    );
    
    assert!(propagated.is_err());
    let error = propagated.unwrap_err();
    assert_eq!(error.inner_error, "test error");
    assert_eq!(error.propagation_site.line, 2);
    assert_eq!(error.function_context, Some("test_function".to_string()));
}

#[test]
fn test_option_propagation() {
    init_test_tracing!();
    
    let operator = ErrorPropagationOperator::new();
    
    // Test Some propagation
    let some_option = CursedOption::Some(100);
    let location = SourceLocation::new(3, 15);
    
    let propagated = operator.apply_question_mark_option(
        some_option,
        location,
        Some("option_test".to_string()),
    );
    
    assert!(propagated.is_ok());
    assert_eq!(propagated.unwrap(), 100);
    
    // Test None propagation
    let none_option: CursedOption<i32> = CursedOption::None;
    let location = SourceLocation::new(4, 20);
    
    let propagated = operator.apply_question_mark_option(
        none_option,
        location,
        Some("option_test".to_string()),
    );
    
    assert!(propagated.is_err());
    let error = propagated.unwrap_err();
    assert_eq!(error.inner_error.message, "Option was None");
    assert_eq!(error.propagation_site.line, 4);
}

#[test]
fn test_error_context_management() {
    init_test_tracing!();
    
    let context_manager = ErrorContextManager::new();
    
    // Push function context
    let function_location = SourceLocation::new(5, 1);
    context_manager.push_function_context(
        "main".to_string(),
        function_location,
        None,
    ).unwrap();
    
    // Get call stack
    let call_stack = context_manager.get_current_call_stack().unwrap();
    assert_eq!(call_stack.len(), 1);
    assert_eq!(call_stack[0].function_name, "main");
    
    // Pop function context
    let popped = context_manager.pop_function_context().unwrap();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().function_name, "main");
    
    // Stack should be empty now
    let empty_stack = context_manager.get_current_call_stack().unwrap();
    assert!(empty_stack.is_empty());
}

#[test]
fn test_comprehensive_error_context_creation() {
    init_test_tracing!();
    
    let context_manager = ErrorContextManager::new();
    
    // Create a propagation error
    let none_error = NoneError {
        message: "Test none error".to_string(),
        location: SourceLocation::new(6, 25),
    };
    
    let propagation_error = PropagationError::new(
        none_error,
        SourceLocation::new(6, 25),
        Some("test_comprehensive".to_string()),
    );
    
    // Create comprehensive context
    let context = context_manager.create_comprehensive_context(&propagation_error, None).unwrap();
    
    assert_eq!(context.propagation_site.line, 6);
    assert_eq!(context.propagation_site.column, 25);
    assert_eq!(context.function_context, Some("test_comprehensive".to_string()));
    assert!(!context.error_id.is_empty());
    assert_eq!(context.propagation_chain.len(), 1);
}

#[test]
fn test_error_chain_tracking() {
    init_test_tracing!();
    
    let context_manager = ErrorContextManager::new();
    
    // Create multiple error contexts
    let context1 = EnhancedErrorContext {
        error_id: "error_1".to_string(),
        propagation_site: SourceLocation::new(10, 5),
        function_context: Some("func1".to_string()),
        call_stack: Vec::new(),
        source_info: None,
        error_type: "TestError1".to_string(),
        propagation_chain: Vec::new(),
        timestamp: std::time::SystemTime::now(),
        additional_info: None,
        related_errors: Vec::new(),
    };
    
    let context2 = EnhancedErrorContext {
        error_id: "error_2".to_string(),
        propagation_site: SourceLocation::new(11, 10),
        function_context: Some("func2".to_string()),
        call_stack: Vec::new(),
        source_info: None,
        error_type: "TestError2".to_string(),
        propagation_chain: Vec::new(),
        timestamp: std::time::SystemTime::now(),
        additional_info: None,
        related_errors: Vec::new(),
    };
    
    // Register contexts
    let context_id1 = context_manager.register_context(context1).unwrap();
    let context_id2 = context_manager.register_context(context2).unwrap();
    
    // Create error chain
    let chain_id = context_manager.create_error_chain(
        context_id1.clone(),
        vec![context_id2.clone()],
    ).unwrap();
    
    assert!(!chain_id.is_empty());
    assert!(chain_id.starts_with("chain_"));
}

#[test]
fn test_error_report_generation() {
    init_test_tracing!();
    
    let context_manager = ErrorContextManager::new();
    
    // Create and register an error context
    let context = EnhancedErrorContext {
        error_id: "report_test_error".to_string(),
        propagation_site: SourceLocation::new(15, 30),
        function_context: Some("report_test".to_string()),
        call_stack: Vec::new(),
        source_info: None,
        error_type: "ReportTestError".to_string(),
        propagation_chain: Vec::new(),
        timestamp: std::time::SystemTime::now(),
        additional_info: None,
        related_errors: Vec::new(),
    };
    
    let context_id = context_manager.register_context(context).unwrap();
    
    // Generate error report
    let report = context_manager.generate_error_report(&context_id).unwrap();
    
    assert_eq!(report.context.error_id, context_id);
    assert_eq!(report.context.error_type, "ReportTestError");
    assert!(!report.report_id.is_empty());
    assert!(report.related_chains.is_empty());
}

#[test]
fn test_propagation_statistics() {
    init_test_tracing!();
    
    let operator = ErrorPropagationOperator::new();
    
    // Perform several propagations
    for i in 0..5 {
        let result = if i % 2 == 0 {
            CursedResult::Ok(i)
        } else {
            CursedResult::Err(format!("error_{}", i))
        };
        
        let location = SourceLocation::new(i + 1, i * 5);
        let _ = operator.apply_question_mark(result, location, Some("stats_test".to_string()));
    }
    
    // Check statistics
    let stats = operator.get_statistics().unwrap();
    assert_eq!(stats.total_attempts, 5);
    assert_eq!(stats.successful_propagations, 3); // Even numbers: 0, 2, 4
    assert_eq!(stats.error_propagations, 2);      // Odd numbers: 1, 3
    
    // Check calculated rates
    assert!((stats.success_rate() - 0.6).abs() < 0.01); // 3/5 = 0.6
    assert!((stats.error_rate() - 0.4).abs() < 0.01);   // 2/5 = 0.4
}

#[test]
fn test_helper_functions() {
    init_test_tracing!();
    
    let operator = helpers::create_default_propagator();
    
    // Test Result helper
    let result = CursedResult::Ok("success".to_string());
    let propagated = helpers::propagate_result(&operator, result, 20, 15, Some("helper_test"));
    assert!(propagated.is_ok());
    assert_eq!(propagated.unwrap(), "success");
    
    // Test Option helper
    let option = CursedOption::Some(999);
    let propagated = helpers::propagate_option(&operator, option, 21, 20, Some("helper_test"));
    assert!(propagated.is_ok());
    assert_eq!(propagated.unwrap(), 999);
    
    // Test error conversion
    let error_result: CursedResult<i32, String> = CursedResult::Err("conversion test".to_string());
    let propagated = helpers::propagate_result(&operator, error_result, 22, 25, Some("helper_test"));
    assert!(propagated.is_err());
    
    let cursed_error = helpers::to_cursed_error(propagated.unwrap_err());
    match cursed_error {
        CursedError::ErrorPropagation { message, line, column } => {
            assert!(message.contains("conversion test"));
            assert_eq!(line, Some(22));
            assert_eq!(column, Some(25));
        }
        _ => panic!("Expected ErrorPropagation variant"),
    }
}

#[test]
fn test_error_context_stack_management() {
    init_test_tracing!();
    
    let operator = ErrorPropagationOperator::new();
    
    // Perform nested error propagations
    let location1 = SourceLocation::new(30, 1);
    let location2 = SourceLocation::new(31, 5);
    let location3 = SourceLocation::new(32, 10);
    
    // First propagation (success)
    let result1 = CursedResult::Ok(1);
    let _ = operator.apply_question_mark(result1, location1, Some("level1".to_string()));
    
    // Second propagation (error)
    let result2: CursedResult<i32, String> = CursedResult::Err("nested error".to_string());
    let _ = operator.apply_question_mark(result2, location2, Some("level2".to_string()));
    
    // Third propagation (success)
    let result3 = CursedResult::Ok(3);
    let _ = operator.apply_question_mark(result3, location3, Some("level3".to_string()));
    
    // Check context chain
    let context_chain = operator.get_error_context_chain().unwrap();
    
    // Should have contexts for all propagations (including successful ones)
    assert!(context_chain.len() >= 2); // At least error propagations are recorded
    
    // Find the error context
    let error_context = context_chain.iter()
        .find(|ctx| ctx.error_type == "Result::Err")
        .expect("Should have error context");
    
    assert_eq!(error_context.location.line, 31);
    assert_eq!(error_context.function_name, Some("level2".to_string()));
}

#[test]
fn test_source_mapping_integration() {
    init_test_tracing!();
    
    let context_manager = ErrorContextManager::new();
    
    // Add source file mapping
    let source_content = r#"
fn test_function() -> Result<i32, String> {
    let value = some_operation()?;
    Ok(value * 2)
}
"#.to_string();
    
    let file_path = std::path::PathBuf::from("test_file.csd");
    context_manager.add_source_mapping(file_path.clone(), source_content).unwrap();
    
    // Create error context that should resolve source information
    let propagation_error = PropagationError::new(
        "source mapping test error".to_string(),
        SourceLocation::new(3, 25), // Line with the ? operator
        Some("test_function".to_string()),
    );
    
    let context = context_manager.create_comprehensive_context(&propagation_error, None).unwrap();
    
    // Check if source info is available
    if let Some(ref source_info) = context.source_info {
        assert_eq!(source_info.file_path, file_path);
        // The source line should contain the code with ?
        if let Some(ref line) = source_info.source_line {
            assert!(line.contains("some_operation()"));
        }
    }
}

#[test]
fn test_error_propagation_chain_building() {
    init_test_tracing!();
    
    // Create a chain of error propagations
    let error1 = NoneError {
        message: "Initial error".to_string(),
        location: SourceLocation::new(40, 5),
    };
    
    let prop1 = PropagationError::new(
        error1,
        SourceLocation::new(40, 5),
        Some("func1".to_string()),
    );
    
    let prop2 = prop1.add_propagation_site(SourceLocation::new(41, 10));
    let prop3 = prop2.add_propagation_site(SourceLocation::new(42, 15));
    
    let full_chain = prop3.full_chain();
    assert_eq!(full_chain.len(), 3);
    assert_eq!(full_chain[0].line, 40);
    assert_eq!(full_chain[1].line, 41);
    assert_eq!(full_chain[2].line, 42);
}

#[test]
fn test_configuration_options() {
    init_test_tracing!();
    
    let mut config = cursed::runtime::error_propagation::PropagationConfig::default();
    config.max_context_depth = 50;
    config.enable_tracing = false;
    config.collect_timing = false;
    
    let operator = ErrorPropagationOperator::with_config(config);
    
    // Test that configuration is respected
    let result = CursedResult::Ok("configured test");
    let location = SourceLocation::new(50, 1);
    
    let propagated = operator.apply_question_mark(result, location, Some("config_test".to_string()));
    assert!(propagated.is_ok());
    assert_eq!(propagated.unwrap(), "configured test");
    
    // Statistics should still work even with timing collection disabled
    let stats = operator.get_statistics().unwrap();
    assert_eq!(stats.total_attempts, 1);
    assert_eq!(stats.successful_propagations, 1);
}

#[test]
fn test_concurrent_error_propagation() {
    init_test_tracing!();
    
    let operator = std::sync::Arc::new(ErrorPropagationOperator::new());
    let handles: Vec<_> = (0..4).map(|i| {
        let op = operator.clone();
        std::thread::spawn(move || {
            for j in 0..10 {
                let result = if (i + j) % 3 == 0 {
                    CursedResult::Err(format!("thread_{}_error_{}", i, j))
                } else {
                    CursedResult::Ok(i * 10 + j)
                };
                
                let location = SourceLocation::new(i + 1, j + 1);
                let _ = op.apply_question_mark(result, location, Some(format!("thread_{}", i)));
            }
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final statistics
    let stats = operator.get_statistics().unwrap();
    assert_eq!(stats.total_attempts, 40); // 4 threads * 10 operations each
    
    // Verify that error rate matches expected pattern (every 3rd operation fails)
    let expected_errors = 40 / 3 + if 40 % 3 > 0 { 1 } else { 0 }; // Approximate
    assert!(stats.error_propagations >= expected_errors - 2 && stats.error_propagations <= expected_errors + 2);
}

#[test]
fn test_performance_characteristics() {
    init_test_tracing!();
    
    let operator = ErrorPropagationOperator::new();
    let start_time = std::time::Instant::now();
    
    // Perform many operations to test performance
    for i in 0..1000 {
        let result = CursedResult::Ok(i);
        let location = SourceLocation::new(i % 100 + 1, i % 50 + 1);
        let _ = operator.apply_question_mark(result, location, Some("perf_test".to_string()));
    }
    
    let elapsed = start_time.elapsed();
    let stats = operator.get_statistics().unwrap();
    
    // Should complete 1000 operations quickly
    assert!(elapsed < Duration::from_millis(100), "Performance test took too long: {:?}", elapsed);
    assert_eq!(stats.total_attempts, 1000);
    assert_eq!(stats.successful_propagations, 1000);
    assert_eq!(stats.error_propagations, 0);
    
    // Average duration should be very small
    let avg_duration = stats.average_duration();
    assert!(avg_duration < Duration::from_micros(100), "Average operation too slow: {:?}", avg_duration);
}
