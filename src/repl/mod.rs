//! CURSED REPL Module
//! 
//! Interactive Read-Eval-Print Loop implementation for CURSED with:
//! - Syntax highlighting and code completion
//! - Multi-line input support
//! - Built-in development commands
//! - Session management
//! - Build system integration

pub mod cursed_repl;
pub mod syntax_highlighter;
pub mod command_system;
pub mod session_manager;
pub mod tab_completion;
pub mod multi_line_editor;
pub mod build_integration;

pub use cursed_repl::CursedRepl;
pub use syntax_highlighter::SyntaxHighlighter;
pub use command_system::{CommandSystem, BuiltinCommand};
pub use session_manager::SessionManager;
pub use tab_completion::TabCompletion;
pub use multi_line_editor::MultiLineEditor;
pub use build_integration::BuildIntegration;

use crate::error::CursedError;

/// Result type for REPL operations
pub type ReplResult<T> = Result<T, CursedError>;

/// REPL configuration options
#[derive(Debug, Clone)]
pub struct ReplConfig {
    pub verbose: bool,
    pub enable_history: bool,
    pub enable_syntax_highlighting: bool,
    pub enable_tab_completion: bool,
    pub history_file: Option<std::path::PathBuf>,
    pub working_directory: Option<std::path::PathBuf>,
    pub timeout: std::time::Duration,
    pub max_history_size: usize,
    pub prompt: String,
    pub continuation_prompt: String,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            enable_history: true,
            enable_syntax_highlighting: true,
            enable_tab_completion: true,
            history_file: None,
            working_directory: None,
            timeout: std::time::Duration::from_secs(30),
            max_history_size: 1000,
            prompt: "cursed> ".to_string(),
            continuation_prompt: "...   > ".to_string(),
        }
    }
}

/// REPL state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ReplState {
    /// Normal interactive mode
    Interactive,
    /// Multi-line input mode
    MultiLine,
    /// Command execution mode
    Command,
    /// Exiting REPL
    Exiting,
    /// Error state requiring recovery
    Error(String),
}

/// REPL input type
#[derive(Debug, Clone)]
pub enum InputType {
    /// CURSED code to execute
    Code(String),
    /// Built-in command
    Command(String, Vec<String>),
    /// Empty input (just pressed enter)
    Empty,
    /// End of input (Ctrl+D)
    EndOfInput,
}

/// REPL output formatting
#[derive(Debug, Clone)]
pub struct ReplOutput {
    pub content: String,
    pub is_error: bool,
    pub show_type: bool,
    pub execution_time: Option<std::time::Duration>,
}

impl ReplOutput {
    pub fn success(content: String) -> Self {
        Self {
            content,
            is_error: false,
            show_type: false,
            execution_time: None,
        }
    }

    pub fn error(content: String) -> Self {
        Self {
            content,
            is_error: true,
            show_type: false,
            execution_time: None,
        }
    }

    pub fn with_type(mut self) -> Self {
        self.show_type = true;
        self
    }

    pub fn with_timing(mut self, duration: std::time::Duration) -> Self {
        self.execution_time = Some(duration);
        self
    }
}
