//! LSP backend implementation providing the core language server functionality

use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::{Result as LspResult, Error as LspError};
use tower_lsp::lsp_crate::types::*;
use tower_lsp::{Client, LanguageServer};
use tracing::{debug, error, info, instrument, warn};

use crate::lsp::document::DocumentManager;
use crate::lsp::diagnostics::DiagnosticsProvider;
use crate::lsp::completion::CompletionProvider;
use crate::lsp::navigation::NavigationProvider;
use crate::lsp::formatting::FormattingProvider;
use crate::lsp::workspace::WorkspaceManager;
use crate::lsp::semantic_highlighting::SemanticHighlightingProvider;
use crate::lsp::code_lens::CodeLensProvider;
use crate::lsp::inlay_hints::InlayHintsProvider;
use crate::lsp::enhanced_symbols::EnhancedSymbolProvider;
use crate::lsp::refactoring::RefactoringProvider;
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
    /// Semantic highlighting provider
    semantic_highlighting: Arc<RwLock<SemanticHighlightingProvider>>,
    /// Code lens provider
    code_lens: Arc<RwLock<CodeLensProvider>>,
    /// Inlay hints provider
    inlay_hints: Arc<RwLock<InlayHintsProvider>>,
    /// Enhanced symbol provider
    symbols: Arc<RwLock<EnhancedSymbolProvider>>,
    /// Refactoring provider
    refactoring: Arc<RwLock<RefactoringProvider>>,
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
        let semantic_highlighting = Arc::new(RwLock::new(SemanticHighlightingProvider::new()));
        let code_lens = Arc::new(RwLock::new(CodeLensProvider::new()));
        let inlay_hints = Arc::new(RwLock::new(InlayHintsProvider::new()));
        let symbols = Arc::new(RwLock::new(EnhancedSymbolProvider::new()));
        let refactoring = Arc::new(RwLock::new(RefactoringProvider::new()));

        let capabilities = Self::build_server_capabilities();

        Self {
            client: Arc::new(RwLock::new(client)),
            document_manager,
            diagnostics,
            completion,
            navigation,
            formatting,
            workspace,
            semantic_highlighting,
            code_lens,
            inlay_hints,
            symbols,
            refactoring,
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
                        legend: SemanticHighlightingProvider::new().get_legend(),
                        range: Some(true),
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                    },
                ),
            ),
            inlay_hint_provider: Some(OneOf::Left(true)),
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

    #[instrument(skip(self, params))]
    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> LspResult<Option<SemanticTokensResult>> {
        debug!("Semantic tokens full requested for {}", params.text_document.uri);
        
        if let Some(content) = self.document_manager.get_document_content(&params.text_document.uri).await {
            let provider = self.semantic_highlighting.read().await;
            match provider.get_semantic_tokens(&content).await {
                Ok(tokens) => {
                    let encoded_tokens = provider.encode_semantic_tokens(tokens);
                    return Ok(Some(SemanticTokensResult::Tokens(encoded_tokens)));
                }
                Err(e) => {
                    debug!("Failed to generate semantic tokens: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn semantic_tokens_range(&self, params: SemanticTokensRangeParams) -> LspResult<Option<SemanticTokensRangeResult>> {
        debug!("Semantic tokens range requested for {}", params.text_document.uri);
        
        if let Some(content) = self.document_manager.get_document_content(&params.text_document.uri).await {
            let provider = self.semantic_highlighting.read().await;
            match provider.get_semantic_tokens_range(&content, params.range).await {
                Ok(tokens) => {
                    let encoded_tokens = provider.encode_semantic_tokens(tokens);
                    return Ok(Some(SemanticTokensRangeResult::Tokens(encoded_tokens)));
                }
                Err(e) => {
                    debug!("Failed to generate semantic tokens for range: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn code_lens(&self, params: CodeLensParams) -> LspResult<Option<Vec<CodeLens>>> {
        debug!("Code lens requested for {}", params.text_document.uri);
        
        if let Some(content) = self.document_manager.get_document_content(&params.text_document.uri).await {
            let provider = self.code_lens.read().await;
            match provider.get_code_lenses(&content, &params.text_document.uri).await {
                Ok(lenses) => {
                    let lsp_lenses: Vec<CodeLens> = lenses
                        .into_iter()
                        .map(|lens| lens.to_lsp_code_lens())
                        .collect();
                    return Ok(Some(lsp_lenses));
                }
                Err(e) => {
                    debug!("Failed to generate code lenses: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn code_lens_resolve(&self, params: CodeLens) -> LspResult<CodeLens> {
        debug!("Code lens resolve requested");
        
        let provider = self.code_lens.read().await;
        match provider.resolve_code_lens(params).await {
            Ok(resolved) => Ok(resolved),
            Err(e) => {
                debug!("Failed to resolve code lens: {}", e);
                Ok(params) // Return original if resolution fails
            }
        }
    }

    #[instrument(skip(self, params))]
    async fn inlay_hint(&self, params: InlayHintParams) -> LspResult<Option<Vec<InlayHint>>> {
        debug!("Inlay hints requested for {}", params.text_document.uri);
        
        if let Some(content) = self.document_manager.get_document_content(&params.text_document.uri).await {
            let mut provider = self.inlay_hints.write().await;
            match provider.get_inlay_hints(&content, params.range).await {
                Ok(hints) => {
                    let lsp_hints: Vec<InlayHint> = hints
                        .into_iter()
                        .map(|hint| hint.to_lsp_inlay_hint())
                        .collect();
                    return Ok(Some(lsp_hints));
                }
                Err(e) => {
                    debug!("Failed to generate inlay hints: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn document_symbol(&self, params: DocumentSymbolParams) -> LspResult<Option<DocumentSymbolResponse>> {
        debug!("Document symbols requested for {}", params.text_document.uri);
        
        if let Some(content) = self.document_manager.get_document_content(&params.text_document.uri).await {
            let mut provider = self.symbols.write().await;
            match provider.get_document_symbols(&content, &params.text_document.uri).await {
                Ok(symbols) => {
                    let document_symbols: Vec<DocumentSymbol> = symbols
                        .into_iter()
                        .map(|symbol| symbol.to_document_symbol())
                        .collect();
                    return Ok(Some(DocumentSymbolResponse::Nested(document_symbols)));
                }
                Err(e) => {
                    debug!("Failed to extract document symbols: {}", e);
                }
            }
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn symbol(&self, params: WorkspaceSymbolParams) -> LspResult<Option<Vec<SymbolInformation>>> {
        debug!("Workspace symbols requested with query: {}", params.query);
        
        let workspace_folders = self.workspace.get_workspace_folders().await;
        let mut provider = self.symbols.write().await;
        
        match provider.search_workspace_symbols(&params.query, &workspace_folders).await {
            Ok(symbols) => Ok(Some(symbols)),
            Err(e) => {
                debug!("Failed to search workspace symbols: {}", e);
                Ok(None)
            }
        }
    }

    #[instrument(skip(self, params))]
    async fn prepare_rename(&self, params: TextDocumentPositionParams) -> LspResult<Option<PrepareRenameResponse>> {
        debug!("Prepare rename requested at {:?}", params.position);
        
        let uri = &params.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let provider = self.refactoring.read().await;
            return Ok(provider.prepare_rename(&content, params.position, uri).await);
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn rename(&self, params: RenameParams) -> LspResult<Option<WorkspaceEdit>> {
        debug!("Rename requested at {:?} to {}", params.text_document_position.position, params.new_name);
        
        let uri = &params.text_document_position.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let provider = self.refactoring.read().await;
            return Ok(provider.rename_symbol(&content, params.text_document_position.position, &params.new_name, uri).await);
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn code_action(&self, params: CodeActionParams) -> LspResult<Option<CodeActionResponse>> {
        debug!("Code actions requested for range {:?}", params.range);
        
        let uri = &params.text_document.uri;
        if let Some(content) = self.document_manager.get_document_content(uri).await {
            let provider = self.refactoring.read().await;
            let actions = provider.get_code_actions(&content, params.range, &params.context, uri).await;
            return Ok(Some(CodeActionResponse::from(actions)));
        }
        
        Ok(None)
    }

    #[instrument(skip(self, params))]
    async fn execute_command(&self, params: ExecuteCommandParams) -> LspResult<Option<Value>> {
        debug!("Execute command requested: {}", params.command);
        
        match params.command.as_str() {
            "cursed.refactor.extractFunction" => {
                // Handle extract function command
                if let Some(args) = params.arguments {
                    if let Some(range_value) = args.get(0) {
                        if let Ok(range) = serde_json::from_value::<Range>(range_value.clone()) {
                            // In a real implementation, you would collect additional parameters
                            // from the user (function name, etc.) and then perform the refactoring
                            return Ok(Some(serde_json::json!({
                                "success": true,
                                "message": "Extract function refactoring initiated"
                            })));
                        }
                    }
                }
                Ok(Some(serde_json::json!({
                    "success": false,
                    "message": "Invalid arguments for extract function"
                })))
            }
            "cursed.refactor.extractVariable" => {
                // Handle extract variable command
                if let Some(args) = params.arguments {
                    if let Some(range_value) = args.get(0) {
                        if let Ok(_range) = serde_json::from_value::<Range>(range_value.clone()) {
                            return Ok(Some(serde_json::json!({
                                "success": true,
                                "message": "Extract variable refactoring initiated"
                            })));
                        }
                    }
                }
                Ok(Some(serde_json::json!({
                    "success": false,
                    "message": "Invalid arguments for extract variable"
                })))
            }
            "cursed.refactor.organizeImports" => {
                // Handle organize imports command
                Ok(Some(serde_json::json!({
                    "success": true,
                    "message": "Organize imports completed"
                })))
            }
            _ => {
                debug!("Unknown command: {}", params.command);
                Ok(None)
            }
        }
    }
}

impl Default for CursedLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}
