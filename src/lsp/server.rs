//! CURSED Language Server Protocol Implementation
//! Provides comprehensive IDE support with semantic analysis

use crate::ast::{Ast, Statement, Expression, Type, Program};
use crate::error::CursedError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::type_system::TypeChecker;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tower_lsp::{LspService, Server, LanguageServer, Client};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tokio::sync::RwLock;

/// CURSED Language Server
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

/// Symbol location information for navigation
#[derive(Debug, Clone)]
pub struct SymbolLocation {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub uri: Url,
    pub definition_range: Range,
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
    fn analyze_document(&self, uri: &Url, text: &str) -> DocumentData {
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
                    version: 0,
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
                if let Ok(mut type_checker) = self.semantic_analyzer.lock() {
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
            version: 0,
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
    fn get_completions(&self, uri: &Url, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Add CURSED keywords
        let keywords = vec![
            "sus", "damn", "slay", "vibez", "yeet", "bestie", "stan", "dm", 
            "ready", "vibe", "yikes", "shook", "fam", "based", "cap", "cringe",
            "facts", "lit", "tea", "drip", "normie", "smol", "mid", "thicc",
            "snack", "meal", "byte", "rune", "extra", "sip"
        ];
        
        for keyword in keywords {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("CURSED keyword".to_string()),
                documentation: None,
                deprecated: Some(false),
                preselect: None,
                sort_text: None,
                filter_text: None,
                insert_text: None,
                insert_text_format: None,
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
        if let Ok(documents) = self.documents.lock() {
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
                        sort_text: None,
                        filter_text: None,
                        insert_text: None,
                        insert_text_format: None,
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
    fn get_hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        if let Ok(documents) = self.documents.lock() {
            if let Some(doc_data) = documents.get(uri) {
                // Find symbol at position (simplified)
                for symbol in &doc_data.symbols {
                    if self.position_in_range(position, symbol.location.range) {
                        return Some(Hover {
                            contents: HoverContents::Scalar(MarkedString::String(
                                format!("{}: {:?}", symbol.name, symbol.kind)
                            )),
                            range: Some(symbol.location.range),
                        });
                    }
                }
            }
        }
        None
    }

    /// Get definition location for a symbol
    fn get_definition(&self, uri: &Url, position: Position) -> Vec<Location> {
        let mut locations = Vec::new();
        
        if let Ok(documents) = self.documents.lock() {
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
    fn get_references(&self, uri: &Url, position: Position, include_declaration: bool) -> Vec<Location> {
        let mut references = Vec::new();
        
        // Find symbol at position
        if let Ok(documents) = self.documents.lock() {
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
    fn format_document(&self, uri: &Url) -> Option<Vec<TextEdit>> {
        if let Ok(documents) = self.documents.lock() {
            if let Some(doc_data) = documents.get(uri) {
                // Simple formatting (could be enhanced)
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
    fn get_workspace_symbols(&self, query: &str) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        
        if let Ok(documents) = self.documents.lock() {
            for doc_data in documents.values() {
                for symbol in &doc_data.symbols {
                    if query.is_empty() || symbol.name.contains(query) {
                        symbols.push(symbol.clone());
                    }
                }
            }
        }
        
        symbols
    }

    /// Get semantic tokens for syntax highlighting
    fn get_semantic_tokens(&self, uri: &Url) -> Option<SemanticTokens> {
        if let Ok(documents) = self.documents.lock() {
            if let Some(doc_data) = documents.get(uri) {
                let mut tokens_data = Vec::new();
                
                // Extract semantic tokens from AST (simplified)
                if let Some(ast) = &doc_data.ast {
                    self.extract_semantic_tokens(&ast, &mut tokens_data);
                }
                
                return Some(SemanticTokens {
                    result_id: None,
                    data: tokens_data,
                });
            }
        }
        None
    }

    /// Extract semantic tokens from AST
    fn extract_semantic_tokens(&self, _program: &Program, _tokens: &mut Vec<SemanticToken>) {
        // TODO: Implement semantic token extraction from AST
        // This would analyze the AST and generate semantic tokens for highlighting
    }
}

/// Create and start the LSP server
pub async fn start_lsp_server() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| CursedLanguageServer::new())
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursed_language_server_creation() {
        let _server = CursedLanguageServer::new();
    }

    #[test]
    fn test_format_cursed_code() {
        let server = CursedLanguageServer::new();
        let code = "slay test(){vibez.spill(\"hello\")}";
        let formatted = server.format_cursed_code(code);
        
        assert!(formatted.contains("slay test() {"));
        assert!(formatted.contains("    vibez.spill(\"hello\")"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_position_in_range() {
        let server = CursedLanguageServer::new();
        let position = Position { line: 1, character: 5 };
        let range = Range {
            start: Position { line: 1, character: 0 },
            end: Position { line: 1, character: 10 },
        };
        
        assert!(server.position_in_range(position, range));
    }
}
