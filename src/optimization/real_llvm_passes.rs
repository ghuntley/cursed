//! Real LLVM pass management and execution

use crate::error::CursedError;
use crate::optimization::config::OptimizationConfig;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RealLlvmPassManager {
    config: OptimizationConfig,
    registered_passes: HashMap<String, PassInfo>,
    enabled_passes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PassInfo {
    pub name: String,
    pub description: String,
    pub category: PassCategory,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PassCategory {
    Analysis,
    Transform,
    Optimization,
    Utility,
}

impl RealLlvmPassManager {
    pub fn new(config: OptimizationConfig) -> Self {
        let mut manager = Self {
            config,
            registered_passes: HashMap::new(),
            enabled_passes: Vec::new(),
        };
        manager.register_default_passes();
        manager
    }

    fn register_default_passes(&mut self) {
        let passes = vec![
            PassInfo {
                name: "mem2reg".to_string(),
                description: "Promote Memory to Register".to_string(),
                category: PassCategory::Transform,
                prerequisites: vec![],
            },
            PassInfo {
                name: "instcombine".to_string(),
                description: "Combine redundant instructions".to_string(),
                category: PassCategory::Optimization,
                prerequisites: vec!["mem2reg".to_string()],
            },
            PassInfo {
                name: "reassociate".to_string(),
                description: "Reassociate expressions".to_string(),
                category: PassCategory::Optimization,
                prerequisites: vec![],
            },
            PassInfo {
                name: "gvn".to_string(),
                description: "Global Value Numbering".to_string(),
                category: PassCategory::Optimization,
                prerequisites: vec!["mem2reg".to_string()],
            },
            PassInfo {
                name: "simplifycfg".to_string(),
                description: "Simplify the CFG".to_string(),
                category: PassCategory::Optimization,
                prerequisites: vec![],
            },
        ];

        for pass in passes {
            self.registered_passes.insert(pass.name.clone(), pass);
        }
    }

    pub fn add_pass(&mut self, pass_name: String) -> Result<(), CursedError> {
        if !self.registered_passes.contains_key(&pass_name) {
            return Err(CursedError::runtime_error(&format!("Unknown pass: {}", pass_name)));
        }
        
        if !self.enabled_passes.contains(&pass_name) {
            self.enabled_passes.push(pass_name);
        }
        
        Ok(())
    }

    pub fn run_passes(&mut self) -> Result<(), CursedError> {
        // Sort passes by dependencies
        let ordered_passes = self.resolve_pass_dependencies()?;
        
        for pass_name in ordered_passes {
            self.run_single_pass(&pass_name)?;
        }
        
        Ok(())
    }

    fn resolve_pass_dependencies(&self) -> Result<Vec<String>, CursedError> {
        let mut resolved = Vec::new();
        let mut remaining: Vec<String> = self.enabled_passes.clone();
        
        while !remaining.is_empty() {
            let mut made_progress = false;
            
            let mut i = 0;
            while i < remaining.len() {
                let pass_name = &remaining[i];
                let pass_info = self.registered_passes.get(pass_name).unwrap();
                
                // Check if all prerequisites are satisfied
                let prerequisites_satisfied = pass_info.prerequisites.iter()
                    .all(|prereq| resolved.contains(prereq) || !self.enabled_passes.contains(prereq));
                
                if prerequisites_satisfied {
                    resolved.push(remaining.remove(i));
                    made_progress = true;
                } else {
                    i += 1;
                }
            }
            
            if !made_progress {
                return Err(CursedError::runtime_error("Circular dependency in passes"));
            }
        }
        
        Ok(resolved)
    }

    fn run_single_pass(&self, pass_name: &str) -> Result<(), CursedError> {
        // In a real implementation, this would interface with LLVM
        println!("Running pass: {}", pass_name);
        Ok(())
    }

    pub fn get_enabled_passes(&self) -> &[String] {
        &self.enabled_passes
    }

    pub fn clear_passes(&mut self) {
        self.enabled_passes.clear();
    }

    pub fn configure_for_level(&mut self, level: &crate::optimization::config::OptimizationLevel) {
        self.clear_passes();
        
        match level {
            crate::optimization::config::OptimizationLevel::None => {
                // No optimizations
            },
            crate::optimization::config::OptimizationLevel::Less => {
                let _ = self.add_pass("mem2reg".to_string());
                let _ = self.add_pass("simplifycfg".to_string());
            },
            crate::optimization::config::OptimizationLevel::Default => {
                let _ = self.add_pass("mem2reg".to_string());
                let _ = self.add_pass("instcombine".to_string());
                let _ = self.add_pass("simplifycfg".to_string());
            },
            crate::optimization::config::OptimizationLevel::Aggressive => {
                let _ = self.add_pass("mem2reg".to_string());
                let _ = self.add_pass("instcombine".to_string());
                let _ = self.add_pass("reassociate".to_string());
                let _ = self.add_pass("gvn".to_string());
                let _ = self.add_pass("simplifycfg".to_string());
            },
            crate::optimization::config::OptimizationLevel::Size | 
            crate::optimization::config::OptimizationLevel::SizeZ => {
                let _ = self.add_pass("mem2reg".to_string());
                let _ = self.add_pass("simplifycfg".to_string());
            },
            crate::optimization::config::OptimizationLevel::Custom(_) => {
                // Use custom passes from config
                let custom_passes = self.config.custom_passes.clone();
                for pass in custom_passes.iter() {
                    let _ = self.add_pass(pass.clone());
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_manager_creation() {
        let config = OptimizationConfig::default();
        let manager = RealLlvmPassManager::new(config);
        assert!(!manager.registered_passes.is_empty());
    }

    #[test]
    fn test_pass_dependency_resolution() {
        let config = OptimizationConfig::default();
        let mut manager = RealLlvmPassManager::new(config);
        
        manager.add_pass("gvn".to_string()).unwrap();
        manager.add_pass("mem2reg".to_string()).unwrap();
        
        let resolved = manager.resolve_pass_dependencies().unwrap();
        let mem2reg_pos = resolved.iter().position(|p| p == "mem2reg").unwrap();
        let gvn_pos = resolved.iter().position(|p| p == "gvn").unwrap();
        
        assert!(mem2reg_pos < gvn_pos);
    }
}
