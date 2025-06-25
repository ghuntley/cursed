use crate::error::CursedError;
/// Template Bundler - Advanced template optimization and bundling system
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, instrument, warn};

use super::template_core::{TemplateConfig, TemplateLoader, Template};
use super::template_syntax::{TemplateAst, TemplateNode, TemplateExpression, BlockNode};
use super::template_cache::CacheEntry;

/// Template bundle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleConfig {
    /// Enable template minification
    /// Enable template compression
    /// Enable dependency optimization
    /// Enable dead code elimination
    /// Bundle format
    /// Optimization level
    /// Maximum bundle size in bytes
    /// Enable source maps
    /// Include debug information
    /// Template versioning strategy
impl Default for BundleConfig {
    fn default() -> Self {
        Self {
            max_bundle_size: 5 * 1024 * 1024, // 5MB
        }
    }
/// Bundle output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BundleFormat {
    /// Raw template bundle
    /// Minified bundle
    /// Optimized bundle with transformations
    /// Compressed bundle
    /// Precompiled bundle (AST-based)
// Import canonical OptimizationLevel from optimization_config
pub use crate::common_types::optimization_level::OptimizationLevel;

/// Template versioning strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// No versioning
    /// Timestamp-based versioning
    /// Content hash-based versioning
    /// Semantic versioning
/// Template bundle metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleMetadata {
    /// Bundle identifier
    /// Bundle version
    /// Creation timestamp
    /// Source templates included
    /// Dependencies graph
    /// Bundle size information
    /// Optimization statistics
    /// Checksum for integrity verification
/// Bundle size information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSizeInfo {
    /// Original size (before optimization)
    /// Minified size
    /// Compressed size
    /// Number of templates
    /// Size reduction ratio
/// Optimization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStats {
    /// Minification time
    /// Compression time
    /// Dead code elimination savings
    /// Dependency optimizations applied
    /// Total optimization time
/// Template bundle entry
#[derive(Debug, Clone)]
pub struct BundleEntry {
    /// Template name
    /// Optimized template content
    /// Precompiled AST (if available)
    /// Source map (if enabled)
    /// Entry metadata
/// Complete template bundle
#[derive(Debug, Clone)]
pub struct TemplateBundle {
    /// Bundle metadata
    /// Bundle entries
    /// Bundle configuration used
    /// Manifest for runtime loading
/// Bundle manifest for runtime template loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleManifest {
    /// Bundle format version
    /// Template mappings
    /// Dependency graph
    /// Bundle integrity information
    /// Loading instructions
/// Template mapping in bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMapping {
    /// Template offset in bundle
    /// Template size
    /// Compression type
    /// Template checksum
    /// Dependencies
/// Template dependency analyzer
#[derive(Debug)]
pub struct DependencyAnalyzer {
    /// Discovered dependencies
    /// Template loader for resolving includes
impl DependencyAnalyzer {
    pub fn new(loader: Arc<dyn TemplateLoader>) -> Self {
        Self {
        }
    }
    
    /// Analyze dependencies for a template
    #[instrument(skip(self, ast))]
    pub fn analyze_dependencies(&mut self, template_name: &str, ast: &TemplateAst) -> crate::error::Result<HashSet<String>> {
        let mut deps = HashSet::new();
        self.analyze_nodes(&ast.nodes, &mut deps)?;
        self.dependencies.insert(template_name.to_string(), deps.clone());
        Ok(deps)
    /// Analyze nodes for dependencies
    fn analyze_nodes(&self, nodes: &[TemplateNode], deps: &mut HashSet<String>) -> crate::error::Result<()> {
        for node in nodes {
            match node {
                TemplateNode::Block { block, .. } => {
                    self.analyze_block_nodes(block, deps)?;
                }
                TemplateNode::Include { template_name, .. } => {
                    deps.insert(template_name.clone());
                }
                TemplateNode::Extends { name, .. } => {
                    deps.insert(name.clone());
                }
                TemplateNode::LowkeyIf { then_branch, else_branch, .. } => {
                    self.analyze_nodes(then_branch, deps)?;
                    if let Some(else_nodes) = else_branch {
                        self.analyze_nodes(else_nodes, deps)?;
                    }
                }
                TemplateNode::StanLoop { body, .. } => {
                    self.analyze_nodes(body, deps)?;
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.analyze_nodes(content, deps)?;
                }
                _ => {}
            }
        }
        Ok(())
    /// Analyze block nodes for dependencies
    fn analyze_block_nodes(&self, block: &BlockNode, deps: &mut HashSet<String>) -> crate::error::Result<()> {
        match block {
            BlockNode::If { then_branch, else_branch, .. } => {
                self.analyze_nodes(then_branch, deps)?;
                if let Some(else_nodes) = else_branch {
                    self.analyze_nodes(else_nodes, deps)?;
                }
            }
            BlockNode::For { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
            BlockNode::While { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
            BlockNode::When { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
            BlockNode::Each { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
            BlockNode::Loop { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
            BlockNode::RangeFor { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
            BlockNode::Match { cases, default_case, .. } => {
                for case in cases {
                    self.analyze_nodes(&case.body, deps)?;
                }
                if let Some(default_nodes) = default_case {
                    self.analyze_nodes(default_nodes, deps)?;
                }
            }
            BlockNode::With { body, .. } => {
                self.analyze_nodes(body, deps)?;
            }
        }
        Ok(())
    /// Get all dependencies for a template (recursive)
    pub fn get_all_dependencies(&self, template_name: &str) -> Vec<String> {
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
        
        all_deps.into_iter().collect()
    /// Detect circular dependencies
    pub fn detect_circular_dependencies(&self) -> crate::error::Result<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for template_name in self.dependencies.keys() {
            if !visited.contains(template_name) {
                if let Some(cycle) = self.find_cycle(template_name, &mut visited, &mut rec_stack, &mut Vec::new()) {
                    cycles.push(cycle);
                }
            }
        Ok(cycles)
    /// Find circular dependency cycle
    fn find_cycle(
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
        rec_stack.remove(template);
        path.pop();
        None
    }
}

/// Template bundler with optimization capabilities
pub struct TemplateBundler {
    /// Bundle configuration
    /// Template loader
    /// Dependency analyzer
    /// Optimization pipeline
    /// Bundle cache
impl TemplateBundler {
    /// Create a new template bundler
    pub fn new(config: BundleConfig, loader: Arc<dyn TemplateLoader>) -> Self {
        let dependency_analyzer = DependencyAnalyzer::new(Arc::clone(&loader));
        
        let mut optimizers: Vec<Box<dyn TemplateOptimizer>> = Vec::new();
        
        if config.enable_minification {
            optimizers.push(Box::new(MinificationOptimizer::new()));
        if config.enable_dead_code_elimination {
            optimizers.push(Box::new(DeadCodeEliminationOptimizer::new()));
        if config.enable_dependency_optimization {
            optimizers.push(Box::new(DependencyOptimizer::new()));
        Self {
        }
    }
    
    /// Create a bundle from a list of template names
    #[instrument(skip(self, template_names))]
    pub async fn create_bundle(&mut self, template_names: &[String], bundle_id: &str) -> crate::error::Result<TemplateBundle> {
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
        
        // Analyze dependencies for all templates
        for template_name in template_names {
            let template_source = self.loader.load(template_name)?;
            let ast = self.parse_template(&template_source)?;
            let deps = self.dependency_analyzer.analyze_dependencies(template_name, &ast)?;
            all_dependencies.insert(template_name.clone(), deps.into_iter().collect());
        // Check for circular dependencies
        let circular_deps = self.dependency_analyzer.detect_circular_dependencies()?;
        if !circular_deps.is_empty() {
            warn!("Circular dependencies detected: {:?}", circular_deps);
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
            });
        // Generate bundle version
        let version = self.generate_version(&all_templates)?;
        
        // Calculate compressed size using actual compression
        let compressed_size = self.calculate_compressed_size(&entries)?;
        optimization_stats.compression_time = {
            let compress_start = Instant::now();
            let _ = self.calculate_compressed_size(&entries)?; // Measure compression time
            compress_start.elapsed()

        // Create bundle metadata
        let metadata = BundleMetadata {
            size_info: BundleSizeInfo {
                reduction_ratio: if original_size > 0 { 
                    (original_size - optimized_size) as f64 / original_size as f64 
                } else { 
                    0.0 
        
        // Create bundle manifest
        let manifest = self.create_bundle_manifest(&entries, &metadata)?;
        
        let bundle = TemplateBundle {
        
        // Cache the bundle
        if let Ok(mut cache) = self.bundle_cache.write() {
            cache.insert(bundle_id.to_string(), bundle.clone());
        let total_time = start_time.elapsed();
        info!(
            "Bundle creation completed"
        );
        
        Ok(bundle)
    /// Parse template source into AST
    fn parse_template(&self, source: &str) -> crate::error::Result<TemplateAst> {
        use super::template_syntax::{TemplateLexer, TemplateParser};
        use super::template_core::TemplateDelimiters;
        
        let delimiters = TemplateDelimiters {
        
        let mut lexer = TemplateLexer::new(source, &delimiters);
        let tokens = lexer.tokenize()?;
        let mut parser = TemplateParser::new(tokens);
        parser.parse()
    /// Calculate bundle checksum
    fn calculate_checksum(&self, entries: &HashMap<String, BundleEntry>) -> crate::error::Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        for (name, entry) in entries {
            name.hash(&mut hasher);
            entry.content.hash(&mut hasher);
        }
        Ok(format!("{:x}", hasher.finish()))
    /// Create bundle manifest
    fn create_bundle_manifest(&self, entries: &HashMap<String, BundleEntry>, metadata: &BundleMetadata) -> crate::error::Result<BundleManifest> {
        Ok(BundleManifest {
        })
    /// Generate bundle version based on versioning strategy
    fn generate_version(&self, templates: &HashSet<String>) -> crate::error::Result<String> {
        match &self.config.versioning_strategy {
            VersioningStrategy::Timestamp => {
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|e| CursedError::TemplateError {
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
        }
    }
    
    /// Calculate compressed size for all bundle entries
    fn calculate_compressed_size(&self, entries: &HashMap<String, BundleEntry>) -> crate::error::Result<()> {
        if !self.config.enable_compression {
            return Ok(entries.values().map(|e| e.content.len()).sum());
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut total_compressed_size = 0;

        for entry in entries.values() {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(entry.content.as_bytes())
                .map_err(|e| CursedError::TemplateError {
                })?;
            
            let compressed_data = encoder.finish()
                .map_err(|e| CursedError::TemplateError {
                })?;
            
            total_compressed_size += compressed_data.len();
        Ok(total_compressed_size)
    /// Calculate bundle checksum
    fn calculate_checksum(&self, entries: &HashMap<String, BundleEntry>) -> crate::error::Result<()> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        let mut sorted_entries: Vec<_> = entries.iter().collect();
        sorted_entries.sort_by_key(|(name, _)| *name);
        
        for (name, entry) in sorted_entries {
            name.hash(&mut hasher);
            entry.content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    /// Create bundle manifest
    fn create_bundle_manifest(&self, entries: &HashMap<String, BundleEntry>, metadata: &BundleMetadata) -> crate::error::Result<()> {
        let mut templates = HashMap::new();
        let mut offset = 0;
        
        for (name, entry) in entries {
            templates.insert(name.clone(), TemplateMapping {
            });
            offset += entry.content.len();
        Ok(BundleManifest {
        })
    /// Calculate checksum for a single entry
    fn calculate_entry_checksum(&self, entry: &BundleEntry) -> crate::error::Result<()> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        entry.content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    /// Serialize bundle to bytes
    pub fn serialize_bundle(&self, bundle: &TemplateBundle) -> crate::error::Result<Vec<u8>> {
        serde_json::to_vec(bundle).map_err(|e| CursedError::TemplateError {
        })
    /// Deserialize bundle from bytes
    pub fn deserialize_bundle(&self, data: &[u8]) -> crate::error::Result<TemplateBundle> {
        serde_json::from_slice(data).map_err(|e| CursedError::TemplateError {
        })
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
/// Template optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Bytes saved by optimization
    /// Number of optimizations applied
    /// Optimization warnings
/// Optimizer types
#[derive(Debug, Clone)]
pub enum OptimizerType {


/// Template optimizer trait
pub trait TemplateOptimizer: Send + Sync {
    /// Optimize template content and AST
    fn optimize(&self, content: &mut String, ast: &mut TemplateAst) -> crate::error::Result<OptimizationResult>;
    
    /// Get optimizer type
    fn optimizer_type(&self) -> OptimizerType;
    
    /// Get optimizer name
    fn name(&self) -> &str;
/// Minification optimizer
pub struct MinificationOptimizer {
    /// Remove whitespace between tags
    /// Remove empty lines
    /// Remove comments
impl MinificationOptimizer {
    pub fn new() -> Self {
        Self {
        }
    }
impl TemplateOptimizer for MinificationOptimizer {
    fn optimize(&self, content: &mut String, _ast: &mut TemplateAst) -> crate::error::Result<OptimizationResult> {
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
            let lines: Vec<&str> = content.split("\n")
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
        })
    fn optimizer_type(&self) -> OptimizerType {
        OptimizerType::Minification
    fn name(&self) -> &str {
        "MinificationOptimizer"
    }
}

/// Dead code elimination optimizer
pub struct DeadCodeEliminationOptimizer {
    /// Variables that are never used
impl DeadCodeEliminationOptimizer {
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Analyze variable usage in template nodes
    fn analyze_variable_usage(&self, nodes: &[TemplateNode], defined: &mut HashSet<String>, used: &mut HashSet<String>) {
        for node in nodes {
            match node {
                TemplateNode::Variable { expression, .. } => {
                    self.collect_used_variables(expression, used);
                }
                TemplateNode::Set { name, value, .. } => {
                    defined.insert(name.clone());
                    self.collect_used_variables(value, used);
                }
                TemplateNode::LowkeyIf { condition, then_branch, else_branch, .. } => {
                    self.collect_used_variables(condition, used);
                    self.analyze_variable_usage(then_branch, defined, used);
                    if let Some(else_nodes) = else_branch {
                        self.analyze_variable_usage(else_nodes, defined, used);
                    }
                }
                TemplateNode::StanLoop { variable, iterator, body, .. } => {
                    defined.insert(variable.clone());
                    self.collect_used_variables(iterator, used);
                    self.analyze_variable_usage(body, defined, used);
                }
                TemplateNode::Block { block, .. } => {
                    self.analyze_block_variables(block, defined, used);
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.analyze_variable_usage(content, defined, used);
                }
                TemplateNode::Include { context, .. } => {
                    if let Some(ctx) = context {
                        for expr in ctx.values() {
                            self.collect_used_variables(expr, used);
                        }
                    }
                }
                _ => {}
            }
        }
    /// Analyze variables in block nodes
    fn analyze_block_variables(&self, block: &BlockNode, defined: &mut HashSet<String>, used: &mut HashSet<String>) {
        match block {
            BlockNode::If { condition, then_branch, else_branch, .. } => {
                self.collect_used_variables(condition, used);
                self.analyze_variable_usage(then_branch, defined, used);
                if let Some(else_nodes) = else_branch {
                    self.analyze_variable_usage(else_nodes, defined, used);
                }
            }
            BlockNode::For { variable, iterator, body, .. } => {
                defined.insert(variable.clone());
                self.collect_used_variables(iterator, used);
                self.analyze_variable_usage(body, defined, used);
            }
            BlockNode::While { condition, body } => {
                self.collect_used_variables(condition, used);
                self.analyze_variable_usage(body, defined, used);
            }
            BlockNode::When { condition, body } => {
                self.collect_used_variables(condition, used);
                self.analyze_variable_usage(body, defined, used);
            }
            BlockNode::Each { iterator, body } => {
                self.collect_used_variables(iterator, used);
                self.analyze_variable_usage(body, defined, used);
            }
            BlockNode::Loop { count, body } => {
                self.collect_used_variables(count, used);
                self.analyze_variable_usage(body, defined, used);
            }
            BlockNode::RangeFor { variable, start, end, step, body } => {
                defined.insert(variable.clone());
                self.collect_used_variables(start, used);
                self.collect_used_variables(end, used);
                if let Some(step_expr) = step {
                    self.collect_used_variables(step_expr, used);
                }
                self.analyze_variable_usage(body, defined, used);
            }
            BlockNode::Match { value, cases, default_case } => {
                self.collect_used_variables(value, used);
                for case in cases {
                    self.collect_used_variables(&case.pattern, used);
                    self.analyze_variable_usage(&case.body, defined, used);
                }
                if let Some(default_nodes) = default_case {
                    self.analyze_variable_usage(default_nodes, defined, used);
                }
            }
            BlockNode::With { context, body } => {
                for expr in context.values() {
                    self.collect_used_variables(expr, used);
                }
                self.analyze_variable_usage(body, defined, used);
            }
        }
    /// Collect variables used in an expression
    fn collect_used_variables(&self, expr: &TemplateExpression, used: &mut HashSet<String>) {
        match expr {
            TemplateExpression::Variable(name) => {
                used.insert(name.clone());
            }
            TemplateExpression::PropertyAccess { object, .. } => {
                self.collect_used_variables(object, used);
            }
            TemplateExpression::IndexAccess { object, index } => {
                self.collect_used_variables(object, used);
                self.collect_used_variables(index, used);
            }
            TemplateExpression::BinaryOp { left, right, .. } => {
                self.collect_used_variables(left, used);
                self.collect_used_variables(right, used);
            }
            TemplateExpression::UnaryOp { operand, .. } => {
                self.collect_used_variables(operand, used);
            }
            TemplateExpression::FunctionCall { args, .. } => {
                for arg in args {
                    self.collect_used_variables(arg, used);
                }
            }
            TemplateExpression::MethodCall { object, args, .. } => {
                self.collect_used_variables(object, used);
                for arg in args {
                    self.collect_used_variables(arg, used);
                }
            }
            TemplateExpression::Conditional { condition, then_expr, else_expr } => {
                self.collect_used_variables(condition, used);
                self.collect_used_variables(then_expr, used);
                self.collect_used_variables(else_expr, used);
            }
            TemplateExpression::Array(elements) => {
                for element in elements {
                    self.collect_used_variables(element, used);
                }
            }
            TemplateExpression::Object(obj) => {
                for value in obj.values() {
                    self.collect_used_variables(value, used);
                }
            }
            TemplateExpression::Sus(expr) | TemplateExpression::Cap(expr) | TemplateExpression::Facts(expr) => {
                self.collect_used_variables(expr, used);
            }
            _ => {} // Literals don't use variables
        }
    }
    
    /// Remove dead code (unused variable assignments)
    fn remove_dead_code(&self, nodes: &mut Vec<TemplateNode>, unused_vars: &HashSet<String>, removed: &mut usize) {
        nodes.retain(|node| {
            match node {
                TemplateNode::Set { name, .. } => {
                    if unused_vars.contains(name) {
                        *removed += 1;
                        debug!("Removing unused variable assignment: {}", name);
                        return false;
                    }
                    true
                }
                _ => true
            }
        });
        
        // Recursively process nested nodes
        for node in nodes {
            match node {
                TemplateNode::LowkeyIf { then_branch, else_branch, .. } => {
                    self.remove_dead_code(then_branch, unused_vars, removed);
                    if let Some(else_nodes) = else_branch {
                        self.remove_dead_code(else_nodes, unused_vars, removed);
                    }
                }
                TemplateNode::StanLoop { body, .. } => {
                    self.remove_dead_code(body, unused_vars, removed);
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.remove_dead_code(content, unused_vars, removed);
                }
                TemplateNode::Block { block, .. } => {
                    self.remove_dead_code_from_block(block, unused_vars, removed);
                }
                _ => {}
            }
        }
    /// Remove dead code from block nodes
    fn remove_dead_code_from_block(&self, block: &mut BlockNode, unused_vars: &HashSet<String>, removed: &mut usize) {
        match block {
            BlockNode::If { then_branch, else_branch, .. } => {
                self.remove_dead_code(then_branch, unused_vars, removed);
                if let Some(else_nodes) = else_branch {
                    self.remove_dead_code(else_nodes, unused_vars, removed);
                }
            }
            BlockNode::For { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
            BlockNode::While { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
            BlockNode::When { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
            BlockNode::Each { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
            BlockNode::Loop { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
            BlockNode::RangeFor { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
            BlockNode::Match { cases, default_case, .. } => {
                for case in cases {
                    self.remove_dead_code(&mut case.body, unused_vars, removed);
                }
                if let Some(default_nodes) = default_case {
                    self.remove_dead_code(default_nodes, unused_vars, removed);
                }
            }
            BlockNode::With { body, .. } => {
                self.remove_dead_code(body, unused_vars, removed);
            }
        }
    /// Remove unreachable code (after return statements, etc.)
    fn remove_unreachable_code(&self, nodes: &mut Vec<TemplateNode>, removed: &mut usize) {
        let mut found_terminating_statement = false;
        let mut indices_to_remove = Vec::new();
        
        for (i, node) in nodes.iter().enumerate() {
            if found_terminating_statement {
                indices_to_remove.push(i);
                *removed += 1;
                continue;
            match node {
                TemplateNode::Block { block_type, .. } => {
                    if block_type == "return" || block_type == "break" || block_type == "continue" {
                        found_terminating_statement = true;
                    }
                }
                _ => {}
            }
        // Remove unreachable nodes in reverse order to maintain indices
        for &i in indices_to_remove.iter().rev() {
            nodes.remove(i);
        // Recursively process nested nodes
        for node in nodes {
            if let TemplateNode::Block { content: Some(content_nodes), .. } = node {
                self.remove_unreachable_code(content_nodes, removed);
            }
        }
    /// Remove empty conditional blocks
    fn remove_empty_blocks(&self, nodes: &mut Vec<TemplateNode>, removed: &mut usize) {
        nodes.retain(|node| {
            match node {
                TemplateNode::Block { block_type, content, .. } => {
                    if block_type == "if" || block_type == "lowkey" {
                        if let Some(content_nodes) = content {
                            if content_nodes.is_empty() {
                                *removed += 1;
                                debug!("Removing empty conditional block");
                                return false;
                            }
                        }
                    }
                    true
                }
                _ => true
            }
        });
        
        // Recursively process nested nodes
        for node in nodes {
            if let TemplateNode::Block { content: Some(content_nodes), .. } = node {
                self.remove_empty_blocks(content_nodes, removed);
            }
        }
    /// Regenerate template content from optimized AST
    fn regenerate_content_from_ast(&self, ast: &TemplateAst) -> crate::error::Result<()> {
        let mut content = String::new();
        
        for node in &ast.nodes {
            self.regenerate_node_content(node, &mut content)?;
        Ok(content)
    /// Regenerate content for a single node
    fn regenerate_node_content(&self, node: &TemplateNode, content: &mut String) -> crate::error::Result<()> {
        match node {
            TemplateNode::Text(text) => {
                content.push_str(text);
            }
            TemplateNode::Variable { expression, filters, .. } => {
                content.push_str("{{ ");
                self.regenerate_expression_content(expression, content)?;
                for filter in filters {
                    content.push_str(" | ");
                    content.push_str(&filter.name);
                    if !filter.args.is_empty() {
                        content.push('(');
                        for (i, arg) in filter.args.iter().enumerate() {
                            if i > 0 { content.push_str(", "); }
                            self.regenerate_expression_content(arg, content)?;
                        }
                        content.push(')');
                    }
                }
                content.push_str(" }}");
            }
            TemplateNode::Block { block_type, attributes, content: block_content } => {
                content.push_str("{% ");
                content.push_str(block_type);
                for (key, value) in attributes {
                    content.push(' ');
                    content.push_str(key);
                    content.push_str("=\"");
                    content.push_str(value);
                    content.push('"');
                }
                content.push_str(" %}");
                
                if let Some(content_nodes) = block_content {
                    for child_node in content_nodes {
                        self.regenerate_node_content(child_node, content)?;
                    }
                    content.push_str("{% end");
                    content.push_str(block_type);
                    content.push_str(" %}");
                }
            }
            TemplateNode::Comment { content: comment_content, .. } => {
                content.push_str("{# ");
                content.push_str(comment_content);
                content.push_str(" #}");
            }
            _ => {
                // For other node types, add basic regeneration
                content.push_str(&format!("<!-- Optimized node: {:?} -->", node));
            }
        }
        
        Ok(())
    /// Regenerate expression content
    fn regenerate_expression_content(&self, expr: &TemplateExpression, content: &mut String) -> crate::error::Result<()> {
        match expr {
            TemplateExpression::String(s) => {
                content.push('"');
                content.push_str(s);
                content.push('"');
            }
            _ => content.push_str("/* complex expression */"),
        }
        Ok(())
    }
}

impl TemplateOptimizer for DeadCodeEliminationOptimizer {
    fn optimize(&self, content: &mut String, ast: &mut TemplateAst) -> crate::error::Result<OptimizationResult> {
        let original_size = content.len();
        let mut optimizations_applied = 0;
        let mut warnings = Vec::new();
        
        // Analyze variables and their usage
        let mut defined_vars = HashSet::new();
        let mut used_vars = HashSet::new();
        
        // First pass: collect all defined and used variables
        self.analyze_variable_usage(&ast.nodes, &mut defined_vars, &mut used_vars);
        
        // Find unused variables
        let unused_vars: HashSet<_> = defined_vars.difference(&used_vars).cloned().collect();
        
        if !unused_vars.is_empty() {
            debug!("Found {} unused variables: {:?}", unused_vars.len(), unused_vars);
            warnings.push(format!("Found {} unused variables that could be optimized", unused_vars.len()));
        // Second pass: remove dead code
        let mut removed_nodes = 0;
        self.remove_dead_code(&mut ast.nodes, &unused_vars, &mut removed_nodes);
        optimizations_applied += removed_nodes;
        
        // Third pass: remove unreachable code after returns/breaks
        let mut unreachable_removed = 0;
        self.remove_unreachable_code(&mut ast.nodes, &mut unreachable_removed);
        optimizations_applied += unreachable_removed;
        
        // Fourth pass: remove empty conditional blocks
        let mut empty_blocks_removed = 0;
        self.remove_empty_blocks(&mut ast.nodes, &mut empty_blocks_removed);
        optimizations_applied += empty_blocks_removed;
        
        // Regenerate content from optimized AST
        if optimizations_applied > 0 {
            *content = self.regenerate_content_from_ast(ast)?;
        let bytes_saved = original_size.saturating_sub(content.len());
        
        Ok(OptimizationResult {
        })
    fn optimizer_type(&self) -> OptimizerType {
        OptimizerType::DeadCodeElimination
    fn name(&self) -> &str {
        "DeadCodeEliminationOptimizer"
    }
}

/// Dependency optimization optimizer
pub struct DependencyOptimizer {
    /// Inline small dependencies
    /// Maximum size for inlining
impl DependencyOptimizer {
    pub fn new() -> Self {
        Self {
            inline_threshold: 1024, // 1KB
        }
    }
    
    /// Find templates that can be inlined
    fn find_inlinable_includes(&self, nodes: &[TemplateNode], includes: &mut Vec<String>) {
        if !self.inline_small_dependencies {
            return;
        for node in nodes {
            match node {
                TemplateNode::Include { template_name, .. } => {
                    // Check if this template should be inlined
                    if self.should_inline_template(template_name) {
                        includes.push(template_name.clone());
                    }
                }
                TemplateNode::LowkeyIf { then_branch, else_branch, .. } => {
                    self.find_inlinable_includes(then_branch, includes);
                    if let Some(else_nodes) = else_branch {
                        self.find_inlinable_includes(else_nodes, includes);
                    }
                }
                TemplateNode::StanLoop { body, .. } => {
                    self.find_inlinable_includes(body, includes);
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.find_inlinable_includes(content, includes);
                }
                TemplateNode::Block { block, .. } => {
                    self.find_inlinable_includes_in_block(block, includes);
                }
                _ => {}
            }
        }
    /// Find inlinable includes in block nodes
    fn find_inlinable_includes_in_block(&self, block: &BlockNode, includes: &mut Vec<String>) {
        match block {
            BlockNode::If { then_branch, else_branch, .. } => {
                self.find_inlinable_includes(then_branch, includes);
                if let Some(else_nodes) = else_branch {
                    self.find_inlinable_includes(else_nodes, includes);
                }
            }
            BlockNode::For { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
            BlockNode::While { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
            BlockNode::When { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
            BlockNode::Each { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
            BlockNode::Loop { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
            BlockNode::RangeFor { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
            BlockNode::Match { cases, default_case, .. } => {
                for case in cases {
                    self.find_inlinable_includes(&case.body, includes);
                }
                if let Some(default_nodes) = default_case {
                    self.find_inlinable_includes(default_nodes, includes);
                }
            }
            BlockNode::With { body, .. } => {
                self.find_inlinable_includes(body, includes);
            }
        }
    /// Check if a template should be inlined based on size
    fn should_inline_template(&self, template_name: &str) -> bool {
        // Simple heuristic: inline templates with certain naming patterns
        // In a real implementation, you'd check the actual file size
        template_name.ends_with("_snippet.html") ||
        template_name.ends_with("_small.html") ||
        template_name.starts_with("inline_") ||
        template_name.len() < 20 // Very simple templates
    /// Inline a template by replacing includes with actual content
    fn inline_template(&self, ast: &mut TemplateAst, template_name: &str) -> crate::error::Result<()> {
        let mut bytes_saved = 0;
        
        // For this implementation, we'll simulate template inlining
        // In a real system, you'd load the template content and parse it
        let inlined_content = format!("<!-- Inlined content from {} -->", template_name);
        let inlined_node = TemplateNode::Text(inlined_content.clone());
        
        self.replace_includes_with_content(&mut ast.nodes, template_name, &inlined_node, &mut bytes_saved);
        
        Ok(bytes_saved)
    /// Replace include nodes with actual inlined content
    fn replace_includes_with_content(
        bytes_saved: &mut usize
    ) {
        for node in nodes.iter_mut() {
            match node {
                TemplateNode::Include { template_name: include_name, .. } => {
                    if include_name == template_name {
                        // Estimate bytes saved (original include syntax vs inlined content)
                        *bytes_saved += format!("{{%% include \"{}\" %%}}", template_name).len();
                        *node = replacement.clone();
                    }
                }
                TemplateNode::LowkeyIf { then_branch, else_branch, .. } => {
                    self.replace_includes_with_content(then_branch, template_name, replacement, bytes_saved);
                    if let Some(else_nodes) = else_branch {
                        self.replace_includes_with_content(else_nodes, template_name, replacement, bytes_saved);
                    }
                }
                TemplateNode::StanLoop { body, .. } => {
                    self.replace_includes_with_content(body, template_name, replacement, bytes_saved);
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.replace_includes_with_content(content, template_name, replacement, bytes_saved);
                }
                _ => {}
            }
        }
    /// Optimize include chains by flattening nested includes
    fn optimize_include_chains(&self, nodes: &mut Vec<TemplateNode>, optimized: &mut usize) {
        // This is a simplified implementation
        // In practice, you'd detect patterns like:
        // {% include "header" %} followed by {% include "nav" %}
        // and optimize them into a single optimized include
        
        let mut i = 0;
        while i < nodes.len().saturating_sub(1) {
            if let (
                TemplateNode::Include { template_name: name2, .. }
            ) = (&nodes[i], &nodes[i + 1]) {
                // Check if these includes can be combined
                if self.can_combine_includes(name1, name2) {
                    // Create a combined include
                    let combined_name = format!("{}_{}_combined", name1, name2);
                    let combined_include = TemplateNode::Include {
                    
                    // Replace both includes with the combined one
                    nodes[i] = combined_include;
                    nodes.remove(i + 1);
                    *optimized += 1;
                    
                    debug!("Combined includes: {} + {} -> combined", name1, name2);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        
        // Recursively optimize nested nodes
        for node in nodes {
            match node {
                TemplateNode::LowkeyIf { then_branch, else_branch, .. } => {
                    self.optimize_include_chains(then_branch, optimized);
                    if let Some(else_nodes) = else_branch {
                        self.optimize_include_chains(else_nodes, optimized);
                    }
                }
                TemplateNode::StanLoop { body, .. } => {
                    self.optimize_include_chains(body, optimized);
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.optimize_include_chains(content, optimized);
                }
                _ => {}
            }
        }
    /// Check if two includes can be combined
    fn can_combine_includes(&self, name1: &str, name2: &str) -> bool {
        // Simple heuristic: combine includes with similar prefixes
        (name1.starts_with("header") && name2.starts_with("nav")) ||
        (name1.starts_with("footer") && name2.starts_with("script")) ||
        (name1.ends_with("_part1") && name2.ends_with("_part2"))
    /// Remove duplicate includes
    fn remove_duplicate_includes(&self, nodes: &mut Vec<TemplateNode>, removed: &mut usize) {
        let mut seen_includes = HashSet::new();
        let mut indices_to_remove = Vec::new();
        
        for (i, node) in nodes.iter().enumerate() {
            if let TemplateNode::Include { template_name, .. } = node {
                if seen_includes.contains(template_name) {
                    indices_to_remove.push(i);
                    *removed += 1;
                    debug!("Removing duplicate include: {}", template_name);
                } else {
                    seen_includes.insert(template_name.clone());
                }
            }
        // Remove duplicates in reverse order to maintain indices
        for &i in indices_to_remove.iter().rev() {
            nodes.remove(i);
        // Recursively process nested nodes
        for node in nodes {
            match node {
                TemplateNode::LowkeyIf { then_branch, else_branch, .. } => {
                    self.remove_duplicate_includes(then_branch, removed);
                    if let Some(else_nodes) = else_branch {
                        self.remove_duplicate_includes(else_nodes, removed);
                    }
                }
                TemplateNode::StanLoop { body, .. } => {
                    self.remove_duplicate_includes(body, removed);
                }
                TemplateNode::BlockDef { content, .. } => {
                    self.remove_duplicate_includes(content, removed);
                }
                _ => {}
            }
        }
    /// Regenerate optimized content from AST
    fn regenerate_optimized_content(&self, ast: &TemplateAst) -> crate::error::Result<()> {
        let mut content = String::new();
        
        for node in &ast.nodes {
            self.regenerate_optimized_node_content(node, &mut content)?;
        Ok(content)
    /// Regenerate content for a single optimized node
    fn regenerate_optimized_node_content(&self, node: &TemplateNode, content: &mut String) -> crate::error::Result<()> {
        match node {
            TemplateNode::Text(text) => {
                content.push_str(text);
            }
            TemplateNode::Include { template_name, .. } => {
                content.push_str(&format!("{{%% include \"{}\" %%}}", template_name));
            }
            TemplateNode::Variable { expression, .. } => {
                content.push_str("{{ ");
                // Simplified variable regeneration
                match expression {
                    _ => content.push_str("/* optimized expression */"),
                }
                content.push_str(" }}");
            }
            TemplateNode::Comment { content: comment_content, .. } => {
                content.push_str(&format!("{{# {} #}}", comment_content));
            }
            _ => {
                // For other node types, add minimal regeneration
                    std::mem::discriminant(node)));
            }
        }
        
        Ok(())
    }
}

impl TemplateOptimizer for DependencyOptimizer {
    fn optimize(&self, content: &mut String, ast: &mut TemplateAst) -> crate::error::Result<OptimizationResult> {
        let original_size = content.len();
        let mut optimizations_applied = 0;
        let mut warnings = Vec::new();
        let mut bytes_saved = 0;
        
        // Find templates to inline
        let mut includes_to_inline = Vec::new();
        self.find_inlinable_includes(&ast.nodes, &mut includes_to_inline);
        
        if !includes_to_inline.is_empty() {
            debug!("Found {} includes that can be inlined", includes_to_inline.len());
            
            // Inline small templates
            for include_path in &includes_to_inline {
                match self.inline_template(ast, include_path) {
                    Ok(inlined_bytes) => {
                        optimizations_applied += 1;
                        bytes_saved += inlined_bytes;
                        debug!("Inlined template: {}", include_path);
                    }
                    Err(e) => {
                        warnings.push(format!("Failed to inline {}: {}", include_path, e));
                    }
                }
            }
        }
        
        // Optimize include chains
        let mut chains_optimized = 0;
        self.optimize_include_chains(&mut ast.nodes, &mut chains_optimized);
        optimizations_applied += chains_optimized;
        
        // Remove duplicate includes
        let mut duplicates_removed = 0;
        self.remove_duplicate_includes(&mut ast.nodes, &mut duplicates_removed);
        optimizations_applied += duplicates_removed;
        
        // Regenerate content if optimizations were applied
        if optimizations_applied > 0 {
            *content = self.regenerate_optimized_content(ast)?;
            bytes_saved += original_size.saturating_sub(content.len());
        Ok(OptimizationResult {
        })
    fn optimizer_type(&self) -> OptimizerType {
        OptimizerType::DependencyOptimization
    fn name(&self) -> &str {
        "DependencyOptimizer"
    }
}

