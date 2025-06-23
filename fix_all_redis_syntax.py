#!/usr/bin/env python3

import re

def fix_all_redis_syntax():
    """Fix all malformed syntax in Redis file"""
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'r') as f:
        content = f.read()
    
    # Replace all instances of "let result: Type = let start = ..." pattern
    # with proper syntax
    lines = content.split('\n')
    fixed_lines = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check for the malformed pattern
        if 'let result:' in line and '= let start =' in line:
            # Extract the type annotation
            type_match = re.search(r'let result:\s*([^=]+)=', line)
            if type_match:
                result_type = type_match.group(1).strip()
                
                # Skip this malformed line and build the correct syntax
                fixed_lines.append(f'        let timer_start = std::time::Instant::now();')
                
                # Look for the next line with the actual operation
                i += 1
                if i < len(lines):
                    operation_line = lines[i].strip()
                    if operation_line.startswith('let result ='):
                        # Extract the operation
                        operation = operation_line.replace('let result = ', '').rstrip(';')
                        fixed_lines.append(f'        let result: RedisResult<{result_type}> = {operation};')
                        
                        # Add the duration line
                        i += 1
                        if i < len(lines):
                            duration_line = lines[i].strip()
                            if 'duration' in duration_line:
                                fixed_lines.append(f'        let duration = timer_start.elapsed();')
                                
                                # Add the error handling
                                i += 1
                                if i < len(lines):
                                    error_line = lines[i].strip()
                                    if 'update_stats_and_handle_error' in error_line:
                                        fixed_lines.append(f'        let result = self.update_stats_and_handle_error(result, duration).await?;')
                                    else:
                                        fixed_lines.append(lines[i])
                                else:
                                    break
                            else:
                                fixed_lines.append(lines[i])
                        else:
                            break
                    else:
                        fixed_lines.append(lines[i])
                else:
                    break
            else:
                fixed_lines.append(line)
        else:
            fixed_lines.append(line)
        
        i += 1
    
    content = '\n'.join(fixed_lines)
    
    with open('src/stdlib/packages/db_nosql/redis.rs', 'w') as f:
        f.write(content)
    
    print("✅ Fixed all Redis syntax errors")

if __name__ == '__main__':
    fix_all_redis_syntax()
