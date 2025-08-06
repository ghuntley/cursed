#!/usr/bin/env python3

"""
CURSED Release Automation System
Handles version management, release tagging, and deployment automation
"""

import os
import sys
import json
import subprocess
import argparse
import semver
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional

class ReleaseManager:
    """Manages CURSED releases, versioning, and deployment"""
    
    def __init__(self, project_root: str):
        self.project_root = Path(project_root)
        self.version_file = self.project_root / "VERSION"
        self.changelog_file = self.project_root / "CHANGELOG.md"
        self.config_file = self.project_root / "deploy" / "release_config.json"
        
    def get_current_version(self) -> str:
        """Get the current version from VERSION file"""
        if self.version_file.exists():
            return self.version_file.read_text().strip()
        return "0.1.0"
    
    def bump_version(self, bump_type: str) -> str:
        """Bump version according to semantic versioning"""
        current_version = self.get_current_version()
        
        if bump_type == "major":
            new_version = semver.bump_major(current_version)
        elif bump_type == "minor":
            new_version = semver.bump_minor(current_version)
        elif bump_type == "patch":
            new_version = semver.bump_patch(current_version)
        else:
            raise ValueError(f"Invalid bump type: {bump_type}")
        
        self.version_file.write_text(new_version)
        return new_version
    
    def create_release_tag(self, version: str, message: str = None) -> bool:
        """Create a git tag for the release"""
        tag_name = f"v{version}"
        
        if message is None:
            message = f"Release {version}"
        
        try:
            # Create annotated tag
            subprocess.run([
                "git", "tag", "-a", tag_name, "-m", message
            ], check=True, cwd=self.project_root)
            
            print(f"✅ Created tag: {tag_name}")
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to create tag: {e}")
            return False
    
    def update_changelog(self, version: str, changes: List[str]):
        """Update CHANGELOG.md with new release information"""
        changelog_content = ""
        
        if self.changelog_file.exists():
            changelog_content = self.changelog_file.read_text()
        
        # Create new changelog entry
        date_str = datetime.now().strftime("%Y-%m-%d")
        new_entry = f"\n## [{version}] - {date_str}\n\n"
        
        for change in changes:
            new_entry += f"- {change}\n"
        
        new_entry += "\n"
        
        # Insert new entry at the top
        if "## [" in changelog_content:
            # Insert after the header
            parts = changelog_content.split("## [", 1)
            changelog_content = parts[0] + new_entry + "## [" + parts[1]
        else:
            # Create initial changelog
            header = "# Changelog\n\nAll notable changes to CURSED will be documented in this file.\n\n"
            changelog_content = header + new_entry + changelog_content
        
        self.changelog_file.write_text(changelog_content)
        print(f"✅ Updated changelog for version {version}")
    
    def validate_release_readiness(self) -> Dict[str, bool]:
        """Check if the project is ready for release"""
        checks = {}
        
        # Check if on main branch
        try:
            branch = subprocess.run([
                "git", "branch", "--show-current"
            ], capture_output=True, text=True, check=True, cwd=self.project_root).stdout.strip()
            checks["on_main_branch"] = branch in ["main", "trunk", "master"]
        except subprocess.CalledProcessError:
            checks["on_main_branch"] = False
        
        # Check for uncommitted changes
        try:
            status = subprocess.run([
                "git", "status", "--porcelain"
            ], capture_output=True, text=True, check=True, cwd=self.project_root).stdout.strip()
            checks["clean_working_tree"] = len(status) == 0
        except subprocess.CalledProcessError:
            checks["clean_working_tree"] = False
        
        # Check if build passes
        try:
            subprocess.run([
                "zig", "build"
            ], check=True, cwd=self.project_root, capture_output=True)
            checks["build_passes"] = True
        except subprocess.CalledProcessError:
            checks["build_passes"] = False
        
        # Check if tests pass
        try:
            subprocess.run([
                "zig", "build", "test"
            ], check=True, cwd=self.project_root, capture_output=True)
            checks["tests_pass"] = True
        except subprocess.CalledProcessError:
            checks["tests_pass"] = False
        
        return checks
    
    def create_release_branch(self, version: str) -> bool:
        """Create a release branch for the given version"""
        branch_name = f"release/{version}"
        
        try:
            subprocess.run([
                "git", "checkout", "-b", branch_name
            ], check=True, cwd=self.project_root)
            
            print(f"✅ Created release branch: {branch_name}")
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to create release branch: {e}")
            return False
    
    def generate_release_notes(self, version: str) -> str:
        """Generate release notes for the version"""
        # Get commits since last tag
        try:
            last_tag = subprocess.run([
                "git", "describe", "--tags", "--abbrev=0"
            ], capture_output=True, text=True, check=True, cwd=self.project_root).stdout.strip()
        except subprocess.CalledProcessError:
            last_tag = ""
        
        # Get commit messages
        if last_tag:
            commit_range = f"{last_tag}..HEAD"
        else:
            commit_range = "--all"
        
        try:
            commits = subprocess.run([
                "git", "log", commit_range, "--oneline", "--no-merges"
            ], capture_output=True, text=True, check=True, cwd=self.project_root).stdout.strip()
        except subprocess.CalledProcessError:
            commits = ""
        
        # Generate release notes
        notes = f"# CURSED {version} Release Notes\n\n"
        notes += f"Released: {datetime.now().strftime('%Y-%m-%d')}\n\n"
        
        if commits:
            notes += "## Changes\n\n"
            for line in commits.split('\n'):
                if line.strip():
                    commit_hash, *message_parts = line.split(' ', 1)
                    message = ' '.join(message_parts) if message_parts else ''
                    notes += f"- {message} ({commit_hash})\n"
        else:
            notes += "## Changes\n\n- Initial release\n"
        
        notes += "\n## Installation\n\n"
        notes += "Download the appropriate package for your platform from the releases page.\n\n"
        notes += "### Quick Install\n"
        notes += "```bash\n"
        notes += f"curl -sSL https://github.com/ghuntley/cursed/releases/download/v{version}/install.sh | bash\n"
        notes += "```\n"
        
        return notes
    
    def deploy_to_staging(self, version: str) -> bool:
        """Deploy the release to staging environment"""
        print(f"🚀 Deploying {version} to staging...")
        
        # Run production pipeline
        pipeline_script = self.project_root / "deploy" / "production_pipeline.sh"
        
        if not pipeline_script.exists():
            print("❌ Production pipeline script not found")
            return False
        
        try:
            env = os.environ.copy()
            env["VERSION"] = version
            env["BUILD_NUMBER"] = datetime.now().strftime("%Y%m%d%H%M%S")
            
            subprocess.run([
                str(pipeline_script)
            ], check=True, cwd=self.project_root, env=env)
            
            print(f"✅ Successfully deployed {version} to staging")
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Staging deployment failed: {e}")
            return False
    
    def create_github_release(self, version: str, release_notes: str) -> bool:
        """Create a GitHub release"""
        tag_name = f"v{version}"
        
        # Check if gh CLI is available
        try:
            subprocess.run(["gh", "--version"], check=True, capture_output=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            print("❌ GitHub CLI (gh) not found. Please install it to create GitHub releases.")
            return False
        
        try:
            # Create release notes file
            notes_file = self.project_root / f"release_notes_{version}.md"
            notes_file.write_text(release_notes)
            
            # Create GitHub release
            subprocess.run([
                "gh", "release", "create", tag_name,
                "--title", f"CURSED {version}",
                "--notes-file", str(notes_file),
                "--draft"
            ], check=True, cwd=self.project_root)
            
            # Clean up
            notes_file.unlink()
            
            print(f"✅ Created GitHub release: {tag_name}")
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to create GitHub release: {e}")
            return False
    
    def upload_release_assets(self, version: str) -> bool:
        """Upload release assets to GitHub"""
        tag_name = f"v{version}"
        
        # Find release artifacts
        artifacts_dir = self.project_root / "dist"
        if not artifacts_dir.exists():
            print("❌ No distribution artifacts found")
            return False
        
        try:
            # Upload each artifact
            for artifact in artifacts_dir.glob("cursed-*"):
                if artifact.is_file():
                    subprocess.run([
                        "gh", "release", "upload", tag_name, str(artifact)
                    ], check=True, cwd=self.project_root)
                    print(f"✅ Uploaded: {artifact.name}")
            
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to upload release assets: {e}")
            return False

def main():
    parser = argparse.ArgumentParser(description="CURSED Release Automation")
    parser.add_argument("command", choices=[
        "bump", "tag", "release", "deploy", "validate", "notes"
    ], help="Command to execute")
    
    parser.add_argument("--type", choices=["major", "minor", "patch"], 
                       default="patch", help="Version bump type")
    parser.add_argument("--version", help="Specific version to use")
    parser.add_argument("--message", help="Release message")
    parser.add_argument("--dry-run", action="store_true", 
                       help="Show what would be done without making changes")
    
    args = parser.parse_args()
    
    # Find project root
    current_dir = Path.cwd()
    project_root = current_dir
    
    # Look for build.zig to confirm project root
    while project_root != project_root.parent:
        if (project_root / "build.zig").exists():
            break
        project_root = project_root.parent
    else:
        print("❌ Could not find project root (no build.zig found)")
        sys.exit(1)
    
    manager = ReleaseManager(str(project_root))
    
    if args.command == "validate":
        print("🔍 Validating release readiness...")
        checks = manager.validate_release_readiness()
        
        all_passed = True
        for check, passed in checks.items():
            status = "✅" if passed else "❌"
            print(f"{status} {check.replace('_', ' ').title()}")
            if not passed:
                all_passed = False
        
        if all_passed:
            print("\n🎉 All checks passed! Ready for release.")
        else:
            print("\n⚠️  Some checks failed. Please fix before releasing.")
            sys.exit(1)
    
    elif args.command == "bump":
        if args.dry_run:
            current = manager.get_current_version()
            print(f"Would bump {current} -> {semver.bump_patch(current) if args.type == 'patch' else 'calculated'}")
        else:
            new_version = manager.bump_version(args.type)
            print(f"✅ Bumped version to {new_version}")
    
    elif args.command == "tag":
        version = args.version or manager.get_current_version()
        if args.dry_run:
            print(f"Would create tag v{version}")
        else:
            manager.create_release_tag(version, args.message)
    
    elif args.command == "notes":
        version = args.version or manager.get_current_version()
        notes = manager.generate_release_notes(version)
        print(notes)
    
    elif args.command == "deploy":
        version = args.version or manager.get_current_version()
        if args.dry_run:
            print(f"Would deploy version {version} to staging")
        else:
            manager.deploy_to_staging(version)
    
    elif args.command == "release":
        # Full release process
        print("🚀 Starting full release process...")
        
        # Validate readiness
        checks = manager.validate_release_readiness()
        if not all(checks.values()):
            print("❌ Release validation failed")
            sys.exit(1)
        
        # Get or bump version
        if args.version:
            version = args.version
            if not args.dry_run:
                manager.version_file.write_text(version)
        else:
            if args.dry_run:
                version = "x.y.z"
            else:
                version = manager.bump_version(args.type)
        
        print(f"📦 Releasing version {version}")
        
        if args.dry_run:
            print("Would execute:")
            print(f"  - Create release tag v{version}")
            print(f"  - Deploy to staging")
            print(f"  - Create GitHub release")
            print(f"  - Upload release assets")
        else:
            # Create tag
            if not manager.create_release_tag(version, args.message):
                sys.exit(1)
            
            # Deploy to staging
            if not manager.deploy_to_staging(version):
                sys.exit(1)
            
            # Generate release notes
            release_notes = manager.generate_release_notes(version)
            
            # Create GitHub release
            if not manager.create_github_release(version, release_notes):
                print("⚠️  GitHub release creation failed, continuing...")
            
            # Upload assets
            if not manager.upload_release_assets(version):
                print("⚠️  Asset upload failed, continuing...")
            
            print(f"🎉 Release {version} completed successfully!")

if __name__ == "__main__":
    main()
