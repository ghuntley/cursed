#!/bin/bash

# CURSED Build System Health Check
# =================================
# Comprehensive health check for the CURSED build system
# Validates environment, dependencies, and build system integrity

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RESET='\033[0m'
BOLD='\033[1m'
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
BLUE='\033[34m'
CYAN='\033[36m'

# Counters
CHECKS_TOTAL=0
CHECKS_PASSED=0
CHECKS_FAILED=0
CHECKS_WARNINGS=0

# Results storage
FAILED_CHECKS=()
WARNING_CHECKS=()

# Logging functions
log() {
    echo -e "${CYAN}[INFO]${RESET} $*"
}

warn() {
    echo -e "${YELLOW}[WARN]${RESET} $*"
    ((CHECKS_WARNINGS++))
    WARNING_CHECKS+=("$*")
}

error() {
    echo -e "${RED}[FAIL]${RESET} $*"
    ((CHECKS_FAILED++))
    FAILED_CHECKS+=("$*")
}

success() {
    echo -e "${GREEN}[PASS]${RESET} $*"
    ((CHECKS_PASSED++))
}

check() {
    ((CHECKS_TOTAL++))
    echo -n "  Checking $1... "
}

pass() {
    echo -e "${GREEN}✓${RESET}"
    ((CHECKS_PASSED++))
}

fail() {
    echo -e "${RED}✗${RESET}"
    ((CHECKS_FAILED++))
    FAILED_CHECKS+=("$1")
}

warning() {
    echo -e "${YELLOW}⚠${RESET}"
    ((CHECKS_WARNINGS++))
    WARNING_CHECKS+=("$1")
}

# Environment checks
check_environment() {
    echo -e "${BOLD}${BLUE}Environment Checks${RESET}"
    echo "=========================="
    
    # Check if we're in the right directory
    check "project root directory"
    if [[ -f "$PROJECT_ROOT/Cargo.toml" ]]; then
        pass
    else
        fail "Not in CURSED project root"
    fi
    
    # Check Rust installation
    check "Rust installation"
    if command -v rustc >/dev/null 2>&1; then
        local rust_version
        rust_version=$(rustc --version)
        pass
        echo "    Version: $rust_version"
    else
        fail "Rust not installed"
    fi
    
    # Check Cargo installation
    check "Cargo installation"
    if command -v cargo >/dev/null 2>&1; then
        local cargo_version
        cargo_version=$(cargo --version)
        pass
        echo "    Version: $cargo_version"
    else
        fail "Cargo not installed"
    fi
    
    # Check LLVM installation
    check "LLVM installation"
    if command -v llvm-config >/dev/null 2>&1; then
        local llvm_version
        llvm_version=$(llvm-config --version 2>/dev/null || echo "unknown")
        pass
        echo "    Version: $llvm_version"
    else
        warning "LLVM not found in PATH"
    fi
    
    # Check linking fix script
    check "linking fix script"
    if [[ -x "$PROJECT_ROOT/fix_linking.sh" ]]; then
        pass
    else
        fail "fix_linking.sh not found or not executable"
    fi
    
    # Check devenv
    check "devenv availability"
    if command -v devenv >/dev/null 2>&1; then
        pass
    else
        warning "devenv not available"
    fi
    
    echo ""
}

# Build system checks
check_build_system() {
    echo -e "${BOLD}${BLUE}Build System Checks${RESET}"
    echo "=========================="
    
    # Check Makefile existence
    check "Makefile existence"
    if [[ -f "$PROJECT_ROOT/Makefile" ]]; then
        pass
    else
        fail "Makefile not found"
    fi
    
    # Check Makefile syntax
    check "Makefile syntax"
    if make -f "$PROJECT_ROOT/Makefile" --dry-run help >/dev/null 2>&1; then
        pass
    else
        fail "Makefile syntax errors"
    fi
    
    # Check optimization Makefile
    check "optimization Makefile"
    if [[ -f "$PROJECT_ROOT/Makefile.optimization" ]]; then
        if make -f "$PROJECT_ROOT/Makefile.optimization" --dry-run opt-help >/dev/null 2>&1; then
            pass
        else
            warning "Makefile.optimization has syntax errors"
        fi
    else
        warning "Makefile.optimization not found"
    fi
    
    # Check for duplicate targets
    check "duplicate targets"
    if [[ -f "$PROJECT_ROOT/Makefile" ]]; then
        local duplicates
        duplicates=$(grep "^[a-zA-Z_-]*:" "$PROJECT_ROOT/Makefile" | sort | uniq -d | wc -l)
        if [[ $duplicates -eq 0 ]]; then
            pass
        else
            warning "$duplicates duplicate targets found"
        fi
    else
        fail "Cannot check - Makefile missing"
    fi
    
    # Check essential targets
    local essential_targets=("build" "test" "clean" "help" "lint" "fmt")
    for target in "${essential_targets[@]}"; do
        check "target '$target'"
        if make -f "$PROJECT_ROOT/Makefile" --dry-run "$target" >/dev/null 2>&1; then
            pass
        else
            fail "Target '$target' not available"
        fi
    done
    
    echo ""
}

# Dependency checks
check_dependencies() {
    echo -e "${BOLD}${BLUE}Dependency Checks${RESET}"
    echo "=========================="
    
    # Check Cargo.toml
    check "Cargo.toml structure"
    if [[ -f "$PROJECT_ROOT/Cargo.toml" ]]; then
        if cargo metadata --no-deps >/dev/null 2>&1; then
            pass
        else
            fail "Cargo.toml has errors"
        fi
    else
        fail "Cargo.toml not found"
    fi
    
    # Check lock file
    check "Cargo.lock file"
    if [[ -f "$PROJECT_ROOT/Cargo.lock" ]]; then
        pass
    else
        warning "Cargo.lock not found - run 'cargo build' to generate"
    fi
    
    # Check LLVM environment variables
    check "LLVM environment setup"
    if [[ -n "${LLVM_SYS_170_PREFIX:-}" ]] || [[ -n "${LLVM_CONFIG_PATH:-}" ]]; then
        pass
        if [[ -n "${LLVM_SYS_170_PREFIX:-}" ]]; then
            echo "    LLVM_SYS_170_PREFIX: $LLVM_SYS_170_PREFIX"
        fi
        if [[ -n "${LLVM_CONFIG_PATH:-}" ]]; then
            echo "    LLVM_CONFIG_PATH: $LLVM_CONFIG_PATH"
        fi
    else
        warning "LLVM environment variables not set"
    fi
    
    # Check library paths for Nix
    check "library paths (Nix)"
    if [[ -n "${LIBRARY_PATH:-}" ]]; then
        pass
        echo "    LIBRARY_PATH configured"
    else
        warning "LIBRARY_PATH not set (may be needed for Nix)"
    fi
    
    # Check required tools
    local tools=("git" "make" "gcc" "pkg-config")
    for tool in "${tools[@]}"; do
        check "tool '$tool'"
        if command -v "$tool" >/dev/null 2>&1; then
            pass
        else
            warning "Tool '$tool' not found"
        fi
    done
    
    echo ""
}

# Build functionality checks
check_build_functionality() {
    echo -e "${BOLD}${BLUE}Build Functionality Checks${RESET}"
    echo "=========================="
    
    cd "$PROJECT_ROOT"
    
    # Check basic help
    check "make help"
    if timeout 10 make help >/dev/null 2>&1; then
        pass
    else
        fail "make help failed or timed out"
    fi
    
    # Check status
    check "make status"
    if timeout 10 make status >/dev/null 2>&1; then
        pass
    else
        warning "make status failed"
    fi
    
    # Check health-check target
    check "make health-check"
    if timeout 10 make health-check >/dev/null 2>&1; then
        pass
    else
        warning "make health-check failed"
    fi
    
    # Check linking fix
    check "linking fix functionality"
    if timeout 10 ./fix_linking.sh echo "test" >/dev/null 2>&1; then
        pass
    else
        fail "linking fix script failed"
    fi
    
    # Check cargo check (quick)
    check "cargo check (syntax)"
    if timeout 30 ./fix_linking.sh cargo check --quiet >/dev/null 2>&1; then
        pass
    else
        warning "cargo check failed (may need dependencies)"
    fi
    
    echo ""
}

# File system checks
check_file_system() {
    echo -e "${BOLD}${BLUE}File System Checks${RESET}"
    echo "=========================="
    
    # Check essential directories
    local dirs=("src" "tests" "examples" "scripts" "docs")
    for dir in "${dirs[@]}"; do
        check "directory '$dir'"
        if [[ -d "$PROJECT_ROOT/$dir" ]]; then
            pass
        else
            warning "Directory '$dir' not found"
        fi
    done
    
    # Check for common build artifacts
    check "target directory"
    if [[ -d "$PROJECT_ROOT/target" ]]; then
        pass
        local size
        size=$(du -sh "$PROJECT_ROOT/target" 2>/dev/null | cut -f1 || echo "unknown")
        echo "    Size: $size"
    else
        warning "Target directory not found (run 'make build')"
    fi
    
    # Check permissions
    check "write permissions"
    if [[ -w "$PROJECT_ROOT" ]]; then
        pass
    else
        fail "No write permission to project directory"
    fi
    
    # Check disk space
    check "disk space"
    local available
    available=$(df -h "$PROJECT_ROOT" | awk 'NR==2 {print $4}' 2>/dev/null || echo "unknown")
    if [[ "$available" != "unknown" ]]; then
        pass
        echo "    Available: $available"
    else
        warning "Could not check disk space"
    fi
    
    echo ""
}

# Performance checks
check_performance() {
    echo -e "${BOLD}${BLUE}Performance Checks${RESET}"
    echo "=========================="
    
    # Check CPU cores
    check "CPU cores"
    local cores
    cores=$(nproc 2>/dev/null || echo "unknown")
    if [[ "$cores" != "unknown" ]]; then
        pass
        echo "    Available cores: $cores"
    else
        warning "Could not detect CPU cores"
    fi
    
    # Check memory
    check "system memory"
    local memory
    memory=$(free -h 2>/dev/null | awk 'NR==2{print $2}' || echo "unknown")
    if [[ "$memory" != "unknown" ]]; then
        pass
        echo "    Total memory: $memory"
    else
        warning "Could not check system memory"
    fi
    
    # Check load average
    check "system load"
    local load
    load=$(uptime | awk -F'load average:' '{print $2}' | awk '{print $1}' | tr -d ',' 2>/dev/null || echo "unknown")
    if [[ "$load" != "unknown" ]]; then
        pass
        echo "    Load average: $load"
    else
        warning "Could not check system load"
    fi
    
    echo ""
}

# Integration checks
check_integration() {
    echo -e "${BOLD}${BLUE}Integration Checks${RESET}"
    echo "=========================="
    
    # Check devenv.nix
    check "devenv.nix configuration"
    if [[ -f "$PROJECT_ROOT/devenv.nix" ]]; then
        pass
    else
        warning "devenv.nix not found"
    fi
    
    # Check .cargo/config.toml
    check "cargo configuration"
    if [[ -f "$PROJECT_ROOT/.cargo/config.toml" ]]; then
        pass
    else
        warning ".cargo/config.toml not found"
    fi
    
    # Check git configuration
    check "git repository"
    if [[ -d "$PROJECT_ROOT/.git" ]]; then
        pass
    else
        warning "Not a git repository"
    fi
    
    # Check CI configuration
    check "CI configuration"
    if [[ -d "$PROJECT_ROOT/.github" ]] || [[ -f "$PROJECT_ROOT/.gitlab-ci.yml" ]]; then
        pass
    else
        warning "No CI configuration found"
    fi
    
    echo ""
}

# Generate summary report
generate_summary() {
    echo -e "${BOLD}${CYAN}Health Check Summary${RESET}"
    echo "=========================="
    echo ""
    echo -e "Total checks: ${BOLD}$CHECKS_TOTAL${RESET}"
    echo -e "Passed: ${GREEN}$CHECKS_PASSED${RESET}"
    echo -e "Failed: ${RED}$CHECKS_FAILED${RESET}"
    echo -e "Warnings: ${YELLOW}$CHECKS_WARNINGS${RESET}"
    echo ""
    
    # Calculate percentage
    local pass_percentage
    if [[ $CHECKS_TOTAL -gt 0 ]]; then
        pass_percentage=$((CHECKS_PASSED * 100 / CHECKS_TOTAL))
    else
        pass_percentage=0
    fi
    
    # Overall status
    if [[ $CHECKS_FAILED -eq 0 ]]; then
        if [[ $CHECKS_WARNINGS -eq 0 ]]; then
            echo -e "Overall status: ${GREEN}${BOLD}EXCELLENT${RESET} (${pass_percentage}%)"
        else
            echo -e "Overall status: ${YELLOW}${BOLD}GOOD${RESET} (${pass_percentage}%)"
        fi
    else
        if [[ $CHECKS_FAILED -lt 3 ]]; then
            echo -e "Overall status: ${YELLOW}${BOLD}NEEDS ATTENTION${RESET} (${pass_percentage}%)"
        else
            echo -e "Overall status: ${RED}${BOLD}CRITICAL ISSUES${RESET} (${pass_percentage}%)"
        fi
    fi
    
    echo ""
    
    # Show failed checks
    if [[ ${#FAILED_CHECKS[@]} -gt 0 ]]; then
        echo -e "${RED}${BOLD}Failed Checks:${RESET}"
        for check in "${FAILED_CHECKS[@]}"; do
            echo -e "  ${RED}✗${RESET} $check"
        done
        echo ""
    fi
    
    # Show warnings
    if [[ ${#WARNING_CHECKS[@]} -gt 0 ]]; then
        echo -e "${YELLOW}${BOLD}Warnings:${RESET}"
        for check in "${WARNING_CHECKS[@]}"; do
            echo -e "  ${YELLOW}⚠${RESET} $check"
        done
        echo ""
    fi
    
    # Recommendations
    echo -e "${BOLD}Recommendations:${RESET}"
    if [[ $CHECKS_FAILED -gt 0 ]]; then
        echo -e "  ${RED}•${RESET} Address failed checks before proceeding with builds"
    fi
    if [[ $CHECKS_WARNINGS -gt 0 ]]; then
        echo -e "  ${YELLOW}•${RESET} Review warnings for potential improvements"
    fi
    if [[ $CHECKS_FAILED -eq 0 && $CHECKS_WARNINGS -eq 0 ]]; then
        echo -e "  ${GREEN}•${RESET} Build system is healthy and ready to use"
    fi
    
    echo -e "  ${CYAN}•${RESET} Run 'make help' to see available build targets"
    echo -e "  ${CYAN}•${RESET} Use 'make status' for ongoing system status"
    echo ""
}

# Main function
main() {
    echo -e "${BOLD}${CYAN}CURSED Build System Health Check${RESET}"
    echo -e "${CYAN}===================================${RESET}"
    echo ""
    
    # Parse arguments
    local verbose=false
    local quick=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --verbose|-v)
                verbose=true
                shift
                ;;
            --quick|-q)
                quick=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --verbose, -v   Show detailed output"
                echo "  --quick, -q     Run quick checks only"
                echo "  --help, -h      Show this help"
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Run checks
    check_environment
    check_build_system
    check_dependencies
    
    if [[ "$quick" != "true" ]]; then
        check_build_functionality
        check_file_system
        check_performance
        check_integration
    fi
    
    # Generate summary
    generate_summary
    
    # Exit with appropriate code
    if [[ $CHECKS_FAILED -gt 0 ]]; then
        exit 1
    elif [[ $CHECKS_WARNINGS -gt 0 ]]; then
        exit 2
    else
        exit 0
    fi
}

# Run main function
main "$@"
