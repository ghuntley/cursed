#!/usr/bin/env python3
"""
Final CURSED Test Suite Validation Report

This script generates a comprehensive report on the validation status
of all test programs after applying fixes.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Tuple

class FinalValidator:
    def __init__(self):
        # Valid CURSED keywords based on specs
        self.keywords = {
            'vibe', 'yeet', 'slay', 'damn', 'yolo', 'sus', 'facts',
            'ready', 'otherwise', 'bestie', 'periodt', 'vibe_check',
            'mood', 'basic', 'ghosted', 'simp', 'be_like', 'squad',
            'collab', 'dm', 'stan', 'flex', 'later', 'based',
            'cringe', 'nah', 'shook', 'fam'
        }
        
        # Issues to check for
        self.validation_results = {
            'fully_compliant': [],
            'minor_issues': [],
            'major_issues': [],
            'parsing_issues': []
        }
        
    def comprehensive_validate(self, filepath: Path) -> Dict:
        """Perform comprehensive validation"""
        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            return {'status': 'error', 'error': str(e), 'issues': []}
            
        issues = []
        
        # 1. Package clause (mandatory)
        if not self.has_valid_package_clause(content):
            issues.append("❌ CRITICAL: Missing valid package clause (vibe <identifier>)")
            
        # 2. Import syntax
        import_issues = self.check_imports(content)
        issues.extend(import_issues)
        
        # 3. Function declarations
        func_issues = self.check_functions(content)
        issues.extend(func_issues)
        
        # 4. Return statements
        return_issues = self.check_returns(content)
        issues.extend(return_issues)
        
        # 5. Variable declarations and types
        var_issues = self.check_variables_and_types(content)
        issues.extend(var_issues)
        
        # 6. Deprecated keywords
        deprecated_issues = self.check_deprecated_keywords(content)
        issues.extend(deprecated_issues)
        
        # 7. Standard library calls
        stdlib_issues = self.check_stdlib_calls(content)
        issues.extend(stdlib_issues)
        
        # 8. Comment syntax
        comment_issues = self.check_comments(content)
        issues.extend(comment_issues)
        
        # Categorize severity
        critical_issues = [i for i in issues if 'CRITICAL' in i]
        major_issues = [i for i in issues if '❌' in i and 'CRITICAL' not in i]
        minor_issues = [i for i in issues if '⚠️' in i]
        
        if critical_issues:
            status = 'critical'
        elif major_issues:
            status = 'major_issues'
        elif minor_issues:
            status = 'minor_issues'
        else:
            status = 'compliant'
            
        return {
            'status': status,
            'issues': issues,
            'critical': critical_issues,
            'major': major_issues,
            'minor': minor_issues,
            'content_lines': len(content.split('\n'))
        }
        
    def has_valid_package_clause(self, content: str) -> bool:
        """Check for valid package clause at start of file"""
        lines = [line.strip() for line in content.split('\n') if line.strip()]
        if not lines:
            return False
            
        # Find first non-comment line
        for line in lines:
            if line and not line.startswith('fr fr') and not line.startswith('#'):
                return bool(re.match(r'^vibe\s+\w+\s*(;?)$', line))
        return False
        
    def check_imports(self, content: str) -> List[str]:
        """Check import statement syntax"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            
            # Invalid import keywords
            if re.match(r'^import\s+', stripped):
                issues.append(f"❌ Line {i}: Use 'yeet' instead of 'import'")
                
            # Malformed yeet syntax
            if stripped.startswith('yeet') and not re.match(r'^yeet\s+"[^"]+"', stripped):
                if not re.match(r'^yeet\s+\(', stripped):  # Allow grouped imports
                    issues.append(f"❌ Line {i}: Invalid yeet syntax")
                    
        return issues
        
    def check_functions(self, content: str) -> List[str]:
        """Check function declarations"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            
            # Invalid function keywords
            if re.match(r'^func\s+', stripped):
                issues.append(f"❌ Line {i}: Use 'slay' instead of 'func'")
            elif re.match(r'^def\s+', stripped):
                issues.append(f"❌ Line {i}: Use 'slay' instead of 'def'")
                
            # Wrong main function name
            if 'main(' in stripped and 'main_character(' not in stripped:
                issues.append(f"❌ Line {i}: Use 'main_character()' instead of 'main()'")
                
        return issues
        
    def check_returns(self, content: str) -> List[str]:
        """Check return statements"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            if re.match(r'^return\b', stripped) or stripped == 'return':
                issues.append(f"❌ Line {i}: Use 'damn' instead of 'return'")
                
        return issues
        
    def check_variables_and_types(self, content: str) -> List[str]:
        """Check variable declarations and type names"""
        issues = []
        lines = content.split('\n')
        
        type_mapping = {
            'i32': 'normie',
            'f32': 'flex_float', 
            'f64': 'flex_float',
            'string': 'tea',
            'str': 'tea',
            'bool': 'lit'
        }
        
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            
            # Variable declaration keywords
            if re.match(r'^var\s+', stripped):
                issues.append(f"❌ Line {i}: Use 'sus' instead of 'var'")
            elif re.match(r'^let\s+', stripped):
                issues.append(f"❌ Line {i}: Use 'sus' instead of 'let'")
                
            # Type names
            for invalid_type, correct_type in type_mapping.items():
                if f': {invalid_type}' in line or f' {invalid_type} ' in line:
                    issues.append(f"⚠️ Line {i}: Consider using '{correct_type}' instead of '{invalid_type}'")
                    
        return issues
        
    def check_deprecated_keywords(self, content: str) -> List[str]:
        """Check for deprecated keywords"""
        issues = []
        lines = content.split('\n')
        
        deprecated = {
            'lowkey': 'ready',
            'highkey': 'otherwise',
            'cap': 'cringe'
        }
        
        for i, line in enumerate(lines, 1):
            for dep_kw, replacement in deprecated.items():
                if re.search(r'\b' + dep_kw + r'\b', line):
                    issues.append(f"⚠️ Line {i}: Replace deprecated '{dep_kw}' with '{replacement}'")
                    
        return issues
        
    def check_stdlib_calls(self, content: str) -> List[str]:
        """Check standard library function calls"""
        issues = []
        lines = content.split('\n')
        
        stdlib_mapping = {
            'yap(': 'vibez.spill(',
            'println(': 'vibez.spill(',
            'print(': 'vibez.spill('
        }
        
        for i, line in enumerate(lines, 1):
            for invalid_call, correct_call in stdlib_mapping.items():
                if invalid_call in line:
                    issues.append(f"⚠️ Line {i}: Use '{correct_call}' instead of '{invalid_call}'")
                    
            # Check for missing vibez import
            if 'vibez.spill(' in line and 'yeet "vibez"' not in content:
                issues.append(f"⚠️ Line {i}: Missing 'yeet \"vibez\"' import")
                break  # Only report once per file
                
        return issues
        
    def check_comments(self, content: str) -> List[str]:
        """Check comment syntax"""
        issues = []
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            stripped = line.strip()
            if stripped.startswith('//'):
                issues.append(f"⚠️ Line {i}: Consider using 'fr fr' or '#' instead of '//'")
            elif '/*' in stripped or '*/' in stripped:
                issues.append(f"⚠️ Line {i}: Consider using 'no cap...on god' for block comments")
                
        return issues

def generate_report():
    """Generate comprehensive validation report"""
    test_programs_dir = Path("/home/ghuntley/cursed/test_suite/test_programs")
    validator = FinalValidator()
    
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
        print("No .csd files found")
        return 1
        
    print("🔍 CURSED Test Suite Final Validation Report")
    print("=" * 80)
    print(f"Validating {len(csd_files)} test programs...")
    print()
    
    # Validate all files
    results = {}
    for filepath in sorted(csd_files):
        result = validator.comprehensive_validate(filepath)
        results[filepath] = result
        
    # Categorize results
    compliant_files = [f for f, r in results.items() if r['status'] == 'compliant']
    minor_issue_files = [f for f, r in results.items() if r['status'] == 'minor_issues']
    major_issue_files = [f for f, r in results.items() if r['status'] == 'major_issues']
    critical_issue_files = [f for f, r in results.items() if r['status'] == 'critical']
    error_files = [f for f, r in results.items() if r['status'] == 'error']
    
    # Summary stats
    print("📊 VALIDATION SUMMARY")
    print("=" * 40)
    print(f"✅ Fully Compliant:     {len(compliant_files):3d} files")
    print(f"⚠️  Minor Issues:       {len(minor_issue_files):3d} files")
    print(f"❌ Major Issues:       {len(major_issue_files):3d} files")
    print(f"🚨 Critical Issues:    {len(critical_issue_files):3d} files")
    print(f"💥 Parse Errors:       {len(error_files):3d} files")
    print(f"📁 Total Files:        {len(csd_files):3d} files")
    print()
    
    # Calculate compliance percentage
    compliance_percentage = (len(compliant_files) / len(csd_files)) * 100
    print(f"🎯 Compliance Rate: {compliance_percentage:.1f}%")
    print()
    
    # Detailed breakdown by directory
    print("📂 BREAKDOWN BY DIRECTORY")
    print("=" * 50)
    
    dirs = {}
    for filepath in csd_files:
        dir_name = filepath.parent.name
        if dir_name not in dirs:
            dirs[dir_name] = {'compliant': 0, 'minor': 0, 'major': 0, 'critical': 0, 'error': 0, 'total': 0}
        
        status = results[filepath]['status']
        dirs[dir_name][status if status != 'minor_issues' and status != 'major_issues' else status.split('_')[0]] += 1
        dirs[dir_name]['total'] += 1
        
    for dir_name, counts in sorted(dirs.items()):
        total = counts['total']
        compliant_pct = (counts['compliant'] / total) * 100
        print(f"{dir_name:20s} | {counts['compliant']:2d}/{total:2d} compliant ({compliant_pct:4.1f}%)")
        if counts['minor'] + counts['major'] + counts['critical'] + counts['error'] > 0:
            issues = []
            if counts['minor']: issues.append(f"{counts['minor']} minor")
            if counts['major']: issues.append(f"{counts['major']} major") 
            if counts['critical']: issues.append(f"{counts['critical']} critical")
            if counts['error']: issues.append(f"{counts['error']} errors")
            print(f"{'':20s} | Issues: {', '.join(issues)}")
        print()
    
    # Files with major/critical issues
    if major_issue_files or critical_issue_files:
        print("🚨 FILES REQUIRING ATTENTION")
        print("=" * 50)
        
        for filepath in critical_issue_files:
            print(f"\n🚨 CRITICAL: {filepath.relative_to(test_programs_dir)}")
            for issue in results[filepath]['critical']:
                print(f"   {issue}")
                
        for filepath in major_issue_files:
            print(f"\n❌ MAJOR: {filepath.relative_to(test_programs_dir)}")
            for issue in results[filepath]['major'][:5]:  # Show first 5 issues
                print(f"   {issue}")
            if len(results[filepath]['major']) > 5:
                print(f"   ... and {len(results[filepath]['major']) - 5} more")
    
    # Success stories
    if compliant_files:
        print("\n✅ FULLY COMPLIANT FILES")
        print("=" * 30)
        for filepath in compliant_files[:10]:  # Show first 10
            print(f"✅ {filepath.relative_to(test_programs_dir)}")
        if len(compliant_files) > 10:
            print(f"... and {len(compliant_files) - 10} more")
    
    print("\n" + "=" * 80)
    print("🎉 VALIDATION COMPLETE")
    
    # Recommendations
    if major_issue_files or critical_issue_files:
        print("\n📝 RECOMMENDATIONS:")
        print("• Focus on files with critical issues first")
        print("• Use Oracle validation for complex syntax issues")
        print("• Run compiler tests after fixing major issues")
    else:
        print("\n🎉 EXCELLENT! All test programs are CURSED-compliant!")
        print("• Ready for comprehensive compiler testing")
        print("• Proceed with interpreter vs compiled mode validation")
        
    return 0

if __name__ == "__main__":
    sys.exit(generate_report())
