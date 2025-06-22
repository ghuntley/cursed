#!/usr/bin/env python3

import os
import re
import glob

def fix_optimization_level_imports():
    """Fix all OptimizationLevel imports to use the canonical source."""
    
    # Pattern to find imports from the old location
    old_import_pattern = r'use crate::optimization::optimization_config::OptimizationLevel'
    new_import = 'use crate::common::optimization_level::OptimizationLevel'
    
    # Find all Rust files
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    fixed_files = []
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Replace the old import pattern
            content = re.sub(old_import_pattern, new_import, content)
            
            # Also fix any aliased imports
            content = re.sub(
                r'use crate::optimization::optimization_config::OptimizationLevel as (\w+)',
                r'use crate::common::optimization_level::OptimizationLevel as \1',
                content
            )
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_files.append(file_path)
                print(f"Fixed imports in: {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return fixed_files

def main():
    print("🔧 Consolidating OptimizationLevel imports to canonical source...")
    fixed_files = fix_optimization_level_imports()
    
    if fixed_files:
        print(f"\n✅ Fixed imports in {len(fixed_files)} files:")
        for file_path in fixed_files:
            print(f"  - {file_path}")
    else:
        print("\n✅ No import fixes needed - all files already use canonical imports")
    
    print("\n🎯 OptimizationLevel consolidation complete!")

if __name__ == "__main__":
    main()
