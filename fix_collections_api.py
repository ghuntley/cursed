#!/usr/bin/env python3
"""
Fix collections API inconsistencies by adding size() methods as aliases to len()
and making return types consistent.
"""

import re

def fix_collections_file(filename):
    """Fix a collections file by adding size() methods after len() methods."""
    
    with open(filename, 'r') as f:
        content = f.read()
    
    # Pattern to find len() method followed by anything but size() method
    len_pattern = r'(    /// Get the number of elements in the \w+\n    pub fn len\(&self\) -> usize \{\n        [^\n]+\n    \})\n(\n    /// (?!Alias for len))'
    
    # Replacement that adds size() method
    replacement = r'\1\n\n    /// Alias for len() for compatibility\n    pub fn size(&self) -> usize {\n        self.len()\n    }\2'
    
    # Apply the replacement
    content = re.sub(len_pattern, replacement, content)
    
    # For sets, also fix BitSet::new() return type
    if 'sets.rs' in filename:
        # Fix BitSet::new to return CollectionsResult<Self>
        content = re.sub(
            r'(impl BitSet \{\n    /// Create a new BitSet with specified number of bits\n    pub fn new\(max_bits: usize\)) -> Self \{',
            r'\1 -> CollectionsResult<Self> {',
            content
        )
        
        # Fix BitSet::new body to return Ok(Self { ... })
        content = re.sub(
            r'(        Ok\(Self \{\n            bits: vec!\[0; num_blocks\],\n            max_bits,\n        \})\)',
            r'\1',
            content
        )
    
    # For stacks, add size() methods to FixedStack and ThreadSafeStack
    if 'stacks.rs' in filename:
        # FixedStack
        content = re.sub(
            r'(    /// Get the number of elements in the fixed stack\n    pub fn len\(&self\) -> usize \{\n        self\.data\.len\(\)\n    \})\n(\n    /// Check if the stack is empty)',
            r'\1\n\n    /// Alias for len() for compatibility\n    pub fn size(&self) -> usize {\n        self.len()\n    }\2',
            content
        )
        
        # ThreadSafeStack
        content = re.sub(
            r'(    /// Get current stack size\n    pub fn len\(&self\) -> CollectionsResult<usize> \{\n[^}]+\})\n(\n    /// Check if stack is empty)',
            r'\1\n\n    /// Alias for len() for compatibility\n    pub fn size(&self) -> CollectionsResult<usize> {\n        self.len()\n    }\2',
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
            fix_collections_file(filename)
            print(f"✅ Fixed {filename}")
        except Exception as e:
            print(f"❌ Error fixing {filename}: {e}")
