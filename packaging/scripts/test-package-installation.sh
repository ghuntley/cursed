#!/bin/bash
# CURSED Programming Language - Package Installation Testing Script
# Oracle Week 3 cross-platform packaging preparation for v1.0 launch
set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DIST_DIR="${PROJECT_ROOT}/dist"
TEST_DIR="${PROJECT_ROOT}/test-installations"
VERSION="${VERSION:-$(cat ${PROJECT_ROOT}/VERSION 2>/dev/null || echo "1.0.0")}"

# Test configuration
TEST_TIMEOUT=300  # 5 minutes timeout for each test
CLEANUP_AFTER_TEST="${CLEANUP_AFTER_TEST:-true}"
VERBOSE="${VERBOSE:-false}"

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

log_test() {
    echo -e "${PURPLE}[TEST]${NC} $1"
}

log_package() {
    echo -e "${CYAN}[PACKAGE]${NC} $1"
}

# Detect current platform
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux*)
            echo "linux"
            ;;
        darwin*)
            echo "macos"
            ;;
        cygwin*|mingw*|msys*|windows*)
            echo "windows"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Create test environment
setup_test_environment() {
    log_info "Setting up test environment..."
    
    # Clean and create test directory
    rm -rf "${TEST_DIR}"
    mkdir -p "${TEST_DIR}"
    
    # Create test program
    cat > "${TEST_DIR}/hello_test.csd" << 'EOF'
yeet "vibez";
yeet "mathz";
yeet "stringz";

sus greeting tea = "Hello from CURSED!";
sus version tea = "v1.0.0";
sus number drip = 42;

vibez.spill(greeting);
vibez.spill("Version:", version);
vibez.spill("Lucky number:", number);

sus doubled drip = mathz.multiply(number, 2);
vibez.spill("Doubled:", doubled);

sus upper_greeting tea = stringz.to_upper(greeting);
vibez.spill("Uppercase:", upper_greeting);

vibez.spill("✅ All tests passed!");
EOF
    
    # Create advanced test program
    cat > "${TEST_DIR}/advanced_test.csd" << 'EOF'
yeet "vibez";
yeet "mathz";
yeet "arrayz";
yeet "testz";

# Test function definitions
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n;
    }
    damn fibonacci(n - 1) + fibonacci(n - 2);
}

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1;
    }
    damn n * factorial(n - 1);
}

# Test arrays and loops
sus numbers []drip = [1, 2, 3, 4, 5];
sus sum drip = 0;

bestie (sus item drip : numbers) {
    sum = sum + item;
}

# Test error handling
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero";
    }
    damn a / b;
}

# Main test execution
test_start("CURSED Advanced Features Test");

# Test basic arithmetic
assert_eq_int(2 + 2, 4);
assert_eq_int(fibonacci(10), 55);
assert_eq_int(factorial(5), 120);
assert_eq_int(sum, 15);

# Test error handling
sus division_result drip = divide(10, 2) fam {
    when "division by zero" -> {
        vibez.spill("Caught division by zero error");
        damn -1;
    }
};
assert_eq_int(division_result, 5);

vibez.spill("🎉 Advanced test completed successfully!");
print_test_summary();
EOF
    
    log_success "Test environment created at: ${TEST_DIR}"
}

# Test archive installation (tar.gz/zip)
test_archive_installation() {
    local platform="$1"
    local archive_file=""
    local extract_command=""
    
    log_test "Testing archive installation for ${platform}..."
    
    # Determine archive file and extract command
    case "${platform}" in
        "linux")
            archive_file="${DIST_DIR}/cursed-${VERSION}-x86_64-linux-gnu.tar.gz"
            extract_command="tar -xzf"
            ;;
        "macos")
            # Try Apple Silicon first, fall back to Intel
            if [[ "$(uname -m)" == "arm64" ]]; then
                archive_file="${DIST_DIR}/cursed-${VERSION}-aarch64-apple-darwin.tar.gz"
            else
                archive_file="${DIST_DIR}/cursed-${VERSION}-x86_64-apple-darwin.tar.gz"
            fi
            extract_command="tar -xzf"
            ;;
        "windows")
            archive_file="${DIST_DIR}/cursed-${VERSION}-x86_64-pc-windows-gnu.zip"
            extract_command="unzip -q"
            ;;
        *)
            log_warning "Unsupported platform for archive test: ${platform}"
            return 1
            ;;
    esac
    
    # Check if archive exists
    if [[ ! -f "${archive_file}" ]]; then
        log_warning "Archive not found: ${archive_file}"
        return 1
    fi
    
    local test_install_dir="${TEST_DIR}/archive_install"
    mkdir -p "${test_install_dir}"
    
    # Extract archive
    cd "${test_install_dir}"
    if ! ${extract_command} "${archive_file}"; then
        log_error "Failed to extract archive: ${archive_file}"
        return 1
    fi
    
    # Find extracted directory
    local extracted_dir=""
    for dir in */; do
        if [[ -d "${dir}" ]]; then
            extracted_dir="${dir}"
            break
        fi
    done
    
    if [[ -z "${extracted_dir}" ]]; then
        log_error "No extracted directory found"
        return 1
    fi
    
    cd "${extracted_dir}"
    
    # Test binary execution
    local cursed_binary="./cursed"
    if [[ "${platform}" == "windows" ]]; then
        cursed_binary="./cursed.exe"
    fi
    
    if [[ ! -f "${cursed_binary}" ]]; then
        log_error "CURSED binary not found: ${cursed_binary}"
        return 1
    fi
    
    # Test version command
    if ! timeout ${TEST_TIMEOUT} ${cursed_binary} --version >/dev/null 2>&1; then
        log_error "Version command failed"
        return 1
    fi
    
    # Test simple program execution
    if ! timeout ${TEST_TIMEOUT} ${cursed_binary} "${TEST_DIR}/hello_test.csd" >/dev/null 2>&1; then
        log_error "Simple program execution failed"
        return 1
    fi
    
    # Test advanced program execution
    if ! timeout ${TEST_TIMEOUT} ${cursed_binary} "${TEST_DIR}/advanced_test.csd" >/dev/null 2>&1; then
        log_error "Advanced program execution failed"
        return 1
    fi
    
    log_success "Archive installation test passed for ${platform}"
    return 0
}

# Test native package installation (DEB/RPM)
test_native_package_installation() {
    local package_type="$1"
    
    log_test "Testing ${package_type} package installation..."
    
    # Check if we're running on appropriate system
    case "${package_type}" in
        "deb")
            if ! command -v dpkg >/dev/null 2>&1; then
                log_warning "dpkg not available - skipping DEB test"
                return 1
            fi
            ;;
        "rpm")
            if ! command -v rpm >/dev/null 2>&1; then
                log_warning "rpm not available - skipping RPM test"
                return 1
            fi
            ;;
        *)
            log_warning "Unsupported package type: ${package_type}"
            return 1
            ;;
    esac
    
    local package_file=""
    local install_command=""
    local remove_command=""
    local query_command=""
    
    case "${package_type}" in
        "deb")
            package_file="${DIST_DIR}/cursed_${VERSION}-1_amd64.deb"
            install_command="sudo dpkg -i"
            remove_command="sudo dpkg -r cursed"
            query_command="dpkg -l cursed"
            ;;
        "rpm")
            package_file="${DIST_DIR}/cursed-${VERSION}-1.x86_64.rpm"
            install_command="sudo rpm -i"
            remove_command="sudo rpm -e cursed"
            query_command="rpm -q cursed"
            ;;
    esac
    
    # Check if package exists
    if [[ ! -f "${package_file}" ]]; then
        log_warning "Package not found: ${package_file}"
        return 1
    fi
    
    # Test package installation (requires sudo)
    if [[ "${EUID}" -ne 0 ]] && ! sudo -n true 2>/dev/null; then
        log_warning "Sudo access required for native package testing - skipping"
        return 1
    fi
    
    # Install package
    log_package "Installing ${package_type} package..."
    if ! ${install_command} "${package_file}"; then
        log_error "Failed to install ${package_type} package"
        return 1
    fi
    
    # Verify installation
    if ! ${query_command} >/dev/null 2>&1; then
        log_error "Package not found after installation"
        ${remove_command} || true
        return 1
    fi
    
    # Test installed binary
    if ! command -v cursed >/dev/null 2>&1; then
        log_error "cursed command not available after installation"
        ${remove_command} || true
        return 1
    fi
    
    # Test version command
    if ! timeout ${TEST_TIMEOUT} cursed --version >/dev/null 2>&1; then
        log_error "Version command failed after installation"
        ${remove_command} || true
        return 1
    fi
    
    # Test program execution
    if ! timeout ${TEST_TIMEOUT} cursed "${TEST_DIR}/hello_test.csd" >/dev/null 2>&1; then
        log_error "Program execution failed after installation"
        ${remove_command} || true
        return 1
    fi
    
    log_success "${package_type} package installation test passed"
    
    # Clean up
    if [[ "${CLEANUP_AFTER_TEST}" == "true" ]]; then
        log_info "Removing installed package..."
        ${remove_command} || true
    fi
    
    return 0
}

# Test Homebrew installation (macOS)
test_homebrew_installation() {
    if [[ "$(detect_platform)" != "macos" ]]; then
        log_warning "Not on macOS - skipping Homebrew test"
        return 1
    fi
    
    if ! command -v brew >/dev/null 2>&1; then
        log_warning "Homebrew not installed - skipping Homebrew test"
        return 1
    fi
    
    log_test "Testing Homebrew installation..."
    
    local formula_path="${PROJECT_ROOT}/packaging/package-managers/homebrew/cursed.rb"
    if [[ ! -f "${formula_path}" ]]; then
        log_error "Homebrew formula not found: ${formula_path}"
        return 1
    fi
    
    # Test formula installation
    log_package "Installing Homebrew formula..."
    if ! brew install "${formula_path}"; then
        log_error "Failed to install Homebrew formula"
        return 1
    fi
    
    # Test installed binary
    if ! command -v cursed >/dev/null 2>&1; then
        log_error "cursed command not available after Homebrew installation"
        brew uninstall cursed || true
        return 1
    fi
    
    # Test functionality
    if ! timeout ${TEST_TIMEOUT} cursed --version >/dev/null 2>&1; then
        log_error "Version command failed after Homebrew installation"
        brew uninstall cursed || true
        return 1
    fi
    
    if ! timeout ${TEST_TIMEOUT} cursed "${TEST_DIR}/hello_test.csd" >/dev/null 2>&1; then
        log_error "Program execution failed after Homebrew installation"
        brew uninstall cursed || true
        return 1
    fi
    
    log_success "Homebrew installation test passed"
    
    # Clean up
    if [[ "${CLEANUP_AFTER_TEST}" == "true" ]]; then
        log_info "Uninstalling Homebrew formula..."
        brew uninstall cursed || true
    fi
    
    return 0
}

# Test Docker container
test_docker_container() {
    if ! command -v docker >/dev/null 2>&1; then
        log_warning "Docker not available - skipping container test"
        return 1
    fi
    
    log_test "Testing Docker container..."
    
    local dockerfile_path="${PROJECT_ROOT}/packaging/docker/Dockerfile"
    if [[ ! -f "${dockerfile_path}" ]]; then
        log_warning "Dockerfile not found - skipping container test"
        return 1
    fi
    
    local image_name="cursed-test:${VERSION}"
    
    # Build container image
    log_package "Building Docker image..."
    if ! docker build -t "${image_name}" -f "${dockerfile_path}" "${PROJECT_ROOT}"; then
        log_error "Failed to build Docker image"
        return 1
    fi
    
    # Test container execution
    log_package "Testing container execution..."
    if ! docker run --rm -v "${TEST_DIR}:/workspace" "${image_name}" cursed --version >/dev/null 2>&1; then
        log_error "Container version command failed"
        docker rmi "${image_name}" || true
        return 1
    fi
    
    # Test program execution in container
    if ! timeout ${TEST_TIMEOUT} docker run --rm -v "${TEST_DIR}:/workspace" "${image_name}" cursed /workspace/hello_test.csd >/dev/null 2>&1; then
        log_error "Container program execution failed"
        docker rmi "${image_name}" || true
        return 1
    fi
    
    log_success "Docker container test passed"
    
    # Clean up
    if [[ "${CLEANUP_AFTER_TEST}" == "true" ]]; then
        log_info "Removing Docker image..."
        docker rmi "${image_name}" || true
    fi
    
    return 0
}

# Test signature verification
test_signature_verification() {
    if ! command -v cosign >/dev/null 2>&1; then
        log_warning "cosign not available - skipping signature verification test"
        return 1
    fi
    
    log_test "Testing signature verification..."
    
    # Find a signed artifact
    local signed_file=""
    for file in "${DIST_DIR}"/*.tar.gz "${DIST_DIR}"/*.zip; do
        if [[ -f "${file}" && -f "${file}.sig" ]]; then
            signed_file="${file}"
            break
        fi
    done
    
    if [[ -z "${signed_file}" ]]; then
        log_warning "No signed artifacts found - skipping signature verification test"
        return 1
    fi
    
    # Find public key
    local public_key_file=""
    for key_file in "${DIST_DIR}"/*public-keys.pem "${PROJECT_ROOT}/packaging/keys"/*.pub; do
        if [[ -f "${key_file}" ]]; then
            public_key_file="${key_file}"
            break
        fi
    done
    
    if [[ -z "${public_key_file}" ]]; then
        log_warning "No public key found - skipping signature verification test"
        return 1
    fi
    
    # Verify signature
    log_package "Verifying signature for: $(basename "${signed_file}")"
    if cosign verify-blob --key "${public_key_file}" --signature "${signed_file}.sig" "${signed_file}" >/dev/null 2>&1; then
        log_success "Signature verification test passed"
        return 0
    else
        log_error "Signature verification failed"
        return 1
    fi
}

# Run comprehensive test suite
run_comprehensive_tests() {
    local platform=$(detect_platform)
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    log_info "Running comprehensive package installation tests..."
    log_info "Platform: ${platform}"
    log_info "Version: ${VERSION}"
    echo ""
    
    setup_test_environment
    
    # Test archive installation
    ((total_tests++))
    if test_archive_installation "${platform}"; then
        ((passed_tests++))
    else
        ((failed_tests++))
    fi
    
    # Test native packages (Linux only)
    if [[ "${platform}" == "linux" ]]; then
        # Test DEB package
        ((total_tests++))
        if test_native_package_installation "deb"; then
            ((passed_tests++))
        else
            ((failed_tests++))
        fi
        
        # Test RPM package
        ((total_tests++))
        if test_native_package_installation "rpm"; then
            ((passed_tests++))
        else
            ((failed_tests++))
        fi
    fi
    
    # Test Homebrew (macOS only)
    if [[ "${platform}" == "macos" ]]; then
        ((total_tests++))
        if test_homebrew_installation; then
            ((passed_tests++))
        else
            ((failed_tests++))
        fi
    fi
    
    # Test Docker container
    ((total_tests++))
    if test_docker_container; then
        ((passed_tests++))
    else
        ((failed_tests++))
    fi
    
    # Test signature verification
    ((total_tests++))
    if test_signature_verification; then
        ((passed_tests++))
    else
        ((failed_tests++))
    fi
    
    # Summary
    echo ""
    echo "================================================================================================="
    log_info "Test Summary:"
    log_info "  Total tests: ${total_tests}"
    log_success "  Passed: ${passed_tests}"
    
    if [[ ${failed_tests} -gt 0 ]]; then
        log_error "  Failed: ${failed_tests}"
        
        # Clean up test directory
        if [[ "${CLEANUP_AFTER_TEST}" == "true" ]]; then
            rm -rf "${TEST_DIR}"
        fi
        
        return 1
    else
        log_success "All tests passed! 🎉"
        
        # Clean up test directory
        if [[ "${CLEANUP_AFTER_TEST}" == "true" ]]; then
            rm -rf "${TEST_DIR}"
        fi
        
        return 0
    fi
}

# Show usage information
show_usage() {
    echo "CURSED Package Installation Testing Script"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  archive             Test archive installation (tar.gz/zip)"
    echo "  deb                 Test DEB package installation"
    echo "  rpm                 Test RPM package installation"
    echo "  homebrew            Test Homebrew installation (macOS)"
    echo "  docker              Test Docker container"
    echo "  signatures          Test signature verification"
    echo "  all                 Run all applicable tests"
    echo "  help                Show this help message"
    echo ""
    echo "Options:"
    echo "  --no-cleanup        Don't clean up after tests"
    echo "  --verbose           Enable verbose output"
    echo "  --timeout SECONDS   Set test timeout (default: 300)"
    echo ""
    echo "Environment Variables:"
    echo "  CLEANUP_AFTER_TEST  Clean up after tests (true/false)"
    echo "  VERBOSE            Enable verbose output (true/false)"
    echo "  TEST_TIMEOUT       Test timeout in seconds"
    echo "  VERSION            Version string"
    echo ""
    echo "Examples:"
    echo "  $0 all                    # Run all tests"
    echo "  $0 archive               # Test archive installation only"
    echo "  $0 deb --no-cleanup      # Test DEB package, keep test files"
    echo "  $0 docker --verbose      # Test Docker with verbose output"
    echo ""
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --no-cleanup)
                CLEANUP_AFTER_TEST=false
                shift
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --timeout)
                TEST_TIMEOUT="$2"
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
    
    # Set verbose mode
    if [[ "${VERBOSE}" == "true" ]]; then
        set -x
    fi
    
    echo "================================================================================================="
    echo "                          CURSED Package Installation Testing"
    echo "================================================================================================="
    echo ""
    
    case "${command}" in
        "archive")
            setup_test_environment
            test_archive_installation "$(detect_platform)"
            ;;
        "deb")
            setup_test_environment
            test_native_package_installation "deb"
            ;;
        "rpm")
            setup_test_environment
            test_native_package_installation "rpm"
            ;;
        "homebrew")
            setup_test_environment
            test_homebrew_installation
            ;;
        "docker")
            setup_test_environment
            test_docker_container
            ;;
        "signatures")
            test_signature_verification
            ;;
        "all")
            run_comprehensive_tests
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
