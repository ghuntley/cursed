#!/bin/bash
# CURSED Programming Language - Automated Packaging Script
# Oracle Week 3 cross-platform packaging preparation for v1.0 launch
set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
PACKAGING_DIR="${PROJECT_ROOT}/packaging"
DIST_DIR="${PROJECT_ROOT}/dist"
BUILD_DIR="${PROJECT_ROOT}/build"
VERSION="${VERSION:-$(cat ${PROJECT_ROOT}/VERSION 2>/dev/null || echo "1.0.0")}"

# Build configuration
BUILD_MODE="${BUILD_MODE:-release}"
ENABLE_LTO="${ENABLE_LTO:-true}"
ENABLE_STRIP="${ENABLE_STRIP:-true}"
PARALLEL_JOBS="${PARALLEL_JOBS:-$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)}"
CROSS_COMPILE="${CROSS_COMPILE:-true}"
SIGN_ARTIFACTS="${SIGN_ARTIFACTS:-true}"

# Platform targets
declare -a TARGETS=(
    "x86_64-linux-gnu"
    "aarch64-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
    "wasm32-wasi"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_build() {
    echo -e "${PURPLE}[BUILD]${NC} $1"
}

log_package() {
    echo -e "${CYAN}[PACKAGE]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    local missing_tools=()
    
    # Essential tools
    if ! command -v zig >/dev/null 2>&1; then
        missing_tools+=("zig")
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        missing_tools+=("tar")
    fi
    
    if ! command -v zip >/dev/null 2>&1; then
        missing_tools+=("zip")
    fi
    
    # Package creation tools (optional)
    if ! command -v fpm >/dev/null 2>&1; then
        log_warning "fpm not found - native package creation will be skipped"
    fi
    
    if ! command -v docker >/dev/null 2>&1; then
        log_warning "docker not found - container builds will be skipped"
    fi
    
    if ! command -v cosign >/dev/null 2>&1; then
        log_warning "cosign not found - artifact signing will be skipped"
        SIGN_ARTIFACTS=false
    fi
    
    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        log_error "Please install them and try again"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Clean previous build artifacts
clean_build() {
    log_info "Cleaning previous build artifacts..."
    
    # Remove build and dist directories
    rm -rf "${DIST_DIR}" "${BUILD_DIR}"
    
    # Clean zig cache
    if [[ -d "${PROJECT_ROOT}/zig-cache" ]]; then
        rm -rf "${PROJECT_ROOT}/zig-cache"
    fi
    
    if [[ -d "${PROJECT_ROOT}/zig-out" ]]; then
        rm -rf "${PROJECT_ROOT}/zig-out"
    fi
    
    # Create fresh directories
    mkdir -p "${DIST_DIR}" "${BUILD_DIR}"
    
    log_success "Build environment cleaned"
}

# Build for a specific target
build_target() {
    local target="$1"
    local target_dir="${BUILD_DIR}/${target}"
    
    log_build "Building for target: ${target}"
    
    # Create target directory
    mkdir -p "${target_dir}"
    
    # Set optimization level
    local optimize_flag=""
    case "${BUILD_MODE}" in
        "debug")
            optimize_flag="-Doptimize=Debug"
            ;;
        "release")
            optimize_flag="-Doptimize=ReleaseFast"
            ;;
        "safe")
            optimize_flag="-Doptimize=ReleaseSafe"
            ;;
        "small")
            optimize_flag="-Doptimize=ReleaseSmall"
            ;;
        *)
            optimize_flag="-Doptimize=ReleaseFast"
            ;;
    esac
    
    # Additional build flags
    local build_flags=()
    build_flags+=("${optimize_flag}")
    build_flags+=("-Dtarget=${target}")
    
    if [[ "${ENABLE_STRIP}" == "true" ]]; then
        build_flags+=("-Dstrip=true")
    fi
    
    # Determine executable suffix
    local exe_suffix=""
    if [[ "${target}" == *"windows"* ]]; then
        exe_suffix=".exe"
    elif [[ "${target}" == *"wasm"* ]]; then
        exe_suffix=".wasm"
    fi
    
    # Build the project
    cd "${PROJECT_ROOT}"
    
    if ! zig build "${build_flags[@]}" --verbose; then
        log_error "Build failed for target: ${target}"
        return 1
    fi
    
    # Copy artifacts to target directory
    if [[ -d "zig-out/bin" ]]; then
        cp -r zig-out/bin/* "${target_dir}/"
        
        # Rename main binary with suffix if needed
        if [[ -f "${target_dir}/cursed" && -n "${exe_suffix}" ]]; then
            mv "${target_dir}/cursed" "${target_dir}/cursed${exe_suffix}"
        fi
        
        log_success "Build completed for target: ${target}"
        
        # List built artifacts
        log_info "Built artifacts for ${target}:"
        ls -la "${target_dir}/"
        
        return 0
    else
        log_error "No build artifacts found for target: ${target}"
        return 1
    fi
}

# Build for all targets
build_all_targets() {
    log_info "Building for all targets..."
    
    local success_count=0
    local total_count=${#TARGETS[@]}
    
    for target in "${TARGETS[@]}"; do
        if build_target "${target}"; then
            ((success_count++))
        else
            log_error "Failed to build target: ${target}"
        fi
    done
    
    log_info "Build summary: ${success_count}/${total_count} targets built successfully"
    
    if [[ ${success_count} -eq 0 ]]; then
        log_error "No targets built successfully"
        return 1
    fi
    
    return 0
}

# Create archive for a target
create_archive() {
    local target="$1"
    local target_dir="${BUILD_DIR}/${target}"
    
    if [[ ! -d "${target_dir}" ]]; then
        log_warning "Skipping archive for missing target: ${target}"
        return 1
    fi
    
    log_package "Creating archive for target: ${target}"
    
    # Determine archive format
    local archive_name="cursed-${VERSION}-${target}"
    local archive_path=""
    
    if [[ "${target}" == *"windows"* ]]; then
        # Create ZIP for Windows
        archive_path="${DIST_DIR}/${archive_name}.zip"
        cd "${BUILD_DIR}"
        zip -r "${archive_path}" "${target}/"
    else
        # Create tar.gz for Unix-like systems
        archive_path="${DIST_DIR}/${archive_name}.tar.gz"
        cd "${BUILD_DIR}"
        tar -czf "${archive_path}" "${target}/"
    fi
    
    if [[ -f "${archive_path}" ]]; then
        local size=$(du -h "${archive_path}" | cut -f1)
        log_success "Created archive: $(basename "${archive_path}") (${size})"
        return 0
    else
        log_error "Failed to create archive for target: ${target}"
        return 1
    fi
}

# Create native packages (DEB, RPM, etc.)
create_native_packages() {
    log_package "Creating native packages..."
    
    if ! command -v fpm >/dev/null 2>&1; then
        log_warning "fpm not available - skipping native package creation"
        return 0
    fi
    
    # Find Linux x64 build
    local linux_x64_dir="${BUILD_DIR}/x86_64-linux-gnu"
    if [[ ! -d "${linux_x64_dir}" ]]; then
        log_warning "Linux x64 build not found - skipping native packages"
        return 0
    fi
    
    local package_version="${VERSION}"
    local package_iteration="1"
    local package_description="CURSED Programming Language: A production-ready systems programming language with ergonomic syntax"
    local package_url="https://cursedlang.org"
    local package_license="MIT"
    local package_maintainer="CURSED Development Team <dev@cursedlang.org>"
    
    # Create DEB package
    log_package "Creating DEB package..."
    fpm -s dir -t deb \
        --name cursed \
        --version "${package_version}" \
        --iteration "${package_iteration}" \
        --description "${package_description}" \
        --url "${package_url}" \
        --license "${package_license}" \
        --maintainer "${package_maintainer}" \
        --architecture x86_64 \
        --depends "libc6" \
        --depends "libgcc-s1" \
        --chdir "${linux_x64_dir}" \
        --package "${DIST_DIR}/cursed_${package_version}-${package_iteration}_amd64.deb" \
        . || log_warning "DEB package creation failed"
    
    # Create RPM package
    log_package "Creating RPM package..."
    fpm -s dir -t rpm \
        --name cursed \
        --version "${package_version}" \
        --iteration "${package_iteration}" \
        --description "${package_description}" \
        --url "${package_url}" \
        --license "${package_license}" \
        --maintainer "${package_maintainer}" \
        --architecture x86_64 \
        --depends "glibc" \
        --depends "libgcc" \
        --chdir "${linux_x64_dir}" \
        --package "${DIST_DIR}/cursed-${package_version}-${package_iteration}.x86_64.rpm" \
        . || log_warning "RPM package creation failed"
    
    log_success "Native package creation completed"
}

# Create checksums file
create_checksums() {
    log_package "Creating checksums file..."
    
    cd "${DIST_DIR}"
    
    # Create SHA256SUMS file
    sha256sum *.tar.gz *.zip *.deb *.rpm 2>/dev/null > SHA256SUMS || true
    
    if [[ -f "SHA256SUMS" && -s "SHA256SUMS" ]]; then
        log_success "Created checksums file: SHA256SUMS"
        
        # Display checksums
        log_info "Checksums:"
        cat SHA256SUMS
    else
        log_warning "No files found for checksum generation"
    fi
}

# Sign all artifacts
sign_all_artifacts() {
    if [[ "${SIGN_ARTIFACTS}" != "true" ]]; then
        log_info "Artifact signing disabled"
        return 0
    fi
    
    log_package "Signing all artifacts..."
    
    local signing_script="${PACKAGING_DIR}/scripts/cosign-signing.sh"
    
    if [[ -f "${signing_script}" ]]; then
        if bash "${signing_script}" sign; then
            log_success "All artifacts signed successfully"
        else
            log_warning "Artifact signing failed - continuing without signatures"
        fi
    else
        log_warning "Signing script not found - skipping artifact signing"
    fi
}

# Update package manager configurations
update_package_configs() {
    log_package "Updating package manager configurations..."
    
    # Update Homebrew formula
    local homebrew_formula="${PACKAGING_DIR}/package-managers/homebrew/cursed.rb"
    if [[ -f "${homebrew_formula}" ]]; then
        # Update version in formula
        sed -i.bak "s/version \".*\"/version \"${VERSION}\"/g" "${homebrew_formula}"
        
        # Calculate SHA256 for source tarball if available
        local source_tarball="${DIST_DIR}/cursed-${VERSION}-source.tar.gz"
        if [[ -f "${source_tarball}" ]]; then
            local sha256=$(sha256sum "${source_tarball}" | cut -d' ' -f1)
            sed -i.bak "s/sha256 \".*\"/sha256 \"${sha256}\"/g" "${homebrew_formula}"
        fi
        
        log_success "Updated Homebrew formula"
    fi
    
    # Update Scoop manifest
    local scoop_manifest="${PACKAGING_DIR}/package-managers/scoop/cursed.json"
    if [[ -f "${scoop_manifest}" ]]; then
        # Update version in manifest
        sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/g" "${scoop_manifest}"
        
        # Update Windows package hash
        local windows_zip="${DIST_DIR}/cursed-${VERSION}-x86_64-pc-windows-gnu.zip"
        if [[ -f "${windows_zip}" ]]; then
            local sha256=$(sha256sum "${windows_zip}" | cut -d' ' -f1)
            sed -i.bak "s/\"hash\": \".*\"/\"hash\": \"${sha256}\"/g" "${scoop_manifest}"
        fi
        
        log_success "Updated Scoop manifest"
    fi
    
    # Update AUR PKGBUILD
    local aur_pkgbuild="${PACKAGING_DIR}/package-managers/aur/PKGBUILD"
    if [[ -f "${aur_pkgbuild}" ]]; then
        sed -i.bak "s/pkgver=.*/pkgver=${VERSION}/g" "${aur_pkgbuild}"
        log_success "Updated AUR PKGBUILD"
    fi
    
    # Update Chocolatey nuspec
    local choco_nuspec="${PACKAGING_DIR}/package-managers/chocolatey/cursed.nuspec"
    if [[ -f "${choco_nuspec}" ]]; then
        sed -i.bak "s/<version>.*<\/version>/<version>${VERSION}<\/version>/g" "${choco_nuspec}"
        log_success "Updated Chocolatey nuspec"
    fi
}

# Create source distribution
create_source_distribution() {
    log_package "Creating source distribution..."
    
    local source_archive="${DIST_DIR}/cursed-${VERSION}-source.tar.gz"
    
    cd "${PROJECT_ROOT}"
    
    # Create source tarball excluding build artifacts and caches
    tar -czf "${source_archive}" \
        --exclude=".git" \
        --exclude="zig-cache" \
        --exclude="zig-out" \
        --exclude="build" \
        --exclude="dist" \
        --exclude="target" \
        --exclude="node_modules" \
        --exclude=".DS_Store" \
        --exclude="*.tmp" \
        --exclude="*.log" \
        .
    
    if [[ -f "${source_archive}" ]]; then
        local size=$(du -h "${source_archive}" | cut -f1)
        log_success "Created source distribution: $(basename "${source_archive}") (${size})"
    else
        log_error "Failed to create source distribution"
        return 1
    fi
}

# Generate release summary
generate_release_summary() {
    log_info "Generating release summary..."
    
    local summary_file="${DIST_DIR}/RELEASE_SUMMARY.md"
    
    cat > "${summary_file}" << EOF
# CURSED v${VERSION} - Release Summary

**Generated:** $(date -u +'%Y-%m-%d %H:%M:%S UTC')
**Build Mode:** ${BUILD_MODE}
**Cross-compilation:** ${CROSS_COMPILE}
**Signed:** ${SIGN_ARTIFACTS}

## Distribution Artifacts

EOF
    
    # List all distribution files
    cd "${DIST_DIR}"
    for file in *; do
        if [[ -f "${file}" && "${file}" != "RELEASE_SUMMARY.md" ]]; then
            local size=$(du -h "${file}" | cut -f1)
            echo "- \`${file}\` (${size})" >> "${summary_file}"
        fi
    done
    
    cat >> "${summary_file}" << EOF

## Supported Platforms

- **Linux**: x86_64, ARM64 (tar.gz, DEB, RPM)
- **macOS**: Intel, Apple Silicon (tar.gz)
- **Windows**: x86_64 (ZIP)
- **WebAssembly**: WASI (WASM)

## Package Managers

- **Homebrew** (macOS): \`brew install cursed\`
- **Scoop** (Windows): \`scoop install cursed\`
- **AUR** (Arch Linux): \`yay -S cursed\`
- **Chocolatey** (Windows): \`choco install cursed\`

## Installation Instructions

### Linux (tar.gz)
\`\`\`bash
wget https://github.com/ghuntley/cursed/releases/download/v${VERSION}/cursed-${VERSION}-x86_64-linux-gnu.tar.gz
tar -xzf cursed-${VERSION}-x86_64-linux-gnu.tar.gz
sudo cp cursed-${VERSION}-x86_64-linux-gnu/cursed /usr/local/bin/
\`\`\`

### macOS (Homebrew)
\`\`\`bash
brew tap cursed/tap
brew install cursed
\`\`\`

### Windows (Scoop)
\`\`\`powershell
scoop bucket add cursed https://github.com/cursed/scoop-cursed.git
scoop install cursed
\`\`\`

## Verification

EOF
    
    if [[ "${SIGN_ARTIFACTS}" == "true" ]]; then
        cat >> "${summary_file}" << EOF
All artifacts are signed with cosign. Verify signatures:

\`\`\`bash
# Download public key
curl -O https://github.com/ghuntley/cursed/releases/download/v${VERSION}/cursed-${VERSION}-cosign-public-keys.pem

# Verify signature (example for Linux package)
cosign verify-blob --key cursed-${VERSION}-cosign-public-keys.pem \\
  --signature cursed-${VERSION}-x86_64-linux-gnu.tar.gz.sig \\
  cursed-${VERSION}-x86_64-linux-gnu.tar.gz
\`\`\`

EOF
    fi
    
    cat >> "${summary_file}" << EOF
Verify checksums:
\`\`\`bash
# Download and verify checksums
curl -O https://github.com/ghuntley/cursed/releases/download/v${VERSION}/SHA256SUMS
sha256sum -c SHA256SUMS
\`\`\`

## Quick Start

\`\`\`bash
# Create hello world program
echo 'yeet "vibez"; vibez.spill("Hello, World!");' > hello.💀

# Run it
cursed hello.💀
\`\`\`

## Resources

- **Website**: https://cursedlang.org
- **Documentation**: https://docs.cursedlang.org
- **Community**: https://discord.gg/cursedlang
- **GitHub**: https://github.com/ghuntley/cursed

---

*This release was automatically generated by the CURSED packaging system.*
EOF
    
    log_success "Generated release summary: RELEASE_SUMMARY.md"
}

# Show usage information
show_usage() {
    echo "CURSED Automated Packaging Script"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  clean              Clean build environment"
    echo "  build              Build for all targets"
    echo "  package            Create distribution packages"
    echo "  sign               Sign all artifacts"
    echo "  release            Complete release process"
    echo "  help               Show this help message"
    echo ""
    echo "Options:"
    echo "  --build-mode MODE  Set build mode (debug, release, safe, small)"
    echo "  --no-lto           Disable link-time optimization"
    echo "  --no-strip         Disable symbol stripping"
    echo "  --no-sign          Disable artifact signing"
    echo "  --jobs N           Set number of parallel jobs"
    echo ""
    echo "Environment Variables:"
    echo "  BUILD_MODE         Build mode (debug, release, safe, small)"
    echo "  ENABLE_LTO         Enable link-time optimization (true/false)"
    echo "  ENABLE_STRIP       Enable symbol stripping (true/false)"
    echo "  SIGN_ARTIFACTS     Enable artifact signing (true/false)"
    echo "  PARALLEL_JOBS      Number of parallel jobs"
    echo "  VERSION            Version string (auto-detected from VERSION file)"
    echo ""
    echo "Examples:"
    echo "  $0 clean                     # Clean build environment"
    echo "  $0 build                     # Build for all targets"
    echo "  $0 package                   # Create all packages"
    echo "  $0 release                   # Complete release process"
    echo "  $0 release --no-sign         # Release without signing"
    echo ""
}

# Parse command line arguments  
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --build-mode)
                BUILD_MODE="$2"
                shift 2
                ;;
            --no-lto)
                ENABLE_LTO=false
                shift
                ;;
            --no-strip)
                ENABLE_STRIP=false
                shift
                ;;
            --no-sign)
                SIGN_ARTIFACTS=false
                shift
                ;;
            --jobs)
                PARALLEL_JOBS="$2"
                shift 2
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            *)
                break
                ;;
        esac
    done
}

# Main execution
main() {
    parse_args "$@"
    
    local command="${1:-help}"
    
    echo "================================================================================================="
    echo "                           CURSED Programming Language - Automated Packaging"
    echo "================================================================================================="
    echo ""
    log_info "Version: ${VERSION}"
    log_info "Build Mode: ${BUILD_MODE}"
    log_info "LTO Enabled: ${ENABLE_LTO}"
    log_info "Strip Enabled: ${ENABLE_STRIP}"  
    log_info "Signing Enabled: ${SIGN_ARTIFACTS}"
    log_info "Parallel Jobs: ${PARALLEL_JOBS}"
    echo ""
    
    case "${command}" in
        "clean")
            check_prerequisites
            clean_build
            ;;
        "build")
            check_prerequisites
            clean_build
            build_all_targets
            ;;
        "package")
            check_prerequisites
            
            # Build if needed
            if [[ ! -d "${BUILD_DIR}" ]]; then
                clean_build
                build_all_targets
            fi
            
            # Create all packages
            for target in "${TARGETS[@]}"; do
                create_archive "${target}"
            done
            
            create_native_packages
            create_source_distribution
            create_checksums
            update_package_configs
            generate_release_summary
            ;;
        "sign")
            sign_all_artifacts
            ;;
        "release")
            check_prerequisites
            clean_build
            build_all_targets
            
            # Create all packages
            for target in "${TARGETS[@]}"; do
                create_archive "${target}"
            done
            
            create_native_packages
            create_source_distribution
            create_checksums
            sign_all_artifacts
            update_package_configs
            generate_release_summary
            
            log_success "🎉 Release packaging completed successfully!"
            log_info "Distribution artifacts are ready in: ${DIST_DIR}"
            ;;
        "help"|"--help"|"-h")
            show_usage
            ;;
        *)
            log_error "Unknown command: ${command}"
            show_usage
            exit 1
            ;;
    esac
}

# Execute main function if script is run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
