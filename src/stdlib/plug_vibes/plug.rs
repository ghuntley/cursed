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

    /// Look up a function by name
    pub fn lookup_func(&self, func_name: &str) -> PluginResult<&PluginFunction> {
        let functions = self.functions.lock().map_err(|_| {
            PluginError::general("Failed to acquire functions lock")
        })?;

        // Note: This is simplified - in a real implementation we'd need to handle
        // the lifetime issues more carefully
        Err(PluginError::function_not_found(func_name))
    }

    /// Look up and bind a symbol to a specific type
    pub fn lookup_symbol<T>(&self, symbol_name: &str) -> PluginResult<T> 
    where
        T: Clone + 'static,
    {
        let value = self.lookup(symbol_name)?;
        
        // This is a simplified implementation - in practice you'd need proper
        // type conversion from Value to T
        Err(PluginError::general("Symbol type conversion not implemented"))
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

    /// Close the plugin and clean up resources
    pub fn close(&mut self) -> PluginResult<()> {
        if !self.is_loaded {
            return Ok(());
        }

        // Call cleanup function if it exists
        if let Ok(_) = self.lookup("Cleanup") {
            // In a real implementation, we'd call the cleanup function here
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

    // Load the dynamic library
    let library = unsafe {
        libloading::Library::new(&path_buf).map_err(|e| {
            PluginError::load_error(&format!("Failed to load library: {}", e))
        })?
    };

    // Get plugin manifest
    let manifest_func: libloading::Symbol<unsafe extern "C" fn() -> PlugInfo> = unsafe {
        library.get(b"PlugManifest").map_err(|_| {
            PluginError::symbol_not_found("PlugManifest")
        })?
    };

    let info = unsafe { manifest_func() };

    // Verify version compatibility if requested
    if options.version_check {
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
            return Err(PluginError::initialization_failed(&format!("Init returned: {}", result)));
        }
    }

    Ok(plugin)
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
