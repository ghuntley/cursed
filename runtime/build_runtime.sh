#!/bin/bash
# Build script for CURSED runtime libraries with arm64 optimization
# This script ensures proper compilation for different architectures

set -e

# Detect architecture and OS
ARCH=$(uname -m)
OS=$(uname -s)

echo "Building CURSED runtime libraries for ${ARCH} on ${OS}..."

# Set architecture-specific compiler flags
case "${ARCH}" in
    "arm64"|"aarch64")
        if [[ "${OS}" == "Darwin" ]]; then
            # macOS arm64 (M1/M2/M3)
            CC=${CC:-clang}
            CFLAGS="-arch arm64 -mmacosx-version-min=11.0 -O2 -g -Wall -Wextra"
            LDFLAGS="-arch arm64"
            echo "Configuring for macOS arm64 (Apple Silicon)"
        else
            # Linux arm64
            CC=${CC:-gcc}
            CFLAGS="-march=armv8-a -O2 -g -Wall -Wextra -fPIC"
            LDFLAGS=""
            echo "Configuring for Linux arm64"
        fi
        ;;
    "x86_64")
        if [[ "${OS}" == "Darwin" ]]; then
            # macOS x86_64
            CC=${CC:-clang}
            CFLAGS="-arch x86_64 -mmacosx-version-min=10.12 -O2 -g -Wall -Wextra"
            LDFLAGS="-arch x86_64"
            echo "Configuring for macOS x86_64"
        else
            # Linux x86_64
            CC=${CC:-gcc}
            CFLAGS="-march=x86-64 -O2 -g -Wall -Wextra -fPIC"
            LDFLAGS=""
            echo "Configuring for Linux x86_64"
        fi
        ;;
    *)
        echo "Warning: Unknown architecture ${ARCH}, using default settings"
        CC=${CC:-gcc}
        CFLAGS="-O2 -g -Wall -Wextra"
        LDFLAGS=""
        ;;
esac

# Function to compile and archive a library
build_library() {
    local source_file="$1"
    local lib_name="$2"
    local obj_file="${source_file%.c}.o"
    local lib_file="lib${lib_name}.a"
    
    echo "Building ${lib_file} from ${source_file}..."
    
    # Compile source to object file
    ${CC} ${CFLAGS} -c "${source_file}" -o "${obj_file}"
    
    # Create static library
    ar rcs "${lib_file}" "${obj_file}"
    
    # Index the archive with ranlib to fix linking issues
    ranlib "${lib_file}"
    
    # Verify architecture
    echo "Verifying architecture of ${lib_file}:"
    if command -v lipo &> /dev/null; then
        lipo -info "${lib_file}"
    elif command -v file &> /dev/null; then
        file "${lib_file}"
    fi
    
    echo "✅ Successfully built ${lib_file}"
}

# Clean old builds
echo "Cleaning old builds..."
rm -f *.o *.a

# Build runtime libraries
build_library "minimal_shims.c" "cursed_minimal_shims"
build_library "interface_runtime.c" "cursed_interface_runtime"
build_library "type_assertion_runtime.c" "cursed_type_assertion_runtime"
build_library "memory_runtime.c" "cursed_memory_runtime"

# Set library search paths for the linker
echo ""
echo "Runtime libraries built successfully!"
echo "Architecture: ${ARCH}"
echo "Compiler: ${CC}"
echo "Flags: ${CFLAGS}"

# Verify all libraries exist
echo ""
echo "Built libraries:"
ls -la *.a

echo ""
echo "✅ Runtime library build complete!"
