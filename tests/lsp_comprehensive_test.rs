//! Comprehensive tests for CURSED LSP implementation
//! 
//! Tests all major LSP features including completion, navigation, semantic highlighting,
//! refactoring, and diagnostics.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tokio;

use cursed::lsp::backend::CursedLanguageServer;
use cursed::lsp::completion::CompletionProvider;
use cursed::lsp::navigation::NavigationProvider;
use cursed::lsp::semantic_highlighting::SemanticHighlightingProvider;
use cursed::lsp::refactoring::RefactoringProvider;
use cursed::lsp::diagnostics::DiagnosticsProvider;

/// Test completion provider functionality
#[tokio::test]
async fn test_enhanced_completion_features() {
    let provider = CompletionProvider::new();
    
    // Test Gen Z slang keyword completion
    let content = "sl";
    let position = Position { line: 0, character: 2 };
    let completions = provider.get_completions(content, position).await;
    
    assert!(!completions.is_empty());
    assert!(completions.iter().any(|c| c.label == "slay"));
    
    // Test function completion with signatures
    let content = "pr";
    let position = Position { line: 0, character: 2 };
    let completions = provider.get_completions(content, position).await;
    
    assert!(completions.iter().any(|c| c.label == "print"));
    assert!(completions.iter().any(|c| c.label == "println"));
    
    // Verify completion has enhanced documentation
    let print_completion = completions.iter().find(|c| c.label == "print").unwrap();
    assert!(print_completion.documentation.is_some());
    assert!(print_completion.detail.is_some());
}

#[tokio::test]
async fn test_context_aware_completion() {
    let provider = CompletionProvider::new();
    
    // Test member completion after dot
    let content = r#"
        facts arr = [1, 2, 3]
        arr.
    "#;
    let position = Position { line: 2, character: 12 };
    let completions = provider.get_completions(content, position).await;
    
    // Should suggest array methods
    assert!(completions.iter().any(|c| c.label == "push"));
    assert!(completions.iter().any(|c| c.label == "len"));
    assert!(completions.iter().any(|c| c.label == "pop"));
}

#[tokio::test]
async fn test_variable_type_aware_completion() {
    let provider = CompletionProvider::new();
    
    // Test with string variable
    let content = r#"
        facts name = "CURSED"
        name.
    "#;
    let position = Position { line: 2, character: 13 };
    let completions = provider.get_completions(content, position).await;
    
    // Should suggest string methods
    assert!(completions.iter().any(|c| c.label == "len"));
    assert!(completions.iter().any(|c| c.label == "trim"));
    assert!(completions.iter().any(|c| c.label == "split"));
}

/// Test navigation provider functionality
#[tokio::test]
async fn test_enhanced_hover_information() {
    let provider = NavigationProvider::new();
    
    // Test hover on built-in function
    let content = "print(\"hello\")";
    let position = Position { line: 0, character: 2 };
    let hover = provider.get_hover_info(content, position).await;
    
    assert!(hover.is_some());
    let hover = hover.unwrap();
    
    if let HoverContents::Markup(markup) = hover.contents {
        assert!(markup.value.contains("print"));
        assert!(markup.value.contains("stdout"));
        assert!(markup.value.contains("Example:"));
    }
}

#[tokio::test]
async fn test_go_to_definition() {
    let provider = NavigationProvider::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    // Test go to definition for user-defined variable
    let content = r#"
        facts my_variable = 42
        println(my_variable)
    "#;
    let position = Position { line: 2, character: 16 }; // On "my_variable" in println
    let definition = provider.get_definition(content, position, &uri).await;
    
    assert!(definition.is_some());
    if let Some(GotoDefinitionResponse::Scalar(location)) = definition {
        assert_eq!(location.range.start.line, 1); // Defined on line 1
    }
}

#[tokio::test]
async fn test_find_references() {
    let provider = NavigationProvider::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    // Test find references for variable
    let content = r#"
        facts count = 0
        sus result = count + 1
        println(count)
    "#;
    let position = Position { line: 1, character: 6 }; // On variable declaration
    let references = provider.find_references(content, position, &uri).await;
    
    assert_eq!(references.len(), 3); // Declaration + 2 usages
}

/// Test semantic highlighting functionality
#[tokio::test]
async fn test_semantic_highlighting_gen_z_keywords() {
    let provider = SemanticHighlightingProvider::new();
    
    let content = r#"
        slay greet(name: string) -> string {
            sus greeting = "Hello, " + name
            bounce greeting
        }
        
        facts PI = 3.14159
        
        lowkey (x > 0) {
            spill "positive number"
        }
    "#;
    
    let tokens = provider.get_semantic_tokens(content).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify Gen Z slang keywords are properly highlighted
    let keyword_count = tokens.iter().filter(|t| {
        matches!(
            t.token_type,
            cursed::lsp::semantic_highlighting::CursedSemanticTokenType::SlayKeyword |
            cursed::lsp::semantic_highlighting::CursedSemanticTokenType::SusKeyword |
            cursed::lsp::semantic_highlighting::CursedSemanticTokenType::FactsKeyword |
            cursed::lsp::semantic_highlighting::CursedSemanticTokenType::LowkeyKeyword |
            cursed::lsp::semantic_highlighting::CursedSemanticTokenType::SpillKeyword
        )
    }).count();
    
    assert!(keyword_count >= 5);
}

#[tokio::test]
async fn test_semantic_token_encoding() {
    let provider = SemanticHighlightingProvider::new();
    
    let content = "slay main() { println(\"Hello\") }";
    let tokens = provider.get_semantic_tokens(content).unwrap();
    let encoded = provider.encode_semantic_tokens(tokens);
    
    assert!(!encoded.data.is_empty());
    // Each token should have 5 values: delta_line, delta_start, length, token_type, modifiers
    assert_eq!(encoded.data.len() % 5, 0);
}

/// Test refactoring provider functionality
#[tokio::test]
async fn test_rename_symbol_preparation() {
    let provider = RefactoringProvider::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    let content = r#"
        facts variable_name = 42
        println(variable_name)
    "#;
    let position = Position { line: 1, character: 8 }; // On variable name
    let prepare_result = provider.prepare_rename(content, position, &uri).await;
    
    assert!(prepare_result.is_some());
}

#[tokio::test]
async fn test_rename_symbol_execution() {
    let provider = RefactoringProvider::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    let content = r#"
        facts old_name = 42
        println(old_name)
        sus another = old_name + 1
    "#;
    let position = Position { line: 1, character: 8 }; // On variable declaration
    let workspace_edit = provider.rename_symbol(content, position, "new_name", &uri).await;
    
    assert!(workspace_edit.is_some());
    let edit = workspace_edit.unwrap();
    assert!(edit.changes.is_some());
    
    let changes = edit.changes.unwrap();
    let file_changes = changes.get(&uri).unwrap();
    assert_eq!(file_changes.len(), 3); // Should rename in 3 places
}

#[tokio::test]
async fn test_extract_function_capability() {
    let provider = RefactoringProvider::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    // Test complex code block that should be extractable
    let content = r#"
        slay main() {
            sus x = 5
            sus y = 10
            sus result = x * x + y * y
            println(result)
        }
    "#;
    
    let range = Range {
        start: Position { line: 2, character: 12 },
        end: Position { line: 5, character: 26 },
    };
    
    let context = CodeActionContext {
        diagnostics: vec![],
        only: None,
        trigger_kind: None,
    };
    
    let actions = provider.get_code_actions(content, range, &context, &uri).await;
    
    // Should include extract function action
    assert!(actions.iter().any(|action| {
        if let CodeActionOrCommand::CodeAction(ca) = action {
            ca.title.contains("Extract Function")
        } else {
            false
        }
    }));
}

#[tokio::test]
async fn test_organize_imports_detection() {
    let provider = RefactoringProvider::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    let content = r#"
        use "external/package"
        use "std/fmt"
        use "./local_module"
        
        slay main() {
            println("Hello")
        }
    "#;
    
    let range = Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 0, character: 0 },
    };
    
    let context = CodeActionContext {
        diagnostics: vec![],
        only: None,
        trigger_kind: None,
    };
    
    let actions = provider.get_code_actions(content, range, &context, &uri).await;
    
    // Should include organize imports action
    assert!(actions.iter().any(|action| {
        if let CodeActionOrCommand::CodeAction(ca) = action {
            ca.title.contains("Organize Imports")
        } else {
            false
        }
    }));
}

/// Test diagnostics provider functionality
#[tokio::test]
async fn test_syntax_error_detection() {
    let provider = DiagnosticsProvider::new();
    
    // Test content with syntax errors
    let content = r#"
        slay invalid_function(
            // Missing closing parenthesis and brace
        facts incomplete = 
    "#;
    
    let diagnostics = provider.get_syntax_diagnostics(content).await;
    
    // Should detect syntax errors
    assert!(!diagnostics.is_empty());
    assert!(diagnostics.iter().any(|d| d.severity == Some(DiagnosticSeverity::ERROR)));
}

#[tokio::test]
async fn test_enhanced_error_messages() {
    let provider = DiagnosticsProvider::new();
    
    // Test semantic errors
    let content = r#"
        slay test_function() {
            facts x = 42
            x = 50  // Error: trying to modify immutable variable
        }
    "#;
    
    let diagnostics = provider.get_semantic_diagnostics(content).await;
    
    // Should provide helpful error messages
    assert!(!diagnostics.is_empty());
    let error_messages: Vec<_> = diagnostics.iter().map(|d| &d.message).collect();
    assert!(error_messages.iter().any(|msg| msg.contains("immutable") || msg.contains("constant")));
}

/// Integration tests for complete LSP workflow
#[tokio::test]
async fn test_complete_lsp_workflow() {
    let server = CursedLanguageServer::new();
    
    // Test initialization
    let init_params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(Url::parse("file:///workspace").unwrap()),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    };
    
    let init_result = server.initialize(init_params).await;
    assert!(init_result.is_ok());
    
    let result = init_result.unwrap();
    assert!(result.capabilities.completion_provider.is_some());
    assert!(result.capabilities.hover_provider.is_some());
    assert!(result.capabilities.definition_provider.is_some());
    assert!(result.capabilities.references_provider.is_some());
    assert!(result.capabilities.document_formatting_provider.is_some());
    assert!(result.capabilities.rename_provider.is_some());
    assert!(result.capabilities.code_action_provider.is_some());
}

#[tokio::test]
async fn test_document_lifecycle() {
    let server = CursedLanguageServer::new();
    let uri = Url::parse("file:///test.csd").unwrap();
    
    // Test document open
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: "slay main() { println(\"Hello\") }".to_string(),
        },
    };
    
    server.did_open(open_params).await;
    
    // Test document change
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "slay main() { println(\"Hello, World!\") }".to_string(),
        }],
    };
    
    server.did_change(change_params).await;
    
    // Test completion after change
    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position { line: 0, character: 20 },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };
    
    let completion_result = server.completion(completion_params).await;
    assert!(completion_result.is_ok());
    
    // Test document close
    let close_params = DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
    };
    
    server.did_close(close_params).await;
}

/// Performance tests for LSP features
#[tokio::test]
async fn test_completion_performance() {
    let provider = CompletionProvider::new();
    
    // Large content to test performance
    let mut large_content = String::new();
    for i in 0..1000 {
        large_content.push_str(&format!("facts var_{} = {}\n", i, i));
    }
    large_content.push_str("pr"); // Trigger completion
    
    let position = Position { line: 1000, character: 2 };
    
    let start = std::time::Instant::now();
    let completions = provider.get_completions(&large_content, position).await;
    let duration = start.elapsed();
    
    // Should complete in reasonable time (< 1 second)
    assert!(duration.as_millis() < 1000);
    assert!(!completions.is_empty());
}

#[tokio::test]
async fn test_semantic_highlighting_performance() {
    let provider = SemanticHighlightingProvider::new();
    
    // Generate large content
    let mut large_content = String::new();
    for i in 0..500 {
        large_content.push_str(&format!(
            "slay function_{}() {{ facts var_{} = {} }}\n",
            i, i, i
        ));
    }
    
    let start = std::time::Instant::now();
    let tokens = provider.get_semantic_tokens(&large_content);
    let duration = start.elapsed();
    
    // Should complete in reasonable time
    assert!(duration.as_millis() < 2000);
    assert!(tokens.is_ok());
    assert!(!tokens.unwrap().is_empty());
}

/// Test LSP configuration and capabilities
#[tokio::test]
async fn test_server_capabilities() {
    let server = CursedLanguageServer::new();
    
    let capabilities = &server.capabilities;
    
    // Verify all expected capabilities are present
    assert!(capabilities.text_document_sync.is_some());
    assert!(capabilities.completion_provider.is_some());
    assert!(capabilities.hover_provider.is_some());
    assert!(capabilities.signature_help_provider.is_some());
    assert!(capabilities.definition_provider.is_some());
    assert!(capabilities.references_provider.is_some());
    assert!(capabilities.document_highlight_provider.is_some());
    assert!(capabilities.document_symbol_provider.is_some());
    assert!(capabilities.workspace_symbol_provider.is_some());
    assert!(capabilities.code_action_provider.is_some());
    assert!(capabilities.code_lens_provider.is_some());
    assert!(capabilities.document_formatting_provider.is_some());
    assert!(capabilities.document_range_formatting_provider.is_some());
    assert!(capabilities.document_on_type_formatting_provider.is_some());
    assert!(capabilities.rename_provider.is_some());
    assert!(capabilities.folding_range_provider.is_some());
    assert!(capabilities.selection_range_provider.is_some());
    assert!(capabilities.semantic_tokens_provider.is_some());
    assert!(capabilities.inlay_hint_provider.is_some());
    
    // Check completion provider trigger characters
    if let Some(completion_provider) = &capabilities.completion_provider {
        assert!(completion_provider.trigger_characters.is_some());
        let triggers = completion_provider.trigger_characters.as_ref().unwrap();
        assert!(triggers.contains(&".".to_string()));
        assert!(triggers.contains(&":".to_string()));
    }
}

/// Test error scenarios and edge cases
#[tokio::test]
async fn test_error_handling() {
    let provider = CompletionProvider::new();
    
    // Test with invalid position
    let content = "slay main() {}";
    let invalid_position = Position { line: 100, character: 50 };
    let completions = provider.get_completions(content, invalid_position).await;
    
    // Should handle gracefully
    assert!(completions.is_empty() || !completions.is_empty()); // Just shouldn't panic
}

#[tokio::test]
async fn test_edge_case_content() {
    let provider = NavigationProvider::new();
    
    // Test with empty content
    let empty_content = "";
    let position = Position { line: 0, character: 0 };
    let hover = provider.get_hover_info(empty_content, position).await;
    
    // Should handle gracefully
    assert!(hover.is_none());
    
    // Test with only whitespace
    let whitespace_content = "   \n  \n   ";
    let hover = provider.get_hover_info(whitespace_content, position).await;
    
    // Should handle gracefully
    assert!(hover.is_none());
}
