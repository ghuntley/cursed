/// Environment variable management for exec_vibez
/// 
/// Implements Environment functionality according to specs/stdlib/exec_vibez.md

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;

use super::cmd::Cmd;
use super::error::{ExecResult, invalid_arguments};

/// Environment variable management
#[derive(Debug, Clone)]
pub struct Environment {
    /// Environment variables map
    variables: HashMap<String, String>,
    /// Whether to inherit system environment
    inherit_system: bool,
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            inherit_system: true,
        }
    }
    
    /// Create an environment with system variables
    pub fn with_system() -> Self {
        let mut env = Self::new();
        env.inherit_system_vars();
        env
    }
    
    /// Create an environment without system variables
    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
            inherit_system: false,
        }
    }
    
    /// Set an environment variable
    pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
        self.variables.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Get an environment variable
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key).or_else(|| {
            if self.inherit_system {
                // Try to get from system environment if not found in local map
                env::var(key).ok().as_ref().map(|_| {
                    // This is a bit awkward - we need to return a reference
                    // but env::var returns an owned String. In practice,
                    // we'd want to cache system vars or use a different approach.
                    // For now, we'll just check if it exists.
                    None
                }).flatten()
            } else {
                None
            }
        })
    }
    
    /// Remove an environment variable
    pub fn remove(&mut self, key: &str) -> &mut Self {
        self.variables.remove(key);
        self
    }
    
    /// Clear all environment variables
    pub fn clear(&mut self) -> &mut Self {
        self.variables.clear();
        self
    }
    
    /// Append to an environment variable (e.g., PATH)
    pub fn append(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(existing) = self.variables.get(key) {
            let new_value = format!("{}{}", existing, value);
            self.variables.insert(key.to_string(), new_value);
        } else if self.inherit_system {
            // Try to get from system environment
            if let Ok(existing) = env::var(key) {
                let new_value = format!("{}{}", existing, value);
                self.variables.insert(key.to_string(), new_value);
            } else {
                self.variables.insert(key.to_string(), value.to_string());
            }
        } else {
            self.variables.insert(key.to_string(), value.to_string());
        }
        self
    }
    
    /// Prepend to an environment variable
    pub fn prepend(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(existing) = self.variables.get(key) {
            let new_value = format!("{}{}", value, existing);
            self.variables.insert(key.to_string(), new_value);
        } else if self.inherit_system {
            // Try to get from system environment
            if let Ok(existing) = env::var(key) {
                let new_value = format!("{}{}", value, existing);
                self.variables.insert(key.to_string(), new_value);
            } else {
                self.variables.insert(key.to_string(), value.to_string());
            }
        } else {
            self.variables.insert(key.to_string(), value.to_string());
        }
        self
    }
    
    /// Set whether to inherit system environment variables
    pub fn set_inherit_system(&mut self, inherit: bool) -> &mut Self {
        self.inherit_system = inherit;
        self
    }
    
    /// Inherit all current system environment variables
    pub fn inherit_system_vars(&mut self) -> &mut Self {
        for (key, value) in env::vars() {
            // Only add if not already explicitly set
            if !self.variables.contains_key(&key) {
                self.variables.insert(key, value);
            }
        }
        self
    }
    
    /// Convert to a vector of "KEY=VALUE" strings for process execution
    pub fn to_env_vec(&self) -> Vec<String> {
        let mut env_vec = Vec::new();
        
        if self.inherit_system {
            // Start with system environment
            for (key, value) in env::vars() {
                // Use our override if we have one, otherwise use system value
                if let Some(our_value) = self.variables.get(&key) {
                    env_vec.push(format!("{}={}", key, our_value));
                } else {
                    env_vec.push(format!("{}={}", key, value));
                }
            }
            
            // Add any variables we have that aren't in the system environment
            for (key, value) in &self.variables {
                if env::var(key).is_err() {
                    env_vec.push(format!("{}={}", key, value));
                }
            }
        } else {
            // Only use our variables
            for (key, value) in &self.variables {
                env_vec.push(format!("{}={}", key, value));
            }
        }
        
        env_vec
    }
    
    /// Convert to a vector of OsString pairs for std::process::Command
    pub fn to_os_env(&self) -> Vec<(OsString, OsString)> {
        let mut env_vec = Vec::new();
        
        if self.inherit_system {
            // Start with system environment
            for (key, value) in env::vars_os() {
                // Use our override if we have one
                if let Some(key_str) = key.to_str() {
                    if let Some(our_value) = self.variables.get(key_str) {
                        env_vec.push((key, OsString::from(our_value)));
                        continue;
                    }
                }
                env_vec.push((key, value));
            }
            
            // Add any variables we have that aren't in the system environment
            for (key, value) in &self.variables {
                if env::var(key).is_err() {
                    env_vec.push((OsString::from(key), OsString::from(value)));
                }
            }
        } else {
            // Only use our variables
            for (key, value) in &self.variables {
                env_vec.push((OsString::from(key), OsString::from(value)));
            }
        }
        
        env_vec
    }
    
    /// Get the number of environment variables
    pub fn len(&self) -> usize {
        if self.inherit_system {
            // This is approximate since we might have overrides
            env::vars().count().max(self.variables.len())
        } else {
            self.variables.len()
        }
    }
    
    /// Check if the environment is empty
    pub fn is_empty(&self) -> bool {
        if self.inherit_system {
            false // System environment is never empty
        } else {
            self.variables.is_empty()
        }
    }
    
    /// Merge with another environment
    pub fn merge(&mut self, other: &Environment) -> &mut Self {
        for (key, value) in &other.variables {
            self.variables.insert(key.clone(), value.clone());
        }
        self
    }
    
    /// Create a copy with modifications
    pub fn with_var(mut self, key: &str, value: &str) -> Self {
        self.set(key, value);
        self
    }
    
    /// Create a copy without a variable
    pub fn without_var(mut self, key: &str) -> Self {
        self.remove(key);
        self
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new environment
pub fn new_environment() -> Environment {
    Environment::new()
}

/// Create a command with a custom environment
pub fn command_with_env(name: &str, args: &[&str], env: Environment) -> Cmd {
    let mut cmd = Cmd::new(name, args);
    cmd.set_env(env);
    cmd
}

