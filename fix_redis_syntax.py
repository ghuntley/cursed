#!/usr/bin/env python3

import re

def fix_redis_syntax():
    """Fix the malformed syntax in Redis file"""
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'r') as f:
        content = f.read()
    
    # Fix malformed lines like: let result: Type = let start = ...
    pattern = r'let result:\s*([^=]+)=\s*let start = std::time::Instant::now\(\);\s*let result = ([^;]+);([^;]+);([^;]+);'
    def replace(match):
        result_type = match.group(1).strip()
        operation = match.group(2).strip()
        duration_line = match.group(3).strip()
        error_handling = match.group(4).strip()
        
        return f'''let start = std::time::Instant::now();
        let result: RedisResult<{result_type}> = {operation};
        {duration_line};
        let result = {error_handling}?;'''
    
    content = re.sub(pattern, replace, content)
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'w') as f:
        f.write(content)
    
    print("✅ Fixed Redis syntax errors")

if __name__ == '__main__':
    fix_redis_syntax()
