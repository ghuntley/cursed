#!/usr/bin/env python3

"""
Systematic fix for critical missing method compilation errors in core modules.

Fixes the following patterns:
1. Parser::new() should be Parser::from_source() 
2. Lexer::new() needs to be implemented or use existing constructor
3. LlvmCodeGenerator::new() should use default() or proper constructor
4. Fix any str::lines() method not found issues
"""

import os
import re
import subprocess
from pathlib import Path

def fix_parser_new_calls(content):
    """Fix Parser::new() calls to use correct constructor"""
    # Pattern 1: Parser::new(lexer) -> Parser::new(lexer) (this is correct)
    # Pattern 2: Parser::new(&mut lexer) -> Parser::new(lexer)
    content = re.sub(r'Parser::new\(&mut\s+([^)]+)\)', r'Parser::new(\1)', content)
    
    # Pattern 3: Parser::new(Lexer::new(...)) patterns that are malformed
    content = re.sub(r'Parser::new\(Lexer::new\(Lexer::new\([^)]+\)\.unwrap\(\)\)', 
                     r'Parser::new(Lexer::new(input.to_string()))', content)
    
    return content

def fix_lexer_new_calls(content):
    """Fix Lexer::new() calls to use correct String parameter"""
    # Pattern 1: Lexer::new("str") -> Lexer::new("str".to_string()) 
    content = re.sub(r'Lexer::new\("([^"]+)"\)', r'Lexer::new("\1".to_string())', content)
    
    # Pattern 2: Lexer::new(variable) where variable is &str -> Lexer::new(variable.to_string())
    # Only fix if the variable looks like it might be a string literal reference
    content = re.sub(r'Lexer::new\(([a-zA-Z_][a-zA-Z0-9_]*)\)(?!\s*\.)', 
                     lambda m: f'Lexer::new({m.group(1)}.to_string())' if not m.group(1).endswith('_string') else m.group(0), 
                     content)
    
    # Pattern 3: Lexer::new(input) where input might be &str
    # Be more conservative - only fix obvious cases
    content = re.sub(r'Lexer::new\((input|source|code)\)(?!\s*\.)', r'Lexer::new(\1.to_string())', content)
    
    return content

def fix_llvm_codegen_new_calls(content):
    """Fix LlvmCodeGenerator::new() calls"""
    # Pattern 1: LlvmCodeGenerator::new(&context, module, builder) -> LlvmCodeGenerator::new().unwrap()
    content = re.sub(r'LlvmCodeGenerator::new\(&[^,]+,\s*[^,]+,\s*[^)]+\)', 
                     'LlvmCodeGenerator::new().unwrap()', content)
    
    # Pattern 2: LlvmCodeGenerator::new(&context, "module") -> LlvmCodeGenerator::new().unwrap()
    content = re.sub(r'LlvmCodeGenerator::new\(&[^,]+,\s*"[^"]+"\)', 
                     'LlvmCodeGenerator::new().unwrap()', content)
    
    # Pattern 3: LlvmCodeGenerator::new(&context, &module) -> LlvmCodeGenerator::new().unwrap()
    content = re.sub(r'LlvmCodeGenerator::new\(&[^,]+,\s*&[^)]+\)', 
                     'LlvmCodeGenerator::new().unwrap()', content)
    
    return content

def fix_str_lines_issues(content):
    """Fix str::lines() method not found issues"""
    # Pattern: string.lines() -> string.split('\n')
    content = re.sub(r'([a-zA-Z_][a-zA-Z0-9_]*)\.lines\(\)', r'\1.split("\\n")', content)
    
    return content

def process_file(file_path):
    """Process a single file to fix compilation errors"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_parser_new_calls(content)
        content = fix_lexer_new_calls(content)
        content = fix_llvm_codegen_new_calls(content)
        content = fix_str_lines_issues(content)
        
        # Only write if changed
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        
        return False
    
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def find_rust_files():
    """Find all Rust files in the project"""
    rust_files = []
    
    # Main source directories
    for root in ['src', 'tests']:
        if os.path.exists(root):
            for path in Path(root).rglob('*.rs'):
                rust_files.append(str(path))
    
    return rust_files

def main():
    print("Fixing critical missing method compilation errors...")
    
    rust_files = find_rust_files()
    print(f"Found {len(rust_files)} Rust files")
    
    fixed_count = 0
    
    for file_path in rust_files:
        if process_file(file_path):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} files")
    
    # Run a quick compilation check to see if we've made progress
    print("\nRunning quick compilation check...")
    try:
        result = subprocess.run(['cargo', 'check', '--tests'], 
                              capture_output=True, text=True, timeout=60)
        if result.returncode == 0:
            print("✅ Basic compilation successful!")
        else:
            print("❌ Still have compilation errors:")
            # Show first few errors
            errors = result.stderr.split('\n')[:20]
            for error in errors:
                if error.strip():
                    print(f"  {error}")
    except Exception as e:
        print(f"Could not run compilation check: {e}")

if __name__ == "__main__":
    main()
