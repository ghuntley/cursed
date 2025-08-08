#!/bin/bash

set -e

echo "Testing Windows cross-compilation for CURSED compiler..."

# Create test directory
mkdir -p windows_test_output

# Test Windows x64 cross-compilation
echo "Building Windows x64 binary..."
zig build-exe -target x86_64-windows -O ReleaseFast src-zig/windows_minimal_main.zig

# Check if binary was created and is valid
if [ -f "windows_minimal_main.exe" ]; then
    echo "✓ Windows x64 binary created successfully"
    
    # Verify file format
    file_info=$(file windows_minimal_main.exe)
    if [[ $file_info == *"PE32+ executable"* && $file_info == *"x86-64"* && $file_info == *"MS Windows"* ]]; then
        echo "✓ Windows binary format verified: $file_info"
    else
        echo "✗ Invalid Windows binary format: $file_info"
        exit 1
    fi
    
    # Move to test output
    mv windows_minimal_main.exe windows_test_output/cursed-windows-minimal.exe
    
    # Show file size
    ls -lh windows_test_output/cursed-windows-minimal.exe
    
else
    echo "✗ Windows x64 binary creation failed"
    exit 1
fi

# Test with different optimization levels
echo "Testing different optimization levels..."

# Debug build
zig build-exe -target x86_64-windows -O Debug src-zig/windows_minimal_main.zig
mv windows_minimal_main.exe windows_test_output/cursed-windows-debug.exe

# ReleaseSafe build
zig build-exe -target x86_64-windows -O ReleaseSafe src-zig/windows_minimal_main.zig
mv windows_minimal_main.exe windows_test_output/cursed-windows-safe.exe

# ReleaseSmall build
zig build-exe -target x86_64-windows -O ReleaseSmall src-zig/windows_minimal_main.zig
mv windows_minimal_main.exe windows_test_output/cursed-windows-small.exe

echo ""
echo "All Windows builds completed successfully:"
ls -lh windows_test_output/

echo ""
echo "File type verification:"
for exe in windows_test_output/*.exe; do
    echo "$(basename $exe): $(file $exe | cut -d: -f2)"
done

echo ""
echo "✓ Windows cross-compilation test completed successfully!"
echo "Generated binaries are ready for Windows deployment."
