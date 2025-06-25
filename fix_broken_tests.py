#!/usr/bin/env python3

import os
import re

def fix_broken_tests():
    """Fix broken test function syntax"""
    
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
            
            # Fix broken test functions specifically
            # Pattern: function header, followed by TODO on separate line, followed by closing brace
            pattern = r'(fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\([^)]*\)\s*(?:->[^{]+)?\s*{\s*)\n\s*//\s*TODO:\s*implement\s*\n\s*}'
            replacement = r'\1\n        // TODO: implement\n    }'
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
            
            # Fix other malformed patterns
            patterns = [
                # Fix test functions with broken indentation
                (r'(#\[test\]\s*fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(\s*\)\s*{\s*)\n\s*//\s*TODO:\s*implement\s*\n\s*}', 
                 r'\1\n        // TODO: implement\n    }'),
                
                # Fix regular functions with broken indentation
                (r'(pub\s+fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\([^)]*\)\s*(?:->[^{]+)?\s*{\s*)\n\s*//\s*TODO:\s*implement\s*\n\s*}', 
                 r'\1\n        // TODO: implement\n    }'),
            ]
            
            for pattern, replacement in patterns:
                content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made += 1
                print(f"Fixed broken tests in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"Fixed {fixes_made} files")

if __name__ == "__main__":
    fix_broken_tests()
