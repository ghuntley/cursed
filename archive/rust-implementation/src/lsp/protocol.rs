//! LSP Protocol Implementation for CURSED Language Server
//! Handles LSP communication and message processing

use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use crate::lsp::server::CursedLanguageServer;
use std::sync::Arc;

/// LSP client for communication with the editor
pub trait CursedLspClient: Send + Sync {
    /// Send diagnostics to the client
    async fn publish_diagnostics(&self, uri: Url, diagnostics: Vec<Diagnostic>, version: Option<i32>);
    
    /// Show message to the user
    async fn show_message(&self, typ: MessageType, message: String);
    
    /// Log message
    async fn log_message(&self, typ: MessageType, message: String);
}

/// CURSED-specific LSP capabilities
pub fn get_cursed_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
            completion_item: None,
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
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
            more_trigger_character: Some(vec!["}".to_string(), ";".to_string()]),
        }),
        rename_provider: Some(OneOf::Left(true)),
        document_link_provider: Some(DocumentLinkOptions {
            resolve_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec![
                "cursed.format".to_string(),
                "cursed.compile".to_string(),
                "cursed.run".to_string(),
            ],
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: None,
        }),
        semantic_tokens_provider: Some(
            SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                SemanticTokensRegistrationOptions {
                    text_document_registration_options: {
                        TextDocumentRegistrationOptions {
                            document_selector: Some(vec![DocumentFilter {
                                language: Some("cursed".to_string()),
                                scheme: Some("file".to_string()),
                                pattern: Some("**/*.csd".to_string()),
                            }]),
                        }
                    },
                    semantic_tokens_options: SemanticTokensOptions {
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::KEYWORD,
                                SemanticTokenType::STRING,
                                SemanticTokenType::NUMBER,
                                SemanticTokenType::OPERATOR,
                                SemanticTokenType::FUNCTION,
                                SemanticTokenType::VARIABLE,
                                SemanticTokenType::TYPE,
                                SemanticTokenType::COMMENT,
                                SemanticTokenType::PARAMETER,
                                SemanticTokenType::PROPERTY,
                            ],
                            token_modifiers: vec![
                                SemanticTokenModifier::DEFINITION,
                                SemanticTokenModifier::READONLY,
                                SemanticTokenModifier::STATIC,
                            ],
                        },
                        range: Some(true),
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                    },
                    static_registration_options: StaticRegistrationOptions::default(),
                }
            )
        ),
        inlay_hint_provider: Some(OneOf::Left(true)),
        ..ServerCapabilities::default()
    }
}
