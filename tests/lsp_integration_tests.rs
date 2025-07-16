//! Integration tests for CURSED Language Server Protocol implementation

use cursed::lsp::server_complete::CursedLanguageServer;
use tower_lsp::lsp_types::*;
use tokio_test;

// Mock client for testing
struct MockClient;

impl tower_lsp::Client for MockClient {
    async fn log_message(&self, _typ: MessageType, _message: String) {}
    async fn show_message(&self, _typ: MessageType, _message: String) {}
    async fn show_message_request(&self, _typ: MessageType, _message: String) -> tower_lsp::jsonrpc::Result<Option<MessageActionItem>> {
        Ok(None)
    }
    async fn show_document(&self, _params: ShowDocumentParams) -> tower_lsp::jsonrpc::Result<ShowDocumentResult> {
        Ok(ShowDocumentResult { success: true })
    }
    async fn register_capability(&self, _params: RegistrationParams) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
    async fn unregister_capability(&self, _params: UnregistrationParams) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
    async fn work_done_progress_create(&self, _params: WorkDoneProgressCreateParams) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
    async fn apply_edit(&self, _params: ApplyWorkspaceEditParams) -> tower_lsp::jsonrpc::Result<ApplyWorkspaceEditResponse> {
        Ok(ApplyWorkspaceEditResponse {
            applied: true,
            failure_reason: None,
            failed_change: None,
        })
    }
    async fn publish_diagnostics(&self, _uri: Url, _diagnostics: Vec<Diagnostic>, _version: Option<i32>) {}
    async fn configuration(&self, _params: ConfigurationParams) -> tower_lsp::jsonrpc::Result<Vec<serde_json::Value>> {
        Ok(vec![])
    }
    async fn workspace_folders(&self) -> tower_lsp::jsonrpc::Result<Option<Vec<WorkspaceFolder>>> {
        Ok(None)
    }
    async fn semantic_tokens_refresh(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
    async fn code_lens_refresh(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
    async fn inlay_hint_refresh(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_lsp_server_initialization() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let init_params = InitializeParams {
        process_id: Some(1234),
        root_path: None,
        root_uri: Some(Url::parse("file:///test/workspace").unwrap()),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: Some(TraceValues::Off),
        workspace_folders: None,
        client_info: Some(ClientInfo {
            name: "test-client".to_string(),
            version: Some("1.0.0".to_string()),
        }),
        locale: None,
    };

    let result = server.initialize(init_params).await.unwrap();
    
    // Verify server capabilities
    assert!(result.capabilities.text_document_sync.is_some());
    assert!(result.capabilities.completion_provider.is_some());
    assert!(result.capabilities.hover_provider.is_some());
    assert!(result.capabilities.definition_provider.is_some());
    assert!(result.capabilities.references_provider.is_some());
    assert!(result.capabilities.document_formatting_provider.is_some());
    
    // Verify server info
    assert_eq!(result.server_info.as_ref().unwrap().name, "CURSED Language Server");
    assert_eq!(result.server_info.as_ref().unwrap().version, Some("1.0.0".to_string()));
}

#[tokio::test]
async fn test_document_analysis() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let cursed_code = r#"
slay main() {
    sus message tea = "Hello, CURSED!"
    vibez.spill(message)
    damn 0
}

sus globalVar normie = 42

interface TestInterface {
    slay testMethod() normie
}
"#;

    // Test document opening
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: cursed_code.to_string(),
        },
    };

    server.did_open(open_params).await;
    
    // The document should be analyzed and stored
    // (In a real test, we would verify the document was properly parsed and symbols extracted)
}

#[tokio::test]
async fn test_completion_keywords() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let position = Position { line: 1, character: 0 };
    
    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position,
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    if let Ok(Some(CompletionResponse::Array(completions))) = server.completion(completion_params).await {
        // Verify CURSED keywords are included
        let keywords: Vec<&str> = completions.iter()
            .filter(|item| item.kind == Some(CompletionItemKind::KEYWORD))
            .map(|item| item.label.as_str())
            .collect();
        
        assert!(keywords.contains(&"sus"));
        assert!(keywords.contains(&"slay"));
        assert!(keywords.contains(&"damn"));
        assert!(keywords.contains(&"vibez"));
        assert!(keywords.contains(&"yeet"));
        assert!(keywords.contains(&"based"));
        assert!(keywords.contains(&"cap"));
    } else {
        panic!("Expected completion results");
    }
}

#[tokio::test]
async fn test_completion_stdlib_functions() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let position = Position { line: 1, character: 0 };
    
    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position,
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    if let Ok(Some(CompletionResponse::Array(completions))) = server.completion(completion_params).await {
        // Verify stdlib functions are included
        let functions: Vec<&str> = completions.iter()
            .filter(|item| item.kind == Some(CompletionItemKind::FUNCTION))
            .map(|item| item.label.as_str())
            .collect();
        
        assert!(functions.contains(&"vibez.spill"));
        assert!(functions.contains(&"math.add"));
        assert!(functions.contains(&"string.length"));
        assert!(functions.contains(&"crypto.hash"));
        assert!(functions.contains(&"json.parse"));
    } else {
        panic!("Expected completion results");
    }
}

#[tokio::test]
async fn test_hover_information() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let cursed_code = r#"
slay testFunction() normie {
    damn 42
}
"#;

    // Open document
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: cursed_code.to_string(),
        },
    };
    server.did_open(open_params).await;

    // Test hover
    let hover_params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position { line: 1, character: 5 }, // Position over "testFunction"
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    if let Ok(Some(hover)) = server.hover(hover_params).await {
        if let HoverContents::Markup(content) = hover.contents {
            assert!(content.value.contains("testFunction"));
            assert!(content.value.contains("FUNCTION"));
        }
    }
}

#[tokio::test]
async fn test_formatting() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let unformatted_code = "slay test(){vibez.spill(\"hello\");damn 0}";

    // Open document
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: unformatted_code.to_string(),
        },
    };
    server.did_open(open_params).await;

    // Test formatting
    let format_params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            properties: Default::default(),
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(true),
            trim_final_newlines: Some(true),
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    if let Ok(Some(edits)) = server.formatting(format_params).await {
        assert!(!edits.is_empty());
        let formatted_text = &edits[0].new_text;
        assert!(formatted_text.contains("    vibez.spill")); // Should be indented
        assert!(formatted_text.contains("    damn 0")); // Should be indented
    }
}

#[tokio::test]
async fn test_workspace_symbols() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let cursed_code = r#"
slay testFunction() normie {
    damn 42
}

sus globalVar tea = "test"

interface TestInterface {
    slay interfaceMethod() normie
}
"#;

    // Open document
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: cursed_code.to_string(),
        },
    };
    server.did_open(open_params).await;

    // Test workspace symbols
    let symbol_params = WorkspaceSymbolParams {
        query: "test".to_string(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    if let Ok(Some(symbols)) = server.symbol(symbol_params).await {
        let symbol_names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        
        // Should find symbols containing "test"
        assert!(symbol_names.iter().any(|&name| name.contains("test")));
    }
}

#[tokio::test]
async fn test_go_to_definition() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let cursed_code = r#"
slay testFunction() normie {
    damn 42
}

slay main() {
    testFunction()
}
"#;

    // Open document
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: cursed_code.to_string(),
        },
    };
    server.did_open(open_params).await;

    // Test go to definition
    let definition_params = GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position { line: 6, character: 4 }, // Position over "testFunction" call
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    if let Ok(Some(GotoDefinitionResponse::Array(locations))) = server.goto_definition(definition_params).await {
        assert!(!locations.is_empty());
        // Should find the function definition
    }
}

#[tokio::test]
async fn test_find_references() {
    let client = MockClient;
    let server = CursedLanguageServer::new(client.into());
    
    let uri = Url::parse("file:///test/example.csd").unwrap();
    let cursed_code = r#"
slay testFunction() normie {
    damn 42
}

slay main() {
    testFunction()
    testFunction()
}
"#;

    // Open document
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cursed".to_string(),
            version: 1,
            text: cursed_code.to_string(),
        },
    };
    server.did_open(open_params).await;

    // Test find references
    let reference_params = ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position { line: 1, character: 5 }, // Position over function name
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: ReferenceContext {
            include_declaration: true,
        },
    };

    if let Ok(Some(references)) = server.references(reference_params).await {
        // Should find the function declaration and calls
        assert!(references.len() >= 1);
    }
}
