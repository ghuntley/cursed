#!/usr/bin/env python3

import os
import re
import glob

def fix_error_imports(directory):
    """Fix common error import issues in Rust files."""
    
    # Find all .rs files
    rust_files = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    print(f"Found {len(rust_files)} Rust files")
    
    fixed_count = 0
    
    for filepath in rust_files:
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix common error import patterns
            
            # 1. Replace `use crate::error::Error` with `use crate::error_types::Error`
            content = re.sub(r'use crate::error::Error;', 'use crate::error_types::Error;', content)
            
            # 2. Replace `use crate::error::{Error,` with `use crate::error_types::{Error,`
            content = re.sub(r'use crate::error::\{Error,', 'use crate::error_types::{Error,', content)
            
            # 3. Replace `use crate::debug::{DebugConfig, SourceLocation}` with proper imports
            content = re.sub(
                r'use crate::debug::\{[^}]*SourceLocation[^}]*\};', 
                'use crate::error_types::SourceLocation;\nuse crate::debug::DebugConfig;', 
                content
            )
            
            # 4. Add missing Error import if file uses Error but doesn't import it
            if 'Result<' in content and '-> Result<' in content and 'use crate::error_types::Error;' not in content and 'Error>' in content:
                # Add import at the top after other imports
                lines = content.split('\n')
                insert_pos = 0
                for i, line in enumerate(lines):
                    if line.startswith('use ') or line.startswith('pub use '):
                        insert_pos = i + 1
                    elif line.strip() == '' and i > 0 and (lines[i-1].startswith('use ') or lines[i-1].startswith('pub use ')):
                        insert_pos = i
                        break
                
                if insert_pos > 0:
                    lines.insert(insert_pos, 'use crate::error_types::Error;')
                    content = '\n'.join(lines)
            
            # 5. Replace remaining Error usage that's clearly meant to be from error_types
            if 'use crate::error_types::Error;' in content:
                # Fix function signatures and return types
                content = re.sub(r'-> Result<([^,>]+), Error>', r'-> Result<\1, crate::error_types::Error>', content)
                content = re.sub(r'Result<([^,>]+), Error>', r'Result<\1, crate::error_types::Error>', content)
            
            # Only write if content changed
            if content != original_content:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_count += 1
                print(f"Fixed: {filepath}")
                
        except Exception as e:
            print(f"Error processing {filepath}: {e}")
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    fix_error_imports("src")
