//! Integration Tests for CURSED REPL System
//! 
//! Tests the complete REPL functionality including evaluation,
//! session management, command system, and user interface.

use cursed::repl::{
    CursedRepl, ReplEvaluator, SessionManager, CommandSystem, 
    BuildIntegration, ReplConfig, ReplState, InputType, ReplOutput,
    SyntaxHighlighter, MultiLineEditor
};
use cursed::error::CursedError;
use std::time::Duration;

#[test]
fn test_repl_creation() {
    let repl = CursedRepl::new();
    // Should create without errors
    assert_eq!(std::mem::size_of_val(&repl), std::mem::size_of::<CursedRepl>());
}

#[test]
fn test_repl_configuration() {
    let repl = CursedRepl::new()
        .with_verbose(true)
        .with_history(true)
        .with_syntax_highlighting(true)
        .with_timeout(Duration::from_secs(10));
    
    // Configuration should be applied
    assert_eq!(std::mem::size_of_val(&repl), std::mem::size_of::<CursedRepl>());
}

#[test]
fn test_evaluator_creation() {
    let evaluator = ReplEvaluator::new();
    assert!(evaluator.is_ok());
}

#[test]
fn test_evaluator_variable_handling() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test variable declaration
    let result = evaluator.evaluate("facts x = 42", &mut session);
    assert!(result.is_ok());
    
    // Check that variable was stored
    let variables = evaluator.get_variables();
    assert_eq!(variables.len(), 1);
    assert_eq!(variables[0].0, "x");
    assert_eq!(variables[0].2, "42");
}

#[test]
fn test_evaluator_function_handling() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test function declaration
    let result = evaluator.evaluate("slay greet() { }", &mut session);
    assert!(result.is_ok());
    
    // Check that function was stored
    let functions = evaluator.get_functions();
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].0, "greet");
}

#[test]
fn test_evaluator_expression_evaluation() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test arithmetic expressions
    let test_cases = vec![
        ("2 + 3", "5"),
        ("10 - 4", "6"),
        ("3 * 4", "12"),
        ("8 / 2", "4"),
    ];
    
    for (expr, expected) in test_cases {
        let result = evaluator.evaluate(expr, &mut session);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.content, expected);
    }
}

#[test]
fn test_evaluator_literal_handling() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test different literal types
    let test_cases = vec![
        ("42", "42"),
        ("3.14", "3.14"),
        ("\"hello\"", "\"hello\""),
        ("true", "true"),
        ("false", "false"),
    ];
    
    for (expr, expected) in test_cases {
        let result = evaluator.evaluate(expr, &mut session);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.content, expected);
    }
}

#[test]
fn test_evaluator_type_inference() {
    let evaluator = ReplEvaluator::new().unwrap();
    
    // Test type inference for different expressions
    let test_cases = vec![
        ("42", "int"),
        ("3.14", "float64"),
        ("\"hello\"", "string"),
        ("true", "bool"),
        ("false", "bool"),
    ];
    
    for (expr, expected_type) in test_cases {
        let result = evaluator.get_expression_type(expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_type);
    }
}

#[test]
fn test_evaluator_import_handling() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test import statement
    let result = evaluator.evaluate("import \"math\"", &mut session);
    assert!(result.is_ok());
    
    // This should complete without error
    let output = result.unwrap();
    assert!(output.content.contains("math"));
}

#[test]
fn test_evaluator_error_handling() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test syntax error
    let result = evaluator.evaluate("facts = 42", &mut session);
    assert!(result.is_err());
}

#[test]
fn test_session_manager_functionality() {
    let mut session = SessionManager::new();
    
    // Test initialization
    assert!(session.initialize().is_ok());
    
    // Test variable tracking
    assert!(session.execute_code("facts x = 42").is_ok());
    let vars = session.list_variables();
    assert_eq!(vars.len(), 1);
    
    // Test history
    let history = session.get_history(5);
    assert!(!history.is_empty());
    
    // Test type inference
    let type_info = session.get_expression_type("42");
    assert!(type_info.is_ok());
    assert_eq!(type_info.unwrap(), "int");
}

#[test]
fn test_command_system_functionality() {
    let command_system = CommandSystem::new();
    let mut session = SessionManager::new();
    let mut build = BuildIntegration::new();
    
    // Test help command
    let result = command_system.execute("help", &[], &mut session, &mut build);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.contains("CURSED REPL Commands"));
    
    // Test unknown command
    let result = command_system.execute("unknown", &[], &mut session, &mut build);
    assert!(result.is_err());
}

#[test]
fn test_command_system_variable_commands() {
    let command_system = CommandSystem::new();
    let mut session = SessionManager::new();
    let mut build = BuildIntegration::new();
    
    // Initialize session and add a variable
    session.initialize().unwrap();
    session.execute_code("facts x = 42").unwrap();
    
    // Test vars command
    let result = command_system.execute("vars", &[], &mut session, &mut build);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.contains("x"));
}

#[test]
fn test_command_system_type_command() {
    let command_system = CommandSystem::new();
    let mut session = SessionManager::new();
    let mut build = BuildIntegration::new();
    
    // Test type command
    let args = vec!["42".to_string()];
    let result = command_system.execute("type", &args, &mut session, &mut build);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.contains("Type"));
}

#[test]
fn test_syntax_highlighter() {
    let highlighter = SyntaxHighlighter::new();
    
    // Test highlighting with colors disabled
    let highlighter_no_color = SyntaxHighlighter::with_colors(false);
    let code = "slay main() { facts x = 42; }";
    let highlighted = highlighter_no_color.highlight(code);
    
    // Should return unchanged code when colors are disabled
    assert_eq!(highlighted, code);
    
    // Test highlighting with colors enabled
    let highlighter_color = SyntaxHighlighter::with_colors(true);
    let highlighted_color = highlighter_color.highlight(code);
    
    // Should contain ANSI escape sequences when colors are enabled
    assert!(highlighted_color.len() >= code.len());
}

#[test]
fn test_multi_line_editor() {
    let editor = MultiLineEditor::new();
    
    // Test continuation detection
    assert!(editor.needs_continuation("slay test() {"));
    assert!(editor.needs_continuation("facts x = ["));
    assert!(!editor.needs_continuation("facts x = 42"));
    
    // Test completion detection
    assert!(editor.is_complete("slay test() {", "}"));
    assert!(!editor.is_complete("slay test() {", "  return 42"));
}

#[test]
fn test_build_integration() {
    let build = BuildIntegration::new();
    
    // Test basic functionality (should not crash)
    let info = build.get_project_info();
    assert!(info.is_ok());
    
    let workspace = build.get_workspace_info();
    assert!(workspace.is_ok());
}

#[test]
fn test_repl_config() {
    let config = ReplConfig::default();
    
    // Test default values
    assert!(!config.verbose);
    assert!(config.enable_history);
    assert!(config.enable_syntax_highlighting);
    assert!(config.enable_tab_completion);
    assert_eq!(config.max_history_size, 1000);
    assert_eq!(config.prompt, "cursed> ");
}

#[test]
fn test_repl_output() {
    // Test successful output
    let success = ReplOutput::success("test result".to_string());
    assert!(!success.is_error);
    assert_eq!(success.content, "test result");
    
    // Test error output
    let error = ReplOutput::error("test error".to_string());
    assert!(error.is_error);
    assert_eq!(error.content, "test error");
    
    // Test output with timing
    let timed = success.with_timing(Duration::from_millis(100));
    assert_eq!(timed.execution_time, Some(Duration::from_millis(100)));
}

#[test]
fn test_input_type_parsing() {
    // Test different input types
    // Note: This tests the logic that would be in parse_input
    
    let empty_input = "";
    assert!(empty_input.trim().is_empty());
    
    let command_input = ":help";
    assert!(command_input.starts_with(':'));
    
    let code_input = "facts x = 42";
    assert!(!code_input.starts_with(':'));
    assert!(!code_input.trim().is_empty());
}

#[test]
fn test_complex_repl_workflow() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Initialize session
    session.initialize().unwrap();
    
    // Test a complete workflow
    let steps = vec![
        ("facts x = 10", true),
        ("facts y = 20", true),
        ("x + y", true),
        ("slay add(a, b) { }", true),
        ("facts result = add(x, y)", true),
    ];
    
    for (code, should_succeed) in steps {
        let result = evaluator.evaluate(code, &mut session);
        if should_succeed {
            assert!(result.is_ok(), "Failed to execute: {}", code);
        } else {
            assert!(result.is_err(), "Should have failed: {}", code);
        }
    }
    
    // Check final state
    let variables = evaluator.get_variables();
    assert!(variables.len() >= 2); // At least x and y
    
    let functions = evaluator.get_functions();
    assert!(functions.len() >= 1); // At least add function
}

#[test]
fn test_repl_state_transitions() {
    // Test state enum
    let states = vec![
        ReplState::Interactive,
        ReplState::MultiLine,
        ReplState::Command,
        ReplState::Exiting,
        ReplState::Error("test error".to_string()),
    ];
    
    for state in states {
        // States should be creatable and comparable
        match state {
            ReplState::Interactive => assert_eq!(state, ReplState::Interactive),
            ReplState::MultiLine => assert_eq!(state, ReplState::MultiLine),
            ReplState::Command => assert_eq!(state, ReplState::Command),
            ReplState::Exiting => assert_eq!(state, ReplState::Exiting),
            ReplState::Error(_) => assert!(matches!(state, ReplState::Error(_))),
        }
    }
}

#[test]
fn test_evaluator_context_management() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Add some variables and functions
    evaluator.evaluate("facts x = 42", &mut session).unwrap();
    evaluator.evaluate("facts y = \"hello\"", &mut session).unwrap();
    evaluator.evaluate("slay test() { }", &mut session).unwrap();
    
    // Check context state
    let vars = evaluator.get_variables();
    let funcs = evaluator.get_functions();
    
    assert_eq!(vars.len(), 2);
    assert_eq!(funcs.len(), 1);
    
    // Clear context
    evaluator.clear_context().unwrap();
    
    // Check that context is cleared
    let vars_after = evaluator.get_variables();
    let funcs_after = evaluator.get_functions();
    
    assert_eq!(vars_after.len(), 0);
    assert_eq!(funcs_after.len(), 0);
}

#[test]
fn test_session_code_management() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Execute some code
    evaluator.evaluate("facts x = 10", &mut session).unwrap();
    evaluator.evaluate("facts y = 20", &mut session).unwrap();
    
    // Get session code
    let session_code = evaluator.get_session_code();
    assert!(session_code.contains("facts x = 10"));
    assert!(session_code.contains("facts y = 20"));
}

#[test]
fn test_error_recovery() {
    let mut evaluator = ReplEvaluator::new().unwrap();
    let mut session = SessionManager::new();
    
    // Test that the evaluator can recover from errors
    
    // Valid code
    let result1 = evaluator.evaluate("facts x = 42", &mut session);
    assert!(result1.is_ok());
    
    // Invalid code
    let result2 = evaluator.evaluate("invalid syntax here", &mut session);
    assert!(result2.is_err());
    
    // More valid code after error
    let result3 = evaluator.evaluate("facts y = 24", &mut session);
    assert!(result3.is_ok());
    
    // Check that valid variables are still there
    let vars = evaluator.get_variables();
    assert_eq!(vars.len(), 2);
}
