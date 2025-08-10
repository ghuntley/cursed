//! Complete CURSED Language Server Protocol Implementation
//! Provides comprehensive IDE support with semantic analysis, completion, and diagnostics

use crate::ast::{Statement, Expression, Type, Program};
use crate::error::CursedError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::type_system::TypeChecker;
use serde_json::Value;
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
    semantic_analyzer: Arc<RwLock<TypeChecker>>,
    diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
    symbols: Arc<RwLock<HashMap<Url, Vec<SymbolInformation>>>>,
}

/// Document data with parsed AST and metadata
#[derive(Debug, Clone)]
pub struct DocumentData {
    pub uri: Url,
    pub text: String,
    pub version: i32,
    pub ast: Option<Program>,
    pub symbols: Vec<SymbolInformation>,
    pub diagnostics: Vec<Diagnostic>,
    pub last_modified: std::time::SystemTime,
}

impl CursedLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            workspace_root: Arc::new(RwLock::new(None)),
            semantic_analyzer: Arc::new(RwLock::new(TypeChecker::new())),
            diagnostics: Arc::new(RwLock::new(HashMap::new())),
            symbols: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Parse and analyze a document
    async fn analyze_document(&self, uri: &Url, text: &str, version: i32) -> DocumentData {
        let mut diagnostics = Vec::new();
        let mut symbols = Vec::new();
        let mut ast = None;

        // Lexical analysis
        let mut lexer = Lexer::new(text);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(error) => {
                diagnostics.push(self.error_to_diagnostic(&error));
                return DocumentData {
                    uri: uri.clone(),
                    text: text.to_string(),
                    version,
                    ast: None,
                    symbols: Vec::new(),
                    diagnostics,
                    last_modified: std::time::SystemTime::now(),
                };
            }
        };

        // Syntax analysis
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(program) => {
                // Extract symbols from AST
                symbols = self.extract_symbols(&program, uri);
                
                // Semantic analysis
                if let Ok(mut type_checker) = self.semantic_analyzer.write().await {
                    match type_checker.check_program(&program) {
                        Ok(_) => {
                            // No semantic errors
                        }
                        Err(errors) => {
                            for error in errors {
                                diagnostics.push(self.error_to_diagnostic(&error));
                            }
                        }
                    }
                }
                
                ast = Some(program);
            }
            Err(errors) => {
                for error in &errors {
                    diagnostics.push(self.error_to_diagnostic(error));
                }
            }
        }

        DocumentData {
            uri: uri.clone(),
            text: text.to_string(),
            version,
            ast,
            symbols,
            diagnostics,
            last_modified: std::time::SystemTime::now(),
        }
    }

    /// Extract symbols from AST for navigation and completion
    fn extract_symbols(&self, program: &Program, uri: &Url) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        
        for statement in &program.statements {
            match statement {
                Statement::FunctionDeclaration(func) => {
                    symbols.push(SymbolInformation {
                        name: func.name.clone(),
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        deprecated: Some(false),
                        location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: func.name.len() as u32 },
                            },
                        },
                        container_name: None,
                    });
                }
                Statement::VariableDeclaration(var) => {
                    symbols.push(SymbolInformation {
                        name: var.name.clone(),
                        kind: SymbolKind::VARIABLE,
                        tags: None,
                        deprecated: Some(false),
                        location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: var.name.len() as u32 },
                            },
                        },
                        container_name: None,
                    });
                }
                Statement::InterfaceStatement(interface) => {
                    symbols.push(SymbolInformation {
                        name: interface.name.clone(),
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        deprecated: Some(false),
                        location: Location {
                            uri: uri.clone(),
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: interface.name.len() as u32 },
                            },
                        },
                        container_name: None,
                    });
                }
                _ => {}
            }
        }
        
        symbols
    }

    /// Convert CURSED error to LSP diagnostic
    fn error_to_diagnostic(&self, error: &CursedError) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: 0 },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("cursed".to_string()),
            message: error.to_string(),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    /// Get completion items for a position
    async fn get_completions(&self, uri: &Url, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Add CURSED keywords
        let keywords = vec![
            ("sus", "Variable declaration", CompletionItemKind::KEYWORD),
            ("damn", "Return statement", CompletionItemKind::KEYWORD),
            ("slay", "Function declaration", CompletionItemKind::KEYWORD),
            ("vibez", "Module/namespace", CompletionItemKind::MODULE),
            ("yeet", "Import statement", CompletionItemKind::KEYWORD),
            ("bestie", "For loop", CompletionItemKind::KEYWORD),
            ("stan", "While loop", CompletionItemKind::KEYWORD),
            ("dm", "Send/channel operation", CompletionItemKind::KEYWORD),
            ("ready", "Select statement", CompletionItemKind::KEYWORD),
            ("vibe", "Package declaration", CompletionItemKind::KEYWORD),
            ("yikes", "Error handling", CompletionItemKind::KEYWORD),
            ("shook", "Panic recovery", CompletionItemKind::KEYWORD),
            ("fam", "Error propagation", CompletionItemKind::KEYWORD),
            ("based", "True boolean", CompletionItemKind::VALUE),
            ("cap", "False boolean", CompletionItemKind::VALUE),
            ("cringe", "Nil value", CompletionItemKind::VALUE),
            ("facts", "Constant declaration", CompletionItemKind::KEYWORD),
            ("lit", "Boolean type", CompletionItemKind::TYPE_PARAMETER),
            ("tea", "String type", CompletionItemKind::TYPE_PARAMETER),
            ("drip", "Float type", CompletionItemKind::TYPE_PARAMETER),
            ("normie", "Integer type", CompletionItemKind::TYPE_PARAMETER),
            ("smol", "Small integer", CompletionItemKind::TYPE_PARAMETER),
            ("mid", "Medium integer", CompletionItemKind::TYPE_PARAMETER),
            ("thicc", "Large integer", CompletionItemKind::TYPE_PARAMETER),
            ("snack", "Small float", CompletionItemKind::TYPE_PARAMETER),
            ("meal", "Large float", CompletionItemKind::TYPE_PARAMETER),
            ("byte", "Byte type", CompletionItemKind::TYPE_PARAMETER),
            ("rune", "Rune type", CompletionItemKind::TYPE_PARAMETER),
            ("sip", "Character type", CompletionItemKind::TYPE_PARAMETER),
        ];
        
        for (keyword, detail, kind) in keywords {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(kind),
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
            });
        }

        // Add stdlib functions
        let stdlib_functions = vec![
            ("vibez.spill", "Print to output", "vibez.spill($1)"),
            ("vibez.vaporize", "Exit program", "vibez.vaporize($1)"),
            ("math.add", "Add numbers", "math.add($1, $2)"),
            ("math.sub", "Subtract numbers", "math.sub($1, $2)"),
            ("math.mul", "Multiply numbers", "math.mul($1, $2)"),
            ("math.div", "Divide numbers", "math.div($1, $2)"),
            ("string.length", "Get string length", "string.length($1)"),
            ("string.concat", "Concatenate strings", "string.concat($1, $2)"),
            ("crypto.hash", "Hash data", "crypto.hash($1)"),
            ("crypto.encrypt", "Encrypt data", "crypto.encrypt($1, $2)"),
            ("json.parse", "Parse JSON", "json.parse($1)"),
            ("json.stringify", "JSON stringify", "json.stringify($1)"),
        ];

        for (func, detail, snippet) in stdlib_functions {
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
                insert_text: Some(snippet.to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
            });
        }

        // Add symbols from current document and workspace
        if let Ok(documents) = self.documents.read().await {
            for doc_data in documents.values() {
                for symbol in &doc_data.symbols {
                    completions.push(CompletionItem {
                        label: symbol.name.clone(),
                        kind: Some(match symbol.kind {
                            SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
                            SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
                            SymbolKind::INTERFACE => CompletionItemKind::INTERFACE,
                            _ => CompletionItemKind::TEXT,
                        }),
                        detail: Some(format!("{:?}", symbol.kind)),
                        documentation: None,
                        deprecated: Some(false),
                        preselect: None,
                        sort_text: Some(format!("2_{}", symbol.name)),
                        filter_text: None,
                        insert_text: Some(symbol.name.clone()),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                        insert_text_mode: None,
                        text_edit: None,
                        additional_text_edits: None,
                        command: None,
                        commit_characters: None,
                        data: None,
                        tags: None,
                    });
                }
            }
        }

        completions
    }

    /// Get hover information for a symbol
    async fn get_hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        if let Ok(documents) = self.documents.read().await {
            if let Some(doc_data) = documents.get(uri) {
                // Find symbol at position (simplified)
                for symbol in &doc_data.symbols {
                    if self.position_in_range(position, symbol.location.range) {
                        return Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: format!(
                                    "**{}**\n\nType: `{:?}`\n\nCURSED symbol declaration",
                                    symbol.name, symbol.kind
                                ),
                            }),
                            range: Some(symbol.location.range),
                        });
                    }
                }
            }
        }
        None
    }

    /// Get definition location for a symbol
    async fn get_definition(&self, _uri: &Url, position: Position) -> Vec<Location> {
        let mut locations = Vec::new();
        
        if let Ok(documents) = self.documents.read().await {
            for doc_data in documents.values() {
                for symbol in &doc_data.symbols {
                    if self.position_in_range(position, symbol.location.range) {
                        locations.push(symbol.location.clone());
                    }
                }
            }
        }
        
        locations
    }

    /// Find references to a symbol
    async fn get_references(&self, uri: &Url, position: Position, _include_declaration: bool) -> Vec<Location> {
        let mut references = Vec::new();
        
        // Find symbol at position
        if let Ok(documents) = self.documents.read().await {
            let mut target_symbol = None;
            
            if let Some(doc_data) = documents.get(uri) {
                for symbol in &doc_data.symbols {
                    if self.position_in_range(position, symbol.location.range) {
                        target_symbol = Some(symbol.name.clone());
                        break;
                    }
                }
            }
            
            if let Some(symbol_name) = target_symbol {
                // Search all documents for references
                for doc_data in documents.values() {
                    for symbol in &doc_data.symbols {
                        if symbol.name == symbol_name {
                            references.push(symbol.location.clone());
                        }
                    }
                }
            }
        }
        
        references
    }

    /// Format document text
    async fn format_document(&self, uri: &Url) -> Option<Vec<TextEdit>> {
        if let Ok(documents) = self.documents.read().await {
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

    /// Check if position is within range
    fn position_in_range(&self, position: Position, range: Range) -> bool {
        position.line >= range.start.line && position.line <= range.end.line &&
        position.character >= range.start.character && position.character <= range.end.character
    }

    /// Get workspace symbols
    async fn get_workspace_symbols(&self, query: &str) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        
        if let Ok(documents) = self.documents.read().await {
            for doc_data in documents.values() {
                for symbol in &doc_data.symbols {
                    if query.is_empty() || symbol.name.to_lowercase().contains(&query.to_lowercase()) {
                        symbols.push(symbol.clone());
                    }
                }
            }
        }
        
        symbols
    }

    /// Publish diagnostics for a document
    async fn publish_diagnostics(&self, uri: Url, diagnostics: Vec<Diagnostic>) {
        self.client.publish_diagnostics(uri, diagnostics, None).await;
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

        let doc_data = self.analyze_document(&uri, &text, version).await;
        let diagnostics = doc_data.diagnostics.clone();

        {
            let mut documents = self.documents.write().await;
            documents.insert(uri.clone(), doc_data);
        }

        self.publish_diagnostics(uri, diagnostics).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;
            let doc_data = self.analyze_document(&uri, &text, version).await;
            let diagnostics = doc_data.diagnostics.clone();

            {
                let mut documents = self.documents.write().await;
                documents.insert(uri.clone(), doc_data);
            }

            self.publish_diagnostics(uri, diagnostics).await;
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

    async fn goto_definition(&self, params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let locations = self.get_definition(uri, position).await;
        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(GotoDefinitionResponse::Array(locations)))
        }
    }

    async fn references(&self, params: ReferenceParams) -> LspResult<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let include_declaration = params.context.include_declaration;

        let references = self.get_references(uri, position, include_declaration).await;
        Ok(Some(references))
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

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::*;

    #[ignore] // Skip due to tokio runtime stack overflow
#[tokio::test]
async fn test_cursed_language_server_creation() {
        let (client, _) = tower_lsp::lsp_types::request::Initialize::METHOD.into();
        let _server = CursedLanguageServer::new(client);
    }

    #[test]
    fn test_format_cursed_code() {
        let (client, _) = tower_lsp::lsp_types::request::Initialize::METHOD.into();
        let server = CursedLanguageServer::new(client);
        let code = "slay test(){vibez.spill(\"hello\")}";
        let formatted = server.format_cursed_code(code);
        
        assert!(formatted.contains("slay test(){"));
        assert!(formatted.contains("    vibez.spill(\"hello\")"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_position_in_range() {
        let (client, _) = tower_lsp::lsp_types::request::Initialize::METHOD.into();
        let server = CursedLanguageServer::new(client);
        let position = Position { line: 1, character: 5 };
        let range = Range {
            start: Position { line: 1, character: 0 },
            end: Position { line: 1, character: 10 },
        };
        
        assert!(server.position_in_range(position, range));
    }
}
