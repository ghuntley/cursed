/// Core environment variable operations for CURSED standard library

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;

use super::error::{EnvError, EnvResult, not_found_error, validate_env_key, validate_env_value, system_error};

/// Get an environment variable value
/// 
/// Returns `Some(value)` if the variable exists, `None` if it doesn't exist.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_env;
/// 
/// if let Some(path) = get_env("PATH") {
///     println!("PATH is set to: {}", path);
/// }
/// ```
pub fn get_env(key: &str) -> Option<String> {
    validate_env_key(key).ok()?;
    env::var(key).ok()
}

/// Set an environment variable
/// 
/// Sets the environment variable `key` to `value` for the current process.
/// Note: This only affects the current process and its children.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::set_env;
/// 
/// set_env("MY_VAR", "my_value")?;
/// ```
pub fn set_env(key: &str, value: &str) -> EnvResult<()> {
    validate_env_key(key)?;
    validate_env_value(value)?;
    
    env::set_var(key, value);
    Ok(())
}

/// Remove an environment variable
/// 
/// Removes the environment variable `key` from the current process.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::remove_env;
/// 
/// remove_env("TEMP_VAR")?;
/// ```
pub fn remove_env(key: &str) -> EnvResult<()> {
    validate_env_key(key)?;
    
    env::remove_var(key);
    Ok(())
}

/// Get all environment variables
/// 
/// Returns a HashMap containing all environment variables and their values.
/// Invalid Unicode sequences are skipped.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_all_env;
/// 
/// let env_vars = get_all_env();
/// for (key, value) in env_vars {
///     println!("{} = {}", key, value);
/// }
/// ```
pub fn get_all_env() -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    for (key, value) in env::vars() {
        result.insert(key, value);
    }
    
    result
}

/// Check if an environment variable exists
/// 
/// Returns `true` if the environment variable is set, `false` otherwise.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::env_exists;
/// 
/// if env_exists("HOME") {
///     println!("HOME environment variable is set");
/// }
/// ```
pub fn env_exists(key: &str) -> bool {
    get_env(key).is_some()
}

/// Get an environment variable with a default value
/// 
/// Returns the value of the environment variable if it exists, 
/// otherwise returns the provided default value.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_env_with_default;
/// 
/// let editor = get_env_with_default("EDITOR", "nano");
/// println!("Using editor: {}", editor);
/// ```
pub fn get_env_with_default(key: &str, default: &str) -> String {
    get_env(key).unwrap_or_else(|| default.to_string())
}

/// Clear all environment variables
/// 
/// **Warning**: This is a destructive operation that removes all environment variables.
/// Use with extreme caution as this can break program execution.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::clear_all_env;
/// 
/// // Only use in testing scenarios
/// clear_all_env()?;
/// ```
pub fn clear_all_env() -> EnvResult<()> {
    let keys: Vec<String> = env::vars().map(|(key, _)| key).collect();
    
    for key in keys {
        env::remove_var(key);
    }
    
    Ok(())
}

/// Get all environment variable keys
/// 
/// Returns a vector of all environment variable names.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_env_keys;
/// 
/// let keys = get_env_keys();
/// println!("Environment variables: {:?}", keys);
/// ```
pub fn get_env_keys() -> Vec<String> {
    env::vars().map(|(key, _)| key).collect()
}

/// Get all environment variable values
/// 
/// Returns a vector of all environment variable values.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_env_values;
/// 
/// let values = get_env_values();
/// println!("Total values: {}", values.len());
/// ```
pub fn get_env_values() -> Vec<String> {
    env::vars().map(|(_, value)| value).collect()
}

/// Get the current working directory from environment
/// 
/// Returns the current working directory path.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_current_dir;
/// 
/// if let Some(dir) = get_current_dir() {
///     println!("Current directory: {}", dir);
/// }
/// ```
pub fn get_current_dir() -> Option<String> {
    env::current_dir().ok()
        .and_then(|path| path.to_str().map(|s| s.to_string()))
}

/// Get the home directory from environment
/// 
/// Returns the user's home directory path from environment variables.
/// Checks HOME on Unix-like systems, and USERPROFILE on Windows.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_home_dir;
/// 
/// if let Some(home) = get_home_dir() {
///     println!("Home directory: {}", home);
/// }
/// ```
pub fn get_home_dir() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        get_env("USERPROFILE").or_else(|| {
            // Fallback to HOMEDRIVE + HOMEPATH on Windows
            let drive = get_env("HOMEDRIVE")?;
            let path = get_env("HOMEPATH")?;
            Some(format!("{}{}", drive, path))
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        get_env("HOME")
    }
}

/// Get the temporary directory from environment
/// 
/// Returns the system temporary directory path from environment variables.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_temp_dir;
/// 
/// if let Some(temp) = get_temp_dir() {
///     println!("Temp directory: {}", temp);
/// }
/// ```
pub fn get_temp_dir() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        get_env("TEMP").or_else(|| get_env("TMP"))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        get_env("TMPDIR")
            .or_else(|| get_env("TMP"))
            .or_else(|| get_env("TEMP"))
            .or_else(|| Some("/tmp".to_string()))
    }
}

/// Get the path separator for the current platform
/// 
/// Returns ";" on Windows, ":" on Unix-like systems.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_path_separator;
/// 
/// let separator = get_path_separator();
/// println!("Path separator: {}", separator);
/// ```
pub fn get_path_separator() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        ";"
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        ":"
    }
}

/// Check if environment variable names are case-sensitive on this platform
/// 
/// Returns `false` on Windows (case-insensitive), `true` on Unix-like systems.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::is_case_sensitive_env;
/// 
/// if is_case_sensitive_env() {
///     println!("Environment variables are case-sensitive");
/// }
/// ```
pub fn is_case_sensitive_env() -> bool {
    #[cfg(target_os = "windows")]
    {
        false
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}

/// Get environment variable, handling OS-specific case sensitivity
/// 
/// On Windows, performs case-insensitive lookup. On Unix, performs exact match.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_env_case_insensitive;
/// 
/// // Will find "PATH", "Path", or "path" on Windows
/// if let Some(path) = get_env_case_insensitive("path") {
///     println!("PATH found: {}", path);
/// }
/// ```
pub fn get_env_case_insensitive(key: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        // On Windows, try exact match first, then case-insensitive
        if let Some(value) = get_env(key) {
            return Some(value);
        }
        
        let key_upper = key.to_uppercase();
        for (env_key, env_value) in env::vars() {
            if env_key.to_uppercase() == key_upper {
                return Some(env_value);
            }
        }
        None
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        get_env(key)
    }
}

/// Get the username from environment
/// 
/// Returns the current username from environment variables.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_username;
/// 
/// if let Some(user) = get_username() {
///     println!("Current user: {}", user);
/// }
/// ```
pub fn get_username() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        get_env("USERNAME")
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        get_env("USER").or_else(|| get_env("LOGNAME"))
    }
}

/// Get the system hostname from environment
/// 
/// Returns the system hostname from environment variables.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::get_hostname;
/// 
/// if let Some(hostname) = get_hostname() {
///     println!("Hostname: {}", hostname);
/// }
/// ```
pub fn get_hostname() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        get_env("COMPUTERNAME").or_else(|| get_env("HOSTNAME"))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        get_env("HOSTNAME").or_else(|| get_env("HOST"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_env_operations() {
        let test_key = "CURSED_TEST_VAR";
        let test_value = "test_value";
        
        // Set and get
        assert!(set_env(test_key, test_value).is_ok());
        assert_eq!(get_env(test_key), Some(test_value.to_string()));
        assert!(env_exists(test_key));
        
        // Remove
        assert!(remove_env(test_key).is_ok());
        assert_eq!(get_env(test_key), None);
        assert!(!env_exists(test_key));
    }

    #[test]
    fn test_get_env_with_default() {
        let nonexistent_key = "CURSED_NONEXISTENT_VAR_12345";
        let default_value = "default";
        
        assert_eq!(
            get_env_with_default(nonexistent_key, default_value),
            default_value
        );
    }

    #[test]
    fn test_invalid_key_validation() {
        assert!(set_env("", "value").is_err());
        assert!(set_env("key\0with\0nulls", "value").is_err());
    }

    #[test]
    fn test_invalid_value_validation() {
        assert!(set_env("TEST_KEY", "value\0with\0nulls").is_err());
    }

    #[test]
    fn test_path_separator() {
        let separator = get_path_separator();
        #[cfg(target_os = "windows")]
        assert_eq!(separator, ";");
        
        #[cfg(not(target_os = "windows"))]
        assert_eq!(separator, ":");
    }

    #[test]
    fn test_case_sensitivity() {
        #[cfg(target_os = "windows")]
        assert!(!is_case_sensitive_env());
        
        #[cfg(not(target_os = "windows"))]
        assert!(is_case_sensitive_env());
    }

    #[test]
    fn test_get_all_env() {
        let env_vars = get_all_env();
        assert!(!env_vars.is_empty());
    }

    #[test]
    fn test_get_env_keys_and_values() {
        let keys = get_env_keys();
        let values = get_env_values();
        assert_eq!(keys.len(), values.len());
        assert!(!keys.is_empty());
    }
}
