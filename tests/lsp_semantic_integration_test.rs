//! Comprehensive integration tests for LSP semantic integration
//!
//! Tests the enhanced LSP features that integrate with CURSED's compiler infrastructure

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use cursed::lsp::{
    workspace::WorkspaceManager,
    diagnostics::DiagnosticsProvider,
    completion::CompletionProvider,
    navigation::NavigationProvider,
};

#[tokio::test]
async fn test_enhanced_workspace_semantic_analysis() {
    let manager = WorkspaceManager::new();
    
    // Create test CURSED content
    let content = r#"
slay calculate(a: normie, b: normie) -> normie {
    facts result = a + b
    bounce result
}

squad Person {
    name: tea,
    age: normie,
}

facts global_var: tea = "hello"
sus mutable_var = 42
"#;
    
    // Test semantic analysis
    let uri = Url::parse("file:///test.csd").unwrap();
    match manager.update_file_content(&uri, content).await {
        Ok(()) => {
            // Verify symbols were extracted with semantic information
            let symbols = manager.search_symbols("").await;
            assert!(!symbols.is_empty(), "Should have extracted symbols");
            
            // Check for function symbol
            assert!(symbols.iter().any(|s| s.name == "calculate"), "Should find calculate function");
            
            // Check for struct symbol
            assert!(symbols.iter().any(|s| s.name == "Person"), "Should find Person struct");
            
            // Check for variable symbols
            assert!(symbols.iter().any(|s| s.name == "global_var"), "Should find global_var");
            assert!(symbols.iter().any(|s| s.name == "mutable_var"), "Should find mutable_var");
        }
        Err(err) => {
            // Even if there are compilation errors, we should still get symbols
            println!("Analysis had errors (expected): {}", err);
            let symbols = manager.search_symbols("").await;
            // Should still extract some symbols even with errors
            assert!(!symbols.is_empty(), "Should extract symbols even with compilation errors");
        }
    }
}

#[tokio::test]
async fn test_enhanced_diagnostics_with_compiler() {
    let provider = DiagnosticsProvider::new();
    
    // Test with valid CURSED code
    let valid_content = r#"
slay main() {
    facts x = 42
    println(x)
}
"#;
    
    let diagnostics = provider.get_syntax_diagnostics(valid_content).await;
    // Should have minimal or no diagnostics for valid code
    for diagnostic in &diagnostics {
        println!("Diagnostic: {:?}", diagnostic);
    }
    
    // Test with invalid CURSED code
    let invalid_content = r#"
slay main( {
    facts x = 42
    unknown_function()
    bounce x
    facts unreachable = "dead code"
}
"#;
    
    let diagnostics = provider.get_syntax_diagnostics(invalid_content).await;
    assert!(!diagnostics.is_empty(), "Should detect syntax errors");
    
    // Test semantic diagnostics
    let semantic_diagnostics = provider.get_semantic_diagnostics(invalid_content).await;
    // Should detect semantic issues like unreachable code
    assert!(!semantic_diagnostics.is_empty(), "Should detect semantic issues");
}

#[tokio::test]  
async fn test_context_aware_completions() {
    let provider = CompletionProvider::new();
    
    // Test completion in variable context
    let content = r#"
facts my_var: normie = 42
sus another_var: tea = "hello"
slay test_func() -> normie {
    facts local_var = 10
    bounce my_
"#;
    
    let position = Position { line: 5, character: 11 }; // After "my_"
    let completions = provider.get_completions(content, position).await;
    
    // Should suggest my_var
    assert!(completions.iter().any(|c| c.label == "my_var"), "Should suggest my_var in scope");
    
    // Test function call completions
    let func_content = r#"
slay calculate(a: normie, b: normie) -> normie {
    bounce a + b
}

slay main() {
    calc
"#;
    
    let func_position = Position { line: 6, character: 8 }; // After "calc"
    let func_completions = provider.get_completions(func_content, func_position).await;
    
    // Should suggest calculate function
    assert!(func_completions.iter().any(|c| c.label == "calculate"), "Should suggest calculate function");
    
    // Verify completion has signature information
    let calc_completion = func_completions.iter().find(|c| c.label == "calculate");
    if let Some(completion) = calc_completion {
        assert!(completion.detail.is_some(), "Function completion should have signature detail");
    }
}

#[tokio::test]
async fn test_enhanced_navigation_with_semantics() {
    let provider = NavigationProvider::new();
    
    let content = r#"
squad Person {
    name: tea,
    age: normie,
}

slay create_person(name: tea, age: normie) -> Person {
    bounce Person { name: name, age: age }
}

slay main() {
    facts person = create_person("Alice", 30)
    println(person.name)
}
"#;
    
    let uri = Url::parse("file:///test.csd").unwrap();
    
    // Test hover on function name
    let hover_position = Position { line: 6, character: 8 }; // On "create_person"
    let hover = provider.get_hover_info(content, hover_position).await;
    
    if let Some(hover_info) = hover {
        if let HoverContents::Markup(markup) = hover_info.contents {
            assert!(markup.value.contains("create_person"), "Hover should contain function name");
            assert!(markup.value.contains("Person"), "Hover should contain return type");
        }
    }
    
    // Test go to definition
    let def_position = Position { line: 12, character: 20 }; // On "create_person" call
    let definition = provider.get_definition(content, def_position, &uri).await;
    
    if let Some(GotoDefinitionResponse::Scalar(location)) = definition {
        assert_eq!(location.range.start.line, 6, "Should go to function definition line");
    }
    
    // Test find references
    let ref_position = Position { line: 6, character: 8 }; // On function definition
    let references = provider.find_references(content, ref_position, &uri).await;
    
    assert!(references.len() >= 2, "Should find definition and usage references");
}

#[tokio::test]
async fn test_cross_file_analysis() {
    let manager = WorkspaceManager::new();
    
    // File 1: module definition
    let module_content = r#"
squad User {
    id: normie,
    name: tea,
}

slay create_user(id: normie, name: tea) -> User {
    bounce User { id: id, name: name }
}
"#;
    
    // File 2: module usage
    let main_content = r#"
use "./user_module"

slay main() {
    facts user = create_user(1, "Alice")
    println(user.name)
}
"#;
    
    let module_uri = Url::parse("file:///user_module.csd").unwrap();
    let main_uri = Url::parse("file:///main.csd").unwrap();
    
    // Add both files to workspace
    let _ = manager.update_file_content(&module_uri, module_content).await;
    let _ = manager.update_file_content(&main_uri, main_content).await;
    
    // Test that symbols from both files are available
    let all_symbols = manager.search_symbols("").await;
    
    // Should find symbols from both files
    assert!(all_symbols.iter().any(|s| s.name == "User"), "Should find User struct from module");
    assert!(all_symbols.iter().any(|s| s.name == "create_user"), "Should find create_user function");
    assert!(all_symbols.iter().any(|s| s.name == "main"), "Should find main function");
    
    // Test workspace statistics
    let stats = manager.get_workspace_stats().await;
    assert_eq!(stats.cursed_files, 2, "Should track 2 CURSED files");
    assert!(stats.symbols > 0, "Should have collected symbols");
}

#[tokio::test]
async fn test_real_time_diagnostics_update() {
    let provider = DiagnosticsProvider::new();
    
    // Start with valid code
    let initial_content = r#"
slay main() {
    facts x = 42
    println(x)
}
"#;
    
    let initial_diagnostics = provider.get_syntax_diagnostics(initial_content).await;
    
    // Introduce a syntax error
    let broken_content = r#"
slay main( {
    facts x = 42
    println(x)
}
"#;
    
    let broken_diagnostics = provider.get_syntax_diagnostics(broken_content).await;
    assert!(!broken_diagnostics.is_empty(), "Should detect syntax error");
    
    // Fix the syntax error
    let fixed_content = r#"
slay main() {
    facts x = 42
    println(x)
}
"#;
    
    let fixed_diagnostics = provider.get_syntax_diagnostics(fixed_content).await;
    // Should have fewer errors after fix (may still have semantic warnings)
    assert!(fixed_diagnostics.len() <= broken_diagnostics.len(), "Should have fewer errors after fix");
}

#[tokio::test]
async fn test_type_aware_member_completions() {
    let provider = CompletionProvider::new();
    
    let content = r#"
squad Person {
    name: tea,
    age: normie,
}

slay main() {
    facts person = Person { name: "Alice", age: 30 }
    person.
"#;
    
    let position = Position { line: 8, character: 11 }; // After "person."
    let completions = provider.get_completions(content, position).await;
    
    // Should suggest struct members
    assert!(completions.iter().any(|c| c.label == "name"), "Should suggest name member");
    assert!(completions.iter().any(|c| c.label == "age"), "Should suggest age member");
    
    // Members should have type information
    let name_completion = completions.iter().find(|c| c.label == "name");
    if let Some(completion) = name_completion {
        if let Some(detail) = &completion.detail {
            assert!(detail.contains("tea") || detail.contains("string"), "Should show type information");
        }
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    
    /// Helper to create test workspace with multiple files
    pub async fn create_test_workspace() -> WorkspaceManager {
        let manager = WorkspaceManager::new();
        
        // Add some test files
        let test_files = vec![
            ("file:///main.csd", r#"
slay main() {
    facts x = calculate(10, 20)
    println(x)
}
"#),
            ("file:///math.csd", r#"
slay calculate(a: normie, b: normie) -> normie {
    bounce a + b
}

slay multiply(a: normie, b: normie) -> normie {
    bounce a * b
}
"#),
            ("file:///types.csd", r#"
squad Point {
    x: normie,
    y: normie,
}

collab Drawable {
    draw() -> void
}
"#),
        ];
        
        for (uri_str, content) in test_files {
            let uri = Url::parse(uri_str).unwrap();
            let _ = manager.update_file_content(&uri, content).await;
        }
        
        manager
    }
    
    /// Helper to verify completion quality
    pub fn verify_completion_quality(completions: &[CompletionItem]) {
        for completion in completions {
            // All completions should have labels
            assert!(!completion.label.is_empty(), "Completion should have non-empty label");
            
            // Function completions should have snippets
            if completion.kind == Some(CompletionItemKind::FUNCTION) {
                if let Some(insert_text) = &completion.insert_text {
                    assert!(insert_text.contains("$"), "Function completion should use snippet format");
                }
            }
            
            // All completions should have appropriate sorting
            assert!(completion.sort_text.is_some(), "Completion should have sort text for proper ordering");
        }
    }
}

#[tokio::test]
async fn test_comprehensive_workspace_analysis() {
    use test_helpers::*;
    
    let manager = create_test_workspace().await;
    
    // Test symbol search across all files
    let all_symbols = manager.search_symbols("").await;
    assert!(all_symbols.len() >= 6, "Should find symbols from all test files");
    
    // Test filtered search
    let func_symbols = manager.search_symbols("calc").await;
    assert!(func_symbols.iter().any(|s| s.name == "calculate"), "Should find calculate function");
    
    // Test workspace statistics
    let stats = manager.get_workspace_stats().await;
    assert_eq!(stats.cursed_files, 3, "Should track all CURSED files");
    assert!(stats.symbols >= 6, "Should have collected all symbols");
    
    println!("Workspace stats: {:?}", stats);
}
