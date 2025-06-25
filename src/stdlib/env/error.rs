use crate::error::CursedError;
/// Environment variable error handling for CURSED standard library

use std::fmt;

/// Specialized error type for environment variable operations
#[derive(Debug, Clone, PartialEq)]
pub enum EnvError {
    /// Environment variable not found
    NotFound {
    /// Invalid value for environment variable
    InvalidValue {
    /// Permission denied when setting/removing environment variable
    PermissionDenied {
    /// Invalid key name (empty, contains null bytes, etc.)
    InvalidKey {
    /// System error when accessing environment
    SystemError {
    /// Unicode conversion error
    UnicodeError {
    /// Environment variable expansion error
    ExpansionError {
    /// General environment error
    General {
// impl fmt::Display for EnvError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             EnvError::NotFound { key, message } => {
//                 write!(f, "Environment variable '{}' not found: {}", key, message)
//             }
//             EnvError::InvalidValue { key, value, expected_type, message } => {
//                 write!(f, "Invalid value '{}' for environment variable '{}' (expected {}): {}", 
//                        value, key, expected_type, message)
//             }
//             EnvError::PermissionDenied { key, operation, message } => {
//                 write!(f, "Permission denied for {} operation on environment variable '{}': {}", 
//                        operation, key, message)
//             }
//             EnvError::InvalidKey { key, message } => {
//                 write!(f, "Invalid environment variable key '{}': {}", key, message)
//             }
//             EnvError::SystemError { operation, message, code } => {
//                 match code {
//                     Some(code) => write!(f, "System error during {}: {} (code: {})", operation, message, code),
//                     None => write!(f, "System error during {}: {}", operation, message),
//                 }
//             }
//             EnvError::UnicodeError { key, message } => {
//                 write!(f, "Unicode error for environment variable '{}': {}", key, message)
//             }
//             EnvError::ExpansionError { input, message } => {
//                 write!(f, "Environment variable expansion error for '{}': {}", input, message)
//             }
//             EnvError::General { message } => {
//                 write!(f, "Environment error: {}", message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for EnvError {}
// 
// impl From<EnvError> for CursedError {
//     fn from(err: EnvError) -> Self {
//         CursedError::Runtime(err.to_string())
//     }
// }

impl From<std::env::VarError> for EnvError {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => EnvError::General {
            std::env::VarError::NotUnicode(os_string) => EnvError::UnicodeError {
        }
    }
impl From<std::ffi::OsString> for EnvError {
    fn from(os_string: std::ffi::OsString) -> Self {
        EnvError::UnicodeError {
        }
    }
/// Result type for environment variable operations
pub type EnvResult<T> = std::result::Result<T, EnvError>;

/// Helper function to create environment variable not found errors
pub fn not_found_error(key: &str) -> EnvError {
    EnvError::NotFound {
    }
}

/// Helper function to create invalid value errors
pub fn invalid_value_error(key: &str, value: &str, expected_type: &str, message: &str) -> EnvError {
    EnvError::InvalidValue {
    }
}

/// Helper function to create permission denied errors
pub fn permission_error(key: &str, operation: &str) -> EnvError {
    EnvError::PermissionDenied {
    }
}

/// Helper function to create invalid key errors
pub fn invalid_key_error(key: &str, reason: &str) -> EnvError {
    EnvError::InvalidKey {
    }
}

/// Helper function to create system errors
pub fn system_error(operation: &str, message: &str, code: Option<i32>) -> EnvError {
    EnvError::SystemError {
    }
}

/// Helper function to create unicode errors
pub fn unicode_error(key: &str, message: &str) -> EnvError {
    EnvError::UnicodeError {
    }
}

/// Helper function to create expansion errors
pub fn expansion_error(input: &str, message: &str) -> EnvError {
    EnvError::ExpansionError {
    }
}

/// Helper function to create general environment errors
pub fn env_error(message: &str) -> EnvError {
    EnvError::General {
    }
}

/// Validates environment variable key for common issues
pub fn validate_env_key(key: &str) -> EnvResult<()> {
    if key.is_empty() {
        return Err(invalid_key_error(key, "Environment variable key cannot be empty"));
    if key.contains('\0') {
        return Err(invalid_key_error(key, "Environment variable key cannot contain null bytes"));
    // Windows has additional restrictions
    #[cfg(target_os = "windows")]
    {
        if key.contains('=') {
            return Err(invalid_key_error(key, "Environment variable key cannot contain '=' on Windows"));
        }
    }
    
    Ok(())
/// Validates environment variable value for common issues
pub fn validate_env_value(value: &str) -> EnvResult<()> {
    if value.contains('\0') {
        return Err(EnvError::General {
        });
    Ok(())
}
