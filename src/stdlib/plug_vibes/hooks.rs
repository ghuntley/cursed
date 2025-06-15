/// Plugin hooks and extension points for host application integration
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};
use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
use crate::stdlib::plug_vibes::plug::Plug;
use crate::stdlib::value::Value;

/// Priority level for hook callbacks
pub type Priority = i32;

/// Hook callback function type
pub type HookCallback = Box<dyn Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync>;

/// Hook callback that can stop further execution
pub type HookCallbackWithBreak = Box<dyn Fn(&[Value]) -> PluginResult<(Vec<Value>, bool)> + Send + Sync>;

/// Hook registration information
#[derive(Clone)]
struct HookRegistration {
    plugin_name: String,
    priority: Priority,
    callback: Arc<HookCallback>,
}

/// Plugin hook system for extensibility
pub struct PlugHook {
    name: String,
    registrations: Arc<Mutex<BTreeMap<Priority, Vec<HookRegistration>>>>,
    enabled: Arc<Mutex<bool>>,
    call_count: Arc<Mutex<u64>>,
    error_count: Arc<Mutex<u64>>,
}

impl PlugHook {
    /// Create a new plugin hook with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            registrations: Arc::new(Mutex::new(BTreeMap::new())),
            enabled: Arc::new(Mutex::new(true)),
            call_count: Arc::new(Mutex::new(0)),
            error_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Get the hook name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Register a plugin with this hook
    pub fn register(&self, plugin: &Plug, priority: Priority) -> PluginResult<()> {
        let plugin_name = plugin.info().name.clone();
        
        // Create a callback that looks up the hook function in the plugin
        let callback = {
            let hook_name = self.name.clone();
            Arc::new(Box::new(move |args: &[Value]| -> PluginResult<Vec<Value>> {
                // In a real implementation, we'd call the plugin's hook function here
                // For now, we'll return the input arguments as-is
                Ok(args.to_vec())
            }) as HookCallback)
        };

        let registration = HookRegistration {
            plugin_name,
            priority,
            callback,
        };

        let mut registrations = self.registrations.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire registrations lock")
        })?;

        registrations.entry(priority)
            .or_insert_with(Vec::new)
            .push(registration);

        Ok(())
    }

    /// Register a custom callback function with this hook
    pub fn register_callback<F>(&self, plugin_name: &str, priority: Priority, callback: F) -> PluginResult<()>
    where
        F: Fn(&[Value]) -> PluginResult<Vec<Value>> + Send + Sync + 'static,
    {
        let registration = HookRegistration {
            plugin_name: plugin_name.to_string(),
            priority,
            callback: Arc::new(Box::new(callback)),
        };

        let mut registrations = self.registrations.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire registrations lock")
        })?;

        registrations.entry(priority)
            .or_insert_with(Vec::new)
            .push(registration);

        Ok(())
    }

    /// Unregister a plugin from this hook
    pub fn unregister(&self, plugin: &Plug) -> PluginResult<()> {
        let plugin_name = &plugin.info().name;
        self.unregister_by_name(plugin_name)
    }

    /// Unregister a plugin by name
    pub fn unregister_by_name(&self, plugin_name: &str) -> PluginResult<()> {
        let mut registrations = self.registrations.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire registrations lock")
        })?;

        // Remove all registrations for this plugin
        for (_, plugin_list) in registrations.iter_mut() {
            plugin_list.retain(|reg| reg.plugin_name != plugin_name);
        }

        // Clean up empty priority levels
        registrations.retain(|_, plugin_list| !plugin_list.is_empty());

        Ok(())
    }

    /// Call all registered hook callbacks with the given arguments
    pub fn call(&self, args: &[Value]) -> Vec<Value> {
        if !self.is_enabled() {
            return args.to_vec();
        }

        self.increment_call_count();

        let registrations = match self.registrations.lock() {
            Ok(regs) => regs,
            Err(_) => {
                self.increment_error_count();
                return args.to_vec();
            }
        };

        let mut current_args = args.to_vec();

        // Call hooks in priority order (highest priority first)
        for (_, plugin_list) in registrations.iter().rev() {
            for registration in plugin_list {
                match (registration.callback)(&current_args) {
                    Ok(result) => {
                        current_args = result;
                    }
                    Err(_) => {
                        self.increment_error_count();
                        // Continue with other callbacks on error
                    }
                }
            }
        }

        current_args
    }

    /// Call hooks until one returns a truthy value
    pub fn call_until_true(&self, args: &[Value]) -> (Vec<Value>, bool) {
        if !self.is_enabled() {
            return (args.to_vec(), false);
        }

        self.increment_call_count();

        let registrations = match self.registrations.lock() {
            Ok(regs) => regs,
            Err(_) => {
                self.increment_error_count();
                return (args.to_vec(), false);
            }
        };

        // Call hooks in priority order (highest priority first)
        for (_, plugin_list) in registrations.iter().rev() {
            for registration in plugin_list {
                match (registration.callback)(args) {
                    Ok(result) => {
                        // Check if result contains a truthy value
                        if result.iter().any(|v| is_truthy(v)) {
                            return (result, true);
                        }
                    }
                    Err(_) => {
                        self.increment_error_count();
                        // Continue with other callbacks on error
                    }
                }
            }
        }

        (args.to_vec(), false)
    }

    /// Call hooks until one returns an error
    pub fn call_until_error(&self, args: &[Value]) -> PluginResult<Vec<Value>> {
        if !self.is_enabled() {
            return Ok(args.to_vec());
        }

        self.increment_call_count();

        let registrations = self.registrations.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire registrations lock")
        })?;

        let mut current_args = args.to_vec();

        // Call hooks in priority order (highest priority first)
        for (_, plugin_list) in registrations.iter().rev() {
            for registration in plugin_list {
                match (registration.callback)(&current_args) {
                    Ok(result) => {
                        current_args = result;
                    }
                    Err(e) => {
                        self.increment_error_count();
                        return Err(e);
                    }
                }
            }
        }

        Ok(current_args)
    }

    /// Enable or disable this hook
    pub fn set_enabled(&self, enabled: bool) -> PluginResult<()> {
        let mut hook_enabled = self.enabled.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire enabled lock")
        })?;
        *hook_enabled = enabled;
        Ok(())
    }

    /// Check if this hook is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.lock()
            .map(|enabled| *enabled)
            .unwrap_or(false)
    }

    /// Get the number of registered plugins
    pub fn registration_count(&self) -> usize {
        self.registrations.lock()
            .map(|regs| regs.values().map(|list| list.len()).sum())
            .unwrap_or(0)
    }

    /// Get call statistics
    pub fn get_call_count(&self) -> u64 {
        self.call_count.lock()
            .map(|count| *count)
            .unwrap_or(0)
    }

    /// Get error statistics
    pub fn get_error_count(&self) -> u64 {
        self.error_count.lock()
            .map(|count| *count)
            .unwrap_or(0)
    }

    /// Get list of registered plugin names with their priorities
    pub fn get_registered_plugins(&self) -> Vec<(String, Priority)> {
        self.registrations.lock()
            .map(|regs| {
                regs.iter()
                    .flat_map(|(priority, list)| {
                        list.iter().map(move |reg| (reg.plugin_name.clone(), *priority))
                    })
                    .collect()
            })
            .unwrap_or_else(|_| Vec::new())
    }

    /// Clear all registrations
    pub fn clear(&self) -> PluginResult<()> {
        let mut registrations = self.registrations.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire registrations lock")
        })?;
        registrations.clear();
        Ok(())
    }

    fn increment_call_count(&self) {
        if let Ok(mut count) = self.call_count.lock() {
            *count += 1;
        }
    }

    fn increment_error_count(&self) {
        if let Ok(mut count) = self.error_count.lock() {
            *count += 1;
        }
    }
}

/// Extension point interface for host applications
pub trait ExtensionPoint: Send + Sync {
    /// Get the name of this extension point
    fn name(&self) -> &str;

    /// Register an extension with this point
    fn register(&self, extension: Box<dyn Any + Send + Sync>) -> PluginResult<()>;

    /// Unregister an extension by its type
    fn unregister(&self, type_id: TypeId) -> PluginResult<()>;

    /// Get all registered extensions
    fn get_extensions(&self) -> Vec<Box<dyn Any + Send + Sync>>;

    /// Get the number of registered extensions
    fn extension_count(&self) -> usize;

    /// Check if an extension of a specific type is registered
    fn has_extension(&self, type_id: TypeId) -> bool;
}

/// Generic extension point implementation
pub struct GenericExtensionPoint {
    name: String,
    extensions: Arc<Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl GenericExtensionPoint {
    /// Create a new extension point
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            extensions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get an extension by type
    pub fn get_extension<T: 'static>(&self) -> Option<Arc<T>> {
        let extensions = self.extensions.lock().ok()?;
        let type_id = TypeId::of::<T>();
        
        // This is a simplified implementation - in practice you'd need
        // more sophisticated type handling
        None
    }

    /// Register a typed extension
    pub fn register_typed<T: Send + Sync + 'static>(&self, extension: T) -> PluginResult<()> {
        let mut extensions = self.extensions.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire extensions lock")
        })?;
        
        let type_id = TypeId::of::<T>();
        extensions.insert(type_id, Box::new(extension));
        Ok(())
    }
}

impl ExtensionPoint for GenericExtensionPoint {
    fn name(&self) -> &str {
        &self.name
    }

    fn register(&self, extension: Box<dyn Any + Send + Sync>) -> PluginResult<()> {
        let mut extensions = self.extensions.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire extensions lock")
        })?;
        
        // Note: This is simplified - getting TypeId from a trait object is complex
        // In a real implementation, you'd need a registration system with explicit type info
        let type_id = extension.as_ref().type_id();
        extensions.insert(type_id, extension);
        Ok(())
    }

    fn unregister(&self, type_id: TypeId) -> PluginResult<()> {
        let mut extensions = self.extensions.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire extensions lock")
        })?;
        
        extensions.remove(&type_id);
        Ok(())
    }

    fn get_extensions(&self) -> Vec<Box<dyn Any + Send + Sync>> {
        self.extensions.lock()
            .map(|exts| {
                // Note: This would require cloning which isn't possible with trait objects
                // In practice, you'd return references or use a different approach
                Vec::new()
            })
            .unwrap_or_else(|_| Vec::new())
    }

    fn extension_count(&self) -> usize {
        self.extensions.lock()
            .map(|exts| exts.len())
            .unwrap_or(0)
    }

    fn has_extension(&self, type_id: TypeId) -> bool {
        self.extensions.lock()
            .map(|exts| exts.contains_key(&type_id))
            .unwrap_or(false)
    }
}

/// Create a new extension point with the given name and extension type
pub fn new_extension_point(name: &str) -> Box<dyn ExtensionPoint> {
    Box::new(GenericExtensionPoint::new(name))
}

/// Hook manager for managing multiple hooks
pub struct HookManager {
    hooks: Arc<Mutex<HashMap<String, Arc<PlugHook>>>>,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            hooks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create or get a hook by name
    pub fn get_or_create_hook(&self, name: &str) -> PluginResult<Arc<PlugHook>> {
        let mut hooks = self.hooks.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire hooks lock")
        })?;

        if let Some(hook) = hooks.get(name) {
            Ok(Arc::clone(hook))
        } else {
            let hook = Arc::new(PlugHook::new(name));
            hooks.insert(name.to_string(), Arc::clone(&hook));
            Ok(hook)
        }
    }

    /// Get a hook by name
    pub fn get_hook(&self, name: &str) -> Option<Arc<PlugHook>> {
        self.hooks.lock()
            .ok()?
            .get(name)
            .map(Arc::clone)
    }

    /// Remove a hook by name
    pub fn remove_hook(&self, name: &str) -> PluginResult<()> {
        let mut hooks = self.hooks.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire hooks lock")
        })?;
        
        hooks.remove(name);
        Ok(())
    }

    /// List all hook names
    pub fn list_hooks(&self) -> Vec<String> {
        self.hooks.lock()
            .map(|hooks| hooks.keys().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Clear all hooks
    pub fn clear(&self) -> PluginResult<()> {
        let mut hooks = self.hooks.lock().map_err(|_| {
            PluginError::hook_error("Failed to acquire hooks lock")
        })?;
        
        hooks.clear();
        Ok(())
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to check if a value is truthy
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(b) => *b,
        Value::Integer(i) => *i != 0,
        Value::Float(f) => *f != 0.0,
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
        Value::Null => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::plug_vibes::plug::{Plug, PlugInfo};
    use std::path::PathBuf;

    fn create_test_plugin(name: &str) -> Plug {
        let mut info = PlugInfo::default();
        info.name = name.to_string();
        Plug::new(PathBuf::from(format!("/test/{}.so", name)), info)
    }

    #[test]
    fn test_hook_creation() {
        let hook = PlugHook::new("test_hook");
        assert_eq!(hook.name(), "test_hook");
        assert!(hook.is_enabled());
        assert_eq!(hook.registration_count(), 0);
        assert_eq!(hook.get_call_count(), 0);
        assert_eq!(hook.get_error_count(), 0);
    }

    #[test]
    fn test_hook_registration() {
        let hook = PlugHook::new("test_hook");
        let plugin = create_test_plugin("test_plugin");

        let result = hook.register(&plugin, 10);
        assert!(result.is_ok());
        assert_eq!(hook.registration_count(), 1);

        let registered = hook.get_registered_plugins();
        assert_eq!(registered.len(), 1);
        assert_eq!(registered[0].0, "test_plugin");
        assert_eq!(registered[0].1, 10);
    }

    #[test]
    fn test_hook_callback_registration() {
        let hook = PlugHook::new("test_hook");

        let result = hook.register_callback("test_plugin", 5, |args| {
            Ok(args.to_vec())
        });
        assert!(result.is_ok());
        assert_eq!(hook.registration_count(), 1);
    }

    #[test]
    fn test_hook_unregistration() {
        let hook = PlugHook::new("test_hook");
        let plugin = create_test_plugin("test_plugin");

        hook.register(&plugin, 10).unwrap();
        assert_eq!(hook.registration_count(), 1);

        let result = hook.unregister(&plugin);
        assert!(result.is_ok());
        assert_eq!(hook.registration_count(), 0);
    }

    #[test]
    fn test_hook_call() {
        let hook = PlugHook::new("test_hook");
        
        hook.register_callback("test_plugin", 10, |args| {
            let mut result = args.to_vec();
            result.push(Value::String("modified".to_string()));
            Ok(result)
        }).unwrap();

        let input = vec![Value::String("input".to_string())];
        let output = hook.call(&input);
        
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], Value::String("input".to_string()));
        assert_eq!(output[1], Value::String("modified".to_string()));
        assert_eq!(hook.get_call_count(), 1);
    }

    #[test]
    fn test_hook_enable_disable() {
        let hook = PlugHook::new("test_hook");
        
        assert!(hook.is_enabled());
        
        hook.set_enabled(false).unwrap();
        assert!(!hook.is_enabled());
        
        hook.set_enabled(true).unwrap();
        assert!(hook.is_enabled());
    }

    #[test]
    fn test_hook_call_until_true() {
        let hook = PlugHook::new("test_hook");
        
        hook.register_callback("plugin1", 10, |_args| {
            Ok(vec![Value::Boolean(false)])
        }).unwrap();
        
        hook.register_callback("plugin2", 5, |_args| {
            Ok(vec![Value::Boolean(true)])
        }).unwrap();

        let input = vec![Value::String("test".to_string())];
        let (output, found_true) = hook.call_until_true(&input);
        
        assert!(found_true);
        assert_eq!(output, vec![Value::Boolean(false)]);
    }

    #[test]
    fn test_extension_point_creation() {
        let ext_point = GenericExtensionPoint::new("test_extension");
        assert_eq!(ext_point.name(), "test_extension");
        assert_eq!(ext_point.extension_count(), 0);
    }

    #[test]
    fn test_extension_point_typed_registration() {
        let ext_point = GenericExtensionPoint::new("test_extension");
        
        let result = ext_point.register_typed("test_string".to_string());
        assert!(result.is_ok());
        assert_eq!(ext_point.extension_count(), 1);
    }

    #[test]
    fn test_hook_manager() {
        let manager = HookManager::new();
        
        // Create a hook
        let hook1 = manager.get_or_create_hook("hook1").unwrap();
        assert_eq!(hook1.name(), "hook1");
        
        // Get the same hook again
        let hook1_again = manager.get_or_create_hook("hook1").unwrap();
        assert_eq!(hook1.name(), hook1_again.name());
        
        // List hooks
        let hooks = manager.list_hooks();
        assert_eq!(hooks.len(), 1);
        assert!(hooks.contains(&"hook1".to_string()));
        
        // Remove hook
        manager.remove_hook("hook1").unwrap();
        let hooks = manager.list_hooks();
        assert_eq!(hooks.len(), 0);
    }

    #[test]
    fn test_is_truthy() {
        assert!(is_truthy(&Value::Boolean(true)));
        assert!(!is_truthy(&Value::Boolean(false)));
        assert!(is_truthy(&Value::Integer(1)));
        assert!(!is_truthy(&Value::Integer(0)));
        assert!(is_truthy(&Value::Float(1.0)));
        assert!(!is_truthy(&Value::Float(0.0)));
        assert!(is_truthy(&Value::String("hello".to_string())));
        assert!(!is_truthy(&Value::String("".to_string())));
        assert!(!is_truthy(&Value::Null));
    }

    #[test]
    fn test_hook_clear() {
        let hook = PlugHook::new("test_hook");
        let plugin = create_test_plugin("test_plugin");

        hook.register(&plugin, 10).unwrap();
        assert_eq!(hook.registration_count(), 1);

        hook.clear().unwrap();
        assert_eq!(hook.registration_count(), 0);
    }
}
