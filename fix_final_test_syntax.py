#!/usr/bin/env python3

import os
import re
import glob

def fix_file_content(content):
    """Fix syntax issues in test files"""
    
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Skip empty lines
        if not line.strip():
            fixed_lines.append(line)
            continue
            
        # Fix unterminated string literals with "fixed" at the end
        if line.endswith('"fixed"'):
            line = line[:-7] + '"'
            
        # Fix broken string concatenations 
        line = re.sub(r'"([^"]*?)"([^"]*?)"([^"]*?)"', r'"\1\2\3"', line)
        
        # Fix specific patterns like assert_eq with unterminated strings
        line = re.sub(r'assert_eq!\(([^,]+), ", ""fixed"\)', r'assert_eq!(\1, "")', line)
        line = re.sub(r'assert_eq!\(([^,]+), "([^"]*?)"([^"]*?)"([^"]*?)"', r'assert_eq!(\1, "\2\3\4")', line)
        
        # Fix mismatched closing delimiters
        line = re.sub(r'\{([^{}]*?)\]', r'{\1}', line)
        line = re.sub(r'\[([^[\]]*?)\}', r'[\1]', line)
        line = re.sub(r'\(([^()]*?)\}', r'(\1)', line)
        line = re.sub(r'\{([^{}]*?)\)', r'{\1}', line)
        
        # Fix unclosed quotes at line end
        if line.count('"') % 2 == 1 and not line.strip().endswith('\\'):
            line += '"'
        
        # Fix function calls with mismatched braces
        line = re.sub(r'(\w+\([^)]*?)\}', r'\1)', line)
        line = re.sub(r'(\w+\([^)]*?)\]', r'\1)', line)
        
        # Fix broken macro calls
        line = re.sub(r'(assert_eq!|assert!|println!|debug!|error!|panic!|format!)\(([^)]*?)""', r'\1(\2)', line)
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def main():
    """Fix syntax errors in test files"""
    
    # List of test files that specifically need fixing
    problem_files = [
        "tests/simple_lexer_test.rs",
        "tests/simple_llvm_test.rs", 
        "tests/simple_jit_test.rs",
        "tests/minimal_interface_test.rs"
    ]
    
    for file_path in problem_files:
        if not os.path.exists(file_path):
            continue
            
        print(f"Processing {file_path}...")
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            fixed_content = fix_file_content(content)
            
            # Write back the fixed content
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
                
            print(f"  ✓ Fixed {file_path}")
            
        except Exception as e:
            print(f"  ✗ Error processing {file_path}: {e}")

if __name__ == "__main__":
    main()
