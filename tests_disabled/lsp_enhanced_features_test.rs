//! Integration tests for enhanced CURSED LSP features
//! 
//! Tests semantic highlighting, code lens, inlay hints, and enhanced symbol support.

use cursed::lsp::semantic_highlighting::{SemanticHighlightingProvider, CursedSemanticTokenType};
use cursed::lsp::code_lens::{CodeLensProvider, CodeLensType, TestStatus};
use cursed::lsp::inlay_hints::{InlayHintsProvider, InlayHintConfig, InlayHintType};
use cursed::lsp::enhanced_symbols::{EnhancedSymbolProvider, CursedSymbolKind};
use tower_lsp::lsp_types::*;

#[tokio::test]
async fn test_semantic_highlighting_cursed_keywords() {
    let provider = SemanticHighlightingProvider::new();
    let content = r#"
        package test_package
        
        import "stdlib::io"
        
        slay greet(name: string) -> string {
            sus greeting = "Hello, " + name;
            facts PI = 3.14159;
            
            lowkey (name.len() > 0) {
                vibez greeting;
            } highkey {
                vibez "Hello, World!";
            }
        }
        
        squad Person {
            name: string,
            age: i32,
        }
        
        collab Greeter {
            greet(name: string) -> string;
        }
        
        slay test_math() {
            sus result = 2 + 2;
            spill "test error" lowkey (result != 4);
        }
    "#;
    
    let tokens = provider.get_semantic_tokens(content).await.unwrap();
    assert!(!tokens.is_empty());
    
    // Check that CURSED-specific keywords are highlighted
    let keyword_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(
            t.token_type,
            CursedSemanticTokenType::SlayKeyword |
            CursedSemanticTokenType::SusKeyword |
            CursedSemanticTokenType::FactsKeyword |
            CursedSemanticTokenType::LowkeyKeyword |
            CursedSemanticTokenType::HighkeyKeyword |
            CursedSemanticTokenType::VibezKeyword |
            CursedSemanticTokenType::SpillKeyword |
            CursedSemanticTokenType::SquadKeyword |
            CursedSemanticTokenType::CollabKeyword
        ))
        .collect();
    
    // Should have found Gen Z slang keywords
    assert!(keyword_tokens.len() >= 8, "Should find at least 8 CURSED keywords");
    
    // Test semantic token encoding
    let encoded = provider.encode_semantic_tokens(tokens);
    assert!(!encoded.data.is_empty());
    assert_eq!(encoded.data.len() % 5, 0); // Each token has 5 values
}

#[tokio::test]
async fn test_semantic_highlighting_range() {
    let provider = SemanticHighlightingProvider::new();
    let content = r#"
        slay test_function() {
            sus variable = "test";
            vibez variable;
        }
    "#;
    
    let range = Range {
        start: Position { line: 1, character: 0 },
        end: Position { line: 3, character: 0 },
    };
    
    let tokens = provider.get_semantic_tokens_range(content, range).await.unwrap();
    assert!(!tokens.is_empty());
    
    // All tokens should be within the specified range
    for token in &tokens {
        assert!(token.line >= range.start.line);
        assert!(token.line <= range.end.line);
    }
}

#[tokio::test]
async fn test_code_lens_generation() {
    let provider = CodeLensProvider::new();
    let content = r#"
        slay greet(name: string) -> string {
            vibez "Hello, " + name;
        }
        
        slay test_greet() {
            // Test function
            sus result = greet("World");
        }
        
        squad Person {
            name: string,
            age: i32,
        }
        
        collab Greeter {
            greet(name: string) -> string;
        }
        
        sus global_variable = "test";
    "#;
    
    let uri = Url::parse("file:///test.csd").unwrap();
    let lenses = provider.get_code_lenses(content, &uri).await.unwrap();
    
    assert!(!lenses.is_empty());
    
    // Should have lenses for functions, types, and variables
    let lens_types: Vec<_> = lenses.iter().map(|l| &l.lens_type).collect();
    assert!(lens_types.contains(&&CodeLensType::ReferenceCount));
    
    // Check for test function detection
    let test_lenses: Vec<_> = lenses
        .iter()
        .filter(|l| l.lens_type == CodeLensType::TestStatus)
        .collect();
    
    assert!(!test_lenses.is_empty(), "Should detect test functions");
}

#[tokio::test]
async fn test_code_lens_test_detection() {
    let provider = CodeLensProvider::new();
    
    // Test various test function naming patterns
    assert!(provider.is_test_function("test_basic"));
    assert!(provider.is_test_function("math_test"));
    assert!(provider.is_test_function("setup_test_data"));
    assert!(!provider.is_test_function("regular_function"));
    assert!(!provider.is_test_function("testing")); // Should not match
}

#[tokio::test]
async fn test_code_lens_resolution() {
    let provider = CodeLensProvider::new();
    
    let code_lens = CodeLens {
        range: Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 10 },
        },
        command: None,
        data: Some(serde_json::json!({
            "symbol": "test_function",
            "type": "references"
        })),
    };
    
    let resolved = provider.resolve_code_lens(code_lens).await.unwrap();
    assert!(resolved.command.is_some());
}

#[tokio::test]
async fn test_inlay_hints_generation() {
    let mut provider = InlayHintsProvider::new();
    let content = r#"
        slay calculate(x: i32, y: i32) -> i32 {
            sus result = x + y;
            vibez result;
        }
        
        slay main() {
            sus value = calculate(5, 10);
            sus text = "Hello";
        }
    "#;
    
    let range = Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 10, character: 0 },
    };
    
    let hints = provider.get_inlay_hints(content, range).await.unwrap();
    
    // Should have some hints for inferred types
    assert!(!hints.is_empty());
    
    // Check hint types
    let hint_types: Vec<_> = hints.iter().map(|h| &h.hint_type).collect();
    assert!(hint_types.contains(&&InlayHintType::TypeHint));
}

#[tokio::test]
async fn test_inlay_hints_configuration() {
    let mut config = InlayHintConfig::default();
    config.show_type_hints = false;
    config.show_parameter_names = true;
    config.max_hint_length = 20;
    
    let mut provider = InlayHintsProvider::with_config(config);
    
    let content = r#"
        slay test(param1: string, param2: i32) {
            sus variable = "test";
        }
    "#;
    
    let range = Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 5, character: 0 },
    };
    
    let hints = provider.get_inlay_hints(content, range).await.unwrap();
    
    // Should have parameter hints but no type hints
    let type_hints: Vec<_> = hints
        .iter()
        .filter(|h| h.hint_type == InlayHintType::TypeHint)
        .collect();
    
    // Type hints should be filtered out based on configuration
    assert!(type_hints.is_empty() || type_hints.len() < hints.len());
}

#[tokio::test]
async fn test_inlay_hints_filtering() {
    let mut config = InlayHintConfig::default();
    config.max_hint_length = 5;
    config.only_complex_types = true;
    
    let provider = InlayHintsProvider::with_config(config);
    
    // Test complex type detection
    assert!(provider.is_complex_type("map<string, i32>"));
    assert!(provider.is_complex_type("chan int"));
    assert!(provider.is_complex_type("Option<T>"));
    assert!(!provider.is_complex_type("string"));
    assert!(!provider.is_complex_type("i32"));
}

#[tokio::test]
async fn test_enhanced_symbols_extraction() {
    let mut provider = EnhancedSymbolProvider::new();
    let content = r#"
        package test_package
        
        import "stdlib::io"
        
        slay greet(name: string) -> string {
            sus local_var = "Hello, " + name;
            vibez local_var;
        }
        
        squad Person {
            name: string,
            age: i32,
        }
        
        collab Greeter {
            greet(name: string) -> string;
        }
        
        facts PI = 3.14159;
        sus global_var = "test";
        
        slay test_function() {
            // Test function
        }
    "#;
    
    let uri = Url::parse("file:///test.csd").unwrap();
    let symbols = provider.get_document_symbols(content, &uri).await.unwrap();
    
    assert!(!symbols.is_empty());
    
    // Check for different symbol types
    let symbol_kinds: Vec<_> = symbols.iter().map(|s| &s.cursed_kind).collect();
    assert!(symbol_kinds.contains(&&CursedSymbolKind::PackageDeclaration));
    assert!(symbol_kinds.contains(&&CursedSymbolKind::ImportDeclaration));
    assert!(symbol_kinds.contains(&&CursedSymbolKind::SlayFunction));
    assert!(symbol_kinds.contains(&&CursedSymbolKind::SquadStruct));
    assert!(symbol_kinds.contains(&&CursedSymbolKind::CollabInterface));
    assert!(symbol_kinds.contains(&&CursedSymbolKind::FactsConstant));
    assert!(symbol_kinds.contains(&&CursedSymbolKind::SusVariable));
    
    // Check for test function detection
    let test_functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.cursed_kind == CursedSymbolKind::TestFunction)
        .collect();
    
    assert!(!test_functions.is_empty(), "Should detect test functions");
    
    // Check symbol hierarchy (functions should have parameter children)
    let function_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| matches!(s.cursed_kind, CursedSymbolKind::SlayFunction | CursedSymbolKind::TestFunction))
        .collect();
    
    // Functions with parameters should have children
    for func in function_symbols {
        if func.name == "greet" {
            assert!(!func.children.is_empty(), "Function with parameters should have children");
            assert!(func.children.iter().any(|child| 
                child.cursed_kind == CursedSymbolKind::Parameter
            ));
        }
    }
}

#[tokio::test]
async fn test_enhanced_symbols_workspace_search() {
    let mut provider = EnhancedSymbolProvider::new();
    
    // Add test symbols to cache
    let test_symbols = vec![
        cursed::lsp::enhanced_symbols::CursedSymbol::new(
            "greet_user".to_string(),
            SymbolKind::FUNCTION,
            CursedSymbolKind::SlayFunction,
            Range::default(),
            Range::default(),
        ),
        cursed::lsp::enhanced_symbols::CursedSymbol::new(
            "UserProfile".to_string(),
            SymbolKind::STRUCT,
            CursedSymbolKind::SquadStruct,
            Range::default(),
            Range::default(),
        ),
        cursed::lsp::enhanced_symbols::CursedSymbol::new(
            "calculate_total".to_string(),
            SymbolKind::FUNCTION,
            CursedSymbolKind::SlayFunction,
            Range::default(),
            Range::default(),
        ),
    ];
    
    provider.workspace_symbols.insert("test".to_string(), test_symbols);
    
    let workspace_folders = vec![];
    
    // Test various search queries
    let results = provider.search_workspace_symbols("user", &workspace_folders).await.unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|s| s.name.to_lowercase().contains("user")));
    
    let results = provider.search_workspace_symbols("calc", &workspace_folders).await.unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|s| s.name.contains("calculate")));
    
    // Test fuzzy matching
    let results = provider.search_workspace_symbols("gu", &workspace_folders).await.unwrap();
    assert!(!results.is_empty());
    
    // Test empty query (should return all symbols)
    let results = provider.search_workspace_symbols("", &workspace_folders).await.unwrap();
    assert_eq!(results.len(), 3);
}

#[tokio::test]
async fn test_enhanced_symbols_fuzzy_matching() {
    let provider = EnhancedSymbolProvider::new();
    
    // Test fuzzy matching algorithm
    assert!(provider.fuzzy_match("greet_user", "gu"));
    assert!(provider.fuzzy_match("UserProfile", "up"));
    assert!(provider.fuzzy_match("calculate_total", "ct"));
    assert!(provider.fuzzy_match("TestFunction", "tf"));
    assert!(!provider.fuzzy_match("greet", "xyz"));
    assert!(!provider.fuzzy_match("short", "verylongpattern"));
}

#[tokio::test]
async fn test_enhanced_symbols_conversions() {
    let symbol = cursed::lsp::enhanced_symbols::CursedSymbol::new(
        "test_function".to_string(),
        SymbolKind::FUNCTION,
        CursedSymbolKind::TestFunction,
        Range {
            start: Position { line: 5, character: 4 },
            end: Position { line: 5, character: 17 },
        },
        Range {
            start: Position { line: 5, character: 4 },
            end: Position { line: 5, character: 17 },
        },
    )
    .with_visibility(cursed::lsp::enhanced_symbols::Visibility::Public)
    .with_async(true)
    .with_generic(false)
    .with_type_info("() -> void".to_string())
    .with_documentation("Test function documentation".to_string());
    
    let uri = Url::parse("file:///test.csd").unwrap();
    
    // Test conversion to different LSP types
    let doc_symbol = symbol.to_document_symbol();
    assert_eq!(doc_symbol.name, "test_function");
    assert_eq!(doc_symbol.kind, SymbolKind::FUNCTION);
    assert_eq!(doc_symbol.range.start.line, 5);
    
    let workspace_symbol = symbol.to_workspace_symbol(uri.clone());
    assert_eq!(workspace_symbol.name, "test_function");
    assert_eq!(workspace_symbol.kind, SymbolKind::FUNCTION);
    
    let symbol_info = symbol.to_symbol_information(uri);
    assert_eq!(symbol_info.name, "test_function");
    assert_eq!(symbol_info.kind, SymbolKind::FUNCTION);
    assert_eq!(symbol_info.location.range.start.line, 5);
}

#[tokio::test]
async fn test_integration_all_features() {
    // Test that all features work together
    let semantic_provider = SemanticHighlightingProvider::new();
    let code_lens_provider = CodeLensProvider::new();
    let mut inlay_provider = InlayHintsProvider::new();
    let mut symbol_provider = EnhancedSymbolProvider::new();
    
    let content = r#"
        slay test_integration(value: i32) -> string {
            sus result = value.to_string();
            vibez result;
        }
    "#;
    
    let uri = Url::parse("file:///integration_test.csd").unwrap();
    let range = Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 6, character: 0 },
    };
    
    // Test semantic highlighting
    let tokens = semantic_provider.get_semantic_tokens(content).await.unwrap();
    assert!(!tokens.is_empty());
    
    // Test code lenses
    let lenses = code_lens_provider.get_code_lenses(content, &uri).await.unwrap();
    assert!(!lenses.is_empty());
    
    // Test inlay hints
    let hints = inlay_provider.get_inlay_hints(content, range).await.unwrap();
    // Hints may be empty depending on configuration, but should not error
    
    // Test document symbols
    let symbols = symbol_provider.get_document_symbols(content, &uri).await.unwrap();
    assert!(!symbols.is_empty());
    
    // All features should work without conflicts
    assert!(tokens.len() > 0);
    assert!(lenses.len() > 0);
    assert!(symbols.len() > 0);
}

#[test]
fn test_test_status_display() {
    assert_eq!(TestStatus::Passed.to_emoji(), "✅");
    assert_eq!(TestStatus::Failed.to_emoji(), "❌");
    assert_eq!(TestStatus::NotRun.to_emoji(), "⚪");
    assert_eq!(TestStatus::Running.to_emoji(), "🔄");
    assert_eq!(TestStatus::Skipped.to_emoji(), "⏭️");
    
    assert_eq!(TestStatus::Passed.to_color(), "green");
    assert_eq!(TestStatus::Failed.to_color(), "red");
    assert_eq!(TestStatus::NotRun.to_color(), "gray");
    assert_eq!(TestStatus::Running.to_color(), "blue");
    assert_eq!(TestStatus::Skipped.to_color(), "yellow");
}
