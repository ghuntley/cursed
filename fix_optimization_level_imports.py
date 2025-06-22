#!/usr/bin/env python3

import os
import re
import sys

def fix_optimization_level_imports(file_path):
    """Fix OptimizationLevel imports in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern 1: Fix imports that use old path patterns
        patterns_to_fix = [
            # Remove OptimizationLevel from existing imports and add canonical import
            (r'use crate::optimization::\{([^}]*), OptimizationLevel([^}]*)\};', 
             r'use crate::optimization::{\1\2};\nuse crate::optimization::config::OptimizationLevel;'),
            
            (r'use crate::codegen::llvm::optimization::\{([^}]*), OptimizationLevel([^}]*)\};',
             r'use crate::codegen::llvm::optimization::{\1\2};\nuse crate::optimization::config::OptimizationLevel;'),
             
            (r'use crate::build_system::\{([^}]*), OptimizationLevel([^}]*)\};',
             r'use crate::build_system::{\1\2};\nuse crate::optimization::config::OptimizationLevel;'),
            
            # Fix standalone imports
            (r'use crate::optimization::OptimizationLevel;',
             r'use crate::optimization::config::OptimizationLevel;'),
             
            (r'use crate::codegen::llvm::optimization::OptimizationLevel;',
             r'use crate::optimization::config::OptimizationLevel;'),
             
            # Fix super imports in passes
            (r'use super::\{([^}]*), OptimizationLevel([^}]*)\};',
             r'use super::{\1\2};\nuse crate::optimization::config::OptimizationLevel;'),
             
            # Fix imports with "as" aliases
            (r'use ([^;]*), OptimizationLevel as ([^,;}]*)',
             r'use \1\nuse crate::optimization::config::OptimizationLevel as \2'),
        ]
        
        for pattern, replacement in patterns_to_fix:
            content = re.sub(pattern, replacement, content)
        
        # Clean up any duplicated imports that might have been created
        lines = content.split('\n')
        new_lines = []
        optimization_level_imports = set()
        
        for line in lines:
            # Track canonical OptimizationLevel imports to avoid duplicates
            if 'use crate::optimization::config::OptimizationLevel' in line:
                if line not in optimization_level_imports:
                    optimization_level_imports.add(line)
                    new_lines.append(line)
            else:
                new_lines.append(line)
        
        content = '\n'.join(new_lines)
        
        # Only write if content changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed imports in: {file_path}")
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def find_rust_files(directory):
    """Find all Rust files in the directory"""
    rust_files = []
    for root, dirs, files in os.walk(directory):
        # Skip target, .git, and other build directories
        dirs[:] = [d for d in dirs if not d.startswith('.') and d not in ['target', 'build']]
        
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    return rust_files

def main():
    src_dir = 'src'
    if not os.path.exists(src_dir):
        print("Error: src directory not found")
        sys.exit(1)
    
    rust_files = find_rust_files(src_dir)
    print(f"Found {len(rust_files)} Rust files")
    
    fixed_count = 0
    for file_path in rust_files:
        if fix_optimization_level_imports(file_path):
            fixed_count += 1
    
    print(f"Fixed imports in {fixed_count} files")

if __name__ == "__main__":
    main()
