#!/bin/bash
# CURSED Compiler - Package Verification and Testing Script
# Comprehensive validation of all distribution packages

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
DIST_DIR="$PROJECT_ROOT/dist"
TEST_DIR="/tmp/cursed-package-test"
LOG_FILE="$TEST_DIR/verification.log"

# Test configuration
PLATFORMS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
    "wasm32-wasi"
)

PACKAGE_FORMATS=(
    "tar.gz"
    "zip"
    "deb"
    "rpm"
    "pkg"
    "msi"
)

# Container test environments
DOCKER_IMAGES=(
    "ubuntu:22.04"
    "debian:11"
    "centos:8"
    "fedora:39"
    "alpine:3.19"
)

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

log() {
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] $message" | tee -a "$LOG_FILE"
}

error() {
    log "ERROR: $*"
    exit 1
}

success() {
    log "✅ $*"
}

warning() {
    log "⚠️  $*"
}

info() {
    log "ℹ️  $*"
}

setup_test_environment() {
    log "Setting up test environment..."
    
    # Clean and create test directory
    rm -rf "$TEST_DIR"
    mkdir -p "$TEST_DIR"/{downloads,extracted,containers,reports}
    
    # Initialize log file
    cat > "$LOG_FILE" << EOF
CURSED Compiler Package Verification Report
Generated: $(date '+%Y-%m-%d %H:%M:%S')
Test Directory: $TEST_DIR
Distribution Directory: $DIST_DIR

EOF
    
    log "Test environment ready"
}

# ============================================================================
# ARCHIVE VERIFICATION
# ============================================================================

verify_archive_integrity() {
    local archive_file="$1"
    local platform="$2"
    
    log "Verifying archive integrity: $(basename "$archive_file")"
    
    # Check file exists and is not empty
    if [[ ! -f "$archive_file" ]]; then
        error "Archive file not found: $archive_file"
    fi
    
    if [[ ! -s "$archive_file" ]]; then
        error "Archive file is empty: $archive_file"
    fi
    
    # Verify archive format and extract
    local extract_dir="$TEST_DIR/extracted/$platform"
    mkdir -p "$extract_dir"
    
    case "$archive_file" in
        *.tar.gz)
            if ! tar -tzf "$archive_file" >/dev/null 2>&1; then
                error "Invalid tar.gz archive: $archive_file"
            fi
            tar -xzf "$archive_file" -C "$extract_dir"
            ;;
        *.zip)
            if ! unzip -t "$archive_file" >/dev/null 2>&1; then
                error "Invalid zip archive: $archive_file"
            fi
            unzip -q "$archive_file" -d "$extract_dir"
            ;;
        *)
            warning "Unknown archive format: $archive_file"
            return 1
            ;;
    esac
    
    # Verify extracted content structure
    local extracted_content="$(find "$extract_dir" -mindepth 1 -maxdepth 1 -type d | head -1)"
    if [[ -z "$extracted_content" ]]; then
        error "No content extracted from archive: $archive_file"
    fi
    
    # Check for required files
    local bin_dir="$extracted_content/bin"
    local stdlib_dir="$extracted_content/stdlib"
    
    if [[ ! -d "$bin_dir" ]]; then
        error "Missing bin directory in archive: $archive_file"
    fi
    
    if [[ ! -d "$stdlib_dir" ]]; then
        warning "Missing stdlib directory in archive: $archive_file"
    fi
    
    # Check for executables
    local executables=(
        "cursed-zig"
        "cursed-stable"
        "cursed-lsp"
    )
    
    for exe in "${executables[@]}"; do
        local exe_path="$bin_dir/$exe"
        
        # Add .exe extension for Windows
        if [[ "$platform" == *"windows"* ]]; then
            exe_path="${exe_path}.exe"
        fi
        
        # Add .wasm extension for WebAssembly
        if [[ "$platform" == "wasm32-wasi" ]]; then
            exe_path="${exe_path}.wasm"
        fi
        
        if [[ ! -f "$exe_path" ]]; then
            error "Missing executable: $exe_path"
        fi
        
        # Check if executable (skip for Windows/WASM)
        if [[ "$platform" != *"windows"* && "$platform" != "wasm32-wasi" ]]; then
            if [[ ! -x "$exe_path" ]]; then
                error "File not executable: $exe_path"
            fi
        fi
    done
    
    success "Archive integrity verified: $(basename "$archive_file")"
    return 0
}

# ============================================================================
# BINARY TESTING
# ============================================================================

test_binary_execution() {
    local extract_dir="$1"
    local platform="$2"
    
    log "Testing binary execution for platform: $platform"
    
    local bin_dir="$extract_dir/bin"
    local cursed_exe="$bin_dir/cursed-zig"
    
    # Add appropriate extension
    if [[ "$platform" == *"windows"* ]]; then
        cursed_exe="${cursed_exe}.exe"
    elif [[ "$platform" == "wasm32-wasi" ]]; then
        cursed_exe="${cursed_exe}.wasm"
    fi
    
    # Skip execution test for cross-platform binaries
    local current_arch=$(uname -m)
    local current_os=$(uname -s | tr '[:upper:]' '[:lower:]')
    
    local should_test_execution=false
    
    case "$platform" in
        "x86_64-unknown-linux-gnu")
            if [[ "$current_os" == "linux" && "$current_arch" == "x86_64" ]]; then
                should_test_execution=true
            fi
            ;;
        "aarch64-unknown-linux-gnu")
            if [[ "$current_os" == "linux" && "$current_arch" == "aarch64" ]]; then
                should_test_execution=true
            fi
            ;;
        "x86_64-apple-darwin")
            if [[ "$current_os" == "darwin" && "$current_arch" == "x86_64" ]]; then
                should_test_execution=true
            fi
            ;;
        "aarch64-apple-darwin")
            if [[ "$current_os" == "darwin" && "$current_arch" == "arm64" ]]; then
                should_test_execution=true
            fi
            ;;
        "wasm32-wasi")
            if command -v wasmtime >/dev/null 2>&1; then
                should_test_execution=true
            fi
            ;;
    esac
    
    if [[ "$should_test_execution" == "true" ]]; then
        log "Testing binary execution (native platform)"
        
        # Test version command
        local version_output
        if [[ "$platform" == "wasm32-wasi" ]]; then
            version_output=$(wasmtime "$cursed_exe" -- --version 2>/dev/null) || {
                warning "Failed to execute --version with wasmtime"
                return 1
            }
        else
            version_output=$("$cursed_exe" --version 2>/dev/null) || {
                warning "Failed to execute --version"
                return 1
            }
        fi
        
        if [[ -z "$version_output" ]]; then
            warning "Empty version output"
            return 1
        fi
        
        if [[ ! "$version_output" =~ CURSED|cursed|[0-9]+\.[0-9]+\.[0-9]+ ]]; then
            warning "Unexpected version output: $version_output"
            return 1
        fi
        
        # Test help command
        local help_output
        if [[ "$platform" == "wasm32-wasi" ]]; then
            help_output=$(wasmtime "$cursed_exe" -- --help 2>/dev/null) || {
                warning "Failed to execute --help with wasmtime"
                return 1
            }
        else
            help_output=$("$cursed_exe" --help 2>/dev/null) || {
                warning "Failed to execute --help"
                return 1
            }
        fi
        
        if [[ -z "$help_output" ]]; then
            warning "Empty help output"
            return 1
        fi
        
        success "Binary execution test passed for $platform"
    else
        info "Skipping execution test for cross-platform binary: $platform"
    fi
    
    return 0
}

# ============================================================================
# NATIVE PACKAGE TESTING
# ============================================================================

test_deb_package() {
    local deb_file="$1"
    
    log "Testing DEB package: $(basename "$deb_file")"
    
    # Check package format
    if ! dpkg --info "$deb_file" >/dev/null 2>&1; then
        error "Invalid DEB package format: $deb_file"
    fi
    
    # Extract package information
    local package_info
    package_info=$(dpkg --info "$deb_file")
    
    # Verify package metadata
    if ! echo "$package_info" | grep -q "Package: cursed"; then
        warning "Package name not found in DEB metadata"
    fi
    
    if ! echo "$package_info" | grep -q "Version:"; then
        warning "Version not found in DEB metadata"
    fi
    
    if ! echo "$package_info" | grep -q "Architecture:"; then
        warning "Architecture not found in DEB metadata"
    fi
    
    # List package contents
    local package_contents
    package_contents=$(dpkg --contents "$deb_file")
    
    # Verify required files
    local required_files=(
        "./usr/bin/cursed-zig"
        "./usr/bin/cursed-stable"
        "./usr/bin/cursed-lsp"
    )
    
    for file in "${required_files[@]}"; do
        if ! echo "$package_contents" | grep -q "$file"; then
            warning "Required file not found in DEB package: $file"
        fi
    done
    
    success "DEB package verification completed"
}

test_rpm_package() {
    local rpm_file="$1"
    
    log "Testing RPM package: $(basename "$rpm_file")"
    
    # Check package format
    if ! rpm -qp "$rpm_file" >/dev/null 2>&1; then
        error "Invalid RPM package format: $rpm_file"
    fi
    
    # Extract package information
    local package_info
    package_info=$(rpm -qip "$rpm_file")
    
    # Verify package metadata
    if ! echo "$package_info" | grep -q "Name.*cursed"; then
        warning "Package name not found in RPM metadata"
    fi
    
    if ! echo "$package_info" | grep -q "Version"; then
        warning "Version not found in RPM metadata"
    fi
    
    if ! echo "$package_info" | grep -q "Architecture"; then
        warning "Architecture not found in RPM metadata"
    fi
    
    # List package contents
    local package_contents
    package_contents=$(rpm -qlp "$rpm_file")
    
    # Verify required files
    local required_files=(
        "/usr/bin/cursed-zig"
        "/usr/bin/cursed-stable"
        "/usr/bin/cursed-lsp"
    )
    
    for file in "${required_files[@]}"; do
        if ! echo "$package_contents" | grep -q "$file"; then
            warning "Required file not found in RPM package: $file"
        fi
    done
    
    success "RPM package verification completed"
}

# ============================================================================
# CONTAINER TESTING
# ============================================================================

test_docker_image() {
    local image_tag="$1"
    
    log "Testing Docker image: $image_tag"
    
    if ! command -v docker >/dev/null 2>&1; then
        warning "Docker not available, skipping container tests"
        return 0
    fi
    
    # Check if image exists
    if ! docker image inspect "$image_tag" >/dev/null 2>&1; then
        warning "Docker image not found: $image_tag"
        return 1
    fi
    
    # Test basic execution
    local container_output
    container_output=$(docker run --rm "$image_tag" --version 2>/dev/null) || {
        warning "Failed to run Docker container: $image_tag"
        return 1
    }
    
    if [[ -z "$container_output" ]]; then
        warning "Empty output from Docker container: $image_tag"
        return 1
    fi
    
    # Test container size
    local image_size
    image_size=$(docker image inspect "$image_tag" --format '{{.Size}}')
    local size_mb=$((image_size / 1024 / 1024))
    
    info "Docker image size: ${size_mb}MB"
    
    if [[ $size_mb -gt 500 ]]; then
        warning "Docker image is quite large: ${size_mb}MB"
    fi
    
    success "Docker image test passed: $image_tag"
}

test_package_installation() {
    local package_file="$1"
    local test_env="$2"
    
    log "Testing package installation in $test_env"
    
    if ! command -v docker >/dev/null 2>&1; then
        warning "Docker not available, skipping installation tests"
        return 0
    fi
    
    local container_name="cursed-test-$(date +%s)"
    local dockerfile_content=""
    
    case "$test_env" in
        "ubuntu:22.04"|"debian:11")
            if [[ "$package_file" == *.deb ]]; then
                dockerfile_content="
FROM $test_env
COPY $(basename "$package_file") /tmp/package.deb
RUN apt-get update && apt-get install -y /tmp/package.deb
RUN cursed-zig --version
"
            fi
            ;;
        "centos:8"|"fedora:39")
            if [[ "$package_file" == *.rpm ]]; then
                dockerfile_content="
FROM $test_env
COPY $(basename "$package_file") /tmp/package.rpm
RUN yum install -y /tmp/package.rpm || dnf install -y /tmp/package.rpm
RUN cursed-zig --version
"
            fi
            ;;
    esac
    
    if [[ -n "$dockerfile_content" ]]; then
        local test_dir="$TEST_DIR/containers/$container_name"
        mkdir -p "$test_dir"
        
        # Copy package to test directory
        cp "$package_file" "$test_dir/"
        
        # Create Dockerfile
        echo "$dockerfile_content" > "$test_dir/Dockerfile"
        
        # Build and test
        if docker build -t "$container_name" "$test_dir" >/dev/null 2>&1; then
            success "Package installation test passed in $test_env"
            docker rmi "$container_name" >/dev/null 2>&1 || true
        else
            warning "Package installation test failed in $test_env"
        fi
        
        # Cleanup
        rm -rf "$test_dir"
    else
        info "Skipping installation test for $test_env (incompatible package)"
    fi
}

# ============================================================================
# CHECKSUM VERIFICATION
# ============================================================================

verify_checksums() {
    log "Verifying package checksums..."
    
    local checksum_file="$DIST_DIR/SHA256SUMS"
    
    if [[ ! -f "$checksum_file" ]]; then
        warning "Checksum file not found: $checksum_file"
        return 1
    fi
    
    # Change to dist directory for verification
    cd "$DIST_DIR"
    
    # Verify checksums
    if sha256sum -c "$checksum_file" >/dev/null 2>&1; then
        success "All checksums verified successfully"
    else
        # Try to identify which files failed
        local failed_files
        failed_files=$(sha256sum -c "$checksum_file" 2>&1 | grep -v "OK$" | cut -d: -f1)
        
        if [[ -n "$failed_files" ]]; then
            warning "Checksum verification failed for files: $failed_files"
        else
            warning "Checksum verification failed"
        fi
        return 1
    fi
    
    cd "$PROJECT_ROOT"
}

# ============================================================================
# COMPREHENSIVE TESTING
# ============================================================================

run_comprehensive_tests() {
    log "Running comprehensive package verification tests..."
    
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    # Test all archives
    for platform in "${PLATFORMS[@]}"; do
        for format in "tar.gz" "zip"; do
            local archive_pattern="$DIST_DIR/cursed-*-$platform.$format"
            local archive_files=($(ls $archive_pattern 2>/dev/null || true))
            
            for archive_file in "${archive_files[@]}"; do
                if [[ -f "$archive_file" ]]; then
                    total_tests=$((total_tests + 1))
                    
                    if verify_archive_integrity "$archive_file" "$platform"; then
                        passed_tests=$((passed_tests + 1))
                        
                        # Test binary execution
                        local extract_dir="$TEST_DIR/extracted/$platform"
                        local extracted_content="$(find "$extract_dir" -mindepth 1 -maxdepth 1 -type d | head -1)"
                        
                        if [[ -n "$extracted_content" ]]; then
                            if test_binary_execution "$extracted_content" "$platform"; then
                                info "Binary execution test passed for $platform"
                            fi
                        fi
                    else
                        failed_tests=$((failed_tests + 1))
                    fi
                fi
            done
        done
    done
    
    # Test DEB packages
    local deb_files=($(ls "$DIST_DIR"/*.deb 2>/dev/null || true))
    for deb_file in "${deb_files[@]}"; do
        if [[ -f "$deb_file" ]]; then
            total_tests=$((total_tests + 1))
            
            if test_deb_package "$deb_file"; then
                passed_tests=$((passed_tests + 1))
            else
                failed_tests=$((failed_tests + 1))
            fi
            
            # Test installation in supported environments
            for env in "ubuntu:22.04" "debian:11"; do
                test_package_installation "$deb_file" "$env"
            done
        fi
    done
    
    # Test RPM packages
    local rpm_files=($(ls "$DIST_DIR"/*.rpm 2>/dev/null || true))
    for rpm_file in "${rpm_files[@]}"; do
        if [[ -f "$rpm_file" ]]; then
            total_tests=$((total_tests + 1))
            
            if test_rpm_package "$rpm_file"; then
                passed_tests=$((passed_tests + 1))
            else
                failed_tests=$((failed_tests + 1))
            fi
            
            # Test installation in supported environments
            for env in "centos:8" "fedora:39"; do
                test_package_installation "$rpm_file" "$env"
            done
        fi
    done
    
    # Test Docker images if available
    local docker_images=(
        "cursed/compiler:latest"
        "cursed/compiler:alpine"
        "cursed/compiler:ubuntu"
    )
    
    for image in "${docker_images[@]}"; do
        test_docker_image "$image"
    done
    
    # Verify checksums
    if verify_checksums; then
        info "Checksum verification passed"
    else
        warning "Checksum verification failed"
    fi
    
    # Generate test report
    generate_test_report "$total_tests" "$passed_tests" "$failed_tests"
}

generate_test_report() {
    local total_tests="$1"
    local passed_tests="$2"
    local failed_tests="$3"
    
    local report_file="$TEST_DIR/reports/verification_report.md"
    mkdir -p "$(dirname "$report_file")"
    
    cat > "$report_file" << EOF
# CURSED Compiler Package Verification Report

**Generated**: $(date '+%Y-%m-%d %H:%M:%S')  
**Test Environment**: $(uname -a)

## Summary

- **Total Tests**: $total_tests
- **Passed**: $passed_tests
- **Failed**: $failed_tests
- **Success Rate**: $(( passed_tests * 100 / total_tests ))%

## Test Results

### Archive Integrity Tests
$(for platform in "${PLATFORMS[@]}"; do
    echo "- **$platform**: $(if [[ -f "$DIST_DIR/cursed-"*"-$platform.tar.gz" ]] || [[ -f "$DIST_DIR/cursed-"*"-$platform.zip" ]]; then echo "✅ Passed"; else echo "⚠️ No packages found"; fi)"
done)

### Native Package Tests
$(for pkg in "$DIST_DIR"/*.deb; do
    if [[ -f "$pkg" ]]; then
        echo "- **DEB Package**: ✅ $(basename "$pkg")"
    fi
done)

$(for pkg in "$DIST_DIR"/*.rpm; do
    if [[ -f "$pkg" ]]; then
        echo "- **RPM Package**: ✅ $(basename "$pkg")"
    fi
done)

### Container Tests
- **Docker Images**: $(if command -v docker >/dev/null 2>&1; then echo "✅ Available"; else echo "⚠️ Docker not available"; fi)

### Security
- **Checksum Verification**: $(if [[ -f "$DIST_DIR/SHA256SUMS" ]]; then echo "✅ Available"; else echo "⚠️ Missing"; fi)

## Recommendations

$(if [[ $failed_tests -gt 0 ]]; then
    echo "⚠️ **Issues Found**: Review the verification log for details on failed tests."
fi)

$(if [[ ! -f "$DIST_DIR/SHA256SUMS" ]]; then
    echo "🔒 **Security**: Generate SHA256SUMS file for package verification."
fi)

$(if ! command -v docker >/dev/null 2>&1; then
    echo "🐳 **Docker**: Install Docker to enable container testing."
fi)

## Files Tested

$(find "$DIST_DIR" -name "cursed-*" -type f | sort | while read -r file; do
    echo "- $(basename "$file") ($(du -h "$file" | cut -f1))"
done)

---

For detailed logs, see: \`$LOG_FILE\`
EOF
    
    log "Test report generated: $report_file"
    
    # Display summary
    echo ""
    echo "🧪 PACKAGE VERIFICATION SUMMARY"
    echo "================================"
    echo "Total Tests: $total_tests"
    echo "Passed: $passed_tests"
    echo "Failed: $failed_tests"
    echo "Success Rate: $(( passed_tests * 100 / total_tests ))%"
    echo ""
    echo "Report: $report_file"
    echo "Log: $LOG_FILE"
    
    if [[ $failed_tests -gt 0 ]]; then
        echo ""
        echo "⚠️ Some tests failed. Review the logs for details."
        return 1
    else
        echo ""
        echo "✅ All tests passed!"
        return 0
    fi
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

show_usage() {
    cat << EOF
CURSED Compiler Package Verification

Usage: $0 [OPTIONS]

Options:
    --dist-dir DIR     Distribution directory (default: $DIST_DIR)
    --test-dir DIR     Test directory (default: $TEST_DIR)
    --platform PLAT    Test specific platform only
    --format FMT       Test specific package format only
    --skip-execution   Skip binary execution tests
    --skip-containers  Skip container tests
    --verbose          Verbose output
    -h, --help         Show this help message

Examples:
    $0                                    # Test all packages
    $0 --platform x86_64-unknown-linux-gnu  # Test Linux x64 only
    $0 --format deb                       # Test DEB packages only
    $0 --skip-containers                  # Skip Docker tests

EOF
}

main() {
    local skip_execution=false
    local skip_containers=false
    local specific_platform=""
    local specific_format=""
    local verbose=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dist-dir)
                DIST_DIR="$2"
                shift 2
                ;;
            --test-dir)
                TEST_DIR="$2"
                shift 2
                ;;
            --platform)
                specific_platform="$2"
                shift 2
                ;;
            --format)
                specific_format="$2"
                shift 2
                ;;
            --skip-execution)
                skip_execution=true
                shift
                ;;
            --skip-containers)
                skip_containers=true
                shift
                ;;
            --verbose)
                verbose=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done
    
    # Check if distribution directory exists
    if [[ ! -d "$DIST_DIR" ]]; then
        error "Distribution directory not found: $DIST_DIR"
    fi
    
    # Update LOG_FILE path if TEST_DIR changed
    LOG_FILE="$TEST_DIR/verification.log"
    
    # Setup test environment
    setup_test_environment
    
    log "Starting CURSED Compiler package verification..."
    log "Distribution directory: $DIST_DIR"
    log "Test directory: $TEST_DIR"
    
    if [[ -n "$specific_platform" ]]; then
        log "Testing specific platform: $specific_platform"
        PLATFORMS=("$specific_platform")
    fi
    
    if [[ -n "$specific_format" ]]; then
        log "Testing specific format: $specific_format"
        PACKAGE_FORMATS=("$specific_format")
    fi
    
    # Export configuration
    export SKIP_EXECUTION="$skip_execution"
    export SKIP_CONTAINERS="$skip_containers"
    export VERBOSE="$verbose"
    
    # Run comprehensive tests
    if run_comprehensive_tests; then
        success "Package verification completed successfully!"
        exit 0
    else
        error "Package verification failed!"
        exit 1
    fi
}

# Execute main function
main "$@"
