#!/usr/bin/env python3

import os
import re
import glob

def fix_invalid_core_paths():
    """Fix invalid crate::core:: module paths"""
    # Pattern to match invalid core module references
    invalid_core_pattern = r'crate::core::type_checker::(TypeChecker|Type)'
    
    # Find all Rust files with these references
    rust_files = glob.glob('src/**/*.rs', recursive=True)
    
    fixes_made = []
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            original_content = content
            
            # Replace invalid core module paths with correct type_system paths
            content = re.sub(r'use crate::core::type_checker::\{([^}]+)\};', r'use crate::type_system::\1;', content)
            content = re.sub(r'use crate::core::type_checker::([A-Za-z_][A-Za-z0-9_]*);', r'use crate::type_system::\1;', content)
            content = re.sub(r'crate::core::type_checker::(TypeChecker|Type)', r'crate::type_system::\1', content)
            
            # Fix performance pipeline references
            content = re.sub(r'use crate::core::performance_pipeline::PerformancePipeline;', 
                           r'use crate::optimization::performance_pipeline::PerformancePipeline;', content)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made.append(file_path)
                print(f"Fixed core module paths in: {file_path}")
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
            
    return fixes_made

def fix_invalid_inkwell_paths():
    """Fix invalid inkwell::crate:: module paths"""
    # Pattern to match invalid inkwell module references
    invalid_inkwell_pattern = r'inkwell::crate::'
    
    # Find all Rust files with these references
    rust_files = glob.glob('src/**/*.rs', recursive=True)
    
    fixes_made = []
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            original_content = content
            
            # Replace invalid inkwell::crate:: with correct inkwell::
            content = re.sub(r'inkwell::crate::', 'inkwell::', content)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made.append(file_path)
                print(f"Fixed inkwell paths in: {file_path}")
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
            
    return fixes_made

def fix_missing_runtime_modules():
    """Fix missing runtime module references"""
    # Create missing runtime modules if they don't exist
    runtime_modules = [
        'src/runtime/mod.rs',
        'src/runtime/stack.rs', 
        'src/runtime/value.rs'
    ]
    
    fixes_made = []
    
    for module_path in runtime_modules:
        if not os.path.exists(module_path):
            # Create minimal stub
            os.makedirs(os.path.dirname(module_path), exist_ok=True)
            
            if module_path.endswith('mod.rs'):
                content = '''/// Runtime system for CURSED
pub mod stack;
pub mod value;

// Advanced runtime features (disabled in minimal build)
#[cfg(feature = "advanced-runtime")]
pub mod goroutine;
#[cfg(feature = "advanced-runtime")]
pub mod debug_runtime;
#[cfg(feature = "advanced-runtime")]
pub mod process;
#[cfg(feature = "advanced-runtime")]
pub mod panic_system;

pub use stack::Stack;
pub use value::Value;
'''
            elif 'stack.rs' in module_path:
                content = '''/// Basic stack implementation for CURSED runtime
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Stack {
    frames: Vec<StackFrame>,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub variables: std::collections::HashMap<String, crate::runtime::value::Value>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }
    
    pub fn push_frame(&mut self, function_name: String) {
        self.frames.push(StackFrame {
            function_name,
            variables: std::collections::HashMap::new(),
        });
    }
    
    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
    
    pub fn current_frame(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }
    
    pub fn set_variable(&mut self, name: String, value: crate::runtime::value::Value) -> Result<(), Error> {
        if let Some(frame) = self.current_frame() {
            frame.variables.insert(name, value);
            Ok(())
        } else {
            Err(Error::Runtime("No active stack frame".to_string()))
        }
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&crate::runtime::value::Value> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.variables.get(name) {
                return Some(value);
            }
        }
        None
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
'''
            elif 'value.rs' in module_path:
                content = '''/// Runtime value representation for CURSED
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool", 
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => {
                let elements: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::Object(o) => {
                let pairs: Vec<String> = o.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", pairs.join(", "))
            }
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}
'''
            
            with open(module_path, 'w', encoding='utf-8') as f:
                f.write(content)
                
            fixes_made.append(module_path)
            print(f"Created missing runtime module: {module_path}")
            
    return fixes_made

def main():
    print("Fixing critical module resolution issues...")
    
    core_fixes = fix_invalid_core_paths()
    inkwell_fixes = fix_invalid_inkwell_paths() 
    runtime_fixes = fix_missing_runtime_modules()
    
    print(f"\nSummary:")
    print(f"- Fixed {len(core_fixes)} files with invalid core:: paths")
    print(f"- Fixed {len(inkwell_fixes)} files with invalid inkwell::crate:: paths")
    print(f"- Created {len(runtime_fixes)} missing runtime modules")
    
    total_fixes = len(core_fixes) + len(inkwell_fixes) + len(runtime_fixes)
    print(f"- Total: {total_fixes} fixes applied")
    
    if total_fixes > 0:
        print("\nRecommendation: Run 'cargo check' to verify the fixes.")

if __name__ == "__main__":
    main()
