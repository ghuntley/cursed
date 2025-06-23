#!/usr/bin/env python3

import re

def fix_redis_timing_calls():
    """Fix all execute_with_timing calls in the Redis implementation"""
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'r') as f:
        content = f.read()
    
    # Pattern 1: Simple execute_with_timing calls that return a value directly
    # self.execute_with_timing(|conn| conn.method(args)).await
    pattern1 = r'self\.execute_with_timing\(\s*\|conn\|\s*conn\.(\w+)\(([^)]*)\)\s*\)\.await'
    def replace1(match):
        method = match.group(1)
        args = match.group(2)
        return f'''let start = std::time::Instant::now();
        let result = self.connection.{method}({args}).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await'''
    
    content = re.sub(pattern1, replace1, content)
    
    # Pattern 2: execute_with_timing calls with type annotation
    # let result: Type = self.execute_with_timing(|conn| conn.method(args)).await?;
    pattern2 = r'let result:\s*([^=]+)=\s*self\.execute_with_timing\(\s*\|conn\|\s*conn\.(\w+)\(([^)]*)\)\s*\)\.await\?;'
    def replace2(match):
        result_type = match.group(1).strip()
        method = match.group(2)
        args = match.group(3)
        return f'''let start = std::time::Instant::now();
        let result: RedisResult<{result_type}> = self.connection.{method}({args}).await;
        let duration = start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;'''
    
    content = re.sub(pattern2, replace2, content)
    
    # Pattern 3: execute_with_timing calls with redis::cmd
    # self.execute_with_timing(|conn| redis::cmd("CMD").query_async(conn)).await
    pattern3 = r'self\.execute_with_timing\(\s*\|conn\|\s*redis::cmd\("([^"]+)"\)\.query_async\(conn\)\s*\)\.await'
    def replace3(match):
        cmd = match.group(1)
        return f'''let start = std::time::Instant::now();
        let result = redis::cmd("{cmd}").query_async(&mut self.connection).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await'''
    
    content = re.sub(pattern3, replace3, content)
    
    # Pattern 4: execute_with_timing calls with cmd and args
    # self.execute_with_timing(|conn| cmd.query_async(conn)).await
    pattern4 = r'self\.execute_with_timing\(\s*\|conn\|\s*cmd\.query_async\(conn\)\s*\)\.await'
    def replace4(match):
        return f'''let start = std::time::Instant::now();
        let result = cmd.query_async(&mut self.connection).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await'''
    
    content = re.sub(pattern4, replace4, content)
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'w') as f:
        f.write(content)
    
    print("✅ Fixed all execute_with_timing calls in Redis implementation")

if __name__ == '__main__':
    fix_redis_timing_calls()
