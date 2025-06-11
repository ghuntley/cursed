#!/usr/bin/env python3
"""
Fix internal API usage in the collections modules.
"""

import re

def fix_file(filename):
    """Fix internal API usage in a file."""
    
    with open(filename, 'r') as f:
        content = f.read()
    
    # Fix unused Result warnings by adding let _ = 
    patterns = [
        (r'(\s+)result\.insert\(([^)]+)\);', r'\1let _ = result.insert(\2);'),
        (r'(\s+)self\.enqueue\(([^)]+)\);', r'\1let _ = self.enqueue(\2);'),
        (r'(\s+)self\.push\(([^)]+)\);', r'\1let _ = self.push(\2);'),
        (r'(\s+)self\.push_back\(([^)]+)\);', r'\1let _ = self.push_back(\2);'),
        (r'(\s+)self\.push_front\(([^)]+)\);', r'\1let _ = self.push_front(\2);'),
        (r'(\s+)queue\.enqueue\(([^)]+)\);', r'\1let _ = queue.enqueue(\2);'),
        (r'(\s+)deque\.push_front\(([^)]+)\);', r'\1let _ = deque.push_front(\2);'),
        (r'(\s+)deque\.push_back\(([^)]+)\);', r'\1let _ = deque.push_back(\2);'),
        (r'(\s+)temp_queue\.enqueue\(([^)]+)\);', r'\1let _ = temp_queue.enqueue(\2);'),
        (r'(\s+)temp_queue\.push\(([^)]+)\);', r'\1let _ = temp_queue.push(\2);'),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)
    
    # Fix the specific test assertions
    if 'sets.rs' in filename:
        content = re.sub(
            r'assert!\(set\.insert\("([^"]+)"\.to_string\(\)\)\);',
            r'assert!(set.insert("\1".to_string()).unwrap());',
            content
        )
        content = re.sub(
            r'assert!\(!set\.insert\("([^"]+)"\.to_string\(\)\)\);',
            r'assert!(!set.insert("\1".to_string()).unwrap());',
            content
        )
    
    with open(filename, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    files = [
        'src/stdlib/collections/sets.rs',
        'src/stdlib/collections/stacks.rs',
        'src/stdlib/collections/queues.rs'
    ]
    
    for filename in files:
        try:
            fix_file(filename)
            print(f"✅ Fixed {filename}")
        except Exception as e:
            print(f"❌ Error fixing {filename}: {e}")
