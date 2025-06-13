//! CLI Module
//! 
//! Command-line interface utilities and subcommand handlers.

pub mod package_manager;
pub mod test_commands;

// Re-export main CLI components
pub use package_manager::{add_package_commands, handle_package_command};
