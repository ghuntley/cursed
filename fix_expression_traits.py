#!/usr/bin/env python3

import re
import os

def fix_expression_impl(file_path):
    """Fix Expression trait implementations to properly extend Node trait"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix imports to include Node trait
        content = re.sub(
            r'use crate::ast::traits::Expression;',
            r'use crate::ast::traits::{Expression, Node};',
            content
        )
        
        # Fix to_string method to string method
        content = re.sub(
            r'impl Expression for (\w+) \{\s*fn to_string\(&self\) -> String \{',
            r'impl Node for \1 {\n    fn string(&self) -> String {',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Add token_literal method after string method 
        content = re.sub(
            r'(    fn string\(&self\) -> String \{[^}]+\})',
            r'\1\n    \n    fn token_literal(&self) -> String {\n        String::new()\n    }\n}',
            content,
            flags=re.MULTILINE | re.DOTALL
        )
        
        # Fix Expression implementation
        content = re.sub(
            r'    \n    fn as_any\(&self\) -> &dyn Any \{\s*self\s*\}\s*\}',
            r'\n\nimpl Expression for \\1 {\n    fn as_any(&self) -> &dyn Any {\n        self\n    }\n    \n    fn clone_box(&self) -> Box<dyn Expression> {\n        Box::new(self.clone())\n    }\n}',
            content
        )
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed Expression trait implementation in {file_path}")
            return True
        else:
            print(f"No changes needed in {file_path}")
            return False
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    expression_files = [
        'src/ast/expressions/literal.rs',
        'src/ast/expressions/channel_ops.rs',
        'src/ast/expressions/goroutine_spawn.rs',
        'src/ast/expressions/error_propagation.rs',
        'src/ast/expressions/block.rs'
    ]
    
    for file_path in expression_files:
        if os.path.exists(file_path):
            fix_expression_impl(file_path)
        else:
            print(f"File not found: {file_path}")

if __name__ == "__main__":
    main()
