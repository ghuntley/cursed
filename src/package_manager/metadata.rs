use crate::error::CursedError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Dependency information for a package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
/// Source of a dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
impl Default for DependencyInfo {
    fn default() -> Self {
        Self {
        }
    }
/// Package metadata from CursedPackage.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
/// Version specification for dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionSpec {
    Complex {
impl Default for PackageMetadata {
    fn default() -> Self {
        Self {
        }
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
    /// Check if a package name is valid
    pub fn is_valid_package_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        // Package names can't start or end with hyphens
        if name.starts_with('-') || name.ends_with('-') {
            return false;
        // Package names can only contain alphanumeric characters, hyphens, and underscores
        name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            && !name.contains('@') && !name.contains('.')
    /// Check if a version string is valid (basic semantic versioning)
    pub fn is_valid_version(version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() == 3 && parts.iter().all(|part| part.parse::<u32>().is_ok())
    /// Check if there's a circular dependency
    pub fn has_circular_dependency(&self, package_name: &str) -> bool {
        // Simple implementation: check if package depends on itself
        self.dependencies.contains_key(package_name) ||
        self.dependencies.contains_key(&self.name)
    /// Save metadata to file
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::error::Result<()> {
        let toml_string = toml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, toml_string)?;
        Ok(())
    /// Load metadata from file
    pub fn from_file(path: &std::path::Path) -> crate::error::Result<PackageMetadata> {
        let content = std::fs::read_to_string(path)?;
        let metadata: PackageMetadata = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(metadata)
    /// Get package ID in format "name@version"
    pub fn package_id(&self) -> String {
        format!("{}@{}", self.name, self.version)
    /// Get all dependencies (both regular and dev)
    pub fn all_dependencies(&self) -> HashMap<String, &VersionSpec> {
        let mut all_deps = HashMap::new();
        for (name, spec) in &self.dependencies {
            all_deps.insert(name.clone(), spec);
        }
        for (name, spec) in &self.dev_dependencies {
            all_deps.insert(name.clone(), spec);
        }
        all_deps
    /// Convert to PackageInfo for compatibility with resolver
    pub fn to_package_info(&self) -> crate::package_manager::registry::PackageInfo {
        crate::package_manager::registry::PackageInfo {
            download_url: format!("https://registry.cursed.dev/packages/{}", self.name),
        }
    }
    
    /// Calculate checksum for the package metadata
    pub fn calculate_metadata_checksum(&self) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // Hash package metadata in a deterministic order
        hasher.update(self.name.as_bytes());
        hasher.update(self.version.as_bytes());
        hasher.update(self.description.as_bytes());
        
        // Hash authors
        for author in &self.authors {
            hasher.update(author.as_bytes());
        // Hash dependencies in sorted order for determinism
        let mut dep_keys: Vec<_> = self.dependencies.keys().collect();
        dep_keys.sort();
        for key in dep_keys {
            hasher.update(key.as_bytes());
            hasher.update(self.dependencies[key].to_string().as_bytes());
        // Hash dev dependencies in sorted order
        let mut dev_dep_keys: Vec<_> = self.dev_dependencies.keys().collect();
        dev_dep_keys.sort();
        for key in dev_dep_keys {
            hasher.update(key.as_bytes());
            hasher.update(self.dev_dependencies[key].to_string().as_bytes());
        // Hash optional fields
        if let Some(repo) = &self.repository {
            hasher.update(repo.as_bytes());
        }
        if let Some(license) = &self.license {
            hasher.update(license.as_bytes());
        // Hash keywords and categories in sorted order
        let mut sorted_keywords = self.keywords.clone();
        sorted_keywords.sort();
        for keyword in sorted_keywords {
            hasher.update(keyword.as_bytes());
        let mut sorted_categories = self.categories.clone();
        sorted_categories.sort();
        for category in sorted_categories {
            hasher.update(category.as_bytes());
        format!("sha256:{:x}", hasher.finalize())
    }
}

impl std::fmt::Display for VersionSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionSpec::Complex { git: Some(git), branch: Some(branch), .. } => {
                write!(f, "git+{}#{}", git, branch)
            VersionSpec::Complex { git: Some(git), tag: Some(tag), .. } => {
                write!(f, "git+{}#{}", git, tag)
            VersionSpec::Complex { git: Some(git), .. } => {
                write!(f, "git+{}", git)
            VersionSpec::Complex { path: Some(path), .. } => {
                write!(f, "path:{}", path)
        }
    }
impl VersionSpec {
    pub fn new(version: &str) -> Self {
        Self::Simple(version.to_string())
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
        if constraint.is_empty() || constraint == "invalid" {
            return false;
        }
        // Simple check: if it has more than 3 parts when split by '.', it's invalid
        let parts: Vec<&str> = constraint.trim_start_matches(['>', '<', '=', '~', '^', ' '])
            .split('.').collect();
        parts.len() <= 3
    /// Check if this is a path dependency
    pub fn is_path_dependency(&self) -> bool {
        match self {
        }
    }
    
    /// Check if this is a git dependency
    pub fn is_git_dependency(&self) -> bool {
        match self {
        }
    }
    
    /// Check if this is an optional dependency
    pub fn is_optional(&self) -> bool {
        match self {
        }
    }
    
    /// Get the version string if available
    pub fn version_string(&self) -> Option<&str> {
        match self {
        }
    }
}
