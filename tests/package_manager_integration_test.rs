/// Integration tests for complete package manager ecosystem
/// 
/// Tests the interaction between lock files, workspaces, and package management
/// functionality in realistic scenarios.

use std::collections::HashMap;
use tempfile::TempDir;

use cursed::package_manager::{
    PackageManager, PackageManagerConfig, PackageMetadata,
    WorkspaceManager, LockFileManager,
};

#[tokio::test]
async fn test_package_manager_lock_file_integration() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();
    
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: project_dir.join("cache"),
        workspace_dir: project_dir.to_path_buf(),
        max_cache_size: 1024 * 1024, // 1MB
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    
    // Mock some installed packages
    let test_packages = vec![
        PackageMetadata {
            name: "test-package-1".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package 1".to_string(),
            authors: vec!["Test Author".to_string()],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        },
        PackageMetadata {
            name: "test-package-2".to_string(),
            version: "2.0.0".to_string(),
            description: "Test package 2".to_string(),
            authors: vec!["Test Author".to_string()],
            dependencies: {
                let mut deps = HashMap::new();
                deps.insert("test-package-1".to_string(), VersionSpec::Simple("1.0.0".to_string()));
                deps
            },
            dev_dependencies: HashMap::new(),
            repository: None,
            license: None,
            keywords: Vec::new(),
            categories: Vec::new(),
        },
    ];
    
    // Generate lock file
    manager.generate_lock_file().unwrap();
    
    // Verify lock file exists
    let lock_file_path = project_dir.join("CursedPackage.lock");
    assert!(lock_file_path.exists());
    
    // Validate lock file
    manager.validate_lock_file().unwrap();
    
    // Check lock file status
    let lock_manager = manager.lock_file_status().unwrap();
    assert!(lock_manager.exists());
}

#[tokio::test]
async fn test_workspace_with_lock_file_integration() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path();
    
    // Create workspace structure
    let member1_dir = workspace_root.join("member1");
    let member2_dir = workspace_root.join("member2");
    std::fs::create_dir_all(&member1_dir).unwrap();
    std::fs::create_dir_all(&member2_dir).unwrap();
    
    // Create member package files
    std::fs::write(member1_dir.join("CursedPackage.toml"), r#"
        name = "member1"
        version = "1.0.0"
        description = "Member 1"
        authors = ["Test"]
        
        [dependencies]
        serde = "1.0.0"
    "#).unwrap();
    
    std::fs::write(member2_dir.join("CursedPackage.toml"), r#"
        name = "member2"
        version = "1.0.0"
        description = "Member 2"
        authors = ["Test"]
        
        [dependencies]
        member1 = "1.0.0"
        tokio = "1.5.0"
    "#).unwrap();
    
    // Initialize workspace
    let members = vec!["member1".to_string(), "member2".to_string()];
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: workspace_root.join("cache"),
        workspace_dir: workspace_root.to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    manager.init_workspace(workspace_root, members).unwrap();
    
    // Verify workspace was created
    assert!(manager.workspace().is_some());
    let workspace = manager.workspace().unwrap();
    assert!(workspace.is_workspace());
    assert_eq!(workspace.members().len(), 2);
    
    // Generate workspace lock file
    if let Some(workspace) = manager.workspace_mut() {
        workspace.generate_lock_file().unwrap();
    }
    
    // Verify workspace lock file exists
    let lock_file_path = workspace_root.join("CursedPackage.lock");
    assert!(lock_file_path.exists());
    
    // Load and verify lock file content
    let content = std::fs::read_to_string(&lock_file_path).unwrap();
    assert!(content.contains("workspace_root"));
}

#[tokio::test]
async fn test_workspace_build_order_integration() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path();
    
    // Create complex workspace structure
    let dirs = ["base", "utils", "app"];
    for dir in &dirs {
        std::fs::create_dir_all(workspace_root.join(dir)).unwrap();
    }
    
    // Create interdependent packages: app -> utils -> base
    std::fs::write(workspace_root.join("base/CursedPackage.toml"), r#"
        name = "base"
        version = "1.0.0"
        description = "Base library"
        authors = ["Test"]
    "#).unwrap();
    
    std::fs::write(workspace_root.join("utils/CursedPackage.toml"), r#"
        name = "utils"
        version = "1.0.0"
        description = "Utility library"
        authors = ["Test"]
        
        [dependencies]
        base = "1.0.0"
        external-lib = "2.0.0"
    "#).unwrap();
    
    std::fs::write(workspace_root.join("app/CursedPackage.toml"), r#"
        name = "app"
        version = "1.0.0"
        description = "Main application"
        authors = ["Test"]
        
        [dependencies]
        utils = "1.0.0"
        base = "1.0.0"
        web-framework = "3.0.0"
    "#).unwrap();
    
    // Initialize workspace
    let members = vec!["base".to_string(), "utils".to_string(), "app".to_string()];
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: workspace_root.join("cache"),
        workspace_dir: workspace_root.to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    manager.init_workspace(workspace_root, members).unwrap();
    
    // Test workspace functionality
    let workspace = manager.workspace().unwrap();
    let build_order = workspace.get_build_order().unwrap();
    
    // Verify correct build order
    assert_eq!(build_order.len(), 3);
    assert_eq!(build_order[0].name, "base");    // No dependencies
    assert_eq!(build_order[1].name, "utils");   // Depends on base
    assert_eq!(build_order[2].name, "app");     // Depends on utils and base
    
    // Test dependency listing
    let dependencies = workspace.list_dependencies();
    assert_eq!(dependencies.len(), 3);
    
    let base_deps = dependencies.get("base").unwrap();
    assert!(base_deps.is_empty());
    
    let utils_deps = dependencies.get("utils").unwrap();
    assert_eq!(utils_deps.len(), 2);
    assert!(utils_deps.contains(&"base 1.0.0".to_string()));
    assert!(utils_deps.contains(&"external-lib 2.0.0".to_string()));
    
    let app_deps = dependencies.get("app").unwrap();
    assert_eq!(app_deps.len(), 3);
    assert!(app_deps.contains(&"utils 1.0.0".to_string()));
    assert!(app_deps.contains(&"base 1.0.0".to_string()));
    assert!(app_deps.contains(&"web-framework 3.0.0".to_string()));
}

#[tokio::test]
async fn test_workspace_lock_file_dependency_resolution() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path();
    
    // Create workspace with shared dependencies
    let member_dirs = ["frontend", "backend", "shared"];
    for dir in &member_dirs {
        std::fs::create_dir_all(workspace_root.join(dir)).unwrap();
    }
    
    // All members use the same version of a common dependency
    std::fs::write(workspace_root.join("shared/CursedPackage.toml"), r#"
        name = "shared"
        version = "1.0.0"
        description = "Shared utilities"
        authors = ["Test"]
        
        [dependencies]
        serde = "1.0.0"
        uuid = "1.2.0"
    "#).unwrap();
    
    std::fs::write(workspace_root.join("frontend/CursedPackage.toml"), r#"
        name = "frontend"
        version = "1.0.0"
        description = "Frontend application"
        authors = ["Test"]
        
        [dependencies]
        shared = "1.0.0"
        serde = "1.0.0"
        web-sys = "2.0.0"
    "#).unwrap();
    
    std::fs::write(workspace_root.join("backend/CursedPackage.toml"), r#"
        name = "backend"
        version = "1.0.0"
        description = "Backend API"
        authors = ["Test"]
        
        [dependencies]
        shared = "1.0.0"
        serde = "1.0.0"
        tokio = "1.5.0"
    "#).unwrap();
    
    // Initialize workspace
    let members = vec!["shared".to_string(), "frontend".to_string(), "backend".to_string()];
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: workspace_root.join("cache"),
        workspace_dir: workspace_root.to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    manager.init_workspace(workspace_root, members).unwrap();
    
    // Generate workspace lock file (should succeed - no conflicts)
    if let Some(workspace) = manager.workspace_mut() {
        workspace.generate_lock_file().unwrap();
    }
    
    // Verify lock file
    let lock_file_path = workspace_root.join("CursedPackage.lock");
    assert!(lock_file_path.exists());
    
    let content = std::fs::read_to_string(&lock_file_path).unwrap();
    
    // Should contain all external dependencies exactly once
    assert!(content.contains("serde"));
    assert!(content.contains("uuid"));
    assert!(content.contains("web-sys"));
    assert!(content.contains("tokio"));
    
    // Load and validate the lock file
    let mut lock_manager = LockFileManager::new(&lock_file_path);
    lock_manager.load().unwrap();
    lock_manager.validate().unwrap();
    
    let packages = lock_manager.get_packages().unwrap();
    
    // Verify that serde appears only once despite being used by multiple members
    let serde_packages: Vec<_> = packages.iter().filter(|p| p.name == "serde").collect();
    assert_eq!(serde_packages.len(), 1);
    assert_eq!(serde_packages[0].version, "1.0.0");
}

#[tokio::test]
async fn test_workspace_member_addition_and_removal() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_root = temp_dir.path();
    
    // Initialize workspace with initial members
    let initial_members = vec!["core".to_string(), "utils".to_string()];
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: workspace_root.join("cache"),
        workspace_dir: workspace_root.to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    manager.init_workspace(workspace_root, initial_members).unwrap();
    
    // Verify initial state
    {
        let workspace = manager.workspace().unwrap();
        let config = workspace.config().unwrap();
        assert_eq!(config.members.len(), 2);
        assert!(config.members.contains(&"core".to_string()));
        assert!(config.members.contains(&"utils".to_string()));
    }
    
    // Add a new member
    {
        let workspace = manager.workspace_mut().unwrap();
        workspace.add_member("plugins".to_string()).unwrap();
    }
    
    // Verify addition
    {
        let workspace = manager.workspace().unwrap();
        let config = workspace.config().unwrap();
        assert_eq!(config.members.len(), 3);
        assert!(config.members.contains(&"plugins".to_string()));
    }
    
    // Remove a member
    {
        let workspace = manager.workspace_mut().unwrap();
        workspace.remove_member("utils").unwrap();
    }
    
    // Verify removal
    {
        let workspace = manager.workspace().unwrap();
        let config = workspace.config().unwrap();
        assert_eq!(config.members.len(), 2);
        assert!(!config.members.contains(&"utils".to_string()));
        assert!(config.members.contains(&"core".to_string()));
        assert!(config.members.contains(&"plugins".to_string()));
    }
    
    // Verify workspace configuration file was updated
    let package_file = workspace_root.join("CursedPackage.toml");
    let content = std::fs::read_to_string(&package_file).unwrap();
    assert!(content.contains("plugins"));
    assert!(!content.contains("utils") || content.split("\n").any(|line| line.contains("utils") && line.contains("#")));
}

#[tokio::test]
async fn test_complete_package_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();
    
    // Create a complete project structure
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: project_dir.join("cache"),
        workspace_dir: project_dir.to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    
    // Step 1: Initialize workspace
    let members = vec!["lib".to_string(), "bin".to_string()];
    manager.init_workspace(project_dir, members).unwrap();
    
    // Step 2: Create member directories and package files
    let lib_dir = project_dir.join("lib");
    let bin_dir = project_dir.join("bin");
    std::fs::create_dir_all(&lib_dir).unwrap();
    std::fs::create_dir_all(&bin_dir).unwrap();
    
    std::fs::write(lib_dir.join("CursedPackage.toml"), r#"
        name = "mylib"
        version = "1.0.0"
        description = "My library"
        authors = ["Dev"]
        
        [dependencies]
        serde = "1.0.0"
    "#).unwrap();
    
    std::fs::write(bin_dir.join("CursedPackage.toml"), r#"
        name = "mybin"
        version = "1.0.0"
        description = "My binary"
        authors = ["Dev"]
        
        [dependencies]
        mylib = "1.0.0"
        clap = "4.0.0"
    "#).unwrap();
    
    // Step 3: Install workspace dependencies
    manager.install_workspace().await.unwrap();
    
    // Step 4: Generate and validate lock file
    manager.generate_lock_file().unwrap();
    manager.validate_lock_file().unwrap();
    
    // Step 5: Build workspace in correct order
    manager.build_workspace().await.unwrap();
    
    // Step 6: Verify everything is in place
    let lock_file_path = project_dir.join("CursedPackage.lock");
    assert!(lock_file_path.exists());
    
    let workspace = manager.workspace().unwrap();
    assert_eq!(workspace.members().len(), 2);
    
    let build_order = workspace.get_build_order().unwrap();
    assert_eq!(build_order.len(), 2);
    // lib should be built before bin (since bin depends on lib)
    assert_eq!(build_order[0].name, "mylib");
    assert_eq!(build_order[1].name, "mybin");
    
    // Step 7: Clean workspace
    manager.clean_workspace().unwrap();
    
    // Verify target directories would be cleaned (we can't test actual build artifacts in this mock)
    // but the clean operation should succeed without errors
}

#[tokio::test]
async fn test_lock_file_deterministic_generation() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();
    
    let config = PackageManagerConfig {
        registry_url: "https://test-registry.example.com".to_string(),
        cache_dir: project_dir.join("cache"),
        workspace_dir: project_dir.to_path_buf(),
        max_cache_size: 1024 * 1024,
        timeout_seconds: 10,
        parallel_downloads: 2,
    };
    
    // Create two package managers with the same setup
    let mut manager1 = PackageManager::new(config.clone()).unwrap();
    let mut manager2 = PackageManager::new(config).unwrap();
    
    // Generate lock file twice with the same inputs
    manager1.generate_lock_file().unwrap();
    
    let lock_file_path = project_dir.join("CursedPackage.lock");
    let content1 = std::fs::read_to_string(&lock_file_path).unwrap();
    
    // Remove the file and generate again
    std::fs::remove_file(&lock_file_path).unwrap();
    manager2.generate_lock_file().unwrap();
    
    let content2 = std::fs::read_to_string(&lock_file_path).unwrap();
    
    // The lock files should be identical (except for timestamps)
    // Parse both and compare structure
    let lock1: cursed::package_manager::LockFile = toml::from_str(&content1).unwrap();
    let lock2: cursed::package_manager::LockFile = toml::from_str(&content2).unwrap();
    
    assert_eq!(lock1.version, lock2.version);
    assert_eq!(lock1.packages.len(), lock2.packages.len());
    
    // Packages should be in the same order
    for (pkg1, pkg2) in lock1.packages.iter().zip(lock2.packages.iter()) {
        assert_eq!(pkg1.name, pkg2.name);
        assert_eq!(pkg1.version, pkg2.version);
        assert_eq!(pkg1.checksum, pkg2.checksum);
        assert_eq!(pkg1.dependencies, pkg2.dependencies);
    }
}
