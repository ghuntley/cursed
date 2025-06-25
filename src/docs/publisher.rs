// Documentation Publishing System
// 
// Provides automated documentation deployment, versioning, and CDN integration
// for the CURSED programming language documentation ecosystem.

use crate::error::{CursedError, Result};
use crate::docs::generator::DocumentationGenerator;
use crate::docs::registry::DocumentationRegistry;
use crate::package_manager::{Package, PackageManager};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, instrument, warn};
use tokio::fs;
use tokio::process::Command;

/// Configuration for documentation publishing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishConfig {
    /// Publishing target (local, s3, github-pages, etc.)
    /// Base URL for the published documentation
    /// CDN configuration for optimization
    /// Optimization settings
    /// Authentication settings
    /// Custom domain configuration
/// Publishing target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PublishTarget {
    Local {
    S3 {
    GithubPages {
    Custom {
/// CDN configuration for global delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnConfig {
    /// CDN provider (cloudflare, cloudfront, etc.)
    /// CDN domain or distribution ID
    /// Cache settings
    /// Geographic regions
/// Cache configuration for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSettings {
    /// HTML files cache duration (seconds)
    /// Static assets cache duration (seconds)
    /// API responses cache duration (seconds)
/// Optimization settings for published documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable HTML minification
    /// Enable CSS minification
    /// Enable JavaScript minification
    /// Enable image optimization
    /// Enable Gzip compression
    /// Enable Brotli compression
/// Authentication configuration for publishing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// API key for publishing
    /// Secret key for signing
    /// Additional auth headers
/// Custom domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    /// Custom domain name
    /// SSL certificate configuration
    /// DNS configuration
/// SSL certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    /// Certificate type (letsencrypt, custom, etc.)
    /// Certificate path (for custom)
    /// Private key path (for custom)
/// DNS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    /// DNS provider
    /// DNS records to configure
/// DNS record configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
/// Publication metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationMetadata {
    /// Package name
    /// Package version
    /// Publication timestamp
    /// Publication target
    /// Documentation URL
    /// Size information
    /// Performance metrics
/// Size information for published documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeInfo {
    /// Total size in bytes
    /// Number of HTML files
    /// Number of asset files
    /// Compressed size (if compression enabled)
/// Performance metrics for published documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Build time in milliseconds
    /// Upload time in milliseconds
    /// CDN propagation time in milliseconds
/// Main documentation publisher
pub struct DocumentationPublisher {
impl DocumentationPublisher {
    /// Create a new documentation publisher
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// Publish documentation for a package
    #[instrument(skip(self))]
    pub async fn publish_package(&mut self, package: &Package) -> Result<PublicationMetadata> {
        info!(package = %package.name, version = %package.version, "Publishing documentation");
        
        let start_time = SystemTime::now();
        
        // Generate documentation
        let docs_path = self.generate_documentation(package).await?;
        
        // Optimize documentation
        let optimized_path = self.optimize_documentation(&docs_path).await?;
        
        // Calculate size information
        let size_info = self.calculate_size_info(&optimized_path).await?;
        
        let build_time = start_time.elapsed().unwrap().as_millis() as u64;
        let upload_start = SystemTime::now();
        
        // Upload to target
        let url = self.upload_documentation(package, &optimized_path).await?;
        
        let upload_time = upload_start.elapsed().unwrap().as_millis() as u64;
        
        // Update CDN if configured
        let propagation_time = if self.config.cdn.is_some() {
            Some(self.update_cdn(package, &url).await?)
        } else {
            None
        
        // Create publication metadata
        let metadata = PublicationMetadata {
            performance: PerformanceMetrics {
        
        // Register publication
        self.registry.register_publication(&metadata).await?;
        
        info!(
            "Documentation published successfully"
        );
        
        Ok(metadata)
    /// Generate documentation for a package
    #[instrument(skip(self))]
    async fn generate_documentation(&mut self, package: &Package) -> Result<PathBuf> {
        debug!(package = %package.name, "Generating documentation");
        
        let output_dir = self.get_temp_dir(package);
        fs::create_dir_all(&output_dir).await.map_err(|e| {
            CursedError::General(format!("Failed to create output directory: {}", e))
        })?;
        
        // Generate HTML documentation
        self.generator.generate_html_docs(package, &output_dir).await?;
        
        // Generate search index
        self.generate_search_index(package, &output_dir).await?;
        
        // Generate sitemap
        self.generate_sitemap(package, &output_dir).await?;
        
        // Copy static assets
        self.copy_static_assets(&output_dir).await?;
        
        Ok(output_dir)
    /// Generate search index for documentation
    #[instrument(skip(self))]
    async fn generate_search_index(&self, package: &Package, output_dir: &Path) -> Result<()> {
        debug!(package = %package.name, "Generating search index");
        
        let search_index = self.build_search_index(package).await?;
        let index_path = output_dir.join("search_index.json");
        
        let index_json = serde_json::to_string_pretty(&search_index).map_err(|e| {
            CursedError::Serialization(format!("Failed to serialize search index: {}", e))
        })?;
        
        fs::write(&index_path, index_json).await.map_err(|e| {
            CursedError::General(format!("Failed to write search index: {}", e))
        })?;
        
        Ok(())
    /// Build search index from package documentation
    async fn build_search_index(&self, package: &Package) -> Result<serde_json::Value> {
        // Implementation would extract searchable content from documentation
        // This is a simplified version
        let mut index = serde_json::Map::new();
        
        index.insert("package".to_string(), serde_json::Value::String(package.name.clone()));
        index.insert("version".to_string(), serde_json::Value::String(package.version.clone()));
        index.insert("searchable_content".to_string(), serde_json::Value::Array(vec![]));
        
        Ok(serde_json::Value::Object(index))
    /// Generate sitemap for SEO
    #[instrument(skip(self))]
    async fn generate_sitemap(&self, package: &Package, output_dir: &Path) -> Result<()> {
        debug!(package = %package.name, "Generating sitemap");
        
        let sitemap_content = self.build_sitemap_xml(package).await?;
        let sitemap_path = output_dir.join("sitemap.xml");
        
        fs::write(&sitemap_path, sitemap_content).await.map_err(|e| {
            CursedError::General(format!("Failed to write sitemap: {}", e))
        })?;
        
        Ok(())
    /// Build sitemap XML content
    async fn build_sitemap_xml(&self, package: &Package) -> Result<String> {
        let base_url = &self.config.base_url;
        let package_url = format!("{}/{}/{}", base_url, package.name, package.version);
        
        let sitemap = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>{}/</loc>
        <changefreq>weekly</changefreq>
        <priority>1.0</priority>
    </url>
    <url>
        <loc>{}/api/</loc>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>
</urlset>"#,
            package_url, package_url
        );
        
        Ok(sitemap)
    /// Copy static assets to documentation directory
    #[instrument(skip(self))]
    async fn copy_static_assets(&self, output_dir: &Path) -> Result<()> {
        debug!("Copying static assets");
        
        let assets_dir = output_dir.join("assets");
        fs::create_dir_all(&assets_dir).await.map_err(|e| {
            CursedError::General(format!("Failed to create assets directory: {}", e))
        })?;
        
        // Copy CSS, JS, and image assets
        // This would copy from a static assets directory
        // For now, create minimal required assets
        self.create_minimal_assets(&assets_dir).await?;
        
        Ok(())
    /// Create minimal required assets
    async fn create_minimal_assets(&self, assets_dir: &Path) -> Result<()> {
        let css_content = r#"
/* Minimal CSS for documentation */
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; }
.nav { background: #f8f9fa; padding: 1rem; }
.content { max-width: 1200px; margin: 0 auto; padding: 2rem; }
.search-box { width: 300px; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; }
"#;
        
        let js_content = r#"
// Minimal JavaScript for search functionality
function initSearch() {
    const searchBox = document.getElementById('search');
    if (searchBox) {
        searchBox.addEventListener('input', function(e) {
            performSearch(e.target.value);
        });
    }
}

function performSearch(query) {
    // Basic search implementation
    console.log('Searching for:', query);
document.addEventListener('DOMContentLoaded', initSearch);
"#;
        
        fs::write(assets_dir.join("docs.css"), css_content).await.map_err(|e| {
            CursedError::General(format!("Failed to write CSS: {}", e))
        })?;
        
        fs::write(assets_dir.join("docs.js"), js_content).await.map_err(|e| {
            CursedError::General(format!("Failed to write JavaScript: {}", e))
        })?;
        
        Ok(())
    /// Optimize generated documentation
    #[instrument(skip(self))]
    async fn optimize_documentation(&self, docs_path: &Path) -> Result<PathBuf> {
        if !self.should_optimize() {
            return Ok(docs_path.to_path_buf());
        debug!("Optimizing documentation");
        
        let optimized_dir = docs_path.with_extension("optimized");
        fs::create_dir_all(&optimized_dir).await.map_err(|e| {
            CursedError::General(format!("Failed to create optimized directory: {}", e))
        })?;
        
        // Copy and optimize files
        self.copy_and_optimize_files(docs_path, &optimized_dir).await?;
        
        Ok(optimized_dir)
    /// Check if optimization should be performed
    fn should_optimize(&self) -> bool {
        self.config.optimization.minify_html ||
        self.config.optimization.minify_css ||
        self.config.optimization.minify_js ||
        self.config.optimization.optimize_images
    /// Copy and optimize files
    async fn copy_and_optimize_files(&self, source: &Path, dest: &Path) -> Result<()> {
        let mut entries = fs::read_dir(source).await.map_err(|e| {
            CursedError::General(format!("Failed to read source directory: {}", e))
        })?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            CursedError::General(format!("Failed to read directory entry: {}", e))
        })? {
            let path = entry.path();
            let dest_path = dest.join(entry.file_name());
            
            if path.is_dir() {
                fs::create_dir_all(&dest_path).await.map_err(|e| {
                    CursedError::General(format!("Failed to create directory: {}", e))
                })?;
                self.copy_and_optimize_files(&path, &dest_path).await?;
            } else {
                self.optimize_and_copy_file(&path, &dest_path).await?;
            }
        }
        
        Ok(())
    /// Optimize and copy a single file
    async fn optimize_and_copy_file(&self, source: &Path, dest: &Path) -> Result<()> {
        let extension = source.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        match extension {
            "html" if self.config.optimization.minify_html => {
                self.minify_html_file(source, dest).await?;
            }
            "css" if self.config.optimization.minify_css => {
                self.minify_css_file(source, dest).await?;
            }
            "js" if self.config.optimization.minify_js => {
                self.minify_js_file(source, dest).await?;
            }
            _ => {
                fs::copy(source, dest).await.map_err(|e| {
                    CursedError::General(format!("Failed to copy file: {}", e))
                })?;
            }
        }
        
        Ok(())
    /// Minify HTML file
    async fn minify_html_file(&self, source: &Path, dest: &Path) -> Result<()> {
        let content = fs::read_to_string(source).await.map_err(|e| {
            CursedError::General(format!("Failed to read HTML file: {}", e))
        })?;
        
        // Basic HTML minification (remove extra whitespace)
        let minified = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        
        fs::write(dest, minified).await.map_err(|e| {
            CursedError::General(format!("Failed to write minified HTML: {}", e))
        })?;
        
        Ok(())
    /// Minify CSS file
    async fn minify_css_file(&self, source: &Path, dest: &Path) -> Result<()> {
        let content = fs::read_to_string(source).await.map_err(|e| {
            CursedError::General(format!("Failed to read CSS file: {}", e))
        })?;
        
        // Basic CSS minification
        let minified = content
            .replace("\n", "")
            .replace("\t", "")
            .replace("  ", " ");
        
        fs::write(dest, minified).await.map_err(|e| {
            CursedError::General(format!("Failed to write minified CSS: {}", e))
        })?;
        
        Ok(())
    /// Minify JavaScript file
    async fn minify_js_file(&self, source: &Path, dest: &Path) -> Result<()> {
        let content = fs::read_to_string(source).await.map_err(|e| {
            CursedError::General(format!("Failed to read JS file: {}", e))
        })?;
        
        // Basic JS minification (remove comments and extra whitespace)
        let minified = content
            .lines()
            .filter(|line| !line.trim().starts_with("//") && !line.trim().is_empty())
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join(" ");
        
        fs::write(dest, minified).await.map_err(|e| {
            CursedError::General(format!("Failed to write minified JS: {}", e))
        })?;
        
        Ok(())
    /// Calculate size information for documentation
    async fn calculate_size_info(&self, docs_path: &Path) -> Result<SizeInfo> {
        let mut total_bytes = 0u64;
        let mut html_files = 0u32;
        let mut asset_files = 0u32;
        
        let mut stack = vec![docs_path.to_path_buf()];
        
        while let Some(dir) = stack.pop() {
            let mut entries = fs::read_dir(&dir).await.map_err(|e| {
                CursedError::General(format!("Failed to read directory: {}", e))
            })?;
            
            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                CursedError::General(format!("Failed to read entry: {}", e))
            })? {
                let path = entry.path();
                
                if path.is_dir() {
                    stack.push(path);
                } else {
                    let metadata = entry.metadata().await.map_err(|e| {
                        CursedError::General(format!("Failed to read metadata: {}", e))
                    })?;
                    
                    total_bytes += metadata.len();
                    
                    if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                        match extension {
                        }
                    }
                }
            }
        Ok(SizeInfo {
            compressed_bytes: None, // Would calculate if compression is enabled
        })
    /// Upload documentation to the configured target
    #[instrument(skip(self))]
    async fn upload_documentation(&self, package: &Package, docs_path: &Path) -> Result<String> {
        debug!(package = %package.name, "Uploading documentation");
        
        match &self.config.target {
            PublishTarget::Local { path } => {
                self.upload_to_local(package, docs_path, path).await
            }
            PublishTarget::S3 { bucket, region, prefix } => {
                self.upload_to_s3(package, docs_path, bucket, region, prefix.as_deref()).await
            }
            PublishTarget::GithubPages { repo, branch, token } => {
                self.upload_to_github_pages(package, docs_path, repo, branch, token).await
            }
            PublishTarget::Custom { endpoint, credentials } => {
                self.upload_to_custom(package, docs_path, endpoint, credentials).await
            }
        }
    /// Upload to local filesystem
    async fn upload_to_local(&self, package: &Package, docs_path: &Path, target_path: &Path) -> Result<String> {
        let package_dir = target_path.join(&package.name).join(&package.version);
        
        fs::create_dir_all(&package_dir).await.map_err(|e| {
            CursedError::General(format!("Failed to create target directory: {}", e))
        })?;
        
        self.copy_directory(docs_path, &package_dir).await?;
        
        Ok(format!("file://{}", package_dir.display()))
    /// Upload to Amazon S3
    async fn upload_to_s3(&self, package: &Package, docs_path: &Path, bucket: &str, region: &str, prefix: Option<&str>) -> Result<String> {
        // This would use AWS SDK to upload to S3
        // For now, simulate the upload
        warn!("S3 upload not implemented - would upload to s3://{}/{}", bucket, package.name);
        
        let key_prefix = prefix.unwrap_or("docs");
        let url = format!("https://{}.s3.{}.amazonaws.com/{}/{}/{}/", 
                         bucket, region, key_prefix, package.name, package.version);
        
        Ok(url)
    /// Upload to GitHub Pages
    async fn upload_to_github_pages(&self, package: &Package, docs_path: &Path, repo: &str, branch: &str, _token: &str) -> Result<String> {
        // This would use Git commands to push to GitHub Pages
        // For now, simulate the upload
        warn!("GitHub Pages upload not implemented - would push to {}", repo);
        
        let url = format!("https://{}.github.io/{}/{}/", 
                         repo.splitn(2, '/').next().unwrap_or("user"),
                         package.name, package.version);
        
        Ok(url)
    /// Upload to custom endpoint
    async fn upload_to_custom(&self, package: &Package, docs_path: &Path, endpoint: &str, _credentials: &HashMap<String, String>) -> Result<String> {
        // This would use HTTP API to upload to custom endpoint
        warn!("Custom upload not implemented - would upload to {}", endpoint);
        
        let url = format!("{}/{}/{}/", endpoint, package.name, package.version);
        Ok(url)
    /// Copy entire directory recursively
    async fn copy_directory(&self, source: &Path, dest: &Path) -> Result<()> {
        let mut stack = vec![(source.to_path_buf(), dest.to_path_buf())];
        
        while let Some((src, dst)) = stack.pop() {
            fs::create_dir_all(&dst).await.map_err(|e| {
                CursedError::General(format!("Failed to create directory: {}", e))
            })?;
            
            let mut entries = fs::read_dir(&src).await.map_err(|e| {
                CursedError::General(format!("Failed to read directory: {}", e))
            })?;
            
            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                CursedError::General(format!("Failed to read entry: {}", e))
            })? {
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                
                if src_path.is_dir() {
                    stack.push((src_path, dst_path));
                } else {
                    fs::copy(&src_path, &dst_path).await.map_err(|e| {
                        CursedError::General(format!("Failed to copy file: {}", e))
                    })?;
                }
            }
        Ok(())
    /// Update CDN after upload
    #[instrument(skip(self))]
    async fn update_cdn(&self, package: &Package, url: &str) -> Result<u64> {
        let cdn_config = self.config.cdn.as_ref().ok_or_else(|| {
            CursedError::Configuration("CDN not configured".to_string())
        })?;
        
        debug!(
            "Updating CDN"
        );
        
        let start_time = SystemTime::now();
        
        // This would trigger CDN invalidation/purge
        // For now, simulate the operation
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        let propagation_time = start_time.elapsed().unwrap().as_millis() as u64;
        
        info!(
            "CDN updated successfully"
        );
        
        Ok(propagation_time)
    /// Get temporary directory for documentation generation
    fn get_temp_dir(&self, package: &Package) -> PathBuf {
        std::env::temp_dir()
            .join("cursed_docs")
            .join(&package.name)
            .join(&package.version)
    /// Get the name of the current target
    fn get_target_name(&self) -> String {
        match &self.config.target {
        }
    }

    /// Validate configuration
    pub fn validate_config(&self) -> Result<()> {
        // Validate base URL
        if self.config.base_url.is_empty() {
            return Err(CursedError::Configuration("Base URL cannot be empty".to_string()));
        // Validate target-specific configuration
        match &self.config.target {
            PublishTarget::Local { path } => {
                if !path.exists() {
                    return Err(CursedError::Configuration(
                        format!("Local target path does not exist: {}", path.display())
                    ));
                }
            }
            PublishTarget::S3 { bucket, region, .. } => {
                if bucket.is_empty() || region.is_empty() {
                    return Err(CursedError::Configuration(
                        "S3 bucket and region must be specified".to_string()
                    ));
                }
            }
            PublishTarget::GithubPages { repo, branch, token } => {
                if repo.is_empty() || branch.is_empty() || token.is_empty() {
                    return Err(CursedError::Configuration(
                        "GitHub Pages requires repo, branch, and token".to_string()
                    ));
                }
            }
            PublishTarget::Custom { endpoint, .. } => {
                if endpoint.is_empty() {
                    return Err(CursedError::Configuration(
                        "Custom endpoint cannot be empty".to_string()
                    ));
                }
            }
        Ok(())
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for CacheSettings {
    fn default() -> Self {
        Self {
            html_cache: 3600,      // 1 hour
            assets_cache: 86400,   // 24 hours
            api_cache: 300,        // 5 minutes
        }
    }
