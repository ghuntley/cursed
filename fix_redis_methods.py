#!/usr/bin/env python3

import re

def fix_redis_methods():
    with open('src/stdlib/packages/db_nosql/redis.rs', 'r') as f:
        content = f.read()
    
    # Pattern to match methods that need fixing
    methods_to_fix = [
        # Simple methods with single connection call
        (r'(pub async fn \w+\(&mut self[^{]*\{[^}]*?)let [^;]*;\s*let result[^=]*= self\.execute_with_timing\(\s*\|conn\| conn\.(\w+)\(([^)]*)\)\s*\)\.await\?;',
         r'\1let start = std::time::Instant::now();\n        let result = self.connection.\2(\3).await;\n        let duration = start.elapsed();\n        let result = self.update_stats_and_handle_error(result, duration).await?;'),
        
        # Methods that return the result directly
        (r'(pub async fn \w+\(&mut self[^{]*\{[^}]*?)let [^;]*;\s*self\.execute_with_timing\(\s*\|conn\| conn\.(\w+)\(([^)]*)\)\s*\)\.await',
         r'\1let start = std::time::Instant::now();\n        let result = self.connection.\2(\3).await;\n        let duration = start.elapsed();\n        self.update_stats_and_handle_error(result, duration).await'),
    ]
    
    for pattern, replacement in methods_to_fix:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'w') as f:
        f.write(content)

if __name__ == "__main__":
    fix_redis_methods()
    print("Fixed Redis method patterns")
