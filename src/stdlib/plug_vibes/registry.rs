use crate::error::CursedError;
/// Plugin registry for managing loaded plugins
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;
// use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
// use crate::stdlib::plug_vibes::plug::{Plug, LoadOptions, load_with_options};

/// Thread-safe plugin registry
pub struct PlugRegistry {
    plugins: Arc<Mutex<HashMap<String, Arc<Mutex<Plug>>>>>,
}

impl PlugRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a plugin with a given name
    pub fn register(&self, name: &str, plugin: Plug) -> PluginResult<()> {
        let mut plugins = self.plugins.lock().map_err(|_| {
            PluginError::registry_error("Failed to acquire registry lock")
        })?;

        if plugins.contains_key(name) {
            return Err(PluginError::already_loaded(name));
        }

        plugins.insert(name.to_string(), Arc::new(Mutex::new(plugin)));
        Ok(())
    }

    /// Unregister a plugin by name
    pub fn unregister(&self, name: &str) -> PluginResult<()> {
        let mut plugins = self.plugins.lock().map_err(|_| {
            PluginError::registry_error("Failed to acquire registry lock")
        })?;

        if let Some(plugin_arc) = plugins.remove(name) {
            // Close the plugin
            if let Ok(mut plugin) = plugin_arc.lock() {
                plugin.close()?;
            }
            Ok(())
        } else {
            Err(PluginError::not_loaded(name))
        }
    }

    /// Get a plugin by name
    pub fn get(&self, name: &str) -> PluginResult<Arc<Mutex<Plug>>> {
        let plugins = self.plugins.lock().map_err(|_| {
            PluginError::registry_error("Failed to acquire registry lock")
        })?;

        plugins.get(name)
            .cloned()
            .ok_or_else(|| PluginError::not_loaded(name))
    }

    /// Check if a plugin is registered
    pub fn contains(&self, name: &str) -> bool {
        self.plugins.lock()
            .map(|plugins| plugins.contains_key(name))
            .unwrap_or(false)
    }

    /// List all registered plugin names
    pub fn list(&self) -> Vec<String> {
        self.plugins.lock()
            .map(|plugins| plugins.keys().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Get the number of registered plugins
    pub fn len(&self) -> usize {
        self.plugins.lock()
            .map(|plugins| plugins.len())
            .unwrap_or(0)
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Load and register a plugin from a file path
    pub fn load_and_register(&self, path: &str, name: &str) -> PluginResult<Arc<Mutex<Plug>>> {
        self.load_and_register_with_options(path, name, LoadOptions::default())
    }

    /// Load and register a plugin with specific options
    pub fn load_and_register_with_options(
        &self, 
        path: &str, 
        name: &str, 
        options: LoadOptions
    ) -> PluginResult<Arc<Mutex<Plug>>> {
        // Check if already registered
        if self.contains(name) {
            return Err(PluginError::already_loaded(name));
        }

        // Load the plugin
        let plugin = load_with_options(path, options)?;
        
        // Register it
        self.register(name, plugin)?;
        
        // Return the registered plugin
        self.get(name)
    }

    /// Load all plugins from a directory
    pub fn load_all(&self, directory: &str) -> PluginResult<HashMap<String, Arc<Mutex<Plug>>>> {
        self.load_all_with_options(directory, LoadOptions::default())
    }

    /// Load all plugins from a directory with specific options
    pub fn load_all_with_options(
        &self, 
        directory: &str, 
        options: LoadOptions
    ) -> PluginResult<HashMap<String, Arc<Mutex<Plug>>>> {
        let dir_path = Path::new(directory);
        
        if !dir_path.exists() {
            return Err(PluginError::plugin_not_found(directory));
        }

        if !dir_path.is_dir() {
            return Err(PluginError::load_error(&format!("{} is not a directory", directory)));
        }

        let mut loaded_plugins = HashMap::new();
        let entries = std::fs::read_dir(dir_path).map_err(|e| {
            PluginError::load_error(&format!("Failed to read directory {}: {}", directory, e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                PluginError::load_error(&format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Only load files with known plugin extensions
                    if extension == "so" || extension == "dll" || extension == "dylib" {
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(name) = file_stem.to_str() {
                                match self.load_and_register_with_options(
                                    path.to_str().unwrap(),
                                    name,
                                    options.clone()
                                ) {
                                    Ok(plugin) => {
                                        loaded_plugins.insert(name.to_string(), plugin);
                                    }
                                    Err(e) => {
                                        // Log error but continue loading other plugins
                                        eprintln!("Failed to load plugin {}: {}", name, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(loaded_plugins)
    }

    /// Reload a plugin by name
    pub fn reload(&self, name: &str) -> PluginResult<Arc<Mutex<Plug>>> {
        // Get the current plugin to get its path
        let plugin_arc = self.get(name)?;
        let path = {
            let plugin = plugin_arc.lock().map_err(|_| {
                PluginError::registry_error("Failed to acquire plugin lock")
            })?;
            plugin.path().to_path_buf()
        };

        // Unregister the old plugin
        self.unregister(name)?;

        // Load and register the new version
        self.load_and_register(path.to_str().unwrap(), name)
    }

    /// Close all plugins and clear the registry
    pub fn close(&self) -> PluginResult<()> {
        let mut plugins = self.plugins.lock().map_err(|_| {
            PluginError::registry_error("Failed to acquire registry lock")
        })?;

        let mut errors = Vec::new();

        // Close all plugins
        for (name, plugin_arc) in plugins.drain() {
            if let Ok(mut plugin) = plugin_arc.lock() {
                if let Err(e) = plugin.close() {
                    errors.push(format!("Failed to close plugin {}: {}", name, e));
                }
            }
        }

        if !errors.is_empty() {
            return Err(PluginError::registry_error(&errors.join("; ")));
        }

        Ok(())
    }

    /// Get registry statistics
    pub fn stats(&self) -> RegistryStats {
        let plugins = self.plugins.lock().unwrap_or_else(|_| {
            // If lock is poisoned, return empty stats
            return RegistryStats::default();
        });

        let total_plugins = plugins.len();
        let mut loaded_plugins = 0;

        for plugin_arc in plugins.values() {
            if let Ok(plugin) = plugin_arc.lock() {
                if plugin.is_loaded() {
                    loaded_plugins += 1;
                }
            }
        }

        RegistryStats {
            total_plugins,
            loaded_plugins,
            failed_plugins: total_plugins - loaded_plugins,
        }
    }
}

impl Default for PlugRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PlugRegistry {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Registry statistics
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    pub total_plugins: usize,
    pub loaded_plugins: usize,
    pub failed_plugins: usize,
}

