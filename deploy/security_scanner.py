#!/usr/bin/env python3

"""
CURSED Security Scanner
Comprehensive security analysis for production deployment
"""

import os
import re
import json
import subprocess
import hashlib
from pathlib import Path
from typing import Dict, List, Set, Optional, Tuple
from dataclasses import dataclass
from datetime import datetime

@dataclass
class SecurityIssue:
    """Represents a security issue found during scanning"""
    severity: str  # critical, high, medium, low
    category: str  # code, dependencies, binary, configuration
    description: str
    file_path: str
    line_number: Optional[int] = None
    recommendation: str = ""

class SecurityScanner:
    """Comprehensive security scanner for CURSED project"""
    
    def __init__(self, project_root: str):
        self.project_root = Path(project_root)
        self.issues: List[SecurityIssue] = []
        
        # Security patterns to scan for
        self.security_patterns = {
            'hardcoded_secrets': [
                r'(?i)(password|passwd|pwd)\s*[:=]\s*["\'][^"\']{8,}["\']',
                r'(?i)(secret|token|key)\s*[:=]\s*["\'][^"\']{16,}["\']',
                r'(?i)(api_key|apikey)\s*[:=]\s*["\'][^"\']{20,}["\']',
                r'(?i)(private_key|privatekey)\s*[:=]\s*["\'][^"\']{32,}["\']'
            ],
            'crypto_issues': [
                r'(?i)md5\s*\(',
                r'(?i)sha1\s*\(',
                r'(?i)des\s*\(',
                r'(?i)rc4\s*\(',
            ],
            'unsafe_functions': [
                r'@ptrCast\s*\(',
                r'@bitCast\s*\(',
                r'@intToPtr\s*\(',
                r'@ptrToInt\s*\(',
                r'unsafe\s*{',
                r'transmute\s*\(',
                r'from_raw_parts\s*\('
            ],
            'path_traversal': [
                r'\.\./',
                r'\.\.\\',
                r'path\.join\([^)]*\.\.[^)]*\)',
            ],
            'command_injection': [
                r'system\s*\(',
                r'exec\s*\(',
                r'spawn\s*\(',
                r'Command::new\([^)]*\$[^)]*\)',
            ]
        }
        
    def scan_file_content(self, file_path: Path, content: str):
        """Scan file content for security issues"""
        lines = content.split('\n')
        
        for category, patterns in self.security_patterns.items():
            for pattern in patterns:
                for line_num, line in enumerate(lines, 1):
                    matches = re.finditer(pattern, line)
                    for match in matches:
                        severity = self._get_severity(category, pattern)
                        description = self._get_description(category, match.group())
                        recommendation = self._get_recommendation(category)
                        
                        issue = SecurityIssue(
                            severity=severity,
                            category='code',
                            description=description,
                            file_path=str(file_path.relative_to(self.project_root)),
                            line_number=line_num,
                            recommendation=recommendation
                        )
                        self.issues.append(issue)
    
    def _get_severity(self, category: str, pattern: str) -> str:
        """Determine severity based on category and pattern"""
        severity_map = {
            'hardcoded_secrets': 'critical',
            'crypto_issues': 'high',
            'unsafe_functions': 'medium',
            'path_traversal': 'high',
            'command_injection': 'critical'
        }
        return severity_map.get(category, 'medium')
    
    def _get_description(self, category: str, match: str) -> str:
        """Get description for the security issue"""
        descriptions = {
            'hardcoded_secrets': f"Potential hardcoded secret detected: {match[:50]}...",
            'crypto_issues': f"Weak cryptographic function used: {match}",
            'unsafe_functions': f"Unsafe function call detected: {match}",
            'path_traversal': f"Potential path traversal vulnerability: {match}",
            'command_injection': f"Potential command injection vulnerability: {match}"
        }
        return descriptions.get(category, f"Security issue detected: {match}")
    
    def _get_recommendation(self, category: str) -> str:
        """Get recommendation for fixing the issue"""
        recommendations = {
            'hardcoded_secrets': "Move secrets to environment variables or secure configuration",
            'crypto_issues': "Use modern cryptographic algorithms (SHA-256, AES, etc.)",
            'unsafe_functions': "Review unsafe operations and add proper bounds checking",
            'path_traversal': "Validate and sanitize file paths before use",
            'command_injection': "Use parameterized commands and validate inputs"
        }
        return recommendations.get(category, "Review and fix the security issue")
    
    def scan_code_files(self):
        """Scan all code files for security issues"""
        print("🔍 Scanning code files for security issues...")
        
        # File extensions to scan
        extensions = {'.zig', '.rs', '.c', '.cpp', '.h', '.hpp', '.💀', '.py', '.js', '.ts'}
        
        for file_path in self.project_root.rglob('*'):
            if (file_path.is_file() and 
                file_path.suffix in extensions and
                not self._should_skip_file(file_path)):
                
                try:
                    content = file_path.read_text(encoding='utf-8', errors='ignore')
                    self.scan_file_content(file_path, content)
                except Exception as e:
                    print(f"⚠️  Warning: Could not scan {file_path}: {e}")
    
    def _should_skip_file(self, file_path: Path) -> bool:
        """Check if file should be skipped during scanning"""
        skip_dirs = {'.git', 'zig-cache', 'zig-out', 'target', 'node_modules', '.devenv'}
        skip_files = {'test', 'spec', 'demo', 'example'}
        
        # Skip files in certain directories
        for part in file_path.parts:
            if part in skip_dirs:
                return True
        
        # Skip test/demo files
        file_name_lower = file_path.name.lower()
        for skip_pattern in skip_files:
            if skip_pattern in file_name_lower:
                return True
        
        return False
    
    def check_dependencies(self):
        """Check for vulnerable dependencies"""
        print("🔍 Checking dependencies for known vulnerabilities...")
        
        # Check Cargo.toml if it exists
        cargo_toml = self.project_root / "Cargo.toml"
        if cargo_toml.exists():
            self._check_rust_dependencies()
        
        # Check package.json if it exists
        package_json = self.project_root / "package.json"
        if package_json.exists():
            self._check_npm_dependencies()
    
    def _check_rust_dependencies(self):
        """Check Rust dependencies using cargo audit"""
        try:
            result = subprocess.run([
                "cargo", "audit", "--json"
            ], capture_output=True, text=True, cwd=self.project_root)
            
            if result.returncode == 0:
                audit_data = json.loads(result.stdout)
                vulnerabilities = audit_data.get('vulnerabilities', [])
                
                for vuln in vulnerabilities:
                    issue = SecurityIssue(
                        severity='high',
                        category='dependencies',
                        description=f"Vulnerable dependency: {vuln.get('package', {}).get('name', 'unknown')} - {vuln.get('advisory', {}).get('title', 'Unknown vulnerability')}",
                        file_path="Cargo.toml",
                        recommendation=f"Update to version {vuln.get('advisory', {}).get('patched_versions', ['latest'])[0]} or later"
                    )
                    self.issues.append(issue)
        except (subprocess.CalledProcessError, FileNotFoundError, json.JSONDecodeError):
            print("⚠️  Could not run cargo audit - install with: cargo install cargo-audit")
    
    def _check_npm_dependencies(self):
        """Check npm dependencies using npm audit"""
        try:
            result = subprocess.run([
                "npm", "audit", "--json"
            ], capture_output=True, text=True, cwd=self.project_root)
            
            if result.stdout:
                audit_data = json.loads(result.stdout)
                vulnerabilities = audit_data.get('vulnerabilities', {})
                
                for package, vuln_info in vulnerabilities.items():
                    severity = vuln_info.get('severity', 'medium')
                    issue = SecurityIssue(
                        severity=severity,
                        category='dependencies',
                        description=f"Vulnerable npm package: {package} - {vuln_info.get('via', ['Unknown issue'])[0]}",
                        file_path="package.json",
                        recommendation="Run 'npm audit fix' to resolve"
                    )
                    self.issues.append(issue)
        except (subprocess.CalledProcessError, FileNotFoundError, json.JSONDecodeError):
            print("⚠️  Could not run npm audit")
    
    def check_binary_security(self):
        """Check compiled binaries for security features"""
        print("🔍 Checking binary security features...")
        
        binary_paths = [
            self.project_root / "zig-out" / "bin" / "cursed",
            self.project_root / "target" / "release" / "cursed"
        ]
        
        for binary_path in binary_paths:
            if binary_path.exists():
                self._analyze_binary(binary_path)
    
    def _analyze_binary(self, binary_path: Path):
        """Analyze a binary for security features"""
        try:
            # Check for stack protection
            if self._has_stack_protection(binary_path):
                print(f"✅ Stack protection enabled in {binary_path.name}")
            else:
                issue = SecurityIssue(
                    severity='medium',
                    category='binary',
                    description=f"Stack protection not enabled in {binary_path.name}",
                    file_path=str(binary_path.relative_to(self.project_root)),
                    recommendation="Enable stack protection in build configuration"
                )
                self.issues.append(issue)
            
            # Check for RELRO
            if self._has_relro(binary_path):
                print(f"✅ RELRO enabled in {binary_path.name}")
            else:
                issue = SecurityIssue(
                    severity='low',
                    category='binary',
                    description=f"RELRO not enabled in {binary_path.name}",
                    file_path=str(binary_path.relative_to(self.project_root)),
                    recommendation="Enable RELRO in linker flags"
                )
                self.issues.append(issue)
            
            # Check for PIE (Position Independent Executable)
            if self._has_pie(binary_path):
                print(f"✅ PIE enabled in {binary_path.name}")
            else:
                issue = SecurityIssue(
                    severity='medium',
                    category='binary',
                    description=f"PIE not enabled in {binary_path.name}",
                    file_path=str(binary_path.relative_to(self.project_root)),
                    recommendation="Enable PIE in build configuration"
                )
                self.issues.append(issue)
                
        except Exception as e:
            print(f"⚠️  Could not analyze binary {binary_path}: {e}")
    
    def _has_stack_protection(self, binary_path: Path) -> bool:
        """Check if binary has stack protection"""
        try:
            result = subprocess.run([
                "readelf", "-s", str(binary_path)
            ], capture_output=True, text=True)
            return "__stack_chk_fail" in result.stdout
        except subprocess.CalledProcessError:
            return False
    
    def _has_relro(self, binary_path: Path) -> bool:
        """Check if binary has RELRO enabled"""
        try:
            result = subprocess.run([
                "readelf", "-l", str(binary_path)
            ], capture_output=True, text=True)
            return "GNU_RELRO" in result.stdout
        except subprocess.CalledProcessError:
            return False
    
    def _has_pie(self, binary_path: Path) -> bool:
        """Check if binary is position independent"""
        try:
            result = subprocess.run([
                "file", str(binary_path)
            ], capture_output=True, text=True)
            return "shared object" in result.stdout or "pie executable" in result.stdout
        except subprocess.CalledProcessError:
            return False
    
    def check_configuration_security(self):
        """Check configuration files for security issues"""
        print("🔍 Checking configuration security...")
        
        config_files = [
            "devenv.nix",
            "build.zig", 
            ".github/workflows/*.yml",
            "Cargo.toml",
            "package.json"
        ]
        
        for pattern in config_files:
            for config_file in self.project_root.glob(pattern):
                if config_file.is_file():
                    self._check_config_file(config_file)
    
    def _check_config_file(self, config_file: Path):
        """Check a specific configuration file"""
        try:
            content = config_file.read_text()
            
            # Check for exposed secrets in config
            secret_patterns = [
                r'(?i)(token|key|password|secret)\s*[:=]\s*["\'][^"\']+["\']',
                r'(?i)github_token\s*[:=]\s*["\'][^"\']+["\']'
            ]
            
            for pattern in secret_patterns:
                matches = re.finditer(pattern, content)
                for match in matches:
                    # Skip if it's clearly a placeholder or environment variable
                    if any(placeholder in match.group().lower() for placeholder in 
                          ['$', 'env', 'placeholder', 'example', 'xxx', 'your']):
                        continue
                    
                    issue = SecurityIssue(
                        severity='high',
                        category='configuration',
                        description=f"Potential secret in configuration: {match.group()[:50]}...",
                        file_path=str(config_file.relative_to(self.project_root)),
                        recommendation="Use environment variables or secure secret management"
                    )
                    self.issues.append(issue)
                    
        except Exception as e:
            print(f"⚠️  Could not check config file {config_file}: {e}")
    
    def generate_report(self) -> Dict:
        """Generate comprehensive security report"""
        # Categorize issues by severity
        severity_counts = {'critical': 0, 'high': 0, 'medium': 0, 'low': 0}
        category_counts = {'code': 0, 'dependencies': 0, 'binary': 0, 'configuration': 0}
        
        for issue in self.issues:
            severity_counts[issue.severity] += 1
            category_counts[issue.category] += 1
        
        report = {
            'scan_date': datetime.now().isoformat(),
            'project_root': str(self.project_root),
            'summary': {
                'total_issues': len(self.issues),
                'severity_breakdown': severity_counts,
                'category_breakdown': category_counts
            },
            'issues': [
                {
                    'severity': issue.severity,
                    'category': issue.category,
                    'description': issue.description,
                    'file_path': issue.file_path,
                    'line_number': issue.line_number,
                    'recommendation': issue.recommendation
                }
                for issue in self.issues
            ],
            'recommendations': self._generate_recommendations()
        }
        
        return report
    
    def _generate_recommendations(self) -> List[str]:
        """Generate overall security recommendations"""
        recommendations = []
        
        if any(issue.severity == 'critical' for issue in self.issues):
            recommendations.append("🚨 Critical security issues found - fix before production deployment")
        
        if any(issue.category == 'dependencies' for issue in self.issues):
            recommendations.append("📦 Update vulnerable dependencies before release")
        
        if any('stack protection' in issue.description for issue in self.issues):
            recommendations.append("🛡️  Enable stack protection in build configuration")
        
        if any('secret' in issue.description.lower() for issue in self.issues):
            recommendations.append("🔑 Move hardcoded secrets to environment variables")
        
        recommendations.extend([
            "🔒 Enable all available security features in build configuration",
            "🧪 Run security scans regularly in CI/CD pipeline",
            "📋 Perform security review before each release",
            "🔄 Keep dependencies updated and monitor for vulnerabilities"
        ])
        
        return recommendations
    
    def save_report(self, output_file: str):
        """Save security report to file"""
        report = self.generate_report()
        
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"📄 Security report saved to {output_file}")
    
    def print_summary(self):
        """Print security scan summary"""
        severity_counts = {'critical': 0, 'high': 0, 'medium': 0, 'low': 0}
        
        for issue in self.issues:
            severity_counts[issue.severity] += 1
        
        print("\n🔒 Security Scan Summary")
        print("=" * 50)
        print(f"Total Issues: {len(self.issues)}")
        print(f"Critical: {severity_counts['critical']}")
        print(f"High: {severity_counts['high']}")
        print(f"Medium: {severity_counts['medium']}")
        print(f"Low: {severity_counts['low']}")
        
        if severity_counts['critical'] > 0:
            print("\n🚨 CRITICAL ISSUES FOUND - DO NOT DEPLOY")
        elif severity_counts['high'] > 0:
            print("\n⚠️  HIGH SEVERITY ISSUES FOUND - REVIEW BEFORE DEPLOYMENT")
        elif len(self.issues) == 0:
            print("\n✅ NO SECURITY ISSUES FOUND")
        else:
            print("\n👍 ONLY LOW/MEDIUM ISSUES FOUND - DEPLOYMENT OK")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Security Scanner")
    parser.add_argument("--project-root", default=".", help="Project root directory")
    parser.add_argument("--output", help="Output file for detailed report")
    parser.add_argument("--format", choices=["json", "html"], default="json", help="Report format")
    
    args = parser.parse_args()
    
    scanner = SecurityScanner(args.project_root)
    
    print("🔒 Starting CURSED security scan...")
    
    # Run all security checks
    scanner.scan_code_files()
    scanner.check_dependencies()
    scanner.check_binary_security() 
    scanner.check_configuration_security()
    
    # Print summary
    scanner.print_summary()
    
    # Save detailed report if requested
    if args.output:
        scanner.save_report(args.output)
    
    # Exit with error code if critical issues found
    critical_issues = sum(1 for issue in scanner.issues if issue.severity == 'critical')
    if critical_issues > 0:
        print(f"\n❌ Exiting with error due to {critical_issues} critical security issues")
        exit(1)
    
    print("\n✅ Security scan completed")

if __name__ == "__main__":
    main()
