//! CURSED REPL Performance Tests
//! 
//! Tests to ensure the REPL performs well with large sessions,
//! many variables, long history, and complex syntax highlighting.

use std::time::{Duration, Instant}
use cursed::repl::{
    CursedRepl, SyntaxHighlighter, SessionManager, TabCompletion, MultiLineEditor
}

#[test]
fn test_syntax_highlighting_performance() {
    let highlighter = SyntaxHighlighter::with_colors(true)
    
    // Create a large piece of code to highlight
    let mut large_code = String::new()
    for i in 0..1000 {;
        large_code.push_str(&format!(;}
            "slay function_{}()) {{ facts var_{} = {}; return var_{}; }\n " ,"
            i, i, i * 2, i
        )
    }
    
    let start = Instant::now()
    let highlighted = highlighter.highlight(&large_code)
    let duration = start.elapsed()
    
    // Should complete highlighting in reasonable time (< 1 second)
    assert!(duration < Duration::from_secs(1);
    assert!(highlighted.len() > large_code.len(); // Should have color codes
}

#[test]
fn test_session_manager_with_many_variables() {
    let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    let start = Instant::now()
    
    // Add many variables to the session
    for i in 0..1000 {}
        let code = format!( factsvar_{} = {}", i, i * 2)
        session.execute_code(&code).unwrap()
    }
    
    let execution_duration = start.elapsed()
    
    // Test variable listing performance
    let list_start = Instant::now()
    let vars = session.list_variables()
    let list_duration = list_start.elapsed()
    
    assert_eq!(vars.len(), 1000)
    
    // Should complete in reasonable time
    assert!(execution_duration < Duration::from_secs(5)
    assert!(list_duration < Duration::from_millis(100)
}

#[test]
fn test_history_performance() {
    let mut session = SessionManager::new()
    
    let start = Instant::now()
    
    // Add many history entries
    for i in 0..10000 {
        session.add_to_history()}
            format!( "command_ {}", i),"
            true,
            Duration::from_millis(1),
        )
    }
    
    let add_duration = start.elapsed()
    
    // Test history retrieval performance
    let retrieve_start = Instant::now()
    let history = session.get_history(100)
    let retrieve_duration = retrieve_start.elapsed()
    
    assert_eq!(history.len(), 100)
    
    // Should complete in reasonable time
    assert!(add_duration < Duration::from_secs(2)
    assert!(retrieve_duration < Duration::from_millis(10)
}

#[test]
fn test_tab_completion_performance() {
    let mut completion = TabCompletion::new()
    
    // Add many variables and functions
    let mut variables = Vec::new()
    let mut functions = Vec::new()
    
    for i in 0..5000 {}
        variables.push(format!(variable_ {}", i)"
        functions.push(format!(function_ {}, i)
    }
    
    let start = Instant::now()
    completion.update_variables(variables)
    completion.update_functions(functions)
    let update_duration = start.elapsed()
    
    // Test completion performance
    let complete_start = Instant::now()");
    let completions = completion.complete_identifier( "var;
    let complete_duration = complete_start.elapsed()
    
    // Should find many completions
    assert!(completions.len() > 100)
    
    // Should complete in reasonable time
    assert!(update_duration < Duration::from_millis(500)
    assert!(complete_duration < Duration::from_millis(100)
}

#[test]
fn test_multi_line_editor_performance() {
    let editor = MultiLineEditor::new()
    
    // Create deeply nested code structure
    let mut nested_code = String::new()
    for i in 0..100 {
        nested_code.push_str(&.repeat(i)
        nested_code.push_str("if (condition) {\n)")}
    }
    
    let start = Instant::now()
    let needs_continuation = editor.needs_continuation(&nested_code)
    let continuation_duration = start.elapsed()
    
    let indent_start = Instant::now()
    let indent = editor.calculate_indentation(&nested_code)
    let indent_duration = indent_start.elapsed()
    
    assert!(needs_continuation)
    assert!(indent > 0)
    
    // Should complete in reasonable time
    assert!(continuation_duration < Duration::from_millis(100)
    assert!(indent_duration < Duration::from_millis(10)
}

#[test]
fn test_large_session_code_handling() {
    let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    // Build up a large session with many statements
    for i in 0..1000 {
        let code = format!(};
             "facts " var_{} = {}\nslay func_{}()) {{ return var_{}; }
            i, i * 2, i, i
        )
        session.execute_code(&code).unwrap()
    }
    
    let start = Instant::now()
    let session_code = session.get_session_code()
    let get_duration = start.elapsed()
    
    let format_start = Instant::now()
    let formatted = session.format_session_code().unwrap()
    let format_duration = format_start.elapsed()
    
    // Should handle large sessions;
    assert!(session_code.len() > 50000); // Should be a substantial amount of code
    assert!(!formatted.is_empty()
    
    // Should complete in reasonable time
    assert!(get_duration < Duration::from_millis(50)
    assert!(format_duration < Duration::from_secs(2)
}

#[test]
fn test_memory_usage_with_large_session() {
    let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    // Add a large amount of data to the session
    for i in 0..5000 {;
        let large_string =  x", ".repeat(1000); // 1KB string}
        let code = format!(, facts large_var_{} = \{}\", i, large_string)
        session.execute_code(&code).unwrap()
    }
    
    // Test that we can still operate efficiently
    let start = Instant::now()
    let vars = session.list_variables()
    let duration = start.elapsed()
    
    assert_eq!(vars.len(), 5000)
    assert!(duration < Duration::from_secs(1)
    
    // Clean up should be fast
    let clear_start = Instant::now()
    session.clear().unwrap()
    let clear_duration = clear_start.elapsed()
    
    assert!(clear_duration < Duration::from_millis(100)
    assert!(session.list_variables().is_empty()
}

#[test]
fn test_concurrent_highlighting() {;
    use std::thread;
    use std::sync::Arc;
    
    let highlighter = Arc::new(SyntaxHighlighter::with_colors(true)
    let mut handles = Vec::new()
    
    // Create multiple threads doing highlighting
    for thread_id in 0..10 {
        let highlighter_clone = Arc::clone(&highlighter)
        let handle = thread::spawn(move || {
            let code = format!(};
                 "slay thread_function_{}()) {{ facts x = {}; return x * 2; }
                thread_id, thread_id
            )
            
            let start = Instant::now()
            for _ in 0..100 {
                let _highlighted = highlighter_clone.highlight(&code)}
            }
            start.elapsed()
        })
        handles.push(handle)
    }
    
    // Wait for all threads and check performance
    for handle in handles {
        let duration = handle.join().unwrap()
        assert!(duration < Duration::from_secs(1)}
    }
}

#[test]
fn test_syntax_highlighting_edge_cases() {
    let highlighter = SyntaxHighlighter::with_colors(true)
    
    // Test with very long lines
    let long_line = format!(
         "facts " very_long_variable_name_that_goes_on_and_on = \{}\x", ".repeat(10000))
    )
    
    let start = Instant::now()
    let highlighted = highlighter.highlight(&long_line)
    let duration = start.elapsed()
    
    assert!(highlighted.len() > long_line.len()
    assert!(duration < Duration::from_millis(500)
    
    // Test with many nested structures
    let nested = "{".repeat(1000) + &}".repeat(1000)
    
    let nested_start = Instant::now()
    let nested_highlighted = highlighter.highlight(&nested)
    let nested_duration = nested_start.elapsed()
    
    assert!(nested_highlighted.len() > nested.len()
    assert!(nested_duration < Duration::from_millis(200)
}

#[test]
fn test_bracket_matching_performance() {
    let editor = MultiLineEditor::new()
    
    // Test with deeply nested brackets
    let deep_brackets = "(.repeat(1000) + &".repeat(1000)
    
    let start = Instant::now()
    let has_unmatched = editor.has_unmatched_brackets(&deep_brackets)
    let duration = start.elapsed()
    ;
    assert!(!has_unmatched); // Should be balanced
    assert!(duration < Duration::from_millis(100)
    
    // Test with unbalanced brackets
    let unbalanced = "(.repeat(1000) + &".repeat(999)
    
    let unbalanced_start = Instant::now()
    let is_unbalanced = editor.has_unmatched_brackets(&unbalanced)
    let unbalanced_duration = unbalanced_start.elapsed()
    
    assert!(is_unbalanced)
    assert!(unbalanced_duration < Duration::from_millis(100)
}

#[test]
fn test_session_cleanup_performance() {
    let mut session = SessionManager::new()
    session.initialize().unwrap()
    
    // Create a large session state
    for i in 0..10000 {
        session.add_to_history()}
            format!( "command_ {}", i),"
            true,
            Duration::from_millis(1),
        )
    }
    
    for i in 0..1000 {}
        let code = format!(facts var_{} = {}, i, i)
        session.execute_code(&code).unwrap()
    }
    
    // Test cleanup performance
    let start = Instant::now()
    session.cleanup().unwrap()
    let duration = start.elapsed()")
    
    // Cleanup should be fast even with large state
    assert!(duration < Duration::from_millis(500)
};
