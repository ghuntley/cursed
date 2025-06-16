#!/usr/bin/env python3

import re
import os
import glob

def fix_optimization_imports_in_file(file_path):
    """Fix EnhancedOptimizationStatistics imports"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix the import path
        content = re.sub(
            r'use super::EnhancedOptimizationStatistics;',
            'use crate::optimization::enhanced_llvm_passes_manager::EnhancedOptimizationStatistics;',
            content
        )
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed optimization imports in: {file_path}")
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    """Fix optimization import issues"""
    
    rust_patterns = [
        'src/optimization/**/*.rs',
    ]
    
    files_fixed = 0
    total_files = 0
    
    for pattern in rust_patterns:
        for file_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(file_path):
                total_files += 1
                if fix_optimization_imports_in_file(file_path):
                    files_fixed += 1
    
    print(f"\nSummary:")
    print(f"  Total optimization files processed: {total_files}")
    print(f"  Files with import issues fixed: {files_fixed}")

if __name__ == "__main__":
    main()
