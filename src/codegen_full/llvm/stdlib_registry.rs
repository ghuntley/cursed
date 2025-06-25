/// Standard Library Function Registry for LLVM Code Generation
/// 
/// This module provides a comprehensive registry of all CURSED stdlib functions
/// with their LLVM type information, GC requirements, and metadata for proper
/// code generation and runtime linking.

use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, FunctionType, BasicType, BasicMetadataTypeEnum};
use inkwell::values::FunctionValue;
use inkwell::AddressSpace;

/// Function metadata for stdlib functions
#[derive(Debug, Clone)]
pub struct StdlibFunction {
/// Comprehensive stdlib function registry
pub struct StdlibRegistry {
/// LLVM integration for stdlib functions
pub struct StdlibLlvmIntegration<'ctx> {
impl StdlibRegistry {
    /// Create a new stdlib registry with all packages
    pub fn new() -> Self {
        let mut registry = Self {
        
        // Register all stdlib packages
        registry.register_core_functions();
        registry.register_vibez_functions();
        registry.register_mathz_functions();
        registry.register_stringz_functions();
        registry.register_dropz_functions();
        registry.register_concurrenz_functions();
        registry.register_web_vibez_functions(); // New HTTP package
        registry.register_sql_vibes_functions(); // New database package
        registry.register_database_packages(); // Additional database functions
        registry.register_json_tea_functions();
        registry.register_regex_vibez_functions();
        registry.register_cryptz_functions();
        registry.register_reflectz_functions();
        registry.register_rizztemplate_functions();
        registry.register_htmlrizzler_functions();
        registry.register_chadlogging_functions();
        registry.register_char_functions();
        registry.register_vibe_life_functions();
        registry.register_timez_functions();
        
        registry
    /// Register core built-in functions
    fn register_core_functions(&mut self) {
        let core_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("core", core_functions);
    /// Register web_vibez HTTP functions - COMPREHENSIVE SET
    fn register_web_vibez_functions(&mut self) {
        let web_vibez_functions = vec![
            // HTTP Server Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // HTTP Client Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Request Handling Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Response Writing Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Utility Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Cookie Functions
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("web_vibez", web_vibez_functions);
    /// Register vibez I/O functions
    fn register_vibez_functions(&mut self) {
        let vibez_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("vibez", vibez_functions);
    /// Register mathematical functions
    fn register_mathz_functions(&mut self) {
        let mathz_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("mathz", mathz_functions);
    /// Register string manipulation functions
    fn register_stringz_functions(&mut self) {
        let stringz_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("stringz", stringz_functions);
    /// Register remaining packages with placeholder functions
    fn register_dropz_functions(&mut self) {
        let dropz_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("dropz", dropz_functions);
    fn register_concurrenz_functions(&mut self) {
        let concurrenz_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("concurrenz", concurrenz_functions);
    fn register_json_tea_functions(&mut self) {
        let json_tea_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("json_tea", json_tea_functions);
    fn register_regex_vibez_functions(&mut self) {
        let regex_vibez_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("regex_vibez", regex_vibez_functions);
    fn register_cryptz_functions(&mut self) {
        let cryptz_functions = vec![
            // Basic hash functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            // Incremental hashing
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            // Utility functions
            StdlibFunction {
            StdlibFunction {
            // Legacy functions (keep for compatibility)
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Symmetric encryption functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Key derivation functions
            StdlibFunction {
            StdlibFunction {
            
            // Utility functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
                description: "Generate IV/nonce".to_string(),
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("cryptz", cryptz_functions);
    fn register_reflectz_functions(&mut self) {
        let reflectz_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("reflectz", reflectz_functions);
    fn register_rizztemplate_functions(&mut self) {
        let rizztemplate_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("rizztemplate", rizztemplate_functions);
    fn register_htmlrizzler_functions(&mut self) {
        let htmlrizzler_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("htmlrizzler", htmlrizzler_functions);
    fn register_chadlogging_functions(&mut self) {
        let chadlogging_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("chadlogging", chadlogging_functions);
    fn register_char_functions(&mut self) {
        let char_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("char", char_functions);
    fn register_vibe_life_functions(&mut self) {
        let vibe_life_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("vibe_life", vibe_life_functions);
    fn register_timez_functions(&mut self) {
        let timez_functions = vec![
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("timez", timez_functions);
    /// Register sql_vibes database functions - COMPREHENSIVE DATABASE SUPPORT
    fn register_sql_vibes_functions(&mut self) {
        let sql_vibes_functions = vec![
            // Connection Management Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Query Execution Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Transaction Management Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Connection Pool Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Database Core Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Migration Functions
            StdlibFunction {
            StdlibFunction {
            
            // ORM Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // NoSQL Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("sql_vibes", sql_vibes_functions);
        self.register_package("db_core", Vec::from([]));
        self.register_package("db_pool", Vec::from([]));
        self.register_package("db_migrate", Vec::from([]));
        self.register_package("db_orm", Vec::from([]));
        self.register_package("db_nosql", Vec::from([]));
    /// Register additional database package functions 
    fn register_database_packages(&mut self) {
        // Register db_query advanced functions
        let db_query_functions = vec![
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Transaction Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Connection Pool Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Query Builder Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Result Set Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
            
            // Driver Management Functions
            StdlibFunction {
            StdlibFunction {
            StdlibFunction {
        ];
        
        self.register_package("db_query", db_query_functions);
    /// Register a package with its functions
    fn register_package(&mut self, package_name: &str, functions: Vec<StdlibFunction>) {
        let mut function_names = Vec::new();
        
        for func in functions {
            let qualified_name = format!("{}.{}", package_name, func.name);
            
            // Store function
            self.functions.insert(func.name.clone(), func.clone());
            self.qualified_functions.insert(qualified_name, func.clone());
            function_names.push(func.name.clone());
        self.packages.insert(package_name.to_string(), function_names);
    /// Get function by name (unqualified)
    pub fn get_function(&self, name: &str) -> Option<&StdlibFunction> {
        self.functions.get(name)
    /// Get function by qualified name (package.function)
    pub fn get_qualified_function(&self, qualified_name: &str) -> Option<&StdlibFunction> {
        self.qualified_functions.get(qualified_name)
    /// Get all packages
    pub fn get_packages(&self) -> impl Iterator<Item = &String> {
        self.packages.keys()
    /// Get functions in a package
    pub fn get_package_functions(&self, package: &str) -> Option<&Vec<String>> {
        self.packages.get(package)
    /// Get all function names
    pub fn get_all_functions(&self) -> impl Iterator<Item = &String> {
        self.functions.keys()
    /// Get total number of functions
    pub fn function_count(&self) -> usize {
        self.functions.len()
    /// Get total number of packages
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }
}

impl<'ctx> StdlibLlvmIntegration<'ctx> {
    /// Create new LLVM integration for stdlib
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        let registry = StdlibRegistry::new();
        let function_declarations = HashMap::new();
        
        Self {
        }
    }
    
    /// Initialize function declarations in LLVM module
    pub fn initialize_function_declarations(&mut self) -> Result<(), String> {
        // Clone the function maps to avoid borrowing issues
        let functions: Vec<_> = self.registry.functions.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let qualified_functions: Vec<_> = self.registry.qualified_functions.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        
        for (name, func_info) in functions {
            self.declare_function(&name, &func_info)?;
        for (qualified_name, func_info) in qualified_functions {
            self.declare_function(&qualified_name, &func_info)?;
        Ok(())
    /// Declare a function in the LLVM module
    fn declare_function(&mut self, name: &str, func_info: &StdlibFunction) -> Result<(), String> {
        let return_type = self.get_llvm_type(&func_info.return_type)?;
        let param_types: Result<Vec<BasicMetadataTypeEnum>, String> = func_info.param_types
            .iter()
            .map(|t| self.get_llvm_type(t).map(|bt| bt.into()))
            .collect();
        
        let param_types = param_types?;
        
        let function_type = if func_info.return_type == "void" {
            self.context.void_type().fn_type(&param_types, func_info.is_variadic)
        } else {
            return_type.fn_type(&param_types, func_info.is_variadic)
        
        let function = self.module.add_function(&func_info.llvm_name, function_type, None);
        self.function_declarations.insert(name.to_string(), function);
        
        Ok(())
    /// Convert CURSED type to LLVM type
    fn get_llvm_type(&self, cursed_type: &str) -> Result<BasicTypeEnum<'ctx>, String> {
        match cursed_type {
            "string" | "slice" | "any" | "error" | "mutex" | "channel" | "regex" | "template" | 
            "response" | "request" | "response_writer" | "handler" | "serve_mux" | "cookie" => {
                // These are pointer types to structures
                let i8_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(i8_ptr.into())
            "any..." => {
                // Variadic parameters - use i8*
                let i8_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(i8_ptr.into())
            _ => {
                // Unknown type, default to i8*
                let i8_ptr = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(i8_ptr.into())
            }
        }
    /// Get function info by name
    pub fn get_function_info(&self, name: &str) -> Option<&StdlibFunction> {
        self.registry.get_function(name)
            .or_else(|| self.registry.get_qualified_function(name))
    /// Get function declaration by name
    pub fn get_function_declaration(&self, name: &str) -> Option<&FunctionValue<'ctx>> {
        self.function_declarations.get(name)
    /// Get all packages
    pub fn get_packages(&self) -> impl Iterator<Item = &String> {
        self.registry.get_packages()
    /// Check if package exists
    pub fn has_package(&self, package: &str) -> bool {
        self.registry.packages.contains_key(package)
    /// Get function count
    pub fn function_count(&self) -> usize {
        self.registry.function_count()
    /// Get package count
    pub fn package_count(&self) -> usize {
        self.registry.package_count()
    }
}

impl Default for StdlibRegistry {
    fn default() -> Self {
        Self::new()
    }
}

