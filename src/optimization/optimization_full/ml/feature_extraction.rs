use crate::ast::AstNodeType;
/// Feature Extraction for ML-Guided Optimization
/// 
/// Extracts relevant features from CURSED source code and compilation context
/// for ML model training and prediction.

use crate::error::{CursedError, Result};
use crate::ast::{AstNode, Expression, Statement, FunctionDeclaration};
use crate::optimization::ml::CompilationContext;

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};
use crate::ast::ASTNode;

/// Feature extractor for CURSED code
#[derive(Debug)]
pub struct FeatureExtractor {
/// Configuration for feature extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
/// Complete feature vector for ML models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureVector {
/// Syntax-level features extracted from source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxFeatures {
    // Basic counts
    
    // Control flow complexity
    
    // Code structure
/// Semantic features from AST analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFeatures {
    // Data flow
    
    // Control flow
    
    // Memory patterns
    
    // Optimization opportunities
/// Performance-related features from profiling data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceFeatures {
/// Target architecture features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetFeatures {
/// CURSED-specific language features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursedSpecificFeatures {
/// Goroutine-specific features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoroutineFeatures {
/// Channel communication features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFeatures {
/// Gen Z slang pattern features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlangFeatures {
/// Interface and type system features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceFeatures {
/// CursedError handling features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlingFeatures {
/// Type system complexity features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSystemFeatures {
/// Compilation context features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFeatures {
/// Memory access patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryAccessPattern {
/// Concurrency patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConcurrencyPattern {
/// Communication patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationPattern {
/// Memory hierarchy description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHierarchy {
/// Resource constraint features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraintFeatures {
/// Feature extraction statistics
#[derive(Debug, Default)]
pub struct ExtractionStatistics {
impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
        }
    }
impl FeatureExtractor {
    /// Create new feature extractor
    pub fn new(config: FeatureConfig) -> Result<Self> {
        Ok(Self {
        })
    /// Extract features from source code
    #[instrument(skip(self, source_code, ast, context))]
    pub fn extract_features(
    ) -> Result<FeatureVector> {
        let start_time = std::time::Instant::now();
        
        // Check cache if enabled
        if self.config.enable_caching {
            let cache_key = self.generate_cache_key(source_code, context);
            if let Some(cached_features) = self.cache.get(&cache_key) {
                self.extraction_stats.cache_hits += 1;
                return Ok(cached_features.clone());
            }
            self.extraction_stats.cache_misses += 1;
        info!("Extracting features from CURSED source code");
        
        // Extract different types of features
        let syntax_features = self.extract_syntax_features(source_code)?;
        let semantic_features = if let Some(ast) = ast {
            self.extract_semantic_features(ast)?
        } else {
            SemanticFeatures::default()
        let performance_features = self.extract_performance_features(source_code)?;
        let target_features = self.extract_target_features()?;
        let cursed_features = if self.config.enable_cursed_specific_features {
            self.extract_cursed_features(source_code, ast)?
        } else {
            CursedSpecificFeatures::default()
        let context_features = if let Some(ctx) = context {
            self.extract_context_features(ctx)?
        } else {
            ContextFeatures::default()
        
        let feature_vector = FeatureVector {
        
        // Cache the result
        if self.config.enable_caching {
            let cache_key = self.generate_cache_key(source_code, context);
            self.cache_feature_vector(cache_key, &feature_vector);
        // Update statistics
        let extraction_time = start_time.elapsed();
        self.extraction_stats.features_extracted += 1;
        self.extraction_stats.total_extraction_time += extraction_time;
        self.extraction_stats.average_extraction_time = 
            self.extraction_stats.total_extraction_time / self.extraction_stats.features_extracted as u32;
        
        debug!("Feature extraction completed in {:?}", extraction_time);
        Ok(feature_vector)
    /// Extract syntax-level features
    fn extract_syntax_features(&self, source_code: &str) -> Result<SyntaxFeatures> {
        let lines: Vec<&str> = source_code.split("\n").collect();
        let lines_of_code = lines.len();
        
        // Count different syntactic elements
        let mut token_count = 0;
        let mut function_count = 0;
        let mut variable_count = 0;
        let mut loop_count = 0;
        let mut conditional_count = 0;
        let mut switch_count = 0;
        let mut comment_lines = 0;
        let mut blank_lines = 0;
        let mut max_nesting = 0;
        let mut current_nesting = 0;
        let mut function_lengths = Vec::new();
        let mut current_function_length = 0;
        let mut in_function = false;
        
        for line in &lines {
            let trimmed = line.trim();
            
            // Count blank lines and comments
            if trimmed.is_empty() {
                blank_lines += 1;
                continue;
            }
            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                comment_lines += 1;
                continue;
            // Count tokens (simple word count)
            token_count += trimmed.split_whitespace().count();
            
            // Track function boundaries
            if trimmed.contains("slay ") || trimmed.contains("fn ") || trimmed.contains("func ") {
                function_count += 1;
                if in_function {
                    function_lengths.push(current_function_length);
                }
                current_function_length = 0;
                in_function = true;
            if in_function {
                current_function_length += 1;
            // Count control structures
            if trimmed.contains("lowkey") || trimmed.contains("if ") {
                conditional_count += 1;
            }
            if trimmed.contains("periodt") || trimmed.contains("while") || trimmed.contains("for") {
                loop_count += 1;
            }
            if trimmed.contains("vibe_check") || trimmed.contains("switch") {
                switch_count += 1;
            // Count variables (simplified)
            if trimmed.contains("sus ") || trimmed.contains("facts ") || 
               trimmed.contains("let ") || trimmed.contains("var ") {
                variable_count += 1;
            // Track nesting depth
            let open_braces = trimmed.chars().filter(|&c| c == '{').count();
            let close_braces = trimmed.chars().filter(|&c| c == '}').count();
            current_nesting += open_braces;
            max_nesting = max_nesting.max(current_nesting);
            current_nesting = current_nesting.saturating_sub(close_braces);
        // Finalize function length tracking
        if in_function {
            function_lengths.push(current_function_length);
        // Calculate averages and ratios
        let average_function_length = if function_count > 0 {
            function_lengths.iter().sum::<usize>() as f64 / function_count as f64
        } else {
            0.0
        
        let max_function_length = function_lengths.iter().max().copied().unwrap_or(0);
        let comment_ratio = if lines_of_code > 0 {
            comment_lines as f64 / lines_of_code as f64
        } else {
            0.0
        let blank_line_ratio = if lines_of_code > 0 {
            blank_lines as f64 / lines_of_code as f64
        } else {
            0.0
        
        // Calculate cyclomatic complexity (simplified)
        let cyclomatic_complexity = 1.0 + conditional_count as f64 + loop_count as f64 + switch_count as f64;
        
        Ok(SyntaxFeatures {
            expression_count: token_count / 3, // Rough estimate
        })
    /// Extract semantic features from AST
    fn extract_semantic_features(&self, ast: &ASTNode) -> Result<SemanticFeatures> {
        let mut semantic_analyzer = SemanticAnalyzer::new();
        semantic_analyzer.analyze(ast)
    /// Extract performance features from profiling data
    fn extract_performance_features(&self, source_code: &str) -> Result<PerformanceFeatures> {
        // In a real implementation, this would use profiling data
        // For now, we'll estimate based on code characteristics
        
        let function_count = source_code.matches("slay ").count() + source_code.matches("fn ").count();
        let loop_count = source_code.matches("periodt").count() + source_code.matches("while").count();
        
        // Estimate execution frequency based on code complexity
        let execution_frequency = if loop_count > 0 {
            (loop_count as f64) * 100.0 // Loops executed frequently
        } else {
            1.0
        
        Ok(PerformanceFeatures {
            cache_miss_rate: 0.05, // Default estimate
            branch_miss_rate: 0.02, // Default estimate  
            instruction_level_parallelism: 2.0, // Conservative estimate
            memory_bandwidth_utilization: 0.3, // Default estimate
            cpu_utilization: 0.7, // Default estimate
            hotspot_functions: Vec::new(), // Would be filled from profiling
        })
    /// Extract target architecture features
    fn extract_target_features(&self) -> Result<TargetFeatures> {
        let architecture = std::env::consts::ARCH.to_string();
        
        let (available_registers, vector_unit_width) = match architecture.as_str() {
        
        let cpu_features = self.detect_cpu_features();
        let cache_sizes = vec![32768, 262144, 8388608]; // L1, L2, L3
        let memory_hierarchy = MemoryHierarchy {
            main_memory_size: 8 * 1024 * 1024 * 1024, // 8GB default
        
        let mut instruction_costs = HashMap::new();
        instruction_costs.insert("add".to_string(), 0.25);
        instruction_costs.insert("mul".to_string(), 1.0);
        instruction_costs.insert("div".to_string(), 10.0);
        instruction_costs.insert("load".to_string(), 3.0);
        instruction_costs.insert("store".to_string(), 1.0);
        instruction_costs.insert("branch".to_string(), 1.0);
        
        Ok(TargetFeatures {
        })
    /// Extract CURSED-specific features
    fn extract_cursed_features(&self, source_code: &str, ast: Option<&ASTNode>) -> Result<CursedSpecificFeatures> {
        let goroutine_features = self.extract_goroutine_features(source_code)?;
        let channel_features = self.extract_channel_features(source_code)?;
        let slang_features = self.extract_slang_features(source_code)?;
        let interface_features = self.extract_interface_features(source_code)?;
        let error_handling_features = self.extract_error_handling_features(source_code)?;
        let type_system_features = self.extract_type_system_features(source_code, ast)?;
        
        Ok(CursedSpecificFeatures {
        })
    /// Extract compilation context features
    fn extract_context_features(&self, context: &CompilationContext) -> Result<ContextFeatures> {
        let optimization_level = match context.optimization_goals.first() {
        
        let resource_constraints = ResourceConstraintFeatures {
            energy_budget: None, // Not currently tracked
        
        Ok(ContextFeatures {
            compilation_flags: Vec::new(), // Would be filled from actual flags
            library_dependencies: 0, // Would be counted from imports
        })
    // Helper methods for CURSED-specific feature extraction
    
    fn extract_goroutine_features(&self, source_code: &str) -> Result<GoroutineFeatures> {
        let goroutine_spawns = source_code.matches("stan ").count();
        let goroutine_joins = source_code.matches("wait").count(); // Simplified
        
        Ok(GoroutineFeatures {
            concurrent_execution_patterns: vec![ConcurrencyPattern::WorkerPool], // Simplified
            synchronization_primitives: source_code.matches("mutex").count() + 
            stack_size_estimates: vec![65536; goroutine_spawns], // 64KB default
            communication_complexity: if goroutine_spawns > 0 {
                (goroutine_spawns as f64).sqrt()
            } else {
                0.0
        })
    fn extract_channel_features(&self, source_code: &str) -> Result<ChannelFeatures> {
        let channel_declarations = source_code.matches("channel").count() + 
                                 source_code.matches("chan ").count();
        let send_operations = source_code.matches("<-").count() / 2; // Rough estimate
        let receive_operations = source_code.matches("<-").count() / 2;
        let select_statements = source_code.matches("select").count();
        
        Ok(ChannelFeatures {
            buffered_channels: channel_declarations / 2, // Estimate
            unbuffered_channels: channel_declarations / 2,
            channel_buffer_sizes: vec![0, 1, 16], // Common sizes
            communication_patterns: vec![CommunicationPattern::OneToOne], // Simplified
        })
    fn extract_slang_features(&self, source_code: &str) -> Result<SlangFeatures> {
        let slay_functions = source_code.matches("slay ").count();
        let yolo_expressions = source_code.matches("yolo").count();
        let sus_variables = source_code.matches("sus ").count();
        let facts_declarations = source_code.matches("facts ").count();
        let periodt_statements = source_code.matches("periodt").count();
        let vibe_check_expressions = source_code.matches("vibe_check").count();
        let stan_goroutine_spawns = source_code.matches("stan ").count();
        
        let total_slang = slay_functions + yolo_expressions + sus_variables + 
                         facts_declarations + periodt_statements + vibe_check_expressions;
        let slang_complexity_score = total_slang as f64 / 10.0; // Normalize
        
        Ok(SlangFeatures {
        })
    fn extract_interface_features(&self, source_code: &str) -> Result<InterfaceFeatures> {
        let interface_declarations = source_code.matches("collab ").count() + 
                                   source_code.matches("interface").count();
        let type_assertions = source_code.matches(".(").count();
        
        Ok(InterfaceFeatures {
            interface_implementations: interface_declarations, // Simplified
            method_count_per_interface: vec![3, 5, 2], // Example data
            polymorphic_calls: interface_declarations * 2, // Estimate
            inheritance_depth: 2, // Default
        })
    fn extract_error_handling_features(&self, source_code: &str) -> Result<ErrorHandlingFeatures> {
        let question_mark_operators = source_code.matches("?").count();
        let panic_calls = source_code.matches("panic").count();
        let recover_calls = source_code.matches("recover").count();
        
        Ok(ErrorHandlingFeatures {
        })
    fn extract_type_system_features(&self, source_code: &str, _ast: Option<&ASTNode>) -> Result<TypeSystemFeatures> {
        let generic_functions = source_code.matches("<T>").count() + 
                              source_code.matches("<T,").count();
        let generic_types = source_code.matches("struct").count() / 2; // Rough estimate
        
        Ok(TypeSystemFeatures {
            constraint_complexity: 1.0, // Default
            type_inference_complexity: 1.0, // Default
        })
    // Utility methods
    
    fn detect_cpu_features(&self) -> Vec<String> {
        let mut features = Vec::new();
        
        // Detect common CPU features
        if is_x86_feature_available("sse2") {
            features.push("sse2".to_string());
        }
        if is_x86_feature_available("avx") {
            features.push("avx".to_string());
        }
        if is_x86_feature_available("avx2") {
            features.push("avx2".to_string());
        features
    fn generate_cache_key(&self, source_code: &str, context: Option<&CompilationContext>) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source_code.hash(&mut hasher);
        if let Some(ctx) = context {
            ctx.target_arch.hash(&mut hasher);
            ctx.optimization_goals.hash(&mut hasher);
        format!("features_{}", hasher.finish())
    fn cache_feature_vector(&mut self, key: String, features: &FeatureVector) {
        if self.cache.len() >= self.config.cache_size {
            // Simple LRU eviction
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        
        self.cache.insert(key, features.clone());
    /// Update configuration
    pub fn update_config(&mut self, config: FeatureConfig) -> Result<()> {
        self.config = config;
        Ok(())
    /// Get extraction statistics
    pub fn get_statistics(&self) -> &ExtractionStatistics {
        &self.extraction_stats
    /// Clear feature cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.extraction_stats.cache_hits = 0;
        self.extraction_stats.cache_misses = 0;
    }
}

/// Semantic analyzer for extracting semantic features
struct SemanticAnalyzer {
impl SemanticAnalyzer {
    fn new() -> Self {
        Self {
        }
    }
    
    fn analyze(&mut self, ast: &ASTNode) -> Result<SemanticFeatures> {
        self.visit_node(ast)?;
        
        Ok(SemanticFeatures {
            live_variable_ranges: vec![10, 20, 30], // Example data
        })
    fn visit_node(&mut self, node: &AstNode) -> Result<()> {
        match &node.node_type {
            _ => {}, // Handle other node types as needed
        }
        Ok(())
    fn visit_function(&mut self, _function: &Function) -> Result<()> {
        self.basic_block_count += 1;
        self.inlinable_functions += 1; // Simplified
        Ok(())
    fn visit_statement(&mut self, _statement: &dyn Statement) -> Result<()> {
        // Analyze statement for semantic features
        Ok(())
    fn visit_expression(&mut self, _expression: &dyn Expression) -> Result<()> {
        // Analyze expression for semantic features
        self.constant_expressions += 1; // Simplified
        Ok(())
    }
}

// Platform-specific feature detection
#[cfg(target_arch = "x86_64")]
fn is_x86_feature_available(feature: &str) -> bool {
    match feature {
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn is_x86_feature_available(_feature: &str) -> bool {
    false
// Default implementations

impl Default for FeatureVector {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SyntaxFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SemanticFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for PerformanceFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for TargetFeatures {
    fn default() -> Self {
        Self {
            memory_hierarchy: MemoryHierarchy {
        }
    }
impl Default for CursedSpecificFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for GoroutineFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ChannelFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SlangFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for InterfaceFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ErrorHandlingFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for TypeSystemFeatures {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ContextFeatures {
    fn default() -> Self {
        Self {
            resource_constraints: ResourceConstraintFeatures {
        }
    }
}
