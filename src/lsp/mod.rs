//! CURSED Language Server Protocol Implementation (Minimal)
//! Provides basic IDE support with completion and diagnostics

pub mod backend;
pub mod minimal_server;

pub use minimal_server::{CursedLanguageServer, start_lsp_server};
