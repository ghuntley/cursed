#!/usr/bin/env python3

import os

def create_minimal_execution_module():
    """Create minimal execution module"""
    content = '''// Minimal execution module for CURSED minimal build

use crate::error::{Error, Result};

// Basic value types for minimal build
#[derive(Debug, Clone)]
pub enum CursedValue {
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl std::fmt::Display for CursedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedValue::Nil => write!(f, "nil"),
            CursedValue::Integer(i) => write!(f, "{}", i),
            CursedValue::Float(fl) => write!(f, "{}", fl),
            CursedValue::String(s) => write!(f, "\\"{}\\"", s),
            CursedValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}

// Basic execution engine for minimal build
pub struct CursedExecutionEngine {
    // Minimal state
}

impl CursedExecutionEngine {
    pub fn new() -> Result<Self> {
        Ok(CursedExecutionEngine {})
    }
    
    pub fn execute(&mut self, source: &str) -> Result<CursedValue> {
        // Minimal implementation - just return nil for now
        tracing::info!("Executing CURSED source (minimal): {}", source.len());
        Ok(CursedValue::Nil)
    }
    
    pub fn execute_file(&mut self, path: &str) -> Result<CursedValue> {
        let source = std::fs::read_to_string(path)?;
        self.execute(&source)
    }
    
    pub fn execute_repl(&mut self, code: &str) -> Result<String> {
        let result = self.execute(code)?;
        Ok(format!("{}", result))
    }
    
    pub fn get_value_manager(&self) -> ValueManager {
        ValueManager {}
    }
}

// Basic value manager for minimal build
pub struct ValueManager {}

impl ValueManager {
    pub fn format_value(&self, value: &CursedValue) -> String {
        format!("{}", value)
    }
}

// Re-export submodules that exist
pub mod execution_context;
'''
    
    with open("src/execution/mod.rs", 'w') as f:
        f.write(content)
    print("Created minimal execution module")

def create_minimal_runtime_modules():
    """Create minimal runtime modules"""
    
    # Create minimal stack module
    stack_content = '''// Minimal stack module for CURSED minimal build

use crate::error::{Error, Result};

pub struct RuntimeStack {
    // Minimal implementation
}

impl RuntimeStack {
    pub fn new() -> Self {
        RuntimeStack {}
    }
    
    pub fn push_frame(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub fn pop_frame(&mut self) -> Result<()> {
        Ok(())
    }
}
'''
    
    os.makedirs("src/runtime", exist_ok=True)
    with open("src/runtime/stack.rs", 'w') as f:
        f.write(stack_content)
    print("Created minimal runtime/stack module")
    
    # Create minimal value module
    value_content = '''// Minimal value module for CURSED minimal build

use crate::error::{Error, Result};

pub use crate::execution::CursedValue;

pub struct ValueManager {
    // Minimal implementation  
}

impl ValueManager {
    pub fn new() -> Self {
        ValueManager {}
    }
    
    pub fn format_value(&self, value: &CursedValue) -> String {
        format!("{}", value)
    }
}
'''
    
    with open("src/runtime/value.rs", 'w') as f:
        f.write(value_content)
    print("Created minimal runtime/value module")
    
    # Create minimal runtime mod.rs
    runtime_mod_content = '''// Minimal runtime module for CURSED minimal build

pub mod stack;
pub mod value;

pub use stack::RuntimeStack;
pub use value::{ValueManager, CursedValue};
'''
    
    with open("src/runtime/mod.rs", 'w') as f:
        f.write(runtime_mod_content)
    print("Created minimal runtime module")

def disable_more_codegen_modules():
    """Disable more problematic codegen modules"""
    
    heavy_modules = [
        "src/codegen/llvm/variable_management.rs",
        "src/codegen/llvm/question_mark.rs", 
        "src/codegen/llvm/result_types.rs",
        "src/codegen/llvm/optimization.rs",
    ]
    
    for module_path in heavy_modules:
        if os.path.exists(module_path):
            # Backup original
            backup_path = module_path + ".full"
            if not os.path.exists(backup_path):
                os.rename(module_path, backup_path)
                print(f"Backed up {module_path}")
            
            # Create minimal stub
            module_name = os.path.basename(module_path).replace('.rs', '').replace('_', '')
            content = f'''// Minimal {module_name} module - disabled for minimal build
use crate::error::{{Error, Result}};

// Minimal placeholder implementations
pub struct {module_name.title()}Disabled {{}}

impl {module_name.title()}Disabled {{
    pub fn new() -> Result<Self> {{
        Err(Error::NotImplemented(
            "{module_name} is disabled in minimal build".to_string()
        ))
    }}
}}
'''
            
            with open(module_path, 'w') as f:
                f.write(content)
            print(f"Created minimal stub for {module_path}")

if __name__ == "__main__":
    create_minimal_execution_module()
    create_minimal_runtime_modules()
    disable_more_codegen_modules()
    print("✅ More heavy modules disabled successfully!")
