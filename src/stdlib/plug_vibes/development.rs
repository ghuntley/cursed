use crate::error::CursedError;
/// Plugin development utilities and helper functions
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
// use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
// use crate::stdlib::plug_vibes::plug::{PlugInfo, HostInfo};
// use crate::stdlib::value::Value;

/// Development configuration
#[derive(Debug, Clone)]
pub struct DevelopmentConfig {
impl Default for DevelopmentConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Global plugin exports registry
static PLUGIN_EXPORTS: OnceLock<Arc<Mutex<HashMap<String, Value>>>> = OnceLock::new();

/// Global plugin hooks registry
static PLUGIN_HOOKS: OnceLock<Arc<Mutex<HashMap<String, Box<dyn Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync>>>>> = OnceLock::new();

/// Global host information
static HOST_INFO: OnceLock<Arc<Mutex<HostInfo>>> = OnceLock::new();

/// Global plugin API version
static PLUGIN_API_VERSION: &str = "1.0";

/// Global running state
static IS_RUNNING_AS_PLUGIN: OnceLock<Arc<Mutex<bool>>> = OnceLock::new();

/// Plugin capabilities interface
pub trait PlugCapabilities: Send + Sync {
    /// Get the list of capabilities this plugin provides
    fn capabilities(&self) -> Vec<String>;

    /// Check if this plugin has a specific capability
    fn has_capability(&self, name: &str) -> bool {
        self.capabilities().contains(&name.to_string())
    }
}

/// Default plugin capabilities implementation
#[derive(Debug, Clone, Default)]
pub struct DefaultPlugCapabilities {
impl DefaultPlugCapabilities {
    /// Create new capabilities with the given list
    pub fn new(capabilities: Vec<String>) -> Self {
        Self { capabilities }
    }

    /// Add a capability
    pub fn add_capability(&mut self, capability: &str) {
        if !self.capabilities.contains(&capability.to_string()) {
            self.capabilities.push(capability.to_string());
        }
    }

    /// Remove a capability
    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }
}

impl PlugCapabilities for DefaultPlugCapabilities {
    fn capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }
}

/// Plugin development context
pub struct PluginContext {
impl PluginContext {
    /// Create a new plugin context
    pub fn new(name: &str, version: &str) -> Self {
        Self {
        }
    }

    /// Export a value from this plugin
    pub fn export(&mut self, name: &str, value: Value) -> PluginResult<()> {
        self.exports.insert(name.to_string(), value);
        register_export(name, value)?;
        Ok(())
    /// Get an exported value
    pub fn get_export(&self, name: &str) -> Option<&Value> {
        self.exports.get(name)
    /// Set capabilities for this plugin
    pub fn set_capabilities(&mut self, capabilities: Box<dyn PlugCapabilities>) {
        self.capabilities = capabilities;
    /// Mark plugin as initialized
    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    /// Check if plugin is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Plugin manifest function that must be exported by all plugins
/// This is a helper for creating plugin manifests
pub fn create_plugin_manifest(
) -> PlugInfo {
    PlugInfo {
    }
}

/// Check if currently running as a plugin
pub fn is_running_as_plugin() -> bool {
    IS_RUNNING_AS_PLUGIN
        .get_or_init(|| Arc::new(Mutex::new(false)))
        .lock()
        .map(|running| *running)
        .unwrap_or(false)
/// Set the running as plugin state (used by plugin loaders)
pub fn set_running_as_plugin(running: bool) -> PluginResult<()> {
    let running_state = IS_RUNNING_AS_PLUGIN
        .get_or_init(|| Arc::new(Mutex::new(false)));
    
    let mut state = running_state.lock().map_err(|_| {
        PluginError::general("Failed to acquire running state lock")
    })?;
    
    *state = running;
    Ok(())
/// Get host application information
pub fn get_host_info() -> HostInfo {
    HOST_INFO
        .get_or_init(|| Arc::new(Mutex::new(HostInfo::default())))
        .lock()
        .map(|info| info.clone())
        .unwrap_or_else(|_| HostInfo::default())
/// Set host information (used by host applications)
pub fn set_host_info(info: HostInfo) -> PluginResult<()> {
    let host_info = HOST_INFO
        .get_or_init(|| Arc::new(Mutex::new(HostInfo::default())));
    
    let mut current_info = host_info.lock().map_err(|_| {
        PluginError::general("Failed to acquire host info lock")
    })?;
    
    *current_info = info;
    Ok(())
/// Get the plugin API version
pub fn get_plugin_api() -> String {
    PLUGIN_API_VERSION.to_string()
/// Register an export from a plugin
pub fn register_export(name: &str, value: Value) -> PluginResult<()> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut export_map = exports.lock().map_err(|_| {
        PluginError::general("Failed to acquire exports lock")
    })?;
    
    export_map.insert(name.to_string(), value);
    Ok(())
/// Get an exported value by name
pub fn get_export(name: &str) -> Option<Value> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    exports.lock()
        .ok()?
        .get(name)
        .cloned()
/// List all exported values
pub fn list_exports() -> Vec<String> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    exports.lock()
        .map(|map| map.keys().cloned().collect())
        .unwrap_or_else(|_| Vec::new())
/// Clear all exports
pub fn clear_exports() -> PluginResult<()> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut export_map = exports.lock().map_err(|_| {
        PluginError::general("Failed to acquire exports lock")
    })?;
    
    export_map.clear();
    Ok(())
/// Register a hook callback
pub fn register_hook<F>(name: &str, callback: F) -> PluginResult<()>
where
{
    let hooks = PLUGIN_HOOKS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut hook_map = hooks.lock().map_err(|_| {
        PluginError::general("Failed to acquire hooks lock")
    })?;
    
    hook_map.insert(name.to_string(), Box::new(callback));
    Ok(())
/// Call a registered hook
pub fn call_hook(name: &str, args: &[Value]) -> PluginResult<Vec<Value>> {
    let hooks = PLUGIN_HOOKS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let hook_map = hooks.lock().map_err(|_| {
        PluginError::general("Failed to acquire hooks lock")
    })?;
    
    if let Some(callback) = hook_map.get(name) {
        callback(args)
    } else {
        Err(PluginError::hook_error(&format!("Hook '{}' not found", name)))
    }
}

/// List all registered hooks
pub fn list_hooks() -> Vec<String> {
    let hooks = PLUGIN_HOOKS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    hooks.lock()
        .map(|map| map.keys().cloned().collect())
        .unwrap_or_else(|_| Vec::new())
/// Clear all hooks
pub fn clear_hooks() -> PluginResult<()> {
    let hooks = PLUGIN_HOOKS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut hook_map = hooks.lock().map_err(|_| {
        PluginError::general("Failed to acquire hooks lock")
    })?;
    
    hook_map.clear();
    Ok(())
/// Plugin initialization helper
pub fn initialize_plugin(context: &mut PluginContext) -> PluginResult<()> {
    if context.is_initialized() {
        return Err(PluginError::initialization_failed("Plugin already initialized"));
    // Set running as plugin state
    set_running_as_plugin(true)?;

    // Register basic exports
    register_export("plugin_name", Value::String(context.name.clone()))?;
    register_export("plugin_version", Value::String(context.version.clone()))?;
    register_export("plugin_api", Value::String(get_plugin_api()))?;

    // Mark as initialized
    context.mark_initialized();

    Ok(())
/// Plugin cleanup helper
pub fn cleanup_plugin(context: &mut PluginContext) -> PluginResult<()> {
    if !context.is_initialized() {
        return Ok(());
    // Clear exports for this plugin
    for export_name in context.exports.keys() {
        // In a real implementation, we'd track which exports belong to which plugin
        // For now, we'll just clear the export from the global registry
        let exports = PLUGIN_EXPORTS
            .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
        
        if let Ok(mut export_map) = exports.lock() {
            export_map.remove(export_name);
        }
    }

    // Clear plugin exports
    context.exports.clear();

    // Mark as not initialized
    context.initialized = false;

    Ok(())
/// Validate plugin compatibility with host
pub fn validate_plugin_compatibility(plugin_info: &PlugInfo) -> PluginResult<()> {
    let host_info = get_host_info();

    // Check API compatibility
    if plugin_info.api != host_info.api_version {
        return Err(PluginError::version_incompatible(&format!(
            plugin_info.api, host_info.api_version
        )));
    // Check platform compatibility
    let current_platform = std::env::consts::OS;
    // For now, assume all plugins are compatible with current platform
    // In a real implementation, check plugin metadata for platform requirements

    Ok(())
/// Create a simple capability checker
pub fn create_capability_checker(capabilities: Vec<String>) -> Box<dyn PlugCapabilities> {
    Box::new(DefaultPlugCapabilities::new(capabilities))
/// Development utilities for testing plugins
pub mod testing {
    use super::*;

    /// Mock host environment for testing
    pub struct MockHost {
    impl MockHost {
        pub fn new() -> Self {
            Self {
            }
        }

        pub fn with_name(mut self, name: &str) -> Self {
            self.info.name = name.to_string();
            self
        pub fn with_version(mut self, version: &str) -> Self {
            self.info.version = version.to_string();
            self
        pub fn setup(&self) -> PluginResult<()> {
            set_host_info(self.info.clone())?;
            set_running_as_plugin(false)?;
            Ok(())
        pub fn teardown(&self) -> PluginResult<()> {
            clear_exports()?;
            clear_hooks()?;
            set_running_as_plugin(false)?;
            Ok(())
        }
    }

    impl Default for MockHost {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Test plugin context
    pub fn create_test_plugin(name: &str, version: &str) -> PluginContext {
        let mut context = PluginContext::new(name, version);
        let capabilities = create_capability_checker(vec![
        ]);
        context.set_capabilities(capabilities);
        context
    /// Simulate plugin loading
    pub fn simulate_plugin_load(context: &mut PluginContext) -> PluginResult<()> {
        initialize_plugin(context)?;
        
        // Add some test exports
        context.export("test_function", Value::String("test_implementation".to_string()))?;
        
        Ok(())
    /// Simulate plugin unloading
    pub fn simulate_plugin_unload(context: &mut PluginContext) -> PluginResult<()> {
        cleanup_plugin(context)
    }
}

/// Scaffold a new plugin with the given name and directory
pub fn scaffold_plugin(name: &str, target_dir: &str) -> PluginResult<PlugInfo> {
    // Create basic plugin info
    let info = PlugInfo {
        ..Default::default()
    
    // In a real implementation, this would create:
    // - Plugin directory structure
    // - Template files (main.cursed, Cargo.toml, etc.)
    // - Build scripts
    // - Documentation templates
    
    // For now, just return the plugin info
    Ok(info)
