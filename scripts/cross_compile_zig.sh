#!/usr/bin/env bash
# CURSED Zig Cross-Compilation Script
# Builds for all supported platforms using Zig's superior cross-compilation

set -euo pipefail
# Note: pipefail may cause issues with some commands - monitor for early exits

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Define supported targets with Zig cross-compilation
declare -A TARGETS=(
    ["x86_64-linux"]="Linux x86_64"
    ["aarch64-linux"]="Linux ARM64" 
    ["x86_64-windows"]="Windows x86_64"
    ["wasm32-wasi"]="WebAssembly WASI"
)

# Function to build specific target
build_target() {
    local target="$1"
    local description="${TARGETS[$target]}"
    
    print_status "Building CURSED compiler for $description ($target)..."
    
    # Clean previous build for this target
    rm -rf zig-out
    
    # Build with Zig cross-compilation
    if zig build -Dtarget="$target" -Doptimize=ReleaseFast; then
        print_success "✅ $description build completed"
        
        # Verify the binary was created and show info
        local binary_patterns=()
        if [[ "$target" == *"windows"* ]]; then
            binary_patterns=("zig-out/bin/cursed-zig.exe")
        elif [[ "$target" == *"wasm"* ]]; then
            binary_patterns=("zig-out/bin/cursed-zig.wasm" "zig-out/bin/cursed-zig")
        else
            binary_patterns=("zig-out/bin/cursed-zig")
        fi
        
        for pattern in "${binary_patterns[@]}"; do
            if [[ -f "$pattern" ]]; then
                local size=$(du -h "$pattern" | cut -f1)
                print_status "Binary: $pattern (size: $size)"
                
                # File type information
                if command -v file >/dev/null 2>&1; then
                    file "$pattern" 2>/dev/null | sed 's/^/  /' || true
                fi
                break
            fi
        done
        
        return 0
    else
        print_error "❌ $description build failed"
        return 1
    fi
}

# Function to test built binary with simple program
test_binary() {
    local target="$1"
    
    if [[ "$target" == *"wasm"* ]]; then
        print_warning "WASM binaries require runtime environment - skipping execution test"
        return 0
    fi
    
    if [[ "$target" == "x86_64-windows" ]]; then
        print_warning "Windows binaries cannot be tested on Linux - skipping execution test"
        return 0
    fi
    
    local binary="zig-out/bin/cursed-zig"
    if [[ ! -f "$binary" ]]; then
        print_warning "Binary not found for testing: $binary"
        return 0
    fi
    
    # Test with simple CURSED program
    echo 'vibez.spill("Cross-compilation test successful!")' > test_cross_compile.💀
    
    if [[ "$target" == "aarch64-linux" ]]; then
        # ARM64 binary on x86_64 - can't execute directly
        print_warning "ARM64 binary on x86_64 host - skipping execution test"
    else
        # x86_64-linux should work
        if "$binary" test_cross_compile.💀 >/dev/null 2>&1; then
            print_success "✅ Binary execution test passed"
        else
            print_warning "⚠️  Binary execution test failed"
        fi
    fi
    
    rm -f test_cross_compile.💀
}

# Function to show help
show_help() {
    echo "CURSED Zig Cross-Compilation Script"
    echo "Usage: $0 [target|--all|--help]"
    echo
    echo "Supported targets:"
    for target in "${!TARGETS[@]}"; do
        echo "  $target - ${TARGETS[$target]}"
    done
    echo
    echo "Options:"
    echo "  --all     Build all supported targets"
    echo "  --test    Build and test all targets"
    echo "  --help    Show this help message"
}

# Function to build all supported targets
build_all_targets() {
    local test_mode="${1:-false}"
    local success_count=0
    local total_count=${#TARGETS[@]}
    
    print_status "Building all $total_count supported targets with Zig cross-compilation..."
    
    # Use explicit list to avoid bash associative array iteration issues
    local target_list=("x86_64-linux" "aarch64-linux" "x86_64-windows" "wasm32-wasi")
    
    for target in "${target_list[@]}"; do
        echo
        print_status "Processing target: $target"
        
        # Use subshell to prevent script exit on error
        if (build_target "$target"); then
            ((success_count++))
            print_status "Target $target successful, count: $success_count"
            
            if [[ "$test_mode" == "true" ]]; then
                test_binary "$target" || true
            fi
        else
            print_error "Failed to build target: $target"
        fi
        echo  # Add spacing between targets
    done
    
    echo
    print_status "Cross-compilation summary:"
    print_success "$success_count/$total_count targets built successfully"
    
    if [[ $success_count -eq $total_count ]]; then
        print_success "🎉 All supported targets built successfully!"
        return 0
    else
        print_warning "⚠️  Some targets failed to build"
        return 1
    fi
}

# Function to validate environment
validate_environment() {
    print_status "Validating Zig cross-compilation environment..."
    
    # Check for Zig
    if ! command -v zig >/dev/null 2>&1; then
        print_error "Zig not found. Please install Zig toolchain."
        exit 1
    fi
    
    # Check Zig version
    local zig_version=$(zig version 2>/dev/null || echo "unknown")
    print_status "Zig version: $zig_version"
    
    # Check if we're in the right directory
    if [[ ! -f "build.zig" ]]; then
        print_error "build.zig not found. Please run from project root directory."
        exit 1
    fi
    
    print_success "Environment validation completed"
}

# Main script logic
main() {
    case "${1:-}" in
        --help|-h)
            show_help
            exit 0
            ;;
        --all|-a)
            validate_environment
            if build_all_targets false; then
                exit 0
            else
                exit 1
            fi
            ;;
        --test|-t)
            validate_environment
            if build_all_targets true; then
                exit 0
            else
                exit 1
            fi
            ;;
        "")
            validate_environment
            if build_all_targets false; then
                exit 0
            else
                exit 1
            fi
            ;;
        *)
            local target="$1"
            if [[ -n "${TARGETS[$target]:-}" ]]; then
                validate_environment
                build_target "$target"
                test_binary "$target"
            else
                print_error "Unknown target: $target"
                show_help
                exit 1
            fi
            ;;
    esac
}

# Run the main function
main "$@"
