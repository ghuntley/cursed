//! Symbol table management for resolved packages

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, instrument};

use crate::ast::base::Program;
use crate::ast::traits::{Expression, Statement};

/// Represents different kinds of symbols that can be exported from a package
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    /// A function declaration
    Function {
        /// Function name
        name: String,
        /// Parameter types (simplified as strings for now)
        parameters: Vec<String>,
        /// Return type (simplified as string for now)
        return_type: Option<String>,
        /// Whether the function is public
        is_public: bool,
    },
    /// A type definition (struct, interface, etc.)
    Type {
        /// Type name
        name: String,
        /// Type category (struct, interface, enum, etc.)
        kind: String,
        /// Whether the type is public
        is_public: bool,
    },
    /// A variable or constant
    Variable {
        /// Variable name
        name: String,
        /// Variable type (simplified as string for now)
        var_type: String,
        /// Whether it's a constant
        is_constant: bool,
        /// Whether the variable is public
        is_public: bool,
    },
    /// A package import
    Import {
        /// Package name being imported
        package_name: String,
        /// Alias name (if any)
        alias: Option<String>,
    },
}

impl SymbolKind {
    /// Get the symbol name
    pub fn name(&self) -> &str {
        match self {
            SymbolKind::Function { name, .. } => name,
            SymbolKind::Type { name, .. } => name,
            SymbolKind::Variable { name, .. } => name,
            SymbolKind::Import { package_name, alias } => {
                alias.as_ref().unwrap_or(package_name)
            }
        }
    }

    /// Check if the symbol is public (exported)
    pub fn is_public(&self) -> bool {
        match self {
            SymbolKind::Function { is_public, .. } => *is_public,
            SymbolKind::Type { is_public, .. } => *is_public,
            SymbolKind::Variable { is_public, .. } => *is_public,
            SymbolKind::Import { .. } => false, // Imports are not "public" in the sense of being exported
        }
    }

    /// Get a string representation of the symbol type
    pub fn symbol_type(&self) -> &'static str {
        match self {
            SymbolKind::Function { .. } => "function",
            SymbolKind::Type { .. } => "type",
            SymbolKind::Variable { .. } => "variable",
            SymbolKind::Import { .. } => "import",
        }
    }
}

/// Represents a symbol in a package
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The kind of symbol
    pub kind: SymbolKind,
    /// Source location information (line, column) - simplified for now
    pub location: Option<(usize, usize)>,
    /// Documentation comment associated with the symbol
    pub documentation: Option<String>,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(kind: SymbolKind) -> Self {
        Self {
            kind,
            location: None,
            documentation: None,
        }
    }

    /// Create a new symbol with location
    pub fn with_location(kind: SymbolKind, line: usize, column: usize) -> Self {
        Self {
            kind,
            location: Some((line, column)),
            documentation: None,
        }
    }

    /// Create a new symbol with documentation
    pub fn with_documentation(kind: SymbolKind, documentation: String) -> Self {
        Self {
            kind,
            location: None,
            documentation: Some(documentation),
        }
    }

    /// Get the symbol name
    pub fn name(&self) -> &str {
        self.kind.name()
    }

    /// Check if the symbol is public
    pub fn is_public(&self) -> bool {
        self.kind.is_public()
    }

    /// Get the symbol type
    pub fn symbol_type(&self) -> &'static str {
        self.kind.symbol_type()
    }

    /// Check if the symbol is exported (alias for is_public)
    pub fn is_exported(&self) -> bool {
        self.is_public()
    }
}

/// Symbol table for a package
#[derive(Debug, Clone)]
pub struct PackageSymbolTable {
    /// Map of symbol names to symbols
    symbols: HashMap<String, Symbol>,
    /// Package name this symbol table belongs to
    package_name: String,
}

impl PackageSymbolTable {
    /// Create a new empty symbol table
    pub fn new(package_name: String) -> Self {
        Self {
            symbols: HashMap::new(),
            package_name,
        }
    }

    /// Create a symbol table from a parsed program
    #[instrument(skip(program), fields(package_name = %package_name))]
    pub fn from_program_with_name(program: &Program, package_name: String) -> Self {
        let mut table = Self::new(package_name);
        table.extract_symbols_from_program(program);
        table
    }

    /// Create a symbol table from a parsed program (using default package name)
    pub fn from_program(program: &Program) -> Self {
        // Try to extract package name from the program
        let package_name = Self::extract_package_name(program)
            .unwrap_or_else(|| "main".to_string());
        Self::from_program_with_name(program, package_name)
    }

    /// Extract package name from the program
    fn extract_package_name(program: &Program) -> Option<String> {
        for stmt in &program.statements {
            let stmt_str = stmt.string();
            if stmt_str.starts_with("vibe ") {
                if let Some(package_decl) = stmt_str.strip_prefix("vibe ") {
                    if let Some(clean_name) = package_decl.strip_suffix(";") {
                        return Some(clean_name.trim().to_string());
                    }
                }
            }
        }
        None
    }

    /// Extract symbols from a program AST
    fn extract_symbols_from_program(&mut self, program: &Program) {
        debug!("Extracting symbols from program for package: {}", self.package_name);
        
        for stmt in &program.statements {
            self.extract_symbols_from_statement(stmt.as_ref());
        }
        
        debug!("Extracted {} symbols for package {}", self.symbols.len(), self.package_name);
    }

    /// Extract symbols from a statement
    fn extract_symbols_from_statement(&mut self, stmt: &dyn Statement) {
        let stmt_str = stmt.string();
        
        // This is a simplified symbol extraction based on string patterns
        // In a real implementation, we would have proper AST node types
        
        // Function declarations
        if stmt_str.contains("slay ") {
            if let Some(symbol) = self.extract_function_symbol(&stmt_str) {
                self.add_symbol(symbol);
            }
        }
        
        // Type declarations (interfaces, structs, etc.)
        else if stmt_str.contains("lowkey ") || stmt_str.contains("sus ") {
            if let Some(symbol) = self.extract_type_symbol(&stmt_str) {
                self.add_symbol(symbol);
            }
        }
        
        // Variable declarations
        else if stmt_str.contains("nocap ") || stmt_str.contains("cap ") {
            if let Some(symbol) = self.extract_variable_symbol(&stmt_str) {
                self.add_symbol(symbol);
            }
        }
        
        // Import statements
        else if stmt_str.contains("uses ") || stmt_str.contains("import ") {
            if let Some(symbol) = self.extract_import_symbol(&stmt_str) {
                self.add_symbol(symbol);
            }
        }
    }

    /// Extract function symbol from statement string
    fn extract_function_symbol(&self, stmt_str: &str) -> Option<Symbol> {
        // Simple pattern matching for function declarations
        // Example: "slay main() { ... }" or "slay publicFunc(x int) int { ... }"
        
        if let Some(start) = stmt_str.find("slay ") {
            let remaining = &stmt_str[start + 5..]; // Skip "slay "
            
            if let Some(paren_pos) = remaining.find('(') {
                let func_name = remaining[..paren_pos].trim().to_string();
                
                // For now, we'll assume all functions are public
                // In a real implementation, we'd check for visibility modifiers
                let is_public = true;
                
                // Extract parameters (simplified)
                let parameters = Vec::new(); // TODO: Parse actual parameters
                let return_type = None; // TODO: Parse return type
                
                let kind = SymbolKind::Function {
                    name: func_name.clone(),
                    parameters,
                    return_type,
                    is_public,
                };
                
                return Some(Symbol::new(kind));
            }
        }
        
        None
    }

    /// Extract type symbol from statement string
    fn extract_type_symbol(&self, stmt_str: &str) -> Option<Symbol> {
        // Simple pattern matching for type declarations
        // Example: "lowkey User sus { ... }" or "lowkey Config struct { ... }"
        
        if let Some(start) = stmt_str.find("lowkey ") {
            let remaining = &stmt_str[start + 7..]; // Skip "lowkey "
            
            if let Some(space_pos) = remaining.find(' ') {
                let type_name = remaining[..space_pos].trim().to_string();
                let after_name = &remaining[space_pos + 1..];
                
                let type_kind = if after_name.starts_with("sus") {
                    "interface"
                } else if after_name.starts_with("struct") {
                    "struct"
                } else {
                    "type"
                };
                
                let kind = SymbolKind::Type {
                    name: type_name,
                    kind: type_kind.to_string(),
                    is_public: true, // Assume public for now
                };
                
                return Some(Symbol::new(kind));
            }
        }
        
        None
    }

    /// Extract variable symbol from statement string
    fn extract_variable_symbol(&self, stmt_str: &str) -> Option<Symbol> {
        // Simple pattern matching for variable declarations
        // Example: "nocap x = 42" or "cap PI = 3.14"
        
        let (keyword, is_constant) = if stmt_str.contains("cap ") {
            ("cap ", true)
        } else if stmt_str.contains("nocap ") {
            ("nocap ", false)
        } else {
            return None;
        };
        
        if let Some(start) = stmt_str.find(keyword) {
            let remaining = &stmt_str[start + keyword.len()..];
            
            if let Some(eq_pos) = remaining.find('=') {
                let var_name = remaining[..eq_pos].trim().to_string();
                
                let kind = SymbolKind::Variable {
                    name: var_name,
                    var_type: "auto".to_string(), // TODO: Infer or parse actual type
                    is_constant,
                    is_public: true, // Assume public for now
                };
                
                return Some(Symbol::new(kind));
            }
        }
        
        None
    }

    /// Extract import symbol from statement string
    fn extract_import_symbol(&self, stmt_str: &str) -> Option<Symbol> {
        // Simple pattern matching for import statements
        // Example: "uses \"vibez\"" or "import \"utils\" as util"
        
        if let Some(start) = stmt_str.find("uses ") {
            let remaining = &stmt_str[start + 5..];
            
            if let Some(quote_start) = remaining.find('"') {
                if let Some(quote_end) = remaining[quote_start + 1..].find('"') {
                    let package_name = remaining[quote_start + 1..quote_start + 1 + quote_end].to_string();
                    
                    let kind = SymbolKind::Import {
                        package_name,
                        alias: None, // TODO: Parse alias
                    };
                    
                    return Some(Symbol::new(kind));
                }
            }
        }
        
        None
    }

    /// Add a symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) {
        let name = symbol.name().to_string();
        self.symbols.insert(name, symbol);
    }

    /// Get a symbol by name
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Get all symbols
    pub fn all_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values().collect()
    }

    /// Get all exported (public) symbols
    pub fn exported_symbols(&self) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|symbol| symbol.is_public())
            .collect()
    }

    /// Get symbols by type
    pub fn symbols_by_type(&self, symbol_type: &str) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|symbol| symbol.symbol_type() == symbol_type)
            .collect()
    }

    /// Get all function symbols
    pub fn functions(&self) -> Vec<&Symbol> {
        self.symbols_by_type("function")
    }

    /// Get all type symbols
    pub fn types(&self) -> Vec<&Symbol> {
        self.symbols_by_type("type")
    }

    /// Get all variable symbols
    pub fn variables(&self) -> Vec<&Symbol> {
        self.symbols_by_type("variable")
    }

    /// Get all import symbols
    pub fn imports(&self) -> Vec<&Symbol> {
        self.symbols_by_type("import")
    }

    /// Check if a symbol exists
    pub fn has_symbol(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Get the package name
    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    /// Get the number of symbols
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Check if the symbol table is empty
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Clear all symbols
    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    /// Get symbol names
    pub fn symbol_names(&self) -> Vec<String> {
        self.symbols.keys().cloned().collect()
    }

    /// Get all constant symbols
    pub fn constants(&self) -> Vec<&Symbol> {
        self.symbols_by_type("variable")
            .into_iter()
            .filter(|s| matches!(s.kind, SymbolKind::Variable { is_constant: true, .. }))
            .collect()
    }

    /// Add a dependency (currently a no-op, dependencies tracked by PackageResolver)
    pub fn add_dependency(&mut self, _package_name: &str) {
        // Dependencies are tracked by the PackageResolver's DependencyTracker
        // This is just a placeholder to satisfy the interface
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::base::Program;

    #[test]
    fn test_symbol_creation() {
        let kind = SymbolKind::Function {
            name: "test_func".to_string(),
            parameters: vec!["int".to_string()],
            return_type: Some("string".to_string()),
            is_public: true,
        };
        
        let symbol = Symbol::new(kind);
        assert_eq!(symbol.name(), "test_func");
        assert!(symbol.is_public());
        assert_eq!(symbol.symbol_type(), "function");
    }

    #[test]
    fn test_symbol_table_creation() {
        let table = PackageSymbolTable::new("test_package".to_string());
        assert_eq!(table.package_name(), "test_package");
        assert!(table.is_empty());
    }

    #[test]
    fn test_symbol_table_operations() {
        let mut table = PackageSymbolTable::new("test".to_string());
        
        let func_kind = SymbolKind::Function {
            name: "test_func".to_string(),
            parameters: vec![],
            return_type: None,
            is_public: true,
        };
        
        let symbol = Symbol::new(func_kind);
        table.add_symbol(symbol);
        
        assert_eq!(table.len(), 1);
        assert!(table.has_symbol("test_func"));
        assert!(table.get_symbol("test_func").is_some());
        
        let functions = table.functions();
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn test_exported_symbols() {
        let mut table = PackageSymbolTable::new("test".to_string());
        
        // Add public function
        let public_func = Symbol::new(SymbolKind::Function {
            name: "public_func".to_string(),
            parameters: vec![],
            return_type: None,
            is_public: true,
        });
        
        // Add private variable
        let private_var = Symbol::new(SymbolKind::Variable {
            name: "private_var".to_string(),
            var_type: "int".to_string(),
            is_constant: false,
            is_public: false,
        });
        
        table.add_symbol(public_func);
        table.add_symbol(private_var);
        
        let exported = table.exported_symbols();
        assert_eq!(exported.len(), 1);
        assert_eq!(exported[0].name(), "public_func");
    }
}
