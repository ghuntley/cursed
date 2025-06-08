#!/usr/bin/env python3
"""Find mismatched braces in Rust source files."""

import sys

def check_braces(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()
    
    depth = 0
    line_depths = []
    
    for i, line in enumerate(lines):
        line_num = i + 1
        opens = line.count('{')
        closes = line.count('}')
        
        depth += opens - closes
        line_depths.append((line_num, depth, opens, closes, line.strip()))
        
        if depth < 0:
            print(f"Line {line_num}: Unexpected closing brace (depth: {depth})")
            print(f"  {line.strip()}")
            return False
    
    print(f"Final depth: {depth}")
    
    if depth != 0:
        print("\nLines with brace changes:")
        for line_num, d, opens, closes, text in line_depths[-20:]:
            if opens > 0 or closes > 0:
                print(f"Line {line_num:4d}: depth={d:2d} (+{opens} -{closes}) {text}")
        
        # Find the last few functions
        print("\nLast few function definitions:")
        for line_num, d, opens, closes, text in line_depths:
            if 'fn ' in text and 'pub' in text:
                print(f"Line {line_num:4d}: {text}")
                
        return False
    
    return True

if __name__ == "__main__":
    filename = sys.argv[1] if len(sys.argv) > 1 else "src/parser/statements.rs"
    check_braces(filename)
