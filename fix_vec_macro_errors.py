#!/usr/bin/env python3

import os
import re
import subprocess

def get_affected_files():
    """Get all files with vec! macro into_vec errors"""
    try:
        result = subprocess.run(
            ['cargo', 'check', '--message-format=json'],
            capture_output=True,
            text=True,
            cwd='.'
        )
        
        affected_files = set()
        for line in result.stderr.split('\n'):
            if 'into_vec' in line and 'not found for slice' in line:
                # Extract file path from error message
                match = re.search(r'-->\s+([^:]+):\d+', line)
                if match:
                    file_path = match.group(1)
                    if file_path.startswith('src/'):
                        affected_files.add(file_path)
        
        return list(affected_files)
    except Exception as e:
        print(f"Error getting affected files: {e}")
        return []

def fix_vec_macro_in_file(file_path):
    """Fix vec! macro issues in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern 1: vec![item1, item2, item3] patterns that are problematic
        # Look for vec! followed by array-like syntax
        
        # Fix simple vec! patterns - convert to proper Vec construction
        patterns_to_fix = [
            # Pattern: vec![Version::parse("x.x.x").unwrap(), ...]
            (r'vec!\[\s*Version::parse\([^]]+\)\s*\.unwrap\(\)\s*,\s*Version::parse\([^]]+\)\s*\.unwrap\(\)\s*\]',
             lambda m: f'vec![{m.group(0)[5:-1]}]'),
            
            # Pattern: vec!["str1", "str2", ...]  
            (r'vec!\[(\s*"[^"]*"\s*(?:,\s*"[^"]*"\s*)*)\]',
             lambda m: f'vec![{m.group(1)}]'),
             
            # Pattern: vec![expr, expr, ...] where exprs are simple
            (r'vec!\[([^[\]{}]+)\]', 
             lambda m: f'vec![{m.group(1)}]'),
        ]
        
        # Apply fixes
        for pattern, replacement in patterns_to_fix:
            if callable(replacement):
                content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
            else:
                content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
        
        # Check if we need to replace with Vec::new() + push pattern or array.to_vec()
        # Look for patterns that suggest we need different approaches
        
        # For really problematic cases, try a different approach
        # Look for the specific error locations and fix them individually
        
        # If content changed, write it back
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed vec! macro issues in {file_path}")
            return True
        else:
            print(f"No changes needed in {file_path}")
            return False
            
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def fix_specific_problematic_patterns():
    """Fix specific known problematic patterns"""
    
    # Define specific fixes for known patterns
    specific_fixes = {
        'src/package_manager/resolver.rs': [
            (r'vec!\[\s*Version::parse\("0\.1\.0"\)\.unwrap\(\),\s*Version::parse\("1\.0\.0"\)\.unwrap\(\),?\s*\]',
             'vec![Version::parse("0.1.0").unwrap(), Version::parse("1.0.0").unwrap()]'),
        ],
        'src/stdlib/glyph_gang/emoji.rs': [
            # Convert vec!["str", "str", ...] to proper format
            (r'vec!\[(\s*"[^"]*"\s*(?:,\s*"[^"]*"\s*)*)\]',
             r'vec![\1]'),
        ],
    }
    
    for file_path, fixes in specific_fixes.items():
        if os.path.exists(file_path):
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                original_content = content
                
                for pattern, replacement in fixes:
                    content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
                
                if content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    print(f"Applied specific fixes to {file_path}")
                    
            except Exception as e:
                print(f"Error applying specific fixes to {file_path}: {e}")

def main():
    print("Fixing vec! macro compilation errors...")
    
    # First, apply specific known fixes
    fix_specific_problematic_patterns()
    
    # Get all affected files
    affected_files = get_affected_files()
    
    if not affected_files:
        print("No affected files found, trying to compile to see current status...")
        subprocess.run(['cargo', 'check'], cwd='.')
        return
    
    print(f"Found {len(affected_files)} affected files")
    
    # Fix each file
    fixed_count = 0
    for file_path in affected_files:
        if fix_vec_macro_in_file(file_path):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")
    
    # Test the fix
    print("Testing fixes...")
    result = subprocess.run(['cargo', 'check'], capture_output=True, text=True, cwd='.')
    
    if result.returncode == 0:
        print("✅ All vec! macro errors fixed!")
    else:
        print("❌ Some errors remain, checking what's left...")
        # Show remaining into_vec errors
        for line in result.stderr.split('\n'):
            if 'into_vec' in line:
                print(f"  Remaining error: {line.strip()}")

if __name__ == "__main__":
    main()
