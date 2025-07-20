#!/bin/bash

# Enhanced Cross-Compilation Test Suite for CURSED
# Based on Oracle guidance for complete validation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    ((TESTS_PASSED++))
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((TESTS_FAILED++))
}

run_test() {
    local test_name="$1"
    local test_command="$2"
    ((TOTAL_TESTS++))
    
    log_info "Running test: $test_name"
    
    if eval "$test_command"; then
        log_success "$test_name"
    else
        log_error "$test_name failed"
    fi
    echo
}

# Create test program
create_test_program() {
    cat > cross_test.csd << 'EOF'
vibez.spill("Cross-compilation test successful!")

sus x drip = 42
sus y drip = 24
sus result drip = x + y

vibez.spill("Result: " + result.spillf())
vibez.spill("CURSED cross-compilation working!")
EOF
}

# Cleanup function
cleanup() {
    rm -f cross_test.csd
    rm -f cross_test
    rm -f cross_test.exe
    rm -f *.wasm
}

# Test native compilation first
test_native_build() {
    log_info "Testing native build (baseline)"
    
    # Test basic compilation
    run_test "Native compilation" "cargo check"
    run_test "Native library tests" "cargo test --lib"
    
    # Test native CURSED program
    if command -v cursed &> /dev/null || cargo run --bin cursed -- --help &> /dev/null; then
        run_test "Native CURSED interpretation" "cargo run --bin cursed cross_test.csd"
        run_test "Native CURSED compilation" "timeout 30s cargo run --bin cursed -- compile cross_test.csd"
        
        if [ -f cross_test ]; then
            run_test "Native executable execution" "./cross_test"
        fi
    else
        log_warning "CURSED binary not available for native testing"
    fi
}

# Test WASM compilation
test_wasm_build() {
    log_info "Testing WebAssembly compilation"
    
    # Check wasm-pack availability
    if command -v wasm-pack &> /dev/null; then
        run_test "WASM pack build" "wasm-pack build --target web --dev"
    else
        log_warning "wasm-pack not available"
    fi
    
    # Test direct WASM target compilation
    run_test "WASM target check" "cargo check --target wasm32-unknown-unknown"
    
    # Try WASM compilation with CURSED
    if command -v cursed &> /dev/null || cargo run --bin cursed -- --help &> /dev/null; then
        run_test "CURSED WASM compilation" "timeout 30s cargo run --bin cursed -- compile --target wasm32-unknown-unknown cross_test.csd"
    fi
}

# Test Linux cross-compilation
test_linux_cross_compilation() {
    log_info "Testing Linux cross-compilation targets"
    
    # x86_64 Linux
    run_test "Linux x86_64 check" "cargo check --target x86_64-unknown-linux-gnu"
    run_test "Linux x86_64 build" "timeout 60s cargo build --target x86_64-unknown-linux-gnu --release"
    
    # ARM64 Linux
    run_test "Linux ARM64 check" "cargo check --target aarch64-unknown-linux-gnu"
    run_test "Linux ARM64 build" "timeout 60s cargo build --target aarch64-unknown-linux-gnu --release"
    
    # Test CURSED cross-compilation to Linux
    if command -v cursed &> /dev/null || cargo run --bin cursed -- --help &> /dev/null; then
        run_test "CURSED Linux x86_64 compilation" "timeout 60s cargo run --bin cursed -- compile --target x86_64-unknown-linux-gnu cross_test.csd"
        run_test "CURSED Linux ARM64 compilation" "timeout 60s cargo run --bin cursed -- compile --target aarch64-unknown-linux-gnu cross_test.csd"
    fi
}

# Test Windows cross-compilation
test_windows_cross_compilation() {
    log_info "Testing Windows cross-compilation"
    
    # Windows x86_64
    run_test "Windows x86_64 check" "cargo check --target x86_64-pc-windows-gnu"
    run_test "Windows x86_64 build" "timeout 60s cargo build --target x86_64-pc-windows-gnu --release"
    
    # Test CURSED cross-compilation to Windows
    if command -v cursed &> /dev/null || cargo run --bin cursed -- --help &> /dev/null; then
        run_test "CURSED Windows compilation" "timeout 60s cargo run --bin cursed -- compile --target x86_64-pc-windows-gnu cross_test.csd"
    fi
}

# Test macOS cross-compilation (when not on macOS)
test_macos_cross_compilation() {
    if [[ "$OSTYPE" != "darwin"* ]]; then
        log_info "Testing macOS cross-compilation from non-macOS"
        
        run_test "macOS x86_64 check" "cargo check --target x86_64-apple-darwin"
        run_test "macOS ARM64 check" "cargo check --target aarch64-apple-darwin"
    else
        log_info "On macOS - testing cross-architecture compilation"
        
        if [[ "$(uname -m)" == "arm64" ]]; then
            run_test "macOS x86_64 from ARM64 check" "cargo check --target x86_64-apple-darwin"
            run_test "macOS x86_64 from ARM64 build" "timeout 60s cargo build --target x86_64-apple-darwin --release"
        else
            run_test "macOS ARM64 from x86_64 check" "cargo check --target aarch64-apple-darwin"
            run_test "macOS ARM64 from x86_64 build" "timeout 60s cargo build --target aarch64-apple-darwin --release"
        fi
    fi
}

# Test environment configuration
test_environment_setup() {
    log_info "Testing cross-compilation environment setup"
    
    # Check LLVM version match
    if command -v llvm-config &> /dev/null; then
        LLVM_VERSION=$(llvm-config --version 2>/dev/null || echo "unknown")
        RUST_LLVM_VERSION=$(rustc -vV | grep "LLVM version" | cut -d: -f2 | tr -d ' ')
        
        if [[ "$LLVM_VERSION" == "$RUST_LLVM_VERSION"* ]]; then
            log_success "LLVM version matches Rust ($LLVM_VERSION)"
        else
            log_warning "LLVM version mismatch: system=$LLVM_VERSION, rust=$RUST_LLVM_VERSION"
        fi
    fi
    
    # Check Zig availability
    if command -v zig &> /dev/null; then
        ZIG_VERSION=$(zig version)
        log_success "Zig available as universal linker ($ZIG_VERSION)"
    else
        log_warning "Zig not available"
    fi
    
    # Check cross-compilation toolchains
    local toolchains=(
        "x86_64-unknown-linux-gnu-gcc"
        "aarch64-unknown-linux-gnu-gcc"
        "x86_64-w64-mingw32-gcc"
    )
    
    for toolchain in "${toolchains[@]}"; do
        if command -v "$toolchain" &> /dev/null; then
            log_success "Cross-compilation toolchain available: $toolchain"
        else
            log_warning "Cross-compilation toolchain missing: $toolchain"
        fi
    done
}

# Test Zig fallback linker
test_zig_fallback() {
    if command -v zig &> /dev/null; then
        log_info "Testing Zig as fallback universal linker"
        
        # Test simple compilation with Zig
        cat > zig_test.c << 'EOF'
#include <stdio.h>
int main() {
    printf("Zig universal linker test\n");
    return 0;
}
EOF
        
        run_test "Zig compile native" "zig cc zig_test.c -o zig_test_native"
        run_test "Zig execute native" "./zig_test_native"
        
        # Test cross-compilation with Zig
        run_test "Zig cross-compile Linux" "zig cc -target x86_64-linux zig_test.c -o zig_test_linux"
        run_test "Zig cross-compile Windows" "zig cc -target x86_64-windows zig_test.c -o zig_test_windows.exe"
        
        # Cleanup
        rm -f zig_test.c zig_test_native zig_test_linux zig_test_windows.exe
    fi
}

# Main test execution
main() {
    echo "============================================================"
    echo "Enhanced CURSED Cross-Compilation Test Suite"
    echo "Based on Oracle guidance for complete validation"
    echo "============================================================"
    echo
    
    # Setup
    trap cleanup EXIT
    create_test_program
    
    # Environment diagnostics
    log_info "Environment Diagnostics:"
    echo "  OS: $(uname -a)"
    echo "  Rust: $(rustc --version)"
    echo "  Cargo: $(cargo --version)"
    if command -v llvm-config &> /dev/null; then
        echo "  LLVM: $(llvm-config --version)"
    fi
    if command -v zig &> /dev/null; then
        echo "  Zig: $(zig version)"
    fi
    echo
    
    # Run test suites
    test_environment_setup
    test_native_build
    test_wasm_build
    test_linux_cross_compilation
    test_windows_cross_compilation
    test_macos_cross_compilation
    test_zig_fallback
    
    # Summary
    echo "============================================================"
    echo "Test Results Summary"
    echo "============================================================"
    echo -e "Total tests: $TOTAL_TESTS"
    echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Failed: $TESTS_FAILED${NC}"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}All tests passed! Cross-compilation setup is working.${NC}"
        exit 0
    else
        echo -e "${RED}Some tests failed. Check the output above for details.${NC}"
        exit 1
    fi
}

main "$@"
