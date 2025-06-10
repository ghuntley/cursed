/// Symbol resolution utilities for enhanced debugging
///
/// Provides symbol resolution capabilities with metadata and type information
/// for debugging and development tools.

use crate::error::Error as CursedError;
use crate::debug::enhanced_debug::{SymbolMetadata, SymbolType, SymbolVisibility, TypeDebugInfo};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

/// Symbol resolver for managing and resolving symbols
pub struct SymbolResolver {
    /// Symbol metadata by qualified name
    symbols: Arc<RwLock<HashMap<String, ResolvedSymbol>>>,
    /// Symbol indices for fast lookup
    symbol_indices: Arc<RwLock<SymbolIndices>>,
    /// Type information registry
    types: Arc<RwLock<HashMap<String, TypeDebugInfo>>>,
}

impl SymbolResolver {
    /// Create new symbol resolver
    pub fn new() -> Self {
        SymbolResolver {
            symbols: Arc::new(RwLock::new(HashMap::new())),
            symbol_indices: Arc::new(RwLock::new(SymbolIndices::new())),
            types: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register symbol with metadata
    pub fn register_symbol(
        &self,
        qualified_name: String,
        metadata: SymbolMetadata,
        location: SymbolLocation,
    ) -> Result<(), CursedError> {
        let resolved_symbol = ResolvedSymbol::new(qualified_name.clone(), metadata, location);

        {
            let mut symbols = self.symbols.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
            symbols.insert(qualified_name.clone(), resolved_symbol.clone());
        }

        {
            let mut indices = self.symbol_indices.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbol indices lock".to_string()))?;
            indices.add_symbol(qualified_name, &resolved_symbol);
        }

        Ok(())
    }

    /// Resolve symbol by qualified name
    pub fn resolve_symbol(&self, qualified_name: &str) -> Result<Option<ResolvedSymbol>, CursedError> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

        Ok(symbols.get(qualified_name).cloned())
    }

    /// Find symbols by pattern
    pub fn find_symbols(&self, pattern: &str) -> Result<Vec<ResolvedSymbol>, CursedError> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;

        let matches: Vec<ResolvedSymbol> = symbols
            .iter()
            .filter(|(name, _)| name.contains(pattern))
            .map(|(_, symbol)| symbol.clone())
            .collect();

        Ok(matches)
    }

    /// Find symbols by type
    pub fn find_symbols_by_type(&self, symbol_type: SymbolType) -> Result<Vec<ResolvedSymbol>, CursedError> {
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
    pub fn find_symbols_in_file(&self, file_path: &PathBuf) -> Result<Vec<ResolvedSymbol>, CursedError> {
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
    pub fn find_gen_z_symbols(&self, keyword: &str) -> Result<Vec<ResolvedSymbol>, CursedError> {
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
    }

    /// Register type information
    pub fn register_type(&self, type_name: String, type_info: TypeDebugInfo) -> Result<(), CursedError> {
        let mut types = self.types.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;

        types.insert(type_name, type_info);
        Ok(())
    }

    /// Resolve type information
    pub fn resolve_type(&self, type_name: &str) -> Result<Option<TypeDebugInfo>, CursedError> {
        let types = self.types.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;

        Ok(types.get(type_name).cloned())
    }

    /// Get symbol completion suggestions
    pub fn get_completions(&self, prefix: &str, max_results: usize) -> Result<Vec<SymbolCompletion>, CursedError> {
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
    }

    /// Get symbol statistics
    pub fn get_statistics(&self) -> Result<SymbolStatistics, CursedError> {
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
                SymbolType::Function => stats.function_count += 1,
                SymbolType::Variable => stats.variable_count += 1,
                SymbolType::Type => stats.type_symbol_count += 1,
                SymbolType::Interface => stats.interface_count += 1,
                SymbolType::Struct => stats.struct_count += 1,
                SymbolType::Constant => stats.constant_count += 1,
                SymbolType::Module => stats.module_count += 1,
                SymbolType::Unknown => stats.unknown_count += 1,
            }

            // Count Gen Z keywords
            if symbol.metadata.attributes.contains_key("gen_z_keyword") {
                stats.gen_z_keyword_count += 1;
            }
        }

        Ok(stats)
    }

    /// Clear all symbols
    pub fn clear(&self) -> Result<(), CursedError> {
        {
            let mut symbols = self.symbols.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
            symbols.clear();
        }

        {
            let mut indices = self.symbol_indices.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire symbol indices lock".to_string()))?;
            indices.clear();
        }

        {
            let mut types = self.types.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;
            types.clear();
        }

        Ok(())
    }
}

/// Resolved symbol with location and metadata
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    /// Qualified symbol name
    pub qualified_name: String,
    /// Symbol metadata
    pub metadata: SymbolMetadata,
    /// Symbol location
    pub location: SymbolLocation,
}

impl ResolvedSymbol {
    /// Create new resolved symbol
    pub fn new(qualified_name: String, metadata: SymbolMetadata, location: SymbolLocation) -> Self {
        ResolvedSymbol {
            qualified_name,
            metadata,
            location,
        }
    }

    /// Get simple (unqualified) name
    pub fn simple_name(&self) -> &str {
        self.qualified_name.split("::").last().unwrap_or(&self.qualified_name)
    }

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
    }

    /// Get Gen Z keyword if present
    pub fn get_gen_z_keyword(&self) -> Option<&String> {
        self.metadata.attributes.get("gen_z_keyword")
    }
}

/// Symbol location information
#[derive(Debug, Clone)]
pub struct SymbolLocation {
    /// File path
    pub file_path: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Symbol range (start, end) in characters
    pub range: Option<(u32, u32)>,
}

impl SymbolLocation {
    /// Create new symbol location
    pub fn new(file_path: PathBuf, line: u32, column: u32) -> Self {
        SymbolLocation {
            file_path,
            line,
            column,
            range: None,
        }
    }

    /// Add range information
    pub fn with_range(mut self, start: u32, end: u32) -> Self {
        self.range = Some((start, end));
        self
    }

    /// Get location string
    pub fn to_string(&self) -> String {
        if let Some(file_name) = self.file_path.file_name() {
            format!("{}:{}:{}", 
                file_name.to_string_lossy(),
                self.line,
                self.column
            )
        } else {
            format!("{}:{}", self.line, self.column)
        }
    }
}

/// Symbol completion information
#[derive(Debug, Clone)]
pub struct SymbolCompletion {
    /// Symbol name
    pub name: String,
    /// Symbol type
    pub symbol_type: SymbolType,
    /// Completion detail (type signature, etc.)
    pub detail: Option<String>,
    /// Documentation
    pub documentation: Option<String>,
    /// Gen Z keyword if applicable
    pub gen_z_keyword: Option<String>,
}

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
            _ => None,
        };

        SymbolCompletion {
            name: symbol.simple_name().to_string(),
            symbol_type: symbol.metadata.symbol_type.clone(),
            detail,
            documentation: symbol.metadata.documentation.clone(),
            gen_z_keyword: symbol.metadata.attributes.get("gen_z_keyword").cloned(),
        }
    }
}

/// Symbol indices for fast lookup
#[derive(Debug)]
struct SymbolIndices {
    /// Symbols by type
    pub by_type: HashMap<SymbolType, Vec<String>>,
    /// Symbols by file
    pub by_file: HashMap<PathBuf, Vec<String>>,
    /// Symbols by visibility
    pub by_visibility: HashMap<SymbolVisibility, Vec<String>>,
}

impl SymbolIndices {
    fn new() -> Self {
        SymbolIndices {
            by_type: HashMap::new(),
            by_file: HashMap::new(),
            by_visibility: HashMap::new(),
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
    }

    fn clear(&mut self) {
        self.by_type.clear();
        self.by_file.clear();
        self.by_visibility.clear();
    }
}

/// Symbol resolver statistics
#[derive(Debug, Clone)]
pub struct SymbolStatistics {
    pub total_symbols: usize,
    pub total_types: usize,
    pub function_count: usize,
    pub variable_count: usize,
    pub type_symbol_count: usize,
    pub interface_count: usize,
    pub struct_count: usize,
    pub constant_count: usize,
    pub module_count: usize,
    pub unknown_count: usize,
    pub gen_z_keyword_count: usize,
}

impl SymbolStatistics {
    fn new() -> Self {
        SymbolStatistics {
            total_symbols: 0,
            total_types: 0,
            function_count: 0,
            variable_count: 0,
            type_symbol_count: 0,
            interface_count: 0,
            struct_count: 0,
            constant_count: 0,
            module_count: 0,
            unknown_count: 0,
            gen_z_keyword_count: 0,
        }
    }
}

impl std::fmt::Display for SymbolStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Symbols: {} total, {} functions, {} variables, {} types, {} Gen Z keywords",
            self.total_symbols,
            self.function_count,
            self.variable_count,
            self.type_symbol_count,
            self.gen_z_keyword_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::enhanced_debug::{SymbolMetadata, SymbolType, SymbolVisibility};

    #[test]
    fn test_symbol_resolver_creation() {
        let resolver = SymbolResolver::new();
        let stats = resolver.get_statistics().unwrap();
        assert_eq!(stats.total_symbols, 0);
    }

    #[test]
    fn test_symbol_registration_and_resolution() {
        let resolver = SymbolResolver::new();

        let metadata = SymbolMetadata::function("test_func", Some("slay"));
        let location = SymbolLocation::new(PathBuf::from("test.csd"), 42, 10);

        let result = resolver.register_symbol("module::test_func".to_string(), metadata, location);
        assert!(result.is_ok());

        let resolved = resolver.resolve_symbol("module::test_func");
        assert!(resolved.is_ok());
        assert!(resolved.unwrap().is_some());
    }

    #[test]
    fn test_symbol_search() {
        let resolver = SymbolResolver::new();

        // Register multiple symbols
        let symbols = vec![
            ("module::test_func1", "slay"),
            ("module::test_func2", "yolo"),
            ("other::helper", "facts"),
        ];

        for (name, keyword) in symbols {
            let metadata = SymbolMetadata::function(name, Some(keyword));
            let location = SymbolLocation::new(PathBuf::from("test.csd"), 1, 1);
            let _ = resolver.register_symbol(name.to_string(), metadata, location);
        }

        // Test pattern search
        let matches = resolver.find_symbols("test");
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // Test type search
        let functions = resolver.find_symbols_by_type(SymbolType::Function);
        assert!(functions.is_ok());
        assert_eq!(functions.unwrap().len(), 3);

        // Test Gen Z keyword search
        let slay_symbols = resolver.find_gen_z_symbols("slay");
        assert!(slay_symbols.is_ok());
        assert_eq!(slay_symbols.unwrap().len(), 1);
    }

    #[test]
    fn test_symbol_completion() {
        let resolver = SymbolResolver::new();

        let metadata = SymbolMetadata::function("test_function", Some("slay"))
            .with_attribute("return_type".to_string(), "sus".to_string());
        let location = SymbolLocation::new(PathBuf::from("test.csd"), 1, 1);

        let _ = resolver.register_symbol("module::test_function".to_string(), metadata, location);

        let completions = resolver.get_completions("test", 10);
        assert!(completions.is_ok());

        let completions = completions.unwrap();
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].name, "test_function");
        assert!(completions[0].detail.is_some());
        assert_eq!(completions[0].gen_z_keyword, Some("slay".to_string()));
    }

    #[test]
    fn test_resolved_symbol_methods() {
        let metadata = SymbolMetadata::function("test_func", Some("slay"));
        let location = SymbolLocation::new(PathBuf::from("test.csd"), 42, 10);
        let symbol = ResolvedSymbol::new("module::test_func".to_string(), metadata, location);

        assert_eq!(symbol.simple_name(), "test_func");
        assert_eq!(symbol.module_name(), Some("module".to_string()));
        assert!(symbol.has_gen_z_keyword());
        assert_eq!(symbol.get_gen_z_keyword(), Some(&"slay".to_string()));
    }

    #[test]
    fn test_symbol_location() {
        let location = SymbolLocation::new(PathBuf::from("test.csd"), 42, 10)
            .with_range(5, 15);

        assert_eq!(location.line, 42);
        assert_eq!(location.column, 10);
        assert_eq!(location.range, Some((5, 15)));
        assert!(location.to_string().contains("test.csd:42:10"));
    }

    #[test]
    fn test_type_registration_and_resolution() {
        let resolver = SymbolResolver::new();

        let type_info = TypeDebugInfo::new("TestStruct".to_string(), crate::debug::enhanced_debug::TypeKind::Struct);
        let result = resolver.register_type("TestStruct".to_string(), type_info);
        assert!(result.is_ok());

        let resolved_type = resolver.resolve_type("TestStruct");
        assert!(resolved_type.is_ok());
        assert!(resolved_type.unwrap().is_some());
    }

    #[test]
    fn test_file_based_symbol_search() {
        let resolver = SymbolResolver::new();

        let file1 = PathBuf::from("file1.csd");
        let file2 = PathBuf::from("file2.csd");

        // Register symbols in different files
        let metadata1 = SymbolMetadata::function("func1", None);
        let location1 = SymbolLocation::new(file1.clone(), 10, 1);
        let _ = resolver.register_symbol("func1".to_string(), metadata1, location1);

        let metadata2 = SymbolMetadata::function("func2", None);
        let location2 = SymbolLocation::new(file2.clone(), 20, 1);
        let _ = resolver.register_symbol("func2".to_string(), metadata2, location2);

        // Test file-based search
        let file1_symbols = resolver.find_symbols_in_file(&file1);
        assert!(file1_symbols.is_ok());
        assert_eq!(file1_symbols.unwrap().len(), 1);

        let file2_symbols = resolver.find_symbols_in_file(&file2);
        assert!(file2_symbols.is_ok());
        assert_eq!(file2_symbols.unwrap().len(), 1);
    }

    #[test]
    fn test_symbol_statistics() {
        let resolver = SymbolResolver::new();

        // Register various types of symbols
        let function_meta = SymbolMetadata::function("func", Some("slay"));
        let variable_meta = SymbolMetadata::variable("var", "sus");
        let location = SymbolLocation::new(PathBuf::from("test.csd"), 1, 1);

        let _ = resolver.register_symbol("func1".to_string(), function_meta.clone(), location.clone());
        let _ = resolver.register_symbol("func2".to_string(), function_meta, location.clone());
        let _ = resolver.register_symbol("var1".to_string(), variable_meta, location);

        let stats = resolver.get_statistics();
        assert!(stats.is_ok());

        let stats = stats.unwrap();
        assert_eq!(stats.total_symbols, 3);
        assert_eq!(stats.function_count, 2);
        assert_eq!(stats.variable_count, 1);
        assert_eq!(stats.gen_z_keyword_count, 2); // Two functions with "slay"
    }

    #[test]
    fn test_resolver_clear() {
        let resolver = SymbolResolver::new();

        let metadata = SymbolMetadata::function("test", None);
        let location = SymbolLocation::new(PathBuf::from("test.csd"), 1, 1);
        let _ = resolver.register_symbol("test".to_string(), metadata, location);

        let stats_before = resolver.get_statistics().unwrap();
        assert_eq!(stats_before.total_symbols, 1);

        let clear_result = resolver.clear();
        assert!(clear_result.is_ok());

        let stats_after = resolver.get_statistics().unwrap();
        assert_eq!(stats_after.total_symbols, 0);
    }
}
