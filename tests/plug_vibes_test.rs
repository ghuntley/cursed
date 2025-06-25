/// Comprehensive test suite for the PlugVibes plugin system
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;
use cursed::stdlib::plug_vibes::*;
use cursed::stdlib::value::Value;

/// Test plugin loading and basic functionality
#[test]
fn test_basic_plugin_operations() {
    // Test LoadOptions creation
    let options = LoadOptions {
        version_check: true,
        verify_signature: false,
        isolation: true,
        sandbox: false,
        timeout: Duration::from_secs(30),
        dependencies: vec!["dep1".to_string()],
        allowed_imports: vec!["std".to_string()],
        debug_logging: true,
    };

    assert!(options.version_check);
    assert!(!options.verify_signature);
    assert!(options.isolation);
    assert_eq!(options.timeout, Duration::from_secs(30));
    assert_eq!(options.dependencies.len(), 1);
    assert_eq!(options.allowed_imports.len(), 1);

    // Test PlugInfo creation
    let info = PlugInfo {
        name: "test_plugin".to_string(),
        version: "1.0.0".to_string(),
        api: "1.0".to_string(),
        author: "Test Author".to_string(),
        description: "A test plugin".to_string(),
        build_time: std::time::SystemTime::now(),
        dependencies: vec!["dep1".to_string()],
        capabilities: vec!["test".to_string(), "utility".to_string()],
        imports: vec!["std".to_string()],
        exports: vec!["test_function".to_string()],
        signature: "test_signature".to_string(),
        is_verified: false,
        is_compatible: true,
    };

    assert_eq!(info.name, "test_plugin");
    assert_eq!(info.version, "1.0.0");
    assert_eq!(info.capabilities.len(), 2);
    assert!(info.is_compatible);
}

/// Test plugin registry functionality
#[test]
fn test_plugin_registry() {
    let registry = PlugRegistry::new();
    
    // Initially empty
    assert_eq!(registry.len(), 0);
    assert!(registry.is_empty());
    assert_eq!(registry.list().len(), 0);

    // Create a test plugin
    let info = PlugInfo::default();
    let plugin = Plug::new(std::path::PathBuf::from("/test/plugin.so"), info);

    // Register plugin
    let result = registry.register("test_plugin", plugin);
    assert!(result.is_ok());
    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
    assert!(registry.contains("test_plugin"));

    // Get plugin
    let retrieved = registry.get("test_plugin");
    assert!(retrieved.is_ok());

    // List plugins
    let list = registry.list();
    assert_eq!(list.len(), 1);
    assert!(list.contains(&"test_plugin".to_string()));

    // Try to register duplicate
    let info2 = PlugInfo::default();
    let plugin2 = Plug::new(std::path::PathBuf::from("/test/plugin2.so"), info2);
    let result = registry.register("test_plugin", plugin2);
    assert!(result.is_err());

    // Unregister plugin
    let result = registry.unregister("test_plugin");
    assert!(result.is_ok());
    assert_eq!(registry.len(), 0);
    assert!(!registry.contains("test_plugin"));

    // Try to unregister non-existent plugin
    let result = registry.unregister("nonexistent");
    assert!(result.is_err());

    // Test registry stats
    let stats = registry.stats();
    assert_eq!(stats.total_plugins, 0);
    assert_eq!(stats.loaded_plugins, 0);
}

/// Test plugin manager functionality
#[test]
fn test_plugin_manager() {
    let temp_dir = TempDir::new().unwrap();
    let plugin_dir = temp_dir.path().to_str().unwrap();

    // Create manager options
    let options = PlugManagerOptions::new()
        .with_plugin_dir(plugin_dir)
        .with_auto_load(false)
        .with_auto_reload(false)
        .with_hot_reload(false)
        .with_watch_interval(Duration::from_secs(1))
        .with_on_plugin_load(|name, _plugin| {
            println!("Plugin loaded: {}", name);
            Ok(())
        })
        .with_on_plugin_error(|name, error| {
            println!("Plugin error in {}: {}", name, error);
        });

    assert_eq!(options.plugin_dir, Some(plugin_dir.to_string()));
    assert!(!options.auto_load);
    assert!(!options.auto_reload);
    assert!(!options.hot_reload);

    // Create manager
    let mut manager = PlugManager::new(options);
    assert!(!manager.is_running());

    // Start manager
    let result = manager.start();
    assert!(result.is_ok());
    assert!(manager.is_running());

    // Test plugin state management
    let plugins = manager.list_plugins();
    assert_eq!(plugins.len(), 0);

    // Test disable/enable plugin
    let result = manager.disable_plugin("test_plugin");
    assert!(result.is_ok());
    assert!(manager.is_plugin_disabled("test_plugin"));

    let result = manager.enable_plugin("test_plugin");
    assert!(result.is_ok());
    assert!(!manager.is_plugin_disabled("test_plugin"));

    // Stop manager
    let result = manager.stop();
    assert!(result.is_ok());
    assert!(!manager.is_running());
}

/// Test plugin versioning system
#[test]
fn test_plugin_versioning() {
    // Test version creation
    let v1 = Version::new(1, 2, 3);
    assert_eq!(v1.major, 1);
    assert_eq!(v1.minor, 2);
    assert_eq!(v1.patch, 3);
    assert_eq!(v1.pre_release, None);

    let v2 = Version::new_with_prerelease(1, 0, 0, "alpha".to_string());
    assert_eq!(v2.pre_release, Some("alpha".to_string()));

    // Test version parsing
    let parsed = parse_version("1.2.3").unwrap();
    assert_eq!(parsed.major, 1);
    assert_eq!(parsed.minor, 2);
    assert_eq!(parsed.patch, 3);

    let parsed_pre = parse_version("1.0.0-beta").unwrap();
    assert_eq!(parsed_pre.pre_release, Some("beta".to_string()));

    // Test version formatting
    assert_eq!(v1.to_string(), "1.2.3");
    assert_eq!(v2.to_string(), "1.0.0-alpha");

    // Test version comparison
    let v3 = Version::new(1, 2, 4);
    let v4 = Version::new(1, 2, 3);
    
    assert!(v3.greater_than(&v1));
    assert!(v1.less_than(&v3));
    assert!(v1.equal(&v4));

    // Test version compatibility
    let v5 = Version::new(1, 3, 0);
    let v6 = Version::new(1, 1, 0);
    let v7 = Version::new(2, 0, 0);

    assert!(v5.compatible(&v6)); // Same major, higher minor
    assert!(!v5.compatible(&v7)); // Different major
    assert!(!v6.compatible(&v5)); // Same major, lower minor

    // Test version constraints
    let constraint = VersionConstraint::AtLeast(Version::new(1, 0, 0));
    assert!(constraint.satisfies(&Version::new(1, 5, 0)));
    assert!(!constraint.satisfies(&Version::new(0, 9, 0)));

    let constraint = VersionConstraint::Compatible(Version::new(1, 0, 0));
    assert!(constraint.satisfies(&Version::new(1, 2, 0)));
    assert!(!constraint.satisfies(&Version::new(2, 0, 0)));

    // Test parsing errors
    assert!(parse_version("invalid").is_err());
    assert!(parse_version("1.2").is_err());
    assert!(parse_version("1.2.3.4").is_err());
}

/// Test plugin sandbox functionality
#[test]
fn test_plugin_sandbox() {
    // Test sandbox options
    let options = SandboxOptions::new()
        .with_memory_limit(128 * 1024 * 1024) // 128MB
        .with_cpu_limit(0.8)
        .with_time_limit(Duration::from_secs(60))
        .with_file_access(vec!["/tmp".to_string()])
        .with_network_access(vec!["localhost".to_string()])
        .with_max_threads(8)
        .with_max_file_descriptors(200)
        .with_syscall_filtering(true, vec!["read".to_string(), "write".to_string()]);

    assert_eq!(options.memory_limit, 128 * 1024 * 1024);
    assert_eq!(options.cpu_limit, 0.8);
    assert_eq!(options.time_limit, Duration::from_secs(60));
    assert!(options.file_access);
    assert!(options.network_access);
    assert_eq!(options.max_threads, 8);
    assert_eq!(options.max_file_descriptors, 200);
    assert!(options.syscall_filtering);

    // Test sandbox creation
    let sandbox = Sandbox::new(options);
    
    // Test resource usage
    let usage = sandbox.get_resource_usage();
    assert_eq!(usage.memory_used, 0);
    assert_eq!(usage.threads_created, 0);

    // Test access checks
    let result = sandbox.check_file_access("/tmp/test.txt");
    assert!(result.is_ok());

    let result = sandbox.check_file_access("/etc/passwd");
    assert!(result.is_err());

    let result = sandbox.check_network_access("localhost");
    assert!(result.is_ok());

    let result = sandbox.check_network_access("malicious.com");
    assert!(result.is_err());

    let result = sandbox.check_syscall("read");
    assert!(result.is_ok());

    let result = sandbox.check_syscall("execve");
    assert!(result.is_err());

    // Test sandbox release
    let result = sandbox.release();
    assert!(result.is_ok());
}

/// Test plugin security features
#[test]
fn test_plugin_security() {
    // Test security manager
    let mut manager = SecurityManager::new();
    assert!(!manager.is_signature_enforcement_enabled());
    assert_eq!(manager.get_trusted_keys().len(), 0);

    // Add trusted key
    let result = manager.add_trusted_key("test_public_key");
    assert!(result.is_ok());
    assert_eq!(manager.get_trusted_keys().len(), 1);

    // Enable signature enforcement
    manager.set_signature_enforcement(true);
    assert!(manager.is_signature_enforcement_enabled());

    // Test signature algorithms
    let result = manager.set_signature_algorithm("RSA-SHA256");
    assert!(result.is_ok());
    assert_eq!(manager.get_signature_algorithm(), "RSA-SHA256");

    let result = manager.set_signature_algorithm("INVALID");
    assert!(result.is_err());

    // Remove trusted key
    let result = manager.remove_trusted_key("test_public_key");
    assert!(result.is_ok());
    assert_eq!(manager.get_trusted_keys().len(), 0);

    // Test key pair generation
    let key_pair = generate_plugin_key_pair().unwrap();
    assert!(!key_pair.private_key.is_empty());
    assert!(!key_pair.public_key.is_empty());
    assert_eq!(key_pair.algorithm, "RSA-SHA256");
    assert_eq!(key_pair.key_size, 2048);

    // Test auth info
    let auth = AuthInfo::new("user")
        .with_password("pass")
        .with_api_key("key123");

    assert_eq!(auth.username, "user");
    assert_eq!(auth.password, Some("pass".to_string()));
    assert_eq!(auth.api_key, Some("key123".to_string()));
}

/// Test plugin hooks system
#[test]
fn test_plugin_hooks() {
    // Test hook creation
    let hook = PlugHook::new("test_hook");
    assert_eq!(hook.name(), "test_hook");
    assert!(hook.is_enabled());
    assert_eq!(hook.registration_count(), 0);

    // Test callback registration
    let result = hook.register_callback("plugin1", 10, |args| {
        let mut result = args.to_vec();
        result.push(Value::String("hook1".to_string()));
        Ok(result)
    });
    assert!(result.is_ok());

    let result = hook.register_callback("plugin2", 5, |args| {
        let mut result = args.to_vec();
        result.push(Value::String("hook2".to_string()));
        Ok(result)
    });
    assert!(result.is_ok());

    assert_eq!(hook.registration_count(), 2);

    // Test hook calling
    let input = vec![Value::String("input".to_string())];
    let output = hook.call(&input);
    assert_eq!(output.len(), 3); // input + 2 hooks
    assert_eq!(output[0], Value::String("input".to_string()));
    // Hooks are called in priority order (highest first), so plugin1 (priority 10) goes first

    // Test call until true
    let (output, found) = hook.call_until_true(&input);
    assert!(!found); // Neither hook returns true

    // Test hook statistics
    assert!(hook.get_call_count() > 0);
    assert_eq!(hook.get_error_count(), 0);

    // Test hook enable/disable
    hook.set_enabled(false).unwrap();
    assert!(!hook.is_enabled());

    let output = hook.call(&input);
    assert_eq!(output, input); // No modification when disabled

    hook.set_enabled(true).unwrap();
    assert!(hook.is_enabled());

    // Test unregistration
    let result = hook.unregister_by_name("plugin1");
    assert!(result.is_ok());
    assert_eq!(hook.registration_count(), 1);

    // Test clear
    let result = hook.clear();
    assert!(result.is_ok());
    assert_eq!(hook.registration_count(), 0);

    // Test hook manager
    let manager = HookManager::new();
    let hook1 = manager.get_or_create_hook("hook1").unwrap();
    let hook2 = manager.get_or_create_hook("hook2").unwrap();

    assert_eq!(hook1.name(), "hook1");
    assert_eq!(hook2.name(), "hook2");

    let hooks = manager.list_hooks();
    assert_eq!(hooks.len(), 2);
    assert!(hooks.contains(&"hook1".to_string()));
    assert!(hooks.contains(&"hook2".to_string()));

    // Test extension points
    let ext_point = new_extension_point("test_extension");
    assert_eq!(ext_point.name(), "test_extension");
    assert_eq!(ext_point.extension_count(), 0);
}

/// Test plugin distribution functionality
#[test]
fn test_plugin_distribution() {
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    let install_dir = temp_dir.path().join("install");

    // Test package manager
    let mut manager = PackageManager::new(cache_dir, install_dir);

    // Test repository management
    let repo = Repository::new("test", "https://example.com/plugins")
        .trusted()
        .with_auth(AuthInfo::new("user").with_password("pass"));

    let result = manager.add_repository(repo);
    assert!(result.is_ok());
    assert_eq!(manager.list_repositories().len(), 1);

    // Try to add duplicate
    let repo2 = Repository::new("test", "https://other.com");
    let result = manager.add_repository(repo2);
    assert!(result.is_err());

    // Remove repository
    let result = manager.remove_repository("test");
    assert!(result.is_ok());
    assert_eq!(manager.list_repositories().len(), 0);

    // Test signature verification
    manager.set_signature_verification(true);

    // Test package metadata
    let metadata = PackageMetadata {
        name: "test-plugin".to_string(),
        version: Version::new(1, 0, 0),
        description: "Test plugin".to_string(),
        author: "Test Author".to_string(),
        license: "MIT".to_string(),
        homepage: Some("https://example.com".to_string()),
        repository: Some("https://github.com/user/repo".to_string()),
        keywords: vec!["test".to_string()],
        categories: vec!["utility".to_string()],
        dependencies: HashMap::new(),
        files: Vec::new(),
        install_scripts: InstallScripts::default(),
        target_platforms: vec!["linux".to_string()],
        min_host_version: Some(Version::new(1, 0, 0)),
        max_host_version: None,
    };

    assert_eq!(metadata.name, "test-plugin");
    assert_eq!(metadata.version, Version::new(1, 0, 0));

    // Test download and publish functions
    let download_result = download_plugin(
        "https://example.com/plugins",
        "test-plugin",
        &Version::new(1, 0, 0),
    );
    assert!(download_result.is_ok());

    // Clean up downloaded file
    if let Ok(path) = download_result {
        let _ = std::fs::remove_file(&path);
    }
}

/// Test plugin development utilities
#[test]
fn test_plugin_development() {
    // Clear any existing state
    development::clear_exports().unwrap();
    development::clear_hooks().unwrap();

    // Test plugin context
    let mut context = development::PluginContext::new("test_plugin", "1.0.0");
    assert_eq!(context.name, "test_plugin");
    assert_eq!(context.version, "1.0.0");
    assert!(!context.is_initialized());

    // Test capabilities
    let capabilities = development::create_capability_checker(vec![
        "test".to_string(),
        "utility".to_string(),
    ]);
    context.set_capabilities(capabilities);
    assert!(context.capabilities.has_capability("test"));
    assert!(context.capabilities.has_capability("utility"));

    // Test plugin initialization
    let result = development::initialize_plugin(&mut context);
    assert!(result.is_ok());
    assert!(context.is_initialized());
    assert!(development::is_running_as_plugin());

    // Test exports
    let result = context.export("test_function", Value::String("implementation".to_string()));
    assert!(result.is_ok());

    let exported = context.get_export("test_function");
    assert!(exported.is_some());

    let global_export = development::get_export("test_function");
    assert!(global_export.is_some());

    // Test hooks
    let result = development::register_hook("test_hook", |args| {
        let mut result = args.to_vec();
        result.push(Value::String("hook_called".to_string()));
        Ok(result)
    });
    assert!(result.is_ok());

    let input = vec![Value::String("input".to_string())];
    let output = development::call_hook("test_hook", &input).unwrap();
    assert_eq!(output.len(), 2);

    // Test host info
    let host_info = development::HostInfo {
        name: "Test Host".to_string(),
        version: "1.0.0".to_string(),
        api_version: "1.0".to_string(),
        platform: "test".to_string(),
        architecture: "test64".to_string(),
    };
    development::set_host_info(host_info.clone()).unwrap();
    let retrieved = development::get_host_info();
    assert_eq!(retrieved.name, "Test Host");

    // Test plugin manifest creation
    let manifest = development::create_plugin_manifest(
        "test_plugin",
        "1.0.0",
        "1.0",
        "Test Author",
        "A test plugin",
        vec!["test".to_string()],
    );
    assert_eq!(manifest.name, "test_plugin");
    assert_eq!(manifest.capabilities.len(), 1);

    // Test plugin cleanup
    let result = development::cleanup_plugin(&mut context);
    assert!(result.is_ok());
    assert!(!context.is_initialized());

    // Test mock host
    let mock_host = development::testing::MockHost::new()
        .with_name("Mock Host")
        .with_version("1.0.0");

    mock_host.setup().unwrap();
    assert_eq!(development::get_host_info().name, "Mock Host");

    mock_host.teardown().unwrap();

    // Test create test plugin
    let mut test_plugin = development::testing::create_test_plugin("test", "1.0.0");
    assert_eq!(test_plugin.name, "test");
    assert!(test_plugin.capabilities.has_capability("test"));

    // Test plugin lifecycle simulation
    development::testing::simulate_plugin_load(&mut test_plugin).unwrap();
    assert!(test_plugin.is_initialized());

    development::testing::simulate_plugin_unload(&mut test_plugin).unwrap();
    assert!(!test_plugin.is_initialized());
}

/// Test LLVM integration features
#[test]
fn test_llvm_integration() {
    // Test LLVM plugin context
    let context = llvm_integration::LLVMPluginContext::new();
    assert_eq!(context.list_runtime_plugins().len(), 0);

    // Test LLVM plugin compiler
    let context_arc = Arc::new(context);
    let compiler = llvm_integration::DefaultLLVMPluginCompiler::new(context_arc.clone());

    // Test plugin load compilation
    let load_ir = compiler.compile_plugin_load("/test/plugin.so", "test_plugin").unwrap();
    assert!(load_ir.contains("cursed_load_plugin"));
    assert!(load_ir.contains("test_plugin"));
    assert!(load_ir.contains("@plugin_path_"));
    assert!(load_ir.contains("@plugin_name_"));

    // Test plugin call compilation
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    let call_ir = compiler.compile_plugin_call("test_plugin", "test_function", &args).unwrap();
    assert!(call_ir.contains("cursed_call_plugin_function"));
    assert!(call_ir.contains("test_plugin"));
    assert!(call_ir.contains("test_function"));
    assert!(call_ir.contains("@arg_0_0"));
    assert!(call_ir.contains("@arg_1_1"));

    // Test plugin unload compilation
    let unload_ir = compiler.compile_plugin_unload("test_plugin").unwrap();
    assert!(unload_ir.contains("cursed_unload_plugin"));
    assert!(unload_ir.contains("test_plugin"));

    // Test FFI declarations
    let ffi_decls = compiler.generate_plugin_ffi_declarations();
    assert!(ffi_decls.contains("declare i32 @cursed_load_plugin"));
    assert!(ffi_decls.contains("declare i32 @cursed_unload_plugin"));
    assert!(ffi_decls.contains("declare i8* @cursed_call_plugin_function"));
    assert!(ffi_decls.contains("declare i8* @cursed_get_plugin_symbol"));
    assert!(ffi_decls.contains("declare i32 @cursed_plugin_exists"));

    // Test context initialization
    let global_context = llvm_integration::initialize_llvm_plugin_context();
    assert_eq!(global_context.list_runtime_plugins().len(), 0);

    // Test runtime plugin registration
    let info = PlugInfo::default();
    let plugin = Arc::new(Mutex::new(Plug::new(
        std::path::PathBuf::from("/test/plugin.so"),
        info,
    )));

    let result = global_context.register_runtime_plugin("test_plugin", plugin);
    assert!(result.is_ok());

    let retrieved = global_context.get_runtime_plugin("test_plugin");
    assert!(retrieved.is_some());

    let plugins = global_context.list_runtime_plugins();
    assert!(plugins.contains(&"test_plugin".to_string()));
}

/// Test error handling throughout the system
#[test]
fn test_error_handling() {
    // Test PluginError creation and formatting
    let errors = vec![
        PluginError::plugin_not_found("/nonexistent/plugin.so"),
        PluginError::load_error("Failed to load library"),
        PluginError::symbol_not_found("missing_symbol"),
        PluginError::function_not_found("missing_function"),
        PluginError::initialization_failed("Init returned error"),
        PluginError::cleanup_failed("Cleanup failed"),
        PluginError::version_incompatible("Version mismatch"),
        PluginError::signature_verification_failed("Invalid signature"),
        PluginError::already_loaded("plugin_name"),
        PluginError::not_loaded("plugin_name"),
        PluginError::sandbox_violation("Memory limit exceeded"),
        PluginError::security_violation("Unauthorized access"),
        PluginError::dependency_missing("required_dep"),
        PluginError::timeout("Operation timeout"),
        PluginError::registry_error("Registry failure"),
        PluginError::manager_error("Manager failure"),
        PluginError::hook_error("Hook failure"),
        PluginError::distribution_error("Distribution failure"),
        PluginError::general("General error"),
    ];

    for error in errors {
        // Test that error can be formatted
        let error_string = error.to_string();
        assert!(!error_string.is_empty());

        // Test that error can be converted to CursedError
        let cursed_error: cursed::error::CursedError = error.into();
        let cursed_string = cursed_error.to_string();
        assert!(!cursed_string.is_empty());
    }
}

/// Test concurrent plugin operations
#[test]
fn test_concurrent_operations() {
    let registry = Arc::new(PlugRegistry::new());
    let handles = (0..10).map(|i| {
        let registry = Arc::clone(&registry);
        thread::spawn(move || {
            let plugin_name = format!("plugin_{}", i);
            let info = PlugInfo::default();
            let plugin = Plug::new(
                std::path::PathBuf::from(format!("/test/{}.so", plugin_name)),
                info,
            );

            // Register plugin
            let result = registry.register(&plugin_name, plugin);
            assert!(result.is_ok());

            // Verify registration
            assert!(registry.contains(&plugin_name));

            // Get plugin
            let retrieved = registry.get(&plugin_name);
            assert!(retrieved.is_ok());

            // Unregister plugin
            let result = registry.unregister(&plugin_name);
            assert!(result.is_ok());
        })
    }).collect::<Vec<_>>();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Registry should be empty after all operations
    assert_eq!(registry.len(), 0);
}

/// Test plugin lifecycle management
#[test]
fn test_plugin_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let plugin_dir = temp_dir.path().to_str().unwrap();

    // Create plugin manager with lifecycle callbacks
    let loaded_plugins = Arc::new(Mutex::new(Vec::new()));
    let unloaded_plugins = Arc::new(Mutex::new(Vec::new()));
    let error_plugins = Arc::new(Mutex::new(Vec::new()));

    let loaded_clone = Arc::clone(&loaded_plugins);
    let unloaded_clone = Arc::clone(&unloaded_plugins);
    let error_clone = Arc::clone(&error_plugins);

    let options = PlugManagerOptions::new()
        .with_plugin_dir(plugin_dir)
        .with_auto_load(false)
        .with_on_plugin_load(move |name, _plugin| {
            loaded_clone.lock().unwrap().push(name.to_string());
            Ok(())
        })
        .with_on_plugin_unload(move |name, _plugin| {
            unloaded_clone.lock().unwrap().push(name.to_string());
            Ok(())
        })
        .with_on_plugin_error(move |name, _error| {
            error_clone.lock().unwrap().push(name.to_string());
        });

    let mut manager = PlugManager::new(options);

    // Start manager
    manager.start().unwrap();

    // Test plugin state management
    assert_eq!(manager.list_plugins().len(), 0);

    // Test disable/enable cycle
    manager.disable_plugin("test_plugin").unwrap();
    assert!(manager.is_plugin_disabled("test_plugin"));

    manager.enable_plugin("test_plugin").unwrap();
    assert!(!manager.is_plugin_disabled("test_plugin"));

    // Stop manager
    manager.stop().unwrap();

    // Verify callback tracking
    let loaded = loaded_plugins.lock().unwrap();
    let unloaded = unloaded_plugins.lock().unwrap();
    let errors = error_plugins.lock().unwrap();

    // Should be empty since we didn't actually load any plugins
    assert_eq!(loaded.len(), 0);
    assert_eq!(unloaded.len(), 0);
    assert_eq!(errors.len(), 0);
}

/// Test memory safety and resource cleanup
#[test]
fn test_memory_safety() {
    // Test that dropping registries properly cleans up
    {
        let registry = PlugRegistry::new();
        let info = PlugInfo::default();
        let plugin = Plug::new(std::path::PathBuf::from("/test/plugin.so"), info);
        registry.register("test", plugin).unwrap();
        assert_eq!(registry.len(), 1);
    } // registry should be dropped and cleaned up here

    // Test that managers can be safely dropped
    {
        let options = PlugManagerOptions::new();
        let mut manager = PlugManager::new(options);
        manager.start().unwrap();
        assert!(manager.is_running());
    } // manager should be dropped and cleaned up here

    // Test sandbox resource cleanup
    {
        let options = SandboxOptions::default();
        let sandbox = Sandbox::new(options);
        let usage = sandbox.get_resource_usage();
        assert_eq!(usage.memory_used, 0);
        sandbox.release().unwrap();
    } // sandbox should be dropped and cleaned up here

    // Test hook cleanup
    {
        let hook = PlugHook::new("test_hook");
        hook.register_callback("test", 1, |args| Ok(args.to_vec())).unwrap();
        assert_eq!(hook.registration_count(), 1);
        hook.clear().unwrap();
        assert_eq!(hook.registration_count(), 0);
    } // hook should be dropped and cleaned up here
}

/// Test integration with the broader CURSED ecosystem
#[test]
fn test_cursed_integration() {
    // Test that PluginError can be converted to CursedError
    let plugin_error = PluginError::general("Test error");
    let cursed_error: cursed::error::CursedError = plugin_error.into();
    assert!(cursed_error.to_string().contains("Test error"));

    // Test that plugin system works with CURSED Values
    let values = vec![
        Value::Boolean(true),
        Value::Integer(42),
        Value::Float(3.14),
        Value::String("test".to_string()),
        Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
        Value::Object({
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), Value::String("value".to_string()));
            map
        }),
        Value::Null,
    ];

    // Test that hooks can process all value types
    let hook = PlugHook::new("value_test_hook");
    hook.register_callback("value_processor", 1, |args| {
        // Process each value type
        let processed: Vec<Value> = args.iter().map(|v| match v {
            Value::Boolean(b) => Value::Boolean(!b),
            Value::Integer(i) => Value::Integer(i + 1),
            Value::Float(f) => Value::Float(f + 1.0),
            Value::String(s) => Value::String(format!("processed_{}", s)),
            Value::Array(a) => Value::Array(a.clone()),
            Value::Object(o) => Value::Object(o.clone()),
            Value::Null => Value::Null,
        }).collect();
        Ok(processed)
    }).unwrap();

    let output = hook.call(&values);
    assert_eq!(output.len(), values.len());

    // Verify transformations
    assert_eq!(output[0], Value::Boolean(false)); // !true
    assert_eq!(output[1], Value::Integer(43)); // 42 + 1
    assert_eq!(output[2], Value::Float(4.14)); // 3.14 + 1.0
    assert_eq!(output[3], Value::String("processed_test".to_string()));
}
