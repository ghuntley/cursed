//! Workspace management for CURSED language server
//! 
//! Handles multi-file projects, workspace folders, and project-wide operations

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use dashmap::DashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, error, info, instrument, warn};
use walkdir::WalkDir;

/// Workspace manager for the LSP server
pub struct WorkspaceManager {
    /// Current workspace folders
    workspace_folders: std::sync::RwLock<Vec<WorkspaceFolder>>,
    /// Root URI if no workspace folders
    root_uri: std::sync::RwLock<Option<Url>>,
    /// Project files cache
    project_files: DashMap<Url, ProjectFile>,
    /// Workspace symbol cache
    workspace_symbols: std::sync::RwLock<Vec<SymbolInformation>>,
}

/// Information about a project file
#[derive(Debug, Clone)]
struct ProjectFile {
    uri: Url,
    path: PathBuf,
    file_type: ProjectFileType,
    last_modified: Option<std::time::SystemTime>,
    symbols: Vec<SymbolInformation>,
}

/// Type of project file
#[derive(Debug, Clone, PartialEq)]
enum ProjectFileType {
    CursedSource,      // .csd files
    CursedPackage,     // CursedPackage.toml
    CursedBuild,       // CursedBuild.toml
    Documentation,     // .md files
    Configuration,     // .toml, .yaml, .json config files
    Other,
}

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn new() -> Self {
        Self {
            workspace_folders: std::sync::RwLock::new(Vec::new()),
            root_uri: std::sync::RwLock::new(None),
            project_files: DashMap::new(),
            workspace_symbols: std::sync::RwLock::new(Vec::new()),
        }
    }

    /// Set workspace folders
    #[instrument(skip(self, folders))]
    pub async fn set_workspace_folders(&self, folders: Vec<WorkspaceFolder>) {
        info!("Setting workspace folders: {:?}", folders.len());
        
        {
            let mut workspace_folders = self.workspace_folders.write().unwrap();
            *workspace_folders = folders;
        }
        
        // Scan workspace for files
        self.scan_workspace().await;
    }

    /// Set root URI (fallback when no workspace folders)
    #[instrument(skip(self))]
    pub async fn set_root_uri(&self, uri: Url) {
        info!("Setting root URI: {}", uri);
        
        {
            let mut root_uri = self.root_uri.write().unwrap();
            *root_uri = Some(uri.clone());
        }
        
        // Convert to workspace folder and scan
        let workspace_folder = WorkspaceFolder {
            uri,
            name: "Root".to_string(),
        };
        self.set_workspace_folders(Vec::from([workspace_folder])).await;
    }

    /// Get all workspace folders
    pub async fn get_workspace_folders(&self) -> Vec<WorkspaceFolder> {
        self.workspace_folders.read().unwrap().clone()
    }

    /// Get workspace root paths
    pub async fn get_workspace_roots(&self) -> Vec<PathBuf> {
        let folders = self.workspace_folders.read().unwrap();
        folders
            .iter()
            .filter_map(|folder| {
                if let Ok(path) = folder.uri.to_file_path() {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Scan workspace for project files
    #[instrument(skip(self))]
    async fn scan_workspace(&self) {
        debug!("Scanning workspace for files");
        
        let folders = self.get_workspace_folders().await;
        
        for folder in folders {
            if let Ok(root_path) = folder.uri.to_file_path() {
                self.scan_directory(&root_path).await;
            }
        }
        
        // Update workspace symbols
        self.update_workspace_symbols().await;
        
        info!("Workspace scan complete. Found {} files", self.project_files.len());
    }

    /// Scan a directory for project files
    #[instrument(skip(self))]
    async fn scan_directory(&self, root_path: &Path) {
        debug!("Scanning directory: {:?}", root_path);
        
        for entry in WalkDir::new(root_path)
            .follow_links(false)
            .max_depth(10) // Reasonable depth limit
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip hidden files and directories
            if path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name.starts_with('.'))
            {
                continue;
            }
            
            // Skip common ignore patterns
            if self.should_ignore_path(path) {
                continue;
            }
            
            if path.is_file() {
                if let Some(file_type) = self.classify_file(path) {
                    if let Ok(uri) = Url::from_file_path(path) {
                        let last_modified = std::fs::metadata(path)
                            .ok()
                            .and_then(|meta| meta.modified().ok());
                        
                        let project_file = ProjectFile {
                            uri: uri.clone(),
                            path: path.to_path_buf(),
                            file_type,
                            last_modified,
                            symbols: Vec::new(), // Will be populated later
                        };
                        
                        self.project_files.insert(uri, project_file);
                    }
                }
            }
        }
    }

    /// Check if a path should be ignored
    fn should_ignore_path(&self, path: &Path) -> bool {
        let ignore_patterns = [
            "target",      // Rust/Cargo build directory
            "node_modules", // Node.js dependencies
            ".git",        // Git directory
            ".svn",        // SVN directory
            ".vscode",     // VS Code settings
            ".idea",       // IntelliJ settings
            "build",       // General build directory
            "dist",        // Distribution directory
            "output",      // Output directory
            ".devenv",     // Nix devenv
            ".direnv",     // direnv
        ];
        
        for component in path.components() {
            if let Some(name) = component.as_os_str().to_str() {
                if ignore_patterns.contains(&name) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Classify a file based on its extension and name
    fn classify_file(&self, path: &Path) -> Option<ProjectFileType> {
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            match file_name {
                "CursedPackage.toml" => return Some(ProjectFileType::CursedPackage),
                "CursedBuild.toml" => return Some(ProjectFileType::CursedBuild),
                _ => {}
            }
        }
        
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension {
                "csd" => Some(ProjectFileType::CursedSource),
                "md" => Some(ProjectFileType::Documentation),
                "toml" | "yaml" | "yml" | "json" => Some(ProjectFileType::Configuration),
                _ => Some(ProjectFileType::Other),
            }
        } else {
            None
        }
    }

    /// Update workspace symbols from all files
    #[instrument(skip(self))]
    async fn update_workspace_symbols(&self) {
        debug!("Updating workspace symbols");
        
        let mut all_symbols = Vec::new();
        
        for entry in self.project_files.iter() {
            let project_file = entry.value();
            if project_file.file_type == ProjectFileType::CursedSource {
                if let Ok(content) = std::fs::read_to_string(&project_file.path) {
                    let symbols = self.extract_symbols_from_content(&content, &project_file.uri);
                    all_symbols.extend(symbols);
                }
            }
        }
        
        {
            let mut workspace_symbols = self.workspace_symbols.write().unwrap();
            *workspace_symbols = all_symbols;
        }
        
        debug!("Updated workspace with {} symbols", 
               self.workspace_symbols.read().unwrap().len());
    }

    /// Extract symbols from file content
    fn extract_symbols_from_content(&self, content: &str, uri: &Url) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Extract function declarations
            if line.contains("slay") || line.contains("yolo") {
                if let Some(func_name) = self.extract_function_name(line) {
                    let location = Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position { line: line_num as u32, character: 0 },
                            end: Position { line: line_num as u32, character: line.len() as u32 },
                        },
                    };
                    
                    symbols.push(SymbolInformation {
                        name: func_name,
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        deprecated: None,
                        location,
                        container_name: None,
                    });
                }
            }
            
            // Extract struct declarations
            if line.contains("squad") {
                if let Some(struct_name) = self.extract_struct_name(line) {
                    let location = Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position { line: line_num as u32, character: 0 },
                            end: Position { line: line_num as u32, character: line.len() as u32 },
                        },
                    };
                    
                    symbols.push(SymbolInformation {
                        name: struct_name,
                        kind: SymbolKind::STRUCT,
                        tags: None,
                        deprecated: None,
                        location,
                        container_name: None,
                    });
                }
            }
            
            // Extract interface declarations
            if line.contains("collab") {
                if let Some(interface_name) = self.extract_interface_name(line) {
                    let location = Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position { line: line_num as u32, character: 0 },
                            end: Position { line: line_num as u32, character: line.len() as u32 },
                        },
                    };
                    
                    symbols.push(SymbolInformation {
                        name: interface_name,
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        deprecated: None,
                        location,
                        container_name: None,
                    });
                }
            }
            
            // Extract variable declarations
            if line.contains("facts") || line.contains("sus") {
                if let Some(var_name) = self.extract_variable_name(line) {
                    let location = Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position { line: line_num as u32, character: 0 },
                            end: Position { line: line_num as u32, character: line.len() as u32 },
                        },
                    };
                    
                    symbols.push(SymbolInformation {
                        name: var_name,
                        kind: SymbolKind::VARIABLE,
                        tags: None,
                        deprecated: None,
                        location,
                        container_name: None,
                    });
                }
            }
        }
        
        symbols
    }

    /// Search workspace symbols
    #[instrument(skip(self))]
    pub async fn search_symbols(&self, query: &str) -> Vec<SymbolInformation> {
        debug!("Searching workspace symbols for: {}", query);
        
        let symbols = self.workspace_symbols.read().unwrap();
        
        if query.is_empty() {
            return symbols.clone();
        }
        
        let query_lower = query.to_ascii_lowercase();
        symbols
            .iter()
            .filter(|symbol| {
                symbol.name.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Get all project files of a specific type
    pub async fn get_files_by_type(&self, file_type: ProjectFileType) -> Vec<Url> {
        self.project_files
            .iter()
            .filter(|entry| entry.value().file_type == file_type)
            .map(|entry| entry.key().clone())
            .collect()
    }

    /// Get all CURSED source files
    pub async fn get_cursed_files(&self) -> Vec<Url> {
        self.get_files_by_type(ProjectFileType::CursedSource).await
    }

    /// Get project configuration files
    pub async fn get_config_files(&self) -> Vec<Url> {
        let mut configs = Vec::new();
        configs.extend(self.get_files_by_type(ProjectFileType::CursedPackage).await);
        configs.extend(self.get_files_by_type(ProjectFileType::CursedBuild).await);
        configs.extend(self.get_files_by_type(ProjectFileType::Configuration).await);
        configs
    }

    /// Check if a file is part of the workspace
    pub async fn is_workspace_file(&self, uri: &Url) -> bool {
        self.project_files.contains_key(uri)
    }

    /// Get file info
    pub async fn get_file_info(&self, uri: &Url) -> Option<ProjectFile> {
        self.project_files.get(uri).map(|entry| entry.value().clone())
    }

    /// Refresh workspace (rescan files)
    #[instrument(skip(self))]
    pub async fn refresh_workspace(&self) {
        info!("Refreshing workspace");
        
        // Clear current files
        self.project_files.clear();
        
        // Rescan
        self.scan_workspace().await;
    }

    /// File change notification
    #[instrument(skip(self))]
    pub async fn handle_file_changes(&self, changes: Vec<FileEvent>) {
        debug!("Handling {} file changes", changes.len());
        
        let mut needs_symbol_update = false;
        
        for change in changes {
            match change.typ {
                x if x == FileChangeType::CREATED => {
                    // Add new file to workspace
                    if let Ok(path) = change.uri.to_file_path() {
                        if let Some(file_type) = self.classify_file(&path) {
                            let last_modified = std::fs::metadata(&path)
                                .ok()
                                .and_then(|meta| meta.modified().ok());
                            
                            let project_file = ProjectFile {
                                uri: change.uri.clone(),
                                path,
                                file_type: file_type.clone(),
                                last_modified,
                                symbols: Vec::new(),
                            };
                            
                            self.project_files.insert(change.uri, project_file);
                            
                            if file_type == ProjectFileType::CursedSource {
                                needs_symbol_update = true;
                            }
                        }
                    }
                }
                x if x == FileChangeType::CHANGED => {
                    // Update file modification time
                    if let Some(mut entry) = self.project_files.get_mut(&change.uri) {
                        if let Ok(path) = change.uri.to_file_path() {
                            entry.last_modified = std::fs::metadata(&path)
                                .ok()
                                .and_then(|meta| meta.modified().ok());
                            
                            if entry.file_type == ProjectFileType::CursedSource {
                                needs_symbol_update = true;
                            }
                        }
                    }
                }
                x if x == FileChangeType::DELETED => {
                    // Remove file from workspace
                    if let Some(file_info) = self.project_files.remove(&change.uri) {
                        if file_info.1.file_type == ProjectFileType::CursedSource {
                            needs_symbol_update = true;
                        }
                    }
                }
                _ => {
                    // Handle unknown file change types
                }
            }
        }
        
        // Update symbols if needed
        if needs_symbol_update {
            self.update_workspace_symbols().await;
        }
    }

    /// Helper methods for symbol extraction

    fn extract_function_name(&self, line: &str) -> Option<String> {
        if let Some(paren_pos) = line.find('(') {
            let before_paren = &line[..paren_pos];
            if let Some(space_pos) = before_paren.rfind(' ') {
                Some(before_paren[space_pos + 1..].trim().to_string())
            } else if let Some(slay_pos) = before_paren.find("slay") {
                Some(before_paren[slay_pos + 4..].trim().to_string())
            } else if let Some(yolo_pos) = before_paren.find("yolo") {
                Some(before_paren[yolo_pos + 4..].trim().to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn extract_struct_name(&self, line: &str) -> Option<String> {
        if let Some(squad_pos) = line.find("squad") {
            let after_squad = &line[squad_pos + 5..];
            if let Some(brace_pos) = after_squad.find('{') {
                Some(after_squad[..brace_pos].trim().to_string())
            } else {
                Some(after_squad.split_whitespace().next()?.to_string())
            }
        } else {
            None
        }
    }

    fn extract_interface_name(&self, line: &str) -> Option<String> {
        if let Some(collab_pos) = line.find("collab") {
            let after_collab = &line[collab_pos + 6..];
            if let Some(brace_pos) = after_collab.find('{') {
                Some(after_collab[..brace_pos].trim().to_string())
            } else {
                Some(after_collab.split_whitespace().next()?.to_string())
            }
        } else {
            None
        }
    }

    fn extract_variable_name(&self, line: &str) -> Option<String> {
        if let Some(facts_pos) = line.find("facts") {
            let after_facts = &line[facts_pos + 5..];
            if let Some(equals_pos) = after_facts.find('=') {
                let var_part = &after_facts[..equals_pos].trim();
                if let Some(colon_pos) = var_part.find(':') {
                    Some(var_part[..colon_pos].trim().to_string())
                } else {
                    Some(var_part.to_string())
                }
            } else {
                None
            }
        } else if let Some(sus_pos) = line.find("sus") {
            let after_sus = &line[sus_pos + 3..];
            if let Some(equals_pos) = after_sus.find('=') {
                let var_part = &after_sus[..equals_pos].trim();
                if let Some(colon_pos) = var_part.find(':') {
                    Some(var_part[..colon_pos].trim().to_string())
                } else {
                    Some(var_part.to_string())
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get workspace statistics
    pub async fn get_workspace_stats(&self) -> WorkspaceStats {
        let total_files = self.project_files.len();
        let cursed_files = self.get_files_by_type(ProjectFileType::CursedSource).await.len();
        let config_files = self.get_config_files().await.len();
        let symbols = self.workspace_symbols.read().unwrap().len();
        
        WorkspaceStats {
            total_files,
            cursed_files,
            config_files,
            symbols,
        }
    }
}

/// Workspace statistics
#[derive(Debug, Clone)]
pub struct WorkspaceStats {
    pub total_files: usize,
    pub cursed_files: usize,
    pub config_files: usize,
    pub symbols: usize,
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_workspace_scanning() {
        let temp_dir = TempDir::new().unwrap();
        let root_path = temp_dir.path();
        
        // Create test files
        fs::write(root_path.join("main.csd"), "slay main() { print(\"hello\") }").unwrap();
        fs::write(root_path.join("CursedPackage.toml"), "[package]\nname = \"test\"").unwrap();
        fs::create_dir(root_path.join("src")).unwrap();
        fs::write(root_path.join("src").join("lib.csd"), "squad MyStruct { value: int }").unwrap();
        
        let manager = WorkspaceManager::new();
        let workspace_folder = WorkspaceFolder {
            uri: Url::from_file_path(root_path).unwrap(),
            name: "Test Workspace".to_string(),
        };
        
        manager.set_workspace_folders(Vec::from([workspace_folder])).await;
        
        // Check that files were found
        let cursed_files = manager.get_cursed_files().await;
        assert_eq!(cursed_files.len(), 2); // main.csd and src/lib.csd
        
        let config_files = manager.get_config_files().await;
        assert_eq!(config_files.len(), 1); // CursedPackage.toml
        
        // Check symbols
        let symbols = manager.search_symbols("").await;
        assert!(!symbols.is_empty());
        assert!(symbols.iter().any(|s| s.name == "main"));
        assert!(symbols.iter().any(|s| s.name == "MyStruct"));
    }

    #[tokio::test]
    async fn test_symbol_search() {
        let manager = WorkspaceManager::new();
        
        // Manually add some symbols for testing
        let symbols = vec![
            SymbolInformation {
                name: "main".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Url::parse("file:///test.csd").unwrap(),
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                },
                container_name: None,
            },
            SymbolInformation {
                name: "calculate".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Url::parse("file:///test.csd").unwrap(),
                    range: Range {
                        start: Position { line: 5, character: 0 },
                        end: Position { line: 5, character: 15 },
                    },
                },
                container_name: None,
            },
        ];
        
        {
            let mut workspace_symbols = manager.workspace_symbols.write().unwrap();
            *workspace_symbols = symbols;
        }
        
        // Test search
        let results = manager.search_symbols("calc").await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "calculate");
        
        let all_results = manager.search_symbols("").await;
        assert_eq!(all_results.len(), 2);
    }

    #[test]
    fn test_file_classification() {
        let manager = WorkspaceManager::new();
        
        assert_eq!(
            manager.classify_file(Path::new("main.csd")),
            Some(ProjectFileType::CursedSource)
        );
        
        assert_eq!(
            manager.classify_file(Path::new("CursedPackage.toml")),
            Some(ProjectFileType::CursedPackage)
        );
        
        assert_eq!(
            manager.classify_file(Path::new("README.md")),
            Some(ProjectFileType::Documentation)
        );
        
        assert_eq!(
            manager.classify_file(Path::new("config.json")),
            Some(ProjectFileType::Configuration)
        );
    }

    #[test]
    fn test_symbol_extraction() {
        let manager = WorkspaceManager::new();
        
        assert_eq!(
            manager.extract_function_name("slay main() {"),
            Some("main".to_string())
        );
        
        assert_eq!(
            manager.extract_struct_name("squad Person {"),
            Some("Person".to_string())
        );
        
        assert_eq!(
            manager.extract_variable_name("facts count: int = 42"),
            Some("count".to_string())
        );
    }
}
