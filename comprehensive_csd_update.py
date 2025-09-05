#!/usr/bin/env python3
"""
Comprehensive update of ALL remaining .💀 references to .💀 across all file types
"""

import os
import sys
import re
import glob
from typing import List, Tuple, Set

# File extensions to process
EXTENSIONS = ['.zig', '.py', '.sh', '.js', '.ts', '.json', '.toml', '.txt', '.c']

def should_skip_file(filepath: str) -> bool:
    """Check if file should be skipped"""
    skip_patterns = [
        'rename_extensions.py',  # Our own file
        'bulk_update_csd_to_skull.py',  # Our own file 
        'comprehensive_csd_update.py',  # This file
        '.git/',
        'zig-out/',
        '__pycache__/',
        '.zig-cache/',
        'node_modules/',
        '.vscode/ipch/',
    ]
    
    for pattern in skip_patterns:
        if pattern in filepath:
            return True
    return False

def update_file_content(filepath: str, content: str) -> Tuple[str, int]:
    """Update content replacing .💀 with .💀 with intelligent context awareness"""
    
    original_content = content
    updates = 0
    
    # Different patterns for different file types
    if filepath.endswith('.zig'):
        # Zig-specific patterns
        patterns = [
            (r'"([^"]*?)\.💀"', r'"\1.💀"'),  # String literals
            (r'([a-zA-Z_][a-zA-Z0-9_]*\.💀)', r'\1.💀'),  # Variable.💀
            (r'file\.💀', 'file.💀'),
            (r'main\.💀', 'main.💀'),
            (r'lib\.💀', 'lib.💀'),
            (r'mod\.💀', 'mod.💀'),
        ]
    
    elif filepath.endswith('.py'):
        # Python-specific patterns
        patterns = [
            (r'"([^"]*?)\.💀"', r'"\1.💀"'),  # Double quoted strings
            (r"'([^']*?)\.💀'", r"'\1.💀'"),  # Single quoted strings
            (r'\.💀\b', '.💀'),  # Basic extension
        ]
    
    elif filepath.endswith('.sh'):
        # Shell script patterns
        patterns = [
            (r'"([^"]*?)\.💀"', r'"\1.💀"'),  # Double quoted
            (r"'([^']*?)\.💀'", r"'\1.💀'"),  # Single quoted
            (r'([a-zA-Z0-9_.-]+)\.💀', r'\1.💀'),  # Unquoted filenames
        ]
    
    elif filepath.endswith(('.js', '.ts')):
        # JavaScript/TypeScript patterns
        patterns = [
            (r'"([^"]*?)\.💀"', r'"\1.💀"'),  # Double quoted
            (r"'([^']*?)\.💀'", r"'\1.💀'"),  # Single quoted
            (r'`([^`]*?)\.💀`', r'`\1.💀`'),  # Template literals
        ]
    
    elif filepath.endswith('.json'):
        # JSON patterns (be careful with escaping)
        patterns = [
            (r'"([^"]*?)\.💀"', r'"\1.💀"'),  # JSON strings
        ]
    
    elif filepath.endswith('.toml'):
        # TOML patterns
        patterns = [
            (r'"([^"]*?)\.💀"', r'"\1.💀"'),  # Double quoted
            (r"'([^']*?)\.💀'", r"'\1.💀'"),  # Single quoted
        ]
    
    elif filepath.endswith('.txt'):
        # Text files - be more conservative
        patterns = [
            (r'([a-zA-Z0-9_.-]+)\.💀\b', r'\1.💀'),  # Basic filenames
        ]
    
    else:
        # Generic patterns
        patterns = [
            (r'\.💀\b', '.💀'),
        ]
    
    # Apply all patterns
    for pattern, replacement in patterns:
        new_content = re.sub(pattern, replacement, content)
        if new_content != content:
            updates += len(re.findall(pattern, content))
            content = new_content
    
    return content, updates

def process_file(filepath: str) -> Tuple[bool, int]:
    """Process a single file"""
    try:
        # Read file
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if file has .💀 references
        if '.💀' not in content:
            return False, 0
        
        # Update content
        new_content, updates = update_file_content(filepath, content)
        
        # Write back if changed
        if new_content != content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True, updates
        
        return False, 0
        
    except Exception as e:
        print(f"❌ Error processing {filepath}: {e}")
        return False, 0

def main():
    print("🔄 CURSED Comprehensive Extension Update")
    print("📁 Updating ALL remaining .💀 references to .💀")
    print("🎯 Processing Zig, Python, Shell, JS/TS, JSON, TOML, and Text files")
    print("=" * 70)
    
    # Find all files to process
    files_to_process = []
    
    for root, dirs, files in os.walk('.'):
        # Skip certain directories
        dirs[:] = [d for d in dirs if not any(skip in d for skip in ['.git', '.zig-cache', 'zig-out', '__pycache__', 'node_modules'])]
        
        for file in files:
            if any(file.endswith(ext) for ext in EXTENSIONS):
                filepath = os.path.join(root, file)
                if not should_skip_file(filepath):
                    files_to_process.append(filepath)
    
    print(f"Found {len(files_to_process)} files to check")
    print()
    
    # Process files
    total_files_updated = 0
    total_updates = 0
    files_with_changes = []
    
    for filepath in sorted(files_to_process):
        updated, count = process_file(filepath)
        if updated:
            print(f"✅ {filepath}: {count} updates")
            files_with_changes.append((filepath, count))
            total_files_updated += 1
            total_updates += count
        elif count == 0:
            # Check if file actually contained .💀
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    if '.💀' in f.read():
                        print(f"⚠️  {filepath}: contains .💀 but no updates made")
            except:
                pass
    
    print("=" * 70)
    print(f"✅ Updated {total_files_updated} files")
    print(f"🔧 Made {total_updates} total replacements")
    
    if files_with_changes:
        print("\n📋 Summary of updated files:")
        for filepath, count in files_with_changes:
            print(f"   {filepath}: {count} changes")
    
    print("\n🎉 Comprehensive update complete!")

if __name__ == "__main__":
    main()
