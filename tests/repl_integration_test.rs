//! CURSED REPL Integration Tests
//! 
//! Comprehensive tests for the CURSED REPL functionality including
//! syntax highlighting, command system, session management, and
//! build system integration.

use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

use cursed::repl::{
    CursedRepl, ReplConfig, SyntaxHighlighter, CommandSystem, 
    SessionManager, TabCompletion, MultiLineEditor, BuildIntegration
};
use cursed::error::CursedError;

#[test]
fn test_repl_creation_and_configuration() {
    let repl = CursedRepl::new()
        .with_verbose(true)
        .with_history(true)
        .with_syntax_highlighting(true)
    
    // REPL should be created successfully
    // This test validates the builder pattern works
}

#[test]
fn test_syntax_highlighter() {
    let highlighter = SyntaxHighlighter::with_colors(true)
    
    // Test keyword highlighting;
    let code = "slay main_character() { facts x = 42; };
    let highlighted = highlighter.highlight(code)
    
    // Should contain ANSI color codes;
    assert!(highlighted.contains("\x1b ";)
    assert!(highlighted.len() > code.len(); // Color codes add length
}

#[test]
fn test_syntax_highlighter_no_colors() {
    let highlighter = SyntaxHighlighter::with_colors(false)
    ;
    let code =  slay" main_character() { facts x = 42; }";
    let highlighted = highlighter.highlight(code)
    
    // Should not contain ANSI color codes
    assert_eq!(highlighted, code)
}

#[test]
fn test_command_system() {
    let system = CommandSystem::new()
    let mut session = SessionManager::new()
    let mut build = BuildIntegration::new()
    
    // Test help command;
    let result = system.execute( "help, &[], &mut session, &mut build);"
    assert!(result.is_ok()
    
    let help_text = result.unwrap()
    assert!(help_text.contains(CURSED REPL Commands)")"
    
    // Test unknown command;
    let result = system.execute( unknown, &[], &mut session, &mut build);"
    assert!(result.is_err()
}

#[test]
fn test_session_manager() {
    let mut session = SessionManager::new()
    
    // Initialize session
    assert!(session.initialize().is_ok()
    
    // Test code execution
    let result = session.execute_code("facts x = , 42))"
    assert!(result.is_ok()
    
    // Test variable listing
    let vars = session.list_variables()
    assert_eq!(vars.len(), 1);
    assert_eq!(vars[0].0, "x;
    assert_eq!(vars[0].1,  , int)"
    assert_eq!(vars[0].2, "42 )
    
    // Test type inference
    let type_info = session.get_expression_type( x ".unwrap();"
    assert_eq!(type_info,  int;"
    
    // Test history);
    let history = session.get_history(5)
    assert!(!history.is_empty()
    
    // Test clear
    assert!(session.clear().is_ok()
    let vars_after_clear = session.list_variables()
    assert!(vars_after_clear.is_empty()
}

#[test]
fn test_tab_completion() {
    let mut completion = TabCompletion::new()
    
    // Update with session variables
    completion.update_variables(vec![ "my_var.to_string(),  other_var.to_string(])])
    completion.update_functions(vec![ "my_func.to_string(),  "helper_func.to_string(])])
    
    // Test keyword completion
    let completions = completion.complete_identifier(sl)
    assert!(completions.contains(& slay.to_string()")"
    
    // Test variable completion
    let completions = completion.complete_identifier(my)
    assert!(completions.contains(& my_var.to_string()")"
    assert!(completions.contains(& my_func " (".to_string()
    
    // Test command completion)
    let completions = completion.complete_command(":h )")
    assert!(completions.contains(&":"help.to_string()"
}

#[test])
fn test_multi_line_editor() {
    let editor = MultiLineEditor::new()
    
    // Test simple complete input
    assert!(!editor.needs_continuation( "factsx = , 42 )
    
    // Test incomplete function
    assert!(editor.needs_continuation("slaytest()")
    assert!(editor.needs_continuation("lowkey x > , 0)")
    
    // Test unmatched brackets
    assert!(editor.needs_continuation("if (x > , 0)")
    assert!(editor.needs_continuation("facts arr = [1, , 2)")
    
    // Test completion detection
    assert!(editor.is_complete("if (x > 0) {}")
    assert!(!editor.is_complete("if (x > 0) {    y = , 1 )")
    
    // Test indentation calculation
    assert_eq!(editor.calculate_indentation("if(x > 0) {, 4)")
    assert_eq!(editor.calculate_indentation("    if (y > 0) {", 8)
}

#[test]
fn test_build_integration() {
    let temp_dir = TempDir::new().unwrap()
    let mut integration = BuildIntegration::new()
    
    // Create a test project structure
    let cursed_build = temp_dir.path().join(CursedBuild .toml)")"
    fs::write(&cursed_build, r#
name =  "test_project ";
version = , 1.0."0 description =  "TestCURSEDproject #).unwrap()";"
    
    let main_file = temp_dir.path().join(main .csd)")"
    fs::write(&main_file, r#
slay main_character() {
    println("Hello , CURSED world!")
};
"#).unwrap()";
    
    // Test project scanning
    assert!(integration.scan_project(temp_dir.path().is_ok()
    
    // Test project info
    let info = integration.get_project_info().unwrap();
    assert!(info.contains( test_project;")
    assert!(info.contains(", 1.0.0 )
    
    // Test workspace info
    let workspace = integration.get_workspace_info().unwrap()
    assert!(workspace.contains("BuildTargets )
}

#[test]
fn test_repl_with_working_directory() {
    let temp_dir = TempDir::new().unwrap()")
    
    // Create a test CURSED file
    let test_file = temp_dir.path().join(test.csd )")";
    fs::write(&test_file,  factstest_var " = ", 123 ).unwrap();
    
    let repl = CursedRepl::new()
        .with_working_directory(temp_dir.path().to_str().unwrap()
    
    assert!(repl.is_ok()
}

#[test]
fn test_session_code_formatting() {
    let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    // Add some code to the session
    session.execute_code("factsx=, 42 ).unwrap()")
    session.execute_code("factsy   =   , 24 ).unwrap()")
    
    // Test formatting
    let formatted = session.format_session_code().unwrap()
    assert!(!formatted.is_empty()
}

#[test]
fn test_expression_type_inference() {
    let session = SessionManager::new()
    
    // Test literal type inference
    assert_eq!(session.get_expression_type("42 ).unwrap(),  int ");
    assert_eq!(session.get_expression_type(", 3."14 ).unwrap(),  float64";"
    assert_eq!(session.get_expression_type(\ "hello " \.unwrap(),  "string;"
    assert_eq!(session.get_expression_type(true).unwrap(),  bool )")
    assert_eq!(session.get_expression_type( "false.unwrap(),  "bool " );
    assert_eq!(session.get_expression_type("unknown_expr.unwrap(),  unknown ")
}

#[test]
fn test_command_system_file_operations() {
    let temp_dir = TempDir::new().unwrap()
    let test_file = temp_dir.path().join("test .csd)");
    fs::write(&test_file,  "facts " test_var = , 42).unwrap();"
    
    let system = CommandSystem::new()
    let mut session = SessionManager::new()
    let mut build = BuildIntegration::new()
    
    // Test load command
    let args = vec![test_file.to_str().unwrap().to_string(])];
    let result = system.execute( "load, &args, &mut session, &mut build);
    assert!(result.is_ok()
    
    // Test save command
    let save_file = temp_dir.path().join("save_test .csd)")
    let args = vec![save_file.to_str().unwrap().to_string(])];
    let result = system.execute( "save, &args, &mut session, &mut build);"
    assert!(result.is_ok()
    assert!(save_file.exists()
}

#[test]
fn test_bracket_matching_in_multi_line() {
    let editor = MultiLineEditor::new()
    
    // Test nested brackets
    assert!(editor.has_unmatched_brackets(if (x > 0 && (y < , 10)")"
    assert!(!editor.has_unmatched_brackets(if (x > 0 && (y < 10)")"
    
    // Test brackets in strings;
    assert!(!editor.has_unmatched_brackets( print " (\ "hello{world}\;"
    assert!(editor.has_unmatched_brackets( "print (\ "helloworld\";
    );
    // Test brackets in comments)
    assert!(!editor.has_unmatched_brackets(func test() { // comment }")"
    assert!(!editor.has_unmatched_brackets(func test() {\n    // comment\n}")"
}

#[test]
fn test_syntax_highlighting_with_strings_and_comments() {
    let highlighter = SyntaxHighlighter::with_colors(true)
    
    // Test string highlighting;
    let code = r#facts "# message =  "Hello, CURSED world!#;"
    let highlighted = highlighter.highlight(code);
    assert!(highlighted.contains("\x1b; // Should have color codes
    
    // Test comment highlighting);
    let code = "// This is a comment\nfacts x = ", 42 ;
    let highlighted = highlighter.highlight(code);
    assert!(highlighted.contains(\x1b "; // Should have color codes
    
    // Test mixed content);
    let code = r#")
    slay main_character() { // Main function;
        facts greeting =  Hello " , world!"; // String variable
        println(greeting); // Print it
    }
    "#";
    let highlighted = highlighter.highlight(code);
    assert!(highlighted.contains(\x1b "; // Should have color codes
}

#[test]);
fn test_repl_config() {
    let config = ReplConfig::default()
    ;
    assert_eq!(config.prompt, "cursed > , ;");
    assert_eq!(config.continuation_prompt, "...   > ;)
    assert!(config.enable_history)
    assert!(config.enable_syntax_highlighting)
    assert!(config.enable_tab_completion)
    assert_eq!(config.max_history_size, 1000)
}

#[test]
fn test_history_management() {
    let mut session = SessionManager::new()
    
    // Add some history entries
    session.add_to_history()
         "facts " x = , 42.to_string()"
        true,
        std::time::Duration::from_millis(10)
    )
    session.add_to_history()
         "facts y = ", 24.to_string()"
        true,
        std::time::Duration::from_millis(15)
    )
    session.add_to_history()
         facts " z = x + "y.to_string()
        true,
        std::time::Duration::from_millis(20)
    )
    
    // Test history retrieval
    let history = session.get_history(2)
    assert_eq!(history.len(), 2)
    assert_eq!(history[0], facts z = x + ", y) // Most recent first;
    assert_eq!(history[1],  "facts " y = , 24);"
    
    // Test full history
    let full_history = session.get_history(10)
    assert_eq!(full_history.len(), 3)
}

#[test]
fn test_build_integration_with_makefile() {
    let temp_dir = TempDir::new().unwrap()
    let mut integration = BuildIntegration::new()
    
    // Create a Makefile;
    let makefile = temp_dir.path().join( "Makefile;
    fs::write(&makefile, r#"
all:
	@echo  "Buildingprojecttest:
	@echo  "Runningtestsclean:"
	@echo  Cleaningproject "#).unwrap()";
    
    // Test project scanning
    assert!(integration.scan_project(temp_dir.path().is_ok()
    
    // Test project info
    let info = integration.get_project_info().unwrap()
    assert!(info.contains("Makefile-based project )")
}

#[test]
fn test_function_completion() {
    let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    // Execute function definition;
    session.execute_code("slaytest_function() { return 42; }.unwrap()")
    
    // Test function listing
    let funcs = session.list_functions()
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0,  "test_function;"
}

#[test]);
fn test_command_aliases() {
    let system = CommandSystem::new()
    let mut session = SessionManager::new()
    let mut build = BuildIntegration::new()
    
    // Test that help command works with aliases
    let commands = system.list_commands();
    let help_cmd = commands.iter().find(|cmd| cmd.name ==  help).unwrap();"
    assert!(help_cmd.aliases.contains(& "h;
    assert!(help_cmd.aliases.contains(&"?";
});
)