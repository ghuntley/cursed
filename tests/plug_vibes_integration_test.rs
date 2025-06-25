/// Integration tests for the PlugVibes plugin system
use cursed::stdlib::plug_vibes::*;
use cursed::stdlib::value::Value;
use std::time::Duration;

#[test]
fn test_load_options_creation() {
    let options = LoadOptions::default();
    assert!(options.version_check);
    assert!(!options.verify_signature);
    assert!(!options.isolation);
    assert!(!options.sandbox);
    assert_eq!(options.timeout, Duration::from_secs(30));
    assert!(options.dependencies.is_empty());
    assert!(options.allowed_imports.is_empty());
}

#[test]
fn test_plug_info_creation() {
    let info = PlugInfo::default();
    assert_eq!(info.name, "");
    assert_eq!(info.version, "0.0.0");
    assert_eq!(info.api, "1.0");
    assert_eq!(info.author, "");
    assert_eq!(info.description, "");
    assert!(info.dependencies.is_empty());
    assert!(info.capabilities.is_empty());
    assert!(info.imports.is_empty());
    assert!(info.exports.is_empty());
    assert_eq!(info.signature, "");
    assert!(!info.is_verified);
    assert!(info.is_compatible);
}

#[test]
fn test_host_info_creation() {
    let host_info = get_host_info();
    assert_eq!(host_info.name, "CURSED");
    assert_eq!(host_info.version, "1.0.0");
    assert_eq!(host_info.api_version, "1.0");
    assert!(!host_info.platform.is_empty());
    assert!(!host_info.architecture.is_empty());
}

#[test]
fn test_plugin_value_converters() {
    // Test i32 converter
    let int_val = 42i32;
    let plugin_val = int_val.to_plugin_value();
    match plugin_val {
        Value::Integer(i) => assert_eq!(i, 42),
        _ => panic!("Expected Integer value"),
    }
    
    let converted_back = i32::from_plugin_value(&plugin_val);
    assert_eq!(converted_back, Some(42));

    // Test f64 converter
    let float_val = 3.14f64;
    let plugin_val = float_val.to_plugin_value();
    match plugin_val {
        Value::Float(f) => assert!((f - 3.14).abs() < f64::EPSILON),
        _ => panic!("Expected Float value"),
    }
    
    let converted_back = f64::from_plugin_value(&plugin_val);
    assert!(converted_back.is_some());
    assert!((converted_back.unwrap() - 3.14).abs() < f64::EPSILON);

    // Test String converter
    let string_val = "hello".to_string();
    let plugin_val = string_val.to_plugin_value();
    match plugin_val {
        Value::String(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected String value"),
    }
    
    let converted_back = String::from_plugin_value(&plugin_val);
    assert_eq!(converted_back, Some("hello".to_string()));

    // Test bool converter
    let bool_val = true;
    let plugin_val = bool_val.to_plugin_value();
    match plugin_val {
        Value::Boolean(b) => assert!(b),
        _ => panic!("Expected Boolean value"),
    }
    
    let converted_back = bool::from_plugin_value(&plugin_val);
    assert_eq!(converted_back, Some(true));
}

#[test]
fn test_plugin_creation() {
    let path = std::path::PathBuf::from("/test/plugin.so");
    let info = PlugInfo::default();
    let plugin = Plug::new(path.clone(), info);
    
    assert_eq!(plugin.path(), &path);
    assert!(!plugin.is_loaded());
    assert_eq!(plugin.symbols().len(), 0);
    assert_eq!(plugin.function_names().len(), 0);
}

#[test]
fn test_plugin_symbol_registration() {
    let path = std::path::PathBuf::from("/test/plugin.so");
    let info = PlugInfo::default();
    let plugin = Plug::new(path, info);
    
    let result = plugin.register_symbol("test_symbol".to_string(), Value::Integer(42));
    assert!(result.is_ok());
    
    let symbols = plugin.symbols();
    assert_eq!(symbols.len(), 1);
    assert!(symbols.contains(&"test_symbol".to_string()));
    
    let lookup_result = plugin.lookup("test_symbol");
    assert!(lookup_result.is_ok());
    match lookup_result.unwrap() {
        Value::Integer(i) => assert_eq!(i, 42),
        _ => panic!("Expected Integer value"),
    }
}

#[test]
fn test_plugin_function_registration() {
    let path = std::path::PathBuf::from("/test/plugin.so");
    let info = PlugInfo::default();
    let plugin = Plug::new(path, info);
    
    // Create a simple function that doubles the first integer argument
    let test_func = create_plugin_function(|args: &[Value]| -> PluginResult<Vec<Value>> {
        if let Some(Value::Integer(i)) = args.first() {
            Ok(vec![Value::Integer(i * 2)])
        } else {
            Err(PluginError::general("Expected integer argument"))
        }
    });
    
    let result = plugin.register_function("double".to_string(), test_func);
    assert!(result.is_ok());
    
    let function_names = plugin.function_names();
    assert_eq!(function_names.len(), 1);
    assert!(function_names.contains(&"double".to_string()));
}

#[test]
fn test_plugin_statistics() {
    let path = std::path::PathBuf::from("/test/plugin.so");
    let mut info = PlugInfo::default();
    info.name = "test_plugin".to_string();
    info.version = "1.0.0".to_string();
    let plugin = Plug::new(path, info);
    
    // Add some symbols and functions
    plugin.register_symbol("symbol1".to_string(), Value::Integer(1)).unwrap();
    plugin.register_symbol("symbol2".to_string(), Value::String("test".to_string())).unwrap();
    
    let test_func = create_plugin_function(|_| Ok(vec![]));
    plugin.register_function("func1".to_string(), test_func).unwrap();
    
    let stats = plugin.get_statistics().unwrap();
    
    assert_eq!(stats.get("loaded"), Some(&Value::Boolean(false)));
    assert_eq!(stats.get("symbol_count"), Some(&Value::Integer(2)));
    assert_eq!(stats.get("function_count"), Some(&Value::Integer(1)));
    assert_eq!(stats.get("name"), Some(&Value::String("test_plugin".to_string())));
    assert_eq!(stats.get("version"), Some(&Value::String("1.0.0".to_string())));
}

#[test]
fn test_plugin_integrity_verification() {
    let path = std::path::PathBuf::from("/nonexistent/plugin.so");
    let info = PlugInfo::default();
    let plugin = Plug::new(path, info);
    
    let integrity = plugin.verify_integrity().unwrap();
    assert!(!integrity); // Should be false because plugin is not loaded and file doesn't exist
}

#[test]
fn test_plugin_registry_creation() {
    let registry = PlugRegistry::new();
    assert_eq!(registry.len(), 0);
    assert!(registry.is_empty());
    assert_eq!(registry.list().len(), 0);
}

#[test]
fn test_plugin_registry_operations() {
    let registry = PlugRegistry::new();
    
    // Create a test plugin
    let path = std::path::PathBuf::from("/test/plugin.so");
    let mut info = PlugInfo::default();
    info.name = "test_plugin".to_string();
    let plugin = Plug::new(path, info);
    
    // Register the plugin
    let result = registry.register("test", plugin);
    assert!(result.is_ok());
    assert_eq!(registry.len(), 1);
    assert!(registry.contains("test"));
    
    // Try to register duplicate
    let path2 = std::path::PathBuf::from("/test/plugin2.so");
    let info2 = PlugInfo::default();
    let plugin2 = Plug::new(path2, info2);
    let duplicate_result = registry.register("test", plugin2);
    assert!(duplicate_result.is_err());
    
    // Get the plugin
    let retrieved = registry.get("test");
    assert!(retrieved.is_ok());
    
    // List plugins
    let list = registry.list();
    assert_eq!(list.len(), 1);
    assert!(list.contains(&"test".to_string()));
    
    // Get stats
    let stats = registry.stats();
    assert_eq!(stats.total_plugins, 1);
    
    // Unregister
    let unregister_result = registry.unregister("test");
    assert!(unregister_result.is_ok());
    assert_eq!(registry.len(), 0);
    assert!(!registry.contains("test"));
}

#[test]
fn test_plugin_registry_get_nonexistent() {
    let registry = PlugRegistry::new();
    let result = registry.get("nonexistent");
    assert!(result.is_err());
    match result.unwrap_err() {
        PluginError::NotLoaded(name) => assert_eq!(name, "nonexistent"),
        _ => panic!("Expected NotLoaded error"),
    }
}

#[test]
fn test_plugin_manager_options() {
    let options = PlugManagerOptions::new()
        .with_plugin_dir("./plugins")
        .with_auto_load(true)
        .with_auto_reload(true)
        .with_hot_reload(true)
        .with_watch_interval(Duration::from_secs(10));
    
    assert_eq!(options.plugin_dir, Some("./plugins".to_string()));
    assert!(options.auto_load);
    assert!(options.auto_reload);
    assert!(options.hot_reload);
    assert_eq!(options.watch_interval, Duration::from_secs(10));
}

#[test]
fn test_managed_plugin_info() {
    let mut info = PlugInfo::default();
    info.name = "test".to_string();
    info.version = "1.0.0".to_string();
    
    let managed_info = ManagedPluginInfo::new(info);
    assert_eq!(managed_info.info.name, "test");
    assert_eq!(managed_info.info.version, "1.0.0");
    assert_eq!(managed_info.state, PluginState::Unloaded);
    assert_eq!(managed_info.load_count, 0);
    assert_eq!(managed_info.error_count, 0);
    assert!(managed_info.last_error.is_none());
}

#[test]
fn test_plugin_helper_functions() {
    // Test is_running_as_plugin
    let is_plugin = is_running_as_plugin();
    // This should be false in a test environment
    assert!(!is_plugin);
    
    // Test get_plugin_api
    let api_version = get_plugin_api();
    assert_eq!(api_version, "1.0");
    
    // Test host info
    let host_info = get_host_info();
    assert_eq!(host_info.name, "CURSED");
}

#[test]
fn test_plugin_error_types() {
    let error1 = PluginError::plugin_not_found("/test/plugin.so");
    match error1 {
        PluginError::PluginNotFound(path) => assert_eq!(path, "/test/plugin.so"),
        _ => panic!("Expected PluginNotFound error"),
    }
    
    let error2 = PluginError::symbol_not_found("test_symbol");
    match error2 {
        PluginError::SymbolNotFound(symbol) => assert_eq!(symbol, "test_symbol"),
        _ => panic!("Expected SymbolNotFound error"),
    }
    
    let error3 = PluginError::function_not_found("test_function");
    match error3 {
        PluginError::FunctionNotFound(func) => assert_eq!(func, "test_function"),
        _ => panic!("Expected FunctionNotFound error"),
    }
}

#[test]
fn test_version_parsing_and_compatibility() {
    // Test valid version parsing
    let v1 = parse_version("1.2.3").unwrap();
    assert_eq!(v1.major, 1);
    assert_eq!(v1.minor, 2);
    assert_eq!(v1.patch, 3);
    assert_eq!(v1.pre_release, None);
    
    // Test version with pre-release
    let v2 = parse_version("1.2.3-alpha").unwrap();
    assert_eq!(v2.major, 1);
    assert_eq!(v2.minor, 2);
    assert_eq!(v2.patch, 3);
    assert_eq!(v2.pre_release, Some("alpha".to_string()));
    
    // Test version compatibility
    let v3 = Version::new(1, 2, 3);
    let v4 = Version::new(1, 1, 0);
    let v5 = Version::new(2, 0, 0);
    
    assert!(v3.compatible(&v4));  // Same major, higher minor
    assert!(!v3.compatible(&v5)); // Different major
    assert!(!v4.compatible(&v3)); // Lower version
    
    // Test version comparison
    assert!(v3.greater_than(&v4));
    assert!(v4.less_than(&v3));
    assert!(v3.equal(&Version::new(1, 2, 3)));
    
    // Test breaking change detection
    assert!(v5.is_breaking_change(&v3));
    assert!(!v3.is_breaking_change(&v4));
    
    // Test feature addition detection
    assert!(v3.is_feature_addition(&v4));
    assert!(!v4.is_feature_addition(&v3));
    
    // Test patch detection
    let v6 = Version::new(1, 2, 4);
    assert!(v6.is_patch(&v3));
    assert!(!v3.is_patch(&v6));
}

#[test]
fn test_version_constraint_satisfaction() {
    let v1 = Version::new(1, 2, 3);
    
    // Test exact constraint
    assert!(VersionConstraint::Exact(v1.clone()).satisfies(&v1));
    assert!(!VersionConstraint::Exact(Version::new(1, 2, 4)).satisfies(&v1));
    
    // Test at least constraint
    assert!(VersionConstraint::AtLeast(Version::new(1, 0, 0)).satisfies(&v1));
    assert!(!VersionConstraint::AtLeast(Version::new(2, 0, 0)).satisfies(&v1));
    
    // Test at most constraint
    assert!(VersionConstraint::AtMost(Version::new(2, 0, 0)).satisfies(&v1));
    assert!(!VersionConstraint::AtMost(Version::new(1, 0, 0)).satisfies(&v1));
    
    // Test range constraint
    assert!(VersionConstraint::Range(Version::new(1, 0, 0), Version::new(2, 0, 0)).satisfies(&v1));
    assert!(!VersionConstraint::Range(Version::new(0, 0, 0), Version::new(1, 0, 0)).satisfies(&v1));
    
    // Test compatible constraint
    assert!(VersionConstraint::Compatible(Version::new(1, 1, 0)).satisfies(&v1));
    assert!(!VersionConstraint::Compatible(Version::new(2, 0, 0)).satisfies(&v1));
    
    // Test any constraint
    assert!(VersionConstraint::Any.satisfies(&v1));
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

#[test]
fn test_create_plugin_function() {
    let func = create_plugin_function(|args: &[Value]| -> PluginResult<Vec<Value>> {
        if args.is_empty() {
            Ok(vec![Value::String("empty".to_string())])
        } else {
            Ok(vec![Value::Integer(args.len() as i64)])
        }
    });
    
    // Test with empty args
    let result1 = func(&[]).unwrap();
    assert_eq!(result1.len(), 1);
    match &result1[0] {
        Value::String(s) => assert_eq!(s, "empty"),
        _ => panic!("Expected String value"),
    }
    
    // Test with args
    let args = vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)];
    let result2 = func(&args).unwrap();
    assert_eq!(result2.len(), 1);
    match &result2[0] {
        Value::Integer(i) => assert_eq!(*i, 3),
        _ => panic!("Expected Integer value"),
    }
}
