use crate::error::CursedError;
/// Plugin manager for comprehensive plugin lifecycle management
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
// use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
// use crate::stdlib::plug_vibes::plug::{Plug, LoadOptions, PlugInfo};
// use crate::stdlib::plug_vibes::registry::PlugRegistry;

/// Callback function types for plugin events
pub type PluginLoadCallback = Box<dyn Fn(&str, &Plug) -> PluginResult<()> + Send + Sync>;
pub type PluginUnloadCallback = Box<dyn Fn(&str, &Plug) -> PluginResult<()> + Send + Sync>;
pub type PluginErrorCallback = Box<dyn Fn(&str, &PluginError) + Send + Sync>;

/// Configuration for the plugin manager
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    pub enable_hot_reload: bool,
    pub watch_directories: Vec<String>,
    pub auto_load_plugins: bool,
    pub plugin_timeout: Duration,
    pub max_plugins: Option<usize>,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            enable_hot_reload: false,
            watch_directories: vec![],
            auto_load_plugins: false,
            plugin_timeout: Duration::from_secs(30),
            max_plugins: None,
        }
    }
}

/// Options for configuring the plugin manager
#[derive(Default)]
pub struct PlugManagerOptions {
    /// Directory to scan for plugins
    pub plugin_dir: Option<String>,
    /// Automatically load plugins on startup
    pub auto_load: bool,
    /// Enable automatic plugin reloading when files change
    pub auto_reload: bool,
    /// Interval for checking plugin file changes
    pub watch_interval: Duration,
    /// Default load options for plugins
    pub load_options: LoadOptions,
    /// Custom plugin registry (if None, creates a new one)
    pub registry: Option<Arc<PlugRegistry>>,
    /// Enable hot reloading of plugins
    pub hot_reload: bool,
    /// Callback when a plugin is loaded
    pub on_plugin_load: Option<PluginLoadCallback>,
    /// Callback when a plugin is unloaded
    pub on_plugin_unload: Option<PluginUnloadCallback>,
    /// Callback when a plugin error occurs
    pub on_plugin_error: Option<PluginErrorCallback>,
}

impl PlugManagerOptions {
    pub fn new() -> Self {
        Self {
            plugin_dir: None,
            auto_load: false,
            auto_reload: false,
            watch_interval: Duration::from_secs(5),
            load_options: LoadOptions::default(),
            registry: None,
            hot_reload: false,
            on_plugin_load: None,
            on_plugin_unload: None,
            on_plugin_error: None,
        }
    }

    pub fn with_plugin_dir(mut self, dir: &str) -> Self {
        self.plugin_dir = Some(dir.to_string());
        self
    }

    pub fn with_auto_load(mut self, auto_load: bool) -> Self {
        self.auto_load = auto_load;
        self
    }

    pub fn with_auto_reload(mut self, auto_reload: bool) -> Self {
        self.auto_reload = auto_reload;
        self
    }

    pub fn with_hot_reload(mut self, hot_reload: bool) -> Self {
        self.hot_reload = hot_reload;
        self
    }

    pub fn with_watch_interval(mut self, interval: Duration) -> Self {
        self.watch_interval = interval;
        self
    }

    pub fn with_load_options(mut self, options: LoadOptions) -> Self {
        self.load_options = options;
        self
    }

    pub fn with_registry(mut self, registry: Arc<PlugRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }

    pub fn with_on_plugin_load<F>(mut self, callback: F) -> Self 
    where
        F: Fn(&str, &Plug) -> PluginResult<()> + Send + Sync + 'static,
    {
        self.on_plugin_load = Some(Box::new(callback));
        self
    }

    pub fn with_on_plugin_unload<F>(mut self, callback: F) -> Self 
    where
        F: Fn(&str, &Plug) -> PluginResult<()> + Send + Sync + 'static,
    {
        self.on_plugin_unload = Some(Box::new(callback));
        self
    }

    pub fn with_on_plugin_error<F>(mut self, callback: F) -> Self 
    where
        F: Fn(&str, &PluginError) + Send + Sync + 'static,
    {
        self.on_plugin_error = Some(Box::new(callback));
        self
    }
}

/// Plugin state tracking
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    Loaded,
    Unloaded,
    Loading,
    Unloading,
    CursedError,
    Disabled,
}

/// Plugin information with state
#[derive(Debug, Clone)]
pub struct ManagedPluginInfo {
    pub info: PlugInfo,
    pub state: PluginState,
    pub last_modified: Option<std::time::SystemTime>,
    pub load_count: u32,
    pub error_count: u32,
    pub last_error: Option<String>,
}

impl ManagedPluginInfo {
    pub fn new(info: PlugInfo) -> Self {
        Self {
            info,
            state: PluginState::Unloaded,
            last_modified: None,
            load_count: 0,
            error_count: 0,
            last_error: None,
        }
    }
}

/// Comprehensive plugin manager
pub struct PlugManager {
    registry: Arc<PlugRegistry>,
    options: PlugManagerOptions,
    plugin_states: Arc<Mutex<HashMap<String, ManagedPluginInfo>>>,
    disabled_plugins: Arc<Mutex<std::collections::HashSet<String>>>,
    is_running: Arc<Mutex<bool>>,
    watcher_handle: Option<thread::JoinHandle<()>>,
    file_timestamps: Arc<Mutex<HashMap<PathBuf, std::time::SystemTime>>>,
}

impl PlugManager {
    /// Create a new plugin manager with default options
    pub fn new() -> Self {
        Self::with_options(PlugManagerOptions::default())
    }

    /// Create a new plugin manager with specific options
    pub fn with_options(options: PlugManagerOptions) -> Self {
        let registry = options.registry.clone()
            .unwrap_or_else(|| Arc::new(PlugRegistry::new()));

        Self {
            registry,
            options,
            plugin_states: Arc::new(Mutex::new(HashMap::new())),
            disabled_plugins: Arc::new(Mutex::new(std::collections::HashSet::new())),
            is_running: Arc::new(Mutex::new(false)),
            watcher_handle: None,
            file_timestamps: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Configure the plugin manager
    pub fn configure(&self, config: ManagerConfig) -> PluginResult<()> {
        // In a real implementation, this would update internal configuration
        // For now, just validate the config
        if config.max_plugins.is_some() && config.max_plugins.unwrap() == 0 {
            return Err(PluginError::manager_error("Invalid max_plugins value"));
        }
        Ok(())
    }

    /// Handle a file change event
    pub fn handle_file_change(&self, path: &str) -> PluginResult<()> {
        // In a real implementation, this would:
        // 1. Check if the file is a plugin
        // 2. Reload the plugin if it's already loaded
        // 3. Load the plugin if it's new
        // For now, just return success
        Ok(())
    }

    /// Start the plugin manager
    pub fn start(&mut self) -> PluginResult<()> {
        {
            let mut running = self.is_running.lock().map_err(|_| {
                PluginError::manager_error("Failed to acquire running state lock")
            })?;
            
            if *running {
                return Err(PluginError::manager_error("Plugin manager already running"));
            }
            
            *running = true;
        }

        // Auto-load plugins if configured
        if self.options.auto_load {
            if let Some(ref plugin_dir) = self.options.plugin_dir {
                self.load_plugins_from_directory(plugin_dir)?;
            }
        }

        // Start file watcher if auto-reload is enabled
        if self.options.auto_reload {
            self.start_file_watcher()?;
        }

        Ok(())
    }

    /// Stop the plugin manager
    pub fn stop(&mut self) -> PluginResult<()> {
        {
            let mut running = self.is_running.lock().map_err(|_| {
                PluginError::manager_error("Failed to acquire running state lock")
            })?;
            
            if !*running {
                return Ok(());
            }
            
            *running = false;
        }

        // Stop file watcher
        if let Some(handle) = self.watcher_handle.take() {
            let _ = handle.join();
        }

        // Unload all plugins
        self.unload_all_plugins()?;

        Ok(())
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.is_running.lock()
            .map(|running| *running)
            .unwrap_or(false)
    }

    /// Load a plugin from a file path
    pub fn load_plugin(&self, path: &str) -> PluginResult<Arc<Mutex<Plug>>> {
        let file_path = Path::new(path);
        let plugin_name = file_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginError::load_error("Invalid plugin file name"))?;

        // Check if plugin is disabled
        if self.is_plugin_disabled(plugin_name) {
            return Err(PluginError::manager_error(&format!("Plugin {} is disabled", plugin_name)));
        }

        // Update state to loading
        self.update_plugin_state(plugin_name, PluginState::Loading)?;

        // Load the plugin
        match self.registry.load_and_register_with_options(path, plugin_name, self.options.load_options.clone()) {
            Ok(plugin_arc) => {
                // Update state and timestamps
                self.update_plugin_state(plugin_name, PluginState::Loaded)?;
                self.update_file_timestamp(path)?;
                self.increment_load_count(plugin_name)?;

                // Call load callback
                if let Some(ref callback) = self.options.on_plugin_load {
                    if let Ok(plugin) = plugin_arc.lock() {
                        if let Err(e) = callback(plugin_name, &*plugin) {
                            self.handle_plugin_error(plugin_name, &e);
                        }
                    }
                }

                Ok(plugin_arc)
            }
            Err(e) => {
                self.update_plugin_state(plugin_name, PluginState::CursedError)?;
                self.handle_plugin_error(plugin_name, &e);
                Err(e)
            }
        }
    }

    /// Unload a plugin by name
    pub fn unload_plugin(&self, name: &str) -> PluginResult<()> {
        // Update state to unloading
        self.update_plugin_state(name, PluginState::Unloading)?;

        // Get the plugin for callback
        let plugin_arc = self.registry.get(name)?;

        // Call unload callback
        if let Some(ref callback) = self.options.on_plugin_unload {
            if let Ok(plugin) = plugin_arc.lock() {
                if let Err(e) = callback(name, &*plugin) {
                    self.handle_plugin_error(name, &e);
                }
            }
        }

        // Unregister from registry
        match self.registry.unregister(name) {
            Ok(()) => {
                self.update_plugin_state(name, PluginState::Unloaded)?;
                Ok(())
            }
            Err(e) => {
                self.update_plugin_state(name, PluginState::CursedError)?;
                self.handle_plugin_error(name, &e);
                Err(e)
            }
        }
    }

    /// Reload a plugin by name
    pub fn reload_plugin(&self, name: &str) -> PluginResult<()> {
        // Check if plugin is disabled
        if self.is_plugin_disabled(name) {
            return Err(PluginError::manager_error(&format!("Plugin {} is disabled", name)));
        }

        // First unload, then load again
        if self.registry.contains(name) {
            self.unload_plugin(name)?;
        }

        // We need to determine the path - in a real implementation,
        // we'd track this information
        if let Some(ref plugin_dir) = self.options.plugin_dir {
            let plugin_path = format!("{}/{}.so", plugin_dir, name);
            self.load_plugin(&plugin_path)?;
        } else {
            return Err(PluginError::manager_error("Cannot reload plugin: no plugin directory configured"));
        }

        Ok(())
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<Arc<Mutex<Plug>>> {
        self.registry.get(name).ok()
    }

    /// List all managed plugins with their information
    pub fn list_plugins(&self) -> Vec<ManagedPluginInfo> {
        self.plugin_states.lock()
            .map(|states| states.values().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }

    /// Install a plugin from a source (URL or local path)
    pub fn install_plugin(&self, src: &str) -> PluginResult<Arc<Mutex<Plug>>> {
        // For now, treat as a local path
        // In a real implementation, this would handle URLs, package managers, etc.
        self.load_plugin(src)
    }

    /// Enable a disabled plugin
    pub fn enable_plugin(&self, name: &str) -> PluginResult<()> {
        let mut disabled = self.disabled_plugins.lock().map_err(|_| {
            PluginError::manager_error("Failed to acquire disabled plugins lock")
        })?;

        disabled.remove(name);
        Ok(())
    }

    /// Disable a plugin (unload and prevent loading)
    pub fn disable_plugin(&self, name: &str) -> PluginResult<()> {
        // Unload if currently loaded
        if self.registry.contains(name) {
            self.unload_plugin(name)?;
        }

        // Add to disabled list
        let mut disabled = self.disabled_plugins.lock().map_err(|_| {
            PluginError::manager_error("Failed to acquire disabled plugins lock")
        })?;

        disabled.insert(name.to_string());
        self.update_plugin_state(name, PluginState::Disabled)?;
        Ok(())
    }

    /// Check if a plugin is disabled
    pub fn is_plugin_disabled(&self, name: &str) -> bool {
        self.disabled_plugins.lock()
            .map(|disabled| disabled.contains(name))
            .unwrap_or(false)
    }

    /// Load all plugins from the configured directory
    fn load_plugins_from_directory(&self, directory: &str) -> PluginResult<()> {
        match self.registry.load_all_with_options(directory, self.options.load_options.clone()) {
            Ok(loaded_plugins) => {
                for (name, _) in loaded_plugins {
                    self.update_plugin_state(&name, PluginState::Loaded)?;
                    self.increment_load_count(&name)?;
                }
                Ok(())
            }
            Err(e) => {
                if let Some(ref callback) = self.options.on_plugin_error {
                    callback("directory_load", &e);
                }
                Err(e)
            }
        }
    }

    /// Unload all plugins
    fn unload_all_plugins(&self) -> PluginResult<()> {
        let plugin_names = self.registry.list();
        for name in plugin_names {
            let _ = self.unload_plugin(&name);
        }
        Ok(())
    }

    /// Start the file watcher for auto-reload
    fn start_file_watcher(&mut self) -> PluginResult<()> {
        if self.options.plugin_dir.is_none() {
            return Ok(());
        }

        let plugin_dir = self.options.plugin_dir.clone().unwrap();
        let watch_interval = self.options.watch_interval;
        let is_running = Arc::clone(&self.is_running);
        let file_timestamps = Arc::clone(&self.file_timestamps);
        
        // Create a simple file watcher
        let handle = thread::spawn(move || {
            while *is_running.lock().unwrap_or_else(|_| {
                std::process::exit(1);
            }) {
                // Check for file changes
                if let Ok(entries) = std::fs::read_dir(&plugin_dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(extension) = path.extension() {
                                    if extension == "so" || extension == "dll" || extension == "dylib" {
                                        if let Ok(metadata) = entry.metadata() {
                                            if let Ok(modified) = metadata.modified() {
                                                let mut timestamps = file_timestamps.lock().unwrap();
                                                let changed = timestamps.get(&path)
                                                    .map(|&old_time| modified > old_time)
                                                    .unwrap_or(true);
                                                
                                                if changed {
                                                    timestamps.insert(path.clone(), modified);
                                                    // In a real implementation, we'd trigger a reload here
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                thread::sleep(watch_interval);
            }
        });

        self.watcher_handle = Some(handle);
        Ok(())
    }

    /// Update plugin state
    fn update_plugin_state(&self, name: &str, state: PluginState) -> PluginResult<()> {
        let mut states = self.plugin_states.lock().map_err(|_| {
            PluginError::manager_error("Failed to acquire plugin states lock")
        })?;

        states.entry(name.to_string())
            .and_modify(|info| info.state = state.clone())
            .or_insert_with(|| {
                let mut info = ManagedPluginInfo::new(PlugInfo::default());
                info.state = state;
                info
            });

        Ok(())
    }

    /// Increment load count for a plugin
    fn increment_load_count(&self, name: &str) -> PluginResult<()> {
        let mut states = self.plugin_states.lock().map_err(|_| {
            PluginError::manager_error("Failed to acquire plugin states lock")
        })?;

        states.entry(name.to_string())
            .and_modify(|info| info.load_count += 1);

        Ok(())
    }

    /// Update file timestamp
    fn update_file_timestamp(&self, path: &str) -> PluginResult<()> {
        let path_buf = PathBuf::from(path);
        if let Ok(metadata) = std::fs::metadata(&path_buf) {
            if let Ok(modified) = metadata.modified() {
                let mut timestamps = self.file_timestamps.lock().map_err(|_| {
                    PluginError::manager_error("Failed to acquire file timestamps lock")
                })?;
                timestamps.insert(path_buf, modified);
            }
        }
        Ok(())
    }

    /// Handle plugin error
    fn handle_plugin_error(&self, name: &str, error: &PluginError) {
        // Update error count
        if let Ok(mut states) = self.plugin_states.lock() {
            states.entry(name.to_string())
                .and_modify(|info| {
                    info.error_count += 1;
                    info.last_error = Some(error.to_string());
                });
        }

        // Call error callback
        if let Some(ref callback) = self.options.on_plugin_error {
            callback(name, error);
        }
    }
}

impl Drop for PlugManager {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

