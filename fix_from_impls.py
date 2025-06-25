#!/usr/bin/env python3

import os
import re

def fix_from_impls():
    """Fix From implementations with incorrect module paths"""
    
    # Find all rust files
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    fixes_made = 0
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix From implementations with incorrect module paths
            patterns = [
                # Fix std::io::CursedError -> std::io::Error
                (r'std::io::CursedError', 'std::io::Error'),
                (r'io::CursedError', 'std::io::Error'),
                
                # Fix external crate error references
                (r'serde_json::CursedError', 'serde_json::Error'),
                (r'rusqlite::CursedError', 'rusqlite::Error'),
                (r'toml::de::CursedError', 'toml::de::Error'),
                (r'toml::ser::CursedError', 'toml::ser::Error'),
                
                # Fix From implementations that use wrong paths
                (r'From<std::io::Error> for std::io::CursedError', 'From<std::io::Error> for crate::error_types::CursedError'),
                (r'From<serde_json::Error> for serde_json::CursedError', 'From<serde_json::Error> for crate::error_types::CursedError'),
                (r'From<rusqlite::Error> for rusqlite::CursedError', 'From<rusqlite::Error> for crate::error_types::CursedError'),
                (r'From<toml::de::Error> for toml::de::CursedError', 'From<toml::de::Error> for crate::error_types::CursedError'),
                (r'From<toml::ser::Error> for toml::ser::CursedError', 'From<toml::ser::Error> for crate::error_types::CursedError'),
            ]
            
            for pattern, replacement in patterns:
                content = re.sub(pattern, replacement, content)
            
            # Fix specific From implementation blocks
            # Fix impl From<std::io::Error> for std::io::CursedError
            from_block_pattern = r'impl From<([^>]+)> for ([^:]+::)+CursedError'
            matches = re.finditer(from_block_pattern, content)
            for match in matches:
                old_impl = match.group(0)
                source_type = match.group(1)
                new_impl = f'impl From<{source_type}> for crate::error_types::CursedError'
                content = content.replace(old_impl, new_impl)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made += 1
                print(f"Fixed From implementations in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"Fixed {fixes_made} files")

if __name__ == "__main__":
    fix_from_impls()
