use crate::error::Error;
// Workspace management for CURSED language server
// 
// Handles multi-file projects, workspace folders, and project-wide operations
// with comprehensive semantic analysis using CURSED's compiler infrastructure

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use dashmap::DashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, error, info, instrument, warn};
use walkdir::WalkDir;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::type_system::TypeChecker, Type;
use crate::ast::Program;
use crate::error::Error as CursedError;
use crate::imports::{ImportResolver, ImportResolverConfig};

/// Workspace manager for the LSP server
pub struct WorkspaceManager {
    /// Current workspace folders
    workspace_folders: std::sync::RwLock<Vec<WorkspaceFolder>>,
    /// Root URI if no workspace folders
    root_uri: std::sync::RwLock<Option<Url>>,
    /// Project files cache
    project_files: DashMap<Url, ProjectFile>,
    /// Workspace symbol cache with semantic information
    workspace_symbols: std::sync::RwLock<Vec<WorkspaceSymbol>>,
    /// Type checker for semantic analysis
    type_checker: std::sync::RwLock<TypeChecker>,
    /// Import resolver for cross-file analysis
    import_resolver: std::sync::RwLock<ImportResolver>,
    /// Parsed AST cache for semantic analysis
    ast_cache: DashMap<Url, Program>,
}

/// Information about a project file with semantic analysis
#[derive(Debug, Clone)]
struct ProjectFile {
    uri: Url,
    path: PathBuf,
    file_type: ProjectFileType,
    last_modified: Option<std::time::SystemTime>,
    symbols: Vec<WorkspaceSymbol>,
    /// Compilation errors and warnings
    diagnostics: Vec<Diagnostic>,
    /// Type information for symbols
    type_info: HashMap<String, Type>,
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
    /// Create a new workspace manager with semantic analysis
    pub fn new() -> Self {
        Self {
            workspace_folders: std::sync::RwLock::new(Vec::new()),
            root_uri: std::sync::RwLock::new(None),
            project_files: DashMap::new(),
            workspace_symbols: std::sync::RwLock::new(Vec::new()),
            type_checker: std::sync::RwLock::new(TypeChecker::new()),
            import_resolver: std::sync::RwLock::new(ImportResolver::new(ImportResolverConfig::default())),
            ast_cache: DashMap::new(),
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
        let mut roots = Vec::new();
        for folder in folders.iter() {
            if let Ok(path) = folder.uri.to_file_path() {
                roots.push(path);
            }
        }
        roots
    }

    /// Scan workspace for project files
    #[instrument(skip(self))]
    pub async fn scan_workspace(&self) {
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
    pub async fn scan_directory(&self, root_path: &Path) {
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
                .map_or(false, |name| name.starts_with("."))
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
                            file_type: file_type.clone(),
                            last_modified,
                            symbols: Vec::new(), // Will be populated by semantic analysis
                            diagnostics: Vec::new(),
                            type_info: HashMap::new(),
                        };
                        
                        self.project_files.insert(uri, project_file);
                    }
                }
            }
        }
    }

    /// Check if a path should be ignored
    pub fn should_ignore_path(&self, path: &Path) -> bool {
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
    pub fn classify_file(&self, path: &Path) -> Option<ProjectFileType> {
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

    /// Update workspace symbols using semantic analysis
    #[instrument(skip(self))]
    pub async fn update_workspace_symbols(&self) {
        debug!("Updating workspace symbols with semantic analysis");
        
        let mut all_symbols = Vec::new();
        
        for entry in self.project_files.iter() {
            let project_file = entry.value();
            if project_file.file_type == ProjectFileType::CursedSource {
                if let Ok(content) = std::fs::read_to_string(&project_file.path) {
                    match self.analyze_file_semantically(&content, &project_file.uri).await {
                        Ok(symbols) => all_symbols.extend(symbols),
                        Err(err) => {
                            warn!("Failed to analyze file {}: {}", project_file.uri, err);
                            // Fallback to basic symbol extraction
                            let basic_symbols = self.extract_basic_symbols(&content, &project_file.uri);
                            all_symbols.extend(basic_symbols);
                        }
                    }
                }
            }
        }
        
        {
            let mut workspace_symbols = self.workspace_symbols.write().unwrap();
            *workspace_symbols = all_symbols;
        }
        
        debug!("Updated workspace with {} symbols using semantic analysis", 
               self.workspace_symbols.read().unwrap().len());
    }

    /// Analyze file semantically using CURSED compiler infrastructure
    pub async fn analyze_file_semantically(&self, content: &str, uri: &Url) -> Result<(), Error> {
        debug!("Performing semantic analysis on {}", uri);
        
        // Parse the file using CURSED lexer and parser
        let lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(lexer)?;
        let ast = parser.parse_program()?;
        
        // Cache the AST for later use
        self.ast_cache.insert(uri.clone(), ast.clone());
        
        // Note: Import resolver updated separately through file scanning
        
        // Type check the AST
        let mut symbols = Vec::new();
        {
            let mut type_checker = self.type_checker.write().unwrap();
            if let Err(err) = type_checker.check_program(&ast) {
                warn!("Type checking failed for {}: {}", uri, err);
                // Continue with symbol extraction even if type checking fails
            }
            
            // Extract symbols from the AST with type information
            symbols = self.extract_symbols_from_ast(&ast, uri, &type_checker);
        }
        
        Ok(symbols)
    }
    
    /// Extract symbols from AST with full semantic information
    pub fn extract_symbols_from_ast(&self, ast: &Program, uri: &Url, type_checker: &TypeChecker) -> Vec<WorkspaceSymbol> {
        let mut symbols = Vec::new();
        
        for (stmt_index, statement) in ast.statements.iter().enumerate() {
            let stmt_str = statement.string();
            let line_num = stmt_index as u32; // Approximate line number
            
            // Extract function declarations
            if stmt_str.contains("slay ") || stmt_str.contains("yolo ") {
                if let Some(func_symbol) = self.extract_function_symbol(&stmt_str, uri, line_num, type_checker) {
                    symbols.push(func_symbol);
                }
            }
            
            // Extract struct declarations
            if stmt_str.contains("squad ") {
                if let Some(struct_symbol) = self.extract_struct_symbol(&stmt_str, uri, line_num) {
                    symbols.push(struct_symbol);
                }
            }
            
            // Extract interface declarations
            if stmt_str.contains("collab ") {
                if let Some(interface_symbol) = self.extract_interface_symbol(&stmt_str, uri, line_num) {
                    symbols.push(interface_symbol);
                }
            }
            
            // Extract variable declarations
            if stmt_str.contains("facts ") || stmt_str.contains("sus ") {
                if let Some(var_symbol) = self.extract_variable_symbol(&stmt_str, uri, line_num, type_checker) {
                    symbols.push(var_symbol);
                }
            }
        }
        
        symbols
    }
    
    /// Extract basic symbols as fallback when semantic analysis fails
    pub fn extract_basic_symbols(&self, content: &str, uri: &Url) -> Vec<WorkspaceSymbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
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
                    
                    symbols.push(WorkspaceSymbol {
                        name: func_name,
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        location: OneOf::Left(location),
                        container_name: None,
                        data: None,
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
                    
                    symbols.push(WorkspaceSymbol {
                        name: struct_name,
                        kind: SymbolKind::STRUCT,
                        tags: None,
                        location: OneOf::Left(location),
                        container_name: None,
                        data: None,
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
                    
                    symbols.push(WorkspaceSymbol {
                        name: interface_name,
                        kind: SymbolKind::INTERFACE,
                        tags: None,
                        location: OneOf::Left(location),
                        container_name: None,
                        data: None,
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
                    
                    symbols.push(WorkspaceSymbol {
                        name: var_name,
                        kind: SymbolKind::VARIABLE,
                        tags: None,
                        location: OneOf::Left(location),
                        container_name: None,
                        data: None,
                    });
                }
            }
        }
        
        symbols
    }
    
    /// Extract function symbol with type information
    fn extract_function_symbol(&self, stmt: &str, uri: &Url, line: u32, type_checker: &TypeChecker) -> Option<WorkspaceSymbol> {
        if let Some(func_name) = self.extract_function_name(stmt) {
            let location = Location {
                uri: uri.clone(),
                range: Range {
                    start: Position { line, character: 0 },
                    end: Position { line, character: stmt.len() as u32 },
                },
            };
            
            // Extract function signature and return type
            let detail = if let Some((_, params, return_type)) = self.extract_function_signature(stmt) {
                if return_type.is_empty() {
                    format!("{}({})", func_name, params)
                } else {
                    format!("{}({}) -> {}", func_name, params, return_type)
                }
            } else {
                func_name.clone()
            };
            
            Some(WorkspaceSymbol {
                name: func_name,
                kind: SymbolKind::FUNCTION,
                tags: None,
                location: OneOf::Left(location),
                container_name: None,
                data: Some(serde_json::json!({
                    "detail": detail,
                    "type": "function"
                })),
            })
        } else {
            None
        }
    }
    
    /// Extract struct symbol
    fn extract_struct_symbol(&self, stmt: &str, uri: &Url, line: u32) -> Option<WorkspaceSymbol> {
        if let Some(struct_name) = self.extract_struct_name(stmt) {
            let location = Location {
                uri: uri.clone(),
                range: Range {
                    start: Position { line, character: 0 },
                    end: Position { line, character: stmt.len() as u32 },
                },
            };
            
            Some(WorkspaceSymbol {
                name: struct_name,
                kind: SymbolKind::STRUCT,
                tags: None,
                location: OneOf::Left(location),
                container_name: None,
                data: Some(serde_json::json!({
                    "type": "struct"
                })),
            })
        } else {
            None
        }
    }
    
    /// Extract interface symbol
    fn extract_interface_symbol(&self, stmt: &str, uri: &Url, line: u32) -> Option<WorkspaceSymbol> {
        if let Some(interface_name) = self.extract_interface_name(stmt) {
            let location = Location {
                uri: uri.clone(),
                range: Range {
                    start: Position { line, character: 0 },
                    end: Position { line, character: stmt.len() as u32 },
                },
            };
            
            Some(WorkspaceSymbol {
                name: interface_name,
                kind: SymbolKind::INTERFACE,
                tags: None,
                location: OneOf::Left(location),
                container_name: None,
                data: Some(serde_json::json!({
                    "type": "interface"
                })),
            })
        } else {
            None
        }
    }
    
    /// Extract variable symbol with type information
    fn extract_variable_symbol(&self, stmt: &str, uri: &Url, line: u32, type_checker: &TypeChecker) -> Option<WorkspaceSymbol> {
        if let Some(var_name) = self.extract_variable_name(stmt) {
            let location = Location {
                uri: uri.clone(),
                range: Range {
                    start: Position { line, character: 0 },
                    end: Position { line, character: stmt.len() as u32 },
                },
            };
            
            // Try to determine variable type
            let var_type = if let Ok(inferred_type) = type_checker.check_type(&var_name) {
                format!("{:?}", inferred_type)
            } else {
                // Fallback to basic type inference
                self.infer_basic_type_from_declaration(stmt)
            };
            
            let is_mutable = stmt.contains("sus ");
            
            Some(WorkspaceSymbol {
                name: var_name,
                kind: SymbolKind::VARIABLE,
                tags: None,
                location: OneOf::Left(location),
                container_name: None,
                data: Some(serde_json::json!({
                    "type": var_type,
                    "mutable": is_mutable
                })),
            })
        } else {
            None
        }
    }
    
    /// Basic type inference from variable declaration
    fn infer_basic_type_from_declaration(&self, decl: &str) -> String {
        if decl.contains("= \"") {
            "tea".to_string() // string
        } else if decl.contains("= true") || decl.contains("= false") {
            "facts".to_string() // bool
        } else if decl.contains("= ") {
            // Try to extract the assigned value
            if let Some(equals_pos) = decl.find("= ") {
                let value = &decl[equals_pos + 2..].trim_end_matches(';');
                if value.parse::<i32>().is_ok() {
                    "normie".to_string() // i32
                } else if value.parse::<f64>().is_ok() {
                    "meal".to_string() // f64
                } else {
                    "unknown".to_string()
                }
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        }
    }
    
    /// Extract function signature from declaration
    fn extract_function_signature(&self, line: &str) -> Option<(String, String, String)> {
        if line.contains("slay") || line.contains("yolo") {
            if let Some(paren_start) = line.find('(') {
                if let Some(paren_end) = line.find(')') {
                    let before_paren = &line[..paren_start];
                    let func_name = before_paren
                        .split_whitespace()
                        .last()?
                        .to_string();
                    
                    let params = line[paren_start + 1..paren_end].to_string();
                    
                    let return_type = if let Some(arrow_pos) = line.find("->") {
                        line[arrow_pos + 2..].split('{').next()?.trim().to_string()
                    } else {
                        String::new()
                    };
                    
                    return Some((func_name, params, return_type));
                }
            }
        }
        None
    }

    /// Search workspace symbols with semantic filtering
    #[instrument(skip(self))]
    pub async fn search_symbols(&self, query: &str) -> Vec<WorkspaceSymbol> {
        debug!("Searching workspace symbols for: {}", query);
        
        let symbols = self.workspace_symbols.read().unwrap();
        
        if query.is_empty() {
            return symbols.clone();
        }
        
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        for symbol in symbols.iter() {
            if symbol.name.to_lowercase().contains(&query_lower) {
                results.push(symbol.clone());
            }
        }
        results
    }
    
    /// Get AST for a file from cache
    pub async fn get_ast(&self, uri: &Url) -> Option<Program> {
        self.ast_cache.get(uri).map(|entry| entry.value().clone())
    }
    
    /// Get type information for a symbol
    pub async fn get_symbol_type(&self, uri: &Url, symbol_name: &str) -> Option<Type> {
        if let Some(file_info) = self.project_files.get(uri) {
            file_info.type_info.get(symbol_name).cloned()
        } else {
            None
        }
    }
    
    /// Get diagnostics for a file
    pub async fn get_file_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        if let Some(file_info) = self.project_files.get(uri) {
            file_info.diagnostics.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Update file content and reanalyze
    pub async fn update_file_content(&self, uri: &Url, content: &str) -> Result<(), Error> {
        debug!("Updating file content for: {}", uri);
        
        // Perform semantic analysis on updated content
        match self.analyze_file_semantically(content, uri).await {
            Ok(symbols) => {
                // Update project file with new symbols and clear diagnostics
                if let Some(mut file_entry) = self.project_files.get_mut(uri) {
                    file_entry.symbols = symbols;
                    file_entry.diagnostics.clear();
                    file_entry.last_modified = Some(std::time::SystemTime::now());
                }
                
                // Update workspace symbols
                self.update_workspace_symbols().await;
                Ok(())
            }
            Err(err) => {
                // Store diagnostic information for errors
                if let Some(mut file_entry) = self.project_files.get_mut(uri) {
                    file_entry.diagnostics = vec![Diagnostic {
                        range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("cursed-lsp".to_string()),
                        message: format!("Analysis failed: {}", err),
                        related_information: None,
                        tags: None,
                        data: None,
                    }];
                }
                Err(err)
            }
        }
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
                                diagnostics: Vec::new(),
                                type_info: HashMap::new(),
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
            WorkspaceSymbol {
                name: "main".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                location: OneOf::Left(Location {
                    uri: Url::parse("file:///test.csd").unwrap(),
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                }),
                container_name: None,
                data: None,
            },
            WorkspaceSymbol {
                name: "calculate".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                location: OneOf::Left(Location {
                    uri: Url::parse("file:///test.csd").unwrap(),
                    range: Range {
                        start: Position { line: 5, character: 0 },
                        end: Position { line: 5, character: 15 },
                    },
                }),
                container_name: None,
                data: None,
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
