/// Template Bundler - Advanced template optimization and bundling system
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, instrument, warn};

use crate::error::Error as CursedError;
use super::template_core::{TemplateConfig, TemplateLoader, Template};
use super::template_syntax::{TemplateAst, TemplateNode, TemplateExpression};
use super::template_cache::CacheEntry;

/// Template bundle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleConfig {
    /// Enable template minification
    pub enable_minification: bool,
    /// Enable template compression
    pub enable_compression: bool,
    /// Enable dependency optimization
    pub enable_dependency_optimization: bool,
    /// Enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Bundle format
    pub bundle_format: BundleFormat,
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    /// Maximum bundle size in bytes
    pub max_bundle_size: usize,
    /// Enable source maps
    pub enable_source_maps: bool,
    /// Include debug information
    pub include_debug_info: bool,
    /// Template versioning strategy
    pub versioning_strategy: VersioningStrategy,
}

impl Default for BundleConfig {
    fn default() -> Self {
        Self {
            enable_minification: true,
            enable_compression: true,
            enable_dependency_optimization: true,
            enable_dead_code_elimination: true,
            bundle_format: BundleFormat::Optimized,
            optimization_level: OptimizationLevel::Production,
            max_bundle_size: 5 * 1024 * 1024, // 5MB
            enable_source_maps: false,
            include_debug_info: false,
            versioning_strategy: VersioningStrategy::ContentHash,
        }
    }
}

/// Bundle output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BundleFormat {
    /// Raw template bundle
    Raw,
    /// Minified bundle
    Minified,
    /// Optimized bundle with transformations
    Optimized,
    /// Compressed bundle
    Compressed,
    /// Precompiled bundle (AST-based)
    Precompiled,
}

/// Template optimization levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization
    None,
    /// Basic optimization (minification only)
    Basic,
    /// Standard optimization (minification + compression)
    Standard,
    /// Production optimization (all optimizations)
    Production,
    /// Aggressive optimization (may break compatibility)
    Aggressive,
}

/// Template versioning strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// No versioning
    None,
    /// Timestamp-based versioning
    Timestamp,
    /// Content hash-based versioning
    ContentHash,
    /// Semantic versioning
    Semantic(String),
}

/// Template bundle metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleMetadata {
    /// Bundle identifier
    pub bundle_id: String,
    /// Bundle version
    pub version: String,
    /// Creation timestamp
    pub created_at: SystemTime,
    /// Source templates included
    pub templates: Vec<String>,
    /// Dependencies graph
    pub dependencies: HashMap<String, Vec<String>>,
    /// Bundle size information
    pub size_info: BundleSizeInfo,
    /// Optimization statistics
    pub optimization_stats: OptimizationStats,
    /// Checksum for integrity verification
    pub checksum: String,
}

/// Bundle size information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSizeInfo {
    /// Original size (before optimization)
    pub original_size: usize,
    /// Minified size
    pub minified_size: usize,
    /// Compressed size
    pub compressed_size: usize,
    /// Number of templates
    pub template_count: usize,
    /// Size reduction ratio
    pub reduction_ratio: f64,
}

/// Optimization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStats {
    /// Minification time
    pub minification_time: Duration,
    /// Compression time
    pub compression_time: Duration,
    /// Dead code elimination savings
    pub dead_code_eliminated: usize,
    /// Dependency optimizations applied
    pub dependency_optimizations: usize,
    /// Total optimization time
    pub total_optimization_time: Duration,
}

/// Template bundle entry
#[derive(Debug, Clone)]
pub struct BundleEntry {
    /// Template name
    pub name: String,
    /// Optimized template content
    pub content: String,
    /// Precompiled AST (if available)
    pub ast: Option<TemplateAst>,
    /// Source map (if enabled)
    pub source_map: Option<String>,
    /// Entry metadata
    pub metadata: HashMap<String, String>,
}

/// Complete template bundle
#[derive(Debug, Clone)]
pub struct TemplateBundle {
    /// Bundle metadata
    pub metadata: BundleMetadata,
    /// Bundle entries
    pub entries: HashMap<String, BundleEntry>,
    /// Bundle configuration used
    pub config: BundleConfig,
    /// Manifest for runtime loading
    pub manifest: BundleManifest,
}

/// Bundle manifest for runtime template loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleManifest {
    /// Bundle format version
    pub format_version: String,
    /// Template mappings
    pub templates: HashMap<String, TemplateMapping>,
    /// Dependency graph
    pub dependencies: HashMap<String, Vec<String>>,
    /// Bundle integrity information
    pub integrity: HashMap<String, String>,
    /// Loading instructions
    pub loader_config: HashMap<String, String>,
}

/// Template mapping in bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMapping {
    /// Template offset in bundle
    pub offset: usize,
    /// Template size
    pub size: usize,
    /// Compression type
    pub compression: Option<String>,
    /// Template checksum
    pub checksum: String,
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Template dependency analyzer
#[derive(Debug)]
pub struct DependencyAnalyzer {
    /// Discovered dependencies
    dependencies: HashMap<String, HashSet<String>>,
    /// Template loader for resolving includes
    loader: Arc<dyn TemplateLoader>,
}

impl DependencyAnalyzer {
    pub fn new(loader: Arc<dyn TemplateLoader>) -> Self {
        Self {
            dependencies: HashMap::new(),
            loader,
        }
    }
    
    /// Analyze dependencies for a template
    #[instrument(skip(self, ast))]
    pub fn analyze_dependencies(&mut self, template_name: &str, ast: &TemplateAst) -> Result<HashSet<String>, CursedError> {
        let mut deps = HashSet::new();
        self.analyze_nodes(&ast.nodes, &mut deps)?;
        self.dependencies.insert(template_name.to_string(), deps.clone());
        Ok(deps)
    }
    
    /// Analyze nodes for dependencies
    fn analyze_nodes(&self, nodes: &[TemplateNode], deps: &mut HashSet<String>) -> Result<(), CursedError> {
        for node in nodes {
            match node {
                TemplateNode::Block { block_type, attributes, content } => {
                    match block_type.as_str() {
                        "include" | "extends" | "import" => {
                            if let Some(template_attr) = attributes.get("template") {
                                deps.insert(template_attr.clone());
                            }
                        }
                        _ => {}
                    }
                    
                    if let Some(content_nodes) = content {
                        self.analyze_nodes(content_nodes, deps)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    /// Get all dependencies for a template (recursive)
    pub fn get_all_dependencies(&self, template_name: &str) -> HashSet<String> {
        let mut all_deps = HashSet::new();
        let mut to_process = VecDeque::new();
        to_process.push_back(template_name.to_string());
        
        while let Some(current_template) = to_process.pop_front() {
            if let Some(deps) = self.dependencies.get(&current_template) {
                for dep in deps {
                    if all_deps.insert(dep.clone()) {
                        to_process.push_back(dep.clone());
                    }
                }
            }
        }
        
        all_deps
    }
    
    /// Detect circular dependencies
    pub fn detect_circular_dependencies(&self) -> Result<Vec<Vec<String>>, CursedError> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for template_name in self.dependencies.keys() {
            if !visited.contains(template_name) {
                if let Some(cycle) = self.find_cycle(template_name, &mut visited, &mut rec_stack, &mut Vec::new()) {
                    cycles.push(cycle);
                }
            }
        }
        
        Ok(cycles)
    }
    
    /// Find circular dependency cycle
    fn find_cycle(
        &self,
        template: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(template.to_string());
        rec_stack.insert(template.to_string());
        path.push(template.to_string());
        
        if let Some(deps) = self.dependencies.get(template) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.find_cycle(dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found cycle
                    let cycle_start = path.iter().position(|t| t == dep).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }
        
        rec_stack.remove(template);
        path.pop();
        None
    }
}

/// Template bundler with optimization capabilities
pub struct TemplateBundler {
    /// Bundle configuration
    config: BundleConfig,
    /// Template loader
    loader: Arc<dyn TemplateLoader>,
    /// Dependency analyzer
    dependency_analyzer: DependencyAnalyzer,
    /// Optimization pipeline
    optimizers: Vec<Box<dyn TemplateOptimizer>>,
    /// Bundle cache
    bundle_cache: Arc<RwLock<HashMap<String, TemplateBundle>>>,
}

impl TemplateBundler {
    /// Create a new template bundler
    pub fn new(config: BundleConfig, loader: Arc<dyn TemplateLoader>) -> Self {
        let dependency_analyzer = DependencyAnalyzer::new(Arc::clone(&loader));
        
        let mut optimizers: Vec<Box<dyn TemplateOptimizer>> = Vec::new();
        
        if config.enable_minification {
            optimizers.push(Box::new(MinificationOptimizer::new()));
        }
        
        if config.enable_dead_code_elimination {
            optimizers.push(Box::new(DeadCodeEliminationOptimizer::new()));
        }
        
        if config.enable_dependency_optimization {
            optimizers.push(Box::new(DependencyOptimizer::new()));
        }
        
        Self {
            config,
            loader,
            dependency_analyzer,
            optimizers,
            bundle_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a bundle from a list of template names
    #[instrument(skip(self, template_names))]
    pub async fn create_bundle(&mut self, template_names: &[String], bundle_id: &str) -> Result<TemplateBundle, CursedError> {
        let start_time = Instant::now();
        info!(bundle_id = bundle_id, template_count = template_names.len(), "Creating template bundle");
        
        // Check cache first
        if let Ok(cache) = self.bundle_cache.read() {
            if let Some(cached_bundle) = cache.get(bundle_id) {
                debug!("Bundle found in cache: {}", bundle_id);
                return Ok(cached_bundle.clone());
            }
        }
        
        let mut entries = HashMap::new();
        let mut all_dependencies = HashMap::new();
        let mut optimization_stats = OptimizationStats {
            minification_time: Duration::from_secs(0),
            compression_time: Duration::from_secs(0),
            dead_code_eliminated: 0,
            dependency_optimizations: 0,
            total_optimization_time: Duration::from_secs(0),
        };
        
        // Analyze dependencies for all templates
        for template_name in template_names {
            let template_source = self.loader.load(template_name)?;
            let ast = self.parse_template(&template_source)?;
            let deps = self.dependency_analyzer.analyze_dependencies(template_name, &ast)?;
            all_dependencies.insert(template_name.clone(), deps.into_iter().collect());
        }
        
        // Check for circular dependencies
        let circular_deps = self.dependency_analyzer.detect_circular_dependencies()?;
        if !circular_deps.is_empty() {
            warn!("Circular dependencies detected: {:?}", circular_deps);
        }
        
        // Collect all templates to include (including dependencies)
        let mut all_templates = HashSet::new();
        for template_name in template_names {
            all_templates.insert(template_name.clone());
            for dep in self.dependency_analyzer.get_all_dependencies(template_name) {
                all_templates.insert(dep);
            }
        }
        
        let mut original_size = 0;
        let mut optimized_size = 0;
        
        // Process each template
        for template_name in &all_templates {
            let template_source = self.loader.load(template_name)?;
            original_size += template_source.len();
            
            let mut optimized_content = template_source.clone();
            let mut ast = self.parse_template(&template_source)?;
            
            // Apply optimizations
            for optimizer in &self.optimizers {
                let opt_start = Instant::now();
                let opt_result = optimizer.optimize(&mut optimized_content, &mut ast)?;
                let opt_time = opt_start.elapsed();
                
                match optimizer.optimizer_type() {
                    OptimizerType::Minification => optimization_stats.minification_time += opt_time,
                    OptimizerType::DeadCodeElimination => {
                        optimization_stats.dead_code_eliminated += opt_result.bytes_saved;
                    }
                    OptimizerType::DependencyOptimization => {
                        optimization_stats.dependency_optimizations += opt_result.optimizations_applied;
                    }
                    _ => {}
                }
            }
            
            optimized_size += optimized_content.len();
            
            // Create bundle entry
            entries.insert(template_name.clone(), BundleEntry {
                name: template_name.clone(),
                content: optimized_content,
                ast: Some(ast),
                source_map: None,
                metadata: HashMap::new(),
            });
        }
        
        // Generate bundle version
        let version = self.generate_version(&all_templates)?;
        
        // Create bundle metadata
        let metadata = BundleMetadata {
            bundle_id: bundle_id.to_string(),
            version,
            created_at: SystemTime::now(),
            templates: all_templates.into_iter().collect(),
            dependencies: all_dependencies,
            size_info: BundleSizeInfo {
                original_size,
                minified_size: optimized_size,
                compressed_size: optimized_size, // TODO: Add actual compression
                template_count: entries.len(),
                reduction_ratio: if original_size > 0 { 
                    (original_size - optimized_size) as f64 / original_size as f64 
                } else { 
                    0.0 
                },
            },
            optimization_stats,
            checksum: self.calculate_checksum(&entries)?,
        };
        
        // Create bundle manifest
        let manifest = self.create_bundle_manifest(&entries, &metadata)?;
        
        let bundle = TemplateBundle {
            metadata,
            entries,
            config: self.config.clone(),
            manifest,
        };
        
        // Cache the bundle
        if let Ok(mut cache) = self.bundle_cache.write() {
            cache.insert(bundle_id.to_string(), bundle.clone());
        }
        
        let total_time = start_time.elapsed();
        info!(
            bundle_id = bundle_id,
            template_count = bundle.entries.len(),
            original_size = bundle.metadata.size_info.original_size,
            optimized_size = bundle.metadata.size_info.minified_size,
            reduction_ratio = bundle.metadata.size_info.reduction_ratio,
            bundle_time_ms = total_time.as_millis(),
            "Bundle creation completed"
        );
        
        Ok(bundle)
    }
    
    /// Parse template source into AST
    fn parse_template(&self, source: &str) -> Result<TemplateAst, CursedError> {
        use super::template_syntax::{TemplateLexer, TemplateParser};
        use super::template_core::TemplateDelimiters;
        
        let delimiters = TemplateDelimiters {
            variable: ("{{".to_string(), "}}".to_string()),
            block: ("{%".to_string(), "%}".to_string()),
            comment: ("{#".to_string(), "#}".to_string()),
        };
        
        let mut lexer = TemplateLexer::new(source, &delimiters);
        let tokens = lexer.tokenize()?;
        let mut parser = TemplateParser::new(tokens);
        parser.parse()
    }
    
    /// Generate bundle version based on versioning strategy
    fn generate_version(&self, templates: &HashSet<String>) -> Result<String, CursedError> {
        match &self.config.versioning_strategy {
            VersioningStrategy::None => Ok("1.0.0".to_string()),
            VersioningStrategy::Timestamp => {
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|e| CursedError::TemplateError {
                        message: format!("Failed to get timestamp: {}", e),
                        source_location: None,
                    })?;
                Ok(format!("t{}", timestamp.as_secs()))
            }
            VersioningStrategy::ContentHash => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                
                let mut hasher = DefaultHasher::new();
                let mut sorted_templates: Vec<_> = templates.iter().collect();
                sorted_templates.sort();
                
                for template_name in sorted_templates {
                    template_name.hash(&mut hasher);
                    if let Ok(content) = self.loader.load(template_name) {
                        content.hash(&mut hasher);
                    }
                }
                
                Ok(format!("h{:x}", hasher.finish()))
            }
            VersioningStrategy::Semantic(version) => Ok(version.clone()),
        }
    }
    
    /// Calculate bundle checksum
    fn calculate_checksum(&self, entries: &HashMap<String, BundleEntry>) -> Result<String, CursedError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        let mut sorted_entries: Vec<_> = entries.iter().collect();
        sorted_entries.sort_by_key(|(name, _)| *name);
        
        for (name, entry) in sorted_entries {
            name.hash(&mut hasher);
            entry.content.hash(&mut hasher);
        }
        
        Ok(format!("{:x}", hasher.finish()))
    }
    
    /// Create bundle manifest
    fn create_bundle_manifest(&self, entries: &HashMap<String, BundleEntry>, metadata: &BundleMetadata) -> Result<BundleManifest, CursedError> {
        let mut templates = HashMap::new();
        let mut offset = 0;
        
        for (name, entry) in entries {
            templates.insert(name.clone(), TemplateMapping {
                offset,
                size: entry.content.len(),
                compression: None,
                checksum: self.calculate_entry_checksum(entry)?,
                dependencies: metadata.dependencies.get(name).cloned().unwrap_or_default(),
            });
            offset += entry.content.len();
        }
        
        Ok(BundleManifest {
            format_version: "1.0".to_string(),
            templates,
            dependencies: metadata.dependencies.clone(),
            integrity: HashMap::new(),
            loader_config: HashMap::new(),
        })
    }
    
    /// Calculate checksum for a single entry
    fn calculate_entry_checksum(&self, entry: &BundleEntry) -> Result<String, CursedError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        entry.content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }
    
    /// Serialize bundle to bytes
    pub fn serialize_bundle(&self, bundle: &TemplateBundle) -> Result<Vec<u8>, CursedError> {
        serde_json::to_vec(bundle).map_err(|e| CursedError::TemplateError {
            message: format!("Failed to serialize bundle: {}", e),
            source_location: None,
        })
    }
    
    /// Deserialize bundle from bytes
    pub fn deserialize_bundle(&self, data: &[u8]) -> Result<TemplateBundle, CursedError> {
        serde_json::from_slice(data).map_err(|e| CursedError::TemplateError {
            message: format!("Failed to deserialize bundle: {}", e),
            source_location: None,
        })
    }
    
    /// Get bundle cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        if let Ok(cache) = self.bundle_cache.read() {
            let bundle_count = cache.len();
            let total_size = cache.values()
                .map(|bundle| bundle.metadata.size_info.minified_size)
                .sum();
            (bundle_count, total_size)
        } else {
            (0, 0)
        }
    }
    
    /// Clear bundle cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.bundle_cache.write() {
            cache.clear();
        }
    }
}

/// Template optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Bytes saved by optimization
    pub bytes_saved: usize,
    /// Number of optimizations applied
    pub optimizations_applied: usize,
    /// Optimization warnings
    pub warnings: Vec<String>,
}

/// Optimizer types
#[derive(Debug, Clone)]
pub enum OptimizerType {
    Minification,
    DeadCodeElimination,
    DependencyOptimization,
    Compression,
}

/// Template optimizer trait
pub trait TemplateOptimizer: Send + Sync {
    /// Optimize template content and AST
    fn optimize(&self, content: &mut String, ast: &mut TemplateAst) -> Result<OptimizationResult, CursedError>;
    
    /// Get optimizer type
    fn optimizer_type(&self) -> OptimizerType;
    
    /// Get optimizer name
    fn name(&self) -> &str;
}

/// Minification optimizer
pub struct MinificationOptimizer {
    /// Remove whitespace between tags
    pub remove_whitespace: bool,
    /// Remove empty lines
    pub remove_empty_lines: bool,
    /// Remove comments
    pub remove_comments: bool,
}

impl MinificationOptimizer {
    pub fn new() -> Self {
        Self {
            remove_whitespace: true,
            remove_empty_lines: true,
            remove_comments: true,
        }
    }
}

impl TemplateOptimizer for MinificationOptimizer {
    fn optimize(&self, content: &mut String, _ast: &mut TemplateAst) -> Result<OptimizationResult, CursedError> {
        let original_size = content.len();
        let mut optimizations_applied = 0;
        
        if self.remove_comments {
            // Remove template comments {# ... #}
            let comment_regex = regex::Regex::new(r"\{#.*?#\}").unwrap();
            let new_content = comment_regex.replace_all(content, "");
            if new_content.len() != content.len() {
                optimizations_applied += 1;
                *content = new_content.to_string();
            }
        }
        
        if self.remove_empty_lines {
            let lines: Vec<&str> = content.lines()
                .filter(|line| !line.trim().is_empty())
                .collect();
            let new_content = lines.join("\n");
            if new_content.len() != content.len() {
                optimizations_applied += 1;
                *content = new_content;
            }
        }
        
        if self.remove_whitespace {
            // Remove excessive whitespace
            let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
            let new_content = whitespace_regex.replace_all(content, " ");
            if new_content.len() != content.len() {
                optimizations_applied += 1;
                *content = new_content.to_string();
            }
        }
        
        let bytes_saved = original_size.saturating_sub(content.len());
        
        Ok(OptimizationResult {
            bytes_saved,
            optimizations_applied,
            warnings: Vec::new(),
        })
    }
    
    fn optimizer_type(&self) -> OptimizerType {
        OptimizerType::Minification
    }
    
    fn name(&self) -> &str {
        "MinificationOptimizer"
    }
}

/// Dead code elimination optimizer
pub struct DeadCodeEliminationOptimizer {
    /// Variables that are never used
    unused_variables: HashSet<String>,
}

impl DeadCodeEliminationOptimizer {
    pub fn new() -> Self {
        Self {
            unused_variables: HashSet::new(),
        }
    }
}

impl TemplateOptimizer for DeadCodeEliminationOptimizer {
    fn optimize(&self, content: &mut String, ast: &mut TemplateAst) -> Result<OptimizationResult, CursedError> {
        let original_size = content.len();
        
        // TODO: Implement dead code elimination logic
        // This would involve analyzing the AST to find unused variables, 
        // unreachable blocks, etc.
        
        Ok(OptimizationResult {
            bytes_saved: 0,
            optimizations_applied: 0,
            warnings: vec!["Dead code elimination not yet implemented".to_string()],
        })
    }
    
    fn optimizer_type(&self) -> OptimizerType {
        OptimizerType::DeadCodeElimination
    }
    
    fn name(&self) -> &str {
        "DeadCodeEliminationOptimizer"
    }
}

/// Dependency optimization optimizer
pub struct DependencyOptimizer {
    /// Inline small dependencies
    pub inline_small_dependencies: bool,
    /// Maximum size for inlining
    pub inline_threshold: usize,
}

impl DependencyOptimizer {
    pub fn new() -> Self {
        Self {
            inline_small_dependencies: true,
            inline_threshold: 1024, // 1KB
        }
    }
}

impl TemplateOptimizer for DependencyOptimizer {
    fn optimize(&self, content: &mut String, ast: &mut TemplateAst) -> Result<OptimizationResult, CursedError> {
        // TODO: Implement dependency optimization logic
        // This would involve inlining small templates, optimizing include chains, etc.
        
        Ok(OptimizationResult {
            bytes_saved: 0,
            optimizations_applied: 0,
            warnings: vec!["Dependency optimization not yet implemented".to_string()],
        })
    }
    
    fn optimizer_type(&self) -> OptimizerType {
        OptimizerType::DependencyOptimization
    }
    
    fn name(&self) -> &str {
        "DependencyOptimizer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::template::template_core::FileSystemLoader;
    
    #[test]
    fn test_bundle_config_creation() {
        let config = BundleConfig::default();
        assert!(config.enable_minification);
        assert!(config.enable_compression);
        assert_eq!(config.optimization_level, OptimizationLevel::Production);
    }
    
    #[test]
    fn test_dependency_analyzer() {
        let loader = Arc::new(FileSystemLoader::new("templates"));
        let mut analyzer = DependencyAnalyzer::new(loader);
        
        // Create a simple AST with an include
        let nodes = vec![
            TemplateNode::Block {
                block_type: "include".to_string(),
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("template".to_string(), "header.html".to_string());
                    attrs
                },
                content: None,
            }
        ];
        
        let ast = TemplateAst { nodes };
        let deps = analyzer.analyze_dependencies("main.html", &ast).unwrap();
        
        assert!(deps.contains("header.html"));
    }
    
    #[test]
    fn test_minification_optimizer() {
        let optimizer = MinificationOptimizer::new();
        let mut content = "  \n\n  Hello   World  \n\n  ".to_string();
        let mut ast = TemplateAst { nodes: Vec::new() };
        
        let result = optimizer.optimize(&mut content, &mut ast).unwrap();
        
        assert!(result.bytes_saved > 0);
        assert!(result.optimizations_applied > 0);
        assert!(content.len() < 20); // Original was longer
    }
    
    #[test]
    fn test_bundle_metadata_creation() {
        let metadata = BundleMetadata {
            bundle_id: "test_bundle".to_string(),
            version: "1.0.0".to_string(),
            created_at: SystemTime::now(),
            templates: vec!["template1.html".to_string(), "template2.html".to_string()],
            dependencies: HashMap::new(),
            size_info: BundleSizeInfo {
                original_size: 1000,
                minified_size: 800,
                compressed_size: 600,
                template_count: 2,
                reduction_ratio: 0.2,
            },
            optimization_stats: OptimizationStats {
                minification_time: Duration::from_millis(10),
                compression_time: Duration::from_millis(5),
                dead_code_eliminated: 100,
                dependency_optimizations: 2,
                total_optimization_time: Duration::from_millis(15),
            },
            checksum: "abc123".to_string(),
        };
        
        assert_eq!(metadata.bundle_id, "test_bundle");
        assert_eq!(metadata.templates.len(), 2);
        assert_eq!(metadata.size_info.reduction_ratio, 0.2);
    }
}
