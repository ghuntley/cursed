#!/usr/bin/env python3

import os
import re
import glob

def fix_error_imports():
    """Fix all error import statements to use correct module path"""
    
    # Find all rust files
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    fixes_made = 0
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix various import patterns
            patterns = [
                # Fix crate::error::CursedError -> crate::error_types::CursedError
                (r'use crate::error::CursedError;', 'use crate::error_types::CursedError;'),
                (r'use crate::error::\{CursedError, CursedError\};', 'use crate::error_types::CursedError;'),
                (r'use crate::error::\{CursedError, CursedError, SourceLocation\};', 'use crate::error_types::{CursedError, SourceLocation};'),
                (r'use crate::error::\{CursedError as CursedError, CursedError\};', 'use crate::error_types::CursedError;'),
                (r'use crate::error::\{CursedError as CursedError, CursedError, SourceLocation\};', 'use crate::error_types::{CursedError, SourceLocation};'),
                (r'use crate::error::\{CursedError as CursedError, SourceLocation\};', 'use crate::error_types::{CursedError, SourceLocation};'),
                (r'use crate::error::\{CursedError, Result\};', 'use crate::error_types::{CursedError, Result};'),
                (r'use crate::error::\{CursedError, Result as CursedResult\};', 'use crate::error_types::{CursedError, Result as CursedResult};'),
                (r'use crate::error::\{Result, CursedError\};', 'use crate::error_types::{CursedError, Result};'),
                (r'use crate::error::SourceLocation;', 'use crate::error_types::SourceLocation;'),
                (r'use crate::error::Result;', 'use crate::error_types::Result;'),
                (r'use crate::error::SourceLocation as ErrorSourceLocation;', 'use crate::error_types::SourceLocation as ErrorSourceLocation;'),
                (r'use crate::error::Result as CursedResult;', 'use crate::error_types::Result as CursedResult;'),
                
                # Fix crate::error::Error -> crate::error_types::CursedError
                (r'use crate::error::Error;', 'use crate::error_types::CursedError;'),
                (r'use crate::error::Error as CursedError;', 'use crate::error_types::CursedError;'),
                (r'use crate::error::\{Error, Result\};', 'use crate::error_types::{CursedError, Result};'),
                
                # Complex patterns
                (r'use crate::error::\{CursedError, ErrorPropagationError, SourceLocation\};', 'use crate::error_types::{CursedError, SourceLocation};'),
            ]
            
            for pattern, replacement in patterns:
                content = re.sub(pattern, replacement, content)
            
            # Fix any remaining crate::error:: to crate::error_types::
            content = re.sub(r'crate::error::', 'crate::error_types::', content)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made += 1
                print(f"Fixed imports in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"Fixed {fixes_made} files")

if __name__ == "__main__":
    fix_error_imports()
