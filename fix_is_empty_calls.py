#!/usr/bin/env python3

import os
import re
import subprocess
import sys

def fix_is_empty_calls():
    """Fix is_empty() calls that are missing trait imports by replacing with .len() == 0"""
    
    # Get the files that have is_empty errors
    cmd = ["./fix_linking.sh", "cargo", "check"]
    process = subprocess.run(cmd, capture_output=True, text=True, cwd="/home/ghuntley/code/cursed")
    
    error_files = set()
    lines = process.stderr.split('\n')
    
    for i, line in enumerate(lines):
        if "no method named `is_empty`" in line:
            # Look for the file reference in surrounding lines
            for j in range(max(0, i-5), min(len(lines), i+5)):
                if "-->" in lines[j] and ".rs:" in lines[j]:
                    file_path = lines[j].split("-->")[1].strip().split(":")[0]
                    if file_path.startswith("src/"):
                        error_files.add(file_path)
                    break
    
    print(f"Found {len(error_files)} files with is_empty() errors:")
    for f in error_files:
        print(f"  {f}")
    
    # Fix each file
    for file_path in error_files:
        if os.path.exists(file_path):
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Replace .is_empty() with .len() == 0
                # Be careful to only replace on collections/slices, not strings
                original_content = content
                
                # Pattern to match .is_empty() calls
                patterns = [
                    (r'\.is_empty\(\)', '.len() == 0'),
                    (r'!([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)\.is_empty\(\)', r'!\1.len() == 0'),
                ]
                
                for pattern, replacement in patterns:
                    content = re.sub(pattern, replacement, content)
                
                if content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    print(f"Fixed is_empty() calls in {file_path}")
                
            except Exception as e:
                print(f"Error processing {file_path}: {e}")

if __name__ == "__main__":
    fix_is_empty_calls()
