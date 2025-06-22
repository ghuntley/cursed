#!/usr/bin/env python3

import re

def fix_redis_file():
    with open('src/stdlib/packages/db_nosql/redis.rs', 'r') as f:
        content = f.read()
    
    # Pattern to match method calls that need fixing
    patterns = [
        # Pattern for methods that take a key parameter
        (r'self\.execute_with_timing\(\s*self\.connection\.(\w+)\(([^)]+)\)\s*\)\.await', 
         lambda m: f'self.execute_with_timing(\n            |conn| conn.{m.group(1)}({m.group(2)})\n        ).await'),
        
        # Pattern for Redis commands using redis::cmd
        (r'self\.execute_with_timing\(\s*redis::cmd\("([^"]+)"\)\.query_async\(&mut self\.connection\)\s*\)\.await',
         lambda m: f'self.execute_with_timing(\n            |conn| redis::cmd("{m.group(1)}").query_async(conn)\n        ).await'),
        
        # Pattern for Redis commands with arguments
        (r'self\.execute_with_timing\(\s*redis::cmd\("([^"]+)"\)\.arg\(([^)]+)\)\.query_async\(&mut self\.connection\)\s*\)\.await',
         lambda m: f'self.execute_with_timing(\n            |conn| redis::cmd("{m.group(1)}").arg({m.group(2)}).query_async(conn)\n        ).await'),
    ]
    
    # Apply patterns
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    
    # Fix specific method patterns that require parameter ownership
    specific_fixes = [
        # LPUSH/RPUSH operations
        (r'let redis_values: Vec<String> = values\.iter\(\)\.map\(value_to_redis_string\)\.collect\(\);\s*self\.execute_with_timing\(\s*\|conn\| conn\.(\w+)\(key, redis_values\)\s*\)\.await',
         lambda m: f'let redis_values: Vec<String> = values.iter().map(value_to_redis_string).collect();\n        let key = key.to_string();\n        self.execute_with_timing(\n            |conn| conn.{m.group(1)}(key, redis_values)\n        ).await'),
        
        # SADD/SREM operations
        (r'let redis_values: Vec<String> = members\.iter\(\)\.map\(value_to_redis_string\)\.collect\(\);\s*self\.execute_with_timing\(\s*\|conn\| conn\.(\w+)\(key, redis_values\)\s*\)\.await',
         lambda m: f'let redis_values: Vec<String> = members.iter().map(value_to_redis_string).collect();\n        let key = key.to_string();\n        self.execute_with_timing(\n            |conn| conn.{m.group(1)}(key, redis_values)\n        ).await'),
        
        # HSET operations
        (r'let redis_value = value_to_redis_string\(value\);\s*let result: i32 = self\.execute_with_timing\(\s*\|conn\| conn\.hset\(key, field, redis_value\)\s*\)\.await\?;',
         'let redis_value = value_to_redis_string(value);\n        let key = key.to_string();\n        let field = field.to_string();\n        let result: i32 = self.execute_with_timing(\n            |conn| conn.hset(key, field, redis_value)\n        ).await?;'),
        
        # SISMEMBER operations  
        (r'let redis_value = value_to_redis_string\(member\);\s*let result: bool = self\.execute_with_timing\(\s*\|conn\| conn\.sismember\(key, redis_value\)\s*\)\.await\?;',
         'let redis_value = value_to_redis_string(member);\n        let key = key.to_string();\n        let result: bool = self.execute_with_timing(\n            |conn| conn.sismember(key, redis_value)\n        ).await?;'),
    ]
    
    for pattern, replacement in specific_fixes:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    
    # Add parameter ownership for all remaining key parameters
    content = re.sub(r'(pub async fn \w+\(&mut self[^{]*\{[^}]*?)(\s*)(self\.execute_with_timing\(\s*\|conn\| conn\.\w+\()(key)(\)|\,)', 
                     lambda m: m.group(1) + '\n        let key = key.to_string();' + m.group(2) + m.group(3) + 'key' + m.group(5), 
                     content, flags=re.MULTILINE | re.DOTALL)
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'w') as f:
        f.write(content)

if __name__ == "__main__":
    fix_redis_file()
    print("Fixed Redis borrowing issues")
