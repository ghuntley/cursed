#!/usr/bin/env python3

import re
import os
import glob

def fix_pass_manager_builder_in_file(file_path):
    """Fix PassManagerBuilder imports and usage"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Remove PassManagerBuilder from imports
        content = re.sub(r', PassManagerBuilder', '', content)
        content = re.sub(r'PassManagerBuilder, ', '', content)
        content = re.sub(r'passes::\{PassManager, PassManagerBuilder\}', 'passes::PassManager', content)
        content = re.sub(r'passes::\{PassManagerBuilder, PassManager\}', 'passes::PassManager', content)
        
        # Remove standalone PassManagerBuilder imports
        content = re.sub(r'use inkwell::passes::PassManagerBuilder;\s*\n', '', content)
        
        # Replace PassManagerBuilder usage patterns with direct pass manager approach
        # Remove PassManagerBuilder::create() calls
        content = re.sub(r'let\s+builder\s*=\s*PassManagerBuilder::create\(\);\s*\n', '', content)
        content = re.sub(r'let\s+mut\s+builder\s*=\s*PassManagerBuilder::create\(\);\s*\n', '', content)
        
        # Remove builder configuration calls
        content = re.sub(r'\s*builder\.set_optimization_level\([^)]+\);\s*\n', '', content)
        content = re.sub(r'\s*builder\.set_size_level\([^)]+\);\s*\n', '', content)
        content = re.sub(r'\s*builder\.set_inliner_with_threshold\([^)]+\);\s*\n', '', content)
        content = re.sub(r'\s*builder\.set_disable_unroll_loops\([^)]+\);\s*\n', '', content)
        content = re.sub(r'\s*builder\.set_disable_unit_at_a_time\([^)]+\);\s*\n', '', content)
        content = re.sub(r'\s*builder\.populate_module_pass_manager\([^)]+\);\s*\n', '', content)
        content = re.sub(r'\s*builder\.populate_function_pass_manager\([^)]+\);\s*\n', '', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed PassManagerBuilder usage in: {file_path}")
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    """Fix PassManagerBuilder issues in all files"""
    
    rust_patterns = [
        'src/**/*.rs',
    ]
    
    files_fixed = 0
    total_files = 0
    
    for pattern in rust_patterns:
        for file_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(file_path):
                total_files += 1
                if fix_pass_manager_builder_in_file(file_path):
                    files_fixed += 1
    
    print(f"\nSummary:")
    print(f"  Total Rust files processed: {total_files}")
    print(f"  Files with PassManagerBuilder fixed: {files_fixed}")

if __name__ == "__main__":
    main()
