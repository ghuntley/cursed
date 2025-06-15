/// Plugin distribution and package management functionality
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
use crate::stdlib::plug_vibes::plug::PlugInfo;
use crate::stdlib::plug_vibes::version::Version;
use crate::stdlib::plug_vibes::security::AuthInfo;

/// Plugin package for distribution
pub struct PluginPackage {
    pub metadata: PackageMetadata,
    pub content: Vec<u8>,
    pub signature: Option<Vec<u8>>,
    pub checksums: HashMap<String, String>,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub author: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub dependencies: HashMap<String, String>,
    pub files: Vec<PackageFile>,
    pub install_scripts: InstallScripts,
    pub target_platforms: Vec<String>,
    pub min_host_version: Option<Version>,
    pub max_host_version: Option<Version>,
}

/// File in the package
#[derive(Debug, Clone)]
pub struct PackageFile {
    pub path: String,
    pub size: u64,
    pub checksum: String,
    pub executable: bool,
    pub install_path: Option<String>,
}

/// Installation scripts
#[derive(Debug, Clone, Default)]
pub struct InstallScripts {
    pub pre_install: Option<String>,
    pub post_install: Option<String>,
    pub pre_uninstall: Option<String>,
    pub post_uninstall: Option<String>,
}

/// Package repository configuration
#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub auth: Option<AuthInfo>,
    pub trusted: bool,
    pub enabled: bool,
}

impl Repository {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            auth: None,
            trusted: false,
            enabled: true,
        }
    }

    pub fn with_auth(mut self, auth: AuthInfo) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn trusted(mut self) -> Self {
        self.trusted = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Package manager for plugin distribution
pub struct PackageManager {
    repositories: Vec<Repository>,
    cache_dir: PathBuf,
    install_dir: PathBuf,
    verify_signatures: bool,
}

impl PackageManager {
    /// Create a new package manager
    pub fn new(cache_dir: PathBuf, install_dir: PathBuf) -> Self {
        Self {
            repositories: Vec::new(),
            cache_dir,
            install_dir,
            verify_signatures: true,
        }
    }

    /// Add a repository
    pub fn add_repository(&mut self, repository: Repository) -> PluginResult<()> {
        // Check if repository already exists
        if self.repositories.iter().any(|r| r.name == repository.name) {
            return Err(PluginError::distribution_error(&format!(
                "Repository {} already exists", repository.name
            )));
        }

        self.repositories.push(repository);
        Ok(())
    }

    /// Remove a repository
    pub fn remove_repository(&mut self, name: &str) -> PluginResult<()> {
        let initial_len = self.repositories.len();
        self.repositories.retain(|r| r.name != name);
        
        if self.repositories.len() == initial_len {
            return Err(PluginError::distribution_error(&format!(
                "Repository {} not found", name
            )));
        }
        
        Ok(())
    }

    /// List all repositories
    pub fn list_repositories(&self) -> &[Repository] {
        &self.repositories
    }

    /// Enable signature verification
    pub fn set_signature_verification(&mut self, verify: bool) {
        self.verify_signatures = verify;
    }

    /// Create cache and install directories if they don't exist
    pub fn ensure_directories(&self) -> PluginResult<()> {
        fs::create_dir_all(&self.cache_dir).map_err(|e| {
            PluginError::distribution_error(&format!("Failed to create cache directory: {}", e))
        })?;

        fs::create_dir_all(&self.install_dir).map_err(|e| {
            PluginError::distribution_error(&format!("Failed to create install directory: {}", e))
        })?;

        Ok(())
    }
}

/// Pack a plugin directory into a distributable package
pub fn pack_plugin(plugin_dir: &str, output_path: &str) -> PluginResult<()> {
    let plugin_path = Path::new(plugin_dir);
    
    if !plugin_path.exists() {
        return Err(PluginError::plugin_not_found(plugin_dir));
    }

    if !plugin_path.is_dir() {
        return Err(PluginError::distribution_error(&format!(
            "{} is not a directory", plugin_dir
        )));
    }

    // Read package metadata
    let metadata_path = plugin_path.join("package.json");
    let metadata = if metadata_path.exists() {
        read_package_metadata(&metadata_path)?
    } else {
        // Generate basic metadata from plugin info
        generate_metadata_from_plugin(plugin_path)?
    };

    // Collect all files
    let files = collect_package_files(plugin_path)?;

    // Create package
    let package = PluginPackage {
        metadata,
        content: create_package_archive(&files)?,
        signature: None,
        checksums: calculate_checksums(&files)?,
    };

    // Write package to output
    write_package(&package, output_path)?;

    Ok(())
}

/// Unpack a plugin package to a directory
pub fn unpack_plugin(package_path: &str, output_dir: &str) -> PluginResult<()> {
    let package = read_package(package_path)?;
    
    // Verify package integrity
    verify_package_integrity(&package)?;

    // Create output directory
    let output_path = Path::new(output_dir);
    fs::create_dir_all(output_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to create output directory: {}", e))
    })?;

    // Extract package content
    extract_package_content(&package, output_path)?;

    // Run post-install scripts if any
    if let Some(ref post_install) = package.metadata.install_scripts.post_install {
        run_install_script(post_install, output_path)?;
    }

    Ok(())
}

/// Verify package integrity and signatures
pub fn verify_package(package_path: &str) -> PluginResult<bool> {
    let package = read_package(package_path)?;
    
    // Verify checksums
    if let Err(_) = verify_package_integrity(&package) {
        return Ok(false);
    }

    // Verify signature if present
    if package.signature.is_some() {
        // In a real implementation, verify the signature
        // For now, assume signature is valid
        return Ok(true);
    }

    // Package is valid if checksums match (no signature required)
    Ok(true)
}

/// List remote plugins from a repository
pub fn list_remote_plugins(repo_url: &str) -> PluginResult<Vec<PlugInfo>> {
    // In a real implementation, this would make HTTP requests to the repository
    // and parse the response to get available plugins
    
    // For now, return an empty list
    Ok(Vec::new())
}

/// Download a plugin from a repository
pub fn download_plugin(
    repo_url: &str,
    plugin_name: &str,
    version: &Version,
) -> PluginResult<String> {
    // In a real implementation, this would:
    // 1. Construct the download URL
    // 2. Make HTTP request to download the plugin package
    // 3. Save to cache directory
    // 4. Return the local path
    
    // For now, simulate download
    let cache_path = format!("/tmp/{}-{}.plug", plugin_name, version);
    
    // Simulate downloading by creating an empty file
    fs::File::create(&cache_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to create cache file: {}", e))
    })?;

    Ok(cache_path)
}

/// Publish a plugin package to a repository
pub fn publish_plugin(
    repo_url: &str,
    package_path: &str,
    auth: &AuthInfo,
) -> PluginResult<()> {
    // Verify package before publishing
    if !verify_package(package_path)? {
        return Err(PluginError::distribution_error("Package verification failed"));
    }

    // In a real implementation, this would:
    // 1. Read the package file
    // 2. Authenticate with the repository
    // 3. Upload the package via HTTP
    // 4. Handle any publication errors
    
    // For now, just validate that the files exist
    if !Path::new(package_path).exists() {
        return Err(PluginError::plugin_not_found(package_path));
    }

    if auth.username.is_empty() {
        return Err(PluginError::distribution_error("Authentication required for publishing"));
    }

    Ok(())
}

// Helper functions

fn read_package_metadata(metadata_path: &Path) -> PluginResult<PackageMetadata> {
    let content = fs::read_to_string(metadata_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to read metadata: {}", e))
    })?;

    // In a real implementation, parse JSON metadata
    // For now, create dummy metadata
    Ok(PackageMetadata {
        name: "unknown".to_string(),
        version: Version::new(1, 0, 0),
        description: "Plugin package".to_string(),
        author: "Unknown".to_string(),
        license: "MIT".to_string(),
        homepage: None,
        repository: None,
        keywords: Vec::new(),
        categories: Vec::new(),
        dependencies: HashMap::new(),
        files: Vec::new(),
        install_scripts: InstallScripts::default(),
        target_platforms: vec!["linux".to_string(), "windows".to_string(), "macos".to_string()],
        min_host_version: None,
        max_host_version: None,
    })
}

fn generate_metadata_from_plugin(plugin_path: &Path) -> PluginResult<PackageMetadata> {
    // Try to extract metadata from plugin binary
    // For now, create basic metadata
    let name = plugin_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(PackageMetadata {
        name,
        version: Version::new(1, 0, 0),
        description: "Auto-generated package metadata".to_string(),
        author: "Unknown".to_string(),
        license: "Unknown".to_string(),
        homepage: None,
        repository: None,
        keywords: Vec::new(),
        categories: Vec::new(),
        dependencies: HashMap::new(),
        files: Vec::new(),
        install_scripts: InstallScripts::default(),
        target_platforms: vec!["linux".to_string()],
        min_host_version: None,
        max_host_version: None,
    })
}

fn collect_package_files(plugin_path: &Path) -> PluginResult<Vec<(PathBuf, Vec<u8>)>> {
    let mut files = Vec::new();
    
    fn collect_files_recursive(
        dir: &Path, 
        base: &Path, 
        files: &mut Vec<(PathBuf, Vec<u8>)>
    ) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let relative_path = path.strip_prefix(base).unwrap().to_path_buf();
                let content = fs::read(&path)?;
                files.push((relative_path, content));
            } else if path.is_dir() {
                collect_files_recursive(&path, base, files)?;
            }
        }
        Ok(())
    }

    collect_files_recursive(plugin_path, plugin_path, &mut files).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to collect files: {}", e))
    })?;

    Ok(files)
}

fn create_package_archive(files: &[(PathBuf, Vec<u8>)]) -> PluginResult<Vec<u8>> {
    // In a real implementation, create a proper archive (tar, zip, etc.)
    // For now, just concatenate all file contents
    let mut archive = Vec::new();
    
    for (path, content) in files {
        archive.extend_from_slice(path.to_string_lossy().as_bytes());
        archive.push(0); // null terminator
        archive.extend_from_slice(&(content.len() as u64).to_le_bytes());
        archive.extend_from_slice(content);
    }

    Ok(archive)
}

fn calculate_checksums(files: &[(PathBuf, Vec<u8>)]) -> PluginResult<HashMap<String, String>> {
    let mut checksums = HashMap::new();
    
    for (path, content) in files {
        // In a real implementation, use SHA256 or another secure hash
        // For now, use a simple checksum
        let checksum = format!("{:x}", content.len() ^ content.iter().map(|&b| b as usize).sum::<usize>());
        checksums.insert(path.to_string_lossy().to_string(), checksum);
    }

    Ok(checksums)
}

fn write_package(package: &PluginPackage, output_path: &str) -> PluginResult<()> {
    let mut file = fs::File::create(output_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to create package file: {}", e))
    })?;

    // In a real implementation, use a proper package format
    // For now, just write the content
    file.write_all(&package.content).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to write package content: {}", e))
    })?;

    Ok(())
}

fn read_package(package_path: &str) -> PluginResult<PluginPackage> {
    let content = fs::read(package_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to read package: {}", e))
    })?;

    // In a real implementation, parse the package format
    // For now, create a dummy package
    Ok(PluginPackage {
        metadata: PackageMetadata {
            name: "test".to_string(),
            version: Version::new(1, 0, 0),
            description: "Test package".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            homepage: None,
            repository: None,
            keywords: Vec::new(),
            categories: Vec::new(),
            dependencies: HashMap::new(),
            files: Vec::new(),
            install_scripts: InstallScripts::default(),
            target_platforms: Vec::new(),
            min_host_version: None,
            max_host_version: None,
        },
        content,
        signature: None,
        checksums: HashMap::new(),
    })
}

fn verify_package_integrity(_package: &PluginPackage) -> PluginResult<()> {
    // In a real implementation, verify checksums and signatures
    // For now, always pass
    Ok(())
}

fn extract_package_content(_package: &PluginPackage, _output_path: &Path) -> PluginResult<()> {
    // In a real implementation, extract the archive to the output directory
    // For now, just create a dummy file
    let dummy_file = _output_path.join("plugin.so");
    fs::write(&dummy_file, b"dummy plugin content").map_err(|e| {
        PluginError::distribution_error(&format!("Failed to extract package: {}", e))
    })?;

    Ok(())
}

fn run_install_script(_script: &str, _install_path: &Path) -> PluginResult<()> {
    // In a real implementation, execute the install script
    // For now, just validate the script exists
    if _script.is_empty() {
        return Err(PluginError::distribution_error("Empty install script"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new("test", "https://example.com/plugins")
            .trusted()
            .with_auth(AuthInfo::new("user").with_password("pass"));

        assert_eq!(repo.name, "test");
        assert_eq!(repo.url, "https://example.com/plugins");
        assert!(repo.trusted);
        assert!(repo.enabled);
        assert!(repo.auth.is_some());
    }

    #[test]
    fn test_package_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let install_dir = temp_dir.path().join("install");

        let manager = PackageManager::new(cache_dir, install_dir);
        assert_eq!(manager.repositories.len(), 0);
        assert!(manager.verify_signatures);
    }

    #[test]
    fn test_package_manager_repositories() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let install_dir = temp_dir.path().join("install");

        let mut manager = PackageManager::new(cache_dir, install_dir);
        
        let repo = Repository::new("test", "https://example.com");
        manager.add_repository(repo).unwrap();
        assert_eq!(manager.list_repositories().len(), 1);

        // Try to add duplicate
        let repo2 = Repository::new("test", "https://other.com");
        let result = manager.add_repository(repo2);
        assert!(result.is_err());

        // Remove repository
        manager.remove_repository("test").unwrap();
        assert_eq!(manager.list_repositories().len(), 0);

        // Try to remove non-existent
        let result = manager.remove_repository("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_package_metadata() {
        let metadata = PackageMetadata {
            name: "test-plugin".to_string(),
            version: Version::new(1, 2, 3),
            description: "A test plugin".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            homepage: Some("https://example.com".to_string()),
            repository: Some("https://github.com/user/repo".to_string()),
            keywords: vec!["test".to_string(), "plugin".to_string()],
            categories: vec!["utility".to_string()],
            dependencies: HashMap::new(),
            files: Vec::new(),
            install_scripts: InstallScripts::default(),
            target_platforms: vec!["linux".to_string()],
            min_host_version: Some(Version::new(1, 0, 0)),
            max_host_version: None,
        };

        assert_eq!(metadata.name, "test-plugin");
        assert_eq!(metadata.version, Version::new(1, 2, 3));
        assert_eq!(metadata.keywords.len(), 2);
    }

    #[test]
    fn test_install_scripts() {
        let scripts = InstallScripts {
            pre_install: Some("echo 'Pre-install'".to_string()),
            post_install: Some("echo 'Post-install'".to_string()),
            pre_uninstall: Some("echo 'Pre-uninstall'".to_string()),
            post_uninstall: Some("echo 'Post-uninstall'".to_string()),
        };

        assert!(scripts.pre_install.is_some());
        assert!(scripts.post_install.is_some());
        assert!(scripts.pre_uninstall.is_some());
        assert!(scripts.post_uninstall.is_some());
    }

    #[test]
    fn test_pack_plugin_nonexistent_directory() {
        let result = pack_plugin("/nonexistent/directory", "/tmp/output.plug");
        assert!(result.is_err());
        match result.unwrap_err() {
            PluginError::PluginNotFound(path) => assert_eq!(path, "/nonexistent/directory"),
            _ => panic!("Expected PluginNotFound error"),
        }
    }

    #[test]
    fn test_verify_package_nonexistent() {
        let result = verify_package("/nonexistent/package.plug");
        assert!(result.is_err());
    }

    #[test]
    fn test_download_plugin() {
        let result = download_plugin(
            "https://example.com/plugins",
            "test-plugin",
            &Version::new(1, 0, 0),
        );
        
        // Should succeed and return a cache path
        assert!(result.is_ok());
        let cache_path = result.unwrap();
        assert!(cache_path.contains("test-plugin-1.0.0"));
        
        // Clean up
        let _ = fs::remove_file(&cache_path);
    }

    #[test]
    fn test_publish_plugin() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test.plug");
        
        // Create a dummy package file
        fs::write(&package_path, b"dummy package").unwrap();
        
        let auth = AuthInfo::new("testuser").with_password("testpass");
        let result = publish_plugin(
            "https://example.com/plugins",
            package_path.to_str().unwrap(),
            &auth,
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_publish_plugin_invalid_auth() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test.plug");
        
        // Create a dummy package file
        fs::write(&package_path, b"dummy package").unwrap();
        
        let auth = AuthInfo::new(""); // Empty username
        let result = publish_plugin(
            "https://example.com/plugins",
            package_path.to_str().unwrap(),
            &auth,
        );
        
        assert!(result.is_err());
    }
}
