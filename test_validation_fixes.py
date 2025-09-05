#!/usr/bin/env python3
"""
CURSED Test Suite Validation and Fix Script

This script validates all test programs against CURSED language specifications
and fixes invalid syntax according to the 8-point validation checklist.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Tuple

class CursedValidator:
    def __init__(self):
        # Valid CURSED keywords based on specs
        self.keywords = {
            'vibe',           # package declaration
            'yeet',           # import
            'slay',           # function declaration
            'damn',           # return (canonical)
            'yolo',           # return (deprecated)
            'sus',            # variable declaration
            'facts',          # constants
            'ready',          # if statement  
            'otherwise',      # else statement
            'bestie',         # for loop
            'periodt',        # while loop
            'vibe_check',     # switch statement
            'mood',           # case
            'basic',          # default
            'ghosted',        # break
            'simp',           # continue
            'be_like',        # type declaration
            'squad',          # struct
            'collab',         # interface
            'dm',             # channel
            'stan',           # goroutine spawn
            'flex',           # range
            'later',          # defer
            'based',          # true
            'cringe',         # false
            'nah',            # nil
            'shook',          # panic
            'fam',            # recover
        }
        
        # Deprecated keywords that should be replaced
        self.deprecated_keywords = {
            'lowkey': 'ready',
            'highkey': 'otherwise', 
            'cap': 'cringe',
        }
        
        # Invalid function names and their replacements
        self.function_name_fixes = {
            'main': 'main_character',
            'damn main': 'slay main_character',
        }
        
        # Type name corrections
        self.type_fixes = {
            'i32': 'normie',
            'f32': 'flex_float',  
            'f64': 'flex_float',
            'string': 'tea',
            'bool': 'lit',
            'str': 'tea',
        }
        
        # Standard library function corrections
        self.stdlib_fixes = {
            'yap(': 'vibez.spill(',
            'println(': 'vibez.spill(',
            'print(': 'vibez.spill(',
        }
        
        self.fixes_applied = []
        
    def validate_file(self, filepath: Path) -> Tuple[bool, List[str]]:
        """Validate a CURSED file and return (is_valid, issues)"""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return False, [f"Failed to read file: {e}"]
            
        issues = []
        
        # 1. Package clause validation
        if not self.validate_package_clause(content):
            issues.append("Missing or invalid package clause (must start with 'vibe <identifier>')")
            
        # 2. Import validation  
        import_issues = self.validate_imports(content)
        issues.extend(import_issues)
        
        # 3. Function declaration validation
        func_issues = self.validate_functions(content)
        issues.extend(func_issues)
        
        # 4. Return statement validation
        return_issues = self.validate_returns(content)
        issues.extend(return_issues)
        
        # 5. Variable declaration validation
        var_issues = self.validate_variables(content)
        issues.extend(var_issues)
        
        # 6. Deprecated keyword validation
        deprecated_issues = self.validate_deprecated_keywords(content)
        issues.extend(deprecated_issues)
        
        # 7. Standard library call validation
        stdlib_issues = self.validate_stdlib_calls(content)
        issues.extend(stdlib_issues)
        
        # 8. Comment validation
        comment_issues = self.validate_comments(content)
        issues.extend(comment_issues)
        
        return len(issues) == 0, issues
        
    def validate_package_clause(self, content: str) -> bool:
        """Check if file starts with valid 'vibe <identifier>' clause"""
        lines = content.strip().split('\n')
        if not lines:
            return False
            
        first_line = lines[0].strip()
        # Skip comment lines
        for line in lines:
            line = line.strip()
            if line and not line.startswith('fr fr') and not line.startswith('#'):
                first_line = line
                break
                
        return bool(re.match(r'^vibe\s+\w+\s*(;?)$', first_line))
        
    def validate_imports(self, content: str) -> List[str]:
        """Validate import statements use 'yeet' syntax"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            line = line.strip()
            # Check for invalid import statements
            if re.match(r'^import\s+', line):
                issues.append(f"Line {i}: Use 'yeet' instead of 'import'")
            elif 'yeet' in line and not re.match(r'^yeet\s+', line):
                # Check for malformed yeet statements
                if not re.match(r'^yeet\s+["\'][^"\']+["\']', line):
                    issues.append(f"Line {i}: Invalid yeet syntax - should be 'yeet \"module_name\"'")
                    
        return issues
        
    def validate_functions(self, content: str) -> List[str]:
        """Validate function declarations use 'slay' keyword"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            line = line.strip()
            # Check for invalid function declarations
            if re.match(r'^func\s+', line):
                issues.append(f"Line {i}: Use 'slay' instead of 'func'")
            elif re.match(r'^damn\s+main\s*\(', line):
                issues.append(f"Line {i}: Use 'slay main_character()' instead of 'damn main()'")
            elif re.match(r'^def\s+', line):
                issues.append(f"Line {i}: Use 'slay' instead of 'def'")
                
        return issues
        
    def validate_returns(self, content: str) -> List[str]:
        """Validate return statements use 'damn' keyword"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            line = line.strip()
            if re.match(r'^return\s', line) or line == 'return':
                issues.append(f"Line {i}: Use 'damn' instead of 'return'")
                
        return issues
        
    def validate_variables(self, content: str) -> List[str]:
        """Validate variable declarations use correct CURSED types"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            line = line.strip()
            # Check for invalid type declarations
            for invalid_type, correct_type in self.type_fixes.items():
                if f': {invalid_type}' in line or f' {invalid_type} ' in line:
                    issues.append(f"Line {i}: Use '{correct_type}' instead of '{invalid_type}'")
                    
            # Check for non-CURSED variable declarations
            if re.match(r'^var\s+', line):
                issues.append(f"Line {i}: Use 'sus' instead of 'var'")
            elif re.match(r'^let\s+', line):
                issues.append(f"Line {i}: Use 'sus' instead of 'let'")
                
        return issues
        
    def validate_deprecated_keywords(self, content: str) -> List[str]:
        """Check for deprecated keywords"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            for deprecated, replacement in self.deprecated_keywords.items():
                if deprecated in line:
                    issues.append(f"Line {i}: Replace deprecated '{deprecated}' with '{replacement}'")
                    
        return issues
        
    def validate_stdlib_calls(self, content: str) -> List[str]:
        """Validate standard library function calls"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            line = line.strip()
            for invalid_call, correct_call in self.stdlib_fixes.items():
                if invalid_call in line:
                    issues.append(f"Line {i}: Replace '{invalid_call}' with '{correct_call}'")
                    
            # Check for missing vibez import when using vibez.spill
            if 'vibez.spill(' in line and 'yeet "vibez"' not in content:
                issues.append(f"Line {i}: Missing 'yeet \"vibez\"' import for vibez.spill()")
                
        return issues
        
    def validate_comments(self, content: str) -> List[str]:
        """Validate comment syntax"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            # Check for invalid comment styles (allowing both fr fr and # for compatibility)
            stripped = line.strip()
            if stripped.startswith('//'):
                issues.append(f"Line {i}: Use 'fr fr' or '#' instead of '//' for comments")
            elif stripped.startswith('/*') or stripped.endswith('*/'):
                issues.append(f"Line {i}: Use 'no cap ... on god' instead of '/* ... */' for block comments")
                
        return issues
        
    def fix_file(self, filepath: Path) -> bool:
        """Fix issues in a CURSED file"""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"Failed to read {filepath}: {e}")
            return False
            
        original_content = content
        fixes_made = []
        
        # Apply fixes in order of importance
        content, package_fixes = self.fix_package_clause(content)
        fixes_made.extend(package_fixes)
        
        content, import_fixes = self.fix_imports(content)
        fixes_made.extend(import_fixes)
        
        content, func_fixes = self.fix_functions(content)
        fixes_made.extend(func_fixes)
        
        content, return_fixes = self.fix_returns(content)
        fixes_made.extend(return_fixes)
        
        content, var_fixes = self.fix_variables(content)
        fixes_made.extend(var_fixes)
        
        content, deprecated_fixes = self.fix_deprecated_keywords(content)
        fixes_made.extend(deprecated_fixes)
        
        content, stdlib_fixes = self.fix_stdlib_calls(content)
        fixes_made.extend(stdlib_fixes)
        
        content, comment_fixes = self.fix_comments(content)
        fixes_made.extend(comment_fixes)
        
        # Only write if changes were made
        if content != original_content:
            try:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                    
                self.fixes_applied.append({
                    'file': str(filepath),
                    'fixes': fixes_made
                })
                return True
                
            except Exception as e:
                print(f"Failed to write {filepath}: {e}")
                return False
                
        return True
        
    def fix_package_clause(self, content: str) -> Tuple[str, List[str]]:
        """Add missing package clause if needed"""
        fixes = []
        lines = content.split('\n')
        
        # Check if first non-comment line is a package clause
        has_package = False
        for line in lines:
            line = line.strip()
            if line and not line.startswith('fr fr') and not line.startswith('#'):
                if line.startswith('vibe '):
                    has_package = True
                break
                
        if not has_package:
            # Insert package clause at the beginning
            content = 'vibe main\n' + content
            fixes.append('Added missing package clause: vibe main')
            
        return content, fixes
        
    def fix_imports(self, content: str) -> Tuple[str, List[str]]:
        """Fix import statements"""
        fixes = []
        
        # Replace 'import' with 'yeet'
        if 'import "' in content:
            content = re.sub(r'^import\s+"([^"]+)"', r'yeet "\1"', content, flags=re.MULTILINE)
            fixes.append("Replaced 'import' with 'yeet'")
            
        return content, fixes
        
    def fix_functions(self, content: str) -> Tuple[str, List[str]]:
        """Fix function declarations"""
        fixes = []
        
        # Replace 'func' with 'slay'
        if re.search(r'^func\s+', content, re.MULTILINE):
            content = re.sub(r'^func\s+', 'slay ', content, flags=re.MULTILINE)
            fixes.append("Replaced 'func' with 'slay'")
            
        # Replace 'damn main(' with 'slay main_character('
        if 'damn main(' in content:
            content = content.replace('damn main(', 'slay main_character(')
            fixes.append("Replaced 'damn main()' with 'slay main_character()'")
            
        # Replace 'main(' with 'main_character('
        if re.search(r'\bmain\s*\(', content):
            content = re.sub(r'\bmain\s*\(', 'main_character(', content)
            fixes.append("Replaced 'main()' with 'main_character()'")
            
        return content, fixes
        
    def fix_returns(self, content: str) -> Tuple[str, List[str]]:
        """Fix return statements"""
        fixes = []
        
        # Replace 'return' with 'damn'
        if re.search(r'^return\b', content, re.MULTILINE):
            content = re.sub(r'^return\b', 'damn', content, flags=re.MULTILINE)
            fixes.append("Replaced 'return' with 'damn'")
            
        return content, fixes
        
    def fix_variables(self, content: str) -> Tuple[str, List[str]]:
        """Fix variable declarations and types"""
        fixes = []
        
        # Replace variable declaration keywords
        if re.search(r'^var\s+', content, re.MULTILINE):
            content = re.sub(r'^var\s+', 'sus ', content, flags=re.MULTILINE)
            fixes.append("Replaced 'var' with 'sus'")
            
        if re.search(r'^let\s+', content, re.MULTILINE):
            content = re.sub(r'^let\s+', 'sus ', content, flags=re.MULTILINE)
            fixes.append("Replaced 'let' with 'sus'")
            
        # Fix type names
        for invalid_type, correct_type in self.type_fixes.items():
            pattern = r'\b' + re.escape(invalid_type) + r'\b'
            if re.search(pattern, content):
                content = re.sub(pattern, correct_type, content)
                fixes.append(f"Replaced type '{invalid_type}' with '{correct_type}'")
                
        return content, fixes
        
    def fix_deprecated_keywords(self, content: str) -> Tuple[str, List[str]]:
        """Fix deprecated keywords"""
        fixes = []
        
        for deprecated, replacement in self.deprecated_keywords.items():
            pattern = r'\b' + re.escape(deprecated) + r'\b'
            if re.search(pattern, content):
                content = re.sub(pattern, replacement, content)
                fixes.append(f"Replaced deprecated '{deprecated}' with '{replacement}'")
                
        return content, fixes
        
    def fix_stdlib_calls(self, content: str) -> Tuple[str, List[str]]:
        """Fix standard library function calls"""
        fixes = []
        
        for invalid_call, correct_call in self.stdlib_fixes.items():
            if invalid_call in content:
                content = content.replace(invalid_call, correct_call)
                fixes.append(f"Replaced '{invalid_call}' with '{correct_call}'")
                
        # Add missing vibez import if needed
        if 'vibez.spill(' in content and 'yeet "vibez"' not in content:
            # Find the position after the package clause to insert import
            lines = content.split('\n')
            insert_pos = 0
            for i, line in enumerate(lines):
                if line.strip().startswith('vibe '):
                    insert_pos = i + 1
                    break
                    
            lines.insert(insert_pos, 'yeet "vibez"')
            content = '\n'.join(lines)
            fixes.append('Added missing import: yeet "vibez"')
            
        return content, fixes
        
    def fix_comments(self, content: str) -> Tuple[str, List[str]]:
        """Fix comment syntax"""
        fixes = []
        
        # Replace // comments with fr fr
        if '//' in content:
            content = re.sub(r'//(.*)', r'fr fr\1', content)
            fixes.append("Replaced '//' comments with 'fr fr'")
            
        # Replace /* */ comments with no cap ... on god
        block_comment_pattern = r'/\*(.*?)\*/'
        if re.search(block_comment_pattern, content, re.DOTALL):
            content = re.sub(block_comment_pattern, r'no cap\1on god', content, flags=re.DOTALL)
            fixes.append("Replaced '/* */' comments with 'no cap ... on god'")
            
        return content, fixes

def main():
    test_programs_dir = Path("/home/ghuntley/cursed/test_suite/test_programs")
    validator = CursedValidator()
    
    if not test_programs_dir.exists():
        print(f"Test programs directory not found: {test_programs_dir}")
        return 1
        
    # Find all .💀 files
    csd_files = []
    for root, dirs, files in os.walk(test_programs_dir):
        for file in files:
            if file.endswith('.💀'):
                csd_files.append(Path(root) / file)
                
    if not csd_files:
        print("No .💀 files found in test programs directory")
        return 1
        
    print(f"Found {len(csd_files)} .💀 files to validate and fix")
    print("=" * 60)
    
    total_files = len(csd_files)
    files_with_issues = 0
    files_fixed = 0
    
    # Validate and fix each file
    for filepath in sorted(csd_files):
        print(f"\n📁 Processing: {filepath.relative_to(test_programs_dir)}")
        
        # First validate
        is_valid, issues = validator.validate_file(filepath)
        
        if not is_valid:
            files_with_issues += 1
            print(f"  ❌ Found {len(issues)} issues:")
            for issue in issues:
                print(f"     • {issue}")
                
            # Attempt to fix
            if validator.fix_file(filepath):
                files_fixed += 1
                print(f"  ✅ Fixed successfully")
                
                # Validate again to confirm fix
                is_valid_after, remaining_issues = validator.validate_file(filepath)
                if is_valid_after:
                    print(f"  ✨ File now validates correctly")
                else:
                    print(f"  ⚠️  Still has {len(remaining_issues)} issues:")
                    for issue in remaining_issues:
                        print(f"     • {issue}")
            else:
                print(f"  ❌ Failed to fix file")
        else:
            print(f"  ✅ Already valid")
            
    print("\n" + "=" * 60)
    print(f"VALIDATION SUMMARY:")
    print(f"Total files processed: {total_files}")
    print(f"Files with issues: {files_with_issues}")
    print(f"Files successfully fixed: {files_fixed}")
    
    if validator.fixes_applied:
        print(f"\nDETAILED FIX REPORT:")
        for fix_info in validator.fixes_applied:
            print(f"\n📄 {Path(fix_info['file']).name}:")
            for fix in fix_info['fixes']:
                print(f"   • {fix}")
                
    return 0

if __name__ == "__main__":
    sys.exit(main())
