//! CURSED Language Server Protocol Implementation
//! Provides comprehensive IDE support with semantic analysis, completion, and diagnostics

pub mod backend;
pub mod completion;
pub mod diagnostics;
pub mod document;
pub mod formatting;
pub mod navigation;
pub mod protocol;
pub mod server;
pub mod semantic_highlighting;
pub mod workspace;

pub use server::{CursedLanguageServer, start_lsp_server};
pub use protocol::CursedLspClient;
