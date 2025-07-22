#!/usr/bin/env bash
# CURSED Cross-Compilation Script with Fixed Target Support
# Only builds targets that are actually supported on Linux

set -euo pipefail

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

# Define supported targets on Linux host
declare -A TARGETS=(
    ["x86_64-unknown-linux-gnu"]="Linux x86_64 (native)"
    ["aarch64-unknown-linux-gnu"]="Linux ARM64" 
    ["x86_64-pc-windows-gnu"]="Windows x86_64"
    ["wasm32-unknown-unknown"]="WebAssembly"
)

# Define unsupported targets with reasons
declare -A UNSUPPORTED_TARGETS=(
    ["x86_64-apple-darwin"]="Requires macOS SDK - not available on Linux"
    ["aarch64-apple-darwin"]="Requires macOS SDK - not available on Linux"
)

# Function to check if target is supported
is_supported_target() {
    local target="$1"
    [[ -n "${TARGETS[$target]:-}" ]]
}

# Function to build specific target
build_target() {
    local target="$1"
    local description="${TARGETS[$target]}"
    
    print_status "Building for $description ($target)..."
    
    # Skip LLVM dependency for WASM
    local cargo_flags=""
    if [[ "$target" == "wasm32-unknown-unknown" ]]; then
        cargo_flags="--no-default-features --features wasm-compatible"
        print_warning "Building WASM target with limited features (no LLVM backend)"
    fi
    
    # Special handling for Windows to check pthread libraries
    if [[ "$target" == "x86_64-pc-windows-gnu" ]]; then
        if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
            print_error "MinGW-w64 compiler not found for Windows cross-compilation"
            return 1
        fi
        
        # Check for pthread libraries
        if ! x86_64-w64-mingw32-gcc -print-file-name=libpthread.a | grep -q libpthread.a; then
            print_warning "Windows pthread library may not be properly linked"
        fi
    fi
    
    # Special handling for ARM64 Linux
    if [[ "$target" == "aarch64-unknown-linux-gnu" ]]; then
        if ! command -v aarch64-unknown-linux-gnu-gcc >/dev/null 2>&1; then
            print_error "ARM64 cross-compiler not found"
            return 1
        fi
    fi
    
    # Use timeout to prevent hanging builds
    if timeout 300 cargo build --target "$target" --release $cargo_flags; then
        print_success "✅ $description build completed"
        
        # Verify the binary was created
        local binary_path="target/$target/release/cursed"
        if [[ "$target" == "x86_64-pc-windows-gnu" ]]; then
            binary_path="target/$target/release/cursed.exe"
        elif [[ "$target" == "wasm32-unknown-unknown" ]]; then
            binary_path="target/$target/release/cursed.wasm"
        fi
        
        if [[ -f "$binary_path" ]]; then
            local size=$(du -h "$binary_path" | cut -f1)
            print_status "Binary size: $size ($binary_path)"
        else
            print_warning "Binary not found at expected location: $binary_path"
        fi
        
        return 0
    else
        print_error "❌ $description build failed or timed out"
        return 1
    fi
}

# Function to validate build environment
validate_environment() {
    print_status "Validating cross-compilation environment..."
    
    # Check for Rust toolchain
    if ! command -v cargo >/dev/null 2>&1; then
        print_error "Cargo not found. Please install Rust toolchain."
        exit 1
    fi
    
    # Check if Rust toolchain is available
    local rust_version=$(rustc --version 2>/dev/null || echo "unknown")
    print_status "Rust toolchain: $rust_version"
    
    # In Nix environments with fenix, targets are pre-installed
    # Skip target installation check since rustup is not available
    if command -v rustup >/dev/null 2>&1; then
        # Check for required cross-compilation targets (only if rustup is available)
        for target in "${!TARGETS[@]}"; do
            if [[ "$target" != "x86_64-unknown-linux-gnu" ]]; then
                if ! rustup target list --installed | grep -q "$target"; then
                    print_warning "Target $target not installed. Installing..."
                    if ! rustup target add "$target"; then
                        print_error "Failed to install target $target"
                        exit 1
                    fi
                fi
            fi
        done
    else
        print_warning "rustup not available - assuming targets are pre-installed (Nix/fenix environment)"
    fi
    
    print_success "Environment validation completed"
}

# Function to show help
show_help() {
    echo "CURSED Cross-Compilation Script"
    echo "Usage: $0 [target|--all|--help]"
    echo
    echo "Supported targets:"
    for target in "${!TARGETS[@]}"; do
        echo "  $target - ${TARGETS[$target]}"
    done
    echo
    echo "Unsupported targets (will be skipped):"
    for target in "${!UNSUPPORTED_TARGETS[@]}"; do
        echo "  $target - ${UNSUPPORTED_TARGETS[$target]}"
    done
    echo
    echo "Options:"
    echo "  --all     Build all supported targets"
    echo "  --help    Show this help message"
}

# Function to build all supported targets
build_all_targets() {
    local success_count=0
    local total_count=${#TARGETS[@]}
    
    print_status "Building all $total_count supported targets..."
    
    for target in "${!TARGETS[@]}"; do
        if build_target "$target"; then
            ((success_count++))
        fi
        echo  # Add spacing between targets
    done
    
    echo
    print_status "Cross-compilation summary:"
    print_success "$success_count/$total_count targets built successfully"
    
    if [[ $success_count -eq $total_count ]]; then
        print_success "🎉 All supported targets built successfully!"
        exit 0
    else
        print_warning "⚠️  Some targets failed to build"
        exit 1
    fi
}

# Main script logic
main() {
    # Ensure we're in the project root
    if [[ ! -f "Cargo.toml" ]]; then
        print_error "Must be run from the project root directory"
        exit 1
    fi
    
    case "${1:-}" in
        --help|-h)
            show_help
            exit 0
            ;;
        --all|-a)
            validate_environment
            build_all_targets
            ;;
        "")
            validate_environment
            build_all_targets
            ;;
        *)
            local target="$1"
            if is_supported_target "$target"; then
                validate_environment
                build_target "$target"
            elif [[ -n "${UNSUPPORTED_TARGETS[$target]:-}" ]]; then
                print_error "Target '$target' is not supported: ${UNSUPPORTED_TARGETS[$target]}"
                exit 1
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
