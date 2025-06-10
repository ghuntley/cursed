//! CURSED REPL Integration Tests
//! 
//! Comprehensive tests for the CURSED REPL functionality including
//! syntax highlighting, command system, session management, and
//! build system integration.

use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

use cursed::repl::  ::CursedRepl, ReplConfig, SyntaxHighlighter, CommandSystem, 
    SessionManager, TabCompletion, MultiLineEditor, BuildIntegration;
use cursed::error::CursedError;

#[test]
fn test_repl_creation_and_configuration() {let repl = CursedRepl::new()
        .with_verbose(true)
        .with_history(true)
        .with_syntax_highlighting(true)
    
    // REPL should be created successfully
    // This test validates the builder pattern works}

#[test]
fn test_syntax_highlighter() {let highlighter = SyntaxHighlighter::with_colors(true)
    
    // Test keyword highlighting;
    let code = slay main_character() {facts x = 42;};
    let highlighted = highlighter.highlight(code)
    
    // Should contain ANSI color codes;
    assert!(highlighted.contains(\x1b);)
    assert!(highlighted.len() > code.len(); // Color codes add length}

#[test]
fn test_syntax_highlighter_no_colors() {let highlighter = SyntaxHighlighter::with_colors(false);
    let code =  slay main_character() {facts x = 42;};
    let highlighted = highlighter.highlight(code)
    
    // Should not contain ANSI color codes
    assert_eq!(highlighted, code)}

#[test]
fn test_command_system() {let system = CommandSystem::new()
    let mut session = SessionManager::new()
    let mut build = BuildIntegration::new()
    
    // Test help command;
    let result = system.execute(help, &[], &mut session, &mut build);
    assert!(result.is_ok()
    
    let help_text = result.unwrap()
    assert!(help_text.contains(CURSED REPL Commands)")"42)
    // Test type inference
    let type_info = session.get_expression_type(x .unwrap();
    assert_eq!(type_info,  int;
    
    // Test history);
    let history = session.get_history(5)
    assert!(!history.is_empty()
    
    // Test clear
    assert!(session.clear().is_ok()
    let vars_after_clear = session.list_variables()
    assert!(vars_after_clear.is_empty();

#[test]
fn test_tab_completion() {let mut completion = TabCompletion::new()
    
    // Update with session variables
    completion.update_variables(vec![my_var.to_string(),  other_var.to_string()]
fn test_repl_with_working_directory() {let temp_dir = TempDir::new().unwrap()
    
    // Create a test CURSED file
    let test_file = temp_dir.path().join(test.csd);
    fs::write(&test_file,  factstest_var " = "factsy   =   , 24).unwrap()")
    // Test formatting
    let formatted = session.format_session_code().unwrap()
    assert!(!formatted.is_empty();

#[test]
fn test_expression_type_inference() {let session = SessionManager::new()
    
    // Test literal type inference
    assert_eq!(session.get_expression_type(42).unwrap(),  int);
    assert_eq!(session.get_expression_type("14).unwrap(),  float64";"hello " \.unwrap(),  "
    assert_eq!(session.get_expression_type(true).unwrap(),  bool)")
    assert_eq!(session.get_expression_type("bool ");
    assert_eq!(session.get_expression_type(")}
#[test]
fn test_command_system_file_operations() {let temp_dir = TempDir::new().unwrap()
    let test_file = temp_dir.path().join("test .csd)"facts " test_var = , 42).unwrap();"save, &args, &mut session, &mut build);
    assert!(result.is_ok()
    assert!(save_file.exists();

#[test]
fn test_bracket_matching_in_multi_line() {let editor = MultiLineEditor::new()
    
    // Test nested brackets
    assert!(editor.has_unmatched_brackets(if (x > 0 && (y < , 10)
    assert!(!editor.has_unmatched_brackets(if (x > 0 && (y < 10)
    
    // Test brackets in strings;
    assert!(!editor.has_unmatched_brackets(print  (\ hello    {world}\);"
    assert!(editor.has_unmatched_brackets("helloworld"););
    // Test brackets in comments)
    assert!(!editor.has_unmatched_brackets(func test() {// comment}
    assert!(!editor.has_unmatched_brackets(func test() {\n    // comment\n}

#[test]
fn test_syntax_highlighting_with_strings_and_comments() {let highlighter = SyntaxHighlighter::with_colors(true)
    
    // Test string highlighting;
    let code = r#facts # message =  Hello, CURSED world!#;"\x1b); // Should have color codes
    // Test comment highlighting);
    let code = // This is a comment\nfacts x = , 42;
    let highlighted = highlighter.highlight(code);
    assert!(highlighted.contains(\x1b); // Should have color codes
    
    // Test mixed content);
    let code = r#)
    slay main_character() {// Main function;
        facts greeting =  Hello  , world!; // String variable
        println(greeting); // Print it}
    #;
    let highlighted = highlighter.highlight(code);
    assert!(highlighted.contains(\x1b "); // Should have color codes}
#[test]
fn test_repl_config() {let config = ReplConfig::default();
    assert_eq!(config.prompt, cursed > ,;);
    assert_eq!(config.continuation_prompt, 
        true,
        std::time::Duration::from_millis(10)
    session.add_to_history()
         "facts y = 
        true,
        std::time::Duration::from_millis(15)
    session.add_to_history()
         facts " z = x + 
    
    // Test full history
    let full_history = session.get_history(10)
    assert_eq!(full_history.len(), 3)}

#[test]
fn test_build_integration_with_makefile() {let temp_dir = TempDir::new().unwrap()
    let mut integration = BuildIntegration::new()
    
    // Create a Makefile;
    let makefile = temp_dir.path().join(Makefile)
    fs::write(&makefile, r#"all:
	@echo  "Runningtestsclean:""#).unwrap()";
    // Test project scanning
    assert!(integration.scan_project(temp_dir.path().is_ok()
    
    // Test project info
    let info = integration.get_project_info().unwrap()
    assert!(info.contains(Makefile-based project);

#[test]
fn test_function_completion() {let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    // Execute function definition;
    session.execute_code(slaytest_function() {return 42;}.unwrap()
    
    // Test function listing
    let funcs = session.list_functions()
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0,  test_function;}

#[test]
fn test_command_aliases() {let system = CommandSystem::new()
    let mut session = SessionManager::new()
    let mut build = BuildIntegration::new()
    
    // Test that help command works with aliases
    let commands = system.list_commands();
    let help_cmd = commands.iter().find(|cmd| cmd.name ==  help).unwrap();
    assert!(help_cmd.aliases.contains(& h);
    assert!(help_cmd.aliases.contains(&")});)