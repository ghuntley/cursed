//! LLVM pass pipeline management and execution

use crate::error::{Result, CursedError};
use crate::optimization::{OptimizationConfig};
use crate::common::optimization_level::OptimizationLevel;
use crate::optimization::metrics::CompilationUnit;

use std::collections::HashMap;
use tracing::{info, debug, warn, instrument};

/// LLVM pass manager for organizing and executing optimization passes
#[derive(Debug)]
pub struct LlvmPassManager {
    config: OptimizationConfig,
    pass_registry: HashMap<String, PassInfo>,
    execution_order: Vec<String>,
}

/// Information about an optimization pass
#[derive(Debug, Clone)]
pub struct PassInfo {
    pub name: String,
    pub description: String,
    pub optimization_level: OptimizationLevel,
    pub estimated_time_cost: f64,
    pub size_impact: SizeImpact,
    pub dependencies: Vec<String>,
}

/// Impact of a pass on code size
#[derive(Debug, Clone, PartialEq)]
pub enum SizeImpact {
    Reduces,    // Pass reduces code size
    Neutral,    // Pass doesn't significantly affect size
    Increases,  // Pass may increase code size
    Variable,   // Pass impact depends on input
}

impl LlvmPassManager {
    /// Create a new pass manager with the given configuration
    #[instrument]
    pub fn new(config: &OptimizationConfig) -> Result<Self> {
        info!("Creating LLVM pass manager for optimization level {:?}", config.optimization_level);
        
        let mut manager = Self {
            config: config.clone(),
            pass_registry: HashMap::new(),
            execution_order: Vec::new(),
        };

        manager.initialize_pass_registry()?;
        manager.build_execution_order()?;

        Ok(manager)
    }

    /// Initialize the registry of available optimization passes
    fn initialize_pass_registry(&mut self) -> Result<()> {
        debug!("Initializing pass registry");

        // Basic optimization passes
        self.register_pass(PassInfo {
            name: "dead-code-elimination".to_string(),
            description: "Remove unreachable and unused code".to_string(),
            optimization_level: OptimizationLevel::O1,
            estimated_time_cost: 0.1,
            size_impact: SizeImpact::Reduces,
            dependencies: vec![],
        });

        self.register_pass(PassInfo {
            name: "constant-folding".to_string(),
            description: "Evaluate constant expressions at compile time".to_string(),
            optimization_level: OptimizationLevel::O1,
            estimated_time_cost: 0.05,
            size_impact: SizeImpact::Reduces,
            dependencies: vec![],
        });

        self.register_pass(PassInfo {
            name: "block-merging".to_string(),
            description: "Merge adjacent basic blocks".to_string(),
            optimization_level: OptimizationLevel::O1,
            estimated_time_cost: 0.03,
            size_impact: SizeImpact::Reduces,
            dependencies: vec![],
        });

        // Standard optimization passes
        self.register_pass(PassInfo {
            name: "function-inlining".to_string(),
            description: "Inline small functions to reduce call overhead".to_string(),
            optimization_level: OptimizationLevel::O2,
            estimated_time_cost: 0.3,
            size_impact: SizeImpact::Variable,
            dependencies: vec!["constant-folding".to_string()],
        });

        self.register_pass(PassInfo {
            name: "loop-optimizations".to_string(),
            description: "Optimize loop structures and access patterns".to_string(),
            optimization_level: OptimizationLevel::O2,
            estimated_time_cost: 0.4,
            size_impact: SizeImpact::Variable,
            dependencies: vec!["dead-code-elimination".to_string()],
        });

        self.register_pass(PassInfo {
            name: "vectorization".to_string(),
            description: "Vectorize loops and operations when possible".to_string(),
            optimization_level: OptimizationLevel::O2,
            estimated_time_cost: 0.5,
            size_impact: SizeImpact::Increases,
            dependencies: vec!["loop-optimizations".to_string()],
        });

        self.register_pass(PassInfo {
            name: "global-value-numbering".to_string(),
            description: "Eliminate redundant computations globally".to_string(),
            optimization_level: OptimizationLevel::O2,
            estimated_time_cost: 0.2,
            size_impact: SizeImpact::Reduces,
            dependencies: vec!["constant-folding".to_string()],
        });

        self.register_pass(PassInfo {
            name: "instruction-combining".to_string(),
            description: "Combine and simplify instruction sequences".to_string(),
            optimization_level: OptimizationLevel::O2,
            estimated_time_cost: 0.15,
            size_impact: SizeImpact::Reduces,
            dependencies: vec![],
        });

        // Aggressive optimization passes
        self.register_pass(PassInfo {
            name: "aggressive-inlining".to_string(),
            description: "Aggressively inline functions for performance".to_string(),
            optimization_level: OptimizationLevel::O3,
            estimated_time_cost: 0.8,
            size_impact: SizeImpact::Increases,
            dependencies: vec!["function-inlining".to_string()],
        });

        self.register_pass(PassInfo {
            name: "loop-unrolling".to_string(),
            description: "Unroll loops to reduce branching overhead".to_string(),
            optimization_level: OptimizationLevel::O3,
            estimated_time_cost: 0.6,
            size_impact: SizeImpact::Increases,
            dependencies: vec!["loop-optimizations".to_string()],
        });

        self.register_pass(PassInfo {
            name: "tail-call-optimization".to_string(),
            description: "Optimize tail calls to reduce stack usage".to_string(),
            optimization_level: OptimizationLevel::O3,
            estimated_time_cost: 0.2,
            size_impact: SizeImpact::Neutral,
            dependencies: vec!["function-inlining".to_string()],
        });

        self.register_pass(PassInfo {
            name: "interprocedural-optimizations".to_string(),
            description: "Cross-function optimizations and analysis".to_string(),
            optimization_level: OptimizationLevel::O3,
            estimated_time_cost: 1.0,
            size_impact: SizeImpact::Variable,
            dependencies: vec!["global-value-numbering".to_string()],
        });

        // Size-specific passes
        self.register_pass(PassInfo {
            name: "size-optimized-inlining".to_string(),
            description: "Conservative inlining focused on size reduction".to_string(),
            optimization_level: OptimizationLevel::Os,
            estimated_time_cost: 0.2,
            size_impact: SizeImpact::Reduces,
            dependencies: vec!["dead-code-elimination".to_string()],
        });

        self.register_pass(PassInfo {
            name: "code-deduplication".to_string(),
            description: "Remove duplicate code sequences".to_string(),
            optimization_level: OptimizationLevel::Os,
            estimated_time_cost: 0.3,
            size_impact: SizeImpact::Reduces,
            dependencies: vec![],
        });

        // Fast optimization passes
        self.register_pass(PassInfo {
            name: "fast-math-optimizations".to_string(),
            description: "Aggressive floating-point optimizations".to_string(),
            optimization_level: OptimizationLevel::Fast,
            estimated_time_cost: 0.1,
            size_impact: SizeImpact::Neutral,
            dependencies: vec![],
        });

        self.register_pass(PassInfo {
            name: "unsafe-optimizations".to_string(),
            description: "Optimizations that may break strict compliance".to_string(),
            optimization_level: OptimizationLevel::Fast,
            estimated_time_cost: 0.2,
            size_impact: SizeImpact::Variable,
            dependencies: vec!["fast-math-optimizations".to_string()],
        });

        Ok(())
    }

    /// Register a new optimization pass
    fn register_pass(&mut self, pass_info: PassInfo) {
        debug!("Registering pass: {}", pass_info.name);
        self.pass_registry.insert(pass_info.name.clone(), pass_info);
    }

    /// Build the execution order based on dependencies and optimization level
    fn build_execution_order(&mut self) -> Result<()> {
        debug!("Building pass execution order");
        
        let target_level = &self.config.optimization_level;
        let mut applicable_passes = Vec::new();

        // Collect passes applicable to current optimization level
        for (name, pass_info) in &self.pass_registry {
            if self.is_pass_applicable(pass_info, target_level) {
                applicable_passes.push(name.clone());
            }
        }

        // Sort passes by dependencies (topological sort)
        self.execution_order = self.topological_sort(applicable_passes)?;
        
        info!("Pass execution order: {:?}", self.execution_order);
        Ok(())
    }

    /// Check if a pass is applicable for the current optimization level
    fn is_pass_applicable(&self, pass_info: &PassInfo, target_level: &OptimizationLevel) -> bool {
        match target_level {
            OptimizationLevel::O0 => false,
            OptimizationLevel::O1 => {
                matches!(pass_info.optimization_level, OptimizationLevel::O1)
            }
            OptimizationLevel::O2 => {
                matches!(
                    pass_info.optimization_level,
                    OptimizationLevel::O1 | OptimizationLevel::O2
                )
            }
            OptimizationLevel::O3 => {
                matches!(
                    pass_info.optimization_level,
                    OptimizationLevel::O1 | OptimizationLevel::O2 | OptimizationLevel::O3
                )
            }
            OptimizationLevel::Os => {
                // For size optimization, include basic passes and size-specific passes
                matches!(
                    pass_info.optimization_level,
                    OptimizationLevel::O1 | OptimizationLevel::Os
                ) && pass_info.size_impact != SizeImpact::Increases
            }
            OptimizationLevel::Fast => {
                matches!(
                    pass_info.optimization_level,
                    OptimizationLevel::O1 | OptimizationLevel::O2 | OptimizationLevel::Fast
                )
            }
        }
    }

    /// Perform topological sort of passes based on dependencies
    fn topological_sort(&self, passes: Vec<String>) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();

        for pass_name in &passes {
            if !visited.contains(pass_name) {
                self.visit_pass(pass_name, &mut result, &mut visited, &mut visiting)?;
            }
        }

        Ok(result)
    }

    /// Visit a pass during topological sort (DFS)
    fn visit_pass(
        &self,
        pass_name: &str,
        result: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        visiting: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        if visiting.contains(pass_name) {
            return Err(CursedError::optimization_error(&format!(
                "Circular dependency detected in pass: {}", pass_name
            )));
        }

        if visited.contains(pass_name) {
            return Ok(());
        }

        visiting.insert(pass_name.to_string());

        if let Some(pass_info) = self.pass_registry.get(pass_name) {
            for dependency in &pass_info.dependencies {
                if self.pass_registry.contains_key(dependency) {
                    self.visit_pass(dependency, result, visited, visiting)?;
                }
            }
        }

        visiting.remove(pass_name);
        visited.insert(pass_name.to_string());
        result.push(pass_name.to_string());

        Ok(())
    }

    /// Apply dead code elimination pass
    #[instrument(skip(self, unit))]
    pub fn apply_dead_code_elimination(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying dead code elimination to {}", unit.name);
        
        // Simulate dead code elimination by reducing estimated size
        let reduction = (unit.estimated_size_bytes as f64 * 0.05) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(10)); // Simulate work
        Ok(())
    }

    /// Apply constant folding pass
    #[instrument(skip(self, unit))]
    pub fn apply_constant_folding(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying constant folding to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.02) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(5));
        Ok(())
    }

    /// Apply block merging pass
    #[instrument(skip(self, unit))]
    pub fn apply_block_merging(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying block merging to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.01) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(3));
        Ok(())
    }

    /// Apply function inlining pass
    #[instrument(skip(self, unit))]
    pub fn apply_function_inlining(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying function inlining to {}", unit.name);
        
        // Inlining may increase or decrease size depending on function sizes
        let change = (unit.estimated_size_bytes as f64 * 0.1) as usize;
        if unit.estimated_size_bytes > 10000 {
            unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(change);
        } else {
            unit.estimated_size_bytes += change;
        }
        
        std::thread::sleep(std::time::Duration::from_millis(30));
        Ok(())
    }

    /// Apply loop optimizations pass
    #[instrument(skip(self, unit))]
    pub fn apply_loop_optimizations(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying loop optimizations to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.08) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(40));
        Ok(())
    }

    /// Apply vectorization pass
    #[instrument(skip(self, unit))]
    pub fn apply_vectorization(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying vectorization to {}", unit.name);
        
        // Vectorization typically increases code size but improves performance
        let increase = (unit.estimated_size_bytes as f64 * 0.15) as usize;
        unit.estimated_size_bytes += increase;
        
        std::thread::sleep(std::time::Duration::from_millis(50));
        Ok(())
    }

    /// Apply global value numbering pass
    #[instrument(skip(self, unit))]
    pub fn apply_global_value_numbering(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying global value numbering to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.06) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(20));
        Ok(())
    }

    /// Apply instruction combining pass
    #[instrument(skip(self, unit))]
    pub fn apply_instruction_combining(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying instruction combining to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.04) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(15));
        Ok(())
    }

    /// Apply aggressive inlining pass
    #[instrument(skip(self, unit))]
    pub fn apply_aggressive_inlining(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying aggressive inlining to {}", unit.name);
        
        // Aggressive inlining typically increases size significantly
        let increase = (unit.estimated_size_bytes as f64 * 0.25) as usize;
        unit.estimated_size_bytes += increase;
        
        std::thread::sleep(std::time::Duration::from_millis(80));
        Ok(())
    }

    /// Apply loop unrolling pass
    #[instrument(skip(self, unit))]
    pub fn apply_loop_unrolling(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying loop unrolling to {}", unit.name);
        
        let increase = (unit.estimated_size_bytes as f64 * 0.2) as usize;
        unit.estimated_size_bytes += increase;
        
        std::thread::sleep(std::time::Duration::from_millis(60));
        Ok(())
    }

    /// Apply tail call optimization pass
    #[instrument(skip(self, unit))]
    pub fn apply_tail_call_optimization(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying tail call optimization to {}", unit.name);
        
        // Tail call optimization is typically size-neutral
        std::thread::sleep(std::time::Duration::from_millis(20));
        Ok(())
    }

    /// Apply interprocedural optimizations pass
    #[instrument(skip(self, unit))]
    pub fn apply_interprocedural_optimizations(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying interprocedural optimizations to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.12) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Apply size-optimized inlining pass
    #[instrument(skip(self, unit))]
    pub fn apply_size_optimized_inlining(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying size-optimized inlining to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.03) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(20));
        Ok(())
    }

    /// Apply code deduplication pass
    #[instrument(skip(self, unit))]
    pub fn apply_code_deduplication(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying code deduplication to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.07) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(30));
        Ok(())
    }

    /// Apply fast math optimizations pass
    #[instrument(skip(self, unit))]
    pub fn apply_fast_math_optimizations(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying fast math optimizations to {}", unit.name);
        std::thread::sleep(std::time::Duration::from_millis(10));
        Ok(())
    }

    /// Apply unsafe optimizations pass
    #[instrument(skip(self, unit))]
    pub fn apply_unsafe_optimizations(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying unsafe optimizations to {}", unit.name);
        
        let reduction = (unit.estimated_size_bytes as f64 * 0.05) as usize;
        unit.estimated_size_bytes = unit.estimated_size_bytes.saturating_sub(reduction);
        
        std::thread::sleep(std::time::Duration::from_millis(20));
        Ok(())
    }

    /// Apply CPU-specific optimizations
    #[instrument(skip(self, unit))]
    pub fn apply_cpu_specific_optimizations(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying CPU-specific optimizations to {}", unit.name);
        std::thread::sleep(std::time::Duration::from_millis(15));
        Ok(())
    }

    /// Apply architecture-specific optimizations
    #[instrument(skip(self, unit))]
    pub fn apply_architecture_optimizations(&self, unit: &mut CompilationUnit) -> Result<()> {
        debug!("Applying architecture optimizations to {}", unit.name);
        std::thread::sleep(std::time::Duration::from_millis(25));
        Ok(())
    }

    /// Apply a custom optimization pass
    #[instrument(skip(self, unit))]
    pub fn apply_custom_pass(&self, unit: &mut CompilationUnit, pass_name: &str) -> Result<()> {
        debug!("Applying custom pass {} to {}", pass_name, unit.name);
        
        // Simulate custom pass execution
        std::thread::sleep(std::time::Duration::from_millis(10));
        Ok(())
    }

    /// Update pass manager configuration
    pub fn update_config(&mut self, config: &OptimizationConfig) -> Result<()> {
        info!("Updating pass manager configuration");
        self.config = config.clone();
        self.build_execution_order()?;
        Ok(())
    }

    /// Get information about all registered passes
    pub fn get_pass_info(&self) -> &HashMap<String, PassInfo> {
        &self.pass_registry
    }

    /// Get the current execution order
    pub fn get_execution_order(&self) -> &[String] {
        &self.execution_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::OptimizationConfig;

    #[test]
    fn test_pass_manager_creation() {
        let config = OptimizationConfig::default();
        let manager = LlvmPassManager::new(&config);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_pass_registration() {
        let config = OptimizationConfig::default();
        let manager = LlvmPassManager::new(&config).unwrap();
        
        let pass_info = manager.get_pass_info();
        assert!(pass_info.contains_key("dead-code-elimination"));
        assert!(pass_info.contains_key("constant-folding"));
        assert!(pass_info.contains_key("function-inlining"));
    }

    #[test]
    fn test_execution_order_dependencies() {
        let config = OptimizationConfig::default();
        let manager = LlvmPassManager::new(&config).unwrap();
        
        let execution_order = manager.get_execution_order();
        
        // Verify that function-inlining comes after constant-folding
        let folding_pos = execution_order.iter().position(|p| p == "constant-folding");
        let inlining_pos = execution_order.iter().position(|p| p == "function-inlining");
        
        if let (Some(folding), Some(inlining)) = (folding_pos, inlining_pos) {
            assert!(folding < inlining, "constant-folding should come before function-inlining");
        }
    }
}
