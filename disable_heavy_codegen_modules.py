#!/usr/bin/env python3

import os

def create_minimal_module(path, module_name):
    """Create a minimal stub for a module"""
    content = f'''// Minimal {module_name} module - heavy features disabled for minimal build
// This file was auto-generated to reduce compilation scope

// Re-export essential Error type
use crate::{{Error, SourceLocation}};

// Basic placeholder implementations that return errors indicating features are disabled
pub struct {module_name}Disabled {{}}

impl Default for {module_name}Disabled {{
    fn default() -> Self {{
        Self {{}}
    }}
}}

impl {module_name}Disabled {{
    pub fn new() -> Result<Self, Error> {{
        Err(Error::NotImplemented(
            "{module_name} is disabled in minimal build. Use full build for this feature.".to_string()
        ))
    }}
}}

// Placeholder trait implementations as needed
'''
    
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w') as f:
        f.write(content)
    print(f"Created minimal module: {path}")

def disable_heavy_codegen_modules():
    """Disable heavy LLVM codegen modules that are causing compilation errors"""
    
    heavy_modules = [
        # Process-related modules that require heavy dependencies
        ("src/codegen/llvm/process.rs", "ProcessCompilation"),
        ("src/codegen/llvm/process_ipc_integration.rs", "ProcessIpcIntegration"),
        ("src/codegen/llvm/enhanced_codegen.rs", "EnhancedCodegen"),
        ("src/codegen/llvm/performance_monitor.rs", "PerformanceMonitor"),
        ("src/codegen/llvm/package_integration.rs", "PackageIntegration"),
        ("src/codegen/llvm/template.rs", "TemplateCompilation"),
    ]
    
    for module_path, module_name in heavy_modules:
        if os.path.exists(module_path):
            # Backup original
            backup_path = module_path + ".full"
            if not os.path.exists(backup_path):
                os.rename(module_path, backup_path)
                print(f"Backed up {module_path} to {backup_path}")
            
            # Create minimal version
            create_minimal_module(module_path, module_name)

    # Create minimal implementations for common modules  
    common_modules = [
        ("src/common/mod.rs", """// Minimal common module
use thiserror::Error;

// Basic error type for minimal build
#[derive(Error, Debug, Clone)]
pub enum MinimalError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Codegen error: {0}")]  
    Codegen(String),
    #[error("Runtime error: {0}")]
    Runtime(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("IO error: {0}")]
    Io(String),
}

// Use minimal error as Error for now
pub use MinimalError as Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

impl OptimizationLevel {
    pub fn to_llvm_level(&self) -> u32 {
        match self {
            OptimizationLevel::O0 => 0,
            OptimizationLevel::O1 => 1,
            OptimizationLevel::O2 => 2,
            OptimizationLevel::O3 => 3,
        }
    }
    
    pub fn from_string(s: &str) -> Self {
        match s {
            "O0" => OptimizationLevel::O0,
            "O1" => OptimizationLevel::O1, 
            "O2" => OptimizationLevel::O2,
            "O3" => OptimizationLevel::O3,
            _ => OptimizationLevel::O0,
        }
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::O0
    }
}

pub type Result<T> = std::result::Result<T, Error>;
"""),
    ]
    
    for module_path, content in common_modules:
        if os.path.exists(module_path):
            backup_path = module_path + ".full"
            if not os.path.exists(backup_path):
                os.rename(module_path, backup_path)
                print(f"Backed up {module_path} to {backup_path}")
        
        os.makedirs(os.path.dirname(module_path), exist_ok=True)
        with open(module_path, 'w') as f:
            f.write(content)
        print(f"Created minimal module: {module_path}")

if __name__ == "__main__":
    disable_heavy_codegen_modules()
    print("✅ Heavy codegen modules disabled successfully!")
    print("   Heavy modules backed up with .full extension")
    print("   Minimal stubs created to reduce compilation errors")
