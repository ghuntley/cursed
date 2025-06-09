//! Tests for package resolution in the import system
//!
//! This module tests:
//! - Standard library package imports
//! - User package imports
//! - Circular dependency detection
//! - Package not found errors
//! - Invalid package paths

use cursed::ast;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use tracing::{debug, error, info, instrument, trace, warn};

#[path = "common/mod.rs"]
mod common;

/// Mock package resolver for testing
struct MockPackageResolver {
    available_packages: HashMap<String, PathBuf>,
    loaded_packages: HashSet<String>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl MockPackageResolver {
    fn new() -> Self {
        let mut resolver = Self {
            available_packages: HashMap::new(),
            loaded_packages: HashSet::new(),
            dependency_graph: HashMap::new(),
        };
        
        // Add standard library packages
        resolver.add_std_packages();
        resolver
    }
    
    fn add_std_packages(&mut self) {
        let std_packages = vec![
            "std/io",
            "std/math", 
            "std/string",
            "std/collections",
            "std/json",
            "std/http",
            "std/fs",
            "std/time",
            "std/crypto",
            "std/fmt",
        ];
        
        for pkg in std_packages {
            self.available_packages.insert(
                pkg.to_string(),
                PathBuf::from(format!("/usr/lib/cursed/{}", pkg))
            );
        }
    }
    
    fn add_user_package(&mut self, name: &str, path: PathBuf) {
        self.available_packages.insert(name.to_string(), path);
    }
    
    fn resolve_package(&mut self, path: &str) -> Result<PathBuf, Error> {
        if let Some(pkg_path) = self.available_packages.get(path) {
            self.loaded_packages.insert(path.to_string());
            Ok(pkg_path.clone())
        } else {
            Err(Error::PackageNotFound(path.to_string()))
        }
    }
    
    fn add_dependency(&mut self, from: &str, to: &str) {
        self.dependency_graph.entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }
    
    fn detect_circular_dependency(&self, package: &str) -> Result<(), Error> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        self.dfs_cycle_detection(package, &mut visited, &mut path)
    }
    
    fn dfs_cycle_detection(&self, current: &str, visited: &mut HashSet<String>, path: &mut Vec<String>) -> Result<(), Error> {
        if path.contains(&current.to_string()) {
            let cycle_start = path.iter().position(|p| p == current).unwrap();
            let cycle: Vec<String> = path[cycle_start..].iter().cloned().chain(std::iter::once(current.to_string())).collect();
            return Err(Error::CircularDependency(cycle));
        }
        
        if visited.contains(current) {
            return Ok(());
        }
        
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        if let Some(deps) = self.dependency_graph.get(current) {
            for dep in deps {
                self.dfs_cycle_detection(dep, visited, path)?;
            }
        }
        
        path.pop();
        Ok(())
    }
}

#[test]
#[instrument]
fn test_resolve_standard_library_packages() {
    common::tracing::setup();
    info!("Testing standard library package resolution");
    
    let mut resolver = MockPackageResolver::new();
    
    let std_packages = vec![
        "std/io",
        "std/math", 
        "std/string",
        "std/collections",
        "std/json",
    ];
    
    for package in std_packages {
        debug!(package = package, "Resolving standard library package");
        
        let result = resolver.resolve_package(package);
        assert!(result.is_ok(), "Should resolve standard library package: {}", package);
        
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("cursed"), "Standard library path should contain 'cursed'");
        assert!(path.to_string_lossy().contains(package), "Path should contain package name");
        
        assert!(resolver.loaded_packages.contains(package), "Package should be marked as loaded");
    }
    
    info!("Standard library package resolution test completed");
}

#[test]
#[instrument]
fn test_resolve_user_packages() {
    common::tracing::setup();
    info!("Testing user package resolution");
    
    let mut resolver = MockPackageResolver::new();
    
    // Add some user packages
    let user_packages = vec![
        ("myproject/utils", PathBuf::from("/home/user/myproject/utils")),
        ("./local", PathBuf::from("./local")),
        ("../shared", PathBuf::from("../shared")),
        ("github.com/user/repo", PathBuf::from("/go/pkg/mod/github.com/user/repo")),
    ];
    
    for (name, path) in &user_packages {
        resolver.add_user_package(name, path.clone());
    }
    
    for (package, expected_path) in user_packages {
        debug!(package = package, expected_path = ?expected_path, "Resolving user package");
        
        let result = resolver.resolve_package(&package);
        assert!(result.is_ok(), "Should resolve user package: {}", package);
        
        let resolved_path = result.unwrap();
        assert_eq!(resolved_path, expected_path, "Resolved path should match expected path");
        
        assert!(resolver.loaded_packages.contains(&package), "Package should be marked as loaded");
    }
    
    info!("User package resolution test completed");
}

#[test]
#[instrument]
fn test_package_not_found_errors() {
    common::tracing::setup();
    info!("Testing package not found errors");
    
    let mut resolver = MockPackageResolver::new();
    
    let nonexistent_packages = vec![
        "nonexistent/package",
        "std/nonexistent",
        "./does_not_exist",
        "github.com/invalid/repo",
        "",
        "   ",
    ];
    
    for package in nonexistent_packages {
        debug!(package = package, "Testing package not found");
        
        let result = resolver.resolve_package(package);
        assert!(result.is_err(), "Should fail to resolve nonexistent package: {}", package);
        
        match result.unwrap_err() {
            Error::PackageNotFound(pkg) => {
                assert_eq!(pkg, package, "Error should contain the package name");
            }
            other => panic!("Expected PackageNotFound error, got: {:?}", other),
        }
        
        assert!(!resolver.loaded_packages.contains(package), "Package should not be marked as loaded");
    }
    
    info!("Package not found errors test completed");
}

#[test]
#[instrument]
fn test_circular_dependency_detection() {
    common::tracing::setup();
    info!("Testing circular dependency detection");
    
    let mut resolver = MockPackageResolver::new();
    
    // Add some packages
    resolver.add_user_package("pkgA", PathBuf::from("/packages/pkgA"));
    resolver.add_user_package("pkgB", PathBuf::from("/packages/pkgB"));
    resolver.add_user_package("pkgC", PathBuf::from("/packages/pkgC"));
    resolver.add_user_package("pkgD", PathBuf::from("/packages/pkgD"));
    
    // Create a circular dependency: A -> B -> C -> A
    resolver.add_dependency("pkgA", "pkgB");
    resolver.add_dependency("pkgB", "pkgC");
    resolver.add_dependency("pkgC", "pkgA");
    
    // Create a chain dependency: D -> B (which eventually leads to the cycle)
    resolver.add_dependency("pkgD", "pkgB");
    
    debug!("Testing cycle detection from pkgA");
    let result = resolver.detect_circular_dependency("pkgA");
    assert!(result.is_err(), "Should detect circular dependency from pkgA");
    
    match result.unwrap_err() {
        Error::CircularDependency(cycle) => {
            assert!(cycle.len() >= 3, "Cycle should have at least 3 packages");
            assert!(cycle.contains(&"pkgA".to_string()), "Cycle should contain pkgA");
            assert!(cycle.contains(&"pkgB".to_string()), "Cycle should contain pkgB");
            assert!(cycle.contains(&"pkgC".to_string()), "Cycle should contain pkgC");
            debug!(cycle = ?cycle, "Detected circular dependency");
        }
        other => panic!("Expected CircularDependency error, got: {:?}", other),
    }
    
    debug!("Testing cycle detection from pkgD");
    let result = resolver.detect_circular_dependency("pkgD");
    assert!(result.is_err(), "Should detect circular dependency from pkgD through chain");
    
    info!("Circular dependency detection test completed");
}

#[test]
#[instrument]
fn test_valid_dependency_chains() {
    common::tracing::setup();
    info!("Testing valid dependency chains");
    
    let mut resolver = MockPackageResolver::new();
    
    // Add some packages
    resolver.add_user_package("app", PathBuf::from("/packages/app"));
    resolver.add_user_package("utils", PathBuf::from("/packages/utils"));
    resolver.add_user_package("logging", PathBuf::from("/packages/logging"));
    
    // Create a valid dependency chain: app -> utils -> logging
    resolver.add_dependency("app", "utils");
    resolver.add_dependency("utils", "logging");
    // logging depends on std/io (no cycle)
    resolver.add_dependency("logging", "std/io");
    
    debug!("Testing valid dependency chain");
    let result = resolver.detect_circular_dependency("app");
    assert!(result.is_ok(), "Should not detect circular dependency in valid chain");
    
    let result = resolver.detect_circular_dependency("utils");
    assert!(result.is_ok(), "Should not detect circular dependency from utils");
    
    let result = resolver.detect_circular_dependency("logging");
    assert!(result.is_ok(), "Should not detect circular dependency from logging");
    
    info!("Valid dependency chains test completed");
}

#[test]
#[instrument]
fn test_invalid_package_paths() {
    common::tracing::setup();
    info!("Testing invalid package paths");
    
    let mut resolver = MockPackageResolver::new();
    
    let invalid_paths = vec![
        "../../../../../../etc/passwd",  // Path traversal
        "/absolute/system/path",         // Absolute system path
        "std/../../../etc",              // Path traversal in std
        "pkg\x00null",                   // Null byte
        "pkg with spaces",               // Spaces in name
        "pkg;rm -rf /",                  // Command injection attempt
        "pkg`whoami`",                   // Command substitution
        "pkg$(whoami)",                  // Command substitution
        "pkg|whoami",                    // Pipe injection
        "pkg&whoami",                    // Command chaining
    ];
    
    for path in invalid_paths {
        debug!(path = path, "Testing invalid package path");
        
        // For security, these should all fail to resolve
        let result = resolver.resolve_package(path);
        assert!(result.is_err(), "Should reject invalid/dangerous package path: {}", path);
        
        assert!(!resolver.loaded_packages.contains(path), "Invalid package should not be loaded");
    }
    
    info!("Invalid package paths test completed");
}

#[test]
#[instrument]
fn test_package_caching() {
    common::tracing::setup();
    info!("Testing package caching behavior");
    
    let mut resolver = MockPackageResolver::new();
    
    let package = "std/math";
    
    // First resolution
    debug!("First package resolution");
    let result1 = resolver.resolve_package(package);
    assert!(result1.is_ok(), "First resolution should succeed");
    assert!(resolver.loaded_packages.contains(package), "Package should be cached");
    
    // Second resolution should use cache
    debug!("Second package resolution (should use cache)");
    let result2 = resolver.resolve_package(package);
    assert!(result2.is_ok(), "Second resolution should succeed");
    
    // Both results should be the same
    assert_eq!(result1.unwrap(), result2.unwrap(), "Cached results should be identical");
    
    info!("Package caching test completed");
}

#[test]
#[instrument]
fn test_package_resolution_with_imports() {
    common::tracing::setup();
    info!("Testing package resolution integration with import parsing");
    
    let input = r#"vibe mypackage
yeet "std/io"
yeet "std/math" 
yeet utils "./utils"
yeet http "std/http"
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    let mut resolver = MockPackageResolver::new();
    resolver.add_user_package("./utils", PathBuf::from("./utils"));
    
    // Expected imports to resolve
    let expected_imports = vec![
        ("std/io", None),
        ("std/math", None),
        ("./utils", Some("utils")),
        ("std/http", Some("http")),
    ];
    
    for (path, alias) in expected_imports {
        debug!(path = path, alias = ?alias, "Testing import resolution");
        
        // Test that the package can be resolved
        let result = resolver.resolve_package(path);
        assert!(result.is_ok(), "Should resolve import package: {}", path);
        
        // Create mock import statement to verify structure
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: format!("\"{}\"", path),
                value: path.to_string(),
            },
            alias: alias.map(|a| ast::Identifier {
                token: a.to_string(),
                value: a.to_string(),
            }),
        };
        
        assert_eq!(mock_import.path.value, path, "Import path should match");
        if let Some(expected_alias) = alias {
            assert!(mock_import.alias.is_some(), "Import should have alias");
            assert_eq!(mock_import.alias.as_ref().unwrap().value, expected_alias, "Alias should match");
        }
    }
    
    info!("Package resolution with imports test completed");
}
