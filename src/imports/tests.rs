//! Tests for the import/module system

use super::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_module(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(format!("{}.csd", name));
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_import_manager_creation() {
        let manager = ImportManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_local_import_resolution() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create a test module
        let module_content = r#"
slay hello() {
    yolo "Hello from module!";
}
        "#;
        create_test_module(temp_path, "test_module", module_content);
        
        // Create import manager with temp directory in search path
        let config = ImportConfig {
            search_paths: vec![temp_path.to_path_buf()],
            stdlib_path: PathBuf::from("stdlib"),
            enable_package_manager: false,
            cache_enabled: true,
        };
        
        let mut manager = ImportManager::with_config(config).unwrap();
        
        // Test import resolution
        let import = ImportStatement {
            path: "test_module.csd".to_string(),
            alias: None,  
            items: vec![],
            is_glob: false,
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            let mut processing = HashSet::new();
            manager.resolve_single_import(&import, &mut processing).await
        });
        
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.module.name, "test_module");
    }

    #[test]
    fn test_circular_import_detection() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create two modules that import each other
        let module_a = r#"
yeet "module_b.csd"

slay func_a() {
    yolo "A";
}
        "#;
        
        let module_b = r#"
yeet "module_a.csd"

slay func_b() {
    yolo "B";
}
        "#;
        
        create_test_module(temp_path, "module_a", module_a);
        create_test_module(temp_path, "module_b", module_b);
        
        let config = ImportConfig {
            search_paths: vec![temp_path.to_path_buf()],
            stdlib_path: PathBuf::from("stdlib"),
            enable_package_manager: false,
            cache_enabled: true,
        };
        
        let mut manager = ImportManager::with_config(config).unwrap();
        
        let import = ImportStatement {
            path: "module_a.csd".to_string(),
            alias: None,
            items: vec![],
            is_glob: false,
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            let mut processing = HashSet::new();
            processing.insert("module_a.csd".to_string()); // Simulate circular reference
            manager.resolve_single_import(&import, &mut processing).await
        });
        
        assert!(result.is_err());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Circular import detected"));
    }

    #[test]
    fn test_import_caching() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        let module_content = r#"
slay cached_function() {
    yolo "Cached!";
}
        "#;
        create_test_module(temp_path, "cached_module", module_content);
        
        let config = ImportConfig {
            search_paths: vec![temp_path.to_path_buf()],
            stdlib_path: PathBuf::from("stdlib"),
            enable_package_manager: false,
            cache_enabled: true,
        };
        
        let mut manager = ImportManager::with_config(config).unwrap();
        
        let import = ImportStatement {
            path: "cached_module.csd".to_string(),
            alias: None,
            items: vec![],
            is_glob: false,
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut processing = HashSet::new();
            
            // First resolution
            let result1 = manager.resolve_single_import(&import, &mut processing).await;
            assert!(result1.is_ok());
            
            // Second resolution should use cache
            processing.clear();
            let result2 = manager.resolve_single_import(&import, &mut processing).await;
            assert!(result2.is_ok());
            
            // Should be cached
            assert!(manager.is_cached(&import.path));
        });
    }

    #[test]
    fn test_package_resolver() {
        let mut resolver = PackageImportResolver::new();
        
        // Add a test package
        resolver.add_package("test_package".to_string(), PathBuf::from("/path/to/package"));
        
        assert!(resolver.has_package("test_package"));
        assert!(!resolver.has_package("nonexistent_package"));
    }

    #[test]
    fn test_module_loader() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        let module_content = r#"
slay exported_function() {
    yolo "Exported!";
}

slay another_function() {
    yolo "Another!";
}
        "#;
        let module_path = create_test_module(temp_path, "loader_test", module_content);
        
        let mut loader = ModuleLoader::new();
        let result = loader.get_or_load_module(&module_path);
        
        assert!(result.is_ok());
        let module = result.unwrap();
        assert_eq!(module.name, "loader_test");
        assert_eq!(module.symbols.len(), 2);
        assert!(module.symbols.contains(&"exported_function".to_string()));
        assert!(module.symbols.contains(&"another_function".to_string()));
    }

    #[test]
    fn test_import_classification() {
        let config = ImportConfig::default();
        let manager = ImportManager::with_config(config).unwrap();
        
        // Test local imports
        let local1 = manager.classify_import("./module.csd").unwrap();
        match local1 {
            ImportSource::Local(_) => {}, // Expected
            _ => panic!("Expected local import"),
        }
        
        let local2 = manager.classify_import("../other/module.csd").unwrap();
        match local2 {
            ImportSource::Local(_) => {}, // Expected  
            _ => panic!("Expected local import"),
        }
        
        // Test standard library imports
        let stdlib = manager.classify_import("std::io").unwrap();
        match stdlib {
            ImportSource::Stdlib(_) => {}, // Expected
            _ => panic!("Expected stdlib import"),
        }
        
        let cursed_std = manager.classify_import("cursed::core").unwrap();
        match cursed_std {
            ImportSource::Stdlib(_) => {}, // Expected
            _ => panic!("Expected stdlib import"),
        }
        
        // Test package imports
        let package = manager.classify_import("some_package").unwrap();
        match package {
            ImportSource::Package(_) => {}, // Expected
            _ => panic!("Expected package import"),
        }
    }
}
