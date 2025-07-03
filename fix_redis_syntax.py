#!/usr/bin/env python3

import os

def fix_redis_syntax():
    """Fix specific syntax errors in redis.rs"""
    
    file_path = 'src/stdlib/packages/db_nosql/redis.rs'
    
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Fix specific broken patterns
    content = content.replace(
        'CursedError::from(ModuleError::Other("Connection is closed".to_string()));)));',
        'CursedError::from(ModuleError::Other("Connection is closed".to_string())));'
    )
    
    content = content.replace(
        'CursedError::from(ModuleError::Other("No connections available".to_string()));)',
        'CursedError::from(ModuleError::Other("No connections available".to_string())))'
    )
    
    content = content.replace(
        'CursedError::from(ModuleError::Other("Module is disabled".to_string()));',
        'CursedError::from(ModuleError::Other("Module is disabled".to_string())));'
    )
    
    content = content.replace(
        'CursedError::from(ModuleError::Other("Module test failed".to_string()));',
        'CursedError::from(ModuleError::Other("Module test failed".to_string())));'
    )
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed syntax in {file_path}")

def fix_mongodb_syntax():
    """Fix specific syntax errors in mongodb.rs"""
    
    file_path = 'src/stdlib/packages/db_nosql/mongodb.rs'
    
    if not os.path.exists(file_path):
        return
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Fix specific broken patterns - similar to redis
    content = content.replace(
        'CursedError::from(ModuleError::Other("Connection is closed".to_string()));)));',
        'CursedError::from(ModuleError::Other("Connection is closed".to_string())));'
    )
    
    content = content.replace(
        'CursedError::from(ModuleError::Other("No connections available".to_string()));)',
        'CursedError::from(ModuleError::Other("No connections available".to_string())))'
    )
    
    content = content.replace(
        'CursedError::from(ModuleError::Other("Module is disabled".to_string()));',
        'CursedError::from(ModuleError::Other("Module is disabled".to_string())));'
    )
    
    content = content.replace(
        'CursedError::from(ModuleError::Other("Module test failed".to_string()));',
        'CursedError::from(ModuleError::Other("Module test failed".to_string())));'
    )
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed syntax in {file_path}")

if __name__ == "__main__":
    fix_redis_syntax()
    fix_mongodb_syntax()
    
    # Test compilation
    import subprocess
    print("\nTesting compilation...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    error_count = result.stderr.count('error:')
    print(f"Compilation errors: {error_count}")
    
    if error_count == 0:
        print("✅ All syntax errors fixed!")
    else:
        print("\nRemaining errors:")
        for line in result.stderr.split('\n')[:15]:
            if line.strip() and ('error:' in line or '-->' in line):
                print(line)
