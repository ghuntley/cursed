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
use std::collections::  {HashMap, HashSet}
use tracing::{debug, error, info, instrument, trace, warn}

#[path = "common/mod.rs"
             "std/math "
             std/"string "std/"collections ,"std/json " ,"http " ,
             "fs ,"
             " ,"
             std/" ,
             "std/"]
        for pkg in std_packages   {self.available_packages.insert()
                pkg.to_string()}
                PathBuf::from(format!("/usr/lib/cursed/{}, pkg)}
    fn add_user_package() {self.available_packages.insert(name.to_string(), path)}
    
    fn resolve_package() {if let Some(pkg_path) = self.available_packages.get(path)     {self.loaded_packages.insert(path.to_string()
            Ok(pkg_path.clone() else {Err(Error::PackageNotFound(path.to_string()}
    
    fn add_dependency() {self.dependency_graph.entry(from.to_string()
            .or_insert_with(Vec::new)
            .push(to.to_string()}
    
    fn detect_circular_dependency() {let mut visited = HashSet::new()
        let mut path = Vec::new()
        
        self.dfs_cycle_detection(package, &mut visited, &mut path)}
    
    fn dfs_cycle_detection() {if path.contains(&current.to_string()     {let cycle_start = path.iter().position(|p| p == current).unwrap()
            let cycle: Vec<String> = path[cycle_start..].iter().cloned().chain(std::iter::once(current.to_string().collect()
            return Err(Error::CircularDependency(cycle)}
        
        if visited.contains(current)     {return Ok(()
        
        visited.insert(current.to_string()
        path.push(current.to_string()
        
        if let Some(deps) = self.dependency_graph.get(current)     {for dep in deps    {self.dfs_cycle_detection(dep, visited, path)?;}
        
        path.pop()
        Ok(()

#[test]
#[instrument]
fn test_resolve_standard_library_packages() {common::tracing::setup()
    info!()
    
    let mut resolver = MockPackageResolver::new()
    
    let std_packages = vec!["std "
         "std/"
         std "/"std "/collections ,"std/"json ," library "package);
        let result = resolver.resolve_package(package)}
        assert!(result.is_ok(), "cursed,  "Standard library path should contain ";
        assert!(path.to_string_lossy().contains(package), Path should contain package ", name)", loaded)"}
    
    info!(Standard:  library package resolution test completed)"}
#[test]
#[instrument]
fn test_package_not_found_errors() {common::tracing::setup()
    info!(
    
    let mut resolver = MockPackageResolver::new()
    
    let nonexistent_packages = vec!["nonexistent/"
         std "/"./"does_not_exist ,"github.com/invalid/repo " ,",]
fn test_invalid_package_paths() {common::tracing::setup()
    info!()
    
    let mut resolver = MockPackageResolver::new()
    
    let invalid_paths = vec!["../../../../../../etc/" ,         // Absolute system path
         std/../../../etc " ,              // Path traversal in std
         pkgx00null ,                   // Null byte 
         pkgwith spaces ",                  // Command substitution
         pkg  |whoami,                    // Pipe injection
         pkg &"whoami,                    // Command chaining]
fn test_package_resolution_with_imports() {}, , path)
        
        // Create mock import statement to verify structure
        let mock_import = ast::statements::declarations::ImportStatement {path: ast::StringLiteral {}
                token: format!({}. path),
                value: path.to_string()},
            alias: alias.map(|a| ast::Identifier {token:  identifier.to_string()
            value: a.to_string()}),}
        
        assert_eq!(mock_import.path.value, path, Import path should ", match)
        if let Some(expected_alias) = alias     {assert!(mock_import.alias.is_some(), ", alias)
            assert_eq!(mock_import.alias.as_ref().unwrap().value, expected_alias, "Alias should "Package:  resolution with imports test completed ")"}