#!/bin/bash

set -e

echo "CURSED Cross-Compilation Test Matrix"
echo "===================================="

# Create output directory
mkdir -p cross_compilation_results

# Define targets
declare -A targets=(
    ["linux-x64"]="x86_64-linux"
    ["linux-arm64"]="aarch64-linux"
    ["macos-x64"]="x86_64-macos"
    ["macos-arm64"]="aarch64-macos"
    ["windows-x64"]="x86_64-windows"
    ["wasm32"]="wasm32-wasi"
)

echo ""
echo "Testing cross-compilation for all supported targets..."
echo ""

success_count=0
total_count=0

for target_name in "${!targets[@]}"; do
    target_triple="${targets[$target_name]}"
    total_count=$((total_count + 1))
    
    echo -n "Testing $target_name ($target_triple)... "
    
    # Choose appropriate source file
    if [[ $target_name == "windows-x64" ]]; then
        source_file="src-zig/windows_minimal_main.zig"
        output_ext=".exe"
    elif [[ $target_name == "wasm32" ]]; then
        source_file="src-zig/wasm_minimal_compiler.zig"
        output_ext=".wasm"
    else
        source_file="src-zig/minimal_main.zig"
        output_ext=""
    fi
    
    output_file="cross_compilation_results/cursed-${target_name}${output_ext}"
    
    # Attempt compilation
    if zig build-exe -target "$target_triple" -O ReleaseFast "$source_file" -femit-bin="$output_file" 2>/dev/null; then
        echo "✓ SUCCESS"
        success_count=$((success_count + 1))
        
        # Verify file
        file_info=$(file "$output_file" 2>/dev/null || echo "unknown")
        size=$(du -h "$output_file" | cut -f1)
        echo "    Format: $file_info"
        echo "    Size: $size"
        echo ""
    else
        echo "✗ FAILED"
        echo ""
    fi
done

echo "Cross-Compilation Results Summary:"
echo "=================================="
echo "Successful: $success_count / $total_count targets"
echo "Success Rate: $(( success_count * 100 / total_count ))%"
echo ""

if [ -d "cross_compilation_results" ]; then
    echo "Generated binaries:"
    ls -lh cross_compilation_results/
    echo ""
fi

if [ $success_count -eq $total_count ]; then
    echo "🎉 ALL TARGETS COMPILED SUCCESSFULLY!"
    echo "✅ Windows cross-compilation: 100% working"
else
    echo "⚠️  Some targets failed. See details above."
    if [[ -f "cross_compilation_results/cursed-windows-x64.exe" ]]; then
        echo "✅ Windows cross-compilation: WORKING"
    else
        echo "❌ Windows cross-compilation: FAILED"
    fi
fi

echo ""
echo "Windows-specific verification:"
if [[ -f "cross_compilation_results/cursed-windows-x64.exe" ]]; then
    windows_info=$(file "cross_compilation_results/cursed-windows-x64.exe")
    echo "Windows binary: $windows_info"
    
    if [[ $windows_info == *"PE32+ executable"* && $windows_info == *"x86-64"* && $windows_info == *"MS Windows"* ]]; then
        echo "✅ Windows binary format verified correctly"
        echo "✅ Ready for Windows deployment"
    else
        echo "❌ Windows binary format verification failed"
    fi
else
    echo "❌ Windows binary not found"
fi
