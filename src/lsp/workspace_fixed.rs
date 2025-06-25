use crate::error::CursedError;
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
use crate::imports::{ImportResolver, ImportResolverConfig};

/// Workspace manager for the LSP server
pub struct WorkspaceManager {
    /// Current workspace folders
    /// Root URI if no workspace folders
    /// Project files cache
    /// Workspace symbol cache with semantic information
    /// Type checker for semantic analysis
    /// Import resolver for cross-file analysis
    /// Parsed AST cache for semantic analysis
/// Information about a project file with semantic analysis
#[derive(Debug, Clone)]
struct ProjectFile {
    /// Compilation errors and warnings
    /// Type information for symbols
/// Type of project file
#[derive(Debug, Clone, PartialEq)]
enum ProjectFileType {
    CursedSource,      // .csd files
    CursedPackage,     // CursedPackage.toml
    CursedBuild,       // CursedBuild.toml
    Documentation,     // .md files
    Configuration,     // .toml, .yaml, .json config files
impl WorkspaceManager {
    /// Create a new workspace manager with semantic analysis
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set workspace folders
    #[instrument(skip(self, folders))]
    pub async fn set_workspace_folders(&self, folders: Vec<WorkspaceFolder>) {
        info!("Setting workspace folders: {:?}", folders.len());
        
        {
            let mut workspace_folders = self.workspace_folders.write().unwrap();
            *workspace_folders = folders;
        // Note: Scan workspace for files would happen here
        // self.scan_workspace().await;
    /// Get all workspace folders
    pub async fn get_workspace_folders(&self) -> Vec<WorkspaceFolder> {
        self.workspace_folders.read().unwrap().clone()
    /// Search workspace symbols with semantic filtering
    #[instrument(skip(self))]
    pub async fn search_symbols(&self, query: &str) -> Vec<WorkspaceSymbol> {
        debug!("Searching workspace symbols for: {}", query);
        
        let symbols = self.workspace_symbols.read().unwrap();
        
        if query.is_empty() {
            return symbols.clone();
        let query_lower = query.to_lowercase();
        (*symbols)
            .iter()
            .filter(|symbol| {
                symbol.name.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    /// Get AST for a file from cache
    pub async fn get_ast(&self, uri: &Url) -> Option<Program> {
        self.ast_cache.get(uri).map(|entry| entry.value().clone())
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
    /// Get file info
    pub async fn get_file_info(&self, uri: &Url) -> Option<ProjectFile> {
        self.project_files.get(uri).map(|entry| entry.value().clone())
    }
}

/// Workspace statistics
#[derive(Debug, Clone)]
pub struct WorkspaceStats {
impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}
