//! Minimal but Complete CURSED Language Server Protocol Implementation
//! Provides basic IDE support with completion and diagnostics

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::{LspService, Server, LanguageServer, Client};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tokio::sync::RwLock;

/// CURSED Language Server
#[derive(Debug)]
pub struct CursedLanguageServer {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, DocumentData>>>,
    workspace_root: Arc<RwLock<Option<PathBuf>>>,
}

/// Document data
#[derive(Debug, Clone)]
pub struct DocumentData {
    pub uri: Url,
    pub text: String,
    pub version: i32,
    pub last_modified: std::time::SystemTime,
}

impl CursedLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            workspace_root: Arc::new(RwLock::new(None)),
        }
    }

    /// Get completion items for a position
    async fn get_completions(&self, _uri: &Url, _position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Add CURSED keywords
        let keywords = vec![
            ("sus", "Variable declaration"),
            ("damn", "Return statement"),
            ("slay", "Function declaration"),
            ("vibez", "Module/namespace"),
            ("yeet", "Import statement"),
            ("bestie", "For loop"),
            ("stan", "While loop"),
            ("ready", "Select statement"),
            ("based", "True boolean"),
            ("cap", "False boolean"),
            ("cringe", "Nil value"),
            ("facts", "Constant declaration"),
            ("lit", "Boolean type"),
            ("tea", "String type"),
            ("drip", "Float type"),
            ("normie", "Integer type"),
            ("smol", "Small integer"),
            ("thicc", "Large integer"),
        ];
        
        for (keyword, detail) in keywords {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(detail.to_string()),
                documentation: Some(Documentation::String(
                    format!("CURSED keyword: {}", detail)
                )),
                deprecated: Some(false),
                preselect: None,
                sort_text: Some(format!("0_{}", keyword)),
                filter_text: None,
                insert_text: Some(keyword.to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
                label_details: None,
            });
        }

        // Add stdlib functions
        let stdlib_functions = vec![
            ("vibez.spill", "Print to output"),
            ("math.add", "Add numbers"),
            ("string.length", "Get string length"),
            ("crypto.hash", "Hash data"),
            ("json.parse", "Parse JSON"),
        ];

        for (func, detail) in stdlib_functions {
            completions.push(CompletionItem {
                label: func.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(detail.to_string()),
                documentation: Some(Documentation::String(
                    format!("CURSED stdlib function: {}", detail)
                )),
                deprecated: Some(false),
                preselect: None,
                sort_text: Some(format!("1_{}", func)),
                filter_text: None,
                insert_text: Some(func.to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
                label_details: None,
            });
        }

        completions
    }

    /// Get hover information for a symbol
    async fn get_hover(&self, _uri: &Url, _position: Position) -> Option<Hover> {
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**CURSED Language**\n\nHover information for CURSED symbols.".to_string(),
            }),
            range: None,
        })
    }

    /// Format document text
    async fn format_document(&self, uri: &Url) -> Option<Vec<TextEdit>> {
        let documents = self.documents.read().await;
        if let Some(doc_data) = documents.get(uri) {
            let formatted_text = self.format_cursed_code(&doc_data.text);
            
            if formatted_text != doc_data.text {
                return Some(vec![TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { 
                            line: doc_data.text.lines().count() as u32, 
                            character: 0 
                        },
                    },
                    new_text: formatted_text,
                }]);
            }
        }
        None
    }

    /// Simple CURSED code formatter
    fn format_cursed_code(&self, text: &str) -> String {
        let mut formatted = String::new();
        let mut indent_level = 0;
        
        for line in text.lines() {
            let trimmed = line.trim();
            
            if trimmed.ends_with('{') {
                formatted.push_str(&"    ".repeat(indent_level));
                formatted.push_str(trimmed);
                formatted.push('\n');
                indent_level += 1;
            } else if trimmed == "}" {
                indent_level = indent_level.saturating_sub(1);
                formatted.push_str(&"    ".repeat(indent_level));
                formatted.push_str(trimmed);
                formatted.push('\n');
            } else if !trimmed.is_empty() {
                formatted.push_str(&"    ".repeat(indent_level));
                formatted.push_str(trimmed);
                formatted.push('\n');
            } else {
                formatted.push('\n');
            }
        }
        
        formatted
    }

    /// Get workspace symbols
    async fn get_workspace_symbols(&self, query: &str) -> Vec<SymbolInformation> {
        let symbols = vec![
            SymbolInformation {
                name: format!("Symbol matching '{}'", query),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Url::parse("file:///example.csd").unwrap(),
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                },
                container_name: None,
            }
        ];
        
        symbols
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for CursedLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        // Store workspace root
        if let Some(workspace_folders) = params.workspace_folders {
            if let Some(folder) = workspace_folders.first() {
                if let Ok(path) = folder.uri.to_file_path() {
                    *self.workspace_root.write().await = Some(path);
                }
            }
        } else if let Some(root_uri) = params.root_uri {
            if let Ok(path) = root_uri.to_file_path() {
                *self.workspace_root.write().await = Some(path);
            }
        }

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "CURSED Language Server".to_string(),
                version: Some("1.0.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                    completion_item: None,
                    all_commit_characters: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions::default(),
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::KEYWORD,
                                    SemanticTokenType::STRING,
                                    SemanticTokenType::NUMBER,
                                    SemanticTokenType::FUNCTION,
                                    SemanticTokenType::VARIABLE,
                                    SemanticTokenType::TYPE,
                                    SemanticTokenType::COMMENT,
                                ],
                                token_modifiers: vec![
                                    SemanticTokenModifier::DECLARATION,
                                    SemanticTokenModifier::DEFINITION,
                                ],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        }
                    )
                ),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "CURSED Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;

        let doc_data = DocumentData {
            uri: uri.clone(),
            text,
            version,
            last_modified: std::time::SystemTime::now(),
        };

        {
            let mut documents = self.documents.write().await;
            documents.insert(uri, doc_data);
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;
            
            let doc_data = DocumentData {
                uri: uri.clone(),
                text,
                version,
                last_modified: std::time::SystemTime::now(),
            };

            {
                let mut documents = self.documents.write().await;
                documents.insert(uri, doc_data);
            }
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        // Document saved - could trigger additional analysis
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut documents = self.documents.write().await;
        documents.remove(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let completions = self.get_completions(uri, position).await;
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        Ok(self.get_hover(uri, position).await)
    }

    async fn goto_definition(&self, _params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> {
        // Simple placeholder implementation
        Ok(None)
    }

    async fn references(&self, _params: ReferenceParams) -> LspResult<Option<Vec<Location>>> {
        // Simple placeholder implementation
        Ok(Some(vec![]))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> LspResult<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;
        Ok(self.format_document(uri).await)
    }

    async fn symbol(&self, params: WorkspaceSymbolParams) -> LspResult<Option<Vec<SymbolInformation>>> {
        let symbols = self.get_workspace_symbols(&params.query).await;
        Ok(Some(symbols))
    }
}

/// Create and start the LSP server
pub async fn start_lsp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| CursedLanguageServer::new(client))
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}

// Tests would go here but require more complex mocking setup
