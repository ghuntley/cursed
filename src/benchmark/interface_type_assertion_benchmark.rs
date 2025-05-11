//! # Interface Type Assertion Benchmarking
//!
//! This module provides benchmarking utilities for measuring the performance of
//! interface type assertions across various inheritance patterns and usage scenarios.
//!
//! The benchmarks cover:
//! 1. Simple interface type assertions
//! 2. Complex inheritance hierarchies
//! 3. Diamond inheritance patterns
//! 4. Performance with various caching strategies

use std::time::{Duration, Instant};
use tracing::{debug, info, instrument};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;

/// Benchmark configuration for interface type assertions
#[derive(Debug, Clone)]
pub struct InterfaceTypeAssertionBenchmarkConfig {
    /// Number of iterations to run for each benchmark
    pub iterations: usize,
    /// Whether to enable warmup iterations
    pub enable_warmup: bool,
    /// Number of warmup iterations
    pub warmup_iterations: usize,
    /// Whether to enable detailed timing breakdown
    pub detailed_timing: bool,
    /// Whether to test diamond inheritance patterns
    pub test_diamond_patterns: bool,
    /// Whether to test deep inheritance hierarchies
    pub test_deep_hierarchies: bool,
    /// Maximum depth for hierarchies
    pub max_hierarchy_depth: usize,
}

impl Default for InterfaceTypeAssertionBenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            enable_warmup: true,
            warmup_iterations: 100,
            detailed_timing: true,
            test_diamond_patterns: true,
            test_deep_hierarchies: true,
            max_hierarchy_depth: 10,
        }
    }
}

/// Results from a single type assertion benchmark
#[derive(Debug, Clone)]
pub struct TypeAssertionBenchmarkResult {
    /// Name of the benchmark
    pub name: String,
    /// Average duration per iteration
    pub avg_duration: Duration,
    /// Minimum duration observed
    pub min_duration: Duration,
    /// Maximum duration observed
    pub max_duration: Duration,
    /// Total iterations run
    pub iterations: usize,
    /// Detailed timing breakdown by phase (if enabled)
    pub phase_timing: Option<BenchmarkPhaseTiming>,
}

/// Detailed timing breakdown by benchmark phase
#[derive(Debug, Clone)]
pub struct BenchmarkPhaseTiming {
    /// Time spent on type lookup
    pub type_lookup_time: Duration,
    /// Time spent on type checking
    pub type_check_time: Duration,
    /// Time spent on error handling
    pub error_handling_time: Duration,
}

/// Interface type assertion benchmark utilities
pub trait InterfaceTypeAssertionBenchmark<'ctx> {
    /// Run a comprehensive benchmark suite for interface type assertions
    fn benchmark_interface_type_assertions(
        &mut self,
        config: InterfaceTypeAssertionBenchmarkConfig,
    ) -> Result<Vec<TypeAssertionBenchmarkResult>, Error>;
    
    /// Benchmark simple type assertions (one interface, one implementation)
    fn benchmark_simple_type_assertions(
        &mut self,
        iterations: usize,
    ) -> Result<TypeAssertionBenchmarkResult, Error>;
    
    /// Benchmark diamond inheritance pattern type assertions
    fn benchmark_diamond_inheritance_type_assertions(
        &mut self,
        iterations: usize,
    ) -> Result<TypeAssertionBenchmarkResult, Error>;
    
    /// Benchmark deep inheritance hierarchy type assertions
    fn benchmark_deep_hierarchy_type_assertions(
        &mut self,
        iterations: usize,
        depth: usize,
    ) -> Result<TypeAssertionBenchmarkResult, Error>;
}

impl<'ctx> InterfaceTypeAssertionBenchmark<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, config), level = "debug")]
    fn benchmark_interface_type_assertions(
        &mut self,
        config: InterfaceTypeAssertionBenchmarkConfig,
    ) -> Result<Vec<TypeAssertionBenchmarkResult>, Error> {
        let mut results = Vec::new();
        
        // Perform warmup if enabled
        if config.enable_warmup {
            info!("Running {} warmup iterations", config.warmup_iterations);
            let _ = self.benchmark_simple_type_assertions(config.warmup_iterations)?;
        }
        
        // Run simple type assertion benchmark
        let simple_result = self.benchmark_simple_type_assertions(config.iterations)?;
        results.push(simple_result);
        
        // Run diamond inheritance benchmark if enabled
        if config.test_diamond_patterns {
            let diamond_result = self.benchmark_diamond_inheritance_type_assertions(config.iterations)?;
            results.push(diamond_result);
        }
        
        // Run deep hierarchy benchmark if enabled
        if config.test_deep_hierarchies {
            let deep_result = self.benchmark_deep_hierarchy_type_assertions(
                config.iterations,
                config.max_hierarchy_depth,
            )?;
            results.push(deep_result);
        }
        
        // Log benchmark summary
        info!("Completed {} interface type assertion benchmarks", results.len());
        for result in &results {
            info!(
                "Benchmark '{}': avg={}ms, min={}ms, max={}ms (iterations={})",
                result.name,
                result.avg_duration.as_secs_f64() * 1000.0,
                result.min_duration.as_secs_f64() * 1000.0,
                result.max_duration.as_secs_f64() * 1000.0,
                result.iterations
            );
        }
        
        Ok(results)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn benchmark_simple_type_assertions(
        &mut self,
        iterations: usize,
    ) -> Result<TypeAssertionBenchmarkResult, Error> {
        let mut durations = Vec::with_capacity(iterations);
        
        // Setup test types
        let interface_type_id = 1000;
        let concrete_type_id = 2000;
        
        // Mock type registry if needed for testing
        if let Some(registry) = self.get_interface_registry_mut() {
            registry.register_interface(interface_type_id, "TestInterface".to_string())?;
            registry.register_concrete_type(concrete_type_id, "TestConcrete".to_string())?;
            registry.register_implementation(concrete_type_id, interface_type_id)?;
        }
        
        // Run the benchmark
        for _ in 0..iterations {
            let start = Instant::now();
            
            // Perform a type assertion check
            let _result = self.type_implements(concrete_type_id, interface_type_id);
            
            // Record the time
            let duration = start.elapsed();
            durations.push(duration);
        }
        
        // Calculate statistics
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / durations.len() as u32;
        let min_duration = durations.iter().min().cloned().unwrap_or_default();
        let max_duration = durations.iter().max().cloned().unwrap_or_default();
        
        Ok(TypeAssertionBenchmarkResult {
            name: "Simple Type Assertions".to_string(),
            avg_duration,
            min_duration,
            max_duration,
            iterations,
            phase_timing: None, // Not implemented yet
        })
    }
    
    #[instrument(skip(self), level = "debug")]
    fn benchmark_diamond_inheritance_type_assertions(
        &mut self,
        iterations: usize,
    ) -> Result<TypeAssertionBenchmarkResult, Error> {
        let mut durations = Vec::with_capacity(iterations);
        
        // Setup diamond inheritance pattern:
        //       BaseInterface (100)
        //           /     \
        //   LeftIface (200)  RightIface (300)
        //           \     /
        //        Concrete (400)
        
        let base_interface_id = 100;
        let left_interface_id = 200;
        let right_interface_id = 300;
        let concrete_type_id = 400;
        
        // Mock type registry for testing
        if let Some(registry) = self.get_interface_registry_mut() {
            // Register interfaces and concrete type
            registry.register_interface(base_interface_id, "BaseInterface".to_string())?;
            registry.register_interface(left_interface_id, "LeftInterface".to_string())?;
            registry.register_interface(right_interface_id, "RightInterface".to_string())?;
            registry.register_concrete_type(concrete_type_id, "DiamondConcrete".to_string())?;
            
            // Register inheritance relationships
            registry.register_implementation(left_interface_id, base_interface_id)?; // Left extends Base
            registry.register_implementation(right_interface_id, base_interface_id)?; // Right extends Base
            registry.register_implementation(concrete_type_id, left_interface_id)?; // Concrete implements Left
            registry.register_implementation(concrete_type_id, right_interface_id)?; // Concrete implements Right
        }
        
        // Run the benchmark
        for _ in 0..iterations {
            let start = Instant::now();
            
            // Detect diamond inheritance
            let _result = self.detect_diamond_inheritance(concrete_type_id, base_interface_id);
            
            // Record the time
            let duration = start.elapsed();
            durations.push(duration);
        }
        
        // Calculate statistics
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / durations.len() as u32;
        let min_duration = durations.iter().min().cloned().unwrap_or_default();
        let max_duration = durations.iter().max().cloned().unwrap_or_default();
        
        Ok(TypeAssertionBenchmarkResult {
            name: "Diamond Inheritance Type Assertions".to_string(),
            avg_duration,
            min_duration,
            max_duration,
            iterations,
            phase_timing: None, // Not implemented yet
        })
    }
    
    #[instrument(skip(self), level = "debug")]
    fn benchmark_deep_hierarchy_type_assertions(
        &mut self,
        iterations: usize,
        depth: usize,
    ) -> Result<TypeAssertionBenchmarkResult, Error> {
        let mut durations = Vec::with_capacity(iterations);
        
        // Setup a deep inheritance hierarchy
        // Interface_0 <- Interface_1 <- ... <- Interface_N <- Concrete
        let mut interface_ids = Vec::with_capacity(depth);
        for i in 0..depth {
            interface_ids.push(1000 + i as u32);
        }
        let concrete_type_id = 2000;
        
        // Mock type registry for testing
        if let Some(registry) = self.get_interface_registry_mut() {
            // Register all interfaces
            for (i, &id) in interface_ids.iter().enumerate() {
                registry.register_interface(id, format!("Interface_{}", i))?;
            }
            registry.register_concrete_type(concrete_type_id, "DeepConcrete".to_string())?;
            
            // Register inheritance relationships
            for i in 1..interface_ids.len() {
                registry.register_implementation(interface_ids[i], interface_ids[i-1])?;
            }
            
            // Concrete implements the deepest interface
            if !interface_ids.is_empty() {
                registry.register_implementation(concrete_type_id, interface_ids[interface_ids.len()-1])?;
            }
        }
        
        // Run the benchmark
        for _ in 0..iterations {
            let start = Instant::now();
            
            // Check if concrete implements the root interface
            let _result = self.find_inheritance_path(concrete_type_id, interface_ids[0]);
            
            // Record the time
            let duration = start.elapsed();
            durations.push(duration);
        }
        
        // Calculate statistics
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / durations.len() as u32;
        let min_duration = durations.iter().min().cloned().unwrap_or_default();
        let max_duration = durations.iter().max().cloned().unwrap_or_default();
        
        Ok(TypeAssertionBenchmarkResult {
            name: format!("Deep Hierarchy Type Assertions (depth={})", depth),
            avg_duration,
            min_duration,
            max_duration,
            iterations,
            phase_timing: None, // Not implemented yet
        })
    }
}

// Helper methods for the benchmarks
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Helper method to get a mutable reference to the interface registry
    fn get_interface_registry_mut(&mut self) -> Option<&mut dyn crate::codegen::llvm::interface_registry::InterfaceTypeRegistry> {
        self.internal_fields.get_mut("interface_registry")
            .and_then(|boxed| boxed.downcast_mut::<Box<dyn crate::codegen::llvm::interface_registry::InterfaceTypeRegistry>>())
            .map(|boxed| boxed.as_mut())
    }
    
    /// Helper to check if a type implements an interface
    fn type_implements(&self, concrete_type_id: u32, interface_type_id: u32) -> bool {
        if let Some(registry) = self.get_interface_registry() {
            registry.type_implements_interface(concrete_type_id, interface_type_id)
        } else {
            false
        }
    }
    
    /// Helper to get the interface registry
    fn get_interface_registry(&self) -> Option<&dyn crate::codegen::llvm::interface_registry::InterfaceTypeRegistry> {
        self.internal_fields.get("interface_registry")
            .and_then(|boxed| boxed.downcast_ref::<Box<dyn crate::codegen::llvm::interface_registry::InterfaceTypeRegistry>>())
            .map(|boxed| boxed.as_ref())
    }
    
    /// Helper method for finding a path in an inheritance hierarchy
    fn find_inheritance_path(&self, concrete_type_id: u32, interface_type_id: u32) -> Option<Vec<u32>> {
        // Check if we have a direct implementation
        if self.type_implements(concrete_type_id, interface_type_id) {
            return Some(vec![concrete_type_id, interface_type_id]);
        }
        
        // Check for interface path finder
        if let Some(path_finder) = self.get_interface_path_finder() {
            if let Ok(paths) = path_finder.find_all_paths(concrete_type_id, interface_type_id) {
                if !paths.is_empty() {
                    return Some(paths[0].path.clone());
                }
            }
        }
        
        None
    }
    
    /// Helper to get the interface path finder
    fn get_interface_path_finder(&self) -> Option<&dyn crate::codegen::llvm::interface_path_finder_enhanced::EnhancedInterfacePathFinder> {
        self.internal_fields.get("interface_path_finder")
            .and_then(|boxed| boxed.downcast_ref::<Box<dyn crate::codegen::llvm::interface_path_finder_enhanced::EnhancedInterfacePathFinder>>())
            .map(|boxed| boxed.as_ref())
    }
    
    /// Helper to detect diamond inheritance patterns
    fn detect_diamond_inheritance(
        &self,
        concrete_type_id: u32,
        interface_type_id: u32
    ) -> Result<Option<crate::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritancePattern>, Error> {
        use crate::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritanceDetection;
        self.detect_diamond_inheritance(concrete_type_id, interface_type_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_benchmark_config_default() {
        let config = InterfaceTypeAssertionBenchmarkConfig::default();
        assert_eq!(config.iterations, 1000);
        assert_eq!(config.enable_warmup, true);
        assert_eq!(config.warmup_iterations, 100);
    }
    
    #[test]
    fn test_benchmark_simple_setup() {
        // Create a simple benchmark configuration with minimal iterations for testing
        let config = InterfaceTypeAssertionBenchmarkConfig {
            iterations: 10,
            enable_warmup: false,
            warmup_iterations: 0,
            detailed_timing: false,
            test_diamond_patterns: false,
            test_deep_hierarchies: false,
            max_hierarchy_depth: 3,
        };
        
        // This is just a setup test, not executing the actual benchmark
        assert_eq!(config.iterations, 10);
    }
}