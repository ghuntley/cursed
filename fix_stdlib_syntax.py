#!/usr/bin/env python3

import os
import re
import glob

def fix_stdlib_syntax():
    """Fix common syntax issues in stdlib modules"""
    
    # Find all stdlib modules
    modules = glob.glob("stdlib/*/mod.csd")
    
    fixed_count = 0
    error_count = 0
    
    for module_path in modules:
        print(f"Checking {module_path}...")
        
        try:
            with open(module_path, 'r') as f:
                content = f.read()
            
            original_content = content
            
            # Fix 1: Function declarations with incorrect return type syntax
            # Pattern: slay func_name() type {  -> should be: slay func_name() type {
            content = re.sub(
                r'slay\s+(\w+)\s*\([^)]*\)\s+(\w+)\s*\n\s*{',
                r'slay \1(\2) \3 {',
                content,
                flags=re.MULTILINE
            )
            
            # Fix 2: Ensure proper function body opening
            # Make sure { is on the same line as function declaration
            content = re.sub(
                r'slay\s+([^{]+?)\s*\n\s*{',
                r'slay \1 {',
                content,
                flags=re.MULTILINE
            )
            
            # Fix 3: Fix malformed variable declarations
            # sus var type = value -> proper syntax
            content = re.sub(
                r'sus\s+(\w+)\s+(\w+)\s*=\s*([^;]+)',
                r'sus \1 \2 = \3',
                content
            )
            
            if content != original_content:
                # Backup original
                backup_path = module_path + '.backup'
                if not os.path.exists(backup_path):
                    with open(backup_path, 'w') as f:
                        f.write(original_content)
                
                # Write fixed version
                with open(module_path, 'w') as f:
                    f.write(content)
                
                print(f"  ✅ Fixed syntax issues in {module_path}")
                fixed_count += 1
            else:
                print(f"  ✅ No issues found in {module_path}")
                
        except Exception as e:
            print(f"  ❌ Error processing {module_path}: {e}")
            error_count += 1
    
    print(f"\n📊 Summary:")
    print(f"  Fixed: {fixed_count} modules")
    print(f"  Errors: {error_count} modules")
    print(f"  Total checked: {len(modules)} modules")

if __name__ == "__main__":
    fix_stdlib_syntax()
