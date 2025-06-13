//! Enhanced symbol support for CURSED language
//! 
//! Provides comprehensive symbol information including document symbols,
//! workspace symbols, symbol hierarchy, and call graph generation.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::ast::*;

/// Enhanced symbol information for CURSED language constructs
#[derive(Debug, Clone)]
pub struct CursedSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub selection_range: Range,
    pub detail: Option<String>,
    pub tags: Option<Vec<SymbolTag>>,
    pub children: Vec<CursedSymbol>,
    
    // CURSED-specific fields
    pub cursed_kind: CursedSymbolKind,
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_generic: bool,
    pub type_info: Option<String>,
    pub documentation: Option<String>,
    pub references: Vec<Location>,
    pub implementations: Vec<Location>,
}

/// CURSED-specific symbol kinds
#[derive(Debug, Clone, PartialEq)]
pub enum CursedSymbolKind {
    // Function types
    SlayFunction,        // slay function declaration
    AsyncFunction,       // async function
    TestFunction,        // test function
    GenericFunction,     // generic function
    
    // Variable types
    SusVariable,         // sus variable declaration
    FactsConstant,       // facts constant declaration
    Parameter,           // function parameter
    LocalVariable,       // local variable
    
    // Type definitions
    SquadStruct,         // squad struct definition
    CollabInterface,     // collab interface definition
    Enum,                // enumeration
    TypeAlias,           // type alias
    GenericParameter,    // generic type parameter
    
    // Control flow
    LowkeyCondition,     // lowkey if statement
    HighkeyElse,         // highkey else statement
    PeriodtSwitch,       // periodt switch statement
    BestieCase,          // bestie case statement
    FlexDefault,         // flex default case
    YoloLoop,            // yolo loop statement
    
    // Concurrency
    StanGoroutine,       // stan goroutine spawn
    ChanChannel,         // chan channel declaration
    CrushOperation,      // crush channel operation
    
    // Error handling
    SpillError,          // spill error/panic
    ErrorPropagation,    // ? error propagation
    
    // Module system
    ImportDeclaration,   // import statement
    PackageDeclaration,  // package declaration
    ModuleDefinition,    // module definition
    
    // Annotations
    Annotation,          // @annotation
    Pragma,              // #pragma
    
    // Special constructs
    MatchExpression,     // pattern matching
    WhenClause,          // when clause
    Lambda,              // lambda expression
    Closure,             // closure
}

/// Visibility levels for symbols
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Package,
    Protected,
}

impl CursedSymbol {
    /// Create a new CURSED symbol
    pub fn new(
        name: String,
        kind: SymbolKind,
        cursed_kind: CursedSymbolKind,
        range: Range,
        selection_range: Range,
    ) -> Self {
        Self {
            name,
            kind,
            range,
            selection_range,
            detail: None,
            tags: None,
            children: Vec::new(),
            cursed_kind,
            visibility: Visibility::Private,
            is_async: false,
            is_generic: false,
            type_info: None,
            documentation: None,
            references: Vec::new(),
            implementations: Vec::new(),
        }
    }
    
    /// Convert to LSP DocumentSymbol
    pub fn to_document_symbol(&self) -> DocumentSymbol {
        #[allow(deprecated)]
        DocumentSymbol {
            name: self.name.clone(),
            detail: self.detail.clone(),
            kind: self.kind,
            tags: self.tags.clone(),
            deprecated: None,
            range: self.range,
            selection_range: self.selection_range,
            children: Some(
                self.children
                    .iter()
                    .map(|child| child.to_document_symbol())
                    .collect()
            ),
        }
    }
    
    /// Convert to LSP SymbolInformation
    pub fn to_symbol_information(&self, uri: Url) -> SymbolInformation {
        #[allow(deprecated)]
        SymbolInformation {
            name: self.name.clone(),
            kind: self.kind,
            tags: self.tags.clone(),
            deprecated: None,
            location: Location::new(uri, self.range),
            container_name: None,
        }
    }
    
    /// Convert to LSP WorkspaceSymbol
    pub fn to_workspace_symbol(&self, uri: Url) -> WorkspaceSymbol {
        WorkspaceSymbol {
            name: self.name.clone(),
            kind: self.kind,
            tags: self.tags.clone(),
            location: OneOf::Left(Location::new(uri, self.range)),
            container_name: None,
            data: None,
        }
    }
    
    /// Add a child symbol
    pub fn add_child(&mut self, child: CursedSymbol) {
        self.children.push(child);
    }
    
    /// Set visibility
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }
    
    /// Set async flag
    pub fn with_async(mut self, is_async: bool) -> Self {
        self.is_async = is_async;
        self
    }
    
    /// Set generic flag
    pub fn with_generic(mut self, is_generic: bool) -> Self {
        self.is_generic = is_generic;
        self
    }
    
    /// Set type information
    pub fn with_type_info(mut self, type_info: String) -> Self {
        self.type_info = Some(type_info);
        self
    }
    
    /// Set documentation
    pub fn with_documentation(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }
    
    /// Add reference location
    pub fn add_reference(&mut self, location: Location) {
        self.references.push(location);
    }
    
    /// Add implementation location
    pub fn add_implementation(&mut self, location: Location) {
        self.implementations.push(location);
    }
}

/// Symbol hierarchy information
#[derive(Debug, Clone)]
pub struct SymbolHierarchy {
    pub symbol: CursedSymbol,
    pub parent: Option<Box<SymbolHierarchy>>,
    pub children: Vec<SymbolHierarchy>,
    pub call_hierarchy: CallHierarchy,
}

/// Call hierarchy information
#[derive(Debug, Clone)]
pub struct CallHierarchy {
    pub incoming_calls: Vec<CallHierarchyItem>,
    pub outgoing_calls: Vec<CallHierarchyItem>,
}

/// Enhanced symbol provider for CURSED language
pub struct EnhancedSymbolProvider {
    /// Document symbols cache
    document_symbols: HashMap<Url, Vec<CursedSymbol>>,
    /// Workspace symbols cache
    workspace_symbols: HashMap<String, Vec<CursedSymbol>>,
    /// Symbol references cache
    symbol_references: HashMap<String, Vec<Location>>,
    /// Symbol implementations cache
    symbol_implementations: HashMap<String, Vec<Location>>,
    /// Call hierarchy cache
    call_hierarchy: HashMap<String, CallHierarchy>,
    /// Type definitions cache
    type_definitions: HashMap<String, Vec<Location>>,
}

impl EnhancedSymbolProvider {
    /// Create a new enhanced symbol provider
    pub fn new() -> Self {
        Self {
            document_symbols: HashMap::new(),
            workspace_symbols: HashMap::new(),
            symbol_references: HashMap::new(),
            symbol_implementations: HashMap::new(),
            call_hierarchy: HashMap::new(),
            type_definitions: HashMap::new(),
        }
    }
    
    /// Get document symbols for the given content
    #[instrument(skip(self, content))]
    pub async fn get_document_symbols(
        &mut self,
        content: &str,
        uri: &Url,
    ) -> Result<Vec<CursedSymbol>, String> {
        debug!("Extracting document symbols");
        
        // Check cache first
        if let Some(cached_symbols) = self.document_symbols.get(uri) {
            return Ok(cached_symbols.clone());
        }
        
        let mut symbols = Vec::new();
        
        // Parse the content
        let mut lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        
        match parser.parse() {
            Ok(ast) => {
                // Extract symbols from AST
                self.extract_symbols_from_ast(&ast, &mut symbols, uri).await;
            }
            Err(e) => {
                debug!("Failed to parse content for symbols: {:?}", e);
                // Fall back to lexical analysis
                self.extract_symbols_lexically(content, &mut symbols, uri).await;
            }
        }
        
        // Cache the symbols
        self.document_symbols.insert(uri.clone(), symbols.clone());
        
        Ok(symbols)
    }
    
    /// Extract symbols from parsed AST
    async fn extract_symbols_from_ast(
        &mut self,
        ast: &Program,
        symbols: &mut Vec<CursedSymbol>,
        uri: &Url,
    ) {
        for statement in &ast.statements {
            match statement {
                Statement::FunctionDeclaration(func_decl) => {
                    let symbol = self.create_function_symbol(func_decl, uri).await;
                    symbols.push(symbol);
                }
                Statement::StructDeclaration(struct_decl) => {
                    let symbol = self.create_struct_symbol(struct_decl, uri).await;
                    symbols.push(symbol);
                }
                Statement::InterfaceDeclaration(interface_decl) => {
                    let symbol = self.create_interface_symbol(interface_decl, uri).await;
                    symbols.push(symbol);
                }
                Statement::VariableDeclaration(var_decl) => {
                    let symbol = self.create_variable_symbol(var_decl, uri).await;
                    symbols.push(symbol);
                }
                Statement::ConstantDeclaration(const_decl) => {
                    let symbol = self.create_constant_symbol(const_decl, uri).await;
                    symbols.push(symbol);
                }
                Statement::ImportDeclaration(import_decl) => {
                    let symbol = self.create_import_symbol(import_decl, uri).await;
                    symbols.push(symbol);
                }
                Statement::PackageDeclaration(package_decl) => {
                    let symbol = self.create_package_symbol(package_decl, uri).await;
                    symbols.push(symbol);
                }
                _ => {}
            }
        }
    }
    
    /// Create function symbol
    async fn create_function_symbol(&mut self, func_decl: &FunctionDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_function_range(func_decl);
        let selection_range = self.get_function_name_range(func_decl);
        
        let kind = if self.is_test_function(&func_decl.name.name) {
            SymbolKind::METHOD
        } else {
            SymbolKind::FUNCTION
        };
        
        let cursed_kind = if self.is_test_function(&func_decl.name.name) {
            CursedSymbolKind::TestFunction
        } else if func_decl.generic_params.is_some() {
            CursedSymbolKind::GenericFunction
        } else {
            CursedSymbolKind::SlayFunction
        };
        
        let mut symbol = CursedSymbol::new(
            func_decl.name.name.clone(),
            kind,
            cursed_kind,
            range,
            selection_range,
        );
        
        // Add function details
        symbol.detail = Some(self.create_function_signature(func_decl));
        symbol.is_generic = func_decl.generic_params.is_some();
        
        // Add parameter symbols as children
        for param in &func_decl.parameters {
            let param_symbol = self.create_parameter_symbol(param, uri).await;
            symbol.add_child(param_symbol);
        }
        
        // Extract local symbols from function body
        if let Some(body) = &func_decl.body {
            self.extract_local_symbols(body, &mut symbol, uri).await;
        }
        
        symbol
    }
    
    /// Create struct symbol
    async fn create_struct_symbol(&mut self, struct_decl: &StructDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_struct_range(struct_decl);
        let selection_range = self.get_struct_name_range(struct_decl);
        
        let mut symbol = CursedSymbol::new(
            struct_decl.name.name.clone(),
            SymbolKind::STRUCT,
            CursedSymbolKind::SquadStruct,
            range,
            selection_range,
        );
        
        symbol.detail = Some("struct".to_string());
        symbol.is_generic = struct_decl.generic_params.is_some();
        
        // Add field symbols as children
        for field in &struct_decl.fields {
            let field_symbol = self.create_field_symbol(field, uri).await;
            symbol.add_child(field_symbol);
        }
        
        symbol
    }
    
    /// Create interface symbol
    async fn create_interface_symbol(&mut self, interface_decl: &InterfaceDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_interface_range(interface_decl);
        let selection_range = self.get_interface_name_range(interface_decl);
        
        let mut symbol = CursedSymbol::new(
            interface_decl.name.name.clone(),
            SymbolKind::INTERFACE,
            CursedSymbolKind::CollabInterface,
            range,
            selection_range,
        );
        
        symbol.detail = Some("interface".to_string());
        symbol.is_generic = interface_decl.generic_params.is_some();
        
        // Add method symbols as children
        for method in &interface_decl.methods {
            let method_symbol = self.create_method_symbol(method, uri).await;
            symbol.add_child(method_symbol);
        }
        
        symbol
    }
    
    /// Create variable symbol
    async fn create_variable_symbol(&mut self, var_decl: &VariableDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_variable_range(var_decl);
        let selection_range = self.get_variable_name_range(var_decl);
        
        let mut symbol = CursedSymbol::new(
            var_decl.name.name.clone(),
            SymbolKind::VARIABLE,
            CursedSymbolKind::SusVariable,
            range,
            selection_range,
        );
        
        // Add type information if available
        if let Some(type_annotation) = &var_decl.type_annotation {
            symbol.type_info = Some(format!("{:?}", type_annotation));
            symbol.detail = Some(format!("sus {}: {}", var_decl.name.name, symbol.type_info.as_ref().unwrap()));
        } else {
            symbol.detail = Some(format!("sus {}", var_decl.name.name));
        }
        
        symbol
    }
    
    /// Create constant symbol
    async fn create_constant_symbol(&mut self, const_decl: &ConstantDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_constant_range(const_decl);
        let selection_range = self.get_constant_name_range(const_decl);
        
        let mut symbol = CursedSymbol::new(
            const_decl.name.name.clone(),
            SymbolKind::CONSTANT,
            CursedSymbolKind::FactsConstant,
            range,
            selection_range,
        );
        
        // Add type information if available
        if let Some(type_annotation) = &const_decl.type_annotation {
            symbol.type_info = Some(format!("{:?}", type_annotation));
            symbol.detail = Some(format!("facts {}: {}", const_decl.name.name, symbol.type_info.as_ref().unwrap()));
        } else {
            symbol.detail = Some(format!("facts {}", const_decl.name.name));
        }
        
        symbol.tags = Some(vec![SymbolTag::READONLY]);
        
        symbol
    }
    
    /// Create import symbol
    async fn create_import_symbol(&mut self, import_decl: &ImportDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_import_range(import_decl);
        let selection_range = range; // For imports, selection range is the same
        
        let name = import_decl.path.clone();
        
        let mut symbol = CursedSymbol::new(
            name.clone(),
            SymbolKind::MODULE,
            CursedSymbolKind::ImportDeclaration,
            range,
            selection_range,
        );
        
        symbol.detail = Some(format!("import \"{}\"", name));
        
        symbol
    }
    
    /// Create package symbol
    async fn create_package_symbol(&mut self, package_decl: &PackageDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_package_range(package_decl);
        let selection_range = self.get_package_name_range(package_decl);
        
        let mut symbol = CursedSymbol::new(
            package_decl.name.name.clone(),
            SymbolKind::PACKAGE,
            CursedSymbolKind::PackageDeclaration,
            range,
            selection_range,
        );
        
        symbol.detail = Some(format!("package {}", package_decl.name.name));
        
        symbol
    }
    
    /// Create parameter symbol
    async fn create_parameter_symbol(&mut self, param: &Parameter, uri: &Url) -> CursedSymbol {
        let range = self.get_parameter_range(param);
        let selection_range = self.get_parameter_name_range(param);
        
        let mut symbol = CursedSymbol::new(
            param.name.name.clone(),
            SymbolKind::VARIABLE,
            CursedSymbolKind::Parameter,
            range,
            selection_range,
        );
        
        symbol.detail = Some(format!("{}: {:?}", param.name.name, param.type_annotation));
        symbol.type_info = Some(format!("{:?}", param.type_annotation));
        
        symbol
    }
    
    /// Create field symbol
    async fn create_field_symbol(&mut self, field: &StructField, uri: &Url) -> CursedSymbol {
        let range = self.get_field_range(field);
        let selection_range = self.get_field_name_range(field);
        
        let mut symbol = CursedSymbol::new(
            field.name.name.clone(),
            SymbolKind::PROPERTY,
            CursedSymbolKind::SusVariable,
            range,
            selection_range,
        );
        
        symbol.detail = Some(format!("{}: {:?}", field.name.name, field.type_annotation));
        symbol.type_info = Some(format!("{:?}", field.type_annotation));
        
        symbol
    }
    
    /// Create method symbol
    async fn create_method_symbol(&mut self, method: &InterfaceMethod, uri: &Url) -> CursedSymbol {
        let range = self.get_method_range(method);
        let selection_range = self.get_method_name_range(method);
        
        let mut symbol = CursedSymbol::new(
            method.name.name.clone(),
            SymbolKind::METHOD,
            CursedSymbolKind::SlayFunction,
            range,
            selection_range,
        );
        
        symbol.detail = Some(self.create_method_signature(method));
        
        symbol
    }
    
    /// Extract local symbols from block statement
    async fn extract_local_symbols(&mut self, block: &BlockStatement, parent: &mut CursedSymbol, uri: &Url) {
        for statement in &block.statements {
            match statement {
                Statement::VariableDeclaration(var_decl) => {
                    let symbol = self.create_variable_symbol(var_decl, uri).await;
                    parent.add_child(symbol);
                }
                Statement::ConstantDeclaration(const_decl) => {
                    let symbol = self.create_constant_symbol(const_decl, uri).await;
                    parent.add_child(symbol);
                }
                // Add more local symbol types as needed
                _ => {}
            }
        }
    }
    
    /// Extract symbols using lexical analysis when parsing fails
    async fn extract_symbols_lexically(
        &mut self,
        content: &str,
        symbols: &mut Vec<CursedSymbol>,
        uri: &Url,
    ) {
        let mut lexer = Lexer::new(content);
        let lines: Vec<&str> = content.lines().collect();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    }
                    
                    // Look for function-like patterns
                    if token.lexeme == "slay" {
                        if let Some(symbol) = self.create_lexical_function_symbol(&token, &lines) {
                            symbols.push(symbol);
                        }
                    }
                    // Look for variable patterns
                    else if token.lexeme == "sus" || token.lexeme == "facts" {
                        if let Some(symbol) = self.create_lexical_variable_symbol(&token, &lines) {
                            symbols.push(symbol);
                        }
                    }
                    // Look for type patterns
                    else if token.lexeme == "squad" || token.lexeme == "collab" {
                        if let Some(symbol) = self.create_lexical_type_symbol(&token, &lines) {
                            symbols.push(symbol);
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
    
    /// Search workspace symbols
    #[instrument(skip(self))]
    pub async fn search_workspace_symbols(
        &mut self,
        query: &str,
        workspace_folders: &[WorkspaceFolder],
    ) -> Result<Vec<WorkspaceSymbol>, String> {
        debug!("Searching workspace symbols for query: {}", query);
        
        let mut results = Vec::new();
        
        // Search through cached symbols
        for (workspace, symbols) in &self.workspace_symbols {
            for symbol in symbols {
                if self.symbol_matches_query(&symbol.name, query) {
                    let uri = Url::parse(&format!("file://{}", workspace)).unwrap();
                    results.push(symbol.to_workspace_symbol(uri));
                }
            }
        }
        
        // If no results and we have workspace folders, scan files
        if results.is_empty() && !workspace_folders.is_empty() {
            self.scan_workspace_for_symbols(workspace_folders, query, &mut results).await;
        }
        
        Ok(results)
    }
    
    /// Scan workspace for symbols
    async fn scan_workspace_for_symbols(
        &mut self,
        workspace_folders: &[WorkspaceFolder],
        query: &str,
        results: &mut Vec<WorkspaceSymbol>,
    ) {
        // This would scan the filesystem for .csd files and extract symbols
        // For now, return empty results
    }
    
    /// Check if symbol matches query
    fn symbol_matches_query(&self, symbol_name: &str, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        
        // Case-insensitive substring match
        symbol_name.to_lowercase().contains(&query.to_lowercase()) ||
        // Fuzzy matching (simplified)
        self.fuzzy_match(symbol_name, query)
    }
    
    /// Simple fuzzy matching
    fn fuzzy_match(&self, text: &str, pattern: &str) -> bool {
        let text = text.to_lowercase();
        let pattern = pattern.to_lowercase();
        
        let mut text_chars = text.chars();
        let mut pattern_chars = pattern.chars().peekable();
        
        while let Some(pattern_char) = pattern_chars.next() {
            loop {
                match text_chars.next() {
                    Some(text_char) if text_char == pattern_char => break,
                    Some(_) => continue,
                    None => return false,
                }
            }
        }
        
        true
    }
    
    /// Get call hierarchy for a symbol
    pub async fn get_call_hierarchy(
        &mut self,
        symbol_name: &str,
        uri: &Url,
        position: Position,
    ) -> Result<CallHierarchy, String> {
        // Check cache first
        if let Some(hierarchy) = self.call_hierarchy.get(symbol_name) {
            return Ok(hierarchy.clone());
        }
        
        // Build call hierarchy
        let hierarchy = self.build_call_hierarchy(symbol_name, uri, position).await?;
        
        // Cache the result
        self.call_hierarchy.insert(symbol_name.to_string(), hierarchy.clone());
        
        Ok(hierarchy)
    }
    
    /// Build call hierarchy for a symbol
    async fn build_call_hierarchy(
        &mut self,
        symbol_name: &str,
        uri: &Url,
        position: Position,
    ) -> Result<CallHierarchy, String> {
        // This would analyze the code to find incoming and outgoing calls
        // For now, return empty hierarchy
        Ok(CallHierarchy {
            incoming_calls: Vec::new(),
            outgoing_calls: Vec::new(),
        })
    }
    
    // Helper methods for range calculation
    
    fn get_function_range(&self, func_decl: &FunctionDeclaration) -> Range {
        // Calculate range from function start to end
        Range {
            start: Position {
                line: (func_decl.name.line - 1) as u32,
                character: 0,
            },
            end: Position {
                line: (func_decl.name.line - 1) as u32,
                character: 100, // Simplified
            },
        }
    }
    
    fn get_function_name_range(&self, func_decl: &FunctionDeclaration) -> Range {
        Range {
            start: Position {
                line: (func_decl.name.line - 1) as u32,
                character: func_decl.name.column as u32,
            },
            end: Position {
                line: (func_decl.name.line - 1) as u32,
                character: (func_decl.name.column + func_decl.name.name.len()) as u32,
            },
        }
    }
    
    fn get_struct_range(&self, struct_decl: &StructDeclaration) -> Range {
        Range {
            start: Position {
                line: (struct_decl.name.line - 1) as u32,
                character: 0,
            },
            end: Position {
                line: (struct_decl.name.line - 1) as u32,
                character: 100,
            },
        }
    }
    
    fn get_struct_name_range(&self, struct_decl: &StructDeclaration) -> Range {
        Range {
            start: Position {
                line: (struct_decl.name.line - 1) as u32,
                character: struct_decl.name.column as u32,
            },
            end: Position {
                line: (struct_decl.name.line - 1) as u32,
                character: (struct_decl.name.column + struct_decl.name.name.len()) as u32,
            },
        }
    }
    
    fn get_interface_range(&self, interface_decl: &InterfaceDeclaration) -> Range {
        Range {
            start: Position {
                line: (interface_decl.name.line - 1) as u32,
                character: 0,
            },
            end: Position {
                line: (interface_decl.name.line - 1) as u32,
                character: 100,
            },
        }
    }
    
    fn get_interface_name_range(&self, interface_decl: &InterfaceDeclaration) -> Range {
        Range {
            start: Position {
                line: (interface_decl.name.line - 1) as u32,
                character: interface_decl.name.column as u32,
            },
            end: Position {
                line: (interface_decl.name.line - 1) as u32,
                character: (interface_decl.name.column + interface_decl.name.name.len()) as u32,
            },
        }
    }
    
    fn get_variable_range(&self, var_decl: &VariableDeclaration) -> Range {
        Range {
            start: Position {
                line: (var_decl.name.line - 1) as u32,
                character: 0,
            },
            end: Position {
                line: (var_decl.name.line - 1) as u32,
                character: 100,
            },
        }
    }
    
    fn get_variable_name_range(&self, var_decl: &VariableDeclaration) -> Range {
        Range {
            start: Position {
                line: (var_decl.name.line - 1) as u32,
                character: var_decl.name.column as u32,
            },
            end: Position {
                line: (var_decl.name.line - 1) as u32,
                character: (var_decl.name.column + var_decl.name.name.len()) as u32,
            },
        }
    }
    
    fn get_constant_range(&self, const_decl: &ConstantDeclaration) -> Range {
        Range {
            start: Position {
                line: (const_decl.name.line - 1) as u32,
                character: 0,
            },
            end: Position {
                line: (const_decl.name.line - 1) as u32,
                character: 100,
            },
        }
    }
    
    fn get_constant_name_range(&self, const_decl: &ConstantDeclaration) -> Range {
        Range {
            start: Position {
                line: (const_decl.name.line - 1) as u32,
                character: const_decl.name.column as u32,
            },
            end: Position {
                line: (const_decl.name.line - 1) as u32,
                character: (const_decl.name.column + const_decl.name.name.len()) as u32,
            },
        }
    }
    
    fn get_import_range(&self, import_decl: &ImportDeclaration) -> Range {
        // Simplified implementation
        Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 100 },
        }
    }
    
    fn get_package_range(&self, package_decl: &PackageDeclaration) -> Range {
        Range {
            start: Position {
                line: (package_decl.name.line - 1) as u32,
                character: 0,
            },
            end: Position {
                line: (package_decl.name.line - 1) as u32,
                character: 100,
            },
        }
    }
    
    fn get_package_name_range(&self, package_decl: &PackageDeclaration) -> Range {
        Range {
            start: Position {
                line: (package_decl.name.line - 1) as u32,
                character: package_decl.name.column as u32,
            },
            end: Position {
                line: (package_decl.name.line - 1) as u32,
                character: (package_decl.name.column + package_decl.name.name.len()) as u32,
            },
        }
    }
    
    fn get_parameter_range(&self, param: &Parameter) -> Range {
        Range {
            start: Position {
                line: (param.name.line - 1) as u32,
                character: param.name.column as u32,
            },
            end: Position {
                line: (param.name.line - 1) as u32,
                character: (param.name.column + param.name.name.len()) as u32,
            },
        }
    }
    
    fn get_parameter_name_range(&self, param: &Parameter) -> Range {
        self.get_parameter_range(param)
    }
    
    fn get_field_range(&self, field: &StructField) -> Range {
        Range {
            start: Position {
                line: (field.name.line - 1) as u32,
                character: field.name.column as u32,
            },
            end: Position {
                line: (field.name.line - 1) as u32,
                character: (field.name.column + field.name.name.len()) as u32,
            },
        }
    }
    
    fn get_field_name_range(&self, field: &StructField) -> Range {
        self.get_field_range(field)
    }
    
    fn get_method_range(&self, method: &InterfaceMethod) -> Range {
        Range {
            start: Position {
                line: (method.name.line - 1) as u32,
                character: method.name.column as u32,
            },
            end: Position {
                line: (method.name.line - 1) as u32,
                character: (method.name.column + method.name.name.len()) as u32,
            },
        }
    }
    
    fn get_method_name_range(&self, method: &InterfaceMethod) -> Range {
        self.get_method_range(method)
    }
    
    // Helper methods for signature creation
    
    fn create_function_signature(&self, func_decl: &FunctionDeclaration) -> String {
        let params: Vec<String> = func_decl
            .parameters
            .iter()
            .map(|p| format!("{}: {:?}", p.name.name, p.type_annotation))
            .collect();
        
        let return_type = func_decl
            .return_type
            .as_ref()
            .map(|t| format!(" -> {:?}", t))
            .unwrap_or_default();
        
        format!("slay {}({}){}", func_decl.name.name, params.join(", "), return_type)
    }
    
    fn create_method_signature(&self, method: &InterfaceMethod) -> String {
        let params: Vec<String> = method
            .parameters
            .iter()
            .map(|p| format!("{}: {:?}", p.name.name, p.type_annotation))
            .collect();
        
        let return_type = method
            .return_type
            .as_ref()
            .map(|t| format!(" -> {:?}", t))
            .unwrap_or_default();
        
        format!("{}({}){}", method.name.name, params.join(", "), return_type)
    }
    
    fn is_test_function(&self, name: &str) -> bool {
        name.starts_with("test_") || name.ends_with("_test") || name.contains("_test_")
    }
    
    // Lexical symbol creation methods
    
    fn create_lexical_function_symbol(&self, token: &Token, lines: &[&str]) -> Option<CursedSymbol> {
        // Simple lexical function symbol creation
        let range = Range {
            start: Position {
                line: (token.line - 1) as u32,
                character: token.column as u32,
            },
            end: Position {
                line: (token.line - 1) as u32,
                character: (token.column + token.lexeme.len()) as u32,
            },
        };
        
        Some(CursedSymbol::new(
            "function".to_string(),
            SymbolKind::FUNCTION,
            CursedSymbolKind::SlayFunction,
            range,
            range,
        ))
    }
    
    fn create_lexical_variable_symbol(&self, token: &Token, lines: &[&str]) -> Option<CursedSymbol> {
        let range = Range {
            start: Position {
                line: (token.line - 1) as u32,
                character: token.column as u32,
            },
            end: Position {
                line: (token.line - 1) as u32,
                character: (token.column + token.lexeme.len()) as u32,
            },
        };
        
        let (kind, cursed_kind) = if token.lexeme == "sus" {
            (SymbolKind::VARIABLE, CursedSymbolKind::SusVariable)
        } else {
            (SymbolKind::CONSTANT, CursedSymbolKind::FactsConstant)
        };
        
        Some(CursedSymbol::new(
            "variable".to_string(),
            kind,
            cursed_kind,
            range,
            range,
        ))
    }
    
    fn create_lexical_type_symbol(&self, token: &Token, lines: &[&str]) -> Option<CursedSymbol> {
        let range = Range {
            start: Position {
                line: (token.line - 1) as u32,
                character: token.column as u32,
            },
            end: Position {
                line: (token.line - 1) as u32,
                character: (token.column + token.lexeme.len()) as u32,
            },
        };
        
        let (kind, cursed_kind) = if token.lexeme == "squad" {
            (SymbolKind::STRUCT, CursedSymbolKind::SquadStruct)
        } else {
            (SymbolKind::INTERFACE, CursedSymbolKind::CollabInterface)
        };
        
        Some(CursedSymbol::new(
            "type".to_string(),
            kind,
            cursed_kind,
            range,
            range,
        ))
    }
}

impl Default for EnhancedSymbolProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_document_symbols_extraction() {
        let mut provider = EnhancedSymbolProvider::new();
        let content = r#"
            package test_package
            
            import "stdlib::io"
            
            slay greet(name: string) -> string {
                sus greeting = "Hello, " + name;
                vibez greeting;
            }
            
            squad Person {
                name: string,
                age: i32,
            }
            
            collab Greeter {
                greet(name: string) -> string;
            }
            
            facts PI = 3.14159;
        "#;
        
        let uri = Url::parse("file:///test.csd").unwrap();
        let symbols = provider.get_document_symbols(content, &uri).await.unwrap();
        
        assert!(!symbols.is_empty());
        
        // Should have package, import, function, struct, interface, and constant symbols
        let symbol_kinds: Vec<_> = symbols.iter().map(|s| s.kind).collect();
        assert!(symbol_kinds.contains(&SymbolKind::PACKAGE));
        assert!(symbol_kinds.contains(&SymbolKind::MODULE));
        assert!(symbol_kinds.contains(&SymbolKind::FUNCTION));
        assert!(symbol_kinds.contains(&SymbolKind::STRUCT));
        assert!(symbol_kinds.contains(&SymbolKind::INTERFACE));
        assert!(symbol_kinds.contains(&SymbolKind::CONSTANT));
    }
    
    #[tokio::test]
    async fn test_workspace_symbol_search() {
        let mut provider = EnhancedSymbolProvider::new();
        let workspace_folders = vec![];
        
        // Add some test symbols to cache
        let test_symbols = vec![
            CursedSymbol::new(
                "greet_user".to_string(),
                SymbolKind::FUNCTION,
                CursedSymbolKind::SlayFunction,
                Range::default(),
                Range::default(),
            ),
            CursedSymbol::new(
                "User".to_string(),
                SymbolKind::STRUCT,
                CursedSymbolKind::SquadStruct,
                Range::default(),
                Range::default(),
            ),
        ];
        
        provider.workspace_symbols.insert("test".to_string(), test_symbols);
        
        // Search for "user"
        let results = provider.search_workspace_symbols("user", &workspace_folders).await.unwrap();
        
        assert!(!results.is_empty());
        assert!(results.iter().any(|s| s.name.contains("user") || s.name.contains("User")));
    }
    
    #[test]
    fn test_fuzzy_matching() {
        let provider = EnhancedSymbolProvider::new();
        
        assert!(provider.fuzzy_match("greet_user", "gu"));
        assert!(provider.fuzzy_match("UserProfile", "up"));
        assert!(provider.fuzzy_match("calculate_total", "ct"));
        assert!(!provider.fuzzy_match("greet", "xyz"));
    }
    
    #[test]
    fn test_symbol_conversion() {
        let symbol = CursedSymbol::new(
            "test_function".to_string(),
            SymbolKind::FUNCTION,
            CursedSymbolKind::TestFunction,
            Range::default(),
            Range::default(),
        );
        
        let uri = Url::parse("file:///test.csd").unwrap();
        
        let doc_symbol = symbol.to_document_symbol();
        assert_eq!(doc_symbol.name, "test_function");
        assert_eq!(doc_symbol.kind, SymbolKind::FUNCTION);
        
        let workspace_symbol = symbol.to_workspace_symbol(uri.clone());
        assert_eq!(workspace_symbol.name, "test_function");
        assert_eq!(workspace_symbol.kind, SymbolKind::FUNCTION);
        
        let symbol_info = symbol.to_symbol_information(uri);
        assert_eq!(symbol_info.name, "test_function");
        assert_eq!(symbol_info.kind, SymbolKind::FUNCTION);
    }
}
