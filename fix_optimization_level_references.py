#!/usr/bin/env python3

import os
import re
import sys

def fix_optimization_level_references(filepath):
    """Fix OptimizationLevel enum references in a single file."""
    
    # Skip directories and binary files
    if os.path.isdir(filepath) or not filepath.endswith('.rs'):
        return False
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except (UnicodeDecodeError, PermissionError):
        print(f"Warning: Could not read {filepath}")
        return False
    
    original_content = content
    
    # Define replacement mappings
    replacements = {
        # Old enum variants -> New enum variants
        'OptimizationLevel::None': 'OptimizationLevel::O0',
        'OptimizationLevel::Less': 'OptimizationLevel::O1', 
        'OptimizationLevel::Basic': 'OptimizationLevel::O1',
        'OptimizationLevel::Default': 'OptimizationLevel::O2',
        'OptimizationLevel::Standard': 'OptimizationLevel::O2',
        'OptimizationLevel::Aggressive': 'OptimizationLevel::O3',
        'OptimizationLevel::Max': 'OptimizationLevel::O3',
        'OptimizationLevel::Size': 'OptimizationLevel::Os',
        'OptimizationLevel::SizeAggressive': 'OptimizationLevel::Oz',
        'OptimizationLevel::MinSize': 'OptimizationLevel::Oz',
        
        # Handle inkwell OptimizationLevel conversions separately
        'inkwell::OptimizationLevel::None': 'inkwell::OptimizationLevel::None',
        'inkwell::OptimizationLevel::Less': 'inkwell::OptimizationLevel::Less',
        'inkwell::OptimizationLevel::Default': 'inkwell::OptimizationLevel::Default',
        'inkwell::OptimizationLevel::Aggressive': 'inkwell::OptimizationLevel::Aggressive',
    }
    
    # Apply replacements
    for old_ref, new_ref in replacements.items():
        # Only replace if it's not already an inkwell reference
        if 'inkwell::' not in old_ref:
            content = content.replace(old_ref, new_ref)
    
    # Special case: fix common patterns in optimization level comparison/matching
    patterns = [
        # Fix patterns like: OptimizationLevel::Basic | OptimizationLevel::Default
        (r'OptimizationLevel::Basic\s*\|\s*OptimizationLevel::Default', 'OptimizationLevel::O1 | OptimizationLevel::O2'),
        (r'OptimizationLevel::Default\s*\|\s*OptimizationLevel::Aggressive', 'OptimizationLevel::O2 | OptimizationLevel::O3'),
        (r'OptimizationLevel::Aggressive\s*\|\s*OptimizationLevel::Size', 'OptimizationLevel::O3 | OptimizationLevel::Os'),
        (r'OptimizationLevel::Size\s*\|\s*OptimizationLevel::SizeAggressive', 'OptimizationLevel::Os | OptimizationLevel::Oz'),
        
        # Fix method calls on optimization level
        (r'OptimizationLevel::None\.default_config\(\)', 'OptimizationLevel::O0.default_config()'),
        (r'OptimizationLevel::Aggressive\.default_config\(\)', 'OptimizationLevel::O3.default_config()'),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)
    
    # Write back if changed
    if content != original_content:
        try:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Updated {filepath}")
            return True
        except PermissionError:
            print(f"Warning: Could not write to {filepath}")
            return False
    
    return False

def main():
    """Fix OptimizationLevel references in the src directory."""
    
    if len(sys.argv) > 1:
        # Process specific files if provided
        files_to_process = sys.argv[1:]
    else:
        # Process all .rs files in src directory
        files_to_process = []
        for root, dirs, files in os.walk('src'):
            for file in files:
                if file.endswith('.rs'):
                    files_to_process.append(os.path.join(root, file))
    
    updated_count = 0
    total_count = len(files_to_process)
    
    print(f"Processing {total_count} files...")
    
    for filepath in files_to_process:
        if fix_optimization_level_references(filepath):
            updated_count += 1
    
    print(f"\nCompleted: {updated_count}/{total_count} files updated")
    
    if updated_count > 0:
        print("\nOptimizationLevel enum references have been updated.")
        print("You may need to:")
        print("1. Update any remaining manual references")
        print("2. Run cargo check to verify compilation")
        print("3. Update imports if needed")

if __name__ == '__main__':
    main()
