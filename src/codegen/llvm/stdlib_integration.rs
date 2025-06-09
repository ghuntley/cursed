//! Standard Library LLVM Integration for CURSED
//!
//! This module provides comprehensive integration between the CURSED standard library
//! and LLVM code generation, enabling compiled CURSED programs to call stdlib functions
//! with proper type safety, error handling, and performance optimization.

use crate::error::Error;
use crate::object::Object;
use crate::stdlib;
use inkwell::context::Context;
use inkwell::module::{Module, Linkage};
use inkwell::types::{BasicType, BasicTypeEnum, FunctionType, BasicMetadataTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, BasicMetadataValueEnum};
use inkwell::AddressSpace;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn, instrument};

/// Function signature for stdlib function implementations
pub type StdlibFunction = fn(&[Arc<Object>]) -> Result<Arc<Object>, Error>;

/// Information about a standard library function for LLVM integration
#[derive(Debug, Clone)]
pub struct StdlibFunctionInfo {
    /// Name of the function as used in CURSED code
    pub name: String,
    /// Package the function belongs to (e.g., "vibez", "mathz")
    pub package: String,
    /// LLVM function type signature
    pub llvm_type: String, // We'll store as string and parse when needed
    /// Rust implementation function
    pub implementation: Option<StdlibFunction>,
    /// Whether this function requires GC integration
    pub requires_gc: bool,
    /// Whether this function is a variadic function
    pub is_variadic: bool,
    /// Return type description
    pub return_type: String,
    /// Parameter type descriptions
    pub param_types: Vec<String>,
}

impl StdlibFunctionInfo {
    /// Creates a new stdlib function info entry
    #[instrument(skip(implementation))]
    pub fn new(
        name: &str,
        package: &str,
        return_type: &str,
        param_types: Vec<String>,
        implementation: Option<StdlibFunction>,
    ) -> Self {
        StdlibFunctionInfo {
            name: name.to_string(),
            package: package.to_string(),
            llvm_type: format!("{} ({})", return_type, param_types.join(", ")),
            implementation,
            requires_gc: return_type.contains("string") || return_type.contains("array") || return_type.contains("map"),
            is_variadic: param_types.iter().any(|t| t.contains("...")),
            return_type: return_type.to_string(),
            param_types,
        }
    }

    /// Gets the fully qualified function name (package.function)
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.package, self.name)
    }
}

/// Registry for standard library function information and LLVM integration
#[derive(Debug)]
pub struct StdlibRegistry {
    /// Map from function name to function info
    functions: HashMap<String, StdlibFunctionInfo>,
    /// Map from qualified name (package.function) to function info
    qualified_functions: HashMap<String, StdlibFunctionInfo>,
    /// Map from package name to list of function names
    packages: HashMap<String, Vec<String>>,
}

impl StdlibRegistry {
    /// Creates a new stdlib registry with all standard library functions
    #[instrument]
    pub fn new() -> Self {
        let mut registry = StdlibRegistry {
            functions: HashMap::new(),
            qualified_functions: HashMap::new(),
            packages: HashMap::new(),
        };

        registry.register_all_functions();
        registry
    }

    /// Registers all standard library functions
    #[instrument(skip(self))]
    fn register_all_functions(&mut self) {
        // Core functions
        self.register_core_functions();
        
        // I/O and formatting functions (vibez package)
        self.register_vibez_functions();
        
        // String manipulation functions (stringz package)
        self.register_stringz_functions();
        
        // Math functions (mathz package)
        self.register_mathz_functions();
        
        // Time functions (timez package)
        self.register_timez_functions();
        
        // OS interaction functions (vibe_life package)
        self.register_vibe_life_functions();
        
        // File I/O functions (dropz package)
        self.register_dropz_functions();
        
        // Concurrency functions (concurrenz package)
        self.register_concurrenz_functions();
        
        // HTTP functions (web_vibez package)
        self.register_web_vibez_functions();
        
        // JSON functions (json_tea package)
        self.register_json_tea_functions();
        
        // Regex functions (regex_vibez package)
        self.register_regex_vibez_functions();
        
        // Crypto functions (cryptz package)
        self.register_cryptz_functions();
        
        // Reflection functions (reflectz package)
        self.register_reflectz_functions();
        
        // Template functions (rizztemplate package)
        self.register_rizztemplate_functions();
        
        // HTML functions (htmlrizzler package)
        self.register_htmlrizzler_functions();
        
        // Logging functions (chadlogging package)
        self.register_chadlogging_functions();
        
        // Character functions (char operations)
        self.register_char_functions();

        info!(
            registered_functions = self.functions.len(),
            registered_packages = self.packages.len(),
            "Standard library registry initialized"
        );
    }

    /// Registers a function in the registry
    #[instrument(skip(self, info))]
    fn register_function(&mut self, info: StdlibFunctionInfo) {
        let qualified_name = info.qualified_name();
        
        // Add to package listing
        self.packages
            .entry(info.package.clone())
            .or_insert_with(Vec::new)
            .push(info.name.clone());
        
        // Add to registries
        self.functions.insert(info.name.clone(), info.clone());
        self.qualified_functions.insert(qualified_name, info);
    }

    /// Registers core built-in functions
    #[instrument(skip(self))]
    fn register_core_functions(&mut self) {
        // len function - gets length of strings, arrays, maps
        self.register_function(StdlibFunctionInfo::new(
            "len",
            "core",
            "i64",
            vec!["any".to_string()],
            Some(stdlib::core_len),
        ));

        // cap function - represents nil/null
        self.register_function(StdlibFunctionInfo::new(
            "cap",
            "core",
            "any",
            vec![],
            Some(stdlib::cap),
        ));

        // append function - appends elements to arrays/slices
        self.register_function(StdlibFunctionInfo::new(
            "append",
            "core",
            "array",
            vec!["array".to_string(), "any...".to_string()],
            Some(stdlib::append),
        ));

        // make function - creates slices and maps
        self.register_function(StdlibFunctionInfo::new(
            "make",
            "core",
            "any",
            vec!["type".to_string(), "i64...".to_string()],
            Some(stdlib::make),
        ));

        // panic function - causes program panic
        self.register_function(StdlibFunctionInfo::new(
            "panic",
            "core",
            "void",
            vec!["string".to_string()],
            Some(stdlib::panic),
        ));

        // recover function - recovers from panic
        self.register_function(StdlibFunctionInfo::new(
            "recover",
            "core",
            "any",
            vec![],
            Some(stdlib::recover),
        ));
    }

    /// Registers vibez package functions (I/O and formatting)
    #[instrument(skip(self))]
    fn register_vibez_functions(&mut self) {
        // spill function - print to stdout
        self.register_function(StdlibFunctionInfo::new(
            "spill",
            "vibez",
            "void",
            vec!["any...".to_string()],
            Some(stdlib::spill),
        ));

        // spillf function - formatted print to stdout  
        self.register_function(StdlibFunctionInfo::new(
            "spillf",
            "vibez",
            "void",
            vec!["string".to_string(), "any...".to_string()],
            Some(stdlib::spillf),
        ));

        // spillstr function - format to string
        self.register_function(StdlibFunctionInfo::new(
            "spillstr",
            "vibez",
            "string",
            vec!["any...".to_string()],
            Some(stdlib::spillstr),
        ));
    }

    /// Registers stringz package functions (string manipulation)
    #[instrument(skip(self))]
    fn register_stringz_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "contains",
            "stringz",
            "bool",
            vec!["string".to_string(), "string".to_string()],
            Some(stdlib::contains),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "join",
            "stringz",
            "string",
            vec!["array".to_string(), "string".to_string()],
            Some(stdlib::join),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "split",
            "stringz",
            "array",
            vec!["string".to_string(), "string".to_string()],
            Some(stdlib::split),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "to_lower",
            "stringz",
            "string",
            vec!["string".to_string()],
            Some(stdlib::to_lower),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "to_upper",
            "stringz",
            "string",
            vec!["string".to_string()],
            Some(stdlib::to_upper),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "trim",
            "stringz",
            "string",
            vec!["string".to_string()],
            Some(stdlib::trim),
        ));
    }

    /// Registers mathz package functions (mathematical operations)
    #[instrument(skip(self))]
    fn register_mathz_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "abs",
            "mathz",
            "number",
            vec!["number".to_string()],
            Some(stdlib::abs),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "sqrt",
            "mathz",
            "f64",
            vec!["number".to_string()],
            Some(stdlib::sqrt),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "sin",
            "mathz",
            "f64",
            vec!["number".to_string()],
            Some(stdlib::sin),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "cos",
            "mathz",
            "f64",
            vec!["number".to_string()],
            Some(stdlib::cos),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "max",
            "mathz",
            "number",
            vec!["number".to_string(), "number".to_string()],
            Some(stdlib::max),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "min",
            "mathz",
            "number",
            vec!["number".to_string(), "number".to_string()],
            Some(stdlib::min),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "pow",
            "mathz",
            "number",
            vec!["number".to_string(), "number".to_string()],
            Some(stdlib::pow),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "random",
            "mathz",
            "number",
            vec!["number...".to_string()],
            Some(stdlib::random),
        ));
    }

    /// Registers timez package functions (time operations)
    #[instrument(skip(self))]
    fn register_timez_functions(&mut self) {
        // Note: Using placeholder implementations since timez functions
        // return complex types that need proper LLVM mapping
        self.register_function(StdlibFunctionInfo::new(
            "now",
            "timez",
            "i64",
            vec![],
            None, // Will need proper implementation
        ));

        self.register_function(StdlibFunctionInfo::new(
            "sleep",
            "timez",
            "void",
            vec!["i64".to_string()],
            None,
        ));
    }

    /// Registers vibe_life package functions (OS interaction)
    #[instrument(skip(self))]
    fn register_vibe_life_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "getenv",
            "vibe_life",
            "string",
            vec!["string".to_string()],
            None, // Will need proper implementation
        ));

        self.register_function(StdlibFunctionInfo::new(
            "setenv",
            "vibe_life",
            "bool",
            vec!["string".to_string(), "string".to_string()],
            None,
        ));

        self.register_function(StdlibFunctionInfo::new(
            "exit",
            "vibe_life",
            "void",
            vec!["i32".to_string()],
            None,
        ));
    }

    /// Registers dropz package functions (file I/O)
    #[instrument(skip(self))]
    fn register_dropz_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "read_file",
            "dropz",
            "string",
            vec!["string".to_string()],
            Some(stdlib::read_file),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "write_file",
            "dropz",
            "bool",
            vec!["string".to_string(), "string".to_string()],
            Some(stdlib::write_file),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "file_exists",
            "dropz",
            "bool",
            vec!["string".to_string()],
            Some(stdlib::file_exists),
        ));
    }

    /// Registers concurrenz package functions (concurrency)
    #[instrument(skip(self))]
    fn register_concurrenz_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "new_mutex",
            "concurrenz",
            "mutex",
            vec![],
            Some(stdlib::new_mutex),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "new_channel",
            "concurrenz",
            "channel",
            vec!["i64".to_string()],
            Some(stdlib::new_channel),
        ));
    }

    /// Registers web_vibez package functions (HTTP)
    #[instrument(skip(self))]
    fn register_web_vibez_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "get",
            "web_vibez",
            "string",
            vec!["string".to_string()],
            Some(stdlib::http_get),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "post",
            "web_vibez",
            "string",
            vec!["string".to_string(), "string".to_string()],
            Some(stdlib::http_post),
        ));
    }

    /// Registers json_tea package functions (JSON)
    #[instrument(skip(self))]
    fn register_json_tea_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "marshal",
            "json_tea",
            "string",
            vec!["any".to_string()],
            Some(stdlib::marshal),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "unmarshal",
            "json_tea",
            "any",
            vec!["string".to_string()],
            Some(stdlib::unmarshal),
        ));
    }

    /// Registers regex_vibez package functions (regular expressions)
    #[instrument(skip(self))]
    fn register_regex_vibez_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "compile",
            "regex_vibez",
            "regex",
            vec!["string".to_string()],
            Some(stdlib::compile),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "match_str",
            "regex_vibez",
            "bool",
            vec!["regex".to_string(), "string".to_string()],
            Some(stdlib::match_str),
        ));
    }

    /// Registers cryptz package functions (cryptography)
    #[instrument(skip(self))]
    fn register_cryptz_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "hash",
            "cryptz",
            "string",
            vec!["string".to_string()],
            None, // Implementation signature doesn't match expected format
        ));

        self.register_function(StdlibFunctionInfo::new(
            "encrypt",
            "cryptz",
            "string",
            vec!["string".to_string(), "string".to_string()],
            None, // Implementation signature doesn't match expected format
        ));

        self.register_function(StdlibFunctionInfo::new(
            "decrypt",
            "cryptz",
            "string",
            vec!["string".to_string(), "string".to_string()],
            None, // Implementation signature doesn't match expected format
        ));
    }

    /// Registers reflectz package functions (reflection)
    #[instrument(skip(self))]
    fn register_reflectz_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "type_name",
            "reflectz",
            "string",
            vec!["any".to_string()],
            Some(stdlib::type_name),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "deep_equal",
            "reflectz",
            "bool",
            vec!["any".to_string(), "any".to_string()],
            Some(stdlib::deep_equal),
        ));
    }

    /// Registers rizztemplate package functions (text templates)
    #[instrument(skip(self))]
    fn register_rizztemplate_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "parse_template",
            "rizztemplate",
            "template",
            vec!["string".to_string()],
            Some(stdlib::parse_template),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "execute_template",
            "rizztemplate",
            "string",
            vec!["template".to_string(), "any".to_string()],
            Some(stdlib::execute_template),
        ));
    }

    /// Registers htmlrizzler package functions (HTML templates)
    #[instrument(skip(self))]
    fn register_htmlrizzler_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "escape_html",
            "htmlrizzler",
            "string",
            vec!["string".to_string()],
            Some(stdlib::escape_html),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "escape_js",
            "htmlrizzler",
            "string",
            vec!["string".to_string()],
            Some(stdlib::escape_js),
        ));
    }

    /// Registers chadlogging package functions (structured logging)
    #[instrument(skip(self))]
    fn register_chadlogging_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "debug",
            "chadlogging",
            "void",
            vec!["string".to_string(), "any...".to_string()],
            None, // Implementation signature doesn't match expected format
        ));

        self.register_function(StdlibFunctionInfo::new(
            "info",
            "chadlogging",
            "void",
            vec!["string".to_string(), "any...".to_string()],
            None, // Implementation signature doesn't match expected format
        ));

        self.register_function(StdlibFunctionInfo::new(
            "error",
            "chadlogging",
            "void",
            vec!["string".to_string(), "any...".to_string()],
            None, // Implementation signature doesn't match expected format
        ));
    }

    /// Registers character operation functions
    #[instrument(skip(self))]
    fn register_char_functions(&mut self) {
        self.register_function(StdlibFunctionInfo::new(
            "is_uppercase",
            "char",
            "bool",
            vec!["char".to_string()],
            Some(stdlib::is_uppercase),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "to_lowercase",
            "char",
            "char",
            vec!["char".to_string()],
            Some(stdlib::char_to_lowercase),
        ));

        self.register_function(StdlibFunctionInfo::new(
            "to_uppercase",
            "char",
            "char",
            vec!["char".to_string()],
            Some(stdlib::char_to_uppercase),
        ));
    }

    /// Gets function info by name
    pub fn get_function(&self, name: &str) -> Option<&StdlibFunctionInfo> {
        self.functions.get(name)
    }

    /// Gets function info by qualified name (package.function)
    pub fn get_qualified_function(&self, qualified_name: &str) -> Option<&StdlibFunctionInfo> {
        self.qualified_functions.get(qualified_name)
    }

    /// Gets all functions in a package
    pub fn get_package_functions(&self, package: &str) -> Option<&Vec<String>> {
        self.packages.get(package)
    }

    /// Gets all registered packages
    pub fn get_packages(&self) -> impl Iterator<Item = &String> {
        self.packages.keys()
    }

    /// Gets all registered functions
    pub fn get_all_functions(&self) -> impl Iterator<Item = &StdlibFunctionInfo> {
        self.functions.values()
    }
}

impl Default for StdlibRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// LLVM integration for standard library functions
pub struct StdlibLlvmIntegration<'ctx> {
    /// The LLVM context
    context: &'ctx Context,
    /// The LLVM module being generated
    module: &'ctx Module<'ctx>,
    /// Registry of stdlib functions
    registry: StdlibRegistry,
    /// Map from function name to LLVM function value
    llvm_functions: HashMap<String, FunctionValue<'ctx>>,
}

impl<'ctx> StdlibLlvmIntegration<'ctx> {
    /// Creates a new stdlib LLVM integration
    #[instrument(skip(context, module))]
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        StdlibLlvmIntegration {
            context,
            module,
            registry: StdlibRegistry::new(),
            llvm_functions: HashMap::new(),
        }
    }

    /// Generates LLVM function declarations for all stdlib functions
    #[instrument(skip(self))]
    pub fn generate_function_declarations(&mut self) -> Result<(), String> {
        debug!("Generating LLVM function declarations for stdlib functions");

        let func_infos: Vec<_> = self.registry.get_all_functions().cloned().collect();
        for func_info in func_infos {
            self.generate_function_declaration(&func_info)?;
        }

        info!(
            declared_functions = self.llvm_functions.len(),
            "Standard library function declarations generated"
        );

        Ok(())
    }

    /// Generates an LLVM function declaration for a specific stdlib function
    #[instrument(skip(self, func_info))]
    fn generate_function_declaration(&mut self, func_info: &StdlibFunctionInfo) -> Result<(), String> {
        // Map CURSED types to LLVM types
        let return_type = self.map_cursed_type_to_llvm(&func_info.return_type)?;
        let param_types: Result<Vec<_>, _> = func_info.param_types.iter()
            .filter(|t| !t.contains("...")) // Skip variadic markers for now
            .map(|t| self.map_cursed_type_to_llvm(t))
            .map(|opt_type| opt_type.map(|t| t.map(|bt| bt.into())))
            .collect();
        let param_types: Vec<inkwell::types::BasicMetadataTypeEnum> = param_types?.into_iter().flatten().collect();

        // Create function type
        let fn_type = match return_type {
            Some(ret_type) => ret_type.fn_type(&param_types, func_info.is_variadic),
            None => self.context.void_type().fn_type(&param_types, func_info.is_variadic),
        };

        // Create function declaration
        let qualified_name = func_info.qualified_name();
        let function = self.module.add_function(&qualified_name, fn_type, Some(Linkage::External));

        // Store for later use
        self.llvm_functions.insert(qualified_name.clone(), function);
        self.llvm_functions.insert(func_info.name.clone(), function);

        debug!(
            function_name = %qualified_name,
            return_type = %func_info.return_type,
            param_count = func_info.param_types.len(),
            "Generated LLVM function declaration"
        );

        Ok(())
    }

    /// Maps CURSED type names to LLVM types
    #[instrument(skip(self))]
    fn map_cursed_type_to_llvm(&self, cursed_type: &str) -> Result<Option<BasicTypeEnum<'ctx>>, String> {
        match cursed_type {
            "void" => Ok(None),
            "bool" => Ok(Some(self.context.bool_type().into())),
            "i32" => Ok(Some(self.context.i32_type().into())),
            "i64" => Ok(Some(self.context.i64_type().into())),
            "f32" => Ok(Some(self.context.f32_type().into())),
            "f64" => Ok(Some(self.context.f64_type().into())),
            "char" => Ok(Some(self.context.i8_type().into())),
            "string" => {
                // String is represented as {i64, i8*} struct
                let i64_type = self.context.i64_type();
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let string_type = self.context.struct_type(&[i64_type.into(), i8_ptr_type.into()], false);
                Ok(Some(string_type.into()))
            },
            "array" | "slice" => {
                // Array/slice is represented as {i64, i64, i8*} struct (length, capacity, data)
                let i64_type = self.context.i64_type();
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let slice_type = self.context.struct_type(&[
                    i64_type.into(), // length
                    i64_type.into(), // capacity  
                    i8_ptr_type.into(), // data pointer
                ], false);
                Ok(Some(slice_type.into()))
            },
            "map" => {
                // Map is represented as opaque pointer for now
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(Some(i8_ptr_type.into()))
            },
            "any" => {
                // Any type is represented as generic pointer
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(Some(i8_ptr_type.into()))
            },
            "number" => {
                // Number can be either int or float, use f64 as common type
                Ok(Some(self.context.f64_type().into()))
            },
            // Complex types that need registry lookup
            "mutex" | "channel" | "regex" | "template" => {
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(Some(i8_ptr_type.into()))
            },
            _ => {
                warn!(type_name = %cursed_type, "Unknown CURSED type, using generic pointer");
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(Some(i8_ptr_type.into()))
            }
        }
    }

    /// Gets the LLVM function for a stdlib function by name
    pub fn get_llvm_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.llvm_functions.get(name).copied()
    }

    /// Gets the function info from the registry
    pub fn get_function_info(&self, name: &str) -> Option<&StdlibFunctionInfo> {
        self.registry.get_function(name)
            .or_else(|| self.registry.get_qualified_function(name))
    }

    /// Gets all registered packages
    pub fn get_packages(&self) -> impl Iterator<Item = &String> {
        self.registry.get_packages()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_stdlib_registry_creation() {
        let registry = StdlibRegistry::new();
        
        // Check that core functions are registered
        assert!(registry.get_function("len").is_some());
        assert!(registry.get_function("cap").is_some());
        
        // Check that package functions are registered
        assert!(registry.get_qualified_function("vibez.spill").is_some());
        assert!(registry.get_qualified_function("mathz.abs").is_some());
        assert!(registry.get_qualified_function("stringz.contains").is_some());
        
        // Check package listing
        assert!(registry.get_packages().any(|p| p == "vibez"));
        assert!(registry.get_packages().any(|p| p == "mathz"));
        assert!(registry.get_packages().any(|p| p == "stringz"));
    }

    #[test]
    fn test_function_info_creation() {
        let info = StdlibFunctionInfo::new(
            "test_func",
            "test_pkg",
            "string",
            vec!["i32".to_string(), "string".to_string()],
            None,
        );
        
        assert_eq!(info.name, "test_func");
        assert_eq!(info.package, "test_pkg");
        assert_eq!(info.qualified_name(), "test_pkg.test_func");
        assert!(info.requires_gc); // String return type requires GC
        assert!(!info.is_variadic);
    }

    #[test]
    fn test_llvm_integration_creation() {
        // Use leaked memory to satisfy lifetime requirements in tests
        let context = Box::leak(Box::new(Context::create()));
        let module = Box::leak(Box::new(context.create_module("test")));
        
        let integration = StdlibLlvmIntegration::new(context, module);
        
        // Should have functions registered
        assert!(integration.get_function_info("len").is_some());
        assert!(integration.get_function_info("vibez.spill").is_some());
    }

    #[test]
    fn test_type_mapping() {
        // Use leaked memory to satisfy lifetime requirements in tests
        let context = Box::leak(Box::new(Context::create()));
        let module = Box::leak(Box::new(context.create_module("test")));
        
        let integration = StdlibLlvmIntegration::new(context, module);
        
        // Test basic type mappings
        assert!(integration.map_cursed_type_to_llvm("i32").unwrap().is_some());
        assert!(integration.map_cursed_type_to_llvm("string").unwrap().is_some());
        assert!(integration.map_cursed_type_to_llvm("void").unwrap().is_none());
        
        // Test array/slice type mapping
        let slice_type = integration.map_cursed_type_to_llvm("array").unwrap();
        assert!(slice_type.is_some());
    }
}
