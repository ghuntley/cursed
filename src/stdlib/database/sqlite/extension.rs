use crate::error::Error;
/// fr fr SQLite extensions and user-defined functions that slay periodt
/// 
/// This module provides support for SQLite extensions, user-defined functions,
/// collations, and virtual tables to extend database functionality.

use std::collections::HashMap;
use std::ffi::CString;
use super::{SqliteError, SqliteResult};

/// fr fr SQLite extension information
#[derive(Debug, Clone)]
pub struct SqliteExtension {
    /// fr fr Extension name
    pub name: String,
    /// fr fr Extension file path
    pub path: String,
    /// fr fr Entry point function name
    pub entry_point: Option<String>,
    /// fr fr Whether extension is loaded
    pub loaded: bool,
    /// fr fr Extension description
    pub description: String,
    /// fr fr Extension version
    pub version: String,
}

impl SqliteExtension {
    /// slay Create new extension definition
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            entry_point: None,
            loaded: false,
            description: String::new(),
            version: String::new(),
        }
    }

    /// slay Set entry point
    pub fn with_entry_point(mut self, entry_point: &str) -> Self {
        self.entry_point = Some(entry_point.to_string());
        self
    }

    /// slay Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// slay Set version
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// slay Validate extension
    pub fn validate(&self) -> SqliteResult<()> {
        if self.name.is_empty() {
            return Err(SqliteError::invalid_parameter("Extension name cannot be empty"));
        }

        if self.path.is_empty() {
            return Err(SqliteError::invalid_parameter("Extension path cannot be empty"));
        }

        Ok(())
    }
}

/// fr fr User-defined function types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    Scalar,
    Aggregate,
    Window,
}

/// fr fr User-defined function definition
#[derive(Debug, Clone)]
pub struct SqliteFunction {
    /// fr fr Function name
    pub name: String,
    /// fr fr Function type
    pub function_type: FunctionType,
    /// fr fr Number of arguments (-1 for variable)
    pub num_args: i32,
    /// fr fr Text encoding
    pub text_encoding: TextEncoding,
    /// fr fr Function description
    pub description: String,
    /// fr fr Whether function is deterministic
    pub deterministic: bool,
    /// fr fr Whether function is registered
    pub registered: bool,
}

/// fr fr Text encoding types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Utf16,
    Utf16be,
    Utf16le,
    Any,
}

impl TextEncoding {
    /// slay Convert to SQLite constant
    pub fn to_sqlite_constant(self) -> i32 {
        match self {
            TextEncoding::Utf8 => 1,
            TextEncoding::Utf16 => 4,
            TextEncoding::Utf16be => 2,
            TextEncoding::Utf16le => 3,
            TextEncoding::Any => 5,
        }
    }
}

impl SqliteFunction {
    /// slay Create new scalar function
    pub fn scalar(name: &str, num_args: i32) -> Self {
        Self {
            name: name.to_string(),
            function_type: FunctionType::Scalar,
            num_args,
            text_encoding: TextEncoding::Utf8,
            description: String::new(),
            deterministic: true,
            registered: false,
        }
    }

    /// slay Create new aggregate function
    pub fn aggregate(name: &str, num_args: i32) -> Self {
        Self {
            name: name.to_string(),
            function_type: FunctionType::Aggregate,
            num_args,
            text_encoding: TextEncoding::Utf8,
            description: String::new(),
            deterministic: false, // Aggregates are typically non-deterministic
            registered: false,
        }
    }

    /// slay Create new window function
    pub fn window(name: &str, num_args: i32) -> Self {
        Self {
            name: name.to_string(),
            function_type: FunctionType::Window,
            num_args,
            text_encoding: TextEncoding::Utf8,
            description: String::new(),
            deterministic: false,
            registered: false,
        }
    }

    /// slay Set encoding
    pub fn with_encoding(mut self, encoding: TextEncoding) -> Self {
        self.text_encoding = encoding;
        self
    }

    /// slay Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// slay Set deterministic flag
    pub fn deterministic(mut self, deterministic: bool) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// slay Validate function definition
    pub fn validate(&self) -> SqliteResult<()> {
        if self.name.is_empty() {
            return Err(SqliteError::invalid_parameter("Function name cannot be empty"));
        }

        if self.num_args < -1 || self.num_args > 127 {
            return Err(SqliteError::invalid_parameter("Invalid number of arguments"));
        }

        Ok(())
    }
}

/// fr fr Collation definition
#[derive(Debug, Clone)]
pub struct SqliteCollation {
    /// fr fr Collation name
    pub name: String,
    /// fr fr Text encoding
    pub text_encoding: TextEncoding,
    /// fr fr Collation description
    pub description: String,
    /// fr fr Whether collation is registered
    pub registered: bool,
}

impl SqliteCollation {
    /// slay Create new collation
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            text_encoding: TextEncoding::Utf8,
            description: String::new(),
            registered: false,
        }
    }

    /// slay Set encoding
    pub fn with_encoding(mut self, encoding: TextEncoding) -> Self {
        self.text_encoding = encoding;
        self
    }

    /// slay Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// slay Validate collation
    pub fn validate(&self) -> SqliteResult<()> {
        if self.name.is_empty() {
            return Err(SqliteError::invalid_parameter("Collation name cannot be empty"));
        }

        Ok(())
    }
}

/// fr fr Virtual table definition
#[derive(Debug, Clone)]
pub struct SqliteVirtualTable {
    /// fr fr Module name
    pub module_name: String,
    /// fr fr Table name
    pub table_name: String,
    /// fr fr Module description
    pub description: String,
    /// fr fr Whether module is registered
    pub registered: bool,
    /// fr fr Module configuration
    pub config: HashMap<String, String>,
}

impl SqliteVirtualTable {
    /// slay Create new virtual table
    pub fn new(module_name: &str, table_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            table_name: table_name.to_string(),
            description: String::new(),
            registered: false,
            config: HashMap::new(),
        }
    }

    /// slay Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// slay Add configuration parameter
    pub fn with_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Validate virtual table
    pub fn validate(&self) -> SqliteResult<()> {
        if self.module_name.is_empty() {
            return Err(SqliteError::invalid_parameter("Module name cannot be empty"));
        }

        if self.table_name.is_empty() {
            return Err(SqliteError::invalid_parameter("Table name cannot be empty"));
        }

        Ok(())
    }
}

/// fr fr Extension manager for SQLite
#[derive(Debug)]
pub struct SqliteExtensionManager {
    /// fr fr Loaded extensions
    extensions: HashMap<String, SqliteExtension>,
    /// fr fr Registered functions
    functions: HashMap<String, SqliteFunction>,
    /// fr fr Registered collations
    collations: HashMap<String, SqliteCollation>,
    /// fr fr Registered virtual tables
    virtual_tables: HashMap<String, SqliteVirtualTable>,
    /// fr fr Whether extensions are enabled
    extensions_enabled: bool,
}

impl SqliteExtensionManager {
    /// slay Create new extension manager
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            functions: HashMap::new(),
            collations: HashMap::new(),
            virtual_tables: HashMap::new(),
            extensions_enabled: false,
        }
    }

    /// slay Enable extension loading
    pub fn enable_extensions(&mut self, enable: bool) -> SqliteResult<()> {
        self.extensions_enabled = enable;
        // This would call sqlite3_enable_load_extension()
        Ok(())
    }

    /// slay Load extension
    pub fn load_extension(&mut self, extension: SqliteExtension) -> SqliteResult<()> {
        if !self.extensions_enabled {
            return Err(SqliteError::extension_error("Extensions are not enabled"));
        }

        extension.validate()?;

        // This would call sqlite3_load_extension()
        // For now, just mark as loaded
        let mut ext = extension;
        ext.loaded = true;

        self.extensions.insert(ext.name.clone(), ext);
        Ok(())
    }

    /// slay Unload extension
    pub fn unload_extension(&mut self, name: &str) -> SqliteResult<()> {
        if let Some(mut extension) = self.extensions.remove(name) {
            extension.loaded = false;
            // This would unload the extension
            Ok(())
        } else {
            Err(SqliteError::extension_error(&format!("Extension '{}' not found", name)))
        }
    }

    /// slay Register function
    pub fn register_function(&mut self, function: SqliteFunction) -> SqliteResult<()> {
        function.validate()?;

        // This would call sqlite3_create_function_v2()
        let mut func = function;
        func.registered = true;

        self.functions.insert(func.name.clone(), func);
        Ok(())
    }

    /// slay Unregister function
    pub fn unregister_function(&mut self, name: &str) -> SqliteResult<()> {
        if self.functions.remove(name).is_some() {
            // This would remove the function from SQLite
            Ok(())
        } else {
            Err(SqliteError::extension_error(&format!("Function '{}' not found", name)))
        }
    }

    /// slay Register collation
    pub fn register_collation(&mut self, collation: SqliteCollation) -> SqliteResult<()> {
        collation.validate()?;

        // This would call sqlite3_create_collation_v2()
        let mut coll = collation;
        coll.registered = true;

        self.collations.insert(coll.name.clone(), coll);
        Ok(())
    }

    /// slay Unregister collation
    pub fn unregister_collation(&mut self, name: &str) -> SqliteResult<()> {
        if self.collations.remove(name).is_some() {
            // This would remove the collation from SQLite
            Ok(())
        } else {
            Err(SqliteError::extension_error(&format!("Collation '{}' not found", name)))
        }
    }

    /// slay Register virtual table
    pub fn register_virtual_table(&mut self, virtual_table: SqliteVirtualTable) -> SqliteResult<()> {
        virtual_table.validate()?;

        // This would call sqlite3_create_module_v2()
        let mut vt = virtual_table;
        vt.registered = true;

        self.virtual_tables.insert(vt.module_name.clone(), vt);
        Ok(())
    }

    /// slay Unregister virtual table
    pub fn unregister_virtual_table(&mut self, module_name: &str) -> SqliteResult<()> {
        if self.virtual_tables.remove(module_name).is_some() {
            // This would remove the virtual table module from SQLite
            Ok(())
        } else {
            Err(SqliteError::extension_error(&format!("Virtual table '{}' not found", module_name)))
        }
    }

    /// slay List loaded extensions
    pub fn list_extensions(&self) -> Vec<&SqliteExtension> {
        self.extensions.values().collect()
    }

    /// slay List registered functions
    pub fn list_functions(&self) -> Vec<&SqliteFunction> {
        self.functions.values().collect()
    }

    /// slay List registered collations
    pub fn list_collations(&self) -> Vec<&SqliteCollation> {
        self.collations.values().collect()
    }

    /// slay List registered virtual tables
    pub fn list_virtual_tables(&self) -> Vec<&SqliteVirtualTable> {
        self.virtual_tables.values().collect()
    }

    /// slay Check if extension is loaded
    pub fn is_extension_loaded(&self, name: &str) -> bool {
        self.extensions.get(name)
            .map(|e| e.loaded)
            .unwrap_or(false)
    }

    /// slay Check if function is registered
    pub fn is_function_registered(&self, name: &str) -> bool {
        self.functions.get(name)
            .map(|f| f.registered)
            .unwrap_or(false)
    }

    /// slay Get extension information
    pub fn get_extension(&self, name: &str) -> Option<&SqliteExtension> {
        self.extensions.get(name)
    }

    /// slay Get function information
    pub fn get_function(&self, name: &str) -> Option<&SqliteFunction> {
        self.functions.get(name)
    }

    /// slay Get collation information
    pub fn get_collation(&self, name: &str) -> Option<&SqliteCollation> {
        self.collations.get(name)
    }

    /// slay Get virtual table information
    pub fn get_virtual_table(&self, module_name: &str) -> Option<&SqliteVirtualTable> {
        self.virtual_tables.get(module_name)
    }
}

impl Default for SqliteExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Common SQLite extensions
impl SqliteExtensionManager {
    /// slay Register common math functions
    pub fn register_math_functions(&mut self) -> SqliteResult<()> {
        let functions = vec![
            SqliteFunction::scalar("abs", 1).with_description("Absolute value"),
            SqliteFunction::scalar("sqrt", 1).with_description("Square root"),
            SqliteFunction::scalar("pow", 2).with_description("Power function"),
            SqliteFunction::scalar("log", 1).with_description("Natural logarithm"),
            SqliteFunction::scalar("log10", 1).with_description("Base-10 logarithm"),
            SqliteFunction::scalar("exp", 1).with_description("Exponential function"),
            SqliteFunction::scalar("sin", 1).with_description("Sine function"),
            SqliteFunction::scalar("cos", 1).with_description("Cosine function"),
            SqliteFunction::scalar("tan", 1).with_description("Tangent function"),
            SqliteFunction::scalar("asin", 1).with_description("Arc sine"),
            SqliteFunction::scalar("acos", 1).with_description("Arc cosine"),
            SqliteFunction::scalar("atan", 1).with_description("Arc tangent"),
            SqliteFunction::scalar("atan2", 2).with_description("Two-argument arc tangent"),
            SqliteFunction::scalar("ceil", 1).with_description("Ceiling function"),
            SqliteFunction::scalar("floor", 1).with_description("Floor function"),
            SqliteFunction::scalar("round", 2).with_description("Round to decimal places"),
            SqliteFunction::scalar("pi", 0).with_description("Pi constant"),
        ];

        for function in functions {
            self.register_function(function)?;
        }

        Ok(())
    }

    /// slay Register string functions
    pub fn register_string_functions(&mut self) -> SqliteResult<()> {
        let functions = vec![
            SqliteFunction::scalar("reverse", 1).with_description("Reverse string"),
            SqliteFunction::scalar("proper", 1).with_description("Proper case"),
            SqliteFunction::scalar("lpad", 3).with_description("Left pad string"),
            SqliteFunction::scalar("rpad", 3).with_description("Right pad string"),
            SqliteFunction::scalar("split_part", 3).with_description("Split string part"),
            SqliteFunction::scalar("regexp", 2).with_description("Regular expression match"),
            SqliteFunction::scalar("regexp_replace", 3).with_description("Regular expression replace"),
        ];

        for function in functions {
            self.register_function(function)?;
        }

        Ok(())
    }

    /// slay Register common collations
    pub fn register_common_collations(&mut self) -> SqliteResult<()> {
        let collations = vec![
            SqliteCollation::new("NOCASE_UTF8")
                .with_description("Case-insensitive UTF-8 collation"),
            SqliteCollation::new("NUMERIC")
                .with_description("Numeric collation"),
            SqliteCollation::new("REVERSE")
                .with_description("Reverse order collation"),
        ];

        for collation in collations {
            self.register_collation(collation)?;
        }

        Ok(())
    }
}

impl SqliteError {
    /// slay Create extension error
    pub fn extension_error(message: &str) -> Self {
        Self::new(super::SqliteErrorCode::ExtensionError, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_creation() {
        let ext = SqliteExtension::new("test_ext", "/path/to/extension.so")
            .with_entry_point("sqlite3_extension_init")
            .with_description("Test extension")
            .with_version("1.0.0");

        assert_eq!(ext.name, "test_ext");
        assert_eq!(ext.path, "/path/to/extension.so");
        assert_eq!(ext.entry_point, Some("sqlite3_extension_init".to_string()));
        assert!(!ext.loaded);
        assert!(ext.validate().is_ok());

        let invalid_ext = SqliteExtension::new("", "");
        assert!(invalid_ext.validate().is_err());
    }

    #[test]
    fn test_function_types() {
        let scalar = SqliteFunction::scalar("test_func", 2)
            .with_description("Test scalar function")
            .deterministic(true);

        assert_eq!(scalar.function_type, FunctionType::Scalar);
        assert_eq!(scalar.num_args, 2);
        assert!(scalar.deterministic);
        assert!(scalar.validate().is_ok());

        let aggregate = SqliteFunction::aggregate("test_agg", 1);
        assert_eq!(aggregate.function_type, FunctionType::Aggregate);
        assert!(!aggregate.deterministic);

        let window = SqliteFunction::window("test_win", 1);
        assert_eq!(window.function_type, FunctionType::Window);

        let invalid_func = SqliteFunction::scalar("", 1);
        assert!(invalid_func.validate().is_err());
    }

    #[test]
    fn test_text_encoding() {
        assert_eq!(TextEncoding::Utf8.to_sqlite_constant(), 1);
        assert_eq!(TextEncoding::Utf16.to_sqlite_constant(), 4);
        assert_eq!(TextEncoding::Any.to_sqlite_constant(), 5);
    }

    #[test]
    fn test_collation() {
        let collation = SqliteCollation::new("test_collation")
            .with_encoding(TextEncoding::Utf16)
            .with_description("Test collation");

        assert_eq!(collation.name, "test_collation");
        assert_eq!(collation.text_encoding, TextEncoding::Utf16);
        assert!(!collation.registered);
        assert!(collation.validate().is_ok());

        let invalid_collation = SqliteCollation::new("");
        assert!(invalid_collation.validate().is_err());
    }

    #[test]
    fn test_virtual_table() {
        let vt = SqliteVirtualTable::new("test_module", "test_table")
            .with_description("Test virtual table")
            .with_config("param1", "value1")
            .with_config("param2", "value2");

        assert_eq!(vt.module_name, "test_module");
        assert_eq!(vt.table_name, "test_table");
        assert_eq!(vt.config.len(), 2);
        assert!(vt.validate().is_ok());

        let invalid_vt = SqliteVirtualTable::new("", "");
        assert!(invalid_vt.validate().is_err());
    }

    #[test]
    fn test_extension_manager() {
        let mut manager = SqliteExtensionManager::new();
        assert!(!manager.extensions_enabled);

        // Enable extensions
        assert!(manager.enable_extensions(true).is_ok());
        assert!(manager.extensions_enabled);

        // Load extension
        let ext = SqliteExtension::new("test", "/path/test.so");
        assert!(manager.load_extension(ext).is_ok());
        assert!(manager.is_extension_loaded("test"));
        assert_eq!(manager.list_extensions().len(), 1);

        // Register function
        let func = SqliteFunction::scalar("test_func", 1);
        assert!(manager.register_function(func).is_ok());
        assert!(manager.is_function_registered("test_func"));
        assert_eq!(manager.list_functions().len(), 1);

        // Register collation
        let coll = SqliteCollation::new("test_collation");
        assert!(manager.register_collation(coll).is_ok());
        assert_eq!(manager.list_collations().len(), 1);

        // Register virtual table
        let vt = SqliteVirtualTable::new("test_module", "test_table");
        assert!(manager.register_virtual_table(vt).is_ok());
        assert_eq!(manager.list_virtual_tables().len(), 1);
    }

    #[test]
    fn test_common_functions() {
        let mut manager = SqliteExtensionManager::new();
        
        assert!(manager.register_math_functions().is_ok());
        assert!(manager.is_function_registered("sqrt"));
        assert!(manager.is_function_registered("sin"));
        assert!(manager.is_function_registered("pi"));

        assert!(manager.register_string_functions().is_ok());
        assert!(manager.is_function_registered("reverse"));
        assert!(manager.is_function_registered("regexp"));

        assert!(manager.register_common_collations().is_ok());
        assert!(manager.get_collation("NOCASE_UTF8").is_some());
    }
}
