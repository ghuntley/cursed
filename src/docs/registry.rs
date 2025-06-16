//! Documentation Registry
//! 
//! Central registry for all published CURSED documentation with package metadata,
//! version tracking, and cross-reference resolution.

use crate::error::{CursedError, Result};
use crate::docs::publisher::PublicationMetadata;
use crate::package::{Package, PackageManager};
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
    pub data_dir: PathBuf,
    /// Registry index file
    pub index_file: PathBuf,
    /// Package metadata cache size
    pub cache_size: usize,
    /// Metadata refresh interval (seconds)
    pub refresh_interval: u64,
    /// Enable automatic dependency resolution
    pub auto_resolve_deps: bool,
    /// Maximum dependency depth
    pub max_dependency_depth: usize,
}

/// Package documentation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDocumentation {
    /// Package name
    pub name: String,
    /// Available versions
    pub versions: HashMap<String, VersionDocumentation>,
    /// Latest version
    pub latest_version: String,
    /// Default version (may differ from latest)
    pub default_version: String,
    /// Package description
    pub description: String,
    /// Package authors
    pub authors: Vec<String>,
    /// Package license
    pub license: String,
    /// Repository URL
    pub repository: Option<String>,
    /// Homepage URL
    pub homepage: Option<String>,
    /// Documentation URL
    pub documentation_url: Option<String>,
    /// Tags/keywords
    pub tags: Vec<String>,
    /// Package categories
    pub categories: Vec<String>,
    /// Documentation quality metrics
    pub quality_metrics: QualityMetrics,
    /// Last updated timestamp
    pub last_updated: u64,
}

/// Version-specific documentation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDocumentation {
    /// Version string
    pub version: String,
    /// Publication metadata
    pub publication: PublicationMetadata,
    /// Module structure
    pub modules: HashMap<String, ModuleDocumentation>,
    /// Public API items
    pub api_items: Vec<ApiItem>,
    /// Dependencies
    pub dependencies: Vec<Dependency>,
    /// Examples
    pub examples: Vec<ExampleDocumentation>,
    /// Changelog
    pub changelog: Option<String>,
    /// Migration guide
    pub migration_guide: Option<String>,
    /// Coverage metrics
    pub coverage: CoverageMetrics,
    /// Cross-references
    pub cross_references: Vec<CrossReference>,
}

/// Module documentation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDocumentation {
    /// Module name
    pub name: String,
    /// Module path
    pub path: String,
    /// Module description
    pub description: String,
    /// Public functions
    pub functions: Vec<String>,
    /// Public structs/types
    pub types: Vec<String>,
    /// Public constants
    pub constants: Vec<String>,
    /// Submodules
    pub submodules: Vec<String>,
    /// Documentation completeness
    pub completeness: f64,
}

/// API item documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiItem {
    /// Item name
    pub name: String,
    /// Item type (function, struct, enum, etc.)
    pub item_type: String,
    /// Module path
    pub module: String,
    /// Item signature
    pub signature: String,
    /// Documentation summary
    pub summary: String,
    /// Full documentation
    pub documentation: String,
    /// Examples
    pub examples: Vec<String>,
    /// Deprecated flag
    pub deprecated: bool,
    /// Stability level
    pub stability: StabilityLevel,
    /// Source location
    pub source_location: Option<SourceLocation>,
}

/// Example documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDocumentation {
    /// Example name
    pub name: String,
    /// Example description
    pub description: String,
    /// Source code
    pub source_code: String,
    /// Output (if applicable)
    pub expected_output: Option<String>,
    /// Dependencies needed
    pub dependencies: Vec<String>,
    /// Category
    pub category: String,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Package name
    pub name: String,
    /// Version requirement
    pub version_req: String,
    /// Optional dependency
    pub optional: bool,
    /// Default features
    pub default_features: bool,
    /// Enabled features
    pub features: Vec<String>,
}

/// Cross-reference to other documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Reference type
    pub ref_type: String,
    /// Target package
    pub target_package: String,
    /// Target version
    pub target_version: String,
    /// Target item
    pub target_item: String,
    /// Reference description
    pub description: String,
}

/// Documentation quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Overall documentation score (0-100)
    pub overall_score: f64,
    /// Documentation completeness (0-100)
    pub completeness: f64,
    /// Example coverage (0-100)
    pub example_coverage: f64,
    /// API documentation coverage (0-100)
    pub api_coverage: f64,
    /// Link health (0-100)
    pub link_health: f64,
    /// Freshness score (0-100)
    pub freshness: f64,
}

/// Coverage metrics for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    /// Total public items
    pub total_items: usize,
    /// Documented public items
    pub documented_items: usize,
    /// Items with examples
    pub items_with_examples: usize,
    /// Coverage percentage
    pub coverage_percentage: f64,
    /// Missing documentation items
    pub missing_docs: Vec<String>,
}

/// Source location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// File path
    pub file: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
}

/// Stability level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StabilityLevel {
    Stable,
    Unstable,
    Experimental,
    Deprecated,
}

/// Difficulty level for examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Registry search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearchQuery {
    /// Search terms
    pub query: String,
    /// Package filter
    pub package: Option<String>,
    /// Version filter
    pub version: Option<String>,
    /// Item type filter
    pub item_type: Option<String>,
    /// Category filter
    pub category: Option<String>,
    /// Minimum quality score
    pub min_quality: Option<f64>,
    /// Sort order
    pub sort_by: SortOrder,
    /// Result limit
    pub limit: usize,
    /// Result offset
    pub offset: usize,
}

/// Sort order for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Relevance,
    Name,
    Quality,
    LastUpdated,
    Popularity,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySearchResult {
    /// Matched item
    pub item: ApiItem,
    /// Relevance score
    pub score: f64,
    /// Package information
    pub package: String,
    /// Version information
    pub version: String,
    /// Match context
    pub context: String,
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total packages
    pub total_packages: usize,
    /// Total versions
    pub total_versions: usize,
    /// Total API items
    pub total_api_items: usize,
    /// Total examples
    pub total_examples: usize,
    /// Average quality score
    pub avg_quality_score: f64,
    /// Top packages by quality
    pub top_packages: Vec<String>,
    /// Recent updates
    pub recent_updates: Vec<String>,
    /// Popular packages
    pub popular_packages: Vec<String>,
}

/// Documentation registry
pub struct DocumentationRegistry {
    config: RegistryConfig,
    packages: RwLock<HashMap<String, PackageDocumentation>>,
    search_index: RwLock<HashMap<String, Vec<RegistrySearchResult>>>,
    statistics: RwLock<RegistryStatistics>,
}

impl DocumentationRegistry {
    /// Create a new documentation registry
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            config,
            packages: RwLock::new(HashMap::new()),
            search_index: RwLock::new(HashMap::new()),
            statistics: RwLock::new(RegistryStatistics {
                total_packages: 0,
                total_versions: 0,
                total_api_items: 0,
                total_examples: 0,
                avg_quality_score: 0.0,
                top_packages: Vec::new(),
                recent_updates: Vec::new(),
                popular_packages: Vec::new(),
            }),
        }
    }

    /// Initialize the registry
    #[instrument(skip(self))]
    pub async fn initialize(&self) -> Result<()> {
        info!(data_dir = %self.config.data_dir.display(), "Initializing documentation registry");

        // Create data directory if it doesn't exist
        fs::create_dir_all(&self.config.data_dir).await.map_err(|e| {
            CursedError::Io(format!("Failed to create data directory: {}", e))
        })?;

        // Load existing registry data
        self.load_registry_data().await?;

        // Build search index
        self.rebuild_search_index().await?;

        // Update statistics
        self.update_statistics().await?;

        info!("Documentation registry initialized");
        Ok(())
    }

    /// Register a new package publication
    #[instrument(skip(self))]
    pub async fn register_publication(&self, metadata: &PublicationMetadata) -> Result<()> {
        info!(
            package = %metadata.package_name,
            version = %metadata.version,
            "Registering package publication"
        );

        let mut packages = self.packages.write().await;
        
        let package_name = &metadata.package_name;
        let version = &metadata.version;

        // Get or create package documentation
        let package_doc = packages.entry(package_name.clone()).or_insert_with(|| {
            PackageDocumentation {
                name: package_name.clone(),
                versions: HashMap::new(),
                latest_version: version.clone(),
                default_version: version.clone(),
                description: String::new(),
                authors: Vec::new(),
                license: String::new(),
                repository: None,
                homepage: None,
                documentation_url: Some(metadata.url.clone()),
                tags: Vec::new(),
                categories: Vec::new(),
                quality_metrics: QualityMetrics {
                    overall_score: 0.0,
                    completeness: 0.0,
                    example_coverage: 0.0,
                    api_coverage: 0.0,
                    link_health: 0.0,
                    freshness: 100.0,
                },
                last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            }
        });

        // Create version documentation
        let version_doc = VersionDocumentation {
            version: version.clone(),
            publication: metadata.clone(),
            modules: HashMap::new(),
            api_items: Vec::new(),
            dependencies: Vec::new(),
            examples: Vec::new(),
            changelog: None,
            migration_guide: None,
            coverage: CoverageMetrics {
                total_items: 0,
                documented_items: 0,
                items_with_examples: 0,
                coverage_percentage: 0.0,
                missing_docs: Vec::new(),
            },
            cross_references: Vec::new(),
        };

        package_doc.versions.insert(version.clone(), version_doc);

        // Update latest version if this is newer
        if self.is_newer_version(version, &package_doc.latest_version) {
            package_doc.latest_version = version.clone();
        }

        package_doc.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Save registry data
        self.save_registry_data().await?;

        // Update search index
        self.update_search_index_for_package(package_name).await?;

        // Update statistics
        self.update_statistics().await?;

        info!(
            package = %metadata.package_name,
            version = %metadata.version,
            "Package publication registered"
        );

        Ok(())
    }

    /// Get package documentation
    pub async fn get_package(&self, name: &str) -> Option<PackageDocumentation> {
        let packages = self.packages.read().await;
        packages.get(name).cloned()
    }

    /// Get version documentation
    pub async fn get_version(&self, package: &str, version: &str) -> Option<VersionDocumentation> {
        let packages = self.packages.read().await;
        packages.get(package)?.versions.get(version).cloned()
    }

    /// List all packages
    pub async fn list_packages(&self) -> Vec<String> {
        let packages = self.packages.read().await;
        packages.keys().cloned().collect()
    }

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
        }

        // Sort results
        match query.sort_by {
            SortOrder::Relevance => results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal)),
            SortOrder::Name => results.sort_by(|a, b| a.item.name.cmp(&b.item.name)),
            SortOrder::Quality => results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)),
            SortOrder::LastUpdated => {}, // Would sort by actual update time
            SortOrder::Popularity => {}, // Would sort by popularity metrics
        }

        // Apply pagination
        let total_results = results.len();
        let start = query.offset.min(total_results);
        let end = (query.offset + query.limit).min(total_results);
        
        Ok(results[start..end].to_vec())
    }

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
        }

        Ok(())
    }

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
                            ref_type: "dependency".to_string(),
                            target_package: dep.name.clone(),
                            target_version: dep_pkg.latest_version.clone(),
                            target_item: String::new(),
                            description: format!("Dependency on {}", dep.name),
                        };
                        cross_refs.push(cross_ref);
                    }
                }
            }
        }

        Ok(cross_refs)
    }

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
                };
                
                let example_coverage = if ver.api_items.is_empty() {
                    0.0
                } else {
                    ver.api_items.iter()
                        .filter(|item| !item.examples.is_empty())
                        .count() as f64 / ver.api_items.len() as f64 * 100.0
                };

                let freshness = {
                    let age_days = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - ver.publication.published_at) / 86400;
                    (100.0 - age_days as f64 * 0.5).max(0.0)
                };

                let link_health = 95.0; // Would check actual links

                let overall_score = (completeness + api_coverage + example_coverage + freshness + link_health) / 5.0;

                return Ok(QualityMetrics {
                    overall_score,
                    completeness,
                    example_coverage,
                    api_coverage,
                    link_health,
                    freshness,
                });
            }
        }

        Ok(QualityMetrics {
            overall_score: 0.0,
            completeness: 0.0,
            example_coverage: 0.0,
            api_coverage: 0.0,
            link_health: 0.0,
            freshness: 0.0,
        })
    }

    /// Get registry statistics
    pub async fn get_statistics(&self) -> RegistryStatistics {
        let statistics = self.statistics.read().await;
        statistics.clone()
    }

    /// Load registry data from disk
    async fn load_registry_data(&self) -> Result<()> {
        if !self.config.index_file.exists() {
            return Ok(());
        }

        let data = fs::read_to_string(&self.config.index_file).await.map_err(|e| {
            CursedError::Io(format!("Failed to read registry index: {}", e))
        })?;

        let packages: HashMap<String, PackageDocumentation> = serde_json::from_str(&data).map_err(|e| {
            CursedError::Serialization(format!("Failed to parse registry index: {}", e))
        })?;

        let mut packages_guard = self.packages.write().await;
        *packages_guard = packages;

        Ok(())
    }

    /// Save registry data to disk
    async fn save_registry_data(&self) -> Result<()> {
        let packages = self.packages.read().await;
        
        let data = serde_json::to_string_pretty(&*packages).map_err(|e| {
            CursedError::Serialization(format!("Failed to serialize registry data: {}", e))
        })?;

        fs::write(&self.config.index_file, data).await.map_err(|e| {
            CursedError::Io(format!("Failed to write registry index: {}", e))
        })?;

        Ok(())
    }

    /// Rebuild search index
    async fn rebuild_search_index(&self) -> Result<()> {
        let packages = self.packages.read().await;
        let mut search_index = self.search_index.write().await;
        
        search_index.clear();

        for (package_name, package_doc) in packages.iter() {
            for (version, version_doc) in &package_doc.versions {
                for api_item in &version_doc.api_items {
                    let result = RegistrySearchResult {
                        item: api_item.clone(),
                        score: 1.0,
                        package: package_name.clone(),
                        version: version.clone(),
                        context: api_item.summary.clone(),
                    };

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
        }

        Ok(())
    }

    /// Update search index for a specific package
    async fn update_search_index_for_package(&self, package_name: &str) -> Result<()> {
        // Simplified - would update only the specific package's entries
        self.rebuild_search_index().await
    }

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
        };

        Ok(())
    }

    /// Check if version A is newer than version B
    fn is_newer_version(&self, version_a: &str, version_b: &str) -> bool {
        // Simplified version comparison - would use proper semver comparison
        version_a > version_b
    }

    /// Validate registry configuration
    pub fn validate_config(&self) -> Result<()> {
        if !self.config.data_dir.exists() {
            return Err(CursedError::Configuration(
                format!("Registry data directory does not exist: {}", self.config.data_dir.display())
            ));
        }

        if self.config.cache_size == 0 {
            return Err(CursedError::Configuration(
                "Cache size must be greater than 0".to_string()
            ));
        }

        if self.config.max_dependency_depth == 0 {
            return Err(CursedError::Configuration(
                "Maximum dependency depth must be greater than 0".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./registry"),
            index_file: PathBuf::from("./registry/index.json"),
            cache_size: 1000,
            refresh_interval: 3600, // 1 hour
            auto_resolve_deps: true,
            max_dependency_depth: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_registry_creation() {
        let config = RegistryConfig::default();
        let registry = DocumentationRegistry::new(config);
        
        let packages = registry.list_packages().await;
        assert!(packages.is_empty());
    }
    
    #[test]
    fn test_version_comparison() {
        let config = RegistryConfig::default();
        let registry = DocumentationRegistry::new(config);
        
        assert!(registry.is_newer_version("1.1.0", "1.0.0"));
        assert!(!registry.is_newer_version("1.0.0", "1.1.0"));
    }
}
