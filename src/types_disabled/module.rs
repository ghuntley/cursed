// Module type for CURSED
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a CURSED module
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Module path
    pub path: Option<PathBuf>,
    /// Exported symbols
    pub exports: HashMap<String, ModuleSymbol>,
    /// Imported modules
    pub imports: HashMap<String, ModuleImport>,
    /// Module metadata
    pub metadata: ModuleMetadata,
    /// Module dependencies
    pub dependencies: Vec<String>,
    /// Whether this module is loaded
    pub loaded: bool,
}

/// Symbol exported by a module
#[derive(Debug, Clone)]
pub struct ModuleSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub visibility: Visibility,
    pub location: Option<crate::error::SourceLocation>,
}

/// Type of symbol
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Function,
    Variable,
    Type,
    Constant,
    Module,
}

/// Symbol visibility
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

/// Module import information
#[derive(Debug, Clone)]
pub struct ModuleImport {
    pub module_name: String,
    pub alias: Option<String>,
    pub symbols: Vec<String>, // Specific symbols to import
    pub wildcard: bool, // Import all symbols
}

/// Module metadata
#[derive(Debug, Clone, Default)]
pub struct ModuleMetadata {
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub created_at: Option<std::time::SystemTime>,
    pub modified_at: Option<std::time::SystemTime>,
}

impl Module {
    /// Create a new module
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            path: None,
            exports: HashMap::new(),
            imports: HashMap::new(),
            metadata: ModuleMetadata::default(),
            dependencies: Vec::new(),
            loaded: false,
        }
    }
    
    /// Create a module with path
    pub fn with_path<S: Into<String>>(name: S, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            path: Some(path),
            exports: HashMap::new(),
            imports: HashMap::new(),
            metadata: ModuleMetadata::default(),
            dependencies: Vec::new(),
            loaded: false,
        }
    }
    
    /// Add an export to the module
    pub fn add_export(&mut self, symbol: ModuleSymbol) {
        self.exports.insert(symbol.name.clone(), symbol);
    }
    
    /// Add an import to the module
    pub fn add_import(&mut self, import: ModuleImport) {
        self.imports.insert(import.module_name.clone(), import);
    }
    
    /// Check if symbol is exported
    pub fn exports_symbol(&self, name: &str) -> bool {
        self.exports.contains_key(name)
    }
    
    /// Get exported symbol
    pub fn get_export(&self, name: &str) -> Option<&ModuleSymbol> {
        self.exports.get(name)
    }
    
    /// Get all public exports
    pub fn public_exports(&self) -> Vec<&ModuleSymbol> {
        self.exports.values()
            .filter(|sym| sym.visibility == Visibility::Public)
            .collect()
    }
    
    /// Add dependency
    pub fn add_dependency<S: Into<String>>(&mut self, dep: S) {
        let dep_name = dep.into();
        if !self.dependencies.contains(&dep_name) {
            self.dependencies.push(dep_name);
        }
    }
    
    /// Check if module has dependency
    pub fn has_dependency(&self, dep: &str) -> bool {
        self.dependencies.contains(&dep.to_string())
    }
    
    /// Mark module as loaded
    pub fn mark_loaded(&mut self) {
        self.loaded = true;
    }
    
    /// Check if module is loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
    
    /// Get module file path
    pub fn file_path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
    
    /// Set module metadata
    pub fn set_metadata(&mut self, metadata: ModuleMetadata) {
        self.metadata = metadata;
    }
    
    /// Get module qualified name (includes path context)
    pub fn qualified_name(&self) -> String {
        if let Some(path) = &self.path {
            format!("{}@{}", self.name, path.display())
        } else {
            self.name.clone()
        }
    }
    
    /// Resolve symbol in this module
    pub fn resolve_symbol(&self, name: &str) -> Option<&ModuleSymbol> {
        // First check direct exports
        if let Some(symbol) = self.exports.get(name) {
            return Some(symbol);
        }
        
        // Then check imported symbols
        for import in self.imports.values() {
            if import.wildcard || import.symbols.contains(&name.to_string()) {
                // In a real implementation, we'd look up the symbol in the imported module
                // For now, we just return None as we don't have access to other modules
            }
        }
        
        None
    }
}

impl ModuleSymbol {
    /// Create a new symbol
    pub fn new<S: Into<String>>(name: S, symbol_type: SymbolType, visibility: Visibility) -> Self {
        Self {
            name: name.into(),
            symbol_type,
            visibility,
            location: None,
        }
    }
    
    /// Create a public function symbol
    pub fn public_function<S: Into<String>>(name: S) -> Self {
        Self::new(name, SymbolType::Function, Visibility::Public)
    }
    
    /// Create a private function symbol
    pub fn private_function<S: Into<String>>(name: S) -> Self {
        Self::new(name, SymbolType::Function, Visibility::Private)
    }
    
    /// Create a public variable symbol
    pub fn public_variable<S: Into<String>>(name: S) -> Self {
        Self::new(name, SymbolType::Variable, Visibility::Public)
    }
    
    /// Create a public type symbol
    pub fn public_type<S: Into<String>>(name: S) -> Self {
        Self::new(name, SymbolType::Type, Visibility::Public)
    }
    
    /// Create a public constant symbol
    pub fn public_constant<S: Into<String>>(name: S) -> Self {
        Self::new(name, SymbolType::Constant, Visibility::Public)
    }
    
    /// Set source location
    pub fn with_location(mut self, location: crate::error::SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl ModuleImport {
    /// Create a new import
    pub fn new<S: Into<String>>(module_name: S) -> Self {
        Self {
            module_name: module_name.into(),
            alias: None,
            symbols: Vec::new(),
            wildcard: false,
        }
    }
    
    /// Create import with alias
    pub fn with_alias<S: Into<String>, A: Into<String>>(module_name: S, alias: A) -> Self {
        Self {
            module_name: module_name.into(),
            alias: Some(alias.into()),
            symbols: Vec::new(),
            wildcard: false,
        }
    }
    
    /// Create import with specific symbols
    pub fn with_symbols<S: Into<String>>(module_name: S, symbols: Vec<String>) -> Self {
        Self {
            module_name: module_name.into(),
            alias: None,
            symbols,
            wildcard: false,
        }
    }
    
    /// Create wildcard import
    pub fn wildcard<S: Into<String>>(module_name: S) -> Self {
        Self {
            module_name: module_name.into(),
            alias: None,
            symbols: Vec::new(),
            wildcard: true,
        }
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new("unnamed")
    }
}
