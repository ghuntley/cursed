/// Example demonstrating CURSED package manager lock file and workspace functionality
/// 
/// Shows how to use the complete package management ecosystem including:
/// - Lock file generation and validation
/// - Workspace initialization and management
/// - Dependency resolution and installation
/// - Build coordination

use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

use cursed::package_manager::{
    PackageManager, PackageManagerConfig, PackageMetadata,
    WorkspaceManager, WorkspaceConfig, LockFileManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 CURSED Package Manager Demo");
    println!("===============================\n");
    
    // Demo 1: Basic Package Management with Lock Files
    demo_basic_package_management().await?;
    
    // Demo 2: Workspace Management
    demo_workspace_management().await?;
    
    // Demo 3: Complete Development Workflow
    demo_complete_workflow().await?;
    
    println!("✅ All demos completed successfully!");
    Ok(())
}

async fn demo_basic_package_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Demo 1: Basic Package Management with Lock Files");
    println!("---------------------------------------------------");
    
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path();
    
    // Configure package manager
    let config = PackageManagerConfig {
        registry_url: "https://packages.cursed-lang.org".to_string(),
        cache_dir: project_dir.join("cache"),
        workspace_dir: project_dir.to_path_buf(),
        max_cache_size: 1024 * 1024 * 1024, // 1GB
        timeout_seconds: 30,
        parallel_downloads: 4,
    };
    
    let mut manager = PackageManager::new(config)?;
    
    // Create a sample package metadata
    let package_metadata = PackageMetadata {
        name: "my-awesome-package".to_string(),
        version: "1.0.0".to_string(),
        description: "An awesome CURSED package".to_string(),
        authors: vec!["Developer <dev@example.com>".to_string()],
        dependencies: {
            let mut deps = HashMap::new();
            deps.insert("json-parser".to_string(), "2.1.0".to_string());
            deps.insert("http-client".to_string(), "1.5.0".to_string());
            deps
        },
        dev_dependencies: {
            let mut dev_deps = HashMap::new();
            dev_deps.insert("test-framework".to_string(), "3.0.0".to_string());
            dev_deps
        },
        repository: Some("https://github.com/user/my-awesome-package".to_string()),
        license: Some("MIT".to_string()),
        keywords: vec!["web".to_string(), "api".to_string()],
        categories: vec!["web-programming".to_string()],
    };
    
    println!("📋 Package: {} v{}", package_metadata.name, package_metadata.version);
    println!("📝 Description: {}", package_metadata.description);
    println!("🔗 Dependencies: {:?}", package_metadata.dependencies);
    
    // Generate lock file
    println!("\n🔒 Generating lock file...");
    manager.generate_lock_file()?;
    
    let lock_file_path = project_dir.join("CursedPackage.lock");
    if lock_file_path.exists() {
        println!("✅ Lock file generated: {:?}", lock_file_path);
        
        // Display lock file content
        let content = std::fs::read_to_string(&lock_file_path)?;
        println!("📄 Lock file content preview:");
        println!("{}", content.lines().take(10).collect::<Vec<_>>().join("\n"));
        if content.lines().count() > 10 {
            println!("... (truncated)");
        }
    }
    
    // Validate lock file
    println!("\n🔍 Validating lock file...");
    manager.validate_lock_file()?;
    println!("✅ Lock file validation passed");
    
    // Show lock file status
    if let Some(lock_manager) = manager.lock_file_status() {
        println!("📊 Lock file status: {}", if lock_manager.exists() { "Present" } else { "Missing" });
    }
    
    println!("✅ Basic package management demo completed\n");
    Ok(())
}

async fn demo_workspace_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Demo 2: Workspace Management");
    println!("--------------------------------");
    
    let temp_dir = TempDir::new()?;
    let workspace_root = temp_dir.path();
    
    // Initialize workspace
    println!("🚀 Initializing workspace...");
    let members = vec![
        "core".to_string(),
        "utils".to_string(),
        "web-api".to_string(),
        "cli-tool".to_string(),
    ];
    
    let workspace = WorkspaceManager::init_workspace(workspace_root, members.clone())?;
    println!("✅ Workspace initialized with {} members", members.len());
    println!("📁 Workspace root: {:?}", workspace.root());
    
    // Display workspace configuration
    if let Some(config) = workspace.config() {
        println!("📋 Workspace configuration:");
        println!("   Members: {:?}", config.members);
        println!("   Exclude: {:?}", config.exclude);
        println!("   Dependencies: {:?}", config.dependencies);
    }
    
    // Create realistic member packages
    println!("\n📦 Creating workspace members...");
    create_workspace_member(workspace_root, "core", &[], "Core library with shared functionality")?;
    create_workspace_member(workspace_root, "utils", &[("core", "1.0.0")], "Utility functions")?;
    create_workspace_member(workspace_root, "web-api", &[("core", "1.0.0"), ("utils", "1.0.0")], "Web API server")?;
    create_workspace_member(workspace_root, "cli-tool", &[("core", "1.0.0"), ("utils", "1.0.0")], "Command-line interface")?;
    
    // Rediscover workspace with actual members
    let workspace = WorkspaceManager::discover(workspace_root)?;
    println!("✅ Discovered {} workspace members", workspace.members().len());
    
    // Display member information
    for member in workspace.members() {
        println!("   📦 {} v{} at {:?}", 
                member.name, 
                member.metadata.version, 
                member.path.file_name().unwrap_or_default());
        if !member.local_dependencies.is_empty() {
            println!("      🔗 Local deps: {:?}", member.local_dependencies);
        }
    }
    
    // Show build order
    println!("\n🏗️  Calculating build order...");
    let build_order = workspace.get_build_order()?;
    println!("📋 Build order:");
    for (i, member) in build_order.iter().enumerate() {
        println!("   {}. {}", i + 1, member.name);
    }
    
    // Generate workspace lock file
    println!("\n🔒 Generating workspace lock file...");
    let mut workspace_mut = workspace;
    workspace_mut.generate_lock_file()?;
    
    let lock_file_path = workspace_root.join("CursedPackage.lock");
    if lock_file_path.exists() {
        println!("✅ Workspace lock file generated");
        
        // Load and display lock file info
        let mut lock_manager = LockFileManager::new(&lock_file_path);
        lock_manager.load()?;
        
        if let Some(packages) = lock_manager.get_packages() {
            println!("📊 Lock file contains {} packages", packages.len());
            for package in packages.iter().take(3) {
                println!("   📦 {} v{}", package.name, package.version);
            }
            if packages.len() > 3 {
                println!("   ... and {} more", packages.len() - 3);
            }
        }
    }
    
    println!("✅ Workspace management demo completed\n");
    Ok(())
}

async fn demo_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Demo 3: Complete Development Workflow");
    println!("----------------------------------------");
    
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path();
    
    // Step 1: Initialize new project
    println!("1️⃣  Initializing new CURSED project...");
    
    let config = PackageManagerConfig {
        registry_url: "https://packages.cursed-lang.org".to_string(),
        cache_dir: project_dir.join("cache"),
        workspace_dir: project_dir.to_path_buf(),
        max_cache_size: 1024 * 1024 * 1024,
        timeout_seconds: 30,
        parallel_downloads: 4,
    };
    
    let mut manager = PackageManager::new(config)?;
    
    // Step 2: Set up workspace
    println!("2️⃣  Setting up workspace...");
    let members = vec!["backend".to_string(), "frontend".to_string(), "shared".to_string()];
    manager.init_workspace(project_dir, members)?;
    
    // Step 3: Create project structure
    println!("3️⃣  Creating project structure...");
    create_realistic_project_structure(project_dir)?;
    
    // Step 4: Discover workspace
    println!("4️⃣  Discovering workspace members...");
    if let Some(workspace) = manager.workspace() {
        println!("   Found {} members", workspace.members().len());
        for member in workspace.members() {
            println!("   📦 {}", member.name);
        }
    }
    
    // Step 5: Install dependencies
    println!("5️⃣  Installing workspace dependencies...");
    manager.install_workspace().await?;
    println!("   ✅ All dependencies installed");
    
    // Step 6: Generate lock file
    println!("6️⃣  Generating lock file...");
    manager.generate_lock_file()?;
    println!("   ✅ Lock file generated");
    
    // Step 7: Validate everything
    println!("7️⃣  Validating project...");
    manager.validate_lock_file()?;
    
    if let Some(workspace) = manager.workspace() {
        workspace.validate()?;
        println!("   ✅ Workspace validation passed");
    }
    
    // Step 8: Build project
    println!("8️⃣  Building project...");
    manager.build_workspace().await?;
    println!("   ✅ Project built successfully");
    
    // Step 9: Show final status
    println!("9️⃣  Final project status:");
    
    let lock_file_path = project_dir.join("CursedPackage.lock");
    if lock_file_path.exists() {
        let metadata = std::fs::metadata(&lock_file_path)?;
        println!("   🔒 Lock file: {} bytes", metadata.len());
    }
    
    if let Some(workspace) = manager.workspace() {
        let dependencies = workspace.list_dependencies();
        let total_deps: usize = dependencies.values().map(|v| v.len()).sum();
        println!("   📦 Total dependencies: {}", total_deps);
        
        let build_order = workspace.get_build_order()?;
        println!("   🏗️  Build order: {}", 
                build_order.iter().map(|m| &m.name).cloned().collect::<Vec<_>>().join(" → "));
    }
    
    // Step 10: Cleanup demo
    println!("🔟 Cleaning up...");
    manager.clean_workspace()?;
    println!("   ✅ Workspace cleaned");
    
    println!("✅ Complete workflow demo finished\n");
    Ok(())
}

fn create_workspace_member(
    workspace_root: &std::path::Path,
    name: &str,
    dependencies: &[(&str, &str)],
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let member_dir = workspace_root.join(name);
    std::fs::create_dir_all(&member_dir)?;
    
    let mut deps_section = String::new();
    if !dependencies.is_empty() {
        deps_section.push_str("\n[dependencies]\n");
        for (dep_name, dep_version) in dependencies {
            deps_section.push_str(&format!("{} = \"{}\"\n", dep_name, dep_version));
        }
    }
    
    let content = format!(r#"
name = "{}"
version = "1.0.0"
description = "{}"
authors = ["Demo Developer <demo@example.com>"]
license = "MIT"
keywords = ["demo", "example"]
categories = ["development-tools"]
{}
"#, name, description, deps_section);
    
    std::fs::write(member_dir.join("CursedPackage.toml"), content)?;
    
    // Create basic source structure
    let src_dir = member_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;
    std::fs::write(src_dir.join("main.csd"), format!(r#"
// {} - {}
slay main() {{
    capicola("Hello from {}!");
}}
"#, name, description, name))?;
    
    println!("   ✅ Created member: {}", name);
    Ok(())
}

fn create_realistic_project_structure(project_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // Backend service
    let backend_dir = project_dir.join("backend");
    std::fs::create_dir_all(backend_dir.join("src"))?;
    std::fs::write(backend_dir.join("CursedPackage.toml"), r#"
name = "backend"
version = "1.0.0"
description = "Backend API service"
authors = ["Dev Team <team@example.com>"]

[dependencies]
shared = "1.0.0"
web-framework = "2.0.0"
database-driver = "1.5.0"
auth-lib = "3.0.0"

[dev_dependencies]
test-utils = "1.0.0"
"#)?;
    
    // Frontend application
    let frontend_dir = project_dir.join("frontend");
    std::fs::create_dir_all(frontend_dir.join("src"))?;
    std::fs::write(frontend_dir.join("CursedPackage.toml"), r#"
name = "frontend"
version = "1.0.0"
description = "Frontend web application"
authors = ["Dev Team <team@example.com>"]

[dependencies]
shared = "1.0.0"
ui-framework = "4.0.0"
http-client = "2.0.0"
state-management = "1.2.0"

[dev_dependencies]
test-runner = "2.0.0"
"#)?;
    
    // Shared library
    let shared_dir = project_dir.join("shared");
    std::fs::create_dir_all(shared_dir.join("src"))?;
    std::fs::write(shared_dir.join("CursedPackage.toml"), r#"
name = "shared"
version = "1.0.0"
description = "Shared types and utilities"
authors = ["Dev Team <team@example.com>"]

[dependencies]
serialization = "1.0.0"
validation = "2.0.0"
crypto = "1.5.0"
"#)?;
    
    // Create some source files
    std::fs::write(backend_dir.join("src/main.csd"), r#"
// Backend API Server
import "shared::types";
import "web_framework";

slay main() {
    facts server = WebServer::new();
    server.listen("0.0.0.0:8080");
    capicola("Backend server started on port 8080");
}
"#)?;
    
    std::fs::write(frontend_dir.join("src/main.csd"), r#"
// Frontend Application
import "shared::types";
import "ui_framework";

slay main() {
    facts app = App::new();
    app.mount("#app");
    capicola("Frontend application started");
}
"#)?;
    
    std::fs::write(shared_dir.join("src/lib.csd"), r#"
// Shared Types and Utilities

pub squad User {
    id: String,
    name: String,
    email: String,
}

pub slay validate_email(email: String) -> bool {
    // Email validation logic
    email.contains("@")
}
"#)?;
    
    println!("   ✅ Created realistic project structure");
    Ok(())
}
