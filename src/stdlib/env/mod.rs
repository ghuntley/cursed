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
    get_env, set_env, remove_env, get_all_env, env_exists,
    get_env_with_default, clear_all_env, get_env_keys, get_env_values,
    get_current_dir, get_home_dir, get_temp_dir, get_username, get_hostname,
    get_path_separator, is_case_sensitive_env, get_env_case_insensitive
};
pub use parsing::{
    parse_env, parse_env_with_default, get_path_env, get_numeric_env,
    get_bool_env, get_float_env, get_int_env, parse_env_list,
    parse_env_colon_list, parse_env_semicolon_list, parse_env_path_list,
    parse_env_config, parse_env_duration, parse_env_memory_size
};
pub use expansion::{
    expand_env_vars, expand_env_vars_with_defaults, has_env_vars,
    validate_env_syntax, extract_env_vars, substitute_env_vars,
    escape_env_value, unescape_env_value
};

// Platform-specific utilities
#[cfg(target_os = "windows")]
pub use core::get_path_separator as windows_path_separator;

#[cfg(not(target_os = "windows"))]
pub use core::get_path_separator as unix_path_separator;
