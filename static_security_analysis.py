#!/usr/bin/env python3
"""
Static Security Analysis Tool for CURSED Codebase
Scans for potential security vulnerabilities in source code
"""

import os
import re
import json
import glob
from typing import List, Dict, Tuple
from pathlib import Path

class SecurityScanner:
    def __init__(self):
        self.vulnerabilities = []
        self.security_patterns = {
            'hardcoded_secrets': [
                r'password\s*=\s*["\'][^"\']+["\']',
                r'api[_-]?key\s*=\s*["\'][^"\']+["\']',
                r'secret\s*=\s*["\'][^"\']+["\']',
                r'token\s*=\s*["\'][A-Za-z0-9]{20,}["\']',
            ],
            'unsafe_functions': [
                r'\bstrcpy\b',
                r'\bsprintf\b', 
                r'\bgets\b',
                r'\bstrcat\b',
                r'\bsystem\b',
                r'\bexec\b',
                r'\beval\b',
            ],
            'sql_injection_risk': [
                r'execute\s*\(\s*["\'][^"\']*\+',
                r'query\s*\(\s*["\'][^"\']*\+',
                r'SELECT\s+.*\+\s*',
                r'INSERT\s+.*\+\s*',
                r'UPDATE\s+.*\+\s*',
                r'DELETE\s+.*\+\s*',
            ],
            'xss_risk': [
                r'innerHTML\s*=\s*[^"\']*\+',
                r'document\.write\s*\(',
                r'\.html\s*\(\s*[^"\']*\+',
            ],
            'path_traversal_risk': [
                r'\.\.\/\.\.\/',
                r'\.\.\\\.\.\\',
                r'\/etc\/passwd',
                r'\/proc\/self\/environ',
            ],
            'weak_crypto': [
                r'\bMD5\b',
                r'\bSHA1\b',
                r'\bDES\b',
                r'\bRC4\b',
            ],
            'debug_code': [
                r'console\.log\s*\(',
                r'print\s*\(',
                r'debug\s*=\s*True',
                r'DEBUG\s*=\s*true',
            ]
        }
    
    def scan_file(self, filepath: str) -> List[Dict]:
        """Scan a single file for security vulnerabilities"""
        vulnerabilities = []
        
        try:
            with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                lines = content.split('\n')
                
                for line_num, line in enumerate(lines, 1):
                    for category, patterns in self.security_patterns.items():
                        for pattern in patterns:
                            if re.search(pattern, line, re.IGNORECASE):
                                vulnerabilities.append({
                                    'file': filepath,
                                    'line': line_num,
                                    'category': category,
                                    'pattern': pattern,
                                    'code': line.strip(),
                                    'severity': self.get_severity(category)
                                })
        except Exception as e:
            print(f"Error scanning {filepath}: {e}")
            
        return vulnerabilities
    
    def get_severity(self, category: str) -> str:
        """Get severity level for vulnerability category"""
        severity_map = {
            'hardcoded_secrets': 'CRITICAL',
            'unsafe_functions': 'HIGH',
            'sql_injection_risk': 'HIGH',
            'xss_risk': 'HIGH',
            'path_traversal_risk': 'HIGH',
            'weak_crypto': 'MEDIUM',
            'debug_code': 'LOW'
        }
        return severity_map.get(category, 'MEDIUM')
    
    def scan_directory(self, directory: str) -> List[Dict]:
        """Scan all source files in directory"""
        extensions = ['*.c', '*.h', '*.zig', '*.rs', '*.py', '*.js', '*.csd']
        all_vulnerabilities = []
        
        for ext in extensions:
            for filepath in glob.glob(os.path.join(directory, '**', ext), recursive=True):
                # Skip test files and documentation
                if any(skip in filepath for skip in ['/test/', '/tests/', '/docs/', '/.git/']):
                    continue
                    
                file_vulns = self.scan_file(filepath)
                all_vulnerabilities.extend(file_vulns)
        
        return all_vulnerabilities
    
    def generate_report(self, vulnerabilities: List[Dict]) -> str:
        """Generate security analysis report"""
        if not vulnerabilities:
            return "✅ No security vulnerabilities detected!"
        
        # Group by severity
        severity_counts = {}
        category_counts = {}
        
        for vuln in vulnerabilities:
            severity = vuln['severity']
            category = vuln['category']
            
            severity_counts[severity] = severity_counts.get(severity, 0) + 1
            category_counts[category] = category_counts.get(category, 0) + 1
        
        report = []
        report.append("🔍 STATIC SECURITY ANALYSIS REPORT")
        report.append("=" * 50)
        report.append("")
        
        # Summary
        report.append("📊 VULNERABILITY SUMMARY:")
        total_vulns = len(vulnerabilities)
        report.append(f"   Total vulnerabilities: {total_vulns}")
        
        for severity in ['CRITICAL', 'HIGH', 'MEDIUM', 'LOW']:
            count = severity_counts.get(severity, 0)
            if count > 0:
                report.append(f"   {severity}: {count}")
        
        report.append("")
        
        # Category breakdown
        report.append("🏷️  CATEGORY BREAKDOWN:")
        for category, count in sorted(category_counts.items()):
            report.append(f"   {category}: {count}")
        
        report.append("")
        
        # Detailed findings
        report.append("🔍 DETAILED FINDINGS:")
        report.append("-" * 30)
        
        # Sort by severity
        severity_order = {'CRITICAL': 0, 'HIGH': 1, 'MEDIUM': 2, 'LOW': 3}
        sorted_vulns = sorted(vulnerabilities, key=lambda x: severity_order.get(x['severity'], 4))
        
        for vuln in sorted_vulns:
            report.append("")
            report.append(f"🚨 {vuln['severity']} - {vuln['category'].upper()}")
            report.append(f"   File: {vuln['file']}")
            report.append(f"   Line: {vuln['line']}")
            report.append(f"   Pattern: {vuln['pattern']}")
            report.append(f"   Code: {vuln['code']}")
        
        return "\\n".join(report)

def main():
    """Main function to run security analysis"""
    scanner = SecurityScanner()
    
    # Scan main source directories
    directories_to_scan = [
        'src-zig',
        'stdlib',
        'tools',
        'examples',
    ]
    
    all_vulnerabilities = []
    
    for directory in directories_to_scan:
        if os.path.exists(directory):
            print(f"🔍 Scanning {directory}...")
            vulns = scanner.scan_directory(directory)
            all_vulnerabilities.extend(vulns)
    
    # Generate and save report
    report = scanner.generate_report(all_vulnerabilities)
    
    # Write to file
    with open('security_analysis_report.txt', 'w') as f:
        f.write(report)
    
    print("\\n" + report)
    
    # Return exit code based on severity
    critical_count = sum(1 for v in all_vulnerabilities if v['severity'] == 'CRITICAL')
    high_count = sum(1 for v in all_vulnerabilities if v['severity'] == 'HIGH')
    
    if critical_count > 0:
        print(f"\\n❌ CRITICAL VULNERABILITIES FOUND: {critical_count}")
        return 2
    elif high_count > 0:
        print(f"\\n⚠️  HIGH SEVERITY VULNERABILITIES FOUND: {high_count}")
        return 1
    else:
        print("\\n✅ No critical or high severity vulnerabilities found")
        return 0

if __name__ == '__main__':
    exit(main())
