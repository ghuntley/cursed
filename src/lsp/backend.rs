//! LSP backend implementation providing the core language server functionality

use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::{Result as LspResult, Error as LspError};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tracing::{debug, error, info, instrument, warn};

use crate::lsp::document::DocumentManager;
use crate::lsp::diagnostics::DiagnosticsProvider;
use crate::lsp::completion::CompletionProvider;
use crate::lsp::navigation::NavigationProvider;
use crate::lsp::formatting::FormattingProvider;
use crate::lsp::workspace::WorkspaceManager;
use crate::parser::Parser;
use crate::lexer::Lexer;

/// The main CURSED language server backend
pub struct CursedLanguageServer {
    /// LSP client handle
    client: Arc<RwLock<Option<Client>>>,
    /// Document manager for tracking open files
    document_manager: Arc<DocumentManager>,
    /// Diagnostics provider
    diagnostics: Arc<DiagnosticsProvider>,
    /// Completion provider
    completion: Arc<CompletionProvider>,
    /// Navigation provider (go to definition, find references, etc.)
    navigation: Arc<NavigationProvider>,
    /// Formatting provider
    formatting: Arc<FormattingProvider>,
    /// Workspace manager
    workspace: Arc<WorkspaceManager>,
    /// Server capabilities
    capabilities: ServerCapabilities,
}

impl CursedLanguageServer {
    /// Create a new CURSED language server
    pub fn new() -> Self {
        Self::new_with_client(None)
    }

    /// Create a new CURSED language server with client
    pub fn new_with_client(client: Option<Client>) -> Self {
        let document_manager = Arc::new(DocumentManager::new());
        let diagnostics = Arc::new(DiagnosticsProvider::new());
        let completion = Arc::new(CompletionProvider::new());
        let navigation = Arc::new(NavigationProvider::new());
        let formatting = Arc::new(FormattingProvider::new());
        let workspace = Arc::new(WorkspaceManager::new());

        let capabilities = Self::build_server_capabilities();

        Self {
            client: Arc::new(RwLock::new(client)),
            document_manager,
            diagnostics,
            completion,
            navigation,
            formatting,
            workspace,
            capabilities,
        }
    }

    /// Build server capabilities
    fn build_server_capabilities() -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(vec![
                    ".".to_string(),
                    ":".to_string(),
                    "(".to_string(),
                    " ".to_string(),
                ]),
                work_done_progress_options: WorkDoneProgressOptions::default(),
                all_commit_characters: None,
                completion_item: None,
            }),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            signature_help_provider: Some(SignatureHelpOptions {
                trigger_characters: Some(Vec::from(["(".to_string(), ",".to_string()])),
                retrigger_characters: None,
                work_done_progress_options: WorkDoneProgressOptions::default(),
            }),
            definition_provider: Some(OneOf::Left(true)),
            declaration_provider: Some(DeclarationCapability::Simple(true)),
            references_provider: Some(OneOf::Left(true)),
            document_highlight_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
            code_lens_provider: Some(CodeLensOptions {
                resolve_provider: Some(true),
            }),
            document_formatting_provider: Some(OneOf::Left(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                first_trigger_character: "{".to_string(),
                more_trigger_character: Some(Vec::from(["}".to_string(), ";".to_string()])),
            }),
            rename_provider: Some(OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: WorkDoneProgressOptions::default(),
            })),
            folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
            selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(
                    SemanticTokensOptions {
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::NAMESPACE,
                                SemanticTokenType::TYPE,
                                SemanticTokenType::CLASS,
                                SemanticTokenType::ENUM,
                                SemanticTokenType::INTERFACE,
                                SemanticTokenType::STRUCT,
                                SemanticTokenType::FUNCTION,
                                SemanticTokenType::VARIABLE,
                                SemanticTokenType::PROPERTY,
                                SemanticTokenType::KEYWORD,
                                SemanticTokenType::COMMENT,
                                SemanticTokenType::STRING,
                                SemanticTokenType::NUMBER,
                                SemanticTokenType::OPERATOR,
                            ],
                            token_modifiers: vec![
                                SemanticTokenModifier::DECLARATION,
                                SemanticTokenModifier::DEFINITION,
                                SemanticTokenModifier::READONLY,
                                SemanticTokenModifier::STATIC,
                                SemanticTokenModifier::DEPRECATED,
                                SemanticTokenModifier::ASYNC,
                            ],
                        },
                        range: Some(true),
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                    },
                ),
            ),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                    did_create: Some(FileOperationRegistrationOptions {
                        filters: vec![FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.csd".to_string(),
                                matches: Some(FileOperationPatternKind::File),
                                options: None,
                            },
                        }],
                    }),
                    will_create: None,
                    did_rename: Some(FileOperationRegistrationOptions {
                        filters: vec![FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.csd".to_string(),
                                matches: Some(FileOperationPatternKind::File),
                                options: None,
                            },
                        }],
                    }),
                    will_rename: None,
                    did_delete: Some(FileOperationRegistrationOptions {
                        filters: vec![FileOperationFilter {
                            scheme: Some("file".to_string()),
                            pattern: FileOperationPattern {
                                glob: "**/*.csd".to_string(),
                                matches: Some(FileOperationPatternKind::File),
                                options: None,
                            },
                        }],
                    }),
                    will_delete: None,
                }),
            }),
            ..ServerCapabilities::default()
        }
    }

    /// Get the client handle
    async fn client(&self) -> Option<Client> {
        self.client.read().await.clone()
    }

    /// Publish diagnostics for a document
    #[instrument(skip(self))]
    async fn publish_diagnostics(&self, uri: Url, diagnostics: Vec<Diagnostic>) {
        if let Some(client) = self.client.read().await.clone() {
            client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    /// Analyze document and publish diagnostics
    #[instrument(skip(self, content))]
    async fn analyze_document(&self, uri: &Url, content: &str) {
        debug!("Analyzing document: {}", uri);
        
        // Get diagnostics from various providers
        let syntax_diagnostics = self.diagnostics.get_syntax_diagnostics(content).await;
        let semantic_diagnostics = self.diagnostics.get_semantic_diagnostics(content).await;
        let lint_diagnostics = self.diagnostics.get_lint_diagnostics(content).await;

        // Combine all diagnostics
        let mut all_diagnostics = Vec::new();
        all_diagnostics.extend(syntax_diagnostics);
        all_diagnostics.extend(semantic_diagnostics);
        all_diagnostics.extend(lint_diagnostics);

        // Publish diagnostics
        self.publish_diagnostics(uri.clone(), all_diagnostics).await;
    }

    /// Custom method: Get AST node at position
    pub async fn get_ast_node(
        &self,
        params: Value,
    ) -> LspResult<Value> {
        // Implementation for getting AST node information
        // This is a CURSED-specific extension
        Ok(serde_json::json!({
            "node_type": "function_declaration",
            "range": params.get("range"),
            "children": []
        }))
    }

    /// Custom method: Get type information at position
    pub async fn get_type_info(
        &self,
        params: Value,
    ) -> LspResult<Value> {
        // Implementation for getting type information
        // This is a CURSED-specific extension
        Ok(serde_json::json!({
            "type": "string",
            "nullable": false,
            "generic_params": []
        }))
    }

    /// Custom method: Format document with CURSED formatter
    pub async fn format_document_custom(
        &self,
        params: Value,
    ) -> LspResult<Value> {
        // Implementation for custom formatting
        if let Some(uri_value) = params.get("textDocument").and_then(|td| td.get("uri")) {
            if let Some(uri_str) = uri_value.as_str() {
                if let Ok(uri) = Url::parse(uri_str) {
                    if let Some(content) = self.document_manager.get_document_content(&uri).await {
                        let formatted = self.formatting.format_document(&content).await
                            .unwrap_or_else(|_| content);
                        return Ok(serde_json::json!({
                            "formatted_content": formatted
                        }));
                    }
                }
            }
        }
        
        Err(LspError::invalid_params("Invalid document URI"))
    }

    /// Custom method: Run linter on document
    pub async fn run_linter(
        &self,
        params: Value,
    ) -> LspResult<Value> {
        // Implementation for running linter
        Ok(serde_json::json!({
            "linter_results": [],
            "warnings_count": 0,
            "errors_count": 0
        }))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for CursedLanguageServer {
    #[instrument(skip(self, params))]
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        info!("Initializing CURSED Language Server");
        
        // Store workspace information
        if let Some(workspace_folders) = params.workspace_folders {
            self.workspace.set_workspace_folders(workspace_folders).await;
        } else if let Some(root_uri) = params.root_uri {
            self.workspace.set_root_uri(root_uri).await;
        }

        Ok(InitializeResult {
            capabilities: self.capabilities.clone(),
            server_info: Some(ServerInfo {
                name: "CURSED Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    #[instrument(skip(self))]
    async fn initialized(&self, _: InitializedParams) {
        info!("CURSED Language Server initialized");
        
        // Register for file system events
        if let Some(client) = self.client().await {
            let registration = Registration {
                id: "workspace/didChangeWatchedFiles".to_string(),
                method: "workspace/didChangeWatchedFiles".to_string(),
                register_options: Some(
                    serde_json::to_value(DidChangeWatchedFilesRegistrationOptions {
                        watchers: vec![
                            FileSystemWatcher {
                                glob_pattern: GlobPattern::String("**/*.csd".to_string()),
                                kind: Some(WatchKind::all()),
                            }
                        ],
                    }).unwrap(),
                ),
            };

            client
                .register_capability(Vec::from([registration]))
                .await
                .unwrap_or_else(|err| {
                    warn!("Failed to register file watcher: {}", err);
                });
        }
    }

    #[instrument(skip(self))]
    async fn shutdown(&self) -> LspResult<()> {
        info!("Shutting down CURSED Language Server");
        Ok(())
    }

    #[instrument(skip(self, params))]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        info!("Document opened: {}", params.text_document.uri);
        
        // Store document
        self.document_manager
            .open_document(
                params.text_document.uri.clone(),
                params.text_document.text.clone(),
                params.text_document.version,
            )
            .await;

        // Analyze and provide diagnostics
        self.analyze_document(&params.text_document.uri, &params.text_document.text)
            .await;
    }

    #[instrument(skip(self, params))]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        debug!("Document changed: {}", params.text_document.uri);
        
        // Update document with incremental changes
        if let Some(new_content) = self.document_manager
            .update_document(
                params.text_document.uri.clone(),
                params.content_changes,
                params.text_document.version,
            )
            .await
        {
            // Re-analyze document
            self.analyze_document(&params.text_document.uri, &new_content)
                .await;
        }
    }

    #[instrument(skip(self, params))]
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("Document closed: {}", params.text_document.uri);
        
        // Remove document and clear diagnostics
        self.document_manager
            .close_document(params.text_document.uri.clone())
            .await;
        
        self.publish_diagnostics(params.text_document.uri, Vec::from([]))
            .await;
    }

    #[instrument(skip(self, params))]
    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        debug!("Completion requested at {:?}", params.text_document_position);
        
        let uri = &params.text_document_position.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let position = params.text_document_position.position;
            let completions = self.completion
                .get_completions(&content, position)
                .await;
            
            return Ok(Some(CompletionResponse::Array(completions)));
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        debug!("Hover requested at {:?}", params.text_document_position_params);
        
        let uri = &params.text_document_position_params.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let position = params.text_document_position_params.position;
            return Ok(self.navigation.get_hover_info(&content, position).await);
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> LspResult<Option<GotoDefinitionResponse>> {
        debug!("Go to definition requested at {:?}", params.text_document_position_params);
        
        let uri = &params.text_document_position_params.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let position = params.text_document_position_params.position;
            return Ok(self.navigation.get_definition(&content, position, uri).await);
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn references(&self, params: ReferenceParams) -> LspResult<Option<Vec<Location>>> {
        debug!("Find references requested at {:?}", params.text_document_position);
        
        let uri = &params.text_document_position.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let position = params.text_document_position.position;
            return Ok(Some(self.navigation.find_references(&content, position, uri).await));
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn formatting(&self, params: DocumentFormattingParams) -> LspResult<Option<Vec<TextEdit>>> {
        debug!("Document formatting requested for {}", params.text_document.uri);
        
        if let Some(content) = self.document_manager.get_document_content(&params.text_document.uri).await {
            return Ok(self.formatting.format_document_edits(&content, params.options).await);
        }
        
        Ok(None)
    }
}

impl Default for CursedLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}
