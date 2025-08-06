#!/usr/bin/env bash
# Enhanced CURSED Cross-Compilation Script
# Supports all 5 target platforms with comprehensive testing and validation

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/cross_compilation_results"
LOG_FILE="$BUILD_DIR/cross_compile.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

info() {
    echo -e "${CYAN}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

# Platform definitions with comprehensive metadata
declare -A PLATFORMS=(
    # Linux platforms
    ["linux-x64"]="x86_64-linux"
    ["linux-arm64"]="aarch64-linux"
    
    # macOS platforms  
    ["macos-x64"]="x86_64-macos"
    ["macos-arm64"]="aarch64-macos"
    
    # Windows platform
    ["windows-x64"]="x86_64-windows"
    
    # WebAssembly platform
    ["wasm32"]="wasm32-wasi"
)

declare -A PLATFORM_DESCRIPTIONS=(
    ["linux-x64"]="Linux x86_64"
    ["linux-arm64"]="Linux ARM64"
    ["macos-x64"]="macOS Intel"
    ["macos-arm64"]="macOS Apple Silicon"
    ["windows-x64"]="Windows x86_64"
    ["wasm32"]="WebAssembly"
)

declare -A PLATFORM_FEATURES=(
    ["linux-x64"]="threading,networking,filesystem,llvm"
    ["linux-arm64"]="threading,networking,filesystem,llvm"
    ["macos-x64"]="threading,networking,filesystem,llvm,frameworks"
    ["macos-arm64"]="threading,networking,filesystem,llvm,frameworks"
    ["windows-x64"]="threading,networking,filesystem,llvm,win32api"
    ["wasm32"]="limited"
)

declare -A EXPECTED_EXECUTABLES=(
    ["linux-x64"]="cursed-linux-x64"
    ["linux-arm64"]="cursed-linux-arm64"
    ["macos-x64"]="cursed-macos-x64"
    ["macos-arm64"]="cursed-macos-arm64"
    ["windows-x64"]="cursed-windows-x64.exe"
    ["wasm32"]="cursed-wasm32.wasm"
)

# Environment validation
validate_environment() {
    log "Validating cross-compilation environment..."
    
    # Check for required tools
    local required_tools=("zig" "git")
    local missing_tools=()
    
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -gt 0 ]; then
        error "Missing required tools: ${missing_tools[*]}"
        info "Please install the missing tools and try again"
        exit 1
    fi
    
    # Check Zig version
    local zig_version
    zig_version=$(zig version 2>/dev/null || echo "unknown")
    info "Zig version: $zig_version"
    
    # Verify we're in the right directory
    if [[ ! -f "$PROJECT_ROOT/build.zig" ]]; then
        error "build.zig not found. Please run from project root directory."
        exit 1
    fi
    
    # Check for platform abstraction layer
    if [[ ! -f "$PROJECT_ROOT/src-zig/platform_abstraction.zig" ]]; then
        warning "Platform abstraction layer not found - some features may be limited"
    fi
    
    success "Environment validation completed"
}

# Setup build environment
setup_build_environment() {
    log "Setting up build environment..."
    
    # Create build directory
    mkdir -p "$BUILD_DIR"
    
    # Initialize log file
    echo "CURSED Cross-Compilation Log - $(date)" > "$LOG_FILE"
    echo "=========================================" >> "$LOG_FILE"
    
    # Clean any previous builds
    if [[ -d "$PROJECT_ROOT/zig-out" ]]; then
        rm -rf "$PROJECT_ROOT/zig-out"
        info "Cleaned previous build artifacts"
    fi
    
    # Set up environment variables for cross-compilation
    export ZIG_GLOBAL_CACHE_DIR="$BUILD_DIR/zig-cache"
    mkdir -p "$ZIG_GLOBAL_CACHE_DIR"
    
    success "Build environment ready"
}

# Build specific platform
build_platform() {
    local platform="$1"
    local zig_target="${PLATFORMS[$platform]}"
    local description="${PLATFORM_DESCRIPTIONS[$platform]}"
    local features="${PLATFORM_FEATURES[$platform]}"
    
    log "Building $description ($platform) with target $zig_target"
    info "Platform features: $features"
    
    # Create platform-specific build directory
    local platform_build_dir="$BUILD_DIR/$platform"
    mkdir -p "$platform_build_dir"
    
    # Build with Zig
    local build_start_time=$(date +%s)
    
    if zig build -Dtarget="$zig_target" -Doptimize=ReleaseFast --verbose 2>&1 | tee "$platform_build_dir/build.log"; then
        local build_end_time=$(date +%s)
        local build_duration=$((build_end_time - build_start_time))
        
        success "✅ $description build completed in ${build_duration}s"
        
        # Verify binary was created
        verify_build_artifacts "$platform" "$platform_build_dir"
        
        # Copy artifacts to platform directory
        copy_build_artifacts "$platform" "$platform_build_dir"
        
        return 0
    else
        local build_end_time=$(date +%s)
        local build_duration=$((build_end_time - build_start_time))
        
        error "❌ $description build failed after ${build_duration}s"
        return 1
    fi
}

# Verify build artifacts
verify_build_artifacts() {
    local platform="$1"
    local platform_build_dir="$2"
    local expected_exe="${EXPECTED_EXECUTABLES[$platform]}"
    
    log "Verifying build artifacts for $platform..."
    
    # Look for the expected executable
    local exe_paths=(
        "$PROJECT_ROOT/zig-out/bin/$expected_exe"
        "$PROJECT_ROOT/zig-out/bin/cursed"
        "$PROJECT_ROOT/zig-out/bin/cursed.exe"
        "$PROJECT_ROOT/zig-out/bin/cursed.wasm"
    )
    
    local found_exe=""
    for exe_path in "${exe_paths[@]}"; do
        if [[ -f "$exe_path" ]]; then
            found_exe="$exe_path"
            break
        fi
    done
    
    if [[ -n "$found_exe" ]]; then
        local size
        size=$(du -h "$found_exe" | cut -f1)
        success "Binary found: $found_exe (size: $size)"
        
        # Get file information
        if command -v file >/dev/null 2>&1; then
            file "$found_exe" 2>/dev/null | tee -a "$platform_build_dir/binary_info.txt" || true
        fi
        
        # Record binary metadata
        echo "Platform: $platform" > "$platform_build_dir/metadata.txt"
        echo "Target: ${PLATFORMS[$platform]}" >> "$platform_build_dir/metadata.txt"
        echo "Binary: $found_exe" >> "$platform_build_dir/metadata.txt"
        echo "Size: $size" >> "$platform_build_dir/metadata.txt"
        echo "Build time: $(date)" >> "$platform_build_dir/metadata.txt"
        
        return 0
    else
        error "No binary found for $platform"
        info "Available files in zig-out/bin/:"
        find "$PROJECT_ROOT/zig-out/bin" -type f 2>/dev/null | head -10 || true
        return 1
    fi
}

# Copy build artifacts
copy_build_artifacts() {
    local platform="$1"
    local platform_build_dir="$2"
    
    log "Copying build artifacts for $platform..."
    
    # Copy all binaries from zig-out/bin
    if [[ -d "$PROJECT_ROOT/zig-out/bin" ]]; then
        cp -r "$PROJECT_ROOT/zig-out/bin"/* "$platform_build_dir/" 2>/dev/null || true
        success "Artifacts copied to $platform_build_dir"
    fi
    
    # Create archive for distribution
    create_platform_archive "$platform" "$platform_build_dir"
}

# Create platform-specific archive
create_platform_archive() {
    local platform="$1"
    local platform_build_dir="$2"
    
    log "Creating archive for $platform..."
    
    local archive_name
    if [[ "$platform" == "windows-x64" ]]; then
        archive_name="cursed-$platform.zip"
        
        # Create zip archive for Windows
        if command -v zip >/dev/null 2>&1; then
            (cd "$platform_build_dir" && zip -r "../$archive_name" . >/dev/null 2>&1)
        else
            # Fallback: tar.gz even for Windows
            archive_name="cursed-$platform.tar.gz"
            tar -czf "$BUILD_DIR/$archive_name" -C "$platform_build_dir" .
        fi
    else
        archive_name="cursed-$platform.tar.gz"
        tar -czf "$BUILD_DIR/$archive_name" -C "$platform_build_dir" .
    fi
    
    if [[ -f "$BUILD_DIR/$archive_name" ]]; then
        local archive_size
        archive_size=$(du -h "$BUILD_DIR/$archive_name" | cut -f1)
        success "Archive created: $archive_name ($archive_size)"
    else
        warning "Failed to create archive for $platform"
    fi
}

# Test built binary
test_binary() {
    local platform="$1"
    local platform_build_dir="$2"
    
    # Skip testing for cross-compiled binaries that can't run on host
    case "$platform" in
        "linux-arm64")
            if [[ "$(uname -m)" != "aarch64" ]]; then
                warning "Skipping execution test for ARM64 binary on $(uname -m) host"
                return 0
            fi
            ;;
        "macos-x64"|"macos-arm64")
            if [[ "$(uname)" != "Darwin" ]]; then
                warning "Skipping execution test for macOS binary on $(uname) host"
                return 0
            fi
            ;;
        "windows-x64")
            if [[ "$(uname)" != CYGWIN* && "$(uname)" != MINGW* ]]; then
                warning "Skipping execution test for Windows binary on $(uname) host"
                return 0
            fi
            ;;
        "wasm32")
            warning "WASM binaries require runtime environment - skipping execution test"
            return 0
            ;;
    esac
    
    log "Testing binary for $platform..."
    
    # Find the binary to test
    local test_binary=""
    for file in "$platform_build_dir"/*; do
        if [[ -x "$file" && ! -d "$file" ]]; then
            test_binary="$file"
            break
        fi
    done
    
    if [[ -z "$test_binary" ]]; then
        warning "No executable binary found for testing in $platform_build_dir"
        return 1
    fi
    
    # Create test CURSED program
    local test_program="$platform_build_dir/test.csd"
    echo 'vibez.spill("Cross-compilation test successful for '$platform'!")' > "$test_program"
    
    # Test version command
    if timeout 10s "$test_binary" --version >/dev/null 2>&1; then
        success "Version command test passed"
    else
        warning "Version command test failed"
    fi
    
    # Test program execution
    if timeout 10s "$test_binary" "$test_program" >/dev/null 2>&1; then
        success "Program execution test passed"
    else
        warning "Program execution test failed"
    fi
    
    # Cleanup test file
    rm -f "$test_program"
}

# Build all platforms
build_all_platforms() {
    local selected_platforms=("$@")
    
    if [[ ${#selected_platforms[@]} -eq 0 ]]; then
        selected_platforms=("${!PLATFORMS[@]}")
    fi
    
    log "Building ${#selected_platforms[@]} platforms: ${selected_platforms[*]}"
    
    local success_count=0
    local total_count=${#selected_platforms[@]}
    local build_results=()
    
    # Build each platform
    for platform in "${selected_platforms[@]}"; do
        echo
        log "========================================"
        log "Building platform: $platform"
        log "========================================"
        
        local platform_build_dir="$BUILD_DIR/$platform"
        
        if build_platform "$platform"; then
            ((success_count++))
            build_results+=("$platform:SUCCESS")
            
            # Test the binary if possible
            test_binary "$platform" "$platform_build_dir"
        else
            build_results+=("$platform:FAILED")
            error "Build failed for $platform"
        fi
    done
    
    # Summary
    echo
    log "========================================"
    log "Cross-compilation summary"
    log "========================================"
    
    for result in "${build_results[@]}"; do
        local platform="${result%%:*}"
        local status="${result##*:}"
        local description="${PLATFORM_DESCRIPTIONS[$platform]}"
        
        if [[ "$status" == "SUCCESS" ]]; then
            success "$description: ✅"
        else
            error "$description: ❌"
        fi
    done
    
    echo
    if [[ $success_count -eq $total_count ]]; then
        success "🎉 All $total_count platforms built successfully!"
        show_distribution_info
        return 0
    else
        warning "⚠️  $success_count/$total_count platforms built successfully"
        return 1
    fi
}

# Show distribution information
show_distribution_info() {
    log "========================================"
    log "Distribution packages created"
    log "========================================"
    
    if [[ -d "$BUILD_DIR" ]]; then
        for archive in "$BUILD_DIR"/*.{tar.gz,zip}; do
            if [[ -f "$archive" ]]; then
                local size
                size=$(du -h "$archive" | cut -f1)
                info "$(basename "$archive"): $size"
            fi
        done
    fi
    
    echo
    info "Build artifacts location: $BUILD_DIR"
    info "Build log: $LOG_FILE"
}

# Platform capability analysis
analyze_platform_capabilities() {
    log "========================================"
    log "Platform capability analysis"
    log "========================================"
    
    for platform in "${!PLATFORMS[@]}"; do
        local features="${PLATFORM_FEATURES[$platform]}"
        local description="${PLATFORM_DESCRIPTIONS[$platform]}"
        
        echo
        info "$description ($platform):"
        
        if [[ "$features" == "limited" ]]; then
            echo "  - Limited functionality (WASM constraints)"
        else
            IFS=',' read -ra feature_array <<< "$features"
            for feature in "${feature_array[@]}"; do
                case "$feature" in
                    "threading") echo "  - ✅ Threading support" ;;
                    "networking") echo "  - ✅ Networking support" ;;
                    "filesystem") echo "  - ✅ File system access" ;;
                    "llvm") echo "  - ✅ LLVM code generation" ;;
                    "frameworks") echo "  - ✅ macOS frameworks" ;;
                    "win32api") echo "  - ✅ Windows APIs" ;;
                esac
            done
        fi
    done
}

# Show help
show_help() {
    cat << EOF
Enhanced CURSED Cross-Compilation Script

Usage: $0 [OPTIONS] [PLATFORMS...]

Options:
    --help, -h          Show this help message
    --all, -a           Build all supported platforms (default)
    --analyze, -c       Analyze platform capabilities
    --test, -t          Build and test all platforms
    --verbose, -v       Enable verbose output
    --clean             Clean build directory before building

Supported platforms:
EOF

    for platform in "${!PLATFORMS[@]}"; do
        local target="${PLATFORMS[$platform]}"
        local description="${PLATFORM_DESCRIPTIONS[$platform]}"
        printf "    %-12s %s (%s)\n" "$platform" "$description" "$target"
    done

    cat << EOF

Examples:
    $0                          # Build all platforms
    $0 linux-x64 macos-arm64   # Build specific platforms
    $0 --analyze               # Show platform capabilities
    $0 --test                  # Build and test all platforms
    $0 --clean --all           # Clean and build all platforms

Environment variables:
    ZIG_GLOBAL_CACHE_DIR       Custom Zig cache directory
    CURSED_BUILD_VERBOSE       Enable verbose build output

Build artifacts will be placed in: $BUILD_DIR
EOF
}

# Main script logic
main() {
    local selected_platforms=()
    local run_tests=false
    local analyze_only=false
    local clean_first=false
    local verbose=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_help
                exit 0
                ;;
            --all|-a)
                selected_platforms=("${!PLATFORMS[@]}")
                shift
                ;;
            --test|-t)
                run_tests=true
                shift
                ;;
            --analyze|-c)
                analyze_only=true
                shift
                ;;
            --verbose|-v)
                verbose=true
                set -x
                shift
                ;;
            --clean)
                clean_first=true
                shift
                ;;
            *)
                # Check if it's a valid platform
                if [[ -n "${PLATFORMS[$1]:-}" ]]; then
                    selected_platforms+=("$1")
                else
                    error "Unknown platform: $1"
                    echo "Run '$0 --help' for available platforms"
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # Clean build directory if requested
    if [[ "$clean_first" == true ]]; then
        if [[ -d "$BUILD_DIR" ]]; then
            rm -rf "$BUILD_DIR"
            info "Cleaned build directory"
        fi
    fi
    
    # Analyze platform capabilities if requested
    if [[ "$analyze_only" == true ]]; then
        analyze_platform_capabilities
        exit 0
    fi
    
    # Default to all platforms if none specified
    if [[ ${#selected_platforms[@]} -eq 0 ]]; then
        selected_platforms=("${!PLATFORMS[@]}")
    fi
    
    # Start the build process
    log "Starting enhanced CURSED cross-compilation"
    log "Target platforms: ${selected_platforms[*]}"
    
    validate_environment
    setup_build_environment
    
    if build_all_platforms "${selected_platforms[@]}"; then
        success "Cross-compilation completed successfully"
        exit 0
    else
        error "Some platforms failed to build"
        exit 1
    fi
}

# Ensure we're in the project root
cd "$PROJECT_ROOT"

# Run main function
main "$@"
