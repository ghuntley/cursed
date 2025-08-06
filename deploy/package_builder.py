#!/usr/bin/env python3

"""
CURSED Package Builder
Creates distribution packages for all supported platforms
"""

import os
import sys
import json
import shutil
import tarfile
import zipfile
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class PlatformConfig:
    """Configuration for a target platform"""
    name: str
    target: str
    archive_format: str  # 'tar.gz' or 'zip'
    executable_suffix: str
    installer_type: Optional[str] = None  # 'deb', 'rpm', 'msi', 'pkg'

class PackageBuilder:
    """Builds distribution packages for CURSED"""
    
    def __init__(self, project_root: str, version: str):
        self.project_root = Path(project_root)
        self.version = version
        self.build_dir = self.project_root / "build_packages"
        self.dist_dir = self.project_root / "dist"
        
        # Platform configurations
        self.platforms = [
            PlatformConfig("linux-x64", "x86_64-linux", "tar.gz", "", "deb"),
            PlatformConfig("linux-arm64", "aarch64-linux", "tar.gz", "", "deb"),
            PlatformConfig("macos-x64", "x86_64-macos", "tar.gz", "", "pkg"),
            PlatformConfig("macos-arm64", "aarch64-macos", "tar.gz", "", "pkg"),
            PlatformConfig("windows-x64", "x86_64-windows", "zip", ".exe", "msi"),
            PlatformConfig("wasm32", "wasm32-freestanding", "tar.gz", ".wasm", None)
        ]
        
        # Files to include in packages
        self.package_files = {
            "binaries": [
                "cursed",
                "cursed-fmt", 
                "cursed-lint",
                "cursed-lsp",
                "cursed-pkg",
                "cursed-doc"
            ],
            "documentation": [
                "README.md",
                "LICENSE", 
                "CHANGELOG.md",
                "docs/"
            ],
            "examples": [
                "examples/",
                "stdlib/"
            ]
        }
    
    def clean_build_directory(self):
        """Clean the build directory"""
        if self.build_dir.exists():
            shutil.rmtree(self.build_dir)
        self.build_dir.mkdir(parents=True)
        
        if not self.dist_dir.exists():
            self.dist_dir.mkdir(parents=True)
    
    def build_for_platform(self, platform: PlatformConfig) -> bool:
        """Build CURSED for a specific platform"""
        print(f"🔨 Building for {platform.name}...")
        
        try:
            # Run Zig build for the target platform
            result = subprocess.run([
                "zig", "build", 
                f"-Dtarget={platform.target}",
                "-Doptimize=ReleaseFast",
                "--prefix", str(self.build_dir / platform.name)
            ], cwd=self.project_root, capture_output=True, text=True)
            
            if result.returncode != 0:
                print(f"❌ Build failed for {platform.name}")
                print(f"Error: {result.stderr}")
                return False
            
            print(f"✅ Build successful for {platform.name}")
            return True
            
        except subprocess.CalledProcessError as e:
            print(f"❌ Build error for {platform.name}: {e}")
            return False
    
    def create_package_structure(self, platform: PlatformConfig) -> Path:
        """Create the package directory structure"""
        package_dir = self.build_dir / f"cursed-{self.version}-{platform.name}"
        package_dir.mkdir(parents=True, exist_ok=True)
        
        # Create standard directories
        (package_dir / "bin").mkdir(exist_ok=True)
        (package_dir / "docs").mkdir(exist_ok=True)
        (package_dir / "examples").mkdir(exist_ok=True)
        (package_dir / "stdlib").mkdir(exist_ok=True)
        
        return package_dir
    
    def copy_binaries(self, platform: PlatformConfig, package_dir: Path) -> bool:
        """Copy binaries to package directory"""
        build_bin_dir = self.build_dir / platform.name / "bin"
        
        if not build_bin_dir.exists():
            print(f"⚠️  Binary directory not found for {platform.name}")
            return False
        
        copied_count = 0
        for binary_name in self.package_files["binaries"]:
            binary_file = build_bin_dir / (binary_name + platform.executable_suffix)
            
            if binary_file.exists():
                shutil.copy2(binary_file, package_dir / "bin")
                copied_count += 1
            else:
                print(f"⚠️  Binary {binary_name} not found for {platform.name}")
        
        print(f"  📦 Copied {copied_count} binaries")
        return copied_count > 0
    
    def copy_documentation(self, package_dir: Path):
        """Copy documentation files to package"""
        copied_count = 0
        
        for doc_item in self.package_files["documentation"]:
            source_path = self.project_root / doc_item
            
            if source_path.exists():
                if source_path.is_file():
                    shutil.copy2(source_path, package_dir)
                    copied_count += 1
                elif source_path.is_dir():
                    dest_path = package_dir / source_path.name
                    if dest_path.exists():
                        shutil.rmtree(dest_path)
                    shutil.copytree(source_path, dest_path)
                    copied_count += 1
        
        print(f"  📚 Copied {copied_count} documentation items")
    
    def copy_examples(self, package_dir: Path):
        """Copy examples and stdlib to package"""
        copied_count = 0
        
        for example_item in self.package_files["examples"]:
            source_path = self.project_root / example_item
            
            if source_path.exists() and source_path.is_dir():
                dest_path = package_dir / source_path.name
                if dest_path.exists():
                    shutil.rmtree(dest_path)
                shutil.copytree(source_path, dest_path)
                copied_count += 1
        
        print(f"  📁 Copied {copied_count} example directories")
    
    def create_install_script(self, platform: PlatformConfig, package_dir: Path):
        """Create installation script for the package"""
        if platform.name.startswith("windows"):
            # Create Windows batch install script
            install_script = package_dir / "install.bat"
            install_content = f'''@echo off
echo Installing CURSED {self.version} for Windows...

set INSTALL_DIR=%ProgramFiles%\\CURSED

if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

copy /Y bin\\*.exe "%INSTALL_DIR%\\"

echo Adding CURSED to PATH...
setx PATH "%PATH%;%INSTALL_DIR%" /M

echo CURSED {self.version} installed successfully!
echo Run 'cursed --version' to verify installation.
pause
'''
            install_script.write_text(install_content)
        else:
            # Create Unix shell install script
            install_script = package_dir / "install.sh"
            install_content = f'''#!/bin/bash

# CURSED {self.version} Installation Script

set -e

INSTALL_DIR="${{INSTALL_DIR:-/usr/local/bin}}"

echo "Installing CURSED {self.version}..."
echo "Install directory: $INSTALL_DIR"

# Check permissions
if [ ! -w "$INSTALL_DIR" ]; then
    echo "Error: No write permission to $INSTALL_DIR"
    echo "Try: sudo $0 or set INSTALL_DIR to a writable directory"
    exit 1
fi

# Install binaries
echo "Installing binaries..."
for binary in bin/*; do
    if [ -f "$binary" ] && [ -x "$binary" ]; then
        cp "$binary" "$INSTALL_DIR/"
        echo "  Installed $(basename "$binary")"
    fi
done

# Install stdlib
STDLIB_DIR="${{STDLIB_DIR:-/usr/local/share/cursed/stdlib}}"
if [ ! -d "$STDLIB_DIR" ]; then
    sudo mkdir -p "$STDLIB_DIR"
fi

if [ -d "stdlib" ]; then
    sudo cp -r stdlib/* "$STDLIB_DIR/"
    echo "  Installed standard library to $STDLIB_DIR"
fi

echo ""
echo "CURSED {self.version} installation completed!"
echo "Try: cursed --version"
'''
            install_script.write_text(install_content)
            install_script.chmod(0o755)
    
    def create_uninstall_script(self, platform: PlatformConfig, package_dir: Path):
        """Create uninstallation script"""
        if platform.name.startswith("windows"):
            uninstall_script = package_dir / "uninstall.bat"
            uninstall_content = f'''@echo off
echo Uninstalling CURSED {self.version}...

set INSTALL_DIR=%ProgramFiles%\\CURSED

if exist "%INSTALL_DIR%" (
    rmdir /S /Q "%INSTALL_DIR%"
    echo CURSED files removed.
) else (
    echo CURSED installation directory not found.
)

echo Note: You may need to manually remove CURSED from your PATH.
pause
'''
            uninstall_script.write_text(uninstall_content)
        else:
            uninstall_script = package_dir / "uninstall.sh"
            uninstall_content = f'''#!/bin/bash

# CURSED {self.version} Uninstallation Script

echo "Uninstalling CURSED {self.version}..."

INSTALL_DIR="${{INSTALL_DIR:-/usr/local/bin}}"
STDLIB_DIR="${{STDLIB_DIR:-/usr/local/share/cursed}}"

# Remove binaries
for binary in cursed cursed-fmt cursed-lint cursed-lsp cursed-pkg cursed-doc; do
    if [ -f "$INSTALL_DIR/$binary" ]; then
        rm "$INSTALL_DIR/$binary"
        echo "  Removed $binary"
    fi
done

# Remove stdlib
if [ -d "$STDLIB_DIR" ]; then
    sudo rm -rf "$STDLIB_DIR"
    echo "  Removed standard library"
fi

echo "CURSED {self.version} uninstalled."
'''
            uninstall_script.write_text(uninstall_content)
            uninstall_script.chmod(0o755)
    
    def create_package_info(self, platform: PlatformConfig, package_dir: Path):
        """Create package information file"""
        info_file = package_dir / "PACKAGE_INFO.json"
        
        info_data = {
            "name": "CURSED Programming Language",
            "version": self.version,
            "platform": platform.name,
            "target": platform.target,
            "build_date": datetime.now().isoformat(),
            "description": "Modern systems programming language with Gen Z vibes",
            "homepage": "https://github.com/ghuntley/cursed",
            "license": "MIT",
            "binaries": self.package_files["binaries"],
            "installation": {
                "script": "install.sh" if not platform.name.startswith("windows") else "install.bat",
                "requirements": "No special requirements",
                "suggested_install_dir": "/usr/local/bin" if not platform.name.startswith("windows") else "%ProgramFiles%\\CURSED"
            }
        }
        
        with open(info_file, 'w') as f:
            json.dump(info_data, f, indent=2)
    
    def create_archive(self, platform: PlatformConfig, package_dir: Path) -> Path:
        """Create archive file for the package"""
        archive_name = f"cursed-{self.version}-{platform.name}.{platform.archive_format}"
        archive_path = self.dist_dir / archive_name
        
        if platform.archive_format == "tar.gz":
            with tarfile.open(archive_path, "w:gz") as tar:
                tar.add(package_dir, arcname=package_dir.name)
        elif platform.archive_format == "zip":
            with zipfile.ZipFile(archive_path, 'w', zipfile.ZIP_DEFLATED) as zip_file:
                for file_path in package_dir.rglob('*'):
                    if file_path.is_file():
                        relative_path = file_path.relative_to(package_dir.parent)
                        zip_file.write(file_path, relative_path)
        
        print(f"  📦 Created archive: {archive_name}")
        return archive_path
    
    def create_debian_package(self, platform: PlatformConfig, package_dir: Path) -> Optional[Path]:
        """Create Debian package (if tools available)"""
        if not platform.name.startswith("linux") or platform.installer_type != "deb":
            return None
        
        # Check if dpkg-deb is available
        if subprocess.run(["which", "dpkg-deb"], capture_output=True).returncode != 0:
            print("  ⚠️  dpkg-deb not available, skipping .deb creation")
            return None
        
        # Create Debian package structure
        deb_dir = self.build_dir / f"cursed-{self.version}-{platform.name}-deb"
        deb_dir.mkdir(exist_ok=True)
        
        # DEBIAN control directory
        debian_dir = deb_dir / "DEBIAN"
        debian_dir.mkdir(exist_ok=True)
        
        # Control file
        control_file = debian_dir / "control"
        control_content = f'''Package: cursed
Version: {self.version}
Section: devel
Priority: optional
Architecture: {"amd64" if "x64" in platform.name else "arm64"}
Maintainer: CURSED Team <team@cursed-lang.org>
Description: CURSED Programming Language
 Modern systems programming language with Gen Z vibes.
 Includes compiler, standard library, and development tools.
Homepage: https://github.com/ghuntley/cursed
'''
        control_file.write_text(control_content)
        
        # Install files
        usr_bin = deb_dir / "usr" / "bin"
        usr_bin.mkdir(parents=True)
        
        usr_share = deb_dir / "usr" / "share" / "cursed"
        usr_share.mkdir(parents=True)
        
        # Copy binaries
        for binary_file in (package_dir / "bin").glob("*"):
            if binary_file.is_file():
                shutil.copy2(binary_file, usr_bin)
        
        # Copy stdlib
        if (package_dir / "stdlib").exists():
            shutil.copytree(package_dir / "stdlib", usr_share / "stdlib")
        
        # Build package
        deb_file = self.dist_dir / f"cursed-{self.version}-{platform.name}.deb"
        
        try:
            subprocess.run([
                "dpkg-deb", "--build", str(deb_dir), str(deb_file)
            ], check=True, capture_output=True)
            
            print(f"  📦 Created Debian package: {deb_file.name}")
            return deb_file
            
        except subprocess.CalledProcessError as e:
            print(f"  ❌ Failed to create Debian package: {e}")
            return None
    
    def generate_checksums(self) -> Path:
        """Generate checksums for all packages"""
        checksum_file = self.dist_dir / f"cursed-{self.version}-checksums.sha256"
        
        with open(checksum_file, 'w') as f:
            for package_file in sorted(self.dist_dir.glob("cursed-*")):
                if package_file.name != checksum_file.name:
                    # Calculate SHA256
                    import hashlib
                    sha256_hash = hashlib.sha256()
                    with open(package_file, "rb") as pf:
                        for chunk in iter(lambda: pf.read(4096), b""):
                            sha256_hash.update(chunk)
                    
                    f.write(f"{sha256_hash.hexdigest()}  {package_file.name}\\n")
        
        print(f"✅ Generated checksums: {checksum_file.name}")
        return checksum_file
    
    def build_all_packages(self) -> Dict[str, bool]:
        """Build packages for all platforms"""
        print(f"🚀 Building CURSED {self.version} packages...")
        
        self.clean_build_directory()
        results = {}
        
        for platform in self.platforms:
            success = True
            
            try:
                # Build for platform
                if not self.build_for_platform(platform):
                    results[platform.name] = False
                    continue
                
                # Create package structure
                package_dir = self.create_package_structure(platform)
                
                # Copy files
                if not self.copy_binaries(platform, package_dir):
                    results[platform.name] = False
                    continue
                
                self.copy_documentation(package_dir)
                self.copy_examples(package_dir)
                
                # Create scripts and info
                self.create_install_script(platform, package_dir)
                self.create_uninstall_script(platform, package_dir)
                self.create_package_info(platform, package_dir)
                
                # Create archive
                archive_path = self.create_archive(platform, package_dir)
                
                # Create native installer if supported
                if platform.installer_type == "deb":
                    self.create_debian_package(platform, package_dir)
                
                results[platform.name] = True
                
            except Exception as e:
                print(f"❌ Failed to build package for {platform.name}: {e}")
                results[platform.name] = False
        
        # Generate checksums
        self.generate_checksums()
        
        return results
    
    def print_summary(self, results: Dict[str, bool]):
        """Print build summary"""
        successful = sum(1 for success in results.values() if success)
        total = len(results)
        
        print(f"\\n📦 Package Build Summary")
        print("=" * 50)
        print(f"Total Platforms: {total}")
        print(f"Successful: {successful}")
        print(f"Failed: {total - successful}")
        print()
        
        for platform, success in results.items():
            status = "✅" if success else "❌"
            print(f"{status} {platform}")
        
        if successful == total:
            print("\\n🎉 All packages built successfully!")
        elif successful > 0:
            print(f"\\n⚠️  {successful}/{total} packages built successfully")
        else:
            print("\\n❌ All package builds failed")
        
        # List generated files
        if any(results.values()):
            print("\\n📁 Generated Files:")
            for file_path in sorted(self.dist_dir.glob("cursed-*")):
                size_mb = file_path.stat().st_size / 1024 / 1024
                print(f"  📦 {file_path.name} ({size_mb:.1f}MB)")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Package Builder")
    parser.add_argument("--project-root", default=".", help="Project root directory")
    parser.add_argument("--version", required=True, help="Version to build")
    parser.add_argument("--platforms", nargs="+", 
                       choices=["linux-x64", "linux-arm64", "macos-x64", "macos-arm64", "windows-x64", "wasm32"],
                       help="Specific platforms to build (default: all)")
    parser.add_argument("--clean", action="store_true", help="Clean build directory before starting")
    
    args = parser.parse_args()
    
    builder = PackageBuilder(args.project_root, args.version)
    
    # Filter platforms if specified
    if args.platforms:
        builder.platforms = [p for p in builder.platforms if p.name in args.platforms]
    
    # Clean if requested
    if args.clean:
        builder.clean_build_directory()
    
    # Build packages
    results = builder.build_all_packages()
    
    # Print summary
    builder.print_summary(results)
    
    # Exit with error if any builds failed
    if not all(results.values()):
        sys.exit(1)

if __name__ == "__main__":
    main()
