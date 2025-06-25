// Minimal module - heavy features disabled

pub mod cursed_repl;
pub mod session_manager;
pub mod types;

pub use cursed_repl::CursedRepl;
pub use session_manager::SessionManager;
pub use types::{
    ReplConfig, ReplState, InputType, ReplOutput, ReplResult, 
    SyntaxHighlighter, CommandSystem, TabCompletion, MultiLineEditor,
    BuildIntegration, ReplEvaluator
};
