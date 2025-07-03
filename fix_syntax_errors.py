#!/usr/bin/env python3

import os

def fix_syntax_errors():
    """Fix syntax errors in the files we modified"""
    
    files_to_fix = [
        'src/stdlib/packages/db_nosql/redis.rs',
        'src/stdlib/packages/db_nosql/mongodb.rs',
    ]
    
    for file_path in files_to_fix:
        if not os.path.exists(file_path):
            continue
        
        print(f"Fixing syntax in {file_path}...")
        
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Fix missing closing parentheses and semicolons
        content = content.replace(
            'CursedError::from(ModuleError::Other("Connection is closed".to_string()))',
            'CursedError::from(ModuleError::Other("Connection is closed".to_string())))'
        )
        
        # Fix any other similar patterns
        content = content.replace(
            'return Err(CursedError::from(ModuleError::Other(',
            'return Err(CursedError::from(ModuleError::Other('
        )
        
        # Fix specific broken lines
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if 'CursedError::from(ModuleError::Other(' in line and not line.strip().endswith(');'):
                if line.strip().endswith('))'):
                    lines[i] = line.replace('))', ')));')
                elif line.strip().endswith(')'):
                    lines[i] = line + ');'
        
        content = '\n'.join(lines)
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"Fixed syntax in {file_path}")

if __name__ == "__main__":
    fix_syntax_errors()
    
    # Test compilation
    import subprocess
    print("\nTesting compilation...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    error_count = result.stderr.count('error:')
    print(f"Compilation errors: {error_count}")
    
    if error_count > 0:
        print("\nFirst few errors:")
        for line in result.stderr.split('\n')[:20]:
            if line.strip():
                print(line)
