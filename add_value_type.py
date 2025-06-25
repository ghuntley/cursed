#!/usr/bin/env python3

import os

def add_value_type_definition():
    """Add Value type definition to common/mod.rs"""
    
    # Define Value type in common/mod.rs
    common_mod_path = 'src/common/mod.rs'
    
    if os.path.exists(common_mod_path):
        with open(common_mod_path, 'r') as f:
            content = f.read()
        
        # Add Value type definition if not present
        if 'pub enum Value {' not in content:
            value_definition = '''
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Object(obj) => write!(f, "{:?}", obj),
        }
    }
}
'''
            content += value_definition
            
            with open(common_mod_path, 'w') as f:
                f.write(content)
            print(f"Added Value type to {common_mod_path}")
    
    # Add Value export to lib.rs
    lib_path = 'src/lib.rs'
    if os.path.exists(lib_path):
        with open(lib_path, 'r') as f:
            content = f.read()
        
        if 'pub use common::Value;' not in content:
            # Find a good place to add the export
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.startswith('pub use error_types'):
                    lines.insert(i + 1, 'pub use common::Value;')
                    break
            
            new_content = '\n'.join(lines)
            with open(lib_path, 'w') as f:
                f.write(new_content)
            print(f"Added Value export to {lib_path}")

if __name__ == '__main__':
    add_value_type_definition()
