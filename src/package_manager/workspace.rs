/// Workspace management for multi-package CURSED projects
/// 
/// Handles workspace discovery, configuration, and operations across
/// multiple related packages within a single project structure.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use glob::glob;

use crate::package_manager::{PackageManagerError, PackageMetadata};
use super::lockfile::{LockFile, LockFileManager};

/// Workspace configuration in CursedPackage.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Member packages (supports glob patterns)
    pub members: Vec<String>,
    
    /// Packages to exclude from workspace
    #[serde(default)]
    pub exclude: Vec<String>,
    
    /// Workspace-level dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    
    /// Default member configuration
    #[serde(default)]
    pub default_members: Vec<String>,
}

/// Workspace member package information
#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    /// Package name
    pub name: String,
    
    /// Path to package directory
    pub path: PathBuf,
    
    /// Package metadata
    pub metadata: PackageMetadata,
    
    /// Local dependencies within workspace
    pub local_dependencies: Vec<String>,
}

/// Workspace manager
#[derive(Debug)]
pub struct WorkspaceManager {
    /// Root directory of the workspace
    root: PathBuf,
    
    /// Workspace configuration
    config: Option<WorkspaceConfig>,
    
    /// Discovered workspace members
    members: Vec<WorkspaceMember>,
    
    /// Workspace-level lock file
    lock_file_manager: LockFileManager,
}

/// Workspace-specific errors
#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Workspace not found - no CursedPackage.toml with [workspace] section found")]
    NotFound,
    
    #[error("Invalid workspace configuration: {reason}")]
    InvalidConfig { reason: String },
    
    #[error("Member package not found: {path}")]
    MemberNotFound { path: String },
    
    #[error("Circular dependency in workspace: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },
    
    #[error("Workspace member {member} has invalid metadata: {reason}")]
    InvalidMemberMetadata { member: String, reason: String },
    
    #[error("Dependency conflict in workspace: {package} requires {version1} and {version2}")]
    DependencyConflict {
        package: String,
        version1: String,
        version2: String,
    },
    
    #[error("Workspace lock file error: {0}")]
    LockFile(#[from] super::lockfile::LockFileError),
    
    #[error("Glob pattern error: {0}")]
    GlobPattern(#[from] glob::PatternError),
    
    #[error("Glob matching error: {0}")]
    GlobMatch(#[from] glob::GlobError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),
}

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn discover<P: AsRef<Path>>(root: P) -> Result<(), Error> {
        let root = root.as_ref().to_path_buf();
        let package_file = root.join("CursedPackage.toml");
        
        if !package_file.exists() {
            return Err(WorkspaceError::NotFound);
        }
        
        // Read and parse package file
        let content = std::fs::read_to_string(&package_file)?;
        let parsed: toml::Value = toml::from_str(&content)?;
        
        // Check if workspace configuration exists
        let config = if let Some(workspace_section) = parsed.get("workspace") {
            Some(workspace_section.clone().try_into().map_err(|e| {
                WorkspaceError::InvalidConfig {
                    reason: format!("Failed to parse workspace config: {}", e),
                }
            })?)
        } else {
            None
        };
        
        let lock_file_manager = LockFileManager::new(root.join("CursedPackage.lock"));
        
        let mut workspace = Self {
            root,
            config,
            members: Vec::new(),
            lock_file_manager,
        };
        
        // Discover members if workspace config exists
        if workspace.config.is_some() {
            workspace.discover_members()?;
        }
        
        Ok(workspace)
    }
    
    /// Check if the current directory is a workspace
    pub fn is_workspace(&self) -> bool {
        self.config.is_some()
    }
    
    /// Get workspace configuration
    pub fn config(&self) -> Option<&WorkspaceConfig> {
        self.config.as_ref()
    }
    
    /// Get workspace members
    pub fn members(&self) -> &[WorkspaceMember] {
        &self.members
    }
    
    /// Get workspace root
    pub fn root(&self) -> &Path {
        &self.root
    }
    
    /// Discover workspace members from configuration
    fn discover_members(&mut self) -> Result<(), Error> {
        let config = self.config.as_ref().ok_or(WorkspaceError::NotFound)?;
        
        let mut discovered_paths = HashSet::new();
        
        // Process member patterns
        for pattern in &config.members {
            let full_pattern = self.root.join(pattern);
            let glob_pattern = full_pattern.to_string_lossy();
            
            for entry in glob(&glob_pattern)? {
                let path = entry?;
                if path.is_dir() {
                    discovered_paths.insert(path);
                }
            }
        }
        
        // Remove excluded paths
        for exclude_pattern in &config.exclude {
            let full_pattern = self.root.join(exclude_pattern);
            let glob_pattern = full_pattern.to_string_lossy();
            
            for entry in glob(&glob_pattern)? {
                let path = entry?;
                discovered_paths.remove(&path);
            }
        }
        
        // Load member metadata
        self.members.clear();
        for member_path in discovered_paths {
            let package_file = member_path.join("CursedPackage.toml");
            
            if package_file.exists() {
                let content = std::fs::read_to_string(&package_file)?;
                let metadata: PackageMetadata = toml::from_str(&content).map_err(|e| {
                    WorkspaceError::InvalidMemberMetadata {
                        member: member_path.to_string_lossy().to_string(),
                        reason: e.to_string(),
                    }
                })?;
                
                // Determine local dependencies within workspace
                let local_dependencies = self.find_local_dependencies(&metadata)?;
                
                let member = WorkspaceMember {
                    name: metadata.name.clone(),
                    path: member_path,
                    metadata,
                    local_dependencies,
                };
                
                self.members.push(member);
            }
        }
        
        // Validate no circular dependencies
        self.validate_dependency_graph()?;
        
        tracing::info!(
            workspace_root = ?self.root,
            member_count = self.members.len(),
            "Workspace discovered successfully"
        );
        
        Ok(())
    }
    
    /// Find dependencies that are local to the workspace
    fn find_local_dependencies(&self, metadata: &PackageMetadata) -> Result<(), Error> {
        let mut local_deps = Vec::new();
        
        for (dep_name, _version) in &metadata.dependencies {
            // Check if this dependency is a workspace member
            if self.members.iter().any(|m| &m.name == dep_name) {
                local_deps.push(dep_name.clone());
            }
        }
        
        Ok(local_deps)
    }
    
    /// Validate that workspace has no circular dependencies
    fn validate_dependency_graph(&self) -> Result<(), Error> {
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        
        for member in &self.members {
            if !visited.contains(&member.name) {
                self.visit_member_for_cycles(&member.name, &mut visited, &mut visiting)?;
            }
        }
        
        Ok(())
    }
    
    /// Visit a member during cycle detection
    fn visit_member_for_cycles(
        &self,
        member_name: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> Result<(), Error> {
        if visiting.contains(member_name) {
            // Found a cycle
            let cycle: Vec<String> = visiting.iter().cloned().collect();
            return Err(WorkspaceError::CircularDependency { cycle });
        }
        
        if visited.contains(member_name) {
            return Ok(());
        }
        
        visiting.insert(member_name.to_string());
        
        // Visit dependencies
        if let Some(member) = self.members.iter().find(|m| m.name == member_name) {
            for dep in &member.local_dependencies {
                self.visit_member_for_cycles(dep, visited, visiting)?;
            }
        }
        
        visiting.remove(member_name);
        visited.insert(member_name.to_string());
        
        Ok(())
    }
    
    /// Get build order for workspace members
    pub fn get_build_order(&self) -> Result<(), Error> {
        let mut build_order = Vec::new();
        let mut built = HashSet::new();
        
        // Repeatedly find members with all dependencies built
        while build_order.len() < self.members.len() {
            let mut made_progress = false;
            
            for member in &self.members {
                if built.contains(&member.name) {
                    continue;
                }
                
                // Check if all local dependencies are built
                let can_build = member.local_dependencies.iter()
                    .all(|dep| built.contains(dep));
                
                if can_build {
                    build_order.push(member);
                    built.insert(member.name.clone());
                    made_progress = true;
                }
            }
            
            if !made_progress {
                // This shouldn't happen if we validated the dependency graph
                return Err(WorkspaceError::CircularDependency {
                    cycle: self.members.iter()
                        .filter(|m| !built.contains(&m.name))
                        .map(|m| m.name.clone())
                        .collect(),
                });
            }
        }
        
        Ok(build_order)
    }
    
    /// Initialize a new workspace
    pub fn init_workspace<P: AsRef<Path>>(
        root: P,
        members: Vec<String>,
    ) -> Result<(), Error> {
        let root = root.as_ref().to_path_buf();
        let package_file = root.join("CursedPackage.toml");
        
        // Create workspace configuration
        let workspace_config = WorkspaceConfig {
            members,
            exclude: Vec::new(),
            dependencies: HashMap::new(),
            default_members: Vec::new(),
        };
        
        // Create basic package metadata for workspace root
        let root_metadata = PackageMetadata {
            name: root.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("workspace")
                .to_string(),
            version: "0.1.0".to_string(),
            description: "A CURSED workspace".to_string(),
            authors: vec!["Your Name <your.email@example.com>".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        };
        
        // Create TOML structure
        let mut toml_value = toml::Value::Table(toml::map::Map::new());
        
        // Add package metadata fields manually
        let table = toml_value.as_table_mut().unwrap();
        table.insert("name".to_string(), toml::Value::String(root_metadata.name));
        table.insert("version".to_string(), toml::Value::String(root_metadata.version));
        table.insert("description".to_string(), toml::Value::String(root_metadata.description));
        table.insert("authors".to_string(), toml::Value::Array(
            root_metadata.authors.into_iter().map(toml::Value::String).collect()
        ));
        
        // Add workspace configuration manually
        let mut workspace_table = toml::map::Map::new();
        workspace_table.insert("members".to_string(), toml::Value::Array(
            workspace_config.members.into_iter().map(toml::Value::String).collect()
        ));
        if !workspace_config.exclude.is_empty() {
            workspace_table.insert("exclude".to_string(), toml::Value::Array(
                workspace_config.exclude.into_iter().map(toml::Value::String).collect()
            ));
        }
        if !workspace_config.dependencies.is_empty() {
            let mut deps_table = toml::map::Map::new();
            for (key, value) in workspace_config.dependencies {
                deps_table.insert(key, toml::Value::String(value));
            }
            workspace_table.insert("dependencies".to_string(), toml::Value::Table(deps_table));
        }
        let workspace_value = toml::Value::Table(workspace_table);
        
        toml_value.as_table_mut().unwrap().insert("workspace".to_string(), workspace_value);
        
        // Write to file
        let content = toml::to_string_pretty(&toml_value).map_err(|e| {
            WorkspaceError::InvalidConfig {
                reason: format!("Failed to serialize TOML: {}", e),
            }
        })?;
        
        std::fs::create_dir_all(&root)?;
        std::fs::write(&package_file, content)?;
        
        // Initialize workspace manager
        Self::discover(root)
    }
    
    /// Add a member to the workspace
    pub fn add_member(&mut self, member_pattern: String) -> Result<(), Error> {
        let config = self.config.as_mut().ok_or(WorkspaceError::NotFound)?;
        
        if !config.members.contains(&member_pattern) {
            config.members.push(member_pattern);
            self.save_config()?;
            self.discover_members()?;
        }
        
        Ok(())
    }
    
    /// Remove a member from the workspace
    pub fn remove_member(&mut self, member_pattern: &str) -> Result<(), Error> {
        let config = self.config.as_mut().ok_or(WorkspaceError::NotFound)?;
        
        config.members.retain(|pattern| pattern != member_pattern);
        self.save_config()?;
        self.discover_members()?;
        
        Ok(())
    }
    
    /// Save workspace configuration
    fn save_config(&self) -> Result<(), Error> {
        let config = self.config.as_ref().ok_or(WorkspaceError::NotFound)?;
        let package_file = self.root.join("CursedPackage.toml");
        
        // Read existing file
        let content = std::fs::read_to_string(&package_file)?;
        let mut toml_value: toml::Value = toml::from_str(&content)?;
        
        // Update workspace section manually
        let mut workspace_table = toml::map::Map::new();
        workspace_table.insert("members".to_string(), toml::Value::Array(
            config.members.iter().map(|s| toml::Value::String(s.clone())).collect()
        ));
        if !config.exclude.is_empty() {
            workspace_table.insert("exclude".to_string(), toml::Value::Array(
                config.exclude.iter().map(|s| toml::Value::String(s.clone())).collect()
            ));
        }
        if !config.dependencies.is_empty() {
            let mut deps_table = toml::map::Map::new();
            for (key, value) in &config.dependencies {
                deps_table.insert(key.clone(), toml::Value::String(value.clone()));
            }
            workspace_table.insert("dependencies".to_string(), toml::Value::Table(deps_table));
        }
        let workspace_value = toml::Value::Table(workspace_table);
        
        toml_value.as_table_mut().unwrap().insert("workspace".to_string(), workspace_value);
        
        // Write back to file
        let new_content = toml::to_string_pretty(&toml_value).map_err(|e| {
            WorkspaceError::InvalidConfig {
                reason: format!("Failed to serialize TOML: {}", e),
            }
        })?;
        
        std::fs::write(&package_file, new_content)?;
        Ok(())
    }
    
    /// Generate workspace-level lock file
    pub fn generate_lock_file(&mut self) -> Result<(), Error> {
        // Collect all dependencies from workspace members
        let mut all_dependencies: HashMap<String, String> = HashMap::new();
        
        for member in &self.members {
            for (name, version) in &member.metadata.dependencies {
                // Check for version conflicts
                let version_str = version.to_string();
                if let Some(existing_version) = all_dependencies.get(name) {
                    if existing_version != &version_str {
                        return Err(WorkspaceError::DependencyConflict {
                            package: name.clone(),
                            version1: existing_version.clone(),
                            version2: version_str,
                        });
                    }
                }
                all_dependencies.insert(name.clone(), version_str);
            }
        }
        
        // Convert to PackageMetadata for lock file generation
        let dependencies: Vec<PackageMetadata> = all_dependencies.into_iter()
            .map(|(name, version)| PackageMetadata {
                name,
                version,
                description: "Workspace dependency".to_string(),
                authors: Vec::new(),
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                repository: None,
                license: None,
                keywords: Vec::new(),
                categories: Vec::new(),
            })
            .collect();
        
        self.lock_file_manager.generate_from_dependencies(
            &dependencies,
            Some(self.root.to_string_lossy().to_string()),
        )?;
        
        self.lock_file_manager.save()?;
        Ok(())
    }
    
    /// Load workspace lock file
    pub fn load_lock_file(&mut self) -> Result<(), Error> {
        self.lock_file_manager.load().map_err(Into::into)
    }
    
    /// Get member by name
    pub fn get_member(&self, name: &str) -> Option<&WorkspaceMember> {
        self.members.iter().find(|m| m.name == name)
    }
    
    /// Get member by path
    pub fn get_member_by_path<P: AsRef<Path>>(&self, path: P) -> Option<&WorkspaceMember> {
        let path = path.as_ref();
        self.members.iter().find(|m| m.path == path)
    }
    
    /// List all workspace dependencies
    pub fn list_dependencies(&self) -> HashMap<String, Vec<String>> {
        let mut dependencies = HashMap::new();
        
        for member in &self.members {
            let mut member_deps = Vec::new();
            
            for (name, version) in &member.metadata.dependencies {
                member_deps.push(format!("{} {}", name, version.to_string()));
            }
            
            dependencies.insert(member.name.clone(), member_deps);
        }
        
        dependencies
    }
    
    /// Validate workspace integrity
    pub fn validate(&self) -> Result<(), Error> {
        // Check that all local dependencies exist in workspace
        for member in &self.members {
            for local_dep in &member.local_dependencies {
                if !self.members.iter().any(|m| &m.name == local_dep) {
                    return Err(WorkspaceError::MemberNotFound {
                        path: local_dep.clone(),
                    });
                }
            }
        }
        
        Ok(())
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            members: Vec::new(),
            exclude: Vec::new(),
            dependencies: HashMap::new(),
            default_members: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_workspace_config_default() {
        let config = WorkspaceConfig::default();
        assert!(config.members.is_empty());
        assert!(config.exclude.is_empty());
        assert!(config.dependencies.is_empty());
    }
    
    #[test]
    fn test_workspace_init() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_root = temp_dir.path();
        
        let members = vec!["package1".to_string(), "package2".to_string()];
        let workspace = WorkspaceManager::init_workspace(workspace_root, members.clone()).unwrap();
        
        assert!(workspace.is_workspace());
        assert_eq!(workspace.config().unwrap().members, members);
        
        // Check that CursedPackage.toml was created
        let package_file = workspace_root.join("CursedPackage.toml");
        assert!(package_file.exists());
    }
    
    #[test]
    fn test_workspace_discovery_no_workspace() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_root = temp_dir.path();
        
        // Create a regular package file without workspace section
        let package_file = workspace_root.join("CursedPackage.toml");
        std::fs::write(&package_file, r#"
            name = "test-package"
            version = "1.0.0"
            description = "Test package"
            authors = ["Test"]
        "#).unwrap();
        
        let workspace = WorkspaceManager::discover(workspace_root).unwrap();
        assert!(!workspace.is_workspace());
    }
    
    #[test]
    fn test_build_order_simple() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_root = temp_dir.path();
        
        // Create workspace with dependencies: package2 -> package1
        let members = vec!["package1".to_string(), "package2".to_string()];
        let mut workspace = WorkspaceManager::init_workspace(workspace_root, members).unwrap();
        
        // Mock members with dependencies
        workspace.members = vec![
            WorkspaceMember {
                name: "package1".to_string(),
                path: workspace_root.join("package1"),
                metadata: PackageMetadata {
                    name: "package1".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Package 1".to_string(),
                    authors: Vec::new(),
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    repository: None,
                    license: None,
                    keywords: Vec::new(),
                    categories: Vec::new(),
                },
                local_dependencies: Vec::new(),
            },
            WorkspaceMember {
                name: "package2".to_string(),
                path: workspace_root.join("package2"),
                metadata: PackageMetadata {
                    name: "package2".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Package 2".to_string(),
                    authors: Vec::new(),
                    dependencies: HashMap::new(),
                    dev_dependencies: HashMap::new(),
                    repository: None,
                    license: None,
                    keywords: Vec::new(),
                    categories: Vec::new(),
                },
                local_dependencies: vec!["package1".to_string()],
            },
        ];
        
        let build_order = workspace.get_build_order().unwrap();
        assert_eq!(build_order.len(), 2);
        assert_eq!(build_order[0].name, "package1");
        assert_eq!(build_order[1].name, "package2");
    }
}
