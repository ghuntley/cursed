/// Workspace Management System for CURSED
/// 
/// This module provides functionality for managing multi-package workspaces
/// including dependency resolution, build ordering, and lock file management.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::package_manager::VersionSpec;

/// Configuration for workspace management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub members: Vec<String>,
    pub exclude: Vec<String>,
    pub dependencies: HashMap<String, VersionSpec>,
    pub dev_dependencies: HashMap<String, VersionSpec>,
    pub workspace_dir: PathBuf,
}

/// Simplified package metadata for workspace members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePackageMetadata {
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

/// Represents a single workspace member
#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    pub name: String,
    pub path: PathBuf,
    pub metadata: WorkspacePackageMetadata,
    pub local_dependencies: Vec<String>,
    pub external_dependencies: HashMap<String, VersionSpec>,
}

/// Main workspace management system
#[derive(Debug)]
pub struct WorkspaceManager {
    config: WorkspaceConfig,
    members: Vec<WorkspaceMember>,
    root_path: PathBuf,
}

impl WorkspaceManager {
    /// Initialize a new workspace with the given members
    pub fn init_workspace(root: &Path, members: Vec<String>) -> crate::error::Result<Self> {
        let config = WorkspaceConfig {
            members,
            exclude: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            workspace_dir: root.to_path_buf(),
        };
        
        // Create workspace configuration file
        let workspace_toml = root.join("CursedWorkspace.toml");
        let toml_content = toml::to_string_pretty(&config)
            .map_err(|e| crate::error::CursedError::General(format!("Failed to serialize workspace config: {}", e)))?;
        
        std::fs::write(workspace_toml, toml_content)?;
        
        Ok(Self {
            config,
            members: Vec::new(),
            root_path: root.to_path_buf(),
        })
    }
    
    /// Discover workspace from existing directory structure
    pub fn discover(root: &Path) -> crate::error::Result<Self> {
        let workspace_toml = root.join("CursedWorkspace.toml");
        
        let config = if workspace_toml.exists() {
            let content = std::fs::read_to_string(&workspace_toml)?;
            toml::from_str(&content)
                .map_err(|e| crate::error::CursedError::General(format!("Failed to parse workspace config: {}", e)))?
        } else {
            // Auto-discover members by looking for CursedPackage.toml files
            let mut members = Vec::new();
            if let Ok(entries) = std::fs::read_dir(root) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            let package_toml = path.join("CursedPackage.toml");
                            if package_toml.exists() {
                                if let Some(name) = path.file_name() {
                                    if let Some(name_str) = name.to_str() {
                                        members.push(name_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            WorkspaceConfig {
                members,
                exclude: Vec::new(),
                dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
                workspace_dir: root.to_path_buf(),
            }
        };
        
        // Load member information
        let mut members = Vec::new();
        for member_name in &config.members {
            let member_path = root.join(member_name);
            let package_toml = member_path.join("CursedPackage.toml");
            
            if package_toml.exists() {
                let content = std::fs::read_to_string(&package_toml)?;
                let metadata: WorkspacePackageMetadata = toml::from_str(&content)
                    .map_err(|e| crate::error::CursedError::General(format!("Failed to parse package metadata for {}: {}", member_name, e)))?;
                
                // Separate local vs external dependencies
                let mut local_dependencies = Vec::new();
                let mut external_dependencies = HashMap::new();
                
                for (dep_name, dep_spec) in &metadata.dependencies {
                    if config.members.contains(dep_name) {
                        local_dependencies.push(dep_name.clone());
                    } else {
                        external_dependencies.insert(dep_name.clone(), dep_spec.clone());
                    }
                }
                
                members.push(WorkspaceMember {
                    name: member_name.clone(),
                    path: member_path,
                    metadata,
                    local_dependencies,
                    external_dependencies,
                });
            }
        }
        
        Ok(Self {
            config,
            members,
            root_path: root.to_path_buf(),
        })
    }
    
    /// Get workspace root path
    pub fn root(&self) -> &Path {
        &self.root_path
    }
    
    /// Get workspace configuration
    pub fn config(&self) -> &WorkspaceConfig {
        &self.config
    }
    
    /// Get all workspace members
    pub fn members(&self) -> &[WorkspaceMember] {
        &self.members
    }
    
    /// Get build order considering local dependencies
    pub fn get_build_order(&self) -> crate::error::Result<Vec<&WorkspaceMember>> {
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        let mut order = Vec::new();
        
        // Create a map for quick lookup
        let member_map: HashMap<String, &WorkspaceMember> = self.members.iter()
            .map(|m| (m.name.clone(), m))
            .collect();
        
        // Topological sort using DFS
        for member in &self.members {
            if !visited.contains(&member.name) {
                self.visit_member(member, &member_map, &mut visited, &mut visiting, &mut order)?;
            }
        }
        
        Ok(order)
    }
    
    /// Helper function for topological sort
    fn visit_member<'a>(
        &self,
        member: &'a WorkspaceMember,
        member_map: &HashMap<String, &'a WorkspaceMember>,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        order: &mut Vec<&'a WorkspaceMember>,
    ) -> crate::error::Result<()> {
        if visiting.contains(&member.name) {
            return Err(crate::error::CursedError::General(
                format!("Circular dependency detected involving {}", member.name)
            ));
        }
        
        if visited.contains(&member.name) {
            return Ok(());
        }
        
        visiting.insert(member.name.clone());
        
        // Visit all local dependencies first
        for dep_name in &member.local_dependencies {
            if let Some(dep_member) = member_map.get(dep_name) {
                self.visit_member(dep_member, member_map, visited, visiting, order)?;
            }
        }
        
        visiting.remove(&member.name);
        visited.insert(member.name.clone());
        order.push(member);
        
        Ok(())
    }
    
    /// List all dependencies for all members
    pub fn list_dependencies(&self) -> HashMap<String, Vec<String>> {
        let mut all_deps = HashMap::new();
        
        for member in &self.members {
            let mut member_deps = Vec::new();
            
            // Add local dependencies
            member_deps.extend(member.local_dependencies.clone());
            
            // Add external dependencies
            member_deps.extend(member.external_dependencies.keys().cloned());
            
            all_deps.insert(member.name.clone(), member_deps);
        }
        
        all_deps
    }
    
    /// Generate lock file for the entire workspace
    pub fn generate_lock_file(&mut self) -> crate::error::Result<()> {
        let lock_file_path = self.root_path.join("CursedPackage.lock");
        
        // Create lock file content
        let mut lock_content = String::new();
        lock_content.push_str("# CURSED Workspace Lock File\n");
        lock_content.push_str("# This file is automatically generated\n\n");
        
        // Add workspace members
        lock_content.push_str("[workspace]\n");
        for member in &self.members {
            lock_content.push_str(&format!("  {} = \"{}\"\n", member.name, member.metadata.version));
        }
        lock_content.push_str("\n");
        
        // Add dependencies for each member
        for member in &self.members {
            if !member.external_dependencies.is_empty() {
                lock_content.push_str(&format!("[dependencies.{}]\n", member.name));
                for (dep_name, dep_spec) in &member.external_dependencies {
                    let version_str = match dep_spec {
                        VersionSpec::Simple(v) => v.clone(),
                        VersionSpec::Range(r) => r.clone(),
                        VersionSpec::Git { url, branch } => format!("git+{}#{}", url, branch.as_deref().unwrap_or("main")),
                    };
                    lock_content.push_str(&format!("  {} = \"{}\"\n", dep_name, version_str));
                }
                lock_content.push_str("\n");
            }
        }
        
        std::fs::write(lock_file_path, lock_content)?;
        Ok(())
    }
    
    /// Validate workspace configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        // Check that all members exist
        for member_name in &self.config.members {
            let member_path = self.root_path.join(member_name);
            if !member_path.exists() {
                return Err(crate::error::CursedError::General(
                    format!("Workspace member {} does not exist at {:?}", member_name, member_path)
                ));
            }
            
            let package_toml = member_path.join("CursedPackage.toml");
            if !package_toml.exists() {
                return Err(crate::error::CursedError::General(
                    format!("Workspace member {} missing CursedPackage.toml", member_name)
                ));
            }
        }
        
        // Check for circular dependencies
        let _build_order = self.get_build_order()?;
        
        Ok(())
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            members: Vec::new(),
            exclude: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            workspace_dir: PathBuf::from("."),
        }
    }
}
