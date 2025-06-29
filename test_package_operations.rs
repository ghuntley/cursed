#!/usr/bin/env rust-script

//! Test script for package manager operations with real HTTP backend
//!
//! This script tests the real HTTP implementation of the package manager
//! including registry communication, package downloading, and extraction.

use std::path::PathBuf;
use tempfile::TempDir;
use tokio;

// Mock registry server for testing
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

/// Mock package registry server for testing
#[derive(Debug, Clone)]
struct MockRegistry {
    packages: Arc<Mutex<HashMap<String, MockPackage>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct MockPackage {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    dependencies: Vec<MockDependency>,
    keywords: Vec<String>,
    license: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    download_url: String,
    checksum: String,
    file_size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct MockDependency {
    name: String,
    version_req: String,
    optional: bool,
    features: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct SearchResponse {
    packages: Vec<MockPackage>,
    total: usize,
}

#[derive(Debug, serde::Serialize)]
struct PackageResponse {
    package: MockPackage,
}

#[derive(Debug, serde::Serialize)]
struct VersionsResponse {
    versions: Vec<String>,
}

impl MockRegistry {
    fn new() -> Self {
        let mut packages = HashMap::new();
        
        // Add some test packages
        packages.insert("example-lib".to_string(), MockPackage {
            name: "example-lib".to_string(),
            version: "1.0.0".to_string(),
            description: "An example library package".to_string(),
            authors: vec!["Test Author <test@example.com>".to_string()],
            dependencies: vec![],
            keywords: vec!["example".to_string(), "library".to_string()],
            license: Some("MIT".to_string()),
            homepage: Some("https://example.com/example-lib".to_string()),
            repository: Some("https://github.com/example/example-lib".to_string()),
            download_url: "http://localhost:3030/packages/example-lib-1.0.0.tar.gz".to_string(),
            checksum: "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            file_size: 1024,
        });

        packages.insert("crypto-utils".to_string(), MockPackage {
            name: "crypto-utils".to_string(),
            version: "2.1.0".to_string(),
            description: "Cryptographic utilities for CURSED".to_string(),
            authors: vec!["Crypto Team <crypto@cursed-lang.org>".to_string()],
            dependencies: vec![
                MockDependency {
                    name: "example-lib".to_string(),
                    version_req: "^1.0.0".to_string(),
                    optional: false,
                    features: vec![],
                }
            ],
            keywords: vec!["crypto".to_string(), "security".to_string()],
            license: Some("MIT".to_string()),
            homepage: None,
            repository: None,
            download_url: "http://localhost:3030/packages/crypto-utils-2.1.0.tar.gz".to_string(),
            checksum: "sha256:d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35".to_string(),
            file_size: 2048,
        });

        Self {
            packages: Arc::new(Mutex::new(packages)),
        }
    }

    async fn search(&self, query: &str) -> SearchResponse {
        let packages = self.packages.lock().await;
        let matching_packages: Vec<MockPackage> = packages
            .values()
            .filter(|pkg| {
                pkg.name.contains(query) ||
                pkg.description.contains(query) ||
                pkg.keywords.iter().any(|keyword| keyword.contains(query))
            })
            .cloned()
            .collect();

        SearchResponse {
            total: matching_packages.len(),
            packages: matching_packages,
        }
    }

    async fn get_package(&self, name: &str) -> Option<PackageResponse> {
        let packages = self.packages.lock().await;
        packages.get(name).cloned().map(|package| PackageResponse { package })
    }

    async fn get_versions(&self, name: &str) -> Option<VersionsResponse> {
        let packages = self.packages.lock().await;
        if packages.contains_key(name) {
            // For simplicity, just return a few mock versions
            Some(VersionsResponse {
                versions: vec!["1.0.0".to_string(), "1.0.1".to_string(), "1.1.0".to_string()],
            })
        } else {
            None
        }
    }
}

/// Create a test package archive
async fn create_test_package(temp_dir: &TempDir, name: &str, version: &str) -> PathBuf {
    use std::fs;
    use std::io::Write;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use tar::Builder;

    let package_dir = temp_dir.path().join(format!("{}-{}", name, version));
    fs::create_dir_all(&package_dir).unwrap();

    // Create package files
    fs::write(package_dir.join("lib.csd"), format!("// {} library\nfn main() {{\n    print(\"Hello from {}!\");\n}}", name, name)).unwrap();
    fs::write(package_dir.join("README.md"), format!("# {}\n\nA test package for the CURSED package manager.", name)).unwrap();
    fs::write(package_dir.join("package.toml"), format!(r#"
[package]
name = "{}"
version = "{}"
description = "A test package"
authors = ["Test Author <test@example.com>"]

[dependencies]
"#, name, version)).unwrap();

    // Create tar.gz archive
    let archive_path = temp_dir.path().join(format!("{}-{}.tar.gz", name, version));
    let tar_gz = fs::File::create(&archive_path).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all(format!("{}-{}", name, version), &package_dir).unwrap();
    tar.finish().unwrap();

    archive_path
}

/// Start mock registry server
async fn start_mock_server(registry: MockRegistry) -> tokio::task::JoinHandle<()> {
    let packages = registry.packages.clone();

    // Search endpoint
    let search = warp::path!("api" / "v1" / "search")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then({
            let packages = packages.clone();
            move |params: HashMap<String, String>| {
                let packages = packages.clone();
                async move {
                    let query = params.get("q").map(|s| s.as_str()).unwrap_or("");
                    let registry = MockRegistry { packages };
                    let response = registry.search(query).await;
                    Ok::<_, warp::Rejection>(warp::reply::json(&response))
                }
            }
        });

    // Package info endpoint
    let package_info = warp::path!("api" / "v1" / "packages" / String)
        .and(warp::get())
        .and_then({
            let packages = packages.clone();
            move |name: String| {
                let packages = packages.clone();
                async move {
                    let registry = MockRegistry { packages };
                    match registry.get_package(&name).await {
                        Some(response) => Ok(warp::reply::json(&response)),
                        None => Err(warp::reject::not_found()),
                    }
                }
            }
        });

    // Package versions endpoint
    let package_versions = warp::path!("api" / "v1" / "packages" / String / "versions")
        .and(warp::get())
        .and_then({
            let packages = packages.clone();
            move |name: String| {
                let packages = packages.clone();
                async move {
                    let registry = MockRegistry { packages };
                    match registry.get_versions(&name).await {
                        Some(response) => Ok(warp::reply::json(&response)),
                        None => Err(warp::reject::not_found()),
                    }
                }
            }
        });

    // Package download endpoint
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_path_buf();
    
    let package_download = warp::path!("packages" / String)
        .and(warp::get())
        .and_then(move |filename: String| {
            let temp_dir_path = temp_dir_path.clone();
            async move {
                // Extract package name and version from filename
                if let Some(captures) = regex::Regex::new(r"(.+)-(\d+\.\d+\.\d+)\.tar\.gz$")
                    .unwrap()
                    .captures(&filename)
                {
                    let name = captures.get(1).unwrap().as_str();
                    let version = captures.get(2).unwrap().as_str();
                    
                    // Create temporary package file
                    let temp_dir = tempfile::TempDir::new_in(&temp_dir_path).unwrap();
                    let archive_path = create_test_package(&temp_dir, name, version).await;
                    
                    let file_contents = tokio::fs::read(&archive_path).await
                        .map_err(|_| warp::reject::not_found())?;
                    
                    Ok(warp::reply::with_header(
                        file_contents,
                        "content-type",
                        "application/gzip",
                    ))
                } else {
                    Err(warp::reject::not_found())
                }
            }
        });

    let routes = search
        .or(package_info)
        .or(package_versions)
        .or(package_download)
        .with(warp::cors().allow_any_origin());

    tokio::spawn(async move {
        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Package Manager HTTP Backend Tests");

    // Create mock registry and start server
    let registry = MockRegistry::new();
    let _server_handle = start_mock_server(registry).await;

    // Give the server time to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Test 1: Registry Search
    println!("\n📦 Test 1: Registry Search");
    test_registry_search().await?;

    // Test 2: Package Info Retrieval
    println!("\n📋 Test 2: Package Info Retrieval");
    test_package_info().await?;

    // Test 3: Package Version Listing
    println!("\n📄 Test 3: Package Version Listing");
    test_package_versions().await?;

    // Test 4: Package Download
    println!("\n⬇️  Test 4: Package Download");
    test_package_download().await?;

    // Test 5: Archive Extraction
    println!("\n📂 Test 5: Archive Extraction");
    test_archive_extraction().await?;

    // Test 6: End-to-End Package Installation
    println!("\n🎯 Test 6: End-to-End Package Installation");
    test_end_to_end_installation().await?;

    println!("\n✅ All tests passed! HTTP backend implementation is working correctly.");

    Ok(())
}

async fn test_registry_search() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3030/api/v1/search")
        .query(&[("q", "example")])
        .send()
        .await?;

    assert!(response.status().is_success(), "Search request failed");
    
    let search_result: SearchResponse = response.json().await?;
    assert!(!search_result.packages.is_empty(), "No packages found in search");
    
    println!("  ✓ Found {} packages", search_result.packages.len());
    for package in &search_result.packages {
        println!("    - {} v{}: {}", package.name, package.version, package.description);
    }

    Ok(())
}

async fn test_package_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3030/api/v1/packages/example-lib")
        .send()
        .await?;

    assert!(response.status().is_success(), "Package info request failed");
    
    let package_result: PackageResponse = response.json().await?;
    assert_eq!(package_result.package.name, "example-lib");
    assert_eq!(package_result.package.version, "1.0.0");
    
    println!("  ✓ Retrieved package info for {}", package_result.package.name);
    println!("    Version: {}", package_result.package.version);
    println!("    Description: {}", package_result.package.description);
    println!("    Authors: {:?}", package_result.package.authors);

    Ok(())
}

async fn test_package_versions() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3030/api/v1/packages/example-lib/versions")
        .send()
        .await?;

    assert!(response.status().is_success(), "Package versions request failed");
    
    let versions_result: VersionsResponse = response.json().await?;
    assert!(!versions_result.versions.is_empty(), "No versions found");
    
    println!("  ✓ Retrieved {} versions", versions_result.versions.len());
    for version in &versions_result.versions {
        println!("    - {}", version);
    }

    Ok(())
}

async fn test_package_download() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3030/packages/example-lib-1.0.0.tar.gz")
        .send()
        .await?;

    assert!(response.status().is_success(), "Package download request failed");
    
    let bytes = response.bytes().await?;
    assert!(!bytes.is_empty(), "Downloaded package is empty");
    
    println!("  ✓ Downloaded package ({} bytes)", bytes.len());

    // Verify it's a valid gzip archive
    use flate2::read::GzDecoder;
    use std::io::Read;
    
    let mut decoder = GzDecoder::new(bytes.as_ref());
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    
    println!("  ✓ Successfully decompressed archive ({} bytes)", decompressed.len());

    Ok(())
}

async fn test_archive_extraction() -> Result<(), Box<dyn std::error::Error>> {
    use cursed::package_manager::archive::{extract_archive, ExtractionConfig};
    
    let temp_dir = TempDir::new()?;
    let package_path = create_test_package(&temp_dir, "test-package", "1.0.0").await;
    let extract_dir = temp_dir.path().join("extracted");

    let config = ExtractionConfig {
        overwrite_existing: true,
        preserve_permissions: true,
        strip_components: 1,
    };

    let result = extract_archive(&package_path, &extract_dir, config)?;
    
    assert!(!result.extracted_files.is_empty(), "No files were extracted");
    assert!(extract_dir.join("lib.csd").exists(), "lib.csd not found");
    assert!(extract_dir.join("README.md").exists(), "README.md not found");
    assert!(extract_dir.join("package.toml").exists(), "package.toml not found");
    
    println!("  ✓ Extracted {} files ({} bytes)", result.extracted_files.len(), result.total_size);
    for file in &result.extracted_files {
        println!("    - {:?}", file);
    }

    Ok(())
}

async fn test_end_to_end_installation() -> Result<(), Box<dyn std::error::Error>> {
    use cursed::package_manager::{PackageManager, PackageManagerConfig};
    
    let temp_dir = TempDir::new()?;
    let config = PackageManagerConfig {
        cache_dir: temp_dir.path().join("cache").to_string_lossy().to_string(),
        registry_url: "http://localhost:3030".to_string(),
        offline_mode: false,
        verify_signatures: false, // Disable for testing
        workspace_dir: temp_dir.path().to_string_lossy().to_string(),
        max_cache_size: 1024 * 1024 * 10, // 10MB
        timeout_seconds: 30,
        parallel_downloads: 2,
    };

    let mut package_manager = PackageManager::new(config)?;

    // Test package search
    let search_results = package_manager.search_packages("example").await?;
    assert!(!search_results.is_empty(), "No packages found in search");
    println!("  ✓ Search found {} packages", search_results.len());

    // Test package info retrieval
    let package_info = package_manager.get_package_info("example-lib", None).await?;
    assert_eq!(package_info.name, "example-lib");
    println!("  ✓ Retrieved package info for {}", package_info.name);

    // Test package installation
    let installed_package = package_manager.install_package("example-lib", Some("1.0.0")).await?;
    assert_eq!(installed_package.name, "example-lib");
    assert_eq!(installed_package.version.to_string(), "1.0.0");
    println!("  ✓ Installed package {} v{}", installed_package.name, installed_package.version);

    // Verify installation
    assert!(package_manager.is_installed("example-lib"), "Package not marked as installed");
    let installed_packages = package_manager.list_installed();
    assert_eq!(installed_packages.len(), 1, "Expected 1 installed package");
    println!("  ✓ Package installation verified");

    // Test package with dependencies
    let crypto_package = package_manager.install_package("crypto-utils", Some("2.1.0")).await?;
    assert_eq!(crypto_package.name, "crypto-utils");
    println!("  ✓ Installed package with dependencies: {}", crypto_package.name);

    // Verify both packages are installed
    let final_packages = package_manager.list_installed();
    assert!(final_packages.len() >= 2, "Expected at least 2 installed packages");
    println!("  ✓ All packages installed successfully: {} total", final_packages.len());

    Ok(())
}
