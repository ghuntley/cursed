/// Environment variable management for processes
/// 
/// This module provides comprehensive environment variable handling for process
/// management, including inheritance, modification, and validation.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::env;
use std::path::PathBuf;

use crate::stdlib::process::error::{ProcessError, ProcessResult, environment_error, invalid_arguments};

/// Environment variable representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvVar {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: Option<String>,
}

impl EnvVar {
    /// Create a new environment variable
    pub fn new<N: Into<String>, V: Into<String>>(name: N, value: V) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
        }
    }
    
    /// Create an environment variable to be removed (unset)
    pub fn unset<N: Into<String>>(name: N) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }
    
    /// Check if this variable should be unset
    pub fn is_unset(&self) -> bool {
        self.value.is_none()
    }
}

/// Environment variable manager for processes
#[derive(Debug, Clone)]
pub struct EnvironmentManager {
    /// Base environment variables (inherited or custom)
    base_env: HashMap<OsString, OsString>,
    /// Additional environment variables to set
    additional_env: HashMap<OsString, OsString>,
    /// Environment variables to remove
    removed_env: Vec<OsString>,
    /// Whether to clear all inherited environment variables
    clear_inherited: bool,
    /// Case sensitivity setting (Windows vs Unix)
    case_sensitive: bool,
}

impl EnvironmentManager {
    /// Create a new environment manager
    pub fn new() -> Self {
        Self {
            base_env: HashMap::new(),
            additional_env: HashMap::new(),
            removed_env: Vec::new(),
            clear_inherited: false,
            case_sensitive: cfg!(not(windows)),
        }
    }

    /// Create environment manager inheriting current process environment
    pub fn inherit_current() -> ProcessResult<Self> {
        let mut manager = Self::new();
        for (key, value) in env::vars_os() {
            manager.base_env.insert(key, value);
        }
        Ok(manager)
    }

    /// Clear all inherited environment variables
    pub fn clear_inherited(&mut self) -> &mut Self {
        self.clear_inherited = true;
        self.base_env.clear();
        self
    }

    /// Set an environment variable
    pub fn set<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let key = key.as_ref().to_os_string();
        let value = value.as_ref().to_os_string();
        
        // Remove from removed list if present
        self.removed_env.retain(|k| k != &key);
        
        // Add to additional environment
        self.additional_env.insert(key, value);
        self
    }

    /// Remove an environment variable
    pub fn remove<K>(&mut self, key: K) -> &mut Self
    where
        K: AsRef<OsStr>,
    {
        let key = key.as_ref().to_os_string();
        
        // Remove from additional environment if present
        self.additional_env.remove(&key);
        
        // Add to removed list
        if !self.removed_env.contains(&key) {
            self.removed_env.push(key);
        }
        
        self
    }

    /// Set multiple environment variables from a map
    pub fn set_multiple<K, V>(&mut self, vars: &HashMap<K, V>) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        for (key, value) in vars {
            self.set(key, value);
        }
        self
    }

    /// Get the value of an environment variable
    pub fn get<K>(&self, key: K) -> Option<&OsStr>
    where
        K: AsRef<OsStr>,
    {
        let key = key.as_ref();
        
        // Check if it's in the removed list
        if self.removed_env.iter().any(|k| k == key) {
            return None;
        }
        
        // Check additional environment first
        if let Some(value) = self.additional_env.get(key) {
            return Some(value);
        }
        
        // Check base environment
        self.base_env.get(key).map(|v| v.as_ref())
    }

    /// Check if an environment variable exists
    pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsRef<OsStr>,
    {
        self.get(key).is_some()
    }

    /// Get all environment variables as a HashMap
    pub fn get_all(&self) -> HashMap<OsString, OsString> {
        let mut result = HashMap::new();
        
        // Start with base environment (unless cleared)
        if !self.clear_inherited {
            result.extend(self.base_env.clone());
        }
        
        // Add additional variables
        result.extend(self.additional_env.clone());
        
        // Remove variables in the removed list
        for key in &self.removed_env {
            result.remove(key);
        }
        
        result
    }

    /// Get environment variables as Vec of (key, value) tuples for process spawning
    pub fn to_command_env(&self) -> Vec<(OsString, OsString)> {
        self.get_all().into_iter().collect()
    }

    /// Validate environment variable name
    pub fn validate_key<K>(key: K) -> ProcessResult<()>
    where
        K: AsRef<OsStr>,
    {
        let key_str = key.as_ref();
        
        // Check for empty key
        if key_str.is_empty() {
            return Err(invalid_arguments("Environment variable name cannot be empty"));
        }
        
        // Convert to string for validation (if possible)
        if let Some(key_string) = key_str.to_str() {
            // Check for invalid characters
            if key_string.contains('=') {
                return Err(invalid_arguments("Environment variable name cannot contain '=' character"));
            }
            
            if key_string.contains('\0') {
                return Err(invalid_arguments("Environment variable name cannot contain null character"));
            }
            
            // Check for Windows reserved names
            #[cfg(windows)]
            {
                let uppercase_key = key_string.to_uppercase();
                if matches!(uppercase_key.as_str(), "CON" | "PRN" | "AUX" | "NUL" |
                    "COM1" | "COM2" | "COM3" | "COM4" | "COM5" | "COM6" | "COM7" | "COM8" | "COM9" |
                    "LPT1" | "LPT2" | "LPT3" | "LPT4" | "LPT5" | "LPT6" | "LPT7" | "LPT8" | "LPT9") {
                    return Err(invalid_arguments(&format!("'{}' is a reserved name on Windows", key_string)));
                }
            }
        }
        
        Ok(())
    }

    /// Set the PATH environment variable
    pub fn set_path<P>(&mut self, paths: &[P]) -> ProcessResult<&mut Self>
    where
        P: AsRef<std::path::Path>,
    {
        let path_separator = if cfg!(windows) { ";" } else { ":" };
        let path_value = paths
            .iter()
            .map(|p| p.as_ref().to_string_lossy())
            .collect::<Vec<_>>()
            .join(path_separator);
        
        self.set("PATH", path_value);
        Ok(self)
    }

    /// Add a directory to the PATH environment variable
    pub fn add_to_path<P>(&mut self, path: P) -> ProcessResult<&mut Self>
    where
        P: AsRef<std::path::Path>,
    {
        let path_str = path.as_ref().to_string_lossy();
        let path_separator = if cfg!(windows) { ";" } else { ":" };
        
        // Get current PATH or empty string
        let current_path = self.get("PATH")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let new_path = if current_path.is_empty() {
            path_str.to_string()
        } else {
            format!("{}{}{}", path_str, path_separator, current_path)
        };
        
        self.set("PATH", new_path);
        Ok(self)
    }

    /// Set the working directory environment variable
    pub fn set_working_dir<P>(&mut self, dir: P) -> ProcessResult<&mut Self>
    where
        P: AsRef<std::path::Path>,
    {
        let dir_str = dir.as_ref().to_string_lossy();
        self.set("PWD", dir_str.as_ref());
        Ok(self)
    }

    /// Set common development environment variables
    pub fn set_development_env(&mut self) -> &mut Self {
        self.set("RUST_BACKTRACE", "1")
            .set("RUST_LOG", "debug")
    }

    /// Set production environment variables
    pub fn set_production_env(&mut self) -> &mut Self {
        self.remove("RUST_BACKTRACE")
            .set("RUST_LOG", "warn")
    }

    /// Clone environment for child process modification
    pub fn clone_for_child(&self) -> Self {
        Self {
            base_env: self.get_all(),
            additional_env: HashMap::new(),
            removed_env: Vec::new(),
            clear_inherited: false,
            case_sensitive: self.case_sensitive,
        }
    }

    /// Get environment variable count
    pub fn len(&self) -> usize {
        self.get_all().len()
    }

    /// Check if environment is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get environment variables matching a prefix
    pub fn get_with_prefix<K>(&self, prefix: K) -> HashMap<OsString, OsString>
    where
        K: AsRef<OsStr>,
    {
        let prefix_str = prefix.as_ref().to_string_lossy();
        self.get_all()
            .into_iter()
            .filter(|(key, _)| {
                key.to_string_lossy().starts_with(prefix_str.as_ref())
            })
            .collect()
    }

    /// Merge with another environment manager
    pub fn merge(&mut self, other: &EnvironmentManager) -> &mut Self {
        // Add all variables from other
        for (key, value) in &other.additional_env {
            self.additional_env.insert(key.clone(), value.clone());
        }
        
        // Add removed variables from other
        for key in &other.removed_env {
            if !self.removed_env.contains(key) {
                self.removed_env.push(key.clone());
            }
        }
        
        self
    }
}

impl Default for EnvironmentManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Environment variable utilities
pub struct EnvironmentUtils;

impl EnvironmentUtils {
    /// Get the system PATH separator
    pub fn path_separator() -> &'static str {
        if cfg!(windows) { ";" } else { ":" }
    }

    /// Parse PATH environment variable into individual paths
    pub fn parse_path(path_value: &str) -> Vec<PathBuf> {
        let separator = Self::path_separator();
        path_value
            .split(separator)
            .map(|p| PathBuf::from(p.trim()))
            .filter(|p| !p.as_os_str().is_empty())
            .collect()
    }

    /// Find executable in PATH
    pub fn find_in_path(executable: &str) -> Option<PathBuf> {
        if let Ok(path_value) = env::var("PATH") {
            let paths = Self::parse_path(&path_value);
            let executable_name = if cfg!(windows) && !executable.ends_with(".exe") {
                format!("{}.exe", executable)
            } else {
                executable.to_string()
            };
            
            for path in paths {
                let full_path = path.join(&executable_name);
                if full_path.is_file() {
                    return Some(full_path);
                }
            }
        }
        None
    }

    /// Expand environment variables in a string
    pub fn expand_variables(input: &str) -> ProcessResult<String> {
        let mut result = input.to_string();
        
        // Handle different variable formats: $VAR, ${VAR}, %VAR% (Windows)
        #[cfg(windows)]
        {
            // Windows %VAR% format
            while let Some(start) = result.find('%') {
                if let Some(end) = result[start + 1..].find('%') {
                    let var_name = &result[start + 1..start + 1 + end];
                    let replacement = env::var(var_name).unwrap_or_default();
                    result.replace_range(start..start + end + 2, &replacement);
                } else {
                    break;
                }
            }
        }
        
        // Unix $VAR and ${VAR} format
        let mut chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == '$' && i + 1 < chars.len() {
                let start = i;
                i += 1;
                
                let (var_name, end) = if chars[i] == '{' {
                    // ${VAR} format
                    i += 1;
                    let var_start = i;
                    while i < chars.len() && chars[i] != '}' {
                        i += 1;
                    }
                    if i >= chars.len() {
                        return Err(environment_error("Unclosed variable reference"));
                    }
                    let var_name: String = chars[var_start..i].iter().collect();
                    i += 1; // Skip closing brace
                    (var_name, i)
                } else {
                    // $VAR format
                    let var_start = i;
                    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                        i += 1;
                    }
                    let var_name: String = chars[var_start..i].iter().collect();
                    (var_name, i)
                };
                
                // Replace variable
                let replacement = env::var(&var_name).unwrap_or_default();
                let replacement_chars: Vec<char> = replacement.chars().collect();
                
                // Replace the variable reference with its value
                chars.splice(start..end, replacement_chars);
                i = start + replacement.len();
            } else {
                i += 1;
            }
        }
        
        Ok(chars.iter().collect())
    }

    /// Validate environment variable value
    pub fn validate_value<V>(value: V) -> ProcessResult<()>
    where
        V: AsRef<OsStr>,
    {
        let value_str = value.as_ref();
        
        // Check for null character
        if let Some(value_string) = value_str.to_str() {
            if value_string.contains('\0') {
                return Err(invalid_arguments("Environment variable value cannot contain null character"));
            }
        }
        
        Ok(())
    }

    /// Get environment variable as specific type
    pub fn get_typed<T>(key: &str) -> ProcessResult<Option<T>>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        match env::var(key) {
            Ok(value) => {
                match value.parse::<T>() {
                    Ok(parsed) => Ok(Some(parsed)),
                    Err(e) => Err(environment_error(&format!(
                        "Failed to parse environment variable '{}': {}", key, e
                    ))),
                }
            }
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(_)) => {
                Err(environment_error(&format!(
                    "Environment variable '{}' contains invalid Unicode", key
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_environment_manager_basic_operations() {
        let mut env = EnvironmentManager::new();
        
        // Test setting and getting
        env.set("TEST_VAR", "test_value");
        assert_eq!(env.get("TEST_VAR").unwrap().to_string_lossy(), "test_value");
        
        // Test contains
        assert!(env.contains_key("TEST_VAR"));
        assert!(!env.contains_key("NONEXISTENT"));
        
        // Test removal
        env.remove("TEST_VAR");
        assert!(!env.contains_key("TEST_VAR"));
    }

    #[test]
    fn test_environment_manager_multiple_vars() {
        let mut env = EnvironmentManager::new();
        let mut vars = HashMap::new();
        vars.insert("VAR1", "value1");
        vars.insert("VAR2", "value2");
        
        env.set_multiple(&vars);
        
        assert_eq!(env.get("VAR1").unwrap().to_string_lossy(), "value1");
        assert_eq!(env.get("VAR2").unwrap().to_string_lossy(), "value2");
        assert_eq!(env.len(), 2);
    }

    #[test]
    fn test_environment_manager_path_operations() {
        let mut env = EnvironmentManager::new();
        
        // Test setting PATH
        let paths = vec!["/usr/bin", "/bin"];
        env.set_path(&paths).unwrap();
        
        let path_value = env.get("PATH").unwrap().to_string_lossy();
        let expected = if cfg!(windows) { "/usr/bin;/bin" } else { "/usr/bin:/bin" };
        assert_eq!(path_value, expected);
        
        // Test adding to PATH
        env.add_to_path("/usr/local/bin").unwrap();
        let path_value = env.get("PATH").unwrap().to_string_lossy();
        let expected = if cfg!(windows) { 
            "/usr/local/bin;/usr/bin;/bin" 
        } else { 
            "/usr/local/bin:/usr/bin:/bin" 
        };
        assert_eq!(path_value, expected);
    }

    #[test]
    fn test_environment_validation() {
        // Test valid key
        assert!(EnvironmentManager::validate_key("VALID_KEY").is_ok());
        
        // Test invalid keys
        assert!(EnvironmentManager::validate_key("").is_err());
        assert!(EnvironmentManager::validate_key("KEY=VALUE").is_err());
        assert!(EnvironmentManager::validate_key("KEY\0NULL").is_err());
    }

    #[test]
    fn test_environment_inheritance() {
        // This test requires a real environment variable to be set
        std::env::set_var("TEST_INHERIT", "inherited_value");
        
        let env = EnvironmentManager::inherit_current().unwrap();
        assert!(env.contains_key("TEST_INHERIT"));
        
        std::env::remove_var("TEST_INHERIT");
    }

    #[test]
    fn test_environment_utils_path_parsing() {
        let path_separator = EnvironmentUtils::path_separator();
        let path_value = format!("/usr/bin{}/bin{}/usr/local/bin", path_separator, path_separator);
        
        let paths = EnvironmentUtils::parse_path(&path_value);
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], PathBuf::from("/usr/bin"));
        assert_eq!(paths[1], PathBuf::from("/bin"));
        assert_eq!(paths[2], PathBuf::from("/usr/local/bin"));
    }

    #[test]
    fn test_environment_variable_expansion() {
        std::env::set_var("TEST_EXPAND", "expanded");
        
        // Test Unix format
        let result = EnvironmentUtils::expand_variables("$TEST_EXPAND").unwrap();
        assert_eq!(result, "expanded");
        
        let result = EnvironmentUtils::expand_variables("${TEST_EXPAND}").unwrap();
        assert_eq!(result, "expanded");
        
        let result = EnvironmentUtils::expand_variables("prefix_$TEST_EXPAND_suffix").unwrap();
        assert_eq!(result, "prefix_expanded_suffix");
        
        std::env::remove_var("TEST_EXPAND");
    }

    #[test]
    fn test_environment_merge() {
        let mut env1 = EnvironmentManager::new();
        env1.set("VAR1", "value1");
        env1.set("COMMON", "env1_value");
        
        let mut env2 = EnvironmentManager::new();
        env2.set("VAR2", "value2");
        env2.set("COMMON", "env2_value");
        env2.remove("VAR1");
        
        env1.merge(&env2);
        
        assert_eq!(env1.get("VAR2").unwrap().to_string_lossy(), "value2");
        assert_eq!(env1.get("COMMON").unwrap().to_string_lossy(), "env2_value");
        assert!(!env1.contains_key("VAR1")); // Should be removed
    }

    #[test]
    fn test_environment_prefix_filtering() {
        let mut env = EnvironmentManager::new();
        env.set("PREFIX_VAR1", "value1");
        env.set("PREFIX_VAR2", "value2");
        env.set("OTHER_VAR", "other");
        
        let prefixed = env.get_with_prefix("PREFIX_");
        assert_eq!(prefixed.len(), 2);
        assert!(prefixed.contains_key(&OsString::from("PREFIX_VAR1")));
        assert!(prefixed.contains_key(&OsString::from("PREFIX_VAR2")));
        assert!(!prefixed.contains_key(&OsString::from("OTHER_VAR")));
    }
}
