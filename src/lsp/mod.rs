//! Language Server Protocol implementation for CURSED
//! 
//! This module provides a full-featured LSP server that enables IDE integration
//! with modern editors like VS Code, Vim, Emacs, and others.

pub mod server;
pub mod backend;
pub mod document;
pub mod protocol;
pub mod diagnostics;
pub mod completion;
pub mod navigation;
pub mod formatting;
pub mod workspace;
pub mod semantic_highlighting;
pub mod code_lens;
pub mod inlay_hints;
pub mod enhanced_symbols;
pub mod refactoring;

pub use server::*;
pub use backend::*;

use tracing::instrument;

/// Initialize the LSP server with tracing
#[instrument]
pub fn init_lsp_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for LSP server
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into())
        )
        .init();

    Ok(())
}
