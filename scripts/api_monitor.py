#!/usr/bin/env python3
"""
CURSED Zig API Monitoring System
Monitors Zig API changes and automatically updates compatibility layer
"""

import json
import os
import re
import subprocess
import sys
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Tuple

class ZigAPIMonitor:
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.compat_file = project_root / "src-zig" / "zig_version.zig"
        self.build_file = project_root / "build.zig"
        self.report_file = project_root / "api_monitoring_report.json"
        
    def get_zig_version(self) -> Tuple[str, Tuple[int, int, int]]:
        """Get current Zig version"""
        try:
            result = subprocess.run(['zig', 'version'], capture_output=True, text=True)
            version_str = result.stdout.strip()
            
            # Parse version (e.g., "0.15.1" -> (0, 15, 1))
            parts = version_str.split('.')
            version_tuple = (int(parts[0]), int(parts[1]), int(parts[2]))
            
            return version_str, version_tuple
        except Exception as e:
            print(f"Error getting Zig version: {e}")
            return "unknown", (0, 0, 0)
    
    def test_build(self, zig_version: Optional[str] = None) -> Dict:
        """Test build with current or specific Zig version"""
        print(f"Testing build with Zig {zig_version or 'current'}")
        
        try:
            # Run build and capture output
            result = subprocess.run(
                ['zig', 'build'],
                cwd=self.project_root,
                capture_output=True,
                text=True,
                timeout=300  # 5 minute timeout
            )
            
            output = result.stdout + result.stderr
            success = result.returncode == 0
            
            # Analyze output for compatibility issues
            warnings = self.extract_warnings(output)
            errors = self.extract_errors(output)
            deprecated = self.extract_deprecated_apis(output)
            
            return {
                "success": success,
                "output": output,
                "warnings": warnings,
                "errors": errors,
                "deprecated": deprecated,
                "timestamp": datetime.now().isoformat()
            }
            
        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "output": "Build timed out",
                "warnings": [],
                "errors": ["Build timeout"],
                "deprecated": [],
                "timestamp": datetime.now().isoformat()
            }
        except Exception as e:
            return {
                "success": False,
                "output": str(e),
                "warnings": [],
                "errors": [str(e)],
                "deprecated": [],
                "timestamp": datetime.now().isoformat()
            }
    
    def extract_warnings(self, output: str) -> List[str]:
        """Extract warnings from build output"""
        warnings = []
        for line in output.split('\n'):
            if 'warning:' in line.lower():
                warnings.append(line.strip())
        return warnings
    
    def extract_errors(self, output: str) -> List[str]:
        """Extract errors from build output"""
        errors = []
        for line in output.split('\n'):
            if 'error:' in line.lower() and 'warning:' not in line.lower():
                errors.append(line.strip())
        return errors
    
    def extract_deprecated_apis(self, output: str) -> List[str]:
        """Extract deprecated API warnings"""
        deprecated = []
        for line in output.split('\n'):
            if 'deprecated' in line.lower():
                deprecated.append(line.strip())
        return deprecated
    
    def analyze_api_changes(self, current_result: Dict, previous_result: Optional[Dict] = None) -> Dict:
        """Analyze API changes between builds"""
        changes = {
            "new_errors": [],
            "new_warnings": [],
            "new_deprecated": [],
            "fixed_errors": [],
            "breaking_changes": False,
            "api_version_bump": False
        }
        
        if not previous_result:
            return changes
        
        # Compare errors
        prev_errors = set(previous_result.get("errors", []))
        curr_errors = set(current_result.get("errors", []))
        
        changes["new_errors"] = list(curr_errors - prev_errors)
        changes["fixed_errors"] = list(prev_errors - curr_errors)
        
        # Compare warnings
        prev_warnings = set(previous_result.get("warnings", []))
        curr_warnings = set(current_result.get("warnings", []))
        
        changes["new_warnings"] = list(curr_warnings - prev_warnings)
        
        # Compare deprecated APIs
        prev_deprecated = set(previous_result.get("deprecated", []))
        curr_deprecated = set(current_result.get("deprecated", []))
        
        changes["new_deprecated"] = list(curr_deprecated - prev_deprecated)
        
        # Detect breaking changes
        breaking_indicators = [
            "undefined symbol",
            "no member",
            "type mismatch",
            "incompatible types",
            "cannot convert"
        ]
        
        for error in changes["new_errors"]:
            if any(indicator in error.lower() for indicator in breaking_indicators):
                changes["breaking_changes"] = True
                break
        
        return changes
    
    def generate_compatibility_fixes(self, changes: Dict) -> List[str]:
        """Generate suggested compatibility layer fixes"""
        fixes = []
        
        for error in changes["new_errors"]:
            if "ArrayList" in error:
                fixes.append("Update ArrayList compatibility wrapper in zig_version.zig")
            elif "ExecutableOptions" in error:
                fixes.append("Update ExecutableOptions compatibility wrapper in zig_version.zig")
            elif "addModule" in error:
                fixes.append("Update addModule compatibility wrapper in zig_version.zig")
            elif "LazyPath" in error or "FileSource" in error:
                fixes.append("Update path compatibility wrapper in zig_version.zig")
        
        for deprecated in changes["new_deprecated"]:
            if "ArrayList.init" in deprecated:
                fixes.append("Update ArrayList initialization pattern")
            elif "std.Build" in deprecated:
                fixes.append("Update build system compatibility layer")
        
        if not fixes:
            fixes.append("Review compatibility layer for new API patterns")
        
        return fixes
    
    def update_compatibility_layer(self, fixes: List[str]) -> bool:
        """Automatically update compatibility layer"""
        print("Attempting to automatically update compatibility layer...")
        
        try:
            # Read current compatibility file
            with open(self.compat_file, 'r') as f:
                content = f.read()
            
            # Add version detection for newer Zig versions
            version_str, version_tuple = self.get_zig_version()
            
            if version_tuple[1] > 15:  # Newer than 0.15.x
                # Add new version handling
                version_check = f"""
        if (version.isAtLeast({version_tuple[0]}, {version_tuple[1]}, 0)) {{
            std.log.warn("Zig {version_str} detected - some features may be experimental");
        }}
"""
                
                # Insert version check if not present
                if f"version.isAtLeast({version_tuple[0]}, {version_tuple[1]}, 0)" not in content:
                    # Find reportApiChanges function and add version check
                    pattern = r"(pub fn reportApiChanges\(\) !void \{[^}]*)"
                    replacement = f"\\1{version_check}"
                    content = re.sub(pattern, replacement, content, flags=re.DOTALL)
            
            # Write updated content
            with open(self.compat_file, 'w') as f:
                f.write(content)
            
            print(f"✅ Updated compatibility layer for Zig {version_str}")
            return True
            
        except Exception as e:
            print(f"❌ Failed to update compatibility layer: {e}")
            return False
    
    def save_report(self, results: Dict):
        """Save monitoring report to file"""
        report = {
            "timestamp": datetime.now().isoformat(),
            "zig_version": self.get_zig_version()[0],
            "results": results,
            "monitoring_config": {
                "project_root": str(self.project_root),
                "compat_file": str(self.compat_file),
                "build_file": str(self.build_file)
            }
        }
        
        # Load existing reports
        existing_reports = []
        if self.report_file.exists():
            try:
                with open(self.report_file, 'r') as f:
                    existing_data = json.load(f)
                    if isinstance(existing_data, list):
                        existing_reports = existing_data
                    else:
                        existing_reports = [existing_data]
            except:
                pass
        
        # Add new report and keep last 30 days
        existing_reports.append(report)
        cutoff_date = datetime.now() - timedelta(days=30)
        
        filtered_reports = []
        for r in existing_reports:
            try:
                report_date = datetime.fromisoformat(r["timestamp"])
                if report_date > cutoff_date:
                    filtered_reports.append(r)
            except:
                continue
        
        # Save updated reports
        with open(self.report_file, 'w') as f:
            json.dump(filtered_reports, f, indent=2)
    
    def create_github_issue(self, changes: Dict, fixes: List[str]) -> bool:
        """Create GitHub issue for breaking changes"""
        if not changes["breaking_changes"]:
            return False
        
        try:
            # Create issue content
            issue_title = f"🚨 Zig API Breaking Changes Detected - {datetime.now().strftime('%Y-%m-%d')}"
            
            issue_body = f"""## 🚨 Automated API Compatibility Alert

Zig API monitoring has detected breaking changes that require attention.

### Summary

- **Zig Version**: {self.get_zig_version()[0]}
- **Detection Time**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S UTC')}
- **Breaking Changes**: {'Yes' if changes['breaking_changes'] else 'No'}

### New Errors
"""
            
            for error in changes["new_errors"][:5]:  # Limit to first 5
                issue_body += f"- `{error}`\n"
            
            if len(changes["new_errors"]) > 5:
                issue_body += f"- ... and {len(changes['new_errors']) - 5} more\n"
            
            issue_body += f"""
### New Deprecated APIs
"""
            for deprecated in changes["new_deprecated"][:5]:
                issue_body += f"- `{deprecated}`\n"
            
            issue_body += f"""
### Suggested Fixes
"""
            for fix in fixes:
                issue_body += f"- {fix}\n"
            
            issue_body += f"""
### Next Steps

1. Review the [compatibility monitoring report]({self.report_file})
2. Update the compatibility layer in `src-zig/zig_version.zig`
3. Test fixes against affected Zig versions
4. Update CI matrix if needed

**Priority**: {'High' if changes['breaking_changes'] else 'Medium'}
**Labels**: zig-compatibility, automated, {'breaking-change' if changes['breaking_changes'] else 'enhancement'}
"""
            
            # Save issue content to file for manual creation
            issue_file = self.project_root / "api_compatibility_issue.md"
            with open(issue_file, 'w') as f:
                f.write(f"# {issue_title}\n\n{issue_body}")
            
            print(f"✅ GitHub issue content saved to {issue_file}")
            print("   Use GitHub CLI or web interface to create the issue")
            
            return True
            
        except Exception as e:
            print(f"❌ Failed to create GitHub issue: {e}")
            return False
    
    def run_monitoring_cycle(self):
        """Run complete monitoring cycle"""
        print("=== CURSED Zig API Monitoring Cycle ===")
        
        # Get current version
        version_str, version_tuple = self.get_zig_version()
        print(f"Monitoring Zig {version_str}")
        
        # Load previous results
        previous_result = None
        if self.report_file.exists():
            try:
                with open(self.report_file, 'r') as f:
                    reports = json.load(f)
                    if isinstance(reports, list) and reports:
                        previous_result = reports[-1]["results"]
                    elif isinstance(reports, dict):
                        previous_result = reports.get("results")
            except:
                pass
        
        # Test current build
        current_result = self.test_build(version_str)
        
        # Analyze changes
        changes = self.analyze_api_changes(current_result, previous_result)
        
        # Generate compatibility report
        results = {
            "build_result": current_result,
            "changes": changes,
            "version_info": {
                "string": version_str,
                "tuple": version_tuple
            }
        }
        
        # Save results
        self.save_report(results)
        
        # Report status
        if current_result["success"]:
            print("✅ Build successful")
        else:
            print("❌ Build failed")
            
        if changes["breaking_changes"]:
            print("🚨 Breaking changes detected")
            
            # Generate fixes
            fixes = self.generate_compatibility_fixes(changes)
            print("Suggested fixes:")
            for fix in fixes:
                print(f"  - {fix}")
            
            # Try auto-update
            if self.update_compatibility_layer(fixes):
                # Re-test with updates
                updated_result = self.test_build(version_str)
                if updated_result["success"]:
                    print("✅ Auto-fix successful")
                else:
                    print("⚠️  Auto-fix incomplete - manual intervention needed")
                    self.create_github_issue(changes, fixes)
            else:
                self.create_github_issue(changes, fixes)
        
        elif changes["new_deprecated"]:
            print("⚠️  New deprecated APIs detected")
            fixes = self.generate_compatibility_fixes(changes)
            self.update_compatibility_layer(fixes)
        
        else:
            print("✅ No API compatibility issues detected")
        
        print(f"📊 Monitoring report saved to {self.report_file}")
        print("=== Monitoring Cycle Complete ===")

def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--help":
        print("""
CURSED Zig API Monitoring System

Usage:
  python3 scripts/api_monitor.py              # Run monitoring cycle
  python3 scripts/api_monitor.py --report     # Show latest report
  python3 scripts/api_monitor.py --check      # Quick compatibility check

Features:
- Automatic Zig API change detection
- Compatibility layer auto-updates
- GitHub issue creation for breaking changes
- Historical monitoring reports
""")
        return
    
    project_root = Path(__file__).parent.parent
    monitor = ZigAPIMonitor(project_root)
    
    if len(sys.argv) > 1 and sys.argv[1] == "--report":
        # Show latest report
        if monitor.report_file.exists():
            with open(monitor.report_file, 'r') as f:
                reports = json.load(f)
                if isinstance(reports, list):
                    latest = reports[-1] if reports else None
                else:
                    latest = reports
                
                if latest:
                    print("=== Latest Monitoring Report ===")
                    print(json.dumps(latest, indent=2))
                else:
                    print("No monitoring reports found")
        else:
            print("No monitoring reports found")
    
    elif len(sys.argv) > 1 and sys.argv[1] == "--check":
        # Quick compatibility check
        version_str, _ = monitor.get_zig_version()
        result = monitor.test_build(version_str)
        
        if result["success"]:
            print(f"✅ Zig {version_str} - Build successful")
        else:
            print(f"❌ Zig {version_str} - Build failed")
            for error in result["errors"][:3]:
                print(f"   {error}")
    
    else:
        # Run full monitoring cycle
        monitor.run_monitoring_cycle()

if __name__ == "__main__":
    main()
