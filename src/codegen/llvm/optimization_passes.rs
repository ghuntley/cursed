//! Optimization Passes implementation for CURSED LLVM compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Registry for managing optimization passes
#[derive(Debug)]
pub struct PassRegistry {
    passes: HashMap<String, Box<dyn OptimizationPass>>,
    configurations: HashMap<String, PassConfiguration>,
}

/// Configuration for optimization passes
#[derive(Debug, Clone)]
pub struct PassConfiguration {
    pub enabled: bool,
    pub priority: u32,
    pub max_iterations: u32,
    pub time_category: PassTimeCategory,
    pub parameters: HashMap<String, String>,
}

/// Trait for optimization passes
pub trait OptimizationPass: std::fmt::Debug + Send + Sync {
    /// Name of the optimization pass
    fn name(&self) -> &str;
    
    /// Run the optimization pass
    fn run(&mut self, code: &str, config: &PassConfiguration) -> Result<PassResult, CursedError>;
    
    /// Check if the pass can be run on the given code
    fn can_run(&self, code: &str) -> bool;
    
    /// Get pass dependencies
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }
}

/// Result of running an optimization pass
#[derive(Debug)]
pub struct PassResult {
    pub success: bool,
    pub transformed_code: String,
    pub execution_time: Duration,
    pub improvements: HashMap<String, f64>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Categories for pass execution timing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassTimeCategory {
    Fast,      // < 10ms
    Medium,    // 10ms - 100ms
    Slow,      // 100ms - 1s
    VerySlow,  // > 1s
}

impl Default for PassConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            priority: 50,
            max_iterations: 1,
            time_category: PassTimeCategory::Medium,
            parameters: HashMap::new(),
        }
    }
}

impl PassRegistry {
    /// Create a new pass registry
    pub fn new() -> Self {
        Self {
            passes: HashMap::new(),
            configurations: HashMap::new(),
        }
    }

    /// Register a new optimization pass
    pub fn register_pass(&mut self, pass: Box<dyn OptimizationPass>) -> Result<(), CursedError> {
        let name = pass.name().to_string();
        self.passes.insert(name.clone(), pass);
        self.configurations.insert(name, PassConfiguration::default());
        Ok(())
    }

    /// Configure a pass
    pub fn configure_pass(&mut self, name: &str, config: PassConfiguration) -> Result<(), CursedError> {
        if !self.passes.contains_key(name) {
            return Err(CursedError::runtime_error(&format!("Pass '{}' not found", name)));
        }
        self.configurations.insert(name.to_string(), config);
        Ok(())
    }

    /// Run a specific pass
    pub fn run_pass(&mut self, name: &str, code: &str) -> Result<PassResult, CursedError> {
        let config = self.configurations.get(name)
            .ok_or_else(|| CursedError::runtime_error(&format!("Configuration for pass '{}' not found", name)))?
            .clone();

        if !config.enabled {
            return Ok(PassResult::skipped(code.to_string()));
        }

        let pass = self.passes.get_mut(name)
            .ok_or_else(|| CursedError::runtime_error(&format!("Pass '{}' not found", name)))?;

        if !pass.can_run(code) {
            return Ok(PassResult::skipped(code.to_string()));
        }

        pass.run(code, &config)
    }

    /// Run all enabled passes
    pub fn run_all_passes(&mut self, code: &str) -> Result<String, CursedError> {
        let mut current_code = code.to_string();
        
        // Sort passes by priority
        let mut pass_names: Vec<_> = self.configurations.iter()
            .filter(|(_, config)| config.enabled)
            .map(|(name, config)| (name.clone(), config.priority))
            .collect();
        pass_names.sort_by_key(|(_, priority)| *priority);

        for (name, _) in pass_names {
            match self.run_pass(&name, &current_code) {
                Ok(result) if result.success => {
                    current_code = result.transformed_code;
                }
                Ok(_) => {
                    // Pass was skipped or failed, continue with current code
                }
                Err(_) => {
                    // Pass failed, continue with current code
                }
            }
        }

        Ok(current_code)
    }

    /// Get all registered pass names
    pub fn get_pass_names(&self) -> Vec<String> {
        self.passes.keys().cloned().collect()
    }

    /// Check if a pass is registered
    pub fn has_pass(&self, name: &str) -> bool {
        self.passes.contains_key(name)
    }
}

impl PassResult {
    /// Create a successful pass result
    pub fn success(transformed_code: String, execution_time: Duration) -> Self {
        Self {
            success: true,
            transformed_code,
            execution_time,
            improvements: HashMap::new(),
            warnings: vec![],
            errors: vec![],
        }
    }

    /// Create a failed pass result
    pub fn failure(error: String, original_code: String) -> Self {
        Self {
            success: false,
            transformed_code: original_code,
            execution_time: Duration::from_millis(0),
            improvements: HashMap::new(),
            warnings: vec![],
            errors: vec![error],
        }
    }

    /// Create a skipped pass result
    pub fn skipped(original_code: String) -> Self {
        Self {
            success: true,
            transformed_code: original_code,
            execution_time: Duration::from_millis(0),
            improvements: HashMap::new(),
            warnings: vec!["Pass was skipped".to_string()],
            errors: vec![],
        }
    }
}

impl Default for PassRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Example optimization pass implementations
#[derive(Debug)]
pub struct DeadCodeEliminationPass;

impl OptimizationPass for DeadCodeEliminationPass {
    fn name(&self) -> &str {
        "dead_code_elimination"
    }

    fn run(&mut self, code: &str, _config: &PassConfiguration) -> Result<PassResult, CursedError> {
        let start_time = Instant::now();
        
        // Simulate dead code elimination
        let transformed_code = format!("// Dead code eliminated\n{}", code);
        let execution_time = start_time.elapsed();
        
        let mut improvements = HashMap::new();
        improvements.insert("code_size_reduction".to_string(), 0.1);
        
        Ok(PassResult {
            success: true,
            transformed_code,
            execution_time,
            improvements,
            warnings: vec![],
            errors: vec![],
        })
    }

    fn can_run(&self, _code: &str) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct InliningPass;

impl OptimizationPass for InliningPass {
    fn name(&self) -> &str {
        "inlining"
    }

    fn run(&mut self, code: &str, _config: &PassConfiguration) -> Result<PassResult, CursedError> {
        let start_time = Instant::now();
        
        // Simulate function inlining
        let transformed_code = format!("// Functions inlined\n{}", code);
        let execution_time = start_time.elapsed();
        
        let mut improvements = HashMap::new();
        improvements.insert("performance_improvement".to_string(), 0.15);
        
        Ok(PassResult {
            success: true,
            transformed_code,
            execution_time,
            improvements,
            warnings: vec![],
            errors: vec![],
        })
    }

    fn can_run(&self, code: &str) -> bool {
        code.contains("function") || code.contains("fn")
    }

    fn dependencies(&self) -> Vec<String> {
        vec!["dead_code_elimination".to_string()]
    }
}
