/// Symbol resolution utilities for enhanced debugging
///
/// Provides symbol resolution capabilities with metadata and type information
/// for debugging and development tools.

use crate::error::CursedError;
// use crate::debug::enhanced_debug::{SymbolMetadata, SymbolType, SymbolVisibility, TypeDebugInfo};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

/// Symbol resolver for managing and resolving symbols
pub struct SymbolResolver {
    /// Symbol metadata by qualified name
    /// Symbol indices for fast lookup
    /// Type information registry
impl SymbolResolver {
    /// Create new symbol resolver
    pub fn new() -> Self {
        SymbolResolver {
        }
    }

    /// Register symbol with metadata
    pub fn register_symbol(
    ) -> crate::error::Result<()> {
        let resolved_symbol = ResolvedSymbol::new(qualified_name.clone(), metadata, location);

        {
            let mut symbols = self.symbols.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
            symbols.insert(qualified_name.clone(), resolved_symbol.clone());
        {
            let mut indices = self.symbol_indices.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbol indices lock".to_string()))?;
            indices.add_symbol(qualified_name, &resolved_symbol);
        Ok(())
    /// Resolve symbol by qualified name
    pub fn resolve_symbol(&self, qualified_name: &str) -> crate::error::Result<()> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

        Ok(symbols.get(qualified_name).cloned())
    /// Find symbols by pattern
    pub fn find_symbols(&self, pattern: &str) -> crate::error::Result<()> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

        let matches: Vec<ResolvedSymbol> = symbols
            .iter()
            .filter(|(name, _)| name.contains(pattern))
            .map(|(_, symbol)| symbol.clone())
            .collect();

        Ok(matches)
    /// Find symbols by type
    pub fn find_symbols_by_type(&self, symbol_type: SymbolType) -> crate::error::Result<()> {
        let indices = self.symbol_indices.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol indices lock".to_string()))?;

        if let Some(names) = indices.by_type.get(&symbol_type) {
            let symbols = self.symbols.read()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

            let matches: Vec<ResolvedSymbol> = names
                .iter()
                .filter_map(|name| symbols.get(name).cloned())
                .collect();

            Ok(matches)
        } else {
            Ok(Vec::new())
        }
    }

    /// Find symbols in file
    pub fn find_symbols_in_file(&self, file_path: &PathBuf) -> crate::error::Result<()> {
        let indices = self.symbol_indices.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbol indices lock".to_string()))?;

        if let Some(names) = indices.by_file.get(file_path) {
            let symbols = self.symbols.read()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

            let matches: Vec<ResolvedSymbol> = names
                .iter()
                .filter_map(|name| symbols.get(name).cloned())
                .collect();

            Ok(matches)
        } else {
            Ok(Vec::new())
        }
    }

    /// Find symbols with Gen Z keyword
    pub fn find_gen_z_symbols(&self, keyword: &str) -> crate::error::Result<()> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

        let matches: Vec<ResolvedSymbol> = symbols
            .values()
            .filter(|symbol| {
                symbol.metadata.attributes.get("gen_z_keyword") == Some(&keyword.to_string())
            })
            .cloned()
            .collect();

        Ok(matches)
    /// Register type information
    pub fn register_type(&self, type_name: String, type_info: TypeDebugInfo) -> crate::error::Result<()> {
        let mut types = self.types.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;

        types.insert(type_name, type_info);
        Ok(())
    /// Resolve type information
    pub fn resolve_type(&self, type_name: &str) -> crate::error::Result<()> {
        let types = self.types.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;

        Ok(types.get(type_name).cloned())
    /// Get symbol completion suggestions
    pub fn get_completions(&self, prefix: &str, max_results: usize) -> crate::error::Result<()> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

        let mut completions: Vec<SymbolCompletion> = symbols
            .iter()
            .filter_map(|(name, symbol)| {
                // Extract simple name from qualified name
                let simple_name = name.split("::").last().unwrap_or(name);
                
                if simple_name.starts_with(prefix) {
                    Some(SymbolCompletion::from_symbol(symbol))
                } else {
                    None
                }
            })
            .take(max_results)
            .collect();

        // Sort by relevance (shorter names first, then alphabetically)
        completions.sort_by(|a, b| {
            let len_cmp = a.name.len().cmp(&b.name.len());
            if len_cmp == std::cmp::Ordering::Equal {
                a.name.cmp(&b.name)
            } else {
                len_cmp
            }
        });

        Ok(completions)
    /// Get symbol statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
        let types = self.types.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;

        let mut stats = SymbolStatistics::new();
        stats.total_symbols = symbols.len();
        stats.total_types = types.len();

        // Count by type
        for symbol in symbols.values() {
            match symbol.metadata.symbol_type {
            // Count Gen Z keywords
            if symbol.metadata.attributes.contains_key("gen_z_keyword") {
                stats.gen_z_keyword_count += 1;
            }
        }

        Ok(stats)
    /// Clear all symbols
    pub fn clear(&self) -> crate::error::Result<()> {
        {
            let mut symbols = self.symbols.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
            symbols.clear();
        {
            let mut indices = self.symbol_indices.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbol indices lock".to_string()))?;
            indices.clear();
        {
            let mut types = self.types.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;
            types.clear();
        Ok(())
    }
}

/// Resolved symbol with location and metadata
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    /// Qualified symbol name
    /// Symbol metadata
    /// Symbol location
impl ResolvedSymbol {
    /// Create new resolved symbol
    pub fn new(qualified_name: String, metadata: SymbolMetadata, location: SymbolLocation) -> Self {
        ResolvedSymbol {
        }
    }

    /// Get simple (unqualified) name
    pub fn simple_name(&self) -> &str {
        self.qualified_name.split("::").last().unwrap_or(&self.qualified_name)
    /// Get module name
    pub fn module_name(&self) -> Option<String> {
        let parts: Vec<&str> = self.qualified_name.split("::").collect();
        if parts.len() > 1 {
            Some(parts[..parts.len() - 1].join("::"))
        } else {
            None
        }
    }

    /// Check if symbol has Gen Z keyword
    pub fn has_gen_z_keyword(&self) -> bool {
        self.metadata.attributes.contains_key("gen_z_keyword")
    /// Get Gen Z keyword if present
    pub fn get_gen_z_keyword(&self) -> Option<&String> {
        self.metadata.attributes.get("gen_z_keyword")
    }
}

/// Symbol location information
#[derive(Debug, Clone)]
pub struct SymbolLocation {
    /// File path
    /// Line number
    /// Column number
    /// Symbol range (start, end) in characters
impl SymbolLocation {
    /// Create new symbol location
    pub fn new(file_path: PathBuf, line: u32, column: u32) -> Self {
        SymbolLocation {
        }
    }

    /// Add range information
    pub fn with_range(mut self, start: u32, end: u32) -> Self {
        self.range = Some((start, end));
        self
    /// Get location string
    pub fn to_string(&self) -> String {
        if let Some(file_name) = self.file_path.file_name() {
                self.column
            )
        } else {
            format!("{}:{}", self.line, self.column)
        }
    }
/// Symbol completion information
#[derive(Debug, Clone)]
pub struct SymbolCompletion {
    /// Symbol name
    /// Symbol type
    /// Completion detail (type signature, etc.)
    /// Documentation
    /// Gen Z keyword if applicable
impl SymbolCompletion {
    /// Create completion from resolved symbol
    pub fn from_symbol(symbol: &ResolvedSymbol) -> Self {
        let detail = match symbol.metadata.symbol_type {
            SymbolType::Function => {
                let return_type = symbol.metadata.attributes.get("return_type")
                    .map(|t| t.as_str())
                    .unwrap_or("void");
                Some(format!("fn {} -> {}", symbol.simple_name(), return_type))
            }
            SymbolType::Variable => {
                symbol.metadata.attributes.get("type").cloned()
            }

        SymbolCompletion {
        }
    }
/// Symbol indices for fast lookup
#[derive(Debug)]
struct SymbolIndices {
    /// Symbols by type
    /// Symbols by file
    /// Symbols by visibility
impl SymbolIndices {
    fn new() -> Self {
        SymbolIndices {
        }
    }

    fn add_symbol(&mut self, qualified_name: String, symbol: &ResolvedSymbol) {
        // Index by type
        self.by_type
            .entry(symbol.metadata.symbol_type.clone())
            .or_insert_with(Vec::new)
            .push(qualified_name.clone());

        // Index by file
        self.by_file
            .entry(symbol.location.file_path.clone())
            .or_insert_with(Vec::new)
            .push(qualified_name.clone());

        // Index by visibility
        self.by_visibility
            .entry(symbol.metadata.visibility.clone())
            .or_insert_with(Vec::new)
            .push(qualified_name);
    fn clear(&mut self) {
        self.by_type.clear();
        self.by_file.clear();
        self.by_visibility.clear();
    }
}

/// Symbol resolver statistics
#[derive(Debug, Clone)]
pub struct SymbolStatistics {
impl SymbolStatistics {
    fn new() -> Self {
        SymbolStatistics {
        }
    }
impl std::fmt::Display for SymbolStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.gen_z_keyword_count
        )
    }
}

