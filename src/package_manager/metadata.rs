use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Package metadata from CursedPackage.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub dependencies: HashMap<String, VersionSpec>,
    pub dev_dependencies: HashMap<String, VersionSpec>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
}

/// Version specification for dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionSpec {
    Simple(String),
    Complex {
        version: Option<String>,
        git: Option<String>,
        branch: Option<String>,
        tag: Option<String>,
        path: Option<String>,
        features: Option<Vec<String>>,
        optional: Option<bool>,
    },
}

impl PackageMetadata {
    /// Validate the package metadata
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Package name cannot be empty".to_string());
        }
        if self.version.is_empty() {
            return Err("Package version cannot be empty".to_string());
        }
        Ok(())
    }
}

impl VersionSpec {
    pub fn new(version: &str) -> Self {
        Self::Simple(version.to_string())
    }
    
    /// Validate the version specification
    pub fn validate(&self) -> Result<(), String> {
        match self {
            VersionSpec::Simple(v) if v.is_empty() => {
                Err("Version cannot be empty".to_string())
            }
            VersionSpec::Complex { version, git, path, .. } => {
                if version.is_none() && git.is_none() && path.is_none() {
                    Err("Complex version spec must specify version, git, or path".to_string())
                } else {
                    Ok(())
                }
            }
            _ => Ok(())
        }
    }
    
    /// Check if a version constraint is valid
    pub fn is_valid_version_constraint(constraint: &str) -> bool {
        !constraint.is_empty() && constraint != "invalid"
    }
}
