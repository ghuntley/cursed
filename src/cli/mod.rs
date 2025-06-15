//! CLI Module
//! 
//! Command-line interface utilities and subcommand handlers.

pub mod package_manager;
pub mod test_commands;
pub mod jit_commands;
pub mod optimization_commands;
pub mod documentation;

// Re-export main CLI components
pub use package_manager::{add_package_commands, handle_package_command};
pub use jit_commands::{add_jit_commands, handle_jit_command};
pub use optimization_commands::{add_optimization_commands, handle_optimization_command};
pub use documentation::{add_documentation_commands, handle_documentation_command, generate_sample_config};
