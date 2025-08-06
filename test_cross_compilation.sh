#!/bin/bash

# Comprehensive Cross-Compilation Test Script for CURSED Compiler
# Tests all supported target platforms and validates functionality

set -e  # Exit on error

echo "🚀 CURSED Cross-Compilation Test Suite"
echo "======================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_PROGRAM="cross_test_program.csd"
BUILD_DIR="test_cross_compilation_output"
VERBOSE=${VERBOSE:-false}

# Create test directory
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_success() {
    print_status "$GREEN" "✅ $1"
}

print_error() {
    print_status "$RED" "❌ $1"
}

print_warning() {
    print_status "$YELLOW" "⚠️  $1"
}

print_info() {
    print_status "$BLUE" "ℹ️  $1"
}

# Create a test CURSED program
create_test_program() {
    print_info "Creating test program: $TEST_PROGRAM"
    
    cat > "$TEST_PROGRAM" << 'EOF'
fr fr Cross-compilation test program for CURSED
yeet "testz"
yeet "vibez"

slay main() {
    vibez.spill("Cross-compilation test successful!")
    vibez.spill("Platform: Native execution")
    
    fr fr Test basic variables
    sus message tea = "Hello from CURSED!"
    sus version drip = 1
    sus enabled lit = based
    
    vibez.spill(message)
    vibez.spill("Version:")
    vibez.spill(version)
    vibez.spill("Enabled:")
    vibez.spill(enabled)
    
    fr fr Test arrays
    sus numbers [normie] = [1, 2, 3, 4, 5]
    vibez.spill("Array test:")
    vibez.spill(numbers[0])
    vibez.spill(numbers[4])
    
    damn 0
}
EOF
    
    print_success "Created test program: $TEST_PROGRAM"
}

# Test compilation for a specific target
test_target_compilation() {
    local target=$1
    local description=$2
    local backend=${3:-llvm}
    
    print_info "Testing compilation for $description ($target)"
    
    local output_file="$BUILD_DIR/cursed_test_$target"
    local compile_cmd="./zig-out/bin/cursed compile $TEST_PROGRAM --target $target --backend $backend -o $output_file"
    
    if [ "$VERBOSE" = "true" ]; then
        compile_cmd="$compile_cmd --verbose"
    fi
    
    echo "  Command: $compile_cmd"
    
    if $compile_cmd 2>&1; then
        # Check if output file was created
        local expected_extension=""
        case $target in
            windows-*) expected_extension=".exe" ;;
            wasm32) expected_extension=".wasm" ;;
        esac
        
        local expected_file="${output_file}${expected_extension}"
        
        if [ -f "$expected_file" ]; then
            local file_size=$(stat -c%s "$expected_file" 2>/dev/null || stat -f%z "$expected_file" 2>/dev/null || echo "unknown")
            print_success "$description: Compiled successfully ($file_size bytes)"
            
            # Test file type if available
            if command -v file >/dev/null 2>&1; then
                local file_type=$(file "$expected_file" 2>/dev/null || echo "unknown type")
                echo "    File type: $file_type"
            fi
            
            return 0
        else
            print_error "$description: Output file not found: $expected_file"
            return 1
        fi
    else
        print_error "$description: Compilation failed"
        return 1
    fi
}

# Test interpretation to ensure the test program is valid
test_interpretation() {
    print_info "Testing interpretation of test program"
    
    local interpret_cmd="./zig-out/bin/cursed $TEST_PROGRAM"
    
    if [ "$VERBOSE" = "true" ]; then
        interpret_cmd="$interpret_cmd --verbose"
    fi
    
    echo "  Command: $interpret_cmd"
    
    if $interpret_cmd > "$BUILD_DIR/interpret_output.txt" 2>&1; then
        print_success "Interpretation: Test program runs successfully"
        
        if [ "$VERBOSE" = "true" ]; then
            echo "  Output:"
            sed 's/^/    /' "$BUILD_DIR/interpret_output.txt"
        fi
        
        return 0
    else
        print_error "Interpretation: Test program failed to run"
        echo "  Error output:"
        sed 's/^/    /' "$BUILD_DIR/interpret_output.txt"
        return 1
    fi
}

# Main test execution
main() {
    echo
    print_info "Starting cross-compilation tests..."
    
    # Check if compiler exists
    if [ ! -f "./zig-out/bin/cursed" ]; then
        print_error "CURSED compiler not found. Please run 'zig build' first."
        exit 1
    fi
    
    # Create test program
    create_test_program
    
    # Test interpretation first
    if ! test_interpretation; then
        print_error "Test program is invalid. Aborting cross-compilation tests."
        exit 1
    fi
    
    echo
    print_info "Starting cross-compilation tests for all supported targets..."
    echo
    
    # Track results
    local total_tests=0
    local successful_tests=0
    local failed_targets=()
    
    # Test all supported targets
    declare -A targets=(
        ["native"]="Host Platform"
        ["linux-x64"]="Linux x86_64"
        ["linux-arm64"]="Linux ARM64"
        ["macos-x64"]="macOS x86_64"
        ["macos-arm64"]="macOS ARM64"
        ["windows-x64"]="Windows x86_64"
        ["wasm32"]="WebAssembly"
    )
    
    for target in "${!targets[@]}"; do
        description="${targets[$target]}"
        
        echo "----------------------------------------"
        total_tests=$((total_tests + 1))
        
        if test_target_compilation "$target" "$description"; then
            successful_tests=$((successful_tests + 1))
        else
            failed_targets+=("$target")
        fi
        
        echo
    done
    
    # Test summary
    echo "========================================"
    print_info "Cross-Compilation Test Summary"
    echo "  Total targets tested: $total_tests"
    echo "  Successful compilations: $successful_tests"
    echo "  Failed compilations: $((total_tests - successful_tests))"
    
    local success_rate=$((successful_tests * 100 / total_tests))
    echo "  Success rate: $success_rate%"
    
    if [ ${#failed_targets[@]} -gt 0 ]; then
        echo
        print_warning "Failed targets:"
        for target in "${failed_targets[@]}"; do
            echo "    - $target"
        done
    fi
    
    echo
    
    # Determine overall result
    if [ $success_rate -ge 80 ]; then
        print_success "Cross-compilation system is working well!"
        exit_code=0
    elif [ $success_rate -ge 50 ]; then
        print_warning "Cross-compilation system has some issues"
        exit_code=1
    else
        print_error "Cross-compilation system needs attention"
        exit_code=2
    fi
    
    # Cleanup test files
    print_info "Cleaning up test files..."
    rm -f "$TEST_PROGRAM"
    
    if [ "$VERBOSE" != "true" ]; then
        rm -rf "$BUILD_DIR"
    else
        print_info "Test artifacts preserved in: $BUILD_DIR"
    fi
    
    exit $exit_code
}

# Handle command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "OPTIONS:"
            echo "  -v, --verbose    Enable verbose output"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "ENVIRONMENT VARIABLES:"
            echo "  VERBOSE=true     Enable verbose output"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Run main function
main
