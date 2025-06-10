#!/usr/bin/env python3

import re

def fix_channel_operations():
    with open('src/runtime/channels/operations.rs', 'r') as f:
        content = f.read()
    
    # Fix all channel field accesses
    content = re.sub(r'self\.channel\.', 'self.channel().', content)
    content = re.sub(r'self\.condvar\.', 'self.condvar().', content)
    content = re.sub(r'\(\*self\.operation_count\)', 'self.operation_count()', content)
    
    with open('src/runtime/channels/operations.rs', 'w') as f:
        f.write(content)
    
    print("Fixed channel field access in operations.rs")

if __name__ == "__main__":
    fix_channel_operations()
