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
pub struct VersionSpec {
    pub version: String,
    pub features: Vec<String>,
    pub optional: bool,
}

impl VersionSpec {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            features: vec![],
            optional: false,
        }
    }
}
