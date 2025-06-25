/// Enhanced debug information system with source mapping and symbol resolution
///
/// Provides rich debug info structures with source mapping, symbol resolution 
/// with metadata and type information, source location tracking with column-level
/// precision, and integration with LLVM debug metadata.

use crate::error::CursedError;
// use crate::runtime::debug_info::{DebugInfo, VariableInfo, EnhancedStackFrame};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::fmt;

/// Enhanced debug information with source mapping
#[derive(Debug, Clone)]
pub struct EnhancedDebugInfo {
    /// Basic debug information  
    pub debug_info: DebugInfo,
    /// Source mapping information
    pub source_map: Option<SourceMap>,
    /// Symbol metadata
    pub symbol_metadata: SymbolMetadata,
    /// Type information
    pub type_info: Option<TypeDebugInfo>,
    /// Scope information
    pub scope_info: ScopeInfo,
}

impl EnhancedDebugInfo {
    /// Create new enhanced debug info
    pub fn new(
        file_path: impl AsRef<Path>,
        line: u32,
        column: u32,
        function_name: String,
    ) -> Self {
        let debug_info = DebugInfo::new(file_path, line, column, function_name);
        
        EnhancedDebugInfo {
            debug_info,
            source_map: None,
            symbol_metadata: SymbolMetadata::new(),
            type_info: None,
            scope_info: ScopeInfo::new(),
        }
    }

    /// Add source mapping
    pub fn with_source_map(mut self, source_map: SourceMap) -> Self {
        self.source_map = Some(source_map);
        self
    }

    /// Add symbol metadata
    pub fn with_symbol_metadata(mut self, metadata: SymbolMetadata) -> Self {
        self.symbol_metadata = metadata;
        self
    }

    /// Add type information
    pub fn with_type_info(mut self, type_info: TypeDebugInfo) -> Self {
        self.type_info = Some(type_info);
        self
    }

    /// Add scope information
    pub fn with_scope_info(mut self, scope_info: ScopeInfo) -> Self {
        self.scope_info = scope_info;
        self
    }

    /// Get source location string
    pub fn location_string(&self) -> String {
        if let Some(file_name) = self.debug_info.file_path.file_name() {
            format!("{}:{}:{}", 
                file_name.to_string_lossy(),
                self.debug_info.line,
                self.debug_info.column
            )
        } else {
            format!("{}:{}", self.debug_info.line, self.debug_info.column)
        }
    }

    /// Get fully qualified symbol name
    pub fn qualified_symbol_name(&self) -> String {
        if let Some(module) = &self.debug_info.module_name {
            format!("{}::{}", module, self.debug_info.function_name)
        } else {
            self.debug_info.function_name.clone()
        }
    }

    /// Check if this debug info represents user code
    pub fn is_user_code(&self) -> bool {
        self.debug_info.file_path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "csd")
            .unwrap_or(false)
    }
}

/// Source mapping information for precise location tracking
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// Original source file
    pub source_file: PathBuf,
    /// Generated source ranges
    pub source_ranges: Vec<SourceRange>,
    /// Line mapping from generated to original
    pub line_mapping: HashMap<u32, u32>,
    /// Column mapping from generated to original  
    pub column_mapping: HashMap<(u32, u32), u32>,
    /// Source content hash for validation
    pub source_hash: u64,
}

impl SourceMap {
    /// Create new source map
    pub fn new(source_file: PathBuf) -> Self {
        SourceMap {
            source_file,
            source_ranges: Vec::new(),
            line_mapping: HashMap::new(),
            column_mapping: HashMap::new(),
            source_hash: 0,
        }
    }

    /// Add source range mapping
    pub fn add_range(
        &mut self,
        generated_line: u32,
        generated_column: u32,
        original_line: u32,
        original_column: u32,
        length: u32,
    ) {
        let range = SourceRange {
            generated_line,
            generated_column,
            original_line,
            original_column,
            length,
        };
        
        self.source_ranges.push(range);
        self.line_mapping.insert(generated_line, original_line);
        self.column_mapping.insert((generated_line, generated_column), original_column);
    }

    /// Map generated location to original
    pub fn map_to_original(&self, line: u32, column: u32) -> Option<(u32, u32)> {
        // Find the best matching source range
        let mut best_match: Option<&SourceRange> = None;
        
        for range in &self.source_ranges {
            if range.generated_line == line && 
               range.generated_column <= column && 
               column < range.generated_column + range.length {
                best_match = Some(range);
                break;
            }
        }
        
        best_match.map(|range| {
            let offset = column - range.generated_column;
            (range.original_line, range.original_column + offset)
        })
    }
}

/// Source range mapping
#[derive(Debug, Clone)]
pub struct SourceRange {
    pub generated_line: u32,
    pub generated_column: u32,
    pub original_line: u32,
    pub original_column: u32,
    pub length: u32,
}

/// Symbol metadata for enhanced debugging
#[derive(Debug, Clone)]
pub struct SymbolMetadata {
    /// Symbol type (function, variable, type, etc.)
    pub symbol_type: SymbolType,
    /// Visibility (public, private, etc.)
    pub visibility: SymbolVisibility,
    /// Symbol attributes (Gen Z slang annotations)
    pub attributes: HashMap<String, String>,
    /// Associated documentation
    pub documentation: Option<String>,
    /// Symbol tags for categorization
    pub tags: Vec<String>,
}

impl SymbolMetadata {
    /// Create new symbol metadata
    pub fn new() -> Self {
        SymbolMetadata {
            symbol_type: SymbolType::Unknown,
            visibility: SymbolVisibility::Private,
            attributes: HashMap::new(),
            documentation: None,
            tags: Vec::new(),
        }
    }

    /// Create metadata for function
    pub fn function(name: &str, gen_z_keyword: Option<&str>) -> Self {
        let mut metadata = SymbolMetadata::new();
        metadata.symbol_type = SymbolType::Function;
        
        if let Some(keyword) = gen_z_keyword {
            metadata.attributes.insert("gen_z_keyword".to_string(), keyword.to_string());
        }
        
        metadata.tags.push("function".to_string());
        metadata
    }

    /// Create metadata for variable
    pub fn variable(name: &str, var_type: &str) -> Self {
        let mut metadata = SymbolMetadata::new();
        metadata.symbol_type = SymbolType::Variable;
        metadata.attributes.insert("type".to_string(), var_type.to_string());
        
        // Map CURSED variable types to Gen Z equivalents
        let gen_z_type = match var_type {
            "i32" => "sus",  // integer
            "bool" => "facts", // boolean
            "f64" => "vibes", // float
            "String" => "tea", // string
            _ => var_type,
        };
        
        metadata.attributes.insert("gen_z_type".to_string(), gen_z_type.to_string());
        metadata.tags.push("variable".to_string());
        metadata
    }

    /// Add attribute
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}

/// Symbol types for categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Function,
    Variable,
    Type,
    Interface,
    Struct,
    Constant,
    Module,
    Unknown,
}

/// Symbol visibility levels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolVisibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Type debug information
#[derive(Debug, Clone)]
pub struct TypeDebugInfo {
    /// Type name
    pub type_name: String,
    /// Type kind (primitive, struct, interface, etc.)
    pub type_kind: TypeKind,
    /// Type size in bytes
    pub size_bytes: Option<usize>,
    /// Type alignment
    pub alignment: Option<usize>,
    /// Field information for composite types
    pub fields: Vec<FieldDebugInfo>,
    /// Generic type parameters
    pub type_parameters: Vec<String>,
}

impl TypeDebugInfo {
    /// Create new type debug info
    pub fn new(type_name: String, type_kind: TypeKind) -> Self {
        TypeDebugInfo {
            type_name,
            type_kind,
            size_bytes: None,
            alignment: None,
            fields: Vec::new(),
            type_parameters: Vec::new(),
        }
    }

    /// Add field information
    pub fn with_field(mut self, field: FieldDebugInfo) -> Self {
        self.fields.push(field);
        self
    }

    /// Add type parameter
    pub fn with_type_parameter(mut self, param: String) -> Self {
        self.type_parameters.push(param);
        self
    }
}

/// Type kinds for categorization
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Primitive,
    Struct,
    Interface,
    Array,
    Slice,
    Map,
    Channel,
    Function,
    Pointer,
    Generic,
}

/// Field debug information
#[derive(Debug, Clone)]
pub struct FieldDebugInfo {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Field offset in bytes
    pub offset: Option<usize>,
    /// Field size in bytes
    pub size: Option<usize>,
    /// Field visibility
    pub visibility: SymbolVisibility,
}

impl FieldDebugInfo {
    /// Create new field debug info
    pub fn new(name: String, field_type: String) -> Self {
        FieldDebugInfo {
            name,
            field_type,
            offset: None,
            size: None,
            visibility: SymbolVisibility::Private,
        }
    }
}

/// Scope information for variable resolution
#[derive(Debug, Clone)]
pub struct ScopeInfo {
    /// Scope type (function, block, module, etc.)
    pub scope_type: ScopeType,
    /// Scope depth (nesting level)
    pub depth: u32,
    /// Parent scope ID
    pub parent_scope: Option<u64>,
    /// Variables in this scope
    pub variables: HashMap<String, VariableInfo>,
    /// Scope start location
    pub start_location: Option<(u32, u32)>,
    /// Scope end location
    pub end_location: Option<(u32, u32)>,
}

impl ScopeInfo {
    /// Create new scope info
    pub fn new() -> Self {
        ScopeInfo {
            scope_type: ScopeType::Block,
            depth: 0,
            parent_scope: None,
            variables: HashMap::new(),
            start_location: None,
            end_location: None,
        }
    }

    /// Create function scope
    pub fn function_scope(depth: u32) -> Self {
        ScopeInfo {
            scope_type: ScopeType::Function,
            depth,
            parent_scope: None,
            variables: HashMap::new(),
            start_location: None,
            end_location: None,
        }
    }

    /// Add variable to scope
    pub fn add_variable(&mut self, name: String, var_info: VariableInfo) {
        self.variables.insert(name, var_info);
    }

    /// Check if variable exists in scope
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}

/// Scope types for categorization
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Module,
    Function,
    Block,
    Loop,
    Conditional,
    Match,
}

/// Debug information registry for centralized management
#[derive(Debug)]
pub struct DebugInfoRegistry {
    /// Enhanced debug information by location
    debug_info: RwLock<HashMap<String, EnhancedDebugInfo>>,
    /// Source maps by file
    source_maps: RwLock<HashMap<PathBuf, SourceMap>>,
    /// Symbol metadata by qualified name
    symbols: RwLock<HashMap<String, SymbolMetadata>>,
    /// Type information by type name
    types: RwLock<HashMap<String, TypeDebugInfo>>,
    /// Scope hierarchy
    scopes: RwLock<HashMap<u64, ScopeInfo>>,
    /// Next scope ID
    next_scope_id: RwLock<u64>,
}

impl DebugInfoRegistry {
    /// Create new debug info registry
    pub fn new() -> Self {
        DebugInfoRegistry {
            debug_info: RwLock::new(HashMap::new()),
            source_maps: RwLock::new(HashMap::new()),
            symbols: RwLock::new(HashMap::new()),
            types: RwLock::new(HashMap::new()),
            scopes: RwLock::new(HashMap::new()),
            next_scope_id: RwLock::new(1),
        }
    }

    /// Register debug information
    pub fn register_debug_info(&self, location_key: String, info: EnhancedDebugInfo) -> crate::error::Result<()> {
        let mut debug_info = self.debug_info.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire debug info lock".to_string()))?;
        
        debug_info.insert(location_key, info);
        Ok(())
    }

    /// Get debug information by location
    pub fn get_debug_info(&self, location_key: &str) -> crate::error::Result<Option<EnhancedDebugInfo>> {
        let debug_info = self.debug_info.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire debug info lock".to_string()))?;
        
        Ok(debug_info.get(location_key).cloned())
    }

    /// Register source map
    pub fn register_source_map(&self, file_path: PathBuf, source_map: SourceMap) -> crate::error::Result<()> {
        let mut source_maps = self.source_maps.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire source maps lock".to_string()))?;
        
        source_maps.insert(file_path, source_map);
        Ok(())
    }

    /// Get source map for file
    pub fn get_source_map(&self, file_path: &Path) -> crate::error::Result<Option<SourceMap>> {
        let source_maps = self.source_maps.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire source maps lock".to_string()))?;
        
        Ok(source_maps.get(file_path).cloned())
    }

    /// Register symbol metadata
    pub fn register_symbol(&self, qualified_name: String, metadata: SymbolMetadata) -> crate::error::Result<()> {
        let mut symbols = self.symbols.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
        
        symbols.insert(qualified_name, metadata);
        Ok(())
    }

    /// Get symbol metadata
    pub fn get_symbol(&self, qualified_name: &str) -> crate::error::Result<Option<SymbolMetadata>> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
        
        Ok(symbols.get(qualified_name).cloned())
    }

    /// Register type information
    pub fn register_type(&self, type_name: String, type_info: TypeDebugInfo) -> crate::error::Result<()> {
        let mut types = self.types.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;
        
        types.insert(type_name, type_info);
        Ok(())
    }

    /// Get type information
    pub fn get_type(&self, type_name: &str) -> crate::error::Result<Option<TypeDebugInfo>> {
        let types = self.types.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;
        
        Ok(types.get(type_name).cloned())
    }

    /// Create new scope
    pub fn create_scope(&self, scope_info: ScopeInfo) -> crate::error::Result<u64> {
        let mut next_id = self.next_scope_id.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire scope ID lock".to_string()))?;
        
        let scope_id = *next_id;
        *next_id += 1;
        
        let mut scopes = self.scopes.write()
            .map_err(|_| CursedError::Runtime("Failed to acquire scopes lock".to_string()))?;
        
        scopes.insert(scope_id, scope_info);
        Ok(scope_id)
    }

    /// Get scope information
    pub fn get_scope(&self, scope_id: u64) -> crate::error::Result<Option<ScopeInfo>> {
        let scopes = self.scopes.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire scopes lock".to_string()))?;
        
        Ok(scopes.get(&scope_id).cloned())
    }

    /// Find all symbols matching pattern
    pub fn find_symbols(&self, pattern: &str) -> Result<Vec<(String, SymbolMetadata)>, CursedError> {
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
        
        let matches: Vec<(String, SymbolMetadata)> = symbols
            .iter()
            .filter(|(name, _)| name.contains(pattern))
            .map(|(name, metadata)| (name.clone(), metadata.clone()))
            .collect();
        
        Ok(matches)
    }

    /// Get debug statistics
    pub fn get_statistics(&self) -> crate::error::Result<DebugStatistics> {
        let debug_info = self.debug_info.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire debug info lock".to_string()))?;
        let symbols = self.symbols.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire symbols lock".to_string()))?;
        let types = self.types.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire types lock".to_string()))?;
        let scopes = self.scopes.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire scopes lock".to_string()))?;
        
        Ok(DebugStatistics {
            debug_info_count: debug_info.len(),
            symbol_count: symbols.len(),
            type_count: types.len(),
            scope_count: scopes.len(),
        })
    }
}

/// Debug statistics for monitoring
#[derive(Debug, Clone)]
pub struct DebugStatistics {
    pub debug_info_count: usize,
    pub symbol_count: usize,
    pub type_count: usize,
    pub scope_count: usize,
}

impl fmt::Display for DebugStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Debug Info: {} entries, Symbols: {}, Types: {}, Scopes: {}",
            self.debug_info_count,
            self.symbol_count,
            self.type_count,
            self.scope_count
        )
    }
}


impl fmt::Display for SymbolVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolVisibility::Public => write!(f, "public"),
            SymbolVisibility::Private => write!(f, "private"),
            SymbolVisibility::Protected => write!(f, "protected"),
            SymbolVisibility::Internal => write!(f, "internal"),
        }
    }
}
