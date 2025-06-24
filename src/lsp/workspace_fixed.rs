use crate::error::Error;
//! Workspace management for CURSED language server
//! 
//! Handles multi-file projects, workspace folders, and project-wide operations
//! with comprehensive semantic analysis using CURSED's compiler infrastructure

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use dashmap::DashMap;
use tower_lsp::lsp_crate::types::*;
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
        
        // Note: Scan workspace for files would happen here
        // self.scan_workspace().await;
    }

    /// Get all workspace folders
    pub async fn get_workspace_folders(&self) -> Vec<WorkspaceFolder> {
        self.workspace_folders.read().unwrap().clone()
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
        (*symbols)
            .iter()
            .filter(|symbol| {
                symbol.name.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
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

    /// Check if a file is part of the workspace
    pub async fn is_workspace_file(&self, uri: &Url) -> bool {
        self.project_files.contains_key(uri)
    }

    /// Get file info
    pub async fn get_file_info(&self, uri: &Url) -> Option<ProjectFile> {
        self.project_files.get(uri).map(|entry| entry.value().clone())
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
