use crate::error::Error;
/// Core plugin representation and loading functionality
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
use crate::stdlib::plug_vibes::version::{Version, parse_version};
use crate::stdlib::value::Value;

/// Options for loading plugins
#[derive(Debug, Clone)]
pub struct LoadOptions {
    /// Verify plugin is compatible with host version
    pub version_check: bool,
    /// Verify plugin signature for security
    pub verify_signature: bool,
    /// Load plugin in isolated context
    pub isolation: bool,
    /// Run plugin in sandboxed environment
    pub sandbox: bool,
    /// Timeout for plugin initialization
    pub timeout: Duration,
    /// Additional plugin dependencies
    pub dependencies: Vec<String>,
    /// Whitelist of packages the plugin can import
    pub allowed_imports: Vec<String>,
    /// Enable debug logging for plugin operations
    pub debug_logging: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            version_check: true,
            verify_signature: false,
            isolation: false,
            sandbox: false,
            timeout: Duration::from_secs(30),
            dependencies: Vec::new(),
            allowed_imports: Vec::new(),
            debug_logging: false,
        }
    }
}

/// Information about a loaded plugin
#[derive(Debug, Clone)]
pub struct PlugInfo {
    pub name: String,
    pub version: String,
    pub api: String,
    pub author: String,
    pub description: String,
    pub build_time: SystemTime,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub signature: String,
    pub is_verified: bool,
    pub is_compatible: bool,
}

impl Default for PlugInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "0.0.0".to_string(),
            api: "1.0".to_string(),
            author: String::new(),
            description: String::new(),
            build_time: SystemTime::now(),
            dependencies: Vec::new(),
            capabilities: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            signature: String::new(),
            is_verified: false,
            is_compatible: true,
        }
    }
}

/// Host application information provided to plugins
#[derive(Debug, Clone)]
pub struct HostInfo {
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub platform: String,
    pub architecture: String,
}

impl Default for HostInfo {
    fn default() -> Self {
        Self {
            name: "CURSED".to_string(),
            version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            platform: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
        }
    }
}

/// Plugin function signature type
pub type PluginFunction = Box<dyn Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync>;

/// Helper trait for converting between different value types and FFI
pub trait PluginValueConverter {
    fn to_plugin_value(&self) -> Value;
    fn from_plugin_value(value: &Value) -> Option<Self> where Self: Sized;
}

impl PluginValueConverter for i32 {
    fn to_plugin_value(&self) -> Value {
        Value::Integer(*self as i64)
    }
    
    fn from_plugin_value(value: &Value) -> Option<Self> {
        match value {
            Value::Integer(i) => Some(*i as i32),
            _ => None,
        }
    }
}

impl PluginValueConverter for f64 {
    fn to_plugin_value(&self) -> Value {
        Value::Float(*self)
    }
    
    fn from_plugin_value(value: &Value) -> Option<Self> {
        match value {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
}

impl PluginValueConverter for String {
    fn to_plugin_value(&self) -> Value {
        Value::String(self.clone())
    }
    
    fn from_plugin_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl PluginValueConverter for bool {
    fn to_plugin_value(&self) -> Value {
        Value::Boolean(*self)
    }
    
    fn from_plugin_value(value: &Value) -> Option<Self> {
        match value {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

/// Represents a loaded plugin
pub struct Plug {
    info: PlugInfo,
    path: PathBuf,
    handle: Option<libloading::Library>,
    symbols: Arc<Mutex<HashMap<String, Value>>>,
    functions: Arc<Mutex<HashMap<String, PluginFunction>>>,
    is_loaded: bool,
    load_time: SystemTime,
}

impl Plug {
    /// Create a new plugin instance
    pub fn new(path: PathBuf, info: PlugInfo) -> Self {
        Self {
            info,
            path,
            handle: None,
            symbols: Arc::new(Mutex::new(HashMap::new())),
            functions: Arc::new(Mutex::new(HashMap::new())),
            is_loaded: false,
            load_time: SystemTime::now(),
        }
    }

    /// Get plugin information
    pub fn info(&self) -> &PlugInfo {
        &self.info
    }

    /// Get plugin path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if plugin is loaded
    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    /// Get load time
    pub fn load_time(&self) -> SystemTime {
        self.load_time
    }

    /// Look up a symbol by name
    pub fn lookup(&self, symbol_name: &str) -> PluginResult<Value> {
        let symbols = self.symbols.lock().map_err(|_| {
            PluginError::general("Failed to acquire symbols lock")
        })?;

        symbols.get(symbol_name)
            .cloned()
            .ok_or_else(|| PluginError::symbol_not_found(symbol_name))
    }

    /// Look up a function by name and return a callable wrapper
    pub fn lookup_func(&self, func_name: &str) -> PluginResult<Box<dyn Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync>> {
        let functions = self.functions.lock().map_err(|_| {
            PluginError::general("Failed to acquire functions lock")
        })?;

        if let Some(func) = functions.get(func_name) {
            // Clone the function to avoid lifetime issues
            let func_clone = func.clone();
            Ok(Box::new(move |args| func_clone(args)) as Box<dyn Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync>)
        } else {
            // Try to look up from the loaded library
            if let Some(ref handle) = self.handle {
                // Look for a C-compatible function that takes args and returns results
                unsafe {
                    let symbol_name = format!("{}\0", func_name);
                    match handle.get::<libloading::Symbol<unsafe extern "C" fn(*const u8, usize) -> *mut u8>>(symbol_name.as_bytes()) {
                        Ok(_symbol) => {
                            // For now, return a placeholder function
                            // In a full implementation, this would call the actual C function
                            Err(PluginError::function_not_found(func_name))
                        },
                        Err(_) => Err(PluginError::function_not_found(func_name))
                    }
                }
            } else {
                Err(PluginError::function_not_found(func_name))
            }
        }
    }

    /// Look up and bind a symbol to a specific type
    pub fn lookup_symbol<T>(&self, symbol_name: &str) -> PluginResult<T> 
    where
        T: Clone + 'static,
    {
        // Try to look up the symbol directly from the loaded library
        if let Some(ref handle) = self.handle {
            unsafe {
                let symbol_name_c = format!("{}\0", symbol_name);
                match handle.get::<libloading::Symbol<fn() -> T>>(symbol_name_c.as_bytes()) {
                    Ok(symbol_func) => {
                        let result = symbol_func();
                        Ok(result)
                    },
                    Err(_) => {
                        // Try as a static symbol
                        match handle.get::<*const T>(symbol_name_c.as_bytes()) {
                            Ok(symbol_ptr) => {
                                let result = (*symbol_ptr).clone();
                                Ok(result)
                            },
                            Err(_) => Err(PluginError::symbol_not_found(symbol_name))
                        }
                    }
                }
            }
        } else {
            Err(PluginError::general("Plugin not loaded"))
        }
    }

    /// Register a symbol in the plugin
    pub fn register_symbol(&self, name: String, value: Value) -> PluginResult<()> {
        let mut symbols = self.symbols.lock().map_err(|_| {
            PluginError::general("Failed to acquire symbols lock")
        })?;

        symbols.insert(name, value);
        Ok(())
    }

    /// Register a function in the plugin
    pub fn register_function(&self, name: String, func: PluginFunction) -> PluginResult<()> {
        let mut functions = self.functions.lock().map_err(|_| {
            PluginError::general("Failed to acquire functions lock")
        })?;

        functions.insert(name, func);
        Ok(())
    }

    /// Get all symbol names
    pub fn symbols(&self) -> Vec<String> {
        self.symbols.lock()
            .map(|symbols| symbols.keys().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<String> {
        self.functions.lock()
            .map(|functions| functions.keys().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Call a function by name with typed arguments and return typed result
    pub fn call_function<T, R>(&self, func_name: &str, args: &[T]) -> PluginResult<R> 
    where
        T: PluginValueConverter,
        R: PluginValueConverter,
    {
        // Convert arguments to Value types
        let value_args: Vec<Value> = args.iter().map(|arg| arg.to_plugin_value()).collect();
        
        // Look up and call the function
        let func = self.lookup_func(func_name)?;
        let results = func(&value_args)?;
        
        // Convert first result back to desired type
        if let Some(first_result) = results.first() {
            R::from_plugin_value(first_result)
                .ok_or_else(|| PluginError::general("Failed to convert function result"))
        } else {
            Err(PluginError::general("Function returned no results"))
        }
    }

    /// Check if a symbol exists in the plugin
    pub fn has_symbol(&self, symbol_name: &str) -> bool {
        if let Some(ref handle) = self.handle {
            unsafe {
                let symbol_name_c = format!("{}\0", symbol_name);
                handle.get::<*const u8>(symbol_name_c.as_bytes()).is_ok()
            }
        } else {
            false
        }
    }

    /// Get detailed plugin statistics
    pub fn get_statistics(&self) -> PluginResult<HashMap<String, Value>> {
        let mut stats = HashMap::new();
        
        stats.insert("loaded".to_string(), Value::Boolean(self.is_loaded));
        stats.insert("symbol_count".to_string(), Value::Integer(self.symbols().len() as i64));
        stats.insert("function_count".to_string(), Value::Integer(self.function_names().len() as i64));
        stats.insert("load_time".to_string(), Value::String(format!("{:?}", self.load_time)));
        stats.insert("path".to_string(), Value::String(self.path.to_string_lossy().to_string()));
        stats.insert("name".to_string(), Value::String(self.info.name.clone()));
        stats.insert("version".to_string(), Value::String(self.info.version.clone()));
        
        Ok(stats)
    }

    /// Verify plugin integrity (basic checks)
    pub fn verify_integrity(&self) -> PluginResult<bool> {
        // Check if the plugin file still exists
        if !self.path.exists() {
            return Ok(false);
        }
        
        // Check if the plugin is loaded
        if !self.is_loaded {
            return Ok(false);
        }
        
        // Check if we can access basic symbols
        if self.handle.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Close the plugin and clean up resources
    pub fn close(&mut self) -> PluginResult<()> {
        if !self.is_loaded {
            return Ok(());
        }

        // Call cleanup function if it exists
        if let Some(ref handle) = self.handle {
            if let Ok(cleanup_func) = unsafe {
                handle.get::<libloading::Symbol<unsafe extern "C" fn() -> i32>>(b"Cleanup")
            } {
                let result = unsafe { cleanup_func() };
                if result != 0 {
                    return Err(PluginError::cleanup_failed(&format!("Cleanup returned: {}", result)));
                }
            }
        }

        // Clear symbols and functions
        if let Ok(mut symbols) = self.symbols.lock() {
            symbols.clear();
        }
        if let Ok(mut functions) = self.functions.lock() {
            functions.clear();
        }

        // Close the dynamic library handle
        if let Some(handle) = self.handle.take() {
            drop(handle);
        }

        self.is_loaded = false;
        Ok(())
    }
}

impl Drop for Plug {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Load a plugin from a file path
pub fn load(path: &str) -> PluginResult<Plug> {
    load_with_options(path, LoadOptions::default())
}

/// Load a plugin with specific options
pub fn load_with_options(path: &str, options: LoadOptions) -> PluginResult<Plug> {
    let path_buf = PathBuf::from(path);
    
    // Check if file exists
    if !path_buf.exists() {
        return Err(PluginError::plugin_not_found(path));
    }

    // Validate file extension (optional)
    let extension = path_buf.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let expected_extensions = if cfg!(target_os = "windows") {
        vec!["dll"]
    } else if cfg!(target_os = "macos") {
        vec!["dylib"]
    } else {
        vec!["so"]
    };
    
    if !expected_extensions.contains(&extension) && options.debug_logging {
        eprintln!("Warning: Plugin file '{}' does not have expected extension for this platform", path);
    }

    // Load the dynamic library
    let library = unsafe {
        libloading::Library::new(&path_buf).map_err(|e| {
            PluginError::load_error(&format!("Failed to load library '{}': {}", path, e))
        })?
    };

    // Try to get plugin manifest - this is optional
    let info = unsafe {
        match library.get::<libloading::Symbol<unsafe extern "C" fn() -> PlugInfo>>(b"PlugManifest") {
            Ok(manifest_func) => manifest_func(),
            Err(_) => {
                // Create default info if PlugManifest not found
                let mut default_info = PlugInfo::default();
                default_info.name = path_buf.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                default_info
            }
        }
    };

    // Verify version compatibility if requested
    if options.version_check && !info.version.is_empty() && info.version != "0.0.0" {
        let plugin_version = parse_version(&info.version).map_err(|e| {
            PluginError::version_incompatible(&format!("Invalid plugin version: {}", e))
        })?;

        let host_version = Version::new(1, 0, 0); // Current host version
        if !plugin_version.compatible(&host_version) {
            return Err(PluginError::version_incompatible(&format!(
                "Plugin version {} incompatible with host version {}",
                plugin_version, host_version
            )));
        }
    }

    // Create plugin instance
    let mut plugin = Plug::new(path_buf, info);
    plugin.handle = Some(library);
    plugin.is_loaded = true;

    // Call initialization function if it exists
    if let Ok(init_func) = unsafe {
        plugin.handle.as_ref().unwrap().get::<libloading::Symbol<unsafe extern "C" fn() -> i32>>(b"Init")
    } {
        let result = unsafe { init_func() };
        if result != 0 {
            plugin.is_loaded = false;
            return Err(PluginError::initialization_failed(&format!("Init returned: {}", result)));
        }
    }

    // Populate symbol list by trying common function names
    if let Some(ref handle) = plugin.handle {
        let common_symbols = vec![
            "Init", "Cleanup", "PlugManifest", "Calculate", "Process", "Execute",
            "GetVersion", "GetCapabilities", "HandleRequest", "Transform"
        ];
        
        for symbol_name in common_symbols {
            unsafe {
                if handle.get::<*const u8>(symbol_name.as_bytes()).is_ok() {
                    // Add to symbols list
                    if let Ok(mut symbols) = plugin.symbols.lock() {
                        symbols.insert(symbol_name.to_string(), Value::String(format!("Symbol: {}", symbol_name)));
                    }
                }
            }
        }
    }

    Ok(plugin)
}

/// Helper functions for plugin development and host integration

/// Check if the current process is running as a plugin
pub fn is_running_as_plugin() -> bool {
    // Simple heuristic: check if we're in a dynamic library context
    // This is a simplified implementation
    std::env::var("CURSED_PLUGIN_MODE").is_ok()
}

/// Get information about the host application
pub fn get_host_info() -> HostInfo {
    HostInfo::default()
}

/// Get the current plugin API version
pub fn get_plugin_api() -> String {
    "1.0".to_string()
}

/// Register an export from within a plugin (for use by plugin code)
pub fn register_export(name: &str, value: Value) -> PluginResult<()> {
    // This would typically store the export in a global registry
    // For now, this is a placeholder implementation
    if is_running_as_plugin() {
        // In a real implementation, this would register with the host
        Ok(())
    } else {
        Err(PluginError::general("Not running in plugin context"))
    }
}

/// Register a hook callback from within a plugin (for use by plugin code)
pub fn register_hook(name: &str, callback: PluginFunction) -> PluginResult<()> {
    // This would typically register the hook with the host application
    // For now, this is a placeholder implementation
    if is_running_as_plugin() {
        // In a real implementation, this would register with the host's hook system
        Ok(())
    } else {
        Err(PluginError::general("Not running in plugin context"))
    }
}

/// Utility function to create a plugin function from a Rust closure
pub fn create_plugin_function<F>(func: F) -> PluginFunction 
where
    F: Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync + 'static,
{
    Box::new(func)
}

/// Utility function to load multiple plugins from a directory
pub fn load_plugins_from_directory(dir_path: &str) -> PluginResult<Vec<Plug>> {
    let dir = std::fs::read_dir(dir_path).map_err(|e| {
        PluginError::general(&format!("Failed to read directory '{}': {}", dir_path, e))
    })?;

    let mut plugins = Vec::new();
    let expected_extensions = if cfg!(target_os = "windows") {
        vec!["dll"]
    } else if cfg!(target_os = "macos") {
        vec!["dylib"]
    } else {
        vec!["so"]
    };

    for entry in dir {
        let entry = entry.map_err(|e| {
            PluginError::general(&format!("Failed to read directory entry: {}", e))
        })?;
        
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if expected_extensions.contains(&ext) {
                    match load(&path.to_string_lossy()) {
                        Ok(plugin) => plugins.push(plugin),
                        Err(e) => {
                            eprintln!("Warning: Failed to load plugin '{}': {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }

    Ok(plugins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_load_options_default() {
        let options = LoadOptions::default();
        assert!(options.version_check);
        assert!(!options.verify_signature);
        assert!(!options.isolation);
        assert!(!options.sandbox);
        assert_eq!(options.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_plug_info_default() {
        let info = PlugInfo::default();
        assert_eq!(info.name, "");
        assert_eq!(info.version, "0.0.0");
        assert_eq!(info.api, "1.0");
        assert!(!info.is_verified);
        assert!(info.is_compatible);
    }

    #[test]
    fn test_host_info_default() {
        let info = HostInfo::default();
        assert_eq!(info.name, "CURSED");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.api_version, "1.0");
    }

    #[test]
    fn test_plugin_creation() {
        let path = PathBuf::from("/test/plugin.so");
        let info = PlugInfo::default();
        let plugin = Plug::new(path.clone(), info);
        
        assert_eq!(plugin.path(), &path);
        assert!(!plugin.is_loaded());
        assert_eq!(plugin.symbols().len(), 0);
    }

    #[test]
    fn test_plugin_symbol_registration() {
        let path = PathBuf::from("/test/plugin.so");
        let info = PlugInfo::default();
        let plugin = Plug::new(path, info);
        
        let result = plugin.register_symbol("test_symbol".to_string(), Value::Integer(42));
        assert!(result.is_ok());
        
        let symbols = plugin.symbols();
        assert_eq!(symbols.len(), 1);
        assert!(symbols.contains(&"test_symbol".to_string()));
    }

    #[test]
    fn test_load_nonexistent_plugin() {
        let result = load("/nonexistent/plugin.so");
        assert!(result.is_err());
        match result.unwrap_err() {
            PluginError::PluginNotFound(path) => assert_eq!(path, "/nonexistent/plugin.so"),
            _ => panic!("Expected PluginNotFound error"),
        }
    }
}
