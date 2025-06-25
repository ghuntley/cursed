#!/usr/bin/env python3

import os
import re

def fix_todo_functions():
    """Fix TODO function implementations that have syntax errors"""
    
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
            
            # Fix TODO function implementations with syntax errors
            patterns = [
                # Fix functions with empty TODO bodies
                (r'fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)\s*\{\s*/\*\s*TODO:\s*implement\s*\*/\s*\}', r'fn \1() {\n    // TODO: implement\n}'),
                (r'fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)\s*->\s*([^{]+)\s*\{\s*/\*\s*TODO:\s*implement\s*\*/\s*\}', r'fn \1() -> \2 {\n    // TODO: implement\n    panic!("Not implemented")\n}'),
                
                # Fix malformed test functions
                (r'#\[test\]\s*fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(\s*\)\s*\{\s*/\*\s*TODO:\s*implement\s*\*/\s*\}', r'#[test]\nfn \1() {\n    // TODO: implement\n}'),
            ]
            
            for pattern, replacement in patterns:
                content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            
            # Fix specific broken function syntax
            content = re.sub(r'fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(\s*\)\s*\{\s*/\*\s*TODO:\s*implement\s*\*/\s*\}', 
                           r'fn \1() {\n    // TODO: implement\n}', content)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made += 1
                print(f"Fixed TODO functions in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"Fixed {fixes_made} files")

if __name__ == "__main__":
    fix_todo_functions()
