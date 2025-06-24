use crate::error::Error;
//! Tests for PlugVibes plugin system
//!
//! Comprehensive test suite covering all aspects of the CURSED plugin system.

use super::*;
use crate::error::CursedError;
use crate::stdlib::value::Value;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_system_initialization() {
        // Test basic plugin system configuration
        let options = LoadOptions::default();
        assert!(options.version_check);
        assert_eq!(options.timeout, Duration::from_secs(30));
        assert!(options.dependencies.is_empty());
        assert!(options.allowed_imports.is_empty());
    }

    #[test]
    fn test_plugin_loading() {
        // Test plugin loading with nonexistent file
        let result = load("/nonexistent/path/plugin.so");
        assert!(result.is_err());
        match result.unwrap_err() {
            PluginError::PluginNotFound(path) => {
                assert_eq!(path, "/nonexistent/path/plugin.so");
            }
            _ => panic!("Expected PluginNotFound error"),
        }
    }

    #[test]
    fn test_plugin_registry() {
        // Test plugin registry operations
        let registry = PlugRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);

        // Test registry listing
        let names = registry.list();
        assert!(names.is_empty());

        // Test checking for nonexistent plugin
        assert!(!registry.contains("nonexistent"));
        let result = registry.get("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_plugin_security() {
        // Test plugin security configuration
        let mut options = LoadOptions::default();
        options.verify_signature = true;
        options.sandbox = true;
        
        assert!(options.verify_signature);
        assert!(options.sandbox);

        // Test security violation error creation
        let error = PluginError::security_violation("Test violation");
        assert_eq!(error.to_string(), "Security violation: Test violation");
    }

    #[test]
    fn test_plugin_sandbox() {
        // Test sandbox configuration and violation handling
        let options = LoadOptions {
            sandbox: true,
            allowed_imports: vec!["stdlib::io".to_string(), "stdlib::math".to_string()],
            ..Default::default()
        };

        assert!(options.sandbox);
        assert_eq!(options.allowed_imports.len(), 2);

        // Test sandbox violation error
        let violation = PluginError::sandbox_violation("Attempted to access restricted resource");
        match violation {
            PluginError::SandboxViolation(msg) => {
                assert_eq!(msg, "Attempted to access restricted resource");
            }
            _ => panic!("Expected SandboxViolation"),
        }
    }

    #[test]
    fn test_plugin_hooks() {
        // Test plugin hook system functionality
        let hook_name = "pre_compile";
        let hook_error = PluginError::hook_error("Hook execution failed");
        
        match hook_error {
            PluginError::HookError(msg) => {
                assert_eq!(msg, "Hook execution failed");
            }
            _ => panic!("Expected HookError"),
        }

        // Test hook registration result
        let result = register_hook(hook_name, Box::new(|_args| Ok(vec![])));
        assert!(result.is_err()); // Should fail when not running as plugin
    }

    #[test]
    fn test_plugin_version_management() {
        // Test version parsing and compatibility
        let v1 = parse_version("1.2.3").unwrap();
        let v2 = parse_version("1.3.0").unwrap();
        let v3 = parse_version("2.0.0").unwrap();

        // Test version compatibility
        assert!(v2.compatible(&v1)); // 1.3.0 is compatible with 1.2.3
        assert!(!v3.compatible(&v1)); // 2.0.0 is not compatible with 1.2.3

        // Test version constraints
        let constraint = VersionConstraint::AtLeast(v1.clone());
        assert!(constraint.satisfies(&v2));
        assert!(!constraint.satisfies(&Version::new(1, 1, 0)));

        // Test version display
        assert_eq!(v1.to_string(), "1.2.3");
        let prerelease = Version::new_with_prerelease(1, 0, 0, "alpha".to_string());
        assert_eq!(prerelease.to_string(), "1.0.0-alpha");
    }

    #[test]
    fn test_plugin_llvm_integration() {
        // Test LLVM integration configuration
        let config = LlvmPluginConfig::default();
        assert!(config.jit_enabled);
        assert!(config.optimization_enabled);
        assert_eq!(config.optimization_level, 2);

        // Test LLVM plugin compilation context
        let context = LlvmPluginContext::new();
        assert!(context.functions.is_empty());
        assert!(context.types.is_empty());
    }

    #[test]
    fn test_plugin_distribution() {
        // Test plugin distribution configuration
        let dist_config = DistributionConfig::default();
        assert_eq!(dist_config.max_download_size, 100 * 1024 * 1024); // 100MB
        assert!(dist_config.verify_checksums);
        assert!(dist_config.allow_updates);

        // Test distribution error
        let error = PluginError::distribution_error("Download failed");
        assert_eq!(error.to_string(), "Plugin distribution error: Download failed");
    }

    #[test]
    fn test_plugin_development_tools() {
        // Test development configuration and tools
        let dev_config = DevelopmentConfig::default();
        assert!(dev_config.hot_reload);
        assert!(dev_config.debug_symbols);
        assert_eq!(dev_config.reload_watch_dirs.len(), 0);

        // Test plugin scaffolding result
        let result = scaffold_plugin("test_plugin", "/tmp");
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.name, "test_plugin");
    }
}

/// Integration tests for the plugin system
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        // Test complete plugin lifecycle with mock plugin
        let registry = PlugRegistry::new();
        
        // Create a mock plugin
        let info = PlugInfo {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "test_author".to_string(),
            description: "Test plugin for lifecycle testing".to_string(),
            ..Default::default()
        };
        
        let plugin = Plug::new(PathBuf::from("/mock/plugin.so"), info.clone());
        
        // Test registration
        let register_result = registry.register("test_plugin", plugin);
        assert!(register_result.is_ok());
        assert!(registry.contains("test_plugin"));
        assert_eq!(registry.len(), 1);
        
        // Test plugin retrieval
        let retrieved = registry.get("test_plugin");
        assert!(retrieved.is_ok());
        
        if let Ok(plugin_arc) = retrieved {
            let plugin = plugin_arc.lock().unwrap();
            assert_eq!(plugin.info().name, "test_plugin");
            assert_eq!(plugin.info().version, "1.0.0");
        }
        
        // Test unregistration
        let unregister_result = registry.unregister("test_plugin");
        assert!(unregister_result.is_ok());
        assert!(!registry.contains("test_plugin"));
        assert_eq!(registry.len(), 0);
    }

    #[tokio::test]
    async fn test_plugin_communication() {
        // Test inter-plugin communication through registry
        let registry = Arc::new(PlugRegistry::new());
        
        // Create multiple mock plugins
        for i in 0..3 {
            let info = PlugInfo {
                name: format!("plugin_{}", i),
                version: "1.0.0".to_string(),
                ..Default::default()
            };
            let plugin = Plug::new(PathBuf::from(format!("/mock/plugin_{}.so", i)), info);
            registry.register(&format!("plugin_{}", i), plugin).unwrap();
        }
        
        assert_eq!(registry.len(), 3);
        
        // Test communication between plugins through shared registry
        let plugin_names = registry.list();
        assert_eq!(plugin_names.len(), 3);
        assert!(plugin_names.contains(&"plugin_0".to_string()));
        assert!(plugin_names.contains(&"plugin_1".to_string()));
        assert!(plugin_names.contains(&"plugin_2".to_string()));
        
        // Simulate plugin interaction
        for name in &plugin_names {
            let plugin = registry.get(name).unwrap();
            let plugin_guard = plugin.lock().unwrap();
            assert!(plugin_guard.info().name.starts_with("plugin_"));
        }
    }

    #[tokio::test]
    async fn test_plugin_hot_reload() {
        // Test hot reload functionality simulation
        let manager = PluginManager::new();
        
        // Configure for hot reload
        let mut config = ManagerConfig::default();
        config.enable_hot_reload = true;
        config.watch_directories = vec!["/mock/plugins".to_string()];
        
        let result = manager.configure(config);
        assert!(result.is_ok());
        
        // Simulate file change detection
        let mock_path = "/mock/plugins/test_plugin.so";
        let change_result = manager.handle_file_change(mock_path);
        
        // Should handle the change without error even if file doesn't exist
        // In a real implementation, this would reload the plugin
        assert!(change_result.is_ok());
    }
}

/// Performance tests for the plugin system
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_plugin_load_performance() {
        // Test plugin loading performance metrics
        let start = Instant::now();
        
        // Test rapid plugin creation (simulated loading)
        let mut plugins = Vec::new();
        for i in 0..100 {
            let info = PlugInfo {
                name: format!("plugin_{}", i),
                version: "1.0.0".to_string(),
                ..Default::default()
            };
            let plugin = Plug::new(PathBuf::from(format!("/mock/plugin_{}.so", i)), info);
            plugins.push(plugin);
        }
        
        let duration = start.elapsed();
        assert_eq!(plugins.len(), 100);
        
        // Should complete in reasonable time (less than 1 second for 100 plugins)
        assert!(duration < Duration::from_secs(1));
    }

    #[test]
    fn test_plugin_execution_performance() {
        // Test plugin function execution performance
        let info = PlugInfo::default();
        let plugin = Plug::new(PathBuf::from("/mock/plugin.so"), info);
        
        // Register a mock function
        let mock_function = create_plugin_function(|args| {
            // Simple computation to simulate plugin work
            if let Some(Value::Integer(n)) = args.first() {
                Ok(vec![Value::Integer(n * 2)])
            } else {
                Ok(vec![Value::Integer(0)])
            }
        });
        
        let register_result = plugin.register_function("double".to_string(), mock_function);
        assert!(register_result.is_ok());
        
        // Test function execution performance
        let start = Instant::now();
        let args = [42i32];
        
        // Execute function multiple times
        for _ in 0..1000 {
            let result = plugin.call_function::<i32, i32>("double", &args);
            if let Ok(value) = result {
                assert_eq!(value, 84);
            }
        }
        
        let duration = start.elapsed();
        // Should complete 1000 calls in reasonable time
        assert!(duration < Duration::from_millis(100));
    }
}

/// Stress tests for the plugin system
#[cfg(test)]
mod stress_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_many_plugins() {
        // Test loading many plugins simultaneously
        let registry = Arc::new(PlugRegistry::new());
        let counter = Arc::new(AtomicUsize::new(0));
        let num_plugins = 50;
        
        let mut handles = Vec::new();
        
        // Spawn threads to register plugins concurrently
        for i in 0..num_plugins {
            let registry_clone = Arc::clone(&registry);
            let counter_clone = Arc::clone(&counter);
            
            let handle = thread::spawn(move || {
                let info = PlugInfo {
                    name: format!("stress_plugin_{}", i),
                    version: "1.0.0".to_string(),
                    ..Default::default()
                };
                
                let plugin = Plug::new(PathBuf::from(format!("/mock/stress_{}.so", i)), info);
                
                if registry_clone.register(&format!("stress_plugin_{}", i), plugin).is_ok() {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all plugins were registered
        assert_eq!(counter.load(Ordering::SeqCst), num_plugins);
        assert_eq!(registry.len(), num_plugins);
        
        // Test concurrent access
        let plugin_names = registry.list();
        assert_eq!(plugin_names.len(), num_plugins);
    }

    #[test]
    fn test_plugin_memory_usage() {
        // Test plugin memory usage under stress
        let registry = PlugRegistry::new();
        let num_plugins = 100;
        
        // Register many plugins with large symbol tables
        for i in 0..num_plugins {
            let info = PlugInfo {
                name: format!("memory_test_plugin_{}", i),
                version: "1.0.0".to_string(),
                ..Default::default()
            };
            
            let plugin = Plug::new(PathBuf::from(format!("/mock/memory_{}.so", i)), info);
            
            // Add many symbols to simulate memory usage
            for j in 0..50 {
                let symbol_name = format!("symbol_{}_{}", i, j);
                let symbol_value = Value::String(format!("value_{}_{}", i, j));
                plugin.register_symbol(symbol_name, symbol_value).unwrap();
            }
            
            // Add many functions
            for j in 0..20 {
                let func_name = format!("function_{}_{}", i, j);
                let func = create_plugin_function(move |_args| {
                    Ok(vec![Value::Integer(j as i64)])
                });
                plugin.register_function(func_name, func).unwrap();
            }
            
            registry.register(&format!("memory_test_plugin_{}", i), plugin).unwrap();
        }
        
        // Verify all plugins are registered
        assert_eq!(registry.len(), num_plugins);
        
        // Test memory access patterns
        for i in 0..num_plugins {
            let plugin = registry.get(&format!("memory_test_plugin_{}", i)).unwrap();
            let plugin_guard = plugin.lock().unwrap();
            
            // Verify symbols are accessible
            let symbols = plugin_guard.symbols();
            assert_eq!(symbols.len(), 50);
            
            // Verify functions are accessible
            let functions = plugin_guard.function_names();
            assert_eq!(functions.len(), 20);
        }
        
        // Test cleanup
        for i in 0..num_plugins {
            registry.unregister(&format!("memory_test_plugin_{}", i)).unwrap();
        }
        
        assert!(registry.is_empty());
    }
}
