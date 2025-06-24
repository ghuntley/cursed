
/// Pass Registry and Management System
/// 
/// Provides registration, dependency resolution, and execution management
/// for optimization passes in the CURSED compiler.

use super::{OptimizationPass, PassConfiguration, PassResult};
use crate::common::optimization_level::OptimizationLevel;
use crate::error::{Error, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn, error};

/// Central registry for optimization passes
pub struct PassRegistry<'ctx> {
    passes: HashMap<String, Box<dyn OptimizationPass<'ctx> + Send + Sync>>,
    dependencies: HashMap<String, Vec<String>>,
    execution_order: Vec<String>,
    statistics: Arc<Mutex<RegistryStatistics>>,
    config: PassConfiguration,
}

impl<'ctx> PassRegistry<'ctx> {
    /// Create a new pass registry
    pub fn new(config: PassConfiguration) -> Self {
        Self {
            passes: HashMap::new(),
            dependencies: HashMap::new(),
            execution_order: Vec::new(),
            statistics: Arc::new(Mutex::new(RegistryStatistics::default())),
            config,
        }
    }
    
    /// Register a new optimization pass
    #[instrument(skip(self, pass))]
    pub fn register_pass<T>(&mut self, pass: T) -> Result<()>
    where
        T: OptimizationPass<'ctx> + Send + Sync + 'static,
    {
        let pass_name = pass.name().to_string();
        let dependencies = pass.dependencies();
        
        info!("Registering optimization pass: {}", pass_name);
        debug!("Pass dependencies: {:?}", dependencies);
        
        // Validate dependencies exist or will be registered
        for dep in &dependencies {
            if !self.passes.contains_key(dep) && !self.is_builtin_pass(dep) {
                warn!("Pass {} depends on unregistered pass: {}", pass_name, dep);
            }
        }
        
        self.dependencies.insert(pass_name.clone(), dependencies);
        self.passes.insert(pass_name.clone(), Box::new(pass));
        
        // Rebuild execution order
        self.rebuild_execution_order()?;
        
        let mut stats = self.statistics.lock().unwrap();
        stats.registered_passes += 1;
        
        info!("Successfully registered pass: {}", pass_name);
        Ok(())
    }
    
    /// Register multiple passes at once
    pub fn register_passes(&mut self, passes: Vec<Box<dyn OptimizationPass<'ctx> + Send + Sync>>) -> Result<()> {
        for pass in passes {
            self.register_pass_boxed(pass)?;
        }
        Ok(())
    }
    
    /// Register a boxed pass
    fn register_pass_boxed(&mut self, pass: Box<dyn OptimizationPass<'ctx> + Send + Sync>) -> Result<()> {
        let pass_name = pass.name().to_string();
        let dependencies = pass.dependencies();
        
        info!("Registering boxed optimization pass: {}", pass_name);
        
        self.dependencies.insert(pass_name.clone(), dependencies);
        self.passes.insert(pass_name.clone(), pass);
        
        self.rebuild_execution_order()?;
        
        let mut stats = self.statistics.lock().unwrap();
        stats.registered_passes += 1;
        
        Ok(())
    }
    
    /// Unregister a pass
    pub fn unregister_pass(&mut self, pass_name: &str) -> Result<()> {
        if self.passes.remove(pass_name).is_some() {
            self.dependencies.remove(pass_name);
            self.rebuild_execution_order()?;
            
            let mut stats = self.statistics.lock().unwrap();
            stats.registered_passes -= 1;
            
            info!("Unregistered pass: {}", pass_name);
            Ok(())
        } else {
            Err(Error::Internal(format!("Pass not found: {}", pass_name)))
        }
    }
    
    /// Get list of registered passes
    pub fn get_registered_passes(&self) -> Vec<String> {
        self.passes.keys().cloned().collect()
    }
    
    /// Get pass by name
    pub fn get_pass(&self, pass_name: &str) -> Option<&dyn OptimizationPass<'ctx>> {
        self.passes.get(pass_name).map(|p| p.as_ref())
    }
    
    /// Get mutable pass by name
    pub fn get_pass_mut(&mut self, pass_name: &str) -> Option<&mut dyn OptimizationPass<'ctx>> {
        self.passes.get_mut(pass_name).map(|p| p.as_mut())
    }
    
    /// Check if a pass is registered
    pub fn is_pass_registered(&self, pass_name: &str) -> bool {
        self.passes.contains_key(pass_name)
    }
    
    /// Get execution order for passes
    pub fn get_execution_order(&self) -> &[String] {
        &self.execution_order
    }
    
    /// Get passes for a specific optimization level
    pub fn get_passes_for_level(&self, level: OptimizationLevel) -> Vec<String> {
        self.passes
            .iter()
            .filter(|(_, pass)| {
                pass.required_optimization_level() <= level && 
                pass.should_run(&self.config)
            })
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Rebuild execution order based on dependencies
    #[instrument(skip(self))]
    fn rebuild_execution_order(&mut self) -> Result<()> {
        debug!("Rebuilding pass execution order");
        
        let order = self.topological_sort()?;
        self.execution_order = order;
        
        debug!("New execution order: {:?}", self.execution_order);
        Ok(())
    }
    
    /// Perform topological sort of passes based on dependencies
    fn topological_sort(&self) -> Result<Vec<String>> {
        let mut in_degree = HashMap::new();
        let mut graph = HashMap::new();
        
        // Build graph and calculate in-degrees
        for (pass_name, deps) in &self.dependencies {
            in_degree.insert(pass_name.clone(), deps.len());
            graph.insert(pass_name.clone(), deps.clone());
        }
        
        // Add passes with no dependencies to queue
        let mut queue = VecDeque::new();
        for (pass_name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(pass_name.clone());
            }
        }
        
        let mut result = Vec::new();
        
        while let Some(current_pass) = queue.pop_front() {
            result.push(current_pass.clone());
            
            // Update in-degrees of dependent passes
            for (pass_name, deps) in &self.dependencies {
                if deps.contains(&current_pass) {
                    if let Some(degree) = in_degree.get_mut(pass_name) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(pass_name.clone());
                        }
                    }
                }
            }
        }
        
        // Check for cycles
        if result.len() != self.passes.len() {
            let missing_passes: Vec<_> = self.passes
                .keys()
                .filter(|&name| !result.contains(name))
                .cloned()
                .collect();
            
            return Err(Error::Internal(format!(
                "Circular dependency detected in passes: {:?}",
                missing_passes
            )));
        }
        
        Ok(result)
    }
    
    /// Check if a pass is a builtin pass (always available)
    fn is_builtin_pass(&self, pass_name: &str) -> bool {
        matches!(
            pass_name,
            "mem2reg" | "instcombine" | "gvn" | "simplifycfg" | "dce" | "adce"
        )
    }
    
    /// Get registry statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    /// Reset all pass statistics
    pub fn reset_statistics(&mut self) {
        for pass in self.passes.values_mut() {
            pass.reset();
        }
        
        let mut stats = self.statistics.lock().unwrap();
        *stats = RegistryStatistics {
            registered_passes: stats.registered_passes,
            ..Default::default()
        };
    }
    
    /// Validate pass dependencies
    pub fn validate_dependencies(&self) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        
        for (pass_name, deps) in &self.dependencies {
            for dep in deps {
                if !self.passes.contains_key(dep) && !self.is_builtin_pass(dep) {
                    issues.push(format!("Pass '{}' depends on missing pass '{}'", pass_name, dep));
                }
            }
        }
        
        if issues.is_empty() {
            Ok(Vec::new())
        } else {
            warn!("Found dependency issues: {:?}", issues);
            Ok(issues)
        }
    }
    
    /// Create a default registry with standard passes
    pub fn create_default_registry(config: PassConfiguration) -> Self {
        let mut registry = Self::new(config);
        
        // Register standard optimization passes
        if let Err(e) = registry.register_standard_passes() {
            error!("Failed to register standard passes: {}", e);
        }
        
        registry
    }
    
    /// Register standard optimization passes
    fn register_standard_passes(&mut self) -> Result<()> {
        use super::{DeadCodeEliminationPass, ConstantPropagationPass, LoopOptimizationPass, InliningPass};
        
        // Dead code elimination
        let dce_pass = DeadCodeEliminationPass::new(self.config.clone());
        self.register_pass(dce_pass)?;
        
        // Constant propagation
        let cp_pass = ConstantPropagationPass::new(self.config.clone());
        self.register_pass(cp_pass)?;
        
        // Loop optimization
        let loop_pass = LoopOptimizationPass::new(self.config.clone());
        self.register_pass(loop_pass)?;
        
        // Function inlining
        let inline_pass = InliningPass::new(self.config.clone());
        self.register_pass(inline_pass)?;
        
        info!("Registered {} standard passes", 4);
        Ok(())
    }
    
    /// Get recommended pass sequence for optimization level
    pub fn get_recommended_sequence(&self, level: OptimizationLevel) -> Vec<String> {
        match level {
            OptimizationLevel::O0 => vec![],
            OptimizationLevel::O1 => vec![
                "dead_code_elimination".to_string(),
                "constant_propagation".to_string(),
            ],
            OptimizationLevel::O2 => vec![
                "dead_code_elimination".to_string(),
                "constant_propagation".to_string(),
                "loop_optimization".to_string(),
                "inlining".to_string(),
            ],
            OptimizationLevel::O3 => vec![
                "dead_code_elimination".to_string(),
                "constant_propagation".to_string(),
                "loop_optimization".to_string(),
                "inlining".to_string(),
                "memory_optimization".to_string(),
                "instruction_combining".to_string(),
                "branch_optimization".to_string(),
            ],
            OptimizationLevel::Os | OptimizationLevel::Oz => vec![
                "dead_code_elimination".to_string(),
                "constant_propagation".to_string(),
                "memory_optimization".to_string(),
            ],
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone, Default)]
pub struct RegistryStatistics {
    pub registered_passes: usize,
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_execution_time: Duration,
    pub dependency_resolution_time: Duration,
}

impl RegistryStatistics {
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }
    
    /// Calculate average execution time
    pub fn average_execution_time(&self) -> Duration {
        if self.total_executions == 0 {
            Duration::from_secs(0)
        } else {
            self.total_execution_time / self.total_executions as u32
        }
    }
}

/// Pass registration helper
pub struct PassRegistration<'ctx> {
    registry: &'ctx mut PassRegistry<'ctx>,
}

impl<'ctx> PassRegistration<'ctx> {
    /// Create a new pass registration helper
    pub fn new(registry: &'ctx mut PassRegistry<'ctx>) -> Self {
        Self { registry }
    }
    
    /// Register a pass with fluent interface
    pub fn register<T>(self, pass: T) -> Result<Self>
    where
        T: OptimizationPass<'ctx> + Send + Sync + 'static,
    {
        self.registry.register_pass(pass)?;
        Ok(self)
    }
    
    /// Finish registration and return registry
    pub fn finish(self) -> &'ctx mut PassRegistry<'ctx> {
        self.registry
    }
}

/// Pass dependency specification
#[derive(Debug, Clone)]
pub struct PassDependency {
    pub name: String,
    pub required: bool,
    pub minimum_optimization_level: OptimizationLevel,
}

impl PassDependency {
    /// Create a required dependency
    pub fn required(name: &str) -> Self {
        Self {
            name: name.to_string(),
            required: true,
            minimum_optimization_level: OptimizationLevel::O0,
        }
    }
    
    /// Create an optional dependency
    pub fn optional(name: &str) -> Self {
        Self {
            name: name.to_string(),
            required: false,
            minimum_optimization_level: OptimizationLevel::O0,
        }
    }
    
    /// Set minimum optimization level for dependency
    pub fn with_level(mut self, level: OptimizationLevel) -> Self {
        self.minimum_optimization_level = level;
        self
    }
}

/// Pass execution context
#[derive(Debug)]
pub struct PassExecutionContext {
    pub optimization_level: OptimizationLevel,
    pub time_budget: Duration,
    pub remaining_time: Duration,
    pub passes_executed: usize,
    pub current_pass: Option<String>,
}

impl PassExecutionContext {
    /// Create a new execution context
    pub fn new(config: &PassConfiguration) -> Self {
        Self {
            optimization_level: config.optimization_level,
            time_budget: config.time_budget,
            remaining_time: config.time_budget,
            passes_executed: 0,
            current_pass: None,
        }
    }
    
    /// Check if there's enough time to run another pass
    pub fn has_time_for_pass(&self, estimated_time: Duration) -> bool {
        self.remaining_time >= estimated_time
    }
    
    /// Update context after pass execution
    pub fn update_after_pass(&mut self, execution_time: Duration) {
        self.remaining_time = self.remaining_time.saturating_sub(execution_time);
        self.passes_executed += 1;
        self.current_pass = None;
    }
    
    /// Set current pass
    pub fn set_current_pass(&mut self, pass_name: String) {
        self.current_pass = Some(pass_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::passes::{PassConfiguration, DeadCodeEliminationPass, ConstantPropagationPass};
    
    #[test]
    fn test_registry_creation() {
        let config = PassConfiguration::default();
        let registry = PassRegistry::new(config);
        
        assert_eq!(registry.get_registered_passes().len(), 0);
        assert_eq!(registry.get_execution_order().len(), 0);
    }
    
    #[test]
    fn test_pass_registration() {
        let config = PassConfiguration::default();
        let mut registry = PassRegistry::new(config.clone());
        
        let dce_pass = DeadCodeEliminationPass::new(config);
        let result = registry.register_pass(dce_pass);
        
        assert!(result.is_ok());
        assert_eq!(registry.get_registered_passes().len(), 1);
        assert!(registry.is_pass_registered("dead_code_elimination"));
    }
    
    #[test]
    fn test_dependency_resolution() {
        let config = PassConfiguration::default();
        let mut registry = PassRegistry::new(config.clone());
        
        // Register passes with dependencies
        let dce_pass = DeadCodeEliminationPass::new(config.clone());
        let cp_pass = ConstantPropagationPass::new(config);
        
        registry.register_pass(dce_pass).unwrap();
        registry.register_pass(cp_pass).unwrap();
        
        let order = registry.get_execution_order();
        assert!(!order.is_empty());
        
        // Validate dependencies are satisfied
        let issues = registry.validate_dependencies().unwrap();
        assert!(issues.is_empty());
    }
    
    #[test]
    fn test_optimization_level_filtering() {
        let config = PassConfiguration::default();
        let mut registry = PassRegistry::new(config.clone());
        
        let dce_pass = DeadCodeEliminationPass::new(config);
        registry.register_pass(dce_pass).unwrap();
        
        let basic_passes = registry.get_passes_for_level(OptimizationLevel::O1);
        assert!(!basic_passes.is_empty());
        
        let none_passes = registry.get_passes_for_level(OptimizationLevel::O0);
        // DCE requires Basic level, so should be empty for None
        assert!(none_passes.is_empty() || none_passes.len() < basic_passes.len());
    }
    
    #[test]
    fn test_recommended_sequences() {
        let config = PassConfiguration::default();
        let registry = PassRegistry::new(config);
        
        let none_seq = registry.get_recommended_sequence(OptimizationLevel::O0);
        let basic_seq = registry.get_recommended_sequence(OptimizationLevel::O1);
        let aggressive_seq = registry.get_recommended_sequence(OptimizationLevel::O3);
        
        assert!(none_seq.is_empty());
        assert!(!basic_seq.is_empty());
        assert!(aggressive_seq.len() > basic_seq.len());
    }
    
    #[test]
    fn test_pass_dependency() {
        let required_dep = PassDependency::required("test_pass");
        assert!(required_dep.required);
        assert_eq!(required_dep.name, "test_pass");
        
        let optional_dep = PassDependency::optional("other_pass")
            .with_level(OptimizationLevel::O3);
        assert!(!optional_dep.required);
        assert_eq!(optional_dep.minimum_optimization_level, OptimizationLevel::O3);
    }
    
    #[test]
    fn test_execution_context() {
        let config = PassConfiguration::default();
        let mut context = PassExecutionContext::new(&config);
        
        assert_eq!(context.passes_executed, 0);
        assert_eq!(context.remaining_time, config.time_budget);
        
        let pass_time = Duration::from_millis(100);
        assert!(context.has_time_for_pass(pass_time));
        
        context.update_after_pass(pass_time);
        assert_eq!(context.passes_executed, 1);
        assert!(context.remaining_time < config.time_budget);
    }
    
    #[test]
    fn test_registry_statistics() {
        let config = PassConfiguration::default();
        let registry = PassRegistry::new(config);
        let stats = registry.get_statistics();
        
        assert_eq!(stats.registered_passes, 0);
        assert_eq!(stats.total_executions, 0);
        assert_eq!(stats.success_rate(), 0.0);
    }
}
