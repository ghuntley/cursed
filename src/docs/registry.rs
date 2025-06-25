// Documentation Registry
// 
// Central registry for all published CURSED documentation with package metadata,
// version tracking, and cross-reference resolution.

use crate::error::{CursedError, Result};
use crate::docs::publisher::PublicationMetadata;
use crate::package_manager::{Package, PackageManager};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Registry data directory
    /// Registry index file
    /// Package metadata cache size
    /// Metadata refresh interval (seconds)
    /// Enable automatic dependency resolution
    /// Maximum dependency depth
/// Package documentation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDocumentation {
    /// Package name
    /// Available versions
    /// Latest version
    /// Default version (may differ from latest)
    /// Package description
    /// Package authors
    /// Package license
    /// Repository URL
    /// Homepage URL
    /// Documentation URL
    /// Tags/keywords
    /// Package categories
    /// Documentation quality metrics
    /// Last updated timestamp
/// Version-specific documentation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDocumentation {
    /// Version string
    /// Publication metadata
    /// Module structure
    /// Public API items
    /// Dependencies
    /// Examples
    /// Changelog
    /// Migration guide
    /// Coverage metrics
    /// Cross-references
/// Module documentation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDocumentation {
    /// Module name
    /// Module path
    /// Module description
    /// Public functions
    /// Public structs/types
    /// Public constants
    /// Submodules
    /// Documentation completeness
/// API item documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiItem {
    /// Item name
    /// Item type (function, struct, enum, etc.)
    /// Module path
    /// Item signature
    /// Documentation summary
    /// Full documentation
    /// Examples
    /// Deprecated flag
    /// Stability level
    /// Source location
/// Example documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDocumentation {
    /// Example name
    /// Example description
    /// Source code
    /// Output (if applicable)
    /// Dependencies needed
    /// Category
    /// Difficulty level
/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Package name
    /// Version requirement
    /// Optional dependency
    /// Default features
    /// Enabled features
/// Cross-reference to other documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Reference type
    /// Target package
    /// Target version
    /// Target item
    /// Reference description
/// Documentation quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Overall documentation score (0-100)
    /// Documentation completeness (0-100)
    /// Example coverage (0-100)
    /// API documentation coverage (0-100)
    /// Link health (0-100)
    /// Freshness score (0-100)
/// Coverage metrics for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    /// Total public items
    /// Documented public items
    /// Items with examples
    /// Coverage percentage
    /// Missing documentation items
/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// File path
    /// Line number
    /// Column number
/// Stability level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StabilityLevel {
/// Difficulty level for examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
/// Registry search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearchQuery {
    /// Search terms
    /// Package filter
    /// Version filter
    /// Item type filter
    /// Category filter
    /// Minimum quality score
    /// Sort order
    /// Result limit
    /// Result offset
/// Sort order for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearchResult {
    /// Matched item
    /// Relevance score
    /// Package information
    /// Version information
    /// Match context
/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total packages
    /// Total versions
    /// Total API items
    /// Total examples
    /// Average quality score
    /// Top packages by quality
    /// Recent updates
    /// Popular packages
/// Documentation registry
pub struct DocumentationRegistry {
impl DocumentationRegistry {
    /// Create a new documentation registry
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            statistics: RwLock::new(RegistryStatistics {
        }
    }

    /// Initialize the registry
    #[instrument(skip(self))]
    pub async fn initialize(&self) -> Result<()> {
        info!(data_dir = %self.config.data_dir.display(), "Initializing documentation registry");

        // Create data directory if it doesn't exist
        fs::create_dir_all(&self.config.data_dir).await.map_err(|e| {
            CursedError::General(format!("Failed to create data directory: {}", e))
        })?;

        // Load existing registry data
        self.load_registry_data().await?;

        // Build search index
        self.rebuild_search_index().await?;

        // Update statistics
        self.update_statistics().await?;

        info!("Documentation registry initialized");
        Ok(())
    /// Register a new package publication
    #[instrument(skip(self))]
    pub async fn register_publication(&self, metadata: &PublicationMetadata) -> Result<()> {
        info!(
            "Registering package publication"
        );

        let mut packages = self.packages.write().await;
        
        let package_name = &metadata.package_name;
        let version = &metadata.version;

        // Get or create package documentation
        let package_doc = packages.entry(package_name.clone()).or_insert_with(|| {
            PackageDocumentation {
                quality_metrics: QualityMetrics {
            }
        });

        // Create version documentation
        let version_doc = VersionDocumentation {
            coverage: CoverageMetrics {

        package_doc.versions.insert(version.clone(), version_doc);

        // Update latest version if this is newer
        if self.is_newer_version(version, &package_doc.latest_version) {
            package_doc.latest_version = version.clone();
        package_doc.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Save registry data
        self.save_registry_data().await?;

        // Update search index
        self.update_search_index_for_package(package_name).await?;

        // Update statistics
        self.update_statistics().await?;

        info!(
            "Package publication registered"
        );

        Ok(())
    /// Get package documentation
    pub async fn get_package(&self, name: &str) -> Option<PackageDocumentation> {
        let packages = self.packages.read().await;
        packages.get(name).cloned()
    /// Get version documentation
    pub async fn get_version(&self, package: &str, version: &str) -> Option<VersionDocumentation> {
        let packages = self.packages.read().await;
        packages.get(package)?.versions.get(version).cloned()
    /// List all packages
    pub async fn list_packages(&self) -> Vec<String> {
        let packages = self.packages.read().await;
        packages.keys().cloned().collect()
    /// Get package versions
    pub async fn get_versions(&self, package: &str) -> Vec<String> {
        let packages = self.packages.read().await;
        if let Some(pkg) = packages.get(package) {
            pkg.versions.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Search registry
    #[instrument(skip(self))]
    pub async fn search(&self, query: &RegistrySearchQuery) -> Result<Vec<RegistrySearchResult>> {
        debug!(query = %query.query, "Searching registry");

        let search_index = self.search_index.read().await;
        let mut results = Vec::new();

        // Simple search implementation
        for (key, search_results) in search_index.iter() {
            if key.to_lowercase().contains(&query.query.to_lowercase()) {
                for result in search_results {
                    // Apply filters
                    if let Some(package_filter) = &query.package {
                        if result.package != *package_filter {
                            continue;
                        }
                    }

                    if let Some(version_filter) = &query.version {
                        if result.version != *version_filter {
                            continue;
                        }
                    }

                    if let Some(type_filter) = &query.item_type {
                        if result.item.item_type != *type_filter {
                            continue;
                        }
                    }

                    results.push(result.clone());
                }
            }
        // Sort results
        match query.sort_by {
            SortOrder::LastUpdated => {}, // Would sort by actual update time
            SortOrder::Popularity => {}, // Would sort by popularity metrics
        // Apply pagination
        let total_results = results.len();
        let start = query.offset.min(total_results);
        let end = (query.offset + query.limit).min(total_results);
        
        Ok(results[start..end].to_vec())
    /// Update package metadata
    #[instrument(skip(self))]
    pub async fn update_package_metadata(&self, package: &Package) -> Result<()> {
        debug!(package = %package.name, "Updating package metadata");

        let mut packages = self.packages.write().await;
        
        if let Some(package_doc) = packages.get_mut(&package.name) {
            package_doc.description = package.description.clone();
            package_doc.authors = package.authors.clone();
            package_doc.license = package.license.clone();
            package_doc.repository = package.repository.clone();
            package_doc.homepage = package.homepage.clone();
            package_doc.tags = package.keywords.clone();
            package_doc.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Ok(())
    /// Resolve cross-references
    #[instrument(skip(self))]
    pub async fn resolve_cross_references(&self, package: &str, version: &str) -> Result<Vec<CrossReference>> {
        debug!(package = %package, version = %version, "Resolving cross-references");

        let packages = self.packages.read().await;
        let mut cross_refs = Vec::new();

        if let Some(pkg) = packages.get(package) {
            if let Some(ver) = pkg.versions.get(version) {
                // Resolve dependency references
                for dep in &ver.dependencies {
                    if let Some(dep_pkg) = packages.get(&dep.name) {
                        let cross_ref = CrossReference {
                        cross_refs.push(cross_ref);
                    }
                }
            }
        }

        Ok(cross_refs)
    /// Calculate quality metrics
    #[instrument(skip(self))]
    pub async fn calculate_quality_metrics(&self, package: &str, version: &str) -> Result<QualityMetrics> {
        debug!(package = %package, version = %version, "Calculating quality metrics");

        let packages = self.packages.read().await;
        
        if let Some(pkg) = packages.get(package) {
            if let Some(ver) = pkg.versions.get(version) {
                let completeness = ver.coverage.coverage_percentage;
                let api_coverage = if ver.api_items.is_empty() {
                    0.0
                } else {
                    ver.api_items.iter()
                        .filter(|item| !item.documentation.is_empty())
                        .count() as f64 / ver.api_items.len() as f64 * 100.0
                
                let example_coverage = if ver.api_items.is_empty() {
                    0.0
                } else {
                    ver.api_items.iter()
                        .filter(|item| !item.examples.is_empty())
                        .count() as f64 / ver.api_items.len() as f64 * 100.0

                let freshness = {
                    let age_days = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - ver.publication.published_at) / 86400;
                    (100.0 - age_days as f64 * 0.5).max(0.0)

                let link_health = 95.0; // Would check actual links

                let overall_score = (completeness + api_coverage + example_coverage + freshness + link_health) / 5.0;

                return Ok(QualityMetrics {
                });
            }
        }

        Ok(QualityMetrics {
        })
    /// Get registry statistics
    pub async fn get_statistics(&self) -> RegistryStatistics {
        let statistics = self.statistics.read().await;
        statistics.clone()
    /// Load registry data from disk
    async fn load_registry_data(&self) -> Result<()> {
        if !self.config.index_file.exists() {
            return Ok(());
        let data = fs::read_to_string(&self.config.index_file).await.map_err(|e| {
            CursedError::General(format!("Failed to read registry index: {}", e))
        })?;

        let packages: HashMap<String, PackageDocumentation> = serde_json::from_str(&data).map_err(|e| {
            CursedError::Serialization(format!("Failed to parse registry index: {}", e))
        })?;

        let mut packages_guard = self.packages.write().await;
        *packages_guard = packages;

        Ok(())
    /// Save registry data to disk
    async fn save_registry_data(&self) -> Result<()> {
        let packages = self.packages.read().await;
        
        let data = serde_json::to_string_pretty(&*packages).map_err(|e| {
            CursedError::Serialization(format!("Failed to serialize registry data: {}", e))
        })?;

        fs::write(&self.config.index_file, data).await.map_err(|e| {
            CursedError::General(format!("Failed to write registry index: {}", e))
        })?;

        Ok(())
    /// Rebuild search index
    async fn rebuild_search_index(&self) -> Result<()> {
        let packages = self.packages.read().await;
        let mut search_index = self.search_index.write().await;
        
        search_index.clear();

        for (package_name, package_doc) in packages.iter() {
            for (version, version_doc) in &package_doc.versions {
                for api_item in &version_doc.api_items {
                    let result = RegistrySearchResult {

                    // Index by name
                    search_index.entry(api_item.name.clone())
                        .or_insert_with(Vec::new)
                        .push(result.clone());

                    // Index by type
                    search_index.entry(api_item.item_type.clone())
                        .or_insert_with(Vec::new)
                        .push(result);
                }
            }
        Ok(())
    /// Update search index for a specific package
    async fn update_search_index_for_package(&self, package_name: &str) -> Result<()> {
        // Simplified - would update only the specific package's entries
        self.rebuild_search_index().await
    /// Update registry statistics
    async fn update_statistics(&self) -> Result<()> {
        let packages = self.packages.read().await;
        let mut statistics = self.statistics.write().await;

        statistics.total_packages = packages.len();
        statistics.total_versions = packages.values().map(|p| p.versions.len()).sum();
        statistics.total_api_items = packages.values()
            .flat_map(|p| p.versions.values())
            .map(|v| v.api_items.len())
            .sum();
        statistics.total_examples = packages.values()
            .flat_map(|p| p.versions.values())
            .map(|v| v.examples.len())
            .sum();

        let quality_scores: Vec<f64> = packages.values()
            .map(|p| p.quality_metrics.overall_score)
            .collect();
        
        statistics.avg_quality_score = if quality_scores.is_empty() {
            0.0
        } else {
            quality_scores.iter().sum::<f64>() / quality_scores.len() as f64

        Ok(())
    /// Check if version A is newer than version B
    fn is_newer_version(&self, version_a: &str, version_b: &str) -> bool {
        // Simplified version comparison - would use proper semver comparison
        version_a > version_b
    /// Validate registry configuration
    pub fn validate_config(&self) -> Result<()> {
        if !self.config.data_dir.exists() {
            return Err(CursedError::Configuration(
                format!("Registry data directory does not exist: {}", self.config.data_dir.display())
            ));
        if self.config.cache_size == 0 {
            return Err(CursedError::Configuration(
                "Cache size must be greater than 0".to_string()
            ));
        if self.config.max_dependency_depth == 0 {
            return Err(CursedError::Configuration(
                "Maximum dependency depth must be greater than 0".to_string()
            ));
        Ok(())
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./registry"),
            index_file: PathBuf::from("./registry/index.json"),
            refresh_interval: 3600, // 1 hour
        }
    }
