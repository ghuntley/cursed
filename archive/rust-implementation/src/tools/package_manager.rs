use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use reqwest;
use tokio;
use semver::Version;
use sha2::{Sha256, Digest};

/// Enhanced package manager for CURSED with dependency resolution and registry integration
#[derive(Debug, Clone)]
pub struct PackageManager {
    pub registry_url: String,
    pub local_cache: PathBuf,
    pub lock_file: PathBuf,
    pub config: PackageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub build_dependencies: HashMap<String, String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageLock {
    pub version: String,
    pub dependencies: HashMap<String, LockedDependency>,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedDependency {
    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryResponse {
    pub name: String,
    pub versions: Vec<PackageVersion>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub version: String,
    pub download_url: String,
    pub checksum: String,
    pub dependencies: HashMap<String, String>,
    pub published_at: String,
    pub yanked: bool,
}

impl PackageManager {
    /// Create new package manager instance
    pub fn new(registry_url: String) -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let cache_dir = home_dir.join(".cursed").join("cache");
        
        Self {
            registry_url,
            local_cache: cache_dir,
            lock_file: PathBuf::from("cursed.lock"),
            config: PackageConfig::default(),
        }
    }

    /// Initialize new package with default configuration
    pub fn init_package(&self, name: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = PackageConfig {
            name: name.to_string(),
            version: version.to_string(),
            description: None,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            authors: vec![],
            license: Some("MIT".to_string()),
            repository: None,
            keywords: vec![],
            categories: vec![],
            exclude: vec![],
            include: vec![],
        };

        let config_toml = toml::to_string_pretty(&config)?;
        fs::write("cursed.toml", config_toml)?;

        // Create basic directory structure
        fs::create_dir_all("src")?;
        fs::create_dir_all("tests")?;
        fs::create_dir_all("examples")?;

        // Create main.csd
        fs::write("src/main.csd", r#"yeet "vibez"

slay main() {
    vibez.spill("Hello, CURSED World!")
}
"#)?;

        // Create README.md
        fs::write("README.md", format!(r#"# {}

A CURSED package.

## Installation

```bash
cursed install {}
```

## Usage

```cursed
yeet "{}"
```

## License

MIT
"#, name, name, name))?;

        println!("✅ Initialized package '{}' v{}", name, version);
        Ok(())
    }

    /// Add dependency to package
    pub fn add_dependency(&mut self, name: &str, version: &str, dev: bool) -> Result<(), Box<dyn std::error::Error>> {
        // Load existing config
        let config_str = fs::read_to_string("cursed.toml")?;
        let mut config: PackageConfig = toml::from_str(&config_str)?;

        // Add to appropriate dependency section
        if dev {
            config.dev_dependencies.insert(name.to_string(), version.to_string());
        } else {
            config.dependencies.insert(name.to_string(), version.to_string());
        }

        // Write back to file
        let config_toml = toml::to_string_pretty(&config)?;
        fs::write("cursed.toml", config_toml)?;

        println!("✅ Added dependency '{}' v{}", name, version);
        Ok(())
    }

    /// Resolve dependencies and create lock file
    pub async fn resolve_dependencies(&self) -> Result<PackageLock, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("cursed.toml")?;
        let config: PackageConfig = toml::from_str(&config_str)?;

        let mut resolved_deps = HashMap::new();
        let mut to_resolve: Vec<(String, String)> = Vec::new();

        // Collect all dependencies
        for (name, version) in &config.dependencies {
            to_resolve.push((name.clone(), version.clone()));
        }

        // Resolve dependencies recursively
        while let Some((name, version)) = to_resolve.pop() {
            if resolved_deps.contains_key(&name) {
                continue;
            }

            let package_info = self.fetch_package_info(&name).await?;
            let resolved_version = self.resolve_version(&package_info, &version)?;

            let locked_dep = LockedDependency {
                version: resolved_version.version.clone(),
                resolved: resolved_version.download_url.clone(),
                integrity: resolved_version.checksum.clone(),
                dependencies: resolved_version.dependencies.clone(),
            };

            resolved_deps.insert(name, locked_dep);

            // Add transitive dependencies
            for (dep_name, dep_version) in &resolved_version.dependencies {
                to_resolve.push((dep_name.clone(), dep_version.clone()));
            }
        }

        let checksum = self.calculate_lock_checksum(&resolved_deps)?;
        let lock = PackageLock {
            version: "1.0.0".to_string(),
            dependencies: resolved_deps,
            checksum,
        };

        // Write lock file
        let lock_toml = toml::to_string_pretty(&lock)?;
        fs::write(&self.lock_file, lock_toml)?;

        Ok(lock)
    }

    /// Install dependencies from lock file
    pub async fn install_dependencies(&self) -> Result<(), Box<dyn std::error::Error>> {
        let lock_str = fs::read_to_string(&self.lock_file)?;
        let lock: PackageLock = toml::from_str(&lock_str)?;

        // Create cache directory
        fs::create_dir_all(&self.local_cache)?;

        for (name, dep) in &lock.dependencies {
            let cache_path = self.local_cache.join(format!("{}-{}", name, dep.version));
            
            if !cache_path.exists() {
                println!("📦 Installing {} v{}", name, dep.version);
                self.download_package(name, dep).await?;
            } else {
                println!("✅ {} v{} already cached", name, dep.version);
            }
        }

        println!("✅ All dependencies installed successfully");
        Ok(())
    }

    /// Fetch package information from registry
    async fn fetch_package_info(&self, name: &str) -> Result<RegistryResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/packages/{}", self.registry_url, name);
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            return Err(format!("Package '{}' not found in registry", name).into());
        }

        let package_info: RegistryResponse = response.json().await?;
        Ok(package_info)
    }

    /// Resolve version from package info
    fn resolve_version(&self, package_info: &RegistryResponse, version_req: &str) -> Result<PackageVersion, Box<dyn std::error::Error>> {
        // Simple version resolution - in production, use proper semver resolution
        for version in &package_info.versions {
            if version.version == version_req || version_req == "*" {
                if !version.yanked {
                    return Ok(version.clone());
                }
            }
        }

        // Find latest compatible version
        let mut compatible_versions: Vec<_> = package_info.versions
            .iter()
            .filter(|v| !v.yanked)
            .collect();

        compatible_versions.sort_by(|a, b| {
            Version::parse(&a.version)
                .unwrap_or_else(|_| Version::new(0, 0, 0))
                .cmp(&Version::parse(&b.version).unwrap_or_else(|_| Version::new(0, 0, 0)))
        });

        if let Some(version) = compatible_versions.last() {
            Ok((*version).clone())
        } else {
            Err(format!("No compatible version found for {}", version_req).into())
        }
    }

    /// Download package from registry
    async fn download_package(&self, name: &str, dep: &LockedDependency) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(&dep.resolved).await?;
        let bytes = response.bytes().await?;

        // Verify checksum
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = format!("{:x}", hasher.finalize());

        if hash != dep.integrity {
            return Err("Package integrity check failed".into());
        }

        // Extract package
        let cache_path = self.local_cache.join(format!("{}-{}", name, dep.version));
        fs::create_dir_all(&cache_path)?;

        // Simple extraction - in production, handle tar.gz, zip, etc.
        fs::write(cache_path.join("package.tar.gz"), &bytes)?;

        Ok(())
    }

    /// Calculate checksum for lock file
    fn calculate_lock_checksum(&self, deps: &HashMap<String, LockedDependency>) -> Result<String, Box<dyn std::error::Error>> {
        let mut hasher = Sha256::new();
        
        // Sort dependencies for consistent checksum
        let mut sorted_deps: Vec<_> = deps.iter().collect();
        sorted_deps.sort_by_key(|(name, _)| name.as_str());

        for (name, dep) in sorted_deps {
            hasher.update(name.as_bytes());
            hasher.update(dep.version.as_bytes());
            hasher.update(dep.integrity.as_bytes());
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Publish package to registry
    pub async fn publish_package(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("cursed.toml")?;
        let config: PackageConfig = toml::from_str(&config_str)?;

        // Create package archive
        let archive_path = self.create_package_archive(&config)?;

        // Upload to registry
        let client = reqwest::Client::new();
        let file_data = tokio::fs::read(&archive_path).await?;
        let body = reqwest::Body::from(file_data);

        let response = client
            .post(&format!("{}/packages", self.registry_url))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/octet-stream")
            .body(body)
            .send()
            .await?;

        if response.status().is_success() {
            println!("✅ Package '{}' v{} published successfully", config.name, config.version);
        } else {
            return Err(format!("Failed to publish package: {}", response.status()).into());
        }

        // Clean up
        fs::remove_file(archive_path)?;
        Ok(())
    }

    /// Create package archive for publishing
    fn create_package_archive(&self, config: &PackageConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let archive_path = PathBuf::from(format!("{}-{}.tar.gz", config.name, config.version));
        
        // In production, implement proper tar.gz creation
        // For now, create a placeholder
        fs::write(&archive_path, b"placeholder archive")?;
        
        Ok(archive_path)
    }

    /// Update dependencies to latest versions
    pub async fn update_dependencies(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Updating dependencies...");
        
        // Remove existing lock file
        if self.lock_file.exists() {
            fs::remove_file(&self.lock_file)?;
        }

        // Resolve dependencies again
        self.resolve_dependencies().await?;
        self.install_dependencies().await?;

        println!("✅ Dependencies updated successfully");
        Ok(())
    }

    /// Check for outdated dependencies
    pub async fn check_outdated(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("cursed.toml")?;
        let config: PackageConfig = toml::from_str(&config_str)?;

        println!("📊 Checking for outdated dependencies...");
        
        for (name, current_version) in &config.dependencies {
            let package_info = self.fetch_package_info(name).await?;
            
            if let Some(latest) = package_info.versions.iter()
                .filter(|v| !v.yanked)
                .max_by_key(|v| Version::parse(&v.version).unwrap_or_else(|_| Version::new(0, 0, 0))) {
                
                let current = Version::parse(current_version).unwrap_or_else(|_| Version::new(0, 0, 0));
                let latest_ver = Version::parse(&latest.version).unwrap_or_else(|_| Version::new(0, 0, 0));
                
                if latest_ver > current {
                    println!("⚠️  {} {} -> {} (latest)", name, current_version, latest.version);
                } else {
                    println!("✅ {} {} (up to date)", name, current_version);
                }
            }
        }

        Ok(())
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            name: "my-package".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            authors: vec![],
            license: Some("MIT".to_string()),
            repository: None,
            keywords: vec![],
            categories: vec![],
            exclude: vec![],
            include: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_package_config_serialization() {
        let config = PackageConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: PackageConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(config.name, deserialized.name);
    }

    #[test]
    fn test_package_manager_creation() {
        let pm = PackageManager::new("https://registry.cursed.dev".to_string());
        assert_eq!(pm.registry_url, "https://registry.cursed.dev");
    }

    #[ignore] // Skip due to tokio runtime stack overflow
#[tokio::test]
async fn test_dependency_resolution() {
        let pm = PackageManager::new("https://registry.cursed.dev".to_string());
        
        // Test would require mock registry
        // In production, use proper mocking framework
    }
}
