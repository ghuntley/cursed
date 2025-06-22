#!/usr/bin/env python3

import os
import re

def fix_optimization_mod_duplicates():
    """Fix duplicate pgo module definitions"""
    mod_path = "src/optimization/mod.rs"
    
    with open(mod_path, 'r') as f:
        content = f.read()
    
    # Remove duplicate pgo module declarations
    lines = content.split('\n')
    seen_pgo = False
    filtered_lines = []
    
    for line in lines:
        if 'pub mod pgo;' in line:
            if not seen_pgo:
                filtered_lines.append(line)
                seen_pgo = True
            # Skip duplicate
        else:
            filtered_lines.append(line)
    
    with open(mod_path, 'w') as f:
        f.write('\n'.join(filtered_lines))

def fix_config_types_duplicates():
    """Fix duplicate struct definitions in config types"""
    types_path = "src/config/types.rs"
    
    with open(types_path, 'r') as f:
        content = f.read()
    
    # Find and remove duplicate CryptoParameters and SecurityContext
    # Keep the first occurrence, remove subsequent ones
    
    # Split into lines and track what we've seen
    lines = content.split('\n')
    filtered_lines = []
    seen_crypto_params = False
    seen_security_context = False
    skip_until_next_struct = False
    brace_count = 0
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        if skip_until_next_struct:
            if stripped.startswith('pub struct') or stripped.startswith('impl') or stripped.startswith('#[derive'):
                if brace_count == 0:
                    skip_until_next_struct = False
            else:
                # Count braces to know when struct definition ends
                brace_count += line.count('{') - line.count('}')
                if brace_count <= 0:
                    skip_until_next_struct = False
                continue
        
        if 'pub struct CryptoParameters' in line:
            if not seen_crypto_params:
                filtered_lines.append(line)
                seen_crypto_params = True
            else:
                skip_until_next_struct = True
                brace_count = line.count('{') - line.count('}')
                continue
        elif 'pub struct SecurityContext' in line:
            if not seen_security_context:
                filtered_lines.append(line)
                seen_security_context = True
            else:
                skip_until_next_struct = True
                brace_count = line.count('{') - line.count('}')
                continue
        else:
            filtered_lines.append(line)
    
    with open(types_path, 'w') as f:
        f.write('\n'.join(filtered_lines))

def check_for_other_duplicates():
    """Check for other duplicate definitions"""
    
    # Check common files for duplicates
    files_to_check = [
        "src/stdlib/mod.rs",
        "src/error/mod.rs", 
        "src/types/mod.rs",
    ]
    
    for file_path in files_to_check:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Look for duplicate pub mod declarations
            pub_mod_pattern = r'pub mod (\w+);'
            matches = re.findall(pub_mod_pattern, content)
            
            if len(matches) != len(set(matches)):
                print(f"Found duplicate pub mod declarations in {file_path}")
                
                # Remove duplicates
                lines = content.split('\n')
                seen_modules = set()
                filtered_lines = []
                
                for line in lines:
                    match = re.search(r'pub mod (\w+);', line)
                    if match:
                        module_name = match.group(1)
                        if module_name not in seen_modules:
                            filtered_lines.append(line)
                            seen_modules.add(module_name)
                        # Skip duplicates
                    else:
                        filtered_lines.append(line)
                
                with open(file_path, 'w') as f:
                    f.write('\n'.join(filtered_lines))
                
                print(f"Fixed duplicates in {file_path}")

def main():
    """Fix all duplicate definition issues"""
    print("Fixing duplicate definitions...")
    
    print("1. Fixing optimization module duplicates...")
    fix_optimization_mod_duplicates()
    
    print("2. Fixing config types duplicates...")
    fix_config_types_duplicates()
    
    print("3. Checking for other duplicates...")
    check_for_other_duplicates()
    
    print("All duplicate definition fixes completed!")

if __name__ == "__main__":
    main()
