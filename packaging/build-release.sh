#!/bin/bash
# CURSED Compiler - Production Release Build Script
# Enterprise-ready multi-platform packaging system

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

# Version management
VERSION_FILE="packaging/VERSION"
if [[ ! -f "$VERSION_FILE" ]]; then
    echo "1.0.0" > "$VERSION_FILE"
fi
VERSION=$(cat "$VERSION_FILE")
BUILD_NUMBER=${BUILD_NUMBER:-$(date +"%Y%m%d%H%M")}
FULL_VERSION="${VERSION}.${BUILD_NUMBER}"

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"
BUILD_DIR="$PROJECT_ROOT/build-release"
PACKAGING_DIR="$PROJECT_ROOT/packaging"

# Build configuration
BUILD_MODE=${BUILD_MODE:-release}
ENABLE_LTO=${ENABLE_LTO:-true}
ENABLE_STRIP=${ENABLE_STRIP:-true}
PARALLEL_JOBS=${PARALLEL_JOBS:-$(nproc)}

# Target platforms
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
    "wasm32-wasi"
)

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [INFO] $*"
}

error() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [ERROR] $*" >&2
    exit 1
}

cleanup() {
    log "Cleaning up build artifacts..."
    rm -rf "$BUILD_DIR"
}

# Setup signal handlers
trap cleanup EXIT INT TERM

# ============================================================================
# BUILD FUNCTIONS
# ============================================================================

setup_build_env() {
    log "Setting up build environment..."
    
    # Clean previous builds
    rm -rf "$DIST_DIR" "$BUILD_DIR"
    mkdir -p "$DIST_DIR" "$BUILD_DIR"
    
    # Set optimization flags
    export CARGO_PROFILE_RELEASE_LTO="$ENABLE_LTO"
    export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
    export CARGO_PROFILE_RELEASE_PANIC="abort"
    
    if [[ "$ENABLE_STRIP" == "true" ]]; then
        export CARGO_PROFILE_RELEASE_STRIP="symbols"
    fi
    
    # Parallel build configuration
    export CARGO_BUILD_JOBS="$PARALLEL_JOBS"
    export NINJA_MAX_JOBS="$PARALLEL_JOBS"
    
    log "Build environment configured"
    log "  Version: $FULL_VERSION"
    log "  Mode: $BUILD_MODE"
    log "  LTO: $ENABLE_LTO"
    log "  Strip: $ENABLE_STRIP"
    log "  Jobs: $PARALLEL_JOBS"
}

build_target() {
    local target="$1"
    local output_dir="$BUILD_DIR/$target"
    
    log "Building for target: $target"
    mkdir -p "$output_dir"
    
    cd "$PROJECT_ROOT"
    
    # Determine build command based on target
    if [[ "$target" == "wasm32-wasi" ]]; then
        # WebAssembly build
        log "Building WebAssembly target..."
        zig build -Dtarget="$target" -Doptimize=ReleaseSmall --prefix "$output_dir"
    else
        # Native builds
        case "$target" in
            *-linux-*)
                log "Building Linux target..."
                zig build -Dtarget="$target" -Doptimize=ReleaseFast --prefix "$output_dir"
                ;;
            *-darwin-*)
                log "Building macOS target..."
                zig build -Dtarget="$target" -Doptimize=ReleaseFast --prefix "$output_dir"
                ;;
            *-windows-*)
                log "Building Windows target..."
                zig build -Dtarget="$target" -Doptimize=ReleaseFast --prefix "$output_dir"
                ;;
            *)
                log "Building generic target..."
                zig build -Dtarget="$target" -Doptimize=ReleaseFast --prefix "$output_dir"
                ;;
        esac
    fi
    
    # Verify build artifacts
    if [[ ! -d "$output_dir/bin" ]]; then
        error "Build failed for target $target: no bin directory found"
    fi
    
    log "Build completed for target: $target"
}

package_target() {
    local target="$1"
    local build_dir="$BUILD_DIR/$target"
    local package_dir="$DIST_DIR/$target"
    
    log "Packaging target: $target"
    mkdir -p "$package_dir"
    
    # Copy binaries
    cp -r "$build_dir/bin" "$package_dir/"
    
    # Add executable suffix for Windows
    if [[ "$target" == *-windows-* ]]; then
        for bin in "$package_dir/bin"/*; do
            if [[ ! "$bin" == *.exe ]]; then
                mv "$bin" "${bin}.exe"
            fi
        done
    fi
    
    # Copy standard library
    if [[ -d "$PROJECT_ROOT/stdlib" ]]; then
        cp -r "$PROJECT_ROOT/stdlib" "$package_dir/"
    fi
    
    # Copy documentation
    cp -r "$PROJECT_ROOT/docs" "$package_dir/" 2>/dev/null || true
    
    # Copy license and readme
    cp "$PROJECT_ROOT/LICENSE" "$package_dir/" 2>/dev/null || echo "# CURSED Compiler License" > "$package_dir/LICENSE"
    cp "$PROJECT_ROOT/README.md" "$package_dir/" 2>/dev/null || echo "# CURSED Compiler" > "$package_dir/README.md"
    
    # Create version info
    cat > "$package_dir/VERSION" << EOF
CURSED Compiler Version $FULL_VERSION
Target: $target
Build Date: $(date -u '+%Y-%m-%d %H:%M:%S UTC')
Build Number: $BUILD_NUMBER
EOF
    
    # Create archive
    cd "$DIST_DIR"
    case "$target" in
        *-windows-*)
            # Windows: ZIP archive
            zip -r "cursed-${FULL_VERSION}-${target}.zip" "$target/"
            ;;
        *)
            # Unix-like: tar.gz archive
            tar -czf "cursed-${FULL_VERSION}-${target}.tar.gz" "$target/"
            ;;
    esac
    
    log "Package created for target: $target"
}

# ============================================================================
# NATIVE PACKAGE FORMATS
# ============================================================================

create_deb_package() {
    log "Creating Debian package..."
    
    local deb_dir="$BUILD_DIR/deb"
    local control_dir="$deb_dir/DEBIAN"
    local install_dir="$deb_dir/usr"
    
    mkdir -p "$control_dir" "$install_dir/bin" "$install_dir/share/cursed"
    
    # Copy binaries (Linux x86_64)
    cp "$BUILD_DIR/x86_64-unknown-linux-gnu/bin"/* "$install_dir/bin/"
    
    # Copy standard library and documentation
    cp -r "$PROJECT_ROOT/stdlib" "$install_dir/share/cursed/"
    cp -r "$PROJECT_ROOT/docs" "$install_dir/share/cursed/" 2>/dev/null || true
    
    # Create control file
    cat > "$control_dir/control" << EOF
Package: cursed
Version: $VERSION
Section: devel
Priority: optional
Architecture: amd64
Maintainer: CURSED Development Team <dev@cursed.dev>
Description: CURSED Programming Language Compiler
 A modern, safe, and performant programming language compiler
 featuring advanced type system, memory safety, and concurrency.
 .
 This package includes the CURSED compiler, standard library,
 and development tools.
Depends: libc6 (>= 2.31), libllvm18 | libllvm17 | libllvm16
EOF
    
    # Create postinst script
    cat > "$control_dir/postinst" << 'EOF'
#!/bin/bash
set -e

# Add cursed to PATH if not already present
if ! echo "$PATH" | grep -q "/usr/bin"; then
    echo 'export PATH="/usr/bin:$PATH"' >> /etc/environment
fi

# Create cursed cache directory
mkdir -p /var/cache/cursed
chmod 755 /var/cache/cursed

echo "CURSED compiler installed successfully!"
echo "Run 'cursed --version' to verify installation."
EOF
    
    chmod 755 "$control_dir/postinst"
    
    # Build package
    dpkg-deb --build "$deb_dir" "$DIST_DIR/cursed_${VERSION}_amd64.deb"
    
    log "Debian package created: cursed_${VERSION}_amd64.deb"
}

create_rpm_package() {
    log "Creating RPM package..."
    
    local rpm_dir="$BUILD_DIR/rpm"
    local spec_file="$rpm_dir/cursed.spec"
    
    mkdir -p "$rpm_dir"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    mkdir -p "$rpm_dir/BUILD/usr"/{bin,share/cursed}
    
    # Copy binaries
    cp "$BUILD_DIR/x86_64-unknown-linux-gnu/bin"/* "$rpm_dir/BUILD/usr/bin/"
    
    # Copy standard library and documentation
    cp -r "$PROJECT_ROOT/stdlib" "$rpm_dir/BUILD/usr/share/cursed/"
    cp -r "$PROJECT_ROOT/docs" "$rpm_dir/BUILD/usr/share/cursed/" 2>/dev/null || true
    
    # Create spec file
    cat > "$spec_file" << EOF
Name: cursed
Version: $VERSION
Release: 1
Summary: CURSED Programming Language Compiler
License: MIT
Group: Development/Languages
URL: https://cursed.dev
Source0: %{name}-%{version}.tar.gz
BuildArch: x86_64

Requires: glibc >= 2.17
Requires: llvm >= 16.0

%description
A modern, safe, and performant programming language compiler
featuring advanced type system, memory safety, and concurrency.

%prep
%setup -q

%build
# Pre-built binaries

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/cursed
cp -r BUILD/usr/* %{buildroot}/usr/

%files
%defattr(-,root,root,-)
/usr/bin/cursed-zig
/usr/bin/cursed-stable
/usr/bin/cursed-lsp
/usr/share/cursed/

%post
echo "CURSED compiler installed successfully!"
echo "Run 'cursed --version' to verify installation."

%changelog
* $(date +'%a %b %d %Y') CURSED Team <dev@cursed.dev> - $VERSION-1
- Initial release
EOF
    
    # Build RPM
    rpmbuild --define "_topdir $rpm_dir" --define "_rpmdir $DIST_DIR" -bb "$spec_file"
    
    log "RPM package created"
}

create_msi_package() {
    log "Creating Windows MSI package..."
    
    # This requires WiX Toolset on Windows or wine
    local msi_dir="$BUILD_DIR/msi"
    local wxs_file="$msi_dir/cursed.wxs"
    
    mkdir -p "$msi_dir"
    
    # Create WiX source file
    cat > "$wxs_file" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Product Id="*" Name="CURSED Compiler" Language="1033" Version="$VERSION" 
           Manufacturer="CURSED Development Team" UpgradeCode="12345678-1234-1234-1234-123456789012">
    
    <Package InstallerVersion="200" Compressed="yes" InstallScope="perMachine" />
    
    <MajorUpgrade DowngradeErrorMessage="A newer version of CURSED is already installed." />
    
    <MediaTemplate EmbedCab="yes" />
    
    <Feature Id="ProductFeature" Title="CURSED Compiler" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
    </Feature>
  </Product>
  
  <Fragment>
    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="ProgramFilesFolder">
        <Directory Id="INSTALLFOLDER" Name="CURSED" />
      </Directory>
    </Directory>
  </Fragment>
  
  <Fragment>
    <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
      <Component Id="CursedExecutable" Guid="*">
        <File Id="cursed.exe" Source="$BUILD_DIR/x86_64-pc-windows-gnu/bin/cursed-zig.exe" />
        <Environment Id="PATH" Name="PATH" Value="[INSTALLFOLDER]" Permanent="yes" Part="last" Action="set" System="yes" />
      </Component>
    </ComponentGroup>
  </Fragment>
</Wix>
EOF
    
    log "MSI package configuration created (requires WiX Toolset to build)"
}

create_macos_pkg() {
    log "Creating macOS package..."
    
    local pkg_dir="$BUILD_DIR/macos-pkg"
    local payload_dir="$pkg_dir/payload"
    local scripts_dir="$pkg_dir/scripts"
    
    mkdir -p "$payload_dir/usr/local"/{bin,share/cursed} "$scripts_dir"
    
    # Copy binaries (prefer ARM64 for modern Macs)
    if [[ -d "$BUILD_DIR/aarch64-apple-darwin" ]]; then
        cp "$BUILD_DIR/aarch64-apple-darwin/bin"/* "$payload_dir/usr/local/bin/"
    else
        cp "$BUILD_DIR/x86_64-apple-darwin/bin"/* "$payload_dir/usr/local/bin/"
    fi
    
    # Copy standard library and documentation
    cp -r "$PROJECT_ROOT/stdlib" "$payload_dir/usr/local/share/cursed/"
    cp -r "$PROJECT_ROOT/docs" "$payload_dir/usr/local/share/cursed/" 2>/dev/null || true
    
    # Create postinstall script
    cat > "$scripts_dir/postinstall" << 'EOF'
#!/bin/bash
set -e

# Add to PATH
echo 'export PATH="/usr/local/bin:$PATH"' >> /etc/paths.d/cursed

echo "CURSED compiler installed successfully!"
echo "Run 'cursed --version' to verify installation."
exit 0
EOF
    
    chmod 755 "$scripts_dir/postinstall"
    
    # Build package
    pkgbuild --root "$payload_dir" \
             --scripts "$scripts_dir" \
             --identifier "dev.cursed.compiler" \
             --version "$VERSION" \
             "$DIST_DIR/cursed-${VERSION}.pkg"
    
    log "macOS package created: cursed-${VERSION}.pkg"
}

# ============================================================================
# CONTAINER PACKAGING
# ============================================================================

create_docker_images() {
    log "Creating Docker images..."
    
    # Base Alpine image
    cat > "$BUILD_DIR/Dockerfile.alpine" << EOF
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \\
    libc6-compat \\
    libgcc \\
    libstdc++ \\
    llvm18-libs

# Create cursed user
RUN adduser -D -s /bin/sh cursed

# Copy binaries and standard library
COPY --from=builder /dist/x86_64-unknown-linux-gnu/bin/* /usr/local/bin/
COPY --from=builder /dist/x86_64-unknown-linux-gnu/stdlib /usr/local/share/cursed/stdlib

# Set permissions
RUN chmod +x /usr/local/bin/cursed-*

# Create cache directory
RUN mkdir -p /var/cache/cursed && chown cursed:cursed /var/cache/cursed

USER cursed
WORKDIR /home/cursed

ENV PATH="/usr/local/bin:\$PATH"
ENV CURSED_CACHE_DIR="/var/cache/cursed"

ENTRYPOINT ["cursed-zig"]
EOF

    # Ubuntu LTS image for compatibility
    cat > "$BUILD_DIR/Dockerfile.ubuntu" << EOF
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \\
    libc6 \\
    libgcc-s1 \\
    libstdc++6 \\
    llvm-18 \\
    && rm -rf /var/lib/apt/lists/*

# Create cursed user
RUN useradd -m -s /bin/bash cursed

# Copy binaries and standard library
COPY --from=builder /dist/x86_64-unknown-linux-gnu/bin/* /usr/local/bin/
COPY --from=builder /dist/x86_64-unknown-linux-gnu/stdlib /usr/local/share/cursed/stdlib

# Set permissions
RUN chmod +x /usr/local/bin/cursed-*

# Create cache directory
RUN mkdir -p /var/cache/cursed && chown cursed:cursed /var/cache/cursed

USER cursed
WORKDIR /home/cursed

ENV PATH="/usr/local/bin:\$PATH"
ENV CURSED_CACHE_DIR="/var/cache/cursed"

ENTRYPOINT ["cursed-zig"]
EOF

    # Multi-stage build Dockerfile
    cat > "$BUILD_DIR/Dockerfile" << EOF
# Multi-stage build for CURSED compiler
FROM alpine:3.19 AS builder

# Install build dependencies
RUN apk add --no-cache \\
    build-base \\
    cmake \\
    llvm18-dev \\
    llvm18-static \\
    zig

# Copy source
COPY . /src
WORKDIR /src

# Build for Linux x86_64
RUN zig build -Dtarget=x86_64-unknown-linux-gnu -Doptimize=ReleaseFast --prefix /dist/x86_64-unknown-linux-gnu

# Production image
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \\
    libc6-compat \\
    libgcc \\
    libstdc++ \\
    llvm18-libs

# Create cursed user
RUN adduser -D -s /bin/sh cursed

# Copy from builder
COPY --from=builder /dist/x86_64-unknown-linux-gnu/bin/* /usr/local/bin/
COPY --from=builder /src/stdlib /usr/local/share/cursed/stdlib

# Set permissions
RUN chmod +x /usr/local/bin/cursed-*

# Create cache directory
RUN mkdir -p /var/cache/cursed && chown cursed:cursed /var/cache/cursed

USER cursed
WORKDIR /home/cursed

ENV PATH="/usr/local/bin:\$PATH"
ENV CURSED_CACHE_DIR="/var/cache/cursed"

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \\
    CMD cursed-zig --version || exit 1

ENTRYPOINT ["cursed-zig"]
CMD ["--help"]
EOF

    log "Docker configurations created"
}

# ============================================================================
# PACKAGE MANAGER INTEGRATIONS
# ============================================================================

create_homebrew_formula() {
    log "Creating Homebrew formula..."
    
    local formula_dir="$BUILD_DIR/homebrew"
    mkdir -p "$formula_dir"
    
    # Calculate SHA256 for macOS packages
    local sha256_arm64=""
    local sha256_x64=""
    
    if [[ -f "$DIST_DIR/cursed-${FULL_VERSION}-aarch64-apple-darwin.tar.gz" ]]; then
        sha256_arm64=$(sha256sum "$DIST_DIR/cursed-${FULL_VERSION}-aarch64-apple-darwin.tar.gz" | cut -d' ' -f1)
    fi
    
    if [[ -f "$DIST_DIR/cursed-${FULL_VERSION}-x86_64-apple-darwin.tar.gz" ]]; then
        sha256_x64=$(sha256sum "$DIST_DIR/cursed-${FULL_VERSION}-x86_64-apple-darwin.tar.gz" | cut -d' ' -f1)
    fi
    
    cat > "$formula_dir/cursed.rb" << EOF
class Cursed < Formula
  desc "CURSED Programming Language Compiler"
  homepage "https://cursed.dev"
  version "$VERSION"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/ghuntley/cursed/releases/download/v#{version}/cursed-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "$sha256_arm64"
    else
      url "https://github.com/ghuntley/cursed/releases/download/v#{version}/cursed-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "$sha256_x64"
    end
  end

  depends_on "llvm@18"

  def install
    bin.install Dir["bin/*"]
    share.install "stdlib" => "cursed/stdlib"
    (share/"cursed/docs").install Dir["docs/*"] if Dir.exist?("docs")
  end

  test do
    system "#{bin}/cursed-zig", "--version"
    system "#{bin}/cursed-zig", "--help"
  end
end
EOF
    
    log "Homebrew formula created: homebrew/cursed.rb"
}

create_apt_repository() {
    log "Creating APT repository structure..."
    
    local apt_dir="$BUILD_DIR/apt-repo"
    local pool_dir="$apt_dir/pool/main/c/cursed"
    local dists_dir="$apt_dir/dists/stable/main/binary-amd64"
    
    mkdir -p "$pool_dir" "$dists_dir"
    
    # Copy DEB package
    cp "$DIST_DIR"/*.deb "$pool_dir/"
    
    # Create Packages file
    cd "$apt_dir"
    dpkg-scanpackages pool/ /dev/null > "$dists_dir/Packages"
    gzip -c "$dists_dir/Packages" > "$dists_dir/Packages.gz"
    
    # Create Release file
    cat > "$apt_dir/dists/stable/Release" << EOF
Origin: CURSED
Label: CURSED
Suite: stable
Codename: stable
Version: $VERSION
Architectures: amd64
Components: main
Description: CURSED Compiler APT Repository
Date: $(date -Ru)
EOF
    
    log "APT repository structure created"
}

create_yum_repository() {
    log "Creating YUM repository structure..."
    
    local yum_dir="$BUILD_DIR/yum-repo"
    mkdir -p "$yum_dir"
    
    # Copy RPM packages
    cp "$DIST_DIR"/*.rpm "$yum_dir/"
    
    # Create repository metadata
    createrepo "$yum_dir"
    
    # Create repository configuration
    cat > "$yum_dir/cursed.repo" << EOF
[cursed]
name=CURSED Compiler Repository
baseurl=https://packages.cursed.dev/yum/
enabled=1
gpgcheck=0
EOF
    
    log "YUM repository structure created"
}

# ============================================================================
# RELEASE AUTOMATION
# ============================================================================

create_github_release() {
    log "Creating GitHub release automation..."
    
    local gh_dir="$BUILD_DIR/github"
    mkdir -p "$gh_dir"
    
    # GitHub Actions workflow
    cat > "$gh_dir/release.yml" << 'EOF'
name: Release

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build and Release
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Zig
      uses: goto-bus-stop/setup-zig@v2
      with:
        version: 0.13.0
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y llvm-18-dev libllvm18
    
    - name: Build release packages
      run: |
        chmod +x packaging/build-release.sh
        ./packaging/build-release.sh
    
    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: |
          dist/*.tar.gz
          dist/*.zip
          dist/*.deb
          dist/*.rpm
          dist/*.pkg
        generate_release_notes: true
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
EOF
    
    # Release script
    cat > "$gh_dir/create-release.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

VERSION=$(cat packaging/VERSION)
TAG="v$VERSION"

echo "Creating release for version $VERSION..."

# Create and push tag
git tag -a "$TAG" -m "Release $VERSION"
git push origin "$TAG"

echo "Release tag $TAG created and pushed"
echo "GitHub Actions will automatically build and create the release"
EOF
    
    chmod +x "$gh_dir/create-release.sh"
    
    log "GitHub release automation created"
}

create_release_notes() {
    log "Creating release notes template..."
    
    cat > "$BUILD_DIR/RELEASE_NOTES.md" << EOF
# CURSED Compiler v$VERSION

## What's New

### Features
- [x] Feature 1: Description of new feature
- [x] Feature 2: Another feature description

### Improvements
- [x] Performance optimization for compilation speed
- [x] Enhanced error messages and diagnostics
- [x] Better cross-platform compatibility

### Bug Fixes
- [x] Fixed issue #123: Description of bug fix
- [x] Resolved memory leak in parser
- [x] Corrected type inference edge cases

## Breaking Changes

> ⚠️ **Important**: This release contains breaking changes

- Change 1: Description and migration path
- Change 2: Another breaking change

## Installation

### Package Managers

**Homebrew (macOS)**:
\`\`\`bash
brew install cursed
\`\`\`

**APT (Ubuntu/Debian)**:
\`\`\`bash
wget -qO- https://packages.cursed.dev/apt/key.gpg | sudo apt-key add -
echo "deb https://packages.cursed.dev/apt stable main" | sudo tee /etc/apt/sources.list.d/cursed.list
sudo apt update && sudo apt install cursed
\`\`\`

**YUM (RHEL/CentOS/Fedora)**:
\`\`\`bash
sudo yum-config-manager --add-repo https://packages.cursed.dev/yum/cursed.repo
sudo yum install cursed
\`\`\`

### Binary Downloads

| Platform | Architecture | Download |
|----------|--------------|----------|
| Linux    | x86_64       | [cursed-$VERSION-x86_64-unknown-linux-gnu.tar.gz](https://github.com/ghuntley/cursed/releases/download/v$VERSION/cursed-$VERSION-x86_64-unknown-linux-gnu.tar.gz) |
| Linux    | ARM64        | [cursed-$VERSION-aarch64-unknown-linux-gnu.tar.gz](https://github.com/ghuntley/cursed/releases/download/v$VERSION/cursed-$VERSION-aarch64-unknown-linux-gnu.tar.gz) |
| macOS    | Intel        | [cursed-$VERSION-x86_64-apple-darwin.tar.gz](https://github.com/ghuntley/cursed/releases/download/v$VERSION/cursed-$VERSION-x86_64-apple-darwin.tar.gz) |
| macOS    | Apple Silicon | [cursed-$VERSION-aarch64-apple-darwin.tar.gz](https://github.com/ghuntley/cursed/releases/download/v$VERSION/cursed-$VERSION-aarch64-apple-darwin.tar.gz) |
| Windows  | x86_64       | [cursed-$VERSION-x86_64-pc-windows-gnu.zip](https://github.com/ghuntley/cursed/releases/download/v$VERSION/cursed-$VERSION-x86_64-pc-windows-gnu.zip) |

### Docker

\`\`\`bash
docker pull cursed/compiler:$VERSION
docker run --rm cursed/compiler:$VERSION --version
\`\`\`

## Verification

All release artifacts are signed and can be verified:

\`\`\`bash
# Download checksums
wget https://github.com/ghuntley/cursed/releases/download/v$VERSION/SHA256SUMS
wget https://github.com/ghuntley/cursed/releases/download/v$VERSION/SHA256SUMS.sig

# Verify signature (requires GPG key)
gpg --verify SHA256SUMS.sig SHA256SUMS

# Verify download
sha256sum -c SHA256SUMS
\`\`\`

## Documentation

- [Language Guide](https://docs.cursed.dev/guide/)
- [API Reference](https://docs.cursed.dev/api/)
- [Standard Library](https://docs.cursed.dev/stdlib/)
- [Examples](https://github.com/ghuntley/cursed/tree/main/examples)

## Support

- [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- [Discussions](https://github.com/ghuntley/cursed/discussions)
- [Discord](https://discord.gg/cursed)

---

**Full Changelog**: https://github.com/ghuntley/cursed/compare/v$(cat packaging/VERSION | cut -d. -f1-2).0...v$VERSION
EOF

    log "Release notes template created"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

main() {
    log "Starting CURSED Compiler release build process..."
    
    # Setup
    setup_build_env
    
    # Build all targets
    for target in "${TARGETS[@]}"; do
        if build_target "$target"; then
            package_target "$target"
        else
            log "Warning: Build failed for target $target, skipping..."
        fi
    done
    
    # Create native packages
    if command -v dpkg-deb >/dev/null 2>&1; then
        create_deb_package
    fi
    
    if command -v rpmbuild >/dev/null 2>&1; then
        create_rpm_package
    fi
    
    if [[ "$OSTYPE" == "darwin"* ]] && command -v pkgbuild >/dev/null 2>&1; then
        create_macos_pkg
    fi
    
    # Create package manager integrations
    create_homebrew_formula
    create_apt_repository
    create_yum_repository
    
    # Create container configurations
    create_docker_images
    
    # Create release automation
    create_github_release
    create_release_notes
    
    # Create checksums
    log "Creating checksums..."
    cd "$DIST_DIR"
    sha256sum *.tar.gz *.zip *.deb *.rpm *.pkg 2>/dev/null > SHA256SUMS || true
    
    # Summary
    log "Release build completed successfully!"
    log "Version: $FULL_VERSION"
    log "Artifacts created in: $DIST_DIR"
    log ""
    log "Created packages:"
    ls -la "$DIST_DIR" | grep -v "^d" | awk '{print "  " $9}' || true
    
    echo ""
    echo "🎉 CURSED Compiler v$FULL_VERSION is ready for distribution!"
    echo ""
    echo "Next steps:"
    echo "1. Test packages on target platforms"
    echo "2. Sign artifacts with GPG"
    echo "3. Upload to package repositories"
    echo "4. Create GitHub release: ./packaging/github/create-release.sh"
    echo "5. Update documentation with new version"
}

# Run main function
main "$@"
