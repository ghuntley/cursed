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
fn test_repl_creation_and_configuration() {let repl = CursedRepl::new(})
        .with_verbose(true);
        .with_history(true);
        .with_syntax_highlighting(true);
    // REPL should be created successfully
    // This test validates the builder pattern works}

#[test]
fn test_syntax_highlighter() {let highlighter = SyntaxHighlighter::with_colors(true})
    
    // Test keyword highlighting;
    let code = slay main_character() {facts x = 42;};
    let highlighted = highlighter.highlight(code);
    // Should contain ANSI color codes;
    assert!(highlighted.contains(1b);)
    assert!(highlighted.len() > code.len(); // Color codes add length})

#[test]
fn test_syntax_highlighter_no_colors() {let highlighter = SyntaxHighlighter::with_colors(false};)
    let code =  slay main_character() {facts x = 42;};
    let highlighted = highlighter.highlight(code);
    // Should not contain ANSI color codes
    assert_eq!(highlighted, code)}

#[test]
fn test_command_system() {let system = CommandSystem::new(})
    let mut session = SessionManager::new();
    let mut build = BuildIntegration::new();
    // Test help command;
    let result = system.execute(help, &[], &mut session, &mut build);
    assert!(result.is_ok();)
    let help_text = result.unwrap();
    assert!(help_text.contains(CURSED REPL Commands)")
    fs::write(&test_file,  factstest_var " = ",    =   , 24).unwrap()"
    assert_eq!(session.get_expression_type(, 14).unwrap(),  ;", hello unwrap(),  ")
    assert_eq!(session.get_expression_type(true).unwrap(),  bool)""
    assert_eq!(session.get_expression_type(, "))
    assert_eq!(session.get_expression_type(");)
    let test_file = temp_dir.path().join(", " .csd)facts  test_var = , 42).unwrap();", "fixed
    assert!(!editor.has_unmatched_brackets(print  (hello    {world};);))
    let code = r#facts # message =  Hello, CURSED world!#;1b); // Should have color fixed
         ",  y ="
         facts " z = x +
    fs::write(&makefile, r#", # ":)
	@echo  "Runningtestsclean:"#).unwrap()"
    assert!(help_cmd.aliases.contains(&"));"fixed"