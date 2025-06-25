use crate::error::CursedError;
/// Plugin distribution and package management functionality
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
// use crate::stdlib::plug_vibes::error::{PluginError, PluginResult};
// use crate::stdlib::plug_vibes::plug::PlugInfo;
// use crate::stdlib::plug_vibes::version::Version;
// use crate::stdlib::plug_vibes::security::AuthInfo;

/// Distribution configuration
#[derive(Debug, Clone)]
pub struct DistributionConfig {
impl Default for DistributionConfig {
    fn default() -> Self {
        Self {
            max_download_size: 100 * 1024 * 1024, // 100MB
        }
    }
/// Plugin package for distribution
pub struct PluginPackage {
/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMetadata {
/// File in the package
#[derive(Debug, Clone)]
pub struct PackageFile {
/// Installation scripts
#[derive(Debug, Clone, Default)]
pub struct InstallScripts {
/// Package repository configuration
#[derive(Debug, Clone)]
pub struct Repository {
impl Repository {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
        }
    }

    pub fn with_auth(mut self, auth: AuthInfo) -> Self {
        self.auth = Some(auth);
        self
    pub fn trusted(mut self) -> Self {
        self.trusted = true;
        self
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Package manager for plugin distribution
pub struct PackageManager {
impl PackageManager {
    /// Create a new package manager
    pub fn new(cache_dir: PathBuf, install_dir: PathBuf) -> Self {
        Self {
        }
    }

    /// Add a repository
    pub fn add_repository(&mut self, repository: Repository) -> PluginResult<()> {
        // Check if repository already exists
        if self.repositories.iter().any(|r| r.name == repository.name) {
            return Err(PluginError::distribution_error(&format!(
                "Repository {} already exists", repository.name
            )));
        self.repositories.push(repository);
        Ok(())
    /// Remove a repository
    pub fn remove_repository(&mut self, name: &str) -> PluginResult<()> {
        let initial_len = self.repositories.len();
        self.repositories.retain(|r| r.name != name);
        
        if self.repositories.len() == initial_len {
            return Err(PluginError::distribution_error(&format!(
                "Repository {} not found", name
            )));
        Ok(())
    /// List all repositories
    pub fn list_repositories(&self) -> &[Repository] {
        &self.repositories
    /// Enable signature verification
    pub fn set_signature_verification(&mut self, verify: bool) {
        self.verify_signatures = verify;
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
    if !plugin_path.is_dir() {
        return Err(PluginError::distribution_error(&format!(
            "{} is not a directory", plugin_dir
        )));
    // Read package metadata
    let metadata_path = plugin_path.join("package.json");
    let metadata = if metadata_path.exists() {
        read_package_metadata(&metadata_path)?
    } else {
        // Generate basic metadata from plugin info
        generate_metadata_from_plugin(plugin_path)?

    // Collect all files
    let files = collect_package_files(plugin_path)?;

    // Create package
    let package = PluginPackage {

    // Write package to output
    write_package(&package, output_path)?;

    Ok(())
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
    Ok(())
/// Verify package integrity and signatures
pub fn verify_package(package_path: &str) -> PluginResult<bool> {
    let package = read_package(package_path)?;
    
    // Verify checksums
    if let Err(_) = verify_package_integrity(&package) {
        return Ok(false);
    // Verify signature if present
    if package.signature.is_some() {
        // In a real implementation, verify the signature
        // For now, assume signature is valid
        return Ok(true);
    // Package is valid if checksums match (no signature required)
    Ok(true)
/// List remote plugins from a repository
pub fn list_remote_plugins(repo_url: &str) -> PluginResult<Vec<PlugInfo>> {
    // In a real implementation, this would make HTTP requests to the repository
    // and parse the response to get available plugins
    
    // For now, return an empty list
    Ok(Vec::new())
/// Download a plugin from a repository
pub fn download_plugin(
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
/// Publish a plugin package to a repository
pub fn publish_plugin(
) -> PluginResult<()> {
    // Verify package before publishing
    if !verify_package(package_path)? {
        return Err(PluginError::distribution_error("Package verification failed"));
    // In a real implementation, this would:
    // 1. Read the package file
    // 2. Authenticate with the repository
    // 3. Upload the package via HTTP
    // 4. Handle any publication errors
    
    // For now, just validate that the files exist
    if !Path::new(package_path).exists() {
        return Err(PluginError::plugin_not_found(package_path));
    if auth.username.is_empty() {
        return Err(PluginError::distribution_error("Authentication required for publishing"));
    Ok(())
// Helper functions

fn read_package_metadata(metadata_path: &Path) -> PluginResult<PackageMetadata> {
    let content = fs::read_to_string(metadata_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to read metadata: {}", e))
    })?;

    // In a real implementation, parse JSON metadata
    // For now, create dummy metadata
    Ok(PackageMetadata {
    })
fn generate_metadata_from_plugin(plugin_path: &Path) -> PluginResult<PackageMetadata> {
    // Try to extract metadata from plugin binary
    // For now, create basic metadata
    let name = plugin_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(PackageMetadata {
    })
fn collect_package_files(plugin_path: &Path) -> PluginResult<Vec<(PathBuf, Vec<u8>)>> {
    let mut files = Vec::new();
    
    fn collect_files_recursive(
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
    collect_files_recursive(plugin_path, plugin_path, &mut files).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to collect files: {}", e))
    })?;

    Ok(files)
fn create_package_archive(files: &[(PathBuf, Vec<u8>)]) -> PluginResult<Vec<u8>> {
    // In a real implementation, create a proper archive (tar, zip, etc.)
    // For now, just concatenate all file contents
    let mut archive = Vec::new();
    
    for (path, content) in files {
        archive.extend_from_slice(path.to_string_lossy().as_bytes());
        archive.push(0); // null terminator
        archive.extend_from_slice(&(content.len() as u64).to_le_bytes());
        archive.extend_from_slice(content);
    Ok(archive)
fn calculate_checksums(files: &[(PathBuf, Vec<u8>)]) -> PluginResult<HashMap<String, String>> {
    let mut checksums = HashMap::new();
    
    for (path, content) in files {
        // In a real implementation, use SHA256 or another secure hash
        // For now, use a simple checksum
        let checksum = format!("{:x}", content.len() ^ content.iter().map(|&b| b as usize).sum::<usize>());
        checksums.insert(path.to_string_lossy().to_string(), checksum);
    Ok(checksums)
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
fn read_package(package_path: &str) -> PluginResult<PluginPackage> {
    let content = fs::read(package_path).map_err(|e| {
        PluginError::distribution_error(&format!("Failed to read package: {}", e))
    })?;

    // In a real implementation, parse the package format
    // For now, create a dummy package
    Ok(PluginPackage {
        metadata: PackageMetadata {
    })
fn verify_package_integrity(_package: &PluginPackage) -> PluginResult<()> {
    // In a real implementation, verify checksums and signatures
    // For now, always pass
    Ok(())
fn extract_package_content(_package: &PluginPackage, _output_path: &Path) -> PluginResult<()> {
    // In a real implementation, extract the archive to the output directory
    // For now, just create a dummy file
    let dummy_file = _output_path.join("plugin.so");
    fs::write(&dummy_file, b"dummy plugin content").map_err(|e| {
        PluginError::distribution_error(&format!("Failed to extract package: {}", e))
    })?;

    Ok(())
fn run_install_script(_script: &str, _install_path: &Path) -> PluginResult<()> {
    // In a real implementation, execute the install script
    // For now, just validate the script exists
    if _script.is_empty() {
        return Err(PluginError::distribution_error("Empty install script"));
    Ok(())
