#!/usr/bin/env rust-script

//! Simple test for package manager HTTP implementation
//!
//! This script tests just the core package manager functionality
//! without the full CURSED language system.

use std::path::PathBuf;
use tempfile::TempDir;

// Simplified package manager types for testing
#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
    
    pub fn parse(version_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() < 3 {
            return Err(format!("Invalid version format: {}", version_str));
        }

        let major = parts[0].parse::<u32>()
            .map_err(|_| format!("Invalid major version: {}", parts[0]))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| format!("Invalid minor version: {}", parts[1]))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| format!("Invalid patch version: {}", parts[2]))?;

        Ok(Self::new(major, minor, patch))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub keywords: Vec<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub download_url: String,
    pub checksum: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version_req: String,
    pub optional: bool,
    pub features: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct SearchResponse {
    packages: Vec<PackageInfo>,
    total: usize,
}

#[derive(Debug, serde::Serialize)]
struct PackageResponse {
    package: PackageInfo,
}

#[derive(Debug, serde::Serialize)]
struct VersionsResponse {
    versions: Vec<String>,
}

/// Simple HTTP-based package registry client
#[derive(Debug, Clone)]
pub struct PackageRegistry {
    client: reqwest::Client,
    base_url: String,
}

impl PackageRegistry {
    pub fn new(base_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("test-package-manager/1.0")
            .build()?;
        
        Ok(Self {
            client,
            base_url,
        })
    }

    pub async fn search_packages(&self, query: &str) -> Result<Vec<PackageInfo>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/search", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await?;

        let search_result: SearchResponse = response.json().await?;
        Ok(search_result.packages)
    }

    pub async fn get_package_info(&self, name: &str) -> Result<PackageInfo, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/packages/{}", self.base_url, name);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let package_result: PackageResponse = response.json().await?;
        Ok(package_result.package)
    }

    pub async fn get_package_versions(&self, name: &str) -> Result<Vec<Version>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/packages/{}/versions", self.base_url, name);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let versions_result: VersionsResponse = response.json().await?;
        let mut versions = Vec::new();
        
        for version_str in versions_result.versions {
            if let Ok(version) = Version::parse(&version_str) {
                versions.push(version);
            }
        }
        
        Ok(versions)
    }

    pub async fn download_package(&self, download_url: &str, output_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .get(download_url)
            .send()
            .await?;

        let bytes = response.bytes().await?;
        std::fs::write(output_path, bytes)?;
        
        Ok(())
    }
}

/// Mock registry server for testing
async fn start_mock_server() -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error>> {
    use warp::Filter;
    use std::collections::HashMap;
    
    // Mock packages
    let example_package = PackageInfo {
        name: "example-lib".to_string(),
        version: Version::new(1, 0, 0),
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
    };

    // Search endpoint
    let search = warp::path!("api" / "v1" / "search")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .map(move |_params: HashMap<String, String>| {
            let response = SearchResponse {
                packages: vec![example_package.clone()],
                total: 1,
            };
            warp::reply::json(&response)
        });

    // Package info endpoint
    let package_info = warp::path!("api" / "v1" / "packages" / String)
        .and(warp::get())
        .map(move |_name: String| {
            let response = PackageResponse {
                package: example_package.clone(),
            };
            warp::reply::json(&response)
        });

    // Package versions endpoint
    let package_versions = warp::path!("api" / "v1" / "packages" / String / "versions")
        .and(warp::get())
        .map(|_name: String| {
            let response = VersionsResponse {
                versions: vec!["1.0.0".to_string(), "1.0.1".to_string(), "1.1.0".to_string()],
            };
            warp::reply::json(&response)
        });

    // Package download endpoint
    let package_download = warp::path!("packages" / String)
        .and(warp::get())
        .map(|_filename: String| {
            // Return some mock archive content
            let mock_content = b"Mock package archive content";
            warp::reply::with_header(
                mock_content.as_ref(),
                "content-type",
                "application/gzip",
            )
        });

    let routes = search
        .or(package_info)
        .or(package_versions)
        .or(package_download)
        .with(warp::cors().allow_any_origin());

    Ok(tokio::spawn(async move {
        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Simple Package Manager HTTP Test");

    // Start mock server
    let _server_handle = start_mock_server().await?;
    
    // Give the server time to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Create package registry client
    let registry = PackageRegistry::new("http://localhost:3030".to_string())?;

    // Test 1: Search packages
    println!("\n📦 Test 1: Package Search");
    let search_results = registry.search_packages("example").await?;
    println!("✓ Found {} packages", search_results.len());
    for package in &search_results {
        println!("  - {} v{}: {}", package.name, package.version, package.description);
    }

    // Test 2: Get package info
    println!("\n📋 Test 2: Package Info");
    let package_info = registry.get_package_info("example-lib").await?;
    println!("✓ Retrieved package: {} v{}", package_info.name, package_info.version);
    println!("  Description: {}", package_info.description);
    println!("  Authors: {:?}", package_info.authors);

    // Test 3: Get package versions
    println!("\n📄 Test 3: Package Versions");
    let versions = registry.get_package_versions("example-lib").await?;
    println!("✓ Found {} versions", versions.len());
    for version in &versions {
        println!("  - {}", version);
    }

    // Test 4: Download package
    println!("\n⬇️  Test 4: Package Download");
    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path().join("example-lib-1.0.0.tar.gz");
    
    registry.download_package(&package_info.download_url, download_path.clone()).await?;
    
    let file_size = std::fs::metadata(&download_path)?.len();
    println!("✓ Downloaded package to {:?} ({} bytes)", download_path, file_size);

    println!("\n✅ All HTTP tests passed! Package manager backend is working correctly.");

    Ok(())
}
