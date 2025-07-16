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
pub mod server_complete;
pub mod simple_server;
pub mod minimal_server;
pub mod semantic_highlighting;
pub mod workspace;

pub use minimal_server::{CursedLanguageServer as MinimalCursedLanguageServer, start_lsp_server};
pub use simple_server::{CursedLanguageServer as SimpleCursedLanguageServer, start_lsp_server as start_simple_lsp_server};
pub use server::{CursedLanguageServer, start_lsp_server as start_original_lsp_server};
pub use server_complete::{start_lsp_server as start_complete_lsp_server};
pub use protocol::CursedLspClient;
