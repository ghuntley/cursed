/// Plugin development utilities and helper functions
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
use crate::stdlib::plug_vibes::plug::{PlugInfo, HostInfo};
use crate::stdlib::value::Value;

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
    capabilities: Vec<String>,
}

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
    pub name: String,
    pub version: String,
    pub exports: HashMap<String, Value>,
    pub capabilities: Box<dyn PlugCapabilities>,
    pub initialized: bool,
}

impl PluginContext {
    /// Create a new plugin context
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            exports: HashMap::new(),
            capabilities: Box::new(DefaultPlugCapabilities::default()),
            initialized: false,
        }
    }

    /// Export a value from this plugin
    pub fn export(&mut self, name: &str, value: Value) -> PluginResult<()> {
        self.exports.insert(name.to_string(), value);
        register_export(name, value)?;
        Ok(())
    }

    /// Get an exported value
    pub fn get_export(&self, name: &str) -> Option<&Value> {
        self.exports.get(name)
    }

    /// Set capabilities for this plugin
    pub fn set_capabilities(&mut self, capabilities: Box<dyn PlugCapabilities>) {
        self.capabilities = capabilities;
    }

    /// Mark plugin as initialized
    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    }

    /// Check if plugin is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Plugin manifest function that must be exported by all plugins
/// This is a helper for creating plugin manifests
pub fn create_plugin_manifest(
    name: &str,
    version: &str,
    api: &str,
    author: &str,
    description: &str,
    capabilities: Vec<String>,
) -> PlugInfo {
    PlugInfo {
        name: name.to_string(),
        version: version.to_string(),
        api: api.to_string(),
        author: author.to_string(),
        description: description.to_string(),
        build_time: std::time::SystemTime::now(),
        dependencies: Vec::new(),
        capabilities,
        imports: Vec::new(),
        exports: Vec::new(),
        signature: String::new(),
        is_verified: false,
        is_compatible: true,
    }
}

/// Check if currently running as a plugin
pub fn is_running_as_plugin() -> bool {
    IS_RUNNING_AS_PLUGIN
        .get_or_init(|| Arc::new(Mutex::new(false)))
        .lock()
        .map(|running| *running)
        .unwrap_or(false)
}

/// Set the running as plugin state (used by plugin loaders)
pub fn set_running_as_plugin(running: bool) -> PluginResult<()> {
    let running_state = IS_RUNNING_AS_PLUGIN
        .get_or_init(|| Arc::new(Mutex::new(false)));
    
    let mut state = running_state.lock().map_err(|_| {
        PluginError::general("Failed to acquire running state lock")
    })?;
    
    *state = running;
    Ok(())
}

/// Get host application information
pub fn get_host_info() -> HostInfo {
    HOST_INFO
        .get_or_init(|| Arc::new(Mutex::new(HostInfo::default())))
        .lock()
        .map(|info| info.clone())
        .unwrap_or_else(|_| HostInfo::default())
}

/// Set host information (used by host applications)
pub fn set_host_info(info: HostInfo) -> PluginResult<()> {
    let host_info = HOST_INFO
        .get_or_init(|| Arc::new(Mutex::new(HostInfo::default())));
    
    let mut current_info = host_info.lock().map_err(|_| {
        PluginError::general("Failed to acquire host info lock")
    })?;
    
    *current_info = info;
    Ok(())
}

/// Get the plugin API version
pub fn get_plugin_api() -> String {
    PLUGIN_API_VERSION.to_string()
}

/// Register an export from a plugin
pub fn register_export(name: &str, value: Value) -> PluginResult<()> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut export_map = exports.lock().map_err(|_| {
        PluginError::general("Failed to acquire exports lock")
    })?;
    
    export_map.insert(name.to_string(), value);
    Ok(())
}

/// Get an exported value by name
pub fn get_export(name: &str) -> Option<Value> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    exports.lock()
        .ok()?
        .get(name)
        .cloned()
}

/// List all exported values
pub fn list_exports() -> Vec<String> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    exports.lock()
        .map(|map| map.keys().cloned().collect())
        .unwrap_or_else(|_| Vec::new())
}

/// Clear all exports
pub fn clear_exports() -> PluginResult<()> {
    let exports = PLUGIN_EXPORTS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut export_map = exports.lock().map_err(|_| {
        PluginError::general("Failed to acquire exports lock")
    })?;
    
    export_map.clear();
    Ok(())
}

/// Register a hook callback
pub fn register_hook<F>(name: &str, callback: F) -> PluginResult<()>
where
    F: Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync + 'static,
{
    let hooks = PLUGIN_HOOKS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut hook_map = hooks.lock().map_err(|_| {
        PluginError::general("Failed to acquire hooks lock")
    })?;
    
    hook_map.insert(name.to_string(), Box::new(callback));
    Ok(())
}

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
}

/// Clear all hooks
pub fn clear_hooks() -> PluginResult<()> {
    let hooks = PLUGIN_HOOKS
        .get_or_init(|| Arc::new(Mutex::new(HashMap::new())));
    
    let mut hook_map = hooks.lock().map_err(|_| {
        PluginError::general("Failed to acquire hooks lock")
    })?;
    
    hook_map.clear();
    Ok(())
}

/// Plugin initialization helper
pub fn initialize_plugin(context: &mut PluginContext) -> PluginResult<()> {
    if context.is_initialized() {
        return Err(PluginError::initialization_failed("Plugin already initialized"));
    }

    // Set running as plugin state
    set_running_as_plugin(true)?;

    // Register basic exports
    register_export("plugin_name", Value::String(context.name.clone()))?;
    register_export("plugin_version", Value::String(context.version.clone()))?;
    register_export("plugin_api", Value::String(get_plugin_api()))?;

    // Mark as initialized
    context.mark_initialized();

    Ok(())
}

/// Plugin cleanup helper
pub fn cleanup_plugin(context: &mut PluginContext) -> PluginResult<()> {
    if !context.is_initialized() {
        return Ok(());
    }

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
}

/// Validate plugin compatibility with host
pub fn validate_plugin_compatibility(plugin_info: &PlugInfo) -> PluginResult<()> {
    let host_info = get_host_info();

    // Check API compatibility
    if plugin_info.api != host_info.api_version {
        return Err(PluginError::version_incompatible(&format!(
            "Plugin API version {} incompatible with host API version {}",
            plugin_info.api, host_info.api_version
        )));
    }

    // Check platform compatibility
    let current_platform = std::env::consts::OS;
    // For now, assume all plugins are compatible with current platform
    // In a real implementation, check plugin metadata for platform requirements

    Ok(())
}

/// Create a simple capability checker
pub fn create_capability_checker(capabilities: Vec<String>) -> Box<dyn PlugCapabilities> {
    Box::new(DefaultPlugCapabilities::new(capabilities))
}

/// Development utilities for testing plugins
pub mod testing {
    use super::*;

    /// Mock host environment for testing
    pub struct MockHost {
        pub info: HostInfo,
        pub exports: HashMap<String, Value>,
    }

    impl MockHost {
        pub fn new() -> Self {
            Self {
                info: HostInfo::default(),
                exports: HashMap::new(),
            }
        }

        pub fn with_name(mut self, name: &str) -> Self {
            self.info.name = name.to_string();
            self
        }

        pub fn with_version(mut self, version: &str) -> Self {
            self.info.version = version.to_string();
            self
        }

        pub fn setup(&self) -> PluginResult<()> {
            set_host_info(self.info.clone())?;
            set_running_as_plugin(false)?;
            Ok(())
        }

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
            "test".to_string(),
            "mock".to_string(),
        ]);
        context.set_capabilities(capabilities);
        context
    }

    /// Simulate plugin loading
    pub fn simulate_plugin_load(context: &mut PluginContext) -> PluginResult<()> {
        initialize_plugin(context)?;
        
        // Add some test exports
        context.export("test_function", Value::String("test_implementation".to_string()))?;
        
        Ok(())
    }

    /// Simulate plugin unloading
    pub fn simulate_plugin_unload(context: &mut PluginContext) -> PluginResult<()> {
        cleanup_plugin(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::testing::*;

    #[test]
    fn test_plugin_context_creation() {
        let context = PluginContext::new("test_plugin", "1.0.0");
        
        assert_eq!(context.name, "test_plugin");
        assert_eq!(context.version, "1.0.0");
        assert!(!context.is_initialized());
        assert_eq!(context.exports.len(), 0);
    }

    #[test]
    fn test_plugin_context_export() {
        let mut context = PluginContext::new("test_plugin", "1.0.0");
        
        let result = context.export("test_export", Value::String("test_value".to_string()));
        assert!(result.is_ok());
        
        let exported = context.get_export("test_export");
        assert!(exported.is_some());
        assert_eq!(exported.unwrap(), &Value::String("test_value".to_string()));
    }

    #[test]
    fn test_create_plugin_manifest() {
        let manifest = create_plugin_manifest(
            "test_plugin",
            "1.0.0",
            "1.0",
            "Test Author",
            "A test plugin",
            vec!["test".to_string(), "utility".to_string()],
        );

        assert_eq!(manifest.name, "test_plugin");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.api, "1.0");
        assert_eq!(manifest.author, "Test Author");
        assert_eq!(manifest.description, "A test plugin");
        assert_eq!(manifest.capabilities.len(), 2);
        assert!(manifest.capabilities.contains(&"test".to_string()));
        assert!(manifest.capabilities.contains(&"utility".to_string()));
    }

    #[test]
    fn test_running_as_plugin_state() {
        // Initially not running as plugin
        assert!(!is_running_as_plugin());

        // Set running as plugin
        set_running_as_plugin(true).unwrap();
        assert!(is_running_as_plugin());

        // Set not running as plugin
        set_running_as_plugin(false).unwrap();
        assert!(!is_running_as_plugin());
    }

    #[test]
    fn test_host_info() {
        let original_info = get_host_info();
        
        let new_info = HostInfo {
            name: "Test Host".to_string(),
            version: "2.0.0".to_string(),
            api_version: "2.0".to_string(),
            platform: "test".to_string(),
            architecture: "test64".to_string(),
        };

        set_host_info(new_info.clone()).unwrap();
        let retrieved_info = get_host_info();
        
        assert_eq!(retrieved_info.name, "Test Host");
        assert_eq!(retrieved_info.version, "2.0.0");
        assert_eq!(retrieved_info.api_version, "2.0");
    }

    #[test]
    fn test_plugin_api() {
        let api = get_plugin_api();
        assert_eq!(api, "1.0");
    }

    #[test]
    fn test_exports() {
        // Clear any existing exports
        clear_exports().unwrap();

        // Register an export
        register_export("test_export", Value::String("test_value".to_string())).unwrap();
        
        // Retrieve the export
        let exported = get_export("test_export");
        assert!(exported.is_some());
        assert_eq!(exported.unwrap(), Value::String("test_value".to_string()));

        // List exports
        let exports = list_exports();
        assert_eq!(exports.len(), 1);
        assert!(exports.contains(&"test_export".to_string()));

        // Clear exports
        clear_exports().unwrap();
        let exports = list_exports();
        assert_eq!(exports.len(), 0);
    }

    #[test]
    fn test_hooks() {
        // Clear any existing hooks
        clear_hooks().unwrap();

        // Register a hook
        register_hook("test_hook", |args| {
            let mut result = args.to_vec();
            result.push(Value::String("hook_called".to_string()));
            Ok(result)
        }).unwrap();

        // Call the hook
        let input = vec![Value::String("input".to_string())];
        let output = call_hook("test_hook", &input).unwrap();
        
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], Value::String("input".to_string()));
        assert_eq!(output[1], Value::String("hook_called".to_string()));

        // List hooks
        let hooks = list_hooks();
        assert_eq!(hooks.len(), 1);
        assert!(hooks.contains(&"test_hook".to_string()));

        // Clear hooks
        clear_hooks().unwrap();
        let hooks = list_hooks();
        assert_eq!(hooks.len(), 0);
    }

    #[test]
    fn test_plugin_initialization() {
        clear_exports().unwrap();
        
        let mut context = PluginContext::new("test_plugin", "1.0.0");
        assert!(!context.is_initialized());

        let result = initialize_plugin(&mut context);
        assert!(result.is_ok());
        assert!(context.is_initialized());
        assert!(is_running_as_plugin());

        // Check that basic exports were registered
        assert!(get_export("plugin_name").is_some());
        assert!(get_export("plugin_version").is_some());
        assert!(get_export("plugin_api").is_some());
    }

    #[test]
    fn test_plugin_cleanup() {
        clear_exports().unwrap();

        let mut context = PluginContext::new("test_plugin", "1.0.0");
        initialize_plugin(&mut context).unwrap();
        
        // Add some exports
        context.export("custom_export", Value::String("custom_value".to_string())).unwrap();
        
        assert!(context.is_initialized());
        assert!(!context.exports.is_empty());

        let result = cleanup_plugin(&mut context);
        assert!(result.is_ok());
        assert!(!context.is_initialized());
        assert!(context.exports.is_empty());
    }

    #[test]
    fn test_capability_checker() {
        let capabilities = vec!["test".to_string(), "utility".to_string()];
        let checker = create_capability_checker(capabilities);

        assert!(checker.has_capability("test"));
        assert!(checker.has_capability("utility"));
        assert!(!checker.has_capability("nonexistent"));

        let caps = checker.capabilities();
        assert_eq!(caps.len(), 2);
        assert!(caps.contains(&"test".to_string()));
        assert!(caps.contains(&"utility".to_string()));
    }

    #[test]
    fn test_default_plug_capabilities() {
        let mut caps = DefaultPlugCapabilities::new(vec!["initial".to_string()]);
        
        assert!(caps.has_capability("initial"));
        assert!(!caps.has_capability("new"));

        caps.add_capability("new");
        assert!(caps.has_capability("new"));

        caps.remove_capability("initial");
        assert!(!caps.has_capability("initial"));
        assert!(caps.has_capability("new"));
    }

    #[test]
    fn test_validate_plugin_compatibility() {
        // Set up host info
        let host_info = HostInfo {
            name: "Test Host".to_string(),
            version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            platform: "linux".to_string(),
            architecture: "x86_64".to_string(),
        };
        set_host_info(host_info).unwrap();

        // Compatible plugin
        let compatible_plugin = PlugInfo {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            api: "1.0".to_string(),
            author: "Test".to_string(),
            description: "Test plugin".to_string(),
            build_time: std::time::SystemTime::now(),
            dependencies: Vec::new(),
            capabilities: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            signature: String::new(),
            is_verified: false,
            is_compatible: true,
        };

        let result = validate_plugin_compatibility(&compatible_plugin);
        assert!(result.is_ok());

        // Incompatible plugin (different API version)
        let mut incompatible_plugin = compatible_plugin.clone();
        incompatible_plugin.api = "2.0".to_string();

        let result = validate_plugin_compatibility(&incompatible_plugin);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_host() {
        let mock_host = MockHost::new()
            .with_name("Mock Host")
            .with_version("1.0.0");

        assert_eq!(mock_host.info.name, "Mock Host");
        assert_eq!(mock_host.info.version, "1.0.0");

        // Setup mock host
        mock_host.setup().unwrap();
        assert_eq!(get_host_info().name, "Mock Host");
        assert!(!is_running_as_plugin());

        // Teardown
        mock_host.teardown().unwrap();
    }

    #[test]
    fn test_create_test_plugin() {
        let mut plugin = create_test_plugin("test", "1.0.0");
        
        assert_eq!(plugin.name, "test");
        assert_eq!(plugin.version, "1.0.0");
        assert!(plugin.capabilities.has_capability("test"));
        assert!(plugin.capabilities.has_capability("mock"));
    }

    #[test]
    fn test_simulate_plugin_lifecycle() {
        clear_exports().unwrap();
        
        let mut plugin = create_test_plugin("test", "1.0.0");
        
        // Simulate loading
        simulate_plugin_load(&mut plugin).unwrap();
        assert!(plugin.is_initialized());
        assert!(is_running_as_plugin());
        assert!(get_export("test_function").is_some());

        // Simulate unloading
        simulate_plugin_unload(&mut plugin).unwrap();
        assert!(!plugin.is_initialized());
        assert!(plugin.exports.is_empty());
    }
}
