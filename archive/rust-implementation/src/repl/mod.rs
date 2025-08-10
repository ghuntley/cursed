//! CURSED REPL (Read-Eval-Print-Loop) implementation
//! 
//! This module provides an interactive shell for the CURSED programming language
//! with advanced features including syntax highlighting, tab completion, 
//! multi-line input, and interactive debugging.

pub mod cursed_repl;
pub mod jit_repl;
pub mod session_manager;
pub mod types;

// Basic implementations (minimal stubs for compilation)
pub mod tab_completion;
pub mod syntax_highlighter;
pub mod multi_line_editor;
pub mod command_system;
pub mod build_integration;

// Advanced implementations (full-featured)
pub mod advanced_tab_completion;
pub mod advanced_syntax_highlighter;
pub mod advanced_multi_line_editor;
pub mod interactive_debugger;
pub mod enhanced_cursed_repl;

// Re-export main types
pub use cursed_repl::CursedRepl;
pub use jit_repl::JitRepl;
pub use session_manager::SessionManager;
pub use types::{
    BuildIntegration, ReplEvaluator, BasicReplEvaluator, BasicBuildIntegration, ReplValue
};

// Re-export advanced features
pub use enhanced_cursed_repl::{EnhancedCursedRepl, ReplConfig};
pub use advanced_tab_completion::CursedCompleter;
pub use advanced_syntax_highlighter::CursedSyntaxHighlighter;
pub use advanced_multi_line_editor::AdvancedMultiLineEditor;
pub use interactive_debugger::InteractiveDebugger;
