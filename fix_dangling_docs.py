#!/usr/bin/env python3
import os
import re

def fix_dangling_doc_comment(file_path):
    """Fix files that end with dangling doc comments."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        if content.strip().endswith("/// fr fr Crypto utilities and helper functions"):
            package_name = os.path.basename(os.path.dirname(file_path))
            
            # Add utils module and init function
            addition = f'''
pub mod utils {{
    use super::*;
    
    /// slay Placeholder utility function
    pub fn placeholder() -> Result<(), Box<dyn std::error::Error>> {{
        Ok(())
    }}
}}

/// fr fr Initialize the {package_name} package
pub fn init_{package_name}() -> Result<(), Box<dyn std::error::Error>> {{
    println!("🔐 {package_name} package initialized - ready bestie!");
    Ok(())
}}
'''
            
            new_content = content + addition
            
            with open(file_path, 'w') as f:
                f.write(new_content)
            
            print(f"Fixed dangling doc comment in: {file_path}")
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

# Find all crypto mod.rs files
crypto_mod_files = []
for root, dirs, files in os.walk("src/stdlib/packages"):
    if "crypto_" in root and "mod.rs" in files:
        crypto_mod_files.append(os.path.join(root, "mod.rs"))

fixed_count = 0
for file_path in crypto_mod_files:
    if fix_dangling_doc_comment(file_path):
        fixed_count += 1

print(f"Fixed {fixed_count} files with dangling doc comments")
