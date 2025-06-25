use crate::error::CursedError;
/// Environment variables module for CURSED standard library
/// 
/// Provides comprehensive environment variable operations with cross-platform support,
/// type safety, and robust error handling.

pub mod error;
pub mod core;
pub mod parsing;
pub mod expansion;

// Re-export all public types and functions
pub use error::{EnvError, EnvResult, env_error, not_found_error, invalid_value_error, permission_error};
pub use core::{
    get_path_separator, is_case_sensitive_env, get_env_case_insensitive
// };
pub use parsing::{
    parse_env_config, parse_env_duration, parse_env_memory_size
// };
pub use expansion::{
    escape_env_value, unescape_env_value
// };

// Platform-specific utilities
#[cfg(target_os = "windows")]
pub use core::get_path_separator as windows_path_separator;

#[cfg(not(target_os = "windows"))]
pub use core::get_path_separator as unix_path_separator;
