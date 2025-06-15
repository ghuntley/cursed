//! Profile-guided optimization (PGO) support

use crate::error::{Result, CursedError};
use crate::optimization::metrics::CompilationUnit;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{info, debug, warn, instrument};
use serde::{Deserialize, Serialize};

/// Profile data collected during program execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub function_counts: HashMap<String, u64>,
    pub basic_block_counts: HashMap<String, u64>,
    pub edge_counts: HashMap<String, u64>,
    pub total_execution_time: Duration,
    pub hot_functions: Vec<HotFunction>,
    pub cold_functions: Vec<String>,
}

/// Information about frequently executed functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotFunction {
    pub name: String,
    pub execution_count: u64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub optimization_priority: OptimizationPriority,
}

/// Priority levels for optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// PGO optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PgoStrategy {
    /// Optimize for overall execution speed
    Speed,
    /// Optimize for code size
    Size,
    /// Balance between speed and size
    Balanced,
    /// Custom optimization with specific weights
    Custom {
        speed_weight: f64,
        size_weight: f64,
        compilation_time_weight: f64,
    },
}

impl Default for PgoStrategy {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Configuration for profile-guided optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgoConfig {
    pub enable_pgo: bool,
    pub profile_data_path: PathBuf,
    pub optimization_strategy: PgoStrategy,
    pub hot_function_threshold: f64,
    pub cold_function_threshold: f64,
    pub max_inline_depth: usize,
    pub enable_speculative_optimization: bool,
}

impl Default for PgoConfig {
    fn default() -> Self {
        Self {
            enable_pgo: false,
            profile_data_path: PathBuf::from("profile.data"),
            optimization_strategy: PgoStrategy::default(),
            hot_function_threshold: 0.1, // 10% of total execution time
            cold_function_threshold: 0.01, // 1% of total execution time
            max_inline_depth: 5,
            enable_speculative_optimization: true,
        }
    }
}

/// Profile-guided optimization manager
#[derive(Debug)]
pub struct ProfileGuidedOptimizer {
    config: PgoConfig,
    profile_data: Option<ProfileData>,
    optimization_decisions: HashMap<String, OptimizationDecision>,
}

/// Optimization decision for a function or compilation unit
#[derive(Debug, Clone)]
pub struct OptimizationDecision {
    pub target: String,
    pub should_inline: bool,
    pub should_vectorize: bool,
    pub should_unroll_loops: bool,
    pub optimization_level: OptimizationPriority,
    pub estimated_speedup: f64,
    pub estimated_size_change: f64,
}

/// Results from applying PGO optimizations
#[derive(Debug, Clone)]
pub struct PgoOptimizationResult {
    pub units_optimized: usize,
    pub hot_functions_optimized: usize,
    pub cold_functions_optimized: usize,
    pub estimated_performance_gain: f64,
    pub estimated_size_change: f64,
    pub optimization_decisions: Vec<OptimizationDecision>,
}

impl ProfileGuidedOptimizer {
    /// Create a new profile-guided optimizer
    #[instrument]
    pub fn new(config: PgoConfig) -> Result<Self> {
        info!("Creating profile-guided optimizer");
        
        let profile_data = if config.enable_pgo && config.profile_data_path.exists() {
            Some(Self::load_profile_data(&config.profile_data_path)?)
        } else {
            None
        };

        Ok(Self {
            config,
            profile_data,
            optimization_decisions: HashMap::new(),
        })
    }

    /// Load profile data from file
    #[instrument]
    fn load_profile_data(path: &Path) -> Result<ProfileData> {
        info!("Loading profile data from: {:?}", path);
        
        let data = std::fs::read_to_string(path).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to read profile data: {}", e))
        })?;

        let profile_data: ProfileData = serde_json::from_str(&data).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to parse profile data: {}", e))
        })?;

        info!(
            "Loaded profile data: {} hot functions, {} cold functions",
            profile_data.hot_functions.len(),
            profile_data.cold_functions.len()
        );

        Ok(profile_data)
    }

    /// Apply profile-guided optimizations to compilation units
    #[instrument(skip(self, units))]
    pub fn apply_pgo_optimizations(&mut self, units: &mut [CompilationUnit]) -> Result<PgoOptimizationResult> {
        if !self.config.enable_pgo {
            return Ok(PgoOptimizationResult {
                units_optimized: 0,
                hot_functions_optimized: 0,
                cold_functions_optimized: 0,
                estimated_performance_gain: 0.0,
                estimated_size_change: 0.0,
                optimization_decisions: Vec::new(),
            });
        }

        let profile_data = match &self.profile_data {
            Some(data) => data,
            None => {
                warn!("No profile data available for PGO");
                return Ok(PgoOptimizationResult {
                    units_optimized: 0,
                    hot_functions_optimized: 0,
                    cold_functions_optimized: 0,
                    estimated_performance_gain: 0.0,
                    estimated_size_change: 0.0,
                    optimization_decisions: Vec::new(),
                });
            }
        };

        info!("Applying PGO optimizations to {} compilation units", units.len());

        let mut units_optimized = 0;
        let mut hot_functions_optimized = 0;
        let mut cold_functions_optimized = 0;
        let mut total_estimated_speedup = 0.0;
        let mut total_estimated_size_change = 0.0;
        let mut all_decisions = Vec::new();

        for unit in units.iter_mut() {
            let decisions = self.analyze_unit_for_pgo(unit, profile_data)?;
            
            if !decisions.is_empty() {
                units_optimized += 1;
                
                for decision in &decisions {
                    // Apply optimization decisions
                    self.apply_optimization_decision(unit, decision)?;
                    
                    // Track statistics
                    match decision.optimization_level {
                        OptimizationPriority::High | OptimizationPriority::Critical => {
                            hot_functions_optimized += 1;
                        }
                        OptimizationPriority::Low => {
                            cold_functions_optimized += 1;
                        }
                        _ => {}
                    }
                    
                    total_estimated_speedup += decision.estimated_speedup;
                    total_estimated_size_change += decision.estimated_size_change;
                }
                
                all_decisions.extend(decisions);
            }
        }

        let result = PgoOptimizationResult {
            units_optimized,
            hot_functions_optimized,
            cold_functions_optimized,
            estimated_performance_gain: total_estimated_speedup / units.len() as f64,
            estimated_size_change: total_estimated_size_change,
            optimization_decisions: all_decisions,
        };

        info!(
            "PGO optimization complete: {} units optimized, {:.2}% estimated performance gain",
            result.units_optimized,
            result.estimated_performance_gain * 100.0
        );

        Ok(result)
    }

    /// Analyze a compilation unit for PGO opportunities
    #[instrument(skip(self, unit, profile_data))]
    fn analyze_unit_for_pgo(
        &mut self,
        unit: &CompilationUnit,
        profile_data: &ProfileData,
    ) -> Result<Vec<OptimizationDecision>> {
        let mut decisions = Vec::new();

        // Check if this unit contains hot functions
        for hot_function in &profile_data.hot_functions {
            if self.unit_contains_function(unit, &hot_function.name) {
                let decision = self.create_hot_function_decision(hot_function)?;
                decisions.push(decision);
            }
        }

        // Check if this unit contains cold functions
        for cold_function in &profile_data.cold_functions {
            if self.unit_contains_function(unit, cold_function) {
                let decision = self.create_cold_function_decision(cold_function)?;
                decisions.push(decision);
            }
        }

        // Apply unit-level optimizations based on overall profile
        if let Some(unit_decision) = self.create_unit_level_decision(unit, profile_data)? {
            decisions.push(unit_decision);
        }

        Ok(decisions)
    }

    /// Check if a compilation unit contains a specific function
    fn unit_contains_function(&self, unit: &CompilationUnit, function_name: &str) -> bool {
        // Simplified check - in real implementation would parse source files
        unit.name.contains(function_name) || 
        unit.source_files.iter().any(|file| file.contains(function_name))
    }

    /// Create optimization decision for a hot function
    fn create_hot_function_decision(&self, hot_function: &HotFunction) -> Result<OptimizationDecision> {
        let priority = hot_function.optimization_priority.clone();
        
        let (should_inline, should_vectorize, should_unroll_loops) = match &self.config.optimization_strategy {
            PgoStrategy::Speed => (true, true, true),
            PgoStrategy::Size => (false, false, false),
            PgoStrategy::Balanced => (
                hot_function.execution_count > 1000,
                hot_function.execution_count > 5000,
                hot_function.execution_count > 10000,
            ),
            PgoStrategy::Custom { speed_weight, size_weight, .. } => {
                let speed_bias = speed_weight > size_weight;
                (speed_bias, speed_bias, speed_bias && hot_function.execution_count > 1000)
            }
        };

        let estimated_speedup = self.calculate_estimated_speedup(hot_function, should_inline, should_vectorize, should_unroll_loops);
        let estimated_size_change = self.calculate_estimated_size_change(should_inline, should_vectorize, should_unroll_loops);

        Ok(OptimizationDecision {
            target: hot_function.name.clone(),
            should_inline,
            should_vectorize,
            should_unroll_loops,
            optimization_level: priority,
            estimated_speedup,
            estimated_size_change,
        })
    }

    /// Create optimization decision for a cold function
    fn create_cold_function_decision(&self, cold_function: &str) -> Result<OptimizationDecision> {
        // Cold functions should be optimized for size
        Ok(OptimizationDecision {
            target: cold_function.to_string(),
            should_inline: false,
            should_vectorize: false,
            should_unroll_loops: false,
            optimization_level: OptimizationPriority::Low,
            estimated_speedup: 0.0,
            estimated_size_change: -0.1, // Small size reduction
        })
    }

    /// Create unit-level optimization decision
    fn create_unit_level_decision(
        &self,
        unit: &CompilationUnit,
        profile_data: &ProfileData,
    ) -> Result<Option<OptimizationDecision>> {
        // Determine if this unit is generally hot or cold
        let hot_function_count = profile_data.hot_functions.iter()
            .filter(|hf| self.unit_contains_function(unit, &hf.name))
            .count();

        let cold_function_count = profile_data.cold_functions.iter()
            .filter(|cf| self.unit_contains_function(unit, cf))
            .count();

        if hot_function_count > cold_function_count {
            // Unit is generally hot
            Ok(Some(OptimizationDecision {
                target: unit.name.clone(),
                should_inline: true,
                should_vectorize: true,
                should_unroll_loops: false,
                optimization_level: OptimizationPriority::Medium,
                estimated_speedup: 0.05,
                estimated_size_change: 0.1,
            }))
        } else if cold_function_count > 0 {
            // Unit is generally cold
            Ok(Some(OptimizationDecision {
                target: unit.name.clone(),
                should_inline: false,
                should_vectorize: false,
                should_unroll_loops: false,
                optimization_level: OptimizationPriority::Low,
                estimated_speedup: 0.0,
                estimated_size_change: -0.05,
            }))
        } else {
            // No profile information for this unit
            Ok(None)
        }
    }

    /// Apply an optimization decision to a compilation unit
    #[instrument(skip(self, unit, decision))]
    fn apply_optimization_decision(
        &mut self,
        unit: &mut CompilationUnit,
        decision: &OptimizationDecision,
    ) -> Result<()> {
        debug!("Applying PGO decision for {}: inline={}, vectorize={}, unroll={}", 
            decision.target, decision.should_inline, decision.should_vectorize, decision.should_unroll_loops);

        // Store decision for later reference
        self.optimization_decisions.insert(decision.target.clone(), decision.clone());

        // Apply size changes to unit
        let size_change = (unit.estimated_size_bytes as f64 * decision.estimated_size_change) as i64;
        if size_change < 0 {
            unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub((-size_change) as usize);
        } else {
            unit.estimated_size_bytes += size_change as usize;
        }

        // Simulate optimization work
        std::thread::sleep(Duration::from_millis(10));

        Ok(())
    }

    /// Calculate estimated speedup for optimization decisions
    fn calculate_estimated_speedup(
        &self,
        hot_function: &HotFunction,
        should_inline: bool,
        should_vectorize: bool,
        should_unroll_loops: bool,
    ) -> f64 {
        let base_speedup = match hot_function.optimization_priority {
            OptimizationPriority::Critical => 0.20,
            OptimizationPriority::High => 0.15,
            OptimizationPriority::Medium => 0.10,
            OptimizationPriority::Low => 0.05,
        };

        let mut speedup = base_speedup;
        
        if should_inline {
            speedup += 0.05;
        }
        if should_vectorize {
            speedup += 0.08;
        }
        if should_unroll_loops {
            speedup += 0.03;
        }

        // Factor in execution frequency
        let frequency_factor = (hot_function.execution_count as f64).log10() / 6.0; // Normalize to 0-1
        speedup * frequency_factor.min(1.0)
    }

    /// Calculate estimated size change for optimization decisions
    fn calculate_estimated_size_change(
        &self,
        should_inline: bool,
        should_vectorize: bool,
        should_unroll_loops: bool,
    ) -> f64 {
        let mut size_change = 0.0;
        
        if should_inline {
            size_change += 0.15; // Inlining increases size
        }
        if should_vectorize {
            size_change += 0.10; // Vectorization increases size
        }
        if should_unroll_loops {
            size_change += 0.25; // Loop unrolling significantly increases size
        }

        size_change
    }

    /// Generate profile data collection instrumentation
    pub fn generate_instrumentation_code(&self, unit: &CompilationUnit) -> Result<String> {
        if !self.config.enable_pgo {
            return Ok(String::new());
        }

        // Generate basic instrumentation code
        let instrumentation = format!(
            r#"
// PGO instrumentation for unit: {}
static mut PROFILE_COUNTER_{}: u64 = 0;

#[no_mangle]
pub extern "C" fn __profile_increment_{}() {{
    unsafe {{
        PROFILE_COUNTER_{} += 1;
    }}
}}

#[no_mangle]
pub extern "C" fn __profile_get_count_{}() -> u64 {{
    unsafe {{ PROFILE_COUNTER_{} }}
}}
"#,
            unit.name,
            unit.name.to_uppercase(),
            unit.name,
            unit.name.to_uppercase(),
            unit.name,
            unit.name.to_uppercase()
        );

        Ok(instrumentation)
    }

    /// Save profile data to file
    pub fn save_profile_data(&self, profile_data: &ProfileData) -> Result<()> {
        let data = serde_json::to_string_pretty(profile_data).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to serialize profile data: {}", e))
        })?;

        std::fs::write(&self.config.profile_data_path, data).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write profile data: {}", e))
        })?;

        info!("Saved profile data to: {:?}", self.config.profile_data_path);
        Ok(())
    }

    /// Get optimization decisions made
    pub fn get_optimization_decisions(&self) -> &HashMap<String, OptimizationDecision> {
        &self.optimization_decisions
    }

    /// Create sample profile data for testing
    pub fn create_sample_profile_data() -> ProfileData {
        let hot_functions = vec![
            HotFunction {
                name: "hot_function_1".to_string(),
                execution_count: 10000,
                total_time: Duration::from_millis(500),
                average_time: Duration::from_nanos(50000),
                optimization_priority: OptimizationPriority::Critical,
            },
            HotFunction {
                name: "hot_function_2".to_string(),
                execution_count: 5000,
                total_time: Duration::from_millis(200),
                average_time: Duration::from_nanos(40000),
                optimization_priority: OptimizationPriority::High,
            },
        ];

        let cold_functions = vec![
            "cold_function_1".to_string(),
            "cold_function_2".to_string(),
        ];

        ProfileData {
            function_counts: [
                ("hot_function_1".to_string(), 10000),
                ("hot_function_2".to_string(), 5000),
                ("cold_function_1".to_string(), 10),
                ("cold_function_2".to_string(), 5),
            ].iter().cloned().collect(),
            basic_block_counts: HashMap::new(),
            edge_counts: HashMap::new(),
            total_execution_time: Duration::from_millis(1000),
            hot_functions,
            cold_functions,
        }
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: PgoConfig) -> Result<()> {
        info!("Updating PGO configuration");
        self.config = new_config;
        
        // Reload profile data if path changed
        if self.config.enable_pgo && self.config.profile_data_path.exists() {
            self.profile_data = Some(Self::load_profile_data(&self.config.profile_data_path)?);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_pgo_optimizer_creation() {
        let config = PgoConfig::default();
        let optimizer = ProfileGuidedOptimizer::new(config);
        assert!(optimizer.is_ok());
    }

    #[test]
    fn test_sample_profile_data_creation() {
        let profile_data = ProfileGuidedOptimizer::create_sample_profile_data();
        assert_eq!(profile_data.hot_functions.len(), 2);
        assert_eq!(profile_data.cold_functions.len(), 2);
        assert!(profile_data.function_counts.contains_key("hot_function_1"));
    }

    #[test]
    fn test_optimization_decision_creation() {
        let config = PgoConfig::default();
        let optimizer = ProfileGuidedOptimizer::new(config).unwrap();
        
        let hot_function = HotFunction {
            name: "test_function".to_string(),
            execution_count: 1000,
            total_time: Duration::from_millis(100),
            average_time: Duration::from_nanos(100000),
            optimization_priority: OptimizationPriority::High,
        };

        let decision = optimizer.create_hot_function_decision(&hot_function);
        assert!(decision.is_ok());
        
        let decision = decision.unwrap();
        assert_eq!(decision.target, "test_function");
        assert_eq!(decision.optimization_level, OptimizationPriority::High);
    }

    #[test]
    fn test_pgo_disabled() {
        let mut config = PgoConfig::default();
        config.enable_pgo = false;
        
        let mut optimizer = ProfileGuidedOptimizer::new(config).unwrap();
        let units = vec![CompilationUnit::new("test_unit".to_string())];
        
        let result = optimizer.apply_pgo_optimizations(&mut units.clone());
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.units_optimized, 0);
    }
}
