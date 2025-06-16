//! Configuration management for optimization profiles and settings
//! 
//! Provides configuration file loading, saving, and management for
//! optimization profiles, user preferences, and system settings.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug, instrument};

use crate::optimization::{
    OptimizationConfig, OptimizationLevel, OptimizationProfile,
    PerformanceMonitoringConfig, PerformanceReportFormat,
};
use crate::error::{Result, CursedError};

/// Default configuration file name
const DEFAULT_CONFIG_FILE: &str = "cursed-optimization.toml";

/// Configuration manager for optimization settings
pub struct OptimizationConfigManager {
    /// Configuration file path
    config_path: PathBuf,
    
    /// Current configuration
    config: ManagedOptimizationConfig,
}

/// Complete managed optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedOptimizationConfig {
    /// Version of configuration format
    pub version: String,
    
    /// Default optimization profile
    pub default_profile: OptimizationProfile,
    
    /// Built-in optimization profiles
    pub builtin_profiles: HashMap<String, OptimizationConfig>,
    
    /// User-defined custom profiles
    pub custom_profiles: HashMap<String, OptimizationConfig>,
    
    /// Global optimization settings
    pub global_settings: GlobalOptimizationSettings,
    
    /// Performance monitoring configuration
    pub performance_monitoring: PerformanceMonitoringConfig,
    
    /// Target-specific configurations
    pub target_configs: HashMap<String, TargetOptimizationConfig>,
}

/// Global optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalOptimizationSettings {
    /// Enable advanced optimizations by default
    pub enable_advanced_by_default: bool,
    
    /// Enable profile-guided optimization when available
    pub enable_pgo_when_available: bool,
    
    /// Enable adaptive optimization
    pub enable_adaptive_optimization: bool,
    
    /// Enable parallel optimization
    pub enable_parallel_optimization: bool,
    
    /// Enable target-specific optimizations
    pub enable_target_specific_optimization: bool,
    
    /// Maximum parallel jobs (0 = auto-detect)
    pub max_parallel_jobs: usize,
    
    /// Optimization timeout in seconds
    pub optimization_timeout_secs: u64,
    
    /// Enable regression detection
    pub enable_regression_detection: bool,
    
    /// Cache directory for optimization artifacts
    pub cache_directory: Option<PathBuf>,
    
    /// Profile data directory for PGO
    pub profile_data_directory: Option<PathBuf>,
}

impl Default for GlobalOptimizationSettings {
    fn default() -> Self {
        Self {
            enable_advanced_by_default: true,
            enable_pgo_when_available: true,
            enable_adaptive_optimization: true,
            enable_parallel_optimization: true,
            enable_target_specific_optimization: true,
            max_parallel_jobs: 0, // Auto-detect
            optimization_timeout_secs: 300, // 5 minutes
            enable_regression_detection: true,
            cache_directory: None,
            profile_data_directory: None,
        }
    }
}

/// Target-specific optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetOptimizationConfig {
    /// Target CPU architecture
    pub target_cpu: Option<String>,
    
    /// Target CPU features
    pub target_features: Vec<String>,
    
    /// Target-specific optimization level override
    pub optimization_level_override: Option<OptimizationLevel>,
    
    /// Enable vectorization for this target
    pub enable_vectorization: bool,
    
    /// Enable loop optimizations for this target
    pub enable_loop_optimizations: bool,
    
    /// Enable target-specific passes
    pub enable_target_specific_passes: bool,
}

impl Default for TargetOptimizationConfig {
    fn default() -> Self {
        Self {
            target_cpu: None,
            target_features: Vec::new(),
            optimization_level_override: None,
            enable_vectorization: true,
            enable_loop_optimizations: true,
            enable_target_specific_passes: true,
        }
    }
}

impl Default for ManagedOptimizationConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            default_profile: OptimizationProfile::Release,
            builtin_profiles: Self::create_builtin_profiles(),
            custom_profiles: HashMap::new(),
            global_settings: GlobalOptimizationSettings::default(),
            performance_monitoring: PerformanceMonitoringConfig::default(),
            target_configs: HashMap::new(),
        }
    }
}

impl ManagedOptimizationConfig {
    /// Create default built-in optimization profiles
    fn create_builtin_profiles() -> HashMap<String, OptimizationConfig> {
        let mut profiles = HashMap::new();
        
        profiles.insert("development".to_string(), OptimizationConfig::development());
        profiles.insert("release".to_string(), OptimizationConfig::release());
        profiles.insert("size".to_string(), OptimizationConfig::size_optimized());
        profiles.insert("fast_compilation".to_string(), OptimizationConfig::fast_compilation());
        
        // Create a debug profile with optimizations
        let mut debug_config = OptimizationConfig::development();
        debug_config.optimization_level = OptimizationLevel::Basic;
        debug_config.debug_info_level = crate::optimization::optimization_config::DebugInfoLevel::Full;
        debug_config.enable_dce = true;
        debug_config.enable_inlining = true;
        profiles.insert("debug".to_string(), debug_config);
        
        profiles
    }
}

impl OptimizationConfigManager {
    /// Create new configuration manager with default config file location
    pub fn new() -> Result<Self> {
        let config_dir = Self::get_default_config_directory()?;
        let config_path = config_dir.join(DEFAULT_CONFIG_FILE);
        Self::with_config_path(config_path)
    }
    
    /// Create configuration manager with specific config file path
    pub fn with_config_path<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_path = config_path.as_ref().to_path_buf();
        let config = if config_path.exists() {
            Self::load_config_from_file(&config_path)?
        } else {
            ManagedOptimizationConfig::default()
        };
        
        Ok(Self {
            config_path,
            config,
        })
    }
    
    /// Get default configuration directory
    fn get_default_config_directory() -> Result<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            let cursed_config_dir = config_dir.join("cursed");
            if !cursed_config_dir.exists() {
                fs::create_dir_all(&cursed_config_dir)?;
            }
            Ok(cursed_config_dir)
        } else {
            // Fallback to current directory
            Ok(PathBuf::from("."))
        }
    }
    
    /// Load configuration from file
    #[instrument(skip(config_path))]
    fn load_config_from_file<P: AsRef<Path>>(config_path: P) -> Result<ManagedOptimizationConfig> {
        let config_content = fs::read_to_string(&config_path)?;
        let mut config: ManagedOptimizationConfig = toml::from_str(&config_content)
            .map_err(|e| CursedError::generic(format!("Invalid config file: {}", e)))?;
        
        // Ensure built-in profiles are always available
        let builtin_profiles = ManagedOptimizationConfig::create_builtin_profiles();
        for (name, profile) in builtin_profiles {
            config.builtin_profiles.insert(name, profile);
        }
        
        debug!("Loaded optimization configuration from {:?}", config_path.as_ref());
        Ok(config)
    }
    
    /// Save configuration to file
    #[instrument(skip(self))]
    pub fn save_config(&self) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        
        let config_content = toml::to_string_pretty(&self.config)
            .map_err(|e| CursedError::generic(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&self.config_path, config_content)?;
        info!("Saved optimization configuration to {:?}", self.config_path);
        Ok(())
    }
    
    /// Get optimization configuration for a profile
    #[instrument(skip(self))]
    pub fn get_profile_config(&self, profile: &OptimizationProfile) -> Result<OptimizationConfig> {
        let profile_name = match profile {
            OptimizationProfile::Development => "development",
            OptimizationProfile::Release => "release",
            OptimizationProfile::Size => "size",
            OptimizationProfile::Debug => "debug",
            OptimizationProfile::Custom(name) => name,
        };
        
        // Check custom profiles first
        if let Some(config) = self.config.custom_profiles.get(profile_name) {
            debug!("Using custom profile: {}", profile_name);
            return Ok(config.clone());
        }
        
        // Check built-in profiles
        if let Some(config) = self.config.builtin_profiles.get(profile_name) {
            debug!("Using built-in profile: {}", profile_name);
            return Ok(config.clone());
        }
        
        // Fallback to release profile
        warn!("Profile '{}' not found, using release profile", profile_name);
        Ok(self.config.builtin_profiles.get("release")
            .cloned()
            .unwrap_or_else(OptimizationConfig::release))
    }
    
    /// Add or update a custom profile
    #[instrument(skip(self, config))]
    pub fn set_custom_profile(&mut self, name: String, config: OptimizationConfig) -> Result<()> {
        info!("Setting custom optimization profile: {}", name);
        self.config.custom_profiles.insert(name, config);
        Ok(())
    }
    
    /// Remove a custom profile
    #[instrument(skip(self))]
    pub fn remove_custom_profile(&mut self, name: &str) -> Result<bool> {
        info!("Removing custom optimization profile: {}", name);
        Ok(self.config.custom_profiles.remove(name).is_some())
    }
    
    /// List all available profiles
    pub fn list_profiles(&self) -> Vec<String> {
        let mut profiles = Vec::new();
        
        // Add built-in profiles
        profiles.extend(self.config.builtin_profiles.keys().cloned());
        
        // Add custom profiles
        profiles.extend(self.config.custom_profiles.keys().cloned());
        
        profiles.sort();
        profiles
    }
    
    /// Get global optimization settings
    pub fn get_global_settings(&self) -> &GlobalOptimizationSettings {
        &self.config.global_settings
    }
    
    /// Update global optimization settings
    #[instrument(skip(self, settings))]
    pub fn set_global_settings(&mut self, settings: GlobalOptimizationSettings) -> Result<()> {
        info!("Updating global optimization settings");
        self.config.global_settings = settings;
        Ok(())
    }
    
    /// Get performance monitoring configuration
    pub fn get_performance_monitoring(&self) -> &PerformanceMonitoringConfig {
        &self.config.performance_monitoring
    }
    
    /// Update performance monitoring configuration
    #[instrument(skip(self, config))]
    pub fn set_performance_monitoring(&mut self, config: PerformanceMonitoringConfig) -> Result<()> {
        info!("Updating performance monitoring configuration");
        self.config.performance_monitoring = config;
        Ok(())
    }
    
    /// Get target-specific configuration
    pub fn get_target_config(&self, target: &str) -> Option<&TargetOptimizationConfig> {
        self.config.target_configs.get(target)
    }
    
    /// Set target-specific configuration
    #[instrument(skip(self, config))]
    pub fn set_target_config(&mut self, target: String, config: TargetOptimizationConfig) -> Result<()> {
        info!("Setting target-specific configuration for: {}", target);
        self.config.target_configs.insert(target, config);
        Ok(())
    }
    
    /// Create a new profile based on an existing one
    #[instrument(skip(self))]
    pub fn create_profile_from_existing(&mut self, base_profile: &str, new_name: String) -> Result<()> {
        let base_config = self.config.builtin_profiles.get(base_profile)
            .or_else(|| self.config.custom_profiles.get(base_profile))
            .cloned()
            .ok_or_else(|| CursedError::generic(format!("Base profile '{}' not found", base_profile)))?;
        
        info!("Creating new profile '{}' based on '{}'", new_name, base_profile);
        self.config.custom_profiles.insert(new_name, base_config);
        Ok(())
    }
    
    /// Reset to default configuration
    #[instrument(skip(self))]
    pub fn reset_to_defaults(&mut self) -> Result<()> {
        info!("Resetting optimization configuration to defaults");
        self.config = ManagedOptimizationConfig::default();
        Ok(())
    }
    
    /// Export configuration to a different file
    #[instrument(skip(self, export_path))]
    pub fn export_config<P: AsRef<Path>>(&self, export_path: P) -> Result<()> {
        let config_content = toml::to_string_pretty(&self.config)
            .map_err(|e| CursedError::generic(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(export_path.as_ref(), config_content)?;
        info!("Exported optimization configuration to {:?}", export_path.as_ref());
        Ok(())
    }
    
    /// Import configuration from a different file
    #[instrument(skip(self, import_path))]
    pub fn import_config<P: AsRef<Path>>(&mut self, import_path: P) -> Result<()> {
        self.config = Self::load_config_from_file(import_path.as_ref())?;
        info!("Imported optimization configuration from {:?}", import_path.as_ref());
        Ok(())
    }
    
    /// Get configuration file path
    pub fn get_config_path(&self) -> &Path {
        &self.config_path
    }
    
    /// Validate current configuration
    #[instrument(skip(self))]
    pub fn validate_config(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Validate global settings
        if self.config.global_settings.max_parallel_jobs > 64 {
            warnings.push("max_parallel_jobs > 64 may cause excessive resource usage".to_string());
        }
        
        if self.config.global_settings.optimization_timeout_secs > 3600 {
            warnings.push("optimization_timeout_secs > 1 hour may cause very long builds".to_string());
        }
        
        // Validate custom profiles
        for (name, profile) in &self.config.custom_profiles {
            if let Err(e) = profile.validate() {
                warnings.push(format!("Custom profile '{}': {}", name, e));
            }
        }
        
        // Check for conflicting settings
        if !self.config.global_settings.enable_parallel_optimization && self.config.global_settings.max_parallel_jobs > 1 {
            warnings.push("max_parallel_jobs > 1 but parallel optimization is disabled".to_string());
        }
        
        if warnings.is_empty() {
            info!("Configuration validation passed");
        } else {
            warn!("Configuration validation found {} warnings", warnings.len());
            for warning in &warnings {
                warn!("  {}", warning);
            }
        }
        
        Ok(warnings)
    }
}

impl Default for OptimizationConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default optimization config manager")
    }
}

/// CLI helper functions for configuration management
pub mod cli {
    use super::*;
    use clap::{Arg, ArgAction, Command};
    
    /// Add configuration management CLI commands
    pub fn add_config_commands(command: Command) -> Command {
        command
            .subcommand(
                Command::new("config")
                    .about("Manage optimization configuration")
                    .subcommand(
                        Command::new("show")
                            .about("Show current configuration")
                            .arg(
                                Arg::new("profile")
                                    .long("profile")
                                    .value_name("NAME")
                                    .help("Show specific profile configuration")
                            )
                    )
                    .subcommand(
                        Command::new("list")
                            .about("List available optimization profiles")
                    )
                    .subcommand(
                        Command::new("create")
                            .about("Create new optimization profile")
                            .arg(
                                Arg::new("name")
                                    .help("Profile name")
                                    .required(true)
                                    .value_name("NAME")
                            )
                            .arg(
                                Arg::new("base")
                                    .long("base")
                                    .value_name("PROFILE")
                                    .help("Base profile to copy from")
                                    .default_value("release")
                            )
                    )
                    .subcommand(
                        Command::new("remove")
                            .about("Remove custom optimization profile")
                            .arg(
                                Arg::new("name")
                                    .help("Profile name to remove")
                                    .required(true)
                                    .value_name("NAME")
                            )
                    )
                    .subcommand(
                        Command::new("reset")
                            .about("Reset configuration to defaults")
                            .arg(
                                Arg::new("confirm")
                                    .long("confirm")
                                    .action(ArgAction::SetTrue)
                                    .help("Confirm reset operation")
                            )
                    )
                    .subcommand(
                        Command::new("export")
                            .about("Export configuration to file")
                            .arg(
                                Arg::new("file")
                                    .help("Output file path")
                                    .required(true)
                                    .value_name("FILE")
                            )
                    )
                    .subcommand(
                        Command::new("import")
                            .about("Import configuration from file")
                            .arg(
                                Arg::new("file")
                                    .help("Input file path")
                                    .required(true)
                                    .value_name("FILE")
                            )
                    )
                    .subcommand(
                        Command::new("validate")
                            .about("Validate current configuration")
                    )
            )
    }
    
    /// Handle configuration management commands
    pub async fn handle_config_command(matches: &clap::ArgMatches) -> Result<()> {
        let mut config_manager = OptimizationConfigManager::new()?;
        
        match matches.subcommand() {
            Some(("show", sub_matches)) => {
                if let Some(profile_name) = sub_matches.get_one::<String>("profile") {
                    let profile = crate::optimization::enablement_system::cli::parse_optimization_profile(profile_name);
                    let config = config_manager.get_profile_config(&profile)?;
                    println!("Profile '{}' configuration:", profile_name);
                    println!("{:#?}", config);
                } else {
                    println!("Current optimization configuration:");
                    println!("{:#?}", config_manager.config);
                }
            }
            Some(("list", _)) => {
                let profiles = config_manager.list_profiles();
                println!("Available optimization profiles:");
                for profile in profiles {
                    println!("  {}", profile);
                }
            }
            Some(("create", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap().clone();
                let base = sub_matches.get_one::<String>("base").unwrap();
                
                config_manager.create_profile_from_existing(base, name.clone())?;
                config_manager.save_config()?;
                
                println!("Created custom profile '{}' based on '{}'", name, base);
            }
            Some(("remove", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap();
                
                if config_manager.remove_custom_profile(name)? {
                    config_manager.save_config()?;
                    println!("Removed custom profile '{}'", name);
                } else {
                    println!("Profile '{}' not found or is a built-in profile", name);
                }
            }
            Some(("reset", sub_matches)) => {
                if sub_matches.get_flag("confirm") {
                    config_manager.reset_to_defaults()?;
                    config_manager.save_config()?;
                    println!("Configuration reset to defaults");
                } else {
                    println!("Use --confirm to reset configuration to defaults");
                }
            }
            Some(("export", sub_matches)) => {
                let file = sub_matches.get_one::<String>("file").unwrap();
                config_manager.export_config(file)?;
                println!("Configuration exported to {}", file);
            }
            Some(("import", sub_matches)) => {
                let file = sub_matches.get_one::<String>("file").unwrap();
                config_manager.import_config(file)?;
                config_manager.save_config()?;
                println!("Configuration imported from {}", file);
            }
            Some(("validate", _)) => {
                let warnings = config_manager.validate_config()?;
                if warnings.is_empty() {
                    println!("✅ Configuration is valid");
                } else {
                    println!("⚠️  Configuration validation warnings:");
                    for warning in warnings {
                        println!("  - {}", warning);
                    }
                }
            }
            _ => {
                println!("Use 'cursed optimize config --help' for available commands");
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_config_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test-config.toml");
        
        let manager = OptimizationConfigManager::with_config_path(&config_path);
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_builtin_profiles() {
        let manager = OptimizationConfigManager::new().unwrap();
        let profiles = manager.list_profiles();
        
        assert!(profiles.contains(&"development".to_string()));
        assert!(profiles.contains(&"release".to_string()));
        assert!(profiles.contains(&"size".to_string()));
        assert!(profiles.contains(&"debug".to_string()));
    }
    
    #[test]
    fn test_custom_profile() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test-config.toml");
        let mut manager = OptimizationConfigManager::with_config_path(&config_path).unwrap();
        
        let custom_config = OptimizationConfig::release();
        manager.set_custom_profile("test_profile".to_string(), custom_config.clone()).unwrap();
        
        let retrieved = manager.get_profile_config(&OptimizationProfile::Custom("test_profile".to_string())).unwrap();
        assert_eq!(retrieved.optimization_level, custom_config.optimization_level);
    }
    
    #[test]
    fn test_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test-config.toml");
        
        {
            let mut manager = OptimizationConfigManager::with_config_path(&config_path).unwrap();
            let custom_config = OptimizationConfig::development();
            manager.set_custom_profile("test_save".to_string(), custom_config).unwrap();
            manager.save_config().unwrap();
        }
        
        {
            let manager = OptimizationConfigManager::with_config_path(&config_path).unwrap();
            let profiles = manager.list_profiles();
            assert!(profiles.contains(&"test_save".to_string()));
        }
    }
    
    #[test]
    fn test_config_validation() {
        let manager = OptimizationConfigManager::new().unwrap();
        let warnings = manager.validate_config().unwrap();
        assert!(warnings.is_empty()); // Default config should be valid
    }
}
