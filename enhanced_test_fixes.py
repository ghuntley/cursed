#!/usr/bin/env python3
"""
Enhanced CURSED Test Suite Validation and Fix Script

This script addresses remaining issues not handled by the initial validation.
"""

import os
import re
import sys
from pathlib import Path

class EnhancedCursedFixer:
    def __init__(self):
        self.fixes_applied = []
        
    def fix_remaining_issues(self, filepath: Path) -> bool:
        """Fix remaining issues not handled by the basic validator"""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"Failed to read {filepath}: {e}")
            return False
            
        original_content = content
        fixes_made = []
        
        # Fix remaining return statements inside functions
        content, return_fixes = self.fix_embedded_returns(content)
        fixes_made.extend(return_fixes)
        
        # Fix malformed import statements
        content, import_fixes = self.fix_malformed_imports(content)
        fixes_made.extend(import_fixes)
        
        # Fix remaining deprecated keyword issues
        content, deprecated_fixes = self.fix_remaining_deprecated(content)
        fixes_made.extend(deprecated_fixes)
        
        # Only write if changes were made
        if content != original_content:
            try:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                    
                if fixes_made:
                    self.fixes_applied.append({
                        'file': str(filepath),
                        'fixes': fixes_made
                    })
                return True
                
            except Exception as e:
                print(f"Failed to write {filepath}: {e}")
                return False
                
        return True
        
    def fix_embedded_returns(self, content: str) -> tuple:
        """Fix return statements that appear inside function bodies"""
        fixes = []
        
        # More comprehensive return statement replacement
        # This handles cases where 'return' might be indented or in middle of line
        patterns = [
            (r'^(\s*)return\b(.*?)$', r'\1damn\2'),  # Indented return statements
            (r'\breturn\b(?!\w)', 'damn'),  # Any return not part of another word
        ]
        
        for pattern, replacement in patterns:
            if re.search(pattern, content, re.MULTILINE):
                content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
                fixes.append(f"Fixed embedded return statements with '{replacement}'")
                
        return content, fixes
        
    def fix_malformed_imports(self, content: str) -> tuple:
        """Fix malformed import statements"""
        fixes = []
        
        # Fix imports that don't follow proper yeet syntax
        # Look for lines that have 'yeet' but malformed syntax
        lines = content.split('\n')
        fixed_lines = []
        
        for line in lines:
            original_line = line
            stripped = line.strip()
            
            # Fix various malformed import patterns
            if 'yeet' in stripped and not re.match(r'^\s*yeet\s+"[^"]+"', stripped):
                # Handle cases like: yeet math, string, vibez
                if re.match(r'^\s*yeet\s+\w+', stripped):
                    # Extract module names and fix syntax
                    modules_match = re.search(r'yeet\s+(.+?)(?:\s*$|\s*//|\s*#)', stripped)
                    if modules_match:
                        modules_text = modules_match.group(1).strip()
                        # Split on commas and clean up module names
                        modules = [m.strip() for m in modules_text.split(',')]
                        if len(modules) == 1 and not modules[0].startswith('"'):
                            # Single module without quotes
                            indent = len(line) - len(line.lstrip())
                            line = ' ' * indent + f'yeet "{modules[0]}"'
                            fixes.append(f'Fixed malformed import: {original_line.strip()} -> {line.strip()}')
                        elif len(modules) > 1:
                            # Multiple modules - convert to multiple yeet statements
                            indent = len(line) - len(line.lstrip())
                            new_lines = []
                            for module in modules:
                                module = module.strip().strip('"')
                                new_lines.append(' ' * indent + f'yeet "{module}"')
                            line = '\n'.join(new_lines)
                            fixes.append(f'Fixed multi-module import: {original_line.strip()} -> multiple yeet statements')
                            
            fixed_lines.append(line)
            
        content = '\n'.join(fixed_lines)
        return content, fixes
        
    def fix_remaining_deprecated(self, content: str) -> tuple:
        """Fix remaining deprecated keyword issues"""
        fixes = []
        
        # Handle cap that might be in expressions or complex contexts
        if re.search(r'\bcap\b', content):
            # More comprehensive cap replacement
            content = re.sub(r'\bcap\b(?!\w)', 'cringe', content)
            fixes.append("Fixed remaining 'cap' -> 'cringe' replacements")
            
        return content, fixes

def main():
    test_programs_dir = Path("/home/ghuntley/cursed/test_suite/test_programs")
    fixer = EnhancedCursedFixer()
    
    if not test_programs_dir.exists():
        print(f"Test programs directory not found: {test_programs_dir}")
        return 1
        
    # Find all .csd files
    csd_files = []
    for root, dirs, files in os.walk(test_programs_dir):
        for file in files:
            if file.endswith('.csd'):
                csd_files.append(Path(root) / file)
                
    if not csd_files:
        print("No .csd files found in test programs directory")
        return 1
        
    print(f"Applying enhanced fixes to {len(csd_files)} .csd files")
    print("=" * 60)
    
    files_processed = 0
    files_with_fixes = 0
    
    # Apply enhanced fixes to each file
    for filepath in sorted(csd_files):
        print(f"🔧 Processing: {filepath.relative_to(test_programs_dir)}")
        
        if fixer.fix_remaining_issues(filepath):
            files_processed += 1
            
    files_with_fixes = len(fixer.fixes_applied)
            
    print("\n" + "=" * 60)
    print(f"ENHANCED FIX SUMMARY:")
    print(f"Files processed: {files_processed}")
    print(f"Files with additional fixes: {files_with_fixes}")
    
    if fixer.fixes_applied:
        print(f"\nDETAILED ENHANCED FIX REPORT:")
        for fix_info in fixer.fixes_applied:
            print(f"\n📄 {Path(fix_info['file']).name}:")
            for fix in fix_info['fixes']:
                print(f"   • {fix}")
                
    return 0

if __name__ == "__main__":
    sys.exit(main())
