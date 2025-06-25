#!/usr/bin/env python3

import os
import re

def comment_problematic_trait_impls():
    """Comment out problematic trait implementations causing E0405 and E0782"""
    
    problematic_patterns = [
        r'impl.*CursedError.*for.*{',
        r'impl.*std::error::Error.*for.*{',
        r'impl.*Error.*for.*CursedError.*{',
        r'impl.*From<.*CursedError.*>.*for.*{',
        r'impl.*Display.*for.*Error.*{',
        r'impl.*Debug.*for.*Error.*{',
        r'impl.*Default.*for.*TypeInference.*{',
    ]
    
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                
                with open(file_path, 'r') as f:
                    content = f.read()
                
                original_content = content
                lines = content.split('\n')
                modified = False
                in_problematic_impl = False
                brace_count = 0
                
                for i, line in enumerate(lines):
                    # Check if line starts a problematic impl block
                    found_problematic = False
                    for pattern in problematic_patterns:
                        if re.match(pattern, line.strip()) and not line.strip().startswith('//'):
                            in_problematic_impl = True
                            brace_count = line.count('{') - line.count('}')
                            lines[i] = '// ' + line
                            modified = True
                            found_problematic = True
                            break
                    
                    # If we're in a problematic impl block, comment out lines
                    if not found_problematic and in_problematic_impl:
                        if not line.strip().startswith('//'):
                            lines[i] = '// ' + line
                            modified = True
                        
                        # Track braces to know when impl block ends
                        brace_count += line.count('{') - line.count('}')
                        if brace_count <= 0:
                            in_problematic_impl = False
                            brace_count = 0
                
                if modified:
                    new_content = '\n'.join(lines)
                    with open(file_path, 'w') as f:
                        f.write(new_content)
                    print(f"Commented problematic impls in {file_path}")

if __name__ == '__main__':
    comment_problematic_trait_impls()
