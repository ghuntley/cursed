use crate::error::CursedError;
/// Environment and system configuration management
/// 
/// This module provides environment and system configuration capabilities including:
/// - Environment variable management
/// - System path manipulation
/// - Registry access (Windows)
/// - Configuration file handling
/// - System-wide settings management

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
// use crate::stdlib::system::info::SystemResult;

/// Environment variable manager
#[derive(Debug, Clone)]
pub struct EnvironmentManager {
/// System path information
#[derive(Debug, Clone)]
pub struct SystemPath {
/// Registry access (Windows-specific)
#[derive(Debug, Clone)]
pub struct Registry {
    #[cfg(target_os = "windows")]
impl Default for EnvironmentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentManager {
    /// Create a new environment manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Get an environment variable
    pub fn get(&mut self, key: &str) -> Option<String> {
        // Check if we have a cached value
        if let Some(value) = self.cached_vars.get(key) {
            return Some(value.clone());
        // Check if the variable was modified
        if let Some(modified) = self.modified_vars.get(key) {
            return modified.clone();
        // Get from system environment
        match env::var(key) {
            Ok(value) => {
                self.cached_vars.insert(key.to_string(), value.clone());
                Some(value)
            }
        }
    }

    /// Set an environment variable
    pub fn set(&mut self, key: &str, value: &str) -> SystemResult<()> {
        env::set_var(key, value);
        self.modified_vars.insert(key.to_string(), Some(value.to_string()));
        self.cached_vars.insert(key.to_string(), value.to_string());
        Ok(())
    /// Remove an environment variable
    pub fn remove(&mut self, key: &str) -> SystemResult<()> {
        env::remove_var(key);
        self.modified_vars.insert(key.to_string(), None);
        self.cached_vars.remove(key);
        Ok(())
    /// Get all environment variables
    pub fn get_all(&mut self) -> HashMap<String, String> {
        let mut all_vars = HashMap::new();

        // Start with system environment
        for (key, value) in env::vars() {
            all_vars.insert(key, value);
        // Apply modifications
        for (key, value) in &self.modified_vars {
            if let Some(val) = value {
                all_vars.insert(key.clone(), val.clone());
            } else {
                all_vars.remove(key);
            }
        }

        all_vars
    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cached_vars.clear();
    }
}

impl Default for SystemPath {
    fn default() -> Self {
        Self::get_system_paths()
    }
}

impl SystemPath {
    /// Get system paths for the current platform
    pub fn get_system_paths() -> Self {
        #[cfg(target_os = "windows")]
        return Self::get_windows_paths();
        
        #[cfg(unix)]
        return Self::get_unix_paths();
        
        #[cfg(not(any(windows, unix)))]
        Self::get_default_paths()
    #[cfg(target_os = "windows")]
    fn get_windows_paths() -> Self {
        Self {
        }
    }

    #[cfg(unix)]
    fn get_unix_paths() -> Self {
        let home = env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/"));
        
        Self {
            temp: env::var("TMPDIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/tmp")),
            system: PathBuf::from("/usr/bin"),
            program_files: PathBuf::from("/usr/local"),
            user_data: env::var("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|_| home.join(".local/share")),
        }
    }

    #[cfg(not(any(windows, unix)))]
    fn get_default_paths() -> Self {
        Self {
            home: PathBuf::from("/"),
            temp: PathBuf::from("/tmp"),
            system: PathBuf::from("/system"),
            program_files: PathBuf::from("/programs"),
            user_data: PathBuf::from("/data"),
            user_config: PathBuf::from("/config"),
            user_cache: PathBuf::from("/cache"),
        }
    }
impl Registry {
    /// Create a new registry accessor
    pub fn new(path: &str) -> Self {
        Self {
            #[cfg(target_os = "windows")]
        }
    }

    /// Read a registry value
    #[cfg(target_os = "windows")]
    pub fn read_string(&self, key: &str) -> SystemResult<String> {
        // Windows registry access would go here
        // For now, return a placeholder
        Ok("registry_value".to_string())
    /// Write a registry value
    #[cfg(target_os = "windows")]
    pub fn write_string(&self, key: &str, value: &str) -> SystemResult<()> {
        // Windows registry write would go here
        Ok(())
    /// Delete a registry value
    #[cfg(target_os = "windows")]
    pub fn delete_value(&self, key: &str) -> SystemResult<()> {
        // Windows registry delete would go here
        Ok(())
    /// Non-Windows platforms don't have registry
    #[cfg(not(target_os = "windows"))]
    pub fn read_string(&self, _key: &str) -> SystemResult<String> {
//         Err(crate::stdlib::system::info::SystemError::UnsupportedOperation("Registry access not available on this platform".to_string()))
    #[cfg(not(target_os = "windows"))]
    pub fn write_string(&self, _key: &str, _value: &str) -> SystemResult<()> {
//         Err(crate::stdlib::system::info::SystemError::UnsupportedOperation("Registry access not available on this platform".to_string()))
    #[cfg(not(target_os = "windows"))]
    pub fn delete_value(&self, _key: &str) -> SystemResult<()> {
//         Err(crate::stdlib::system::info::SystemError::UnsupportedOperation("Registry access not available on this platform".to_string()))
    }
}

/// Get an environment variable
pub fn get_environment_variable(key: &str) -> Option<String> {
    env::var(key).ok()
/// Set an environment variable
pub fn set_environment_variable(key: &str, value: &str) -> SystemResult<()> {
    env::set_var(key, value);
    Ok(())
/// Remove an environment variable
pub fn remove_environment_variable(key: &str) -> SystemResult<()> {
    env::remove_var(key);
    Ok(())
/// Get all environment variables
pub fn get_all_environment_variables() -> HashMap<String, String> {
    env::vars().collect()
/// Get system paths
pub fn get_system_paths() -> SystemPath {
    SystemPath::get_system_paths()
/// Get the PATH environment variable as a vector of paths
pub fn get_path_variable() -> Vec<PathBuf> {
    env::var("PATH")
        .unwrap_or_default()
        .split(if cfg!(windows) { ';' } else { ':' })
        .map(PathBuf::from)
        .collect()
/// Add a path to the PATH environment variable
pub fn add_to_path(path: &str) -> SystemResult<()> {
    let current_path = env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ";" } else { ":" };
    let new_path = if current_path.is_empty() {
        path.to_string()
    } else {
        format!("{}{}{}", current_path, separator, path)
    env::set_var("PATH", new_path);
    Ok(())
/// Remove a path from the PATH environment variable
pub fn remove_from_path(path: &str) -> SystemResult<()> {
    let current_path = env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ";" } else { ":" };
    let paths: Vec<&str> = current_path.split(separator).collect();
    let filtered_paths: Vec<&str> = paths.into_iter().filter(|&p| p != path).collect();
    let new_path = filtered_paths.join(separator);
    env::set_var("PATH", new_path);
    Ok(())
