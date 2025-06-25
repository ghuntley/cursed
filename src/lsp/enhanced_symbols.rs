use crate::error::CursedError;
// Enhanced symbol support for CURSED language
// 
// Provides comprehensive symbol information including document symbols,
// workspace symbols, symbol hierarchy, and call graph generation.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::ast::{self, *};

/// Extension trait to add missing methods to &str
trait StrExt {
    fn lines(&self) -> std::str::Lines;
impl StrExt for &str {
    fn lines(&self) -> std::str::Lines {
        str::lines(self)
    }
}

/// Enhanced symbol information for CURSED language constructs
#[derive(Debug, Clone)]
pub struct CursedSymbol {
    
    // CURSED-specific fields
impl CursedSymbol {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn add_child(&mut self, child: CursedSymbol) {
        self.children.push(child);
    pub fn to_workspace_symbol(&self, uri: &Url) -> WorkspaceSymbol {
        WorkspaceSymbol {
            location: OneOf::Left(Location {
        }
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
    
    // CursedError handling
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
    StructField,         // struct field
    InterfaceMethod,     // interface method
/// Visibility levels for symbols
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
impl CursedSymbol {
    /// Convert to LSP DocumentSymbol
    pub fn to_document_symbol(&self) -> DocumentSymbol {
        #[allow(deprecated)]
        DocumentSymbol {
            children: Some(
                self.children
                    .iter()
                    .map(|child| child.to_document_symbol())
                    .collect()
        }
    }
    
    /// Convert to LSP SymbolInformation
    pub fn to_symbol_information(&self, uri: Url) -> SymbolInformation {
        #[allow(deprecated)]
        SymbolInformation {
        }
    }
    
    /// Set visibility
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    /// Set async flag
    pub fn with_async(mut self, is_async: bool) -> Self {
        self.is_async = is_async;
        self
    /// Set generic flag
    pub fn with_generic(mut self, is_generic: bool) -> Self {
        self.is_generic = is_generic;
        self
    /// Set type information
    pub fn with_type_info(mut self, type_info: String) -> Self {
        self.type_info = Some(type_info);
        self
    /// Set documentation
    pub fn with_documentation(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    /// Add reference location
    pub fn add_reference(&mut self, location: Location) {
        self.references.push(location);
    /// Add implementation location
    pub fn add_implementation(&mut self, location: Location) {
        self.implementations.push(location);
    }
}

/// Symbol hierarchy information
#[derive(Debug, Clone)]
pub struct SymbolHierarchy {
/// Call hierarchy information
#[derive(Debug, Clone)]
pub struct CallHierarchy {
/// Enhanced symbol provider for CURSED language
pub struct EnhancedSymbolProvider {
    /// Document symbols cache
    /// Workspace symbols cache
    /// Symbol references cache
    /// Symbol implementations cache
    /// Call hierarchy cache
    /// Type definitions cache
impl EnhancedSymbolProvider {
    fn convert_import_statement_to_declaration(&self, import_stmt: &ImportStatement) -> ImportDeclaration {
        ImportDeclaration {
        }
    }

    fn convert_package_statement_to_declaration(&self, package_stmt: &PackageStatement) -> PackageDeclaration {
        PackageDeclaration {
        }
    }

    fn convert_to_core_struct_field(&self, field: &ast::declarations::main::StructField) -> core_types::StructField {
        core_types::StructField {
        }
    }

    fn convert_to_core_interface_method(&self, method: &ast::declarations::main::InterfaceMethod) -> core_types::InterfaceMethod {
        core_types::InterfaceMethod {
        }
    }

    fn convert_to_core_method_declaration(&self, method: &ast::declarations::main::MethodDeclaration) -> core_types::InterfaceMethod {
        core_types::InterfaceMethod {
        }
    }

    fn convert_to_core_variable_declaration(&self, var_decl: &ast::declarations::main::VariableDeclaration) -> core_types::VariableDeclaration {
        core_types::VariableDeclaration {
        }
    }

    /// Create a new enhanced symbol provider
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Get document symbols for the given content
    #[instrument(skip(self, content))]
    pub async fn get_document_symbols(
    ) -> Result<Vec<CursedSymbol>, String> {
        debug!("Extracting document symbols");
        
        // Check cache first
        if let Some(cached_symbols) = self.document_symbols.get(uri) {
            return Ok(cached_symbols.clone());
        let mut symbols = Vec::new();
        
        // Parse the content
        let mut lexer = Lexer::new(content);
        let mut parser = match Parser::new(lexer) {
            Err(e) => {
                debug!("Failed to create parser: {:?}", e);
                return Ok(symbols);
            }
        
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
    /// Create method signature string
    fn create_method_signature(&self, name: &str, params: &[String], return_type: Option<&str>) -> String {
        let params_str = params.join(", ");
        match return_type {
        }
    }

    /// Create variable symbol
    fn create_variable_symbol(&self, name: &str, var_type: Option<&str>, range: Range, kind: SymbolKind) -> CursedSymbol {
        let mut symbol = CursedSymbol::new(
        );
        if let Some(t) = var_type {
            symbol.type_info = Some(t.to_string());
        }
        symbol
    /// Create lexical function symbol from line
    fn create_lexical_function_symbol(&self, line: &str, line_num: u32) -> Option<CursedSymbol> {
        if line.trim().starts_with("slay ") || line.contains("fn ") {
            let start = Position::new(line_num, 0);
            let end = Position::new(line_num, line.len() as u32);
            let range = Range::new(start, end);
            
            // Extract function name (simplified)
            let name = line.split_whitespace()
                .nth(1)
                .unwrap_or("unnamed")
                .split('(')
                .next()
                .unwrap_or("unnamed")
                .to_string();
                
            Some(CursedSymbol::new(
            ))
        } else {
            None
        }
    }

    /// Create lexical variable symbol from line
    fn create_lexical_variable_symbol(&self, line: &str, line_num: u32) -> Option<CursedSymbol> {
        if line.trim().starts_with("sus ") || line.trim().starts_with("facts ") {
            let start = Position::new(line_num, 0);
            let end = Position::new(line_num, line.len() as u32);
            let range = Range::new(start, end);
            
            // Extract variable name (simplified)
            let name = line.split_whitespace()
                .nth(1)
                .unwrap_or("unnamed")
                .split(|c: char| c == '=' || c == ':')
                .next()
                .unwrap_or("unnamed")
                .trim()
                .to_string();
                
            Some(CursedSymbol::new(
            ))
        } else {
            None
        }
    }

    /// Create lexical type symbol from line
    fn create_lexical_type_symbol(&self, line: &str, line_num: u32) -> Option<CursedSymbol> {
        if line.trim().starts_with("squad ") || line.trim().starts_with("collab ") {
            let start = Position::new(line_num, 0);
            let end = Position::new(line_num, line.len() as u32);
            let range = Range::new(start, end);
            
            let name = line.split_whitespace()
                .nth(1)
                .unwrap_or("unnamed")
                .split(|c: char| c == '{' || c == '(' || c == ' ')
                .next()
                .unwrap_or("unnamed")
                .trim()
                .to_string();
                
            let (kind, cursed_kind) = if line.trim().starts_with("squad ") {
                (SymbolKind::STRUCT, CursedSymbolKind::SquadStruct)
            } else {
                (SymbolKind::INTERFACE, CursedSymbolKind::CollabInterface)
                
            Some(CursedSymbol::new(
            ))
        } else {
            None
        }
    }

    /// Check if symbol matches query
    fn symbol_matches_query(&self, symbol: &CursedSymbol, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        symbol.name.to_lowercase().contains(&query_lower) ||
        symbol.detail.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower)) ||
        symbol.documentation.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
    /// Scan workspace for symbols
    async fn scan_workspace_for_symbols(&mut self, workspace_folders: &[WorkspaceFolder]) -> Vec<CursedSymbol> {
        let mut all_symbols = Vec::new();
        
        for folder in workspace_folders {
            // Simple implementation - in real world would recursively scan files
            let folder_path = folder.uri.path();
            debug!("Scanning workspace folder: {}", folder_path);
            
            // This would scan .csd files in the workspace
            // For now, return empty to prevent compilation errors
        all_symbols
    /// Build call hierarchy for symbol
    fn build_call_hierarchy(&mut self, symbol: &CursedSymbol, uri: &Url) -> CallHierarchy {
        CallHierarchy {
        }
    }

    // Helper methods for extracting range information
    fn get_function_range(&self, func: &declarations::main::FunctionStatement) -> Range {
        // Simplified - return dummy range
        Range::new(Position::new(0, 0), Position::new(1, 0))
    fn get_function_name_range(&self, func: &declarations::main::FunctionStatement) -> Range {
        // Simplified - return dummy range
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn get_struct_range(&self, struct_decl: &StructDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(1, 0))
    fn get_struct_name_range(&self, struct_decl: &StructDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn get_interface_range(&self, interface_decl: &InterfaceDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(1, 0))
    fn get_interface_name_range(&self, interface_decl: &InterfaceDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn get_variable_range(&self, var_decl: &VariableDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 20))
    fn get_variable_name_range(&self, var_decl: &VariableDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn get_constant_range(&self, const_decl: &ConstantDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 20))
    fn get_constant_name_range(&self, const_decl: &ConstantDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn get_import_range(&self, import_decl: &ImportDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 30))
    fn get_package_range(&self, package_decl: &PackageDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 20))
    fn get_package_name_range(&self, package_decl: &PackageDeclaration) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn get_parameter_range(&self, param: &Parameter) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 15))
    fn get_parameter_name_range(&self, param: &Parameter) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 8))
    fn get_field_range(&self, field: &StructField) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 15))
    fn get_field_name_range(&self, field: &StructField) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 8))
    fn get_method_range(&self, method: &InterfaceMethod) -> Range {
        Range::new(Position::new(0, 0), Position::new(1, 0))
    fn get_method_name_range(&self, method: &InterfaceMethod) -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 10))
    fn is_test_function(&self, name: &str) -> bool {
        name.starts_with("test_") || name.contains("test")
    fn create_function_symbol(&self, func: &declarations::main::FunctionStatement) -> CursedSymbol {
        let range = self.get_function_range(func);
        let selection_range = self.get_function_name_range(func);
        CursedSymbol::new(
        )
    fn create_struct_symbol(&self, struct_decl: &StructDeclaration) -> CursedSymbol {
        let range = self.get_struct_range(struct_decl);
        let selection_range = self.get_struct_name_range(struct_decl);
        CursedSymbol::new(
        )
    fn create_interface_symbol(&self, interface_decl: &InterfaceDeclaration) -> CursedSymbol {
        let range = self.get_interface_range(interface_decl);
        let selection_range = self.get_interface_name_range(interface_decl);
        CursedSymbol::new(
        )
    fn create_import_symbol(&self, import_decl: &ImportDeclaration) -> CursedSymbol {
        let range = self.get_import_range(import_decl);
        CursedSymbol::new(
        )
    fn create_package_symbol(&self, package_decl: &PackageDeclaration) -> CursedSymbol {
        let range = self.get_package_range(package_decl);
        let selection_range = self.get_package_name_range(package_decl);
        CursedSymbol::new(
        )
    fn create_parameter_symbol(&self, param: &Parameter) -> CursedSymbol {
        let range = self.get_parameter_range(param);
        let selection_range = self.get_parameter_name_range(param);
        CursedSymbol::new(
        )
    fn create_field_symbol(&self, field: &StructField) -> CursedSymbol {
        let range = self.get_field_range(self.convert_to_core_struct_field(field));
        let selection_range = self.get_field_name_range(self.convert_to_core_struct_field(field));
        CursedSymbol::new(
        )
    fn create_method_symbol(&self, method: &InterfaceMethod) -> CursedSymbol {
        let range = self.get_method_range(self.convert_to_core_interface_method(method));
        let selection_range = self.get_method_name_range(self.convert_to_core_interface_method(method));
        CursedSymbol::new(
        )
    fn extract_local_symbols(&self, block: &BlockStatement, symbols: &mut Vec<CursedSymbol>) {
        // Simplified implementation for local symbol extraction
        // In a real implementation, this would traverse the block and extract local variables
    /// Extract symbols from parsed AST
    async fn extract_symbols_from_ast(
    ) {
        for statement in &ast.statements {
            if let Some(func_decl) = statement.as_any().downcast_ref::<FunctionStatement>() {
                let symbol = self.create_function_symbol(func_decl);
                symbols.push(symbol);
            } else if let Some(struct_decl) = statement.as_any().downcast_ref::<SquadStatement>() {
                let symbol = self.create_struct_symbol(struct_decl);
                symbols.push(symbol);
            } else if let Some(interface_decl) = statement.as_any().downcast_ref::<CollabStatement>() {
                let symbol = self.create_interface_symbol(interface_decl);
                symbols.push(symbol);
            } else if let Some(var_decl) = statement.as_any().downcast_ref::<VariableStatement>() {
                // Extract information from variable statement for proper call
                let var_name = &var_decl.name;
                let var_type = var_decl.var_type.as_ref().map(|t| t.to_string()).as_deref();
                let range = Range::default(); // Use default range for now
                let symbol = self.create_variable_symbol(var_name, var_type, range, SymbolKind::VARIABLE);
                symbols.push(symbol);
            // TODO: Fix ConstantDeclaration when proper type is available
            // } else if let Some(const_decl) = statement.as_any().downcast_ref::<ConstantStatement>() {
            //     let symbol = self.create_constant_symbol(const_decl, uri).await;
            //     symbols.push(symbol);
            } else if let Some(import_decl) = statement.as_any().downcast_ref::<ImportStatement>() {
                let symbol = self.create_import_symbol(&self.convert_import_statement_to_declaration(import_decl));
                symbols.push(symbol);
            } else if let Some(package_decl) = statement.as_any().downcast_ref::<PackageStatement>() {
                let symbol = self.create_package_symbol(&self.convert_package_statement_to_declaration(package_decl));
                symbols.push(symbol);
            }
        }
    /// Create function symbol
    async fn create_function_symbol(&mut self, func_decl: &FunctionDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_function_range(func_decl);
        let selection_range = self.get_function_name_range(func_decl);
        
        let kind = if self.is_test_function(&func_decl.name.value) {
            SymbolKind::METHOD
        } else {
            SymbolKind::FUNCTION
        
        let cursed_kind = if self.is_test_function(&func_decl.name.value) {
            CursedSymbolKind::TestFunction
        } else if !func_decl.type_parameters.is_empty() {
            CursedSymbolKind::GenericFunction
        } else {
            CursedSymbolKind::SlayFunction
        
        let mut symbol = CursedSymbol::new(
        );
        
        // Add function details
        symbol.detail = Some(self.create_function_signature(func_decl));
        symbol.is_generic = func_decl.type_parameters.len() > 0;
        
        // Add parameter symbols as children
        for param in &func_decl.parameters {
            let param_symbol = self.create_parameter_symbol(param);
            symbol.add_child(param_symbol);
        // Extract local symbols from function body
        let mut local_symbols = Vec::new();
        self.extract_local_symbols(&func_decl.body, &mut local_symbols);
        for local_symbol in local_symbols {
            symbol.add_child(local_symbol);
        symbol
    /// Create struct symbol
    async fn create_struct_symbol(&mut self, struct_decl: &StructDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_struct_range(struct_decl);
        let selection_range = self.get_struct_name_range(struct_decl);
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some("struct".to_string());
        symbol.is_generic = struct_decl.type_parameters.len() > 0;
        
        // Add field symbols as children
        for field in &struct_decl.fields {
            let field_symbol = self.create_field_symbol(&self.convert_to_core_struct_field(field));
            symbol.add_child(field_symbol);
        symbol
    /// Create interface symbol
    async fn create_interface_symbol(&mut self, interface_decl: &InterfaceDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_interface_range(interface_decl);
        let selection_range = self.get_interface_name_range(interface_decl);
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some("interface".to_string());
        symbol.is_generic = interface_decl.type_parameters.len() > 0;
        
        // Add method symbols as children
        for method in &interface_decl.methods {
            let method_symbol = self.create_method_symbol(&self.convert_to_core_interface_method(method));
            symbol.add_child(method_symbol);
        symbol
    /// Create variable symbol
    async fn create_variable_symbol(&mut self, var_decl: &VariableDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_variable_range(self.convert_to_core_variable_declaration(var_decl));
        let selection_range = self.get_variable_name_range(self.convert_to_core_variable_declaration(var_decl));
        
        let mut symbol = CursedSymbol::new(
        );
        
        // Add type information if available
        if let Some(type_annotation) = &var_decl.type_annotation {
            symbol.type_info = Some(format!("{:?}", type_annotation));
            symbol.detail = Some(format!("sus {}: {}", var_decl.name.name, symbol.type_info.as_ref().unwrap()));
        } else {
            symbol.detail = Some(format!("sus {}", var_decl.name.name));
        symbol
    /// Create constant symbol
    async fn create_constant_symbol(&mut self, const_decl: &ConstantDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_constant_range(const_decl);
        let selection_range = self.get_constant_name_range(const_decl);
        
        let mut symbol = CursedSymbol::new(
        );
        
        // Add type information if available
        if let Some(type_annotation) = &const_decl.type_annotation {
            symbol.type_info = Some(format!("{:?}", type_annotation));
            symbol.detail = Some(format!("facts {}: {}", const_decl.name.name, symbol.type_info.as_ref().unwrap()));
        } else {
            symbol.detail = Some(format!("facts {}", const_decl.name.name));
        // symbol.tags = Some(vec![SymbolTag::READONLY]); // READONLY not available
        
        symbol
    /// Create import symbol
    async fn create_import_symbol(&mut self, import_decl: &ImportDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_import_range(import_decl);
        let selection_range = range; // For imports, selection range is the same
        
        let name = import_decl.path.clone();
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some(format!("import \"{}\"", name));
        
        symbol
    /// Create package symbol
    async fn create_package_symbol(&mut self, package_decl: &PackageDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_package_range(package_decl);
        let selection_range = self.get_package_name_range(package_decl);
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some(format!("package {}", package_decl.name.value));
        
        symbol
    /// Create parameter symbol
    fn create_parameter_symbol(&self, param: &Parameter) -> CursedSymbol {
        let range = self.get_parameter_range(param);
        let selection_range = self.get_parameter_name_range(param);
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some(format!("{}: {}", param.name, param.param_type));
        symbol.type_info = Some(param.param_type.clone());
        
        symbol
    /// Create field symbol
    fn create_field_symbol(&self, field: &StructField) -> CursedSymbol {
        let range = self.get_field_range(self.convert_to_core_struct_field(field));
        let selection_range = self.get_field_name_range(self.convert_to_core_struct_field(field));
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some(format!("{}: {:?}", field.name.value, field.type_annotation));
        symbol.type_info = Some(format!("{:?}", field.type_annotation));
        
        symbol
    /// Create method symbol
    fn create_method_symbol(&self, method: &InterfaceMethod) -> CursedSymbol {
        let range = self.get_method_range(self.convert_to_core_interface_method(method));
        let selection_range = self.get_method_name_range(self.convert_to_core_interface_method(method));
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some(self.create_method_signature(&method.name.value, &[], None));
        
        symbol
    /// Create basic variable symbol
    fn create_variable_symbol_basic(&self, name: &str, var_type: &str) -> CursedSymbol {
        let range = Range::new(Position::new(0, 0), Position::new(0, 0));
        let mut symbol = CursedSymbol::new(
        );
        symbol.detail = Some(format!("{}: {}", name, var_type));
        symbol
    /// Extract local symbols from block statement
    fn extract_local_symbols(&self, block: &BlockStatement, symbols: &mut Vec<CursedSymbol>) {
        for statement in &block.statements {
            if let Some(var_decl) = statement.as_any().downcast_ref::<VariableStatement>() {
                let symbol = self.create_variable_symbol_basic(&var_decl.name.value, "variable");
                symbols.push(symbol);
            }
            // TODO: Add ConstantDeclaration when available
            // if let Some(const_decl) = statement.as_any().downcast_ref::<ConstantStatement>() {
            //     let symbol = self.create_constant_symbol(const_decl, uri).await;
            //     symbols.push(symbol);
            // }
            // Add more local symbol types as needed
        }
    }
    
    /// Extract symbols using lexical analysis when parsing fails
    async fn extract_symbols_lexically(
    ) {
        let mut lexer = Lexer::new(content);
        let lines: Vec<&str> = content.lines().collect();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    // Look for function-like patterns
                    if token.literal == "slay" {
                        if let Some(symbol) = self.create_lexical_function_symbol(&token.literal, line_num) {
                            symbols.push(symbol);
                        }
                    }
                    // Look for variable patterns
                    else if token.literal == "sus" || token.literal == "facts" {
                        if let Some(symbol) = self.create_lexical_variable_symbol(&token.literal, line_num) {
                            symbols.push(symbol);
                        }
                    }
                    // Look for type patterns
                    else if token.literal == "squad" || token.literal == "collab" {
                        if let Some(symbol) = self.create_lexical_type_symbol(&token.literal, line_num) {
                            symbols.push(symbol);
                        }
                    }
                }
            }
        }
    /// Search workspace symbols
    #[instrument(skip(self))]
    pub async fn search_workspace_symbols(
    ) -> Result<Vec<WorkspaceSymbol>, String> {
        debug!("Searching workspace symbols for query: {}", query);
        
        let mut results = Vec::new();
        
        // Search through cached symbols
        for (workspace, symbols) in &self.workspace_symbols {
            for symbol in symbols {
                if self.symbol_matches_query(symbol, query) {
                    let uri = Url::parse(&format!("file://{}", workspace)).unwrap();
                    results.push(symbol.to_workspace_symbol(&uri));
                }
            }
        // If no results and we have workspace folders, scan files
        if results.len() == 0 && workspace_folders.len() > 0 {
            let additional_symbols = self.scan_workspace_for_symbols(workspace_folders).await;
            for symbol in additional_symbols {
                if self.symbol_matches_query(&symbol, query) {
                    let uri = Url::parse("file:///").unwrap(); // Default URI
                    results.push(symbol.to_workspace_symbol(&uri));
                }
            }
        Ok(results)
    /// Scan workspace for symbols
    async fn scan_workspace_for_symbols(
    ) {
        // This would scan the filesystem for .csd files and extract symbols
        // For now, return empty results
    /// Check if symbol matches query
    fn symbol_matches_query(&self, symbol_name: &str, query: &str) -> bool {
        if query.len() == 0 {
            return true;
        // Case-insensitive substring match
        symbol_name.to_lowercase().contains(&query.to_lowercase()) ||
        // Fuzzy matching (simplified)
        self.fuzzy_match(symbol_name, query)
    /// Simple fuzzy matching
    fn fuzzy_match(&self, text: &str, pattern: &str) -> bool {
        let text = text.to_lowercase();
        let pattern = pattern.to_lowercase();
        
        let mut text_chars = text.chars();
        let mut pattern_chars = pattern.chars().peekable();
        
        while let Some(pattern_char) = pattern_chars.next() {
            loop {
                match text_chars.next() {
                }
            }
        true
    /// Get call hierarchy for a symbol
    pub async fn get_call_hierarchy(
    ) -> Result<CallHierarchy, String> {
        // Check cache first
        if let Some(hierarchy) = self.call_hierarchy.get(symbol_name) {
            return Ok(hierarchy.clone());
        // Create a dummy symbol and build call hierarchy  
        let dummy_symbol = self.create_variable_symbol_basic(symbol_name, "unknown");
        let hierarchy = self.build_call_hierarchy(&dummy_symbol, uri);
        
        // Cache the result
        self.call_hierarchy.insert(symbol_name.to_string(), hierarchy.clone());
        
        Ok(hierarchy)
    /// Build call hierarchy for a symbol by name
    async fn build_call_hierarchy_by_name(
    ) -> Result<CallHierarchy, String> {
        // This would analyze the code to find incoming and outgoing calls
        // For now, return empty hierarchy
        Ok(CallHierarchy {
        })
    // Helper methods for range calculation
    
    fn get_function_range(&self, func_decl: &FunctionDeclaration) -> Range {
        // Calculate range from function start to end
        Range {
            start: Position {
            end: Position {
                character: 100, // Simplified
        }
    }
    
    fn get_function_name_range(&self, func_decl: &FunctionDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_struct_range(&self, struct_decl: &StructDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_struct_name_range(&self, struct_decl: &StructDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_interface_range(&self, interface_decl: &InterfaceDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_interface_name_range(&self, interface_decl: &InterfaceDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_variable_range(&self, var_decl: &VariableDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_variable_name_range(&self, var_decl: &VariableDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_constant_range(&self, const_decl: &ConstantDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_constant_name_range(&self, const_decl: &ConstantDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_import_range(&self, import_decl: &ImportDeclaration) -> Range {
        // Simplified implementation
        Range {
        }
    }
    
    fn get_package_range(&self, package_decl: &PackageDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_package_name_range(&self, package_decl: &PackageDeclaration) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_parameter_range(&self, param: &Parameter) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_parameter_name_range(&self, param: &Parameter) -> Range {
        self.get_parameter_range(param)
    fn get_field_range(&self, field: &StructField) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_field_name_range(&self, field: &StructField) -> Range {
        self.get_field_range(self.convert_to_core_struct_field(field))
    fn get_method_range(&self, method: &InterfaceMethod) -> Range {
        Range {
            start: Position {
            end: Position {
        }
    }
    
    fn get_method_name_range(&self, method: &InterfaceMethod) -> Range {
        self.get_method_range(self.convert_to_core_interface_method(method))
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
    fn is_test_function(&self, name: &str) -> bool {
        name.starts_with("test_") || name.ends_with("_test") || name.contains("_test_")
    // Lexical symbol creation methods
    
    fn create_lexical_function_symbol(&self, token: &Token, lines: &[&str]) -> Option<CursedSymbol> {
        // Simple lexical function symbol creation
        let range = Range {
            start: Position {
            end: Position {
        
        Some(CursedSymbol::new(
        ))
    fn create_lexical_variable_symbol(&self, token: &Token, lines: &[&str]) -> Option<CursedSymbol> {
        let range = Range {
            start: Position {
            end: Position {
        
        let (kind, cursed_kind) = if token.lexeme == "sus" {
            (SymbolKind::VARIABLE, CursedSymbolKind::SusVariable)
        } else {
            (SymbolKind::CONSTANT, CursedSymbolKind::FactsConstant)
        
        Some(CursedSymbol::new(
        ))
    fn create_lexical_type_symbol(&self, token: &Token, lines: &[&str]) -> Option<CursedSymbol> {
        let range = Range {
            start: Position {
            end: Position {
        
        let (kind, cursed_kind) = if token.lexeme == "squad" {
            (SymbolKind::STRUCT, CursedSymbolKind::SquadStruct)
        } else {
            (SymbolKind::INTERFACE, CursedSymbolKind::CollabInterface)
        
        Some(CursedSymbol::new(
        ))
    // Missing methods needed by the async functions
    
    async fn create_field_symbol(&mut self, field: &crate::ast::declarations::StructField, uri: &Url) -> CursedSymbol {
        let range = self.get_field_range(self.convert_to_core_struct_field(field));
        let selection_range = self.get_field_name_range(self.convert_to_core_struct_field(field));
        
        CursedSymbol::new(
        )
    fn get_field_range(&self, _field: &crate::ast::declarations::StructField) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_field_name_range(&self, _field: &crate::ast::declarations::StructField) -> Range {
        // Placeholder implementation  
        Range::default()
    async fn create_method_symbol(&mut self, method: &crate::ast::declarations::InterfaceMethod, uri: &Url) -> CursedSymbol {
        let range = self.get_method_range(self.convert_to_core_interface_method(method));
        let selection_range = self.get_method_name_range(self.convert_to_core_interface_method(method));
        
        let mut symbol = CursedSymbol::new(
        );
        
        symbol.detail = Some(self.create_method_signature(&method.name, &[], None));
        symbol
    fn get_method_range(&self, _method: &crate::ast::declarations::InterfaceMethod) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_method_name_range(&self, _method: &crate::ast::declarations::InterfaceMethod) -> Range {
        // Placeholder implementation
        Range::default()
    fn create_method_signature(&self, method: &crate::ast::declarations::InterfaceMethod) -> String {
        format!("{}()", method.name)
    fn get_interface_range(&self, _interface_decl: &crate::ast::declarations::InterfaceDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_interface_name_range(&self, _interface_decl: &crate::ast::declarations::InterfaceDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_variable_range(&self, _var_decl: &crate::ast::declarations::VariableDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_variable_name_range(&self, _var_decl: &crate::ast::declarations::VariableDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_constant_range(&self, _const_decl: &crate::ast::declarations::ConstantDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_constant_name_range(&self, _const_decl: &crate::ast::declarations::ConstantDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_import_range(&self, _import_decl: &crate::ast::declarations::ImportDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_package_range(&self, _package_decl: &crate::ast::declarations::PackageDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_package_name_range(&self, _package_decl: &crate::ast::declarations::PackageDeclaration) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_parameter_range(&self, _param: &crate::ast::declarations::Parameter) -> Range {
        // Placeholder implementation
        Range::default()
    fn get_parameter_name_range(&self, _param: &crate::ast::declarations::Parameter) -> Range {
        // Placeholder implementation
        Range::default()
    async fn create_variable_symbol(&mut self, var_decl: &crate::ast::declarations::VariableDeclaration, uri: &Url) -> CursedSymbol {
        let range = self.get_variable_range(self.convert_to_core_variable_declaration(var_decl));
        let selection_range = self.get_variable_name_range(self.convert_to_core_variable_declaration(var_decl));
        
        CursedSymbol::new(
        )
    fn create_lexical_function_symbol(&self, _token: &crate::lexer::Token, _lines: &[&str]) -> Option<CursedSymbol> {
        // Placeholder implementation
        None
    fn create_lexical_variable_symbol(&self, _token: &crate::lexer::Token, _lines: &[&str]) -> Option<CursedSymbol> {
        // Placeholder implementation
        None
    fn create_lexical_type_symbol(&self, _token: &crate::lexer::Token, _lines: &[&str]) -> Option<CursedSymbol> {
        // Placeholder implementation
        None
    fn symbol_matches_query(&self, symbol_name: &str, query: &str) -> bool {
        symbol_name.to_lowercase().contains(&query.to_lowercase())
    async fn scan_workspace_for_symbols(
    ) {
        // Placeholder implementation - would scan workspace files for symbols
        // For now, just return empty results
    }
}

impl Default for EnhancedSymbolProvider {
    fn convert_to_core_struct_field(&self, field: &ast::declarations::main::StructField) -> core_types::StructField {
        core_types::StructField {
        }
    }

    fn convert_to_core_interface_method(&self, method: &ast::declarations::main::InterfaceMethod) -> core_types::InterfaceMethod {
        core_types::InterfaceMethod {
        }
    }

    fn convert_to_core_method_declaration(&self, method: &ast::declarations::main::MethodDeclaration) -> core_types::InterfaceMethod {
        core_types::InterfaceMethod {
        }
    }

    fn convert_to_core_variable_declaration(&self, var_decl: &ast::declarations::main::VariableDeclaration) -> core_types::VariableDeclaration {
        core_types::VariableDeclaration {
        }
    }

    fn default() -> Self {
        Self::new()
    }
}

