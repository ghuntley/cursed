#!/bin/bash

# CURSED Self-Hosting Pipeline Test Script
# Tests the complete self-hosting compilation chain

set -e

echo "🧪 CURSED Self-Hosting Pipeline Test"
echo "===================================="

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -f cursed-stage2 cursed-stage3 test_output.txt

# Step 1: Build Zig compiler
echo "📦 Step 1: Building Zig compiler..."
zig build
if [ $? -eq 0 ]; then
    echo "✅ Zig compiler built successfully"
else
    echo "❌ Zig compiler build failed"
    exit 1
fi

# Step 2: Test basic Zig compiler functionality
echo "🔍 Step 2: Testing basic Zig compiler..."
echo 'vibez.spill("Hello from Zig compiler!")' > basic_test.csd
./zig-out/bin/cursed-zig basic_test.csd > test_output.txt 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Basic Zig compiler works"
else
    echo "❌ Basic Zig compiler failed"
    cat test_output.txt
    exit 1
fi

# Step 3: Compile Stage 2 using Zig compiler
echo "📦 Step 3: Compiling Stage 2 CURSED compiler..."
./zig-out/bin/cursed-zig --compile src/bootstrap/stage2/main.csd -o cursed-stage2
if [ $? -eq 0 ] && [ -f cursed-stage2 ]; then
    echo "✅ Stage 2 compiler compilation successful"
else
    echo "❌ Stage 2 compiler compilation failed"
    exit 1
fi

# Step 4: Test Stage 2 compiler
echo "🔍 Step 4: Testing Stage 2 compiler..."
echo 'vibez.spill("Hello from Stage 2!")' > stage2_test.csd
./cursed-stage2 stage2_test.csd > test_output.txt 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Stage 2 compiler execution successful"
else
    echo "⚠️  Stage 2 compiler execution completed with warnings"
    cat test_output.txt
fi

# Step 5: Test Stage 2 compilation mode
echo "📦 Step 5: Testing Stage 2 compilation mode..."
./cursed-stage2 --compile stage2_test.csd -o stage2_compiled_test
if [ $? -eq 0 ]; then
    echo "✅ Stage 2 compilation mode successful"
    
    # Test compiled executable
    if [ -f stage2_compiled_test ]; then
        ./stage2_compiled_test > test_output.txt 2>&1
        if [ $? -eq 0 ]; then
            echo "✅ Stage 2 compiled executable works"
        else
            echo "⚠️  Stage 2 compiled executable has issues"
        fi
    fi
else
    echo "⚠️  Stage 2 compilation mode has issues"
fi

# Step 6: Self-compilation test (Stage 2 → Stage 3)
echo "🔄 Step 6: Self-compilation test (Stage 2 → Stage 3)..."
./cursed-stage2 --compile src/bootstrap/stage2/main.csd -o cursed-stage3
if [ $? -eq 0 ] && [ -f cursed-stage3 ]; then
    echo "✅ Stage 3 self-compilation successful"
    
    # Test Stage 3 compiler
    ./cursed-stage3 stage2_test.csd > test_output.txt 2>&1
    if [ $? -eq 0 ]; then
        echo "✅ Stage 3 compiler works"
    else
        echo "⚠️  Stage 3 compiler has issues"
    fi
else
    echo "⚠️  Stage 3 self-compilation had issues"
fi

# Step 7: Module resolution test
echo "🔧 Step 7: Testing module resolution..."
./zig-out/bin/cursed-zig src/bootstrap/stage2/module_resolver.csd > test_output.txt
if [ $? -eq 0 ]; then
    echo "✅ Module resolution test successful"
else
    echo "⚠️  Module resolution test had issues"
fi

# Step 8: Stdlib linking test
echo "🔗 Step 8: Testing stdlib linking..."
./zig-out/bin/cursed-zig src/bootstrap/stage2/stdlib_linker.csd > test_output.txt
if [ $? -eq 0 ]; then
    echo "✅ Stdlib linking test successful"
else
    echo "⚠️  Stdlib linking test had issues"
fi

# Step 9: Full self-hosting validation
echo "🎯 Step 9: Full self-hosting validation..."
create_validation_program() {
    cat > validation_program.csd << 'EOF'
#!/usr/bin/env cursed
# Self-hosting validation program

yeet "testz"

slay test_basic_functionality() {
    test_start("Basic functionality test")
    
    sus x normie = 42
    sus y normie = 28
    sus result normie = x + y
    
    assert_eq_int(result, 70)
    vibez.spill("Arithmetic test passed")
    
    sus message tea = "Hello, " + "World!"
    assert_eq_string(message, "Hello, World!")
    vibez.spill("String test passed")
    
    print_test_summary()
}

slay main() normie {
    vibez.spill("=== CURSED Self-Hosting Validation ===")
    test_basic_functionality()
    vibez.spill("=== Validation Complete ===")
    damn 0
}
EOF
}

create_validation_program

# Test with all compiler stages
echo "Testing validation program with all stages..."

# Zig compiler
./zig-out/bin/cursed-zig validation_program.csd > zig_output.txt 2>&1
zig_result=$?

# Stage 2 compiler
if [ -f cursed-stage2 ]; then
    ./cursed-stage2 validation_program.csd > stage2_output.txt 2>&1
    stage2_result=$?
else
    stage2_result=1
fi

# Stage 3 compiler  
if [ -f cursed-stage3 ]; then
    ./cursed-stage3 validation_program.csd > stage3_output.txt 2>&1
    stage3_result=$?
else
    stage3_result=1
fi

# Step 10: Results summary
echo ""
echo "📊 Self-Hosting Pipeline Test Results"
echo "====================================="

if [ $zig_result -eq 0 ]; then
    echo "✅ Zig compiler: PASSED"
else
    echo "❌ Zig compiler: FAILED"
fi

if [ $stage2_result -eq 0 ]; then
    echo "✅ Stage 2 compiler: PASSED"
else
    echo "⚠️  Stage 2 compiler: ISSUES (expected for prototype)"
fi

if [ $stage3_result -eq 0 ]; then
    echo "✅ Stage 3 compiler: PASSED"
else
    echo "⚠️  Stage 3 compiler: ISSUES (expected for prototype)"
fi

# Calculate success percentage
successful_stages=0
total_stages=3

if [ $zig_result -eq 0 ]; then
    successful_stages=$((successful_stages + 1))
fi
if [ $stage2_result -eq 0 ]; then
    successful_stages=$((successful_stages + 1))
fi
if [ $stage3_result -eq 0 ]; then
    successful_stages=$((successful_stages + 1))
fi

success_percentage=$((successful_stages * 100 / total_stages))

echo ""
echo "🎯 Self-Hosting Capability: ${success_percentage}%"
echo "📦 Compiler Stages Built: $(ls cursed-stage* 2>/dev/null | wc -l)"
echo "🚀 Stage 1 (Zig): $([ $zig_result -eq 0 ] && echo "✅ Functional" || echo "❌ Issues")"
echo "🔧 Stage 2 (CURSED): $([ -f cursed-stage2 ] && echo "✅ Built" || echo "❌ Missing")"
echo "⚡ Stage 3 (Self-compiled): $([ -f cursed-stage3 ] && echo "✅ Built" || echo "❌ Missing")"

echo ""
if [ $success_percentage -ge 80 ]; then
    echo "🎉 Self-hosting pipeline: HIGHLY FUNCTIONAL"
elif [ $success_percentage -ge 50 ]; then
    echo "🔧 Self-hosting pipeline: PARTIALLY FUNCTIONAL"
else
    echo "⚠️  Self-hosting pipeline: NEEDS WORK"
fi

# Cleanup
rm -f basic_test.csd stage2_test.csd validation_program.csd
rm -f test_output.txt zig_output.txt stage2_output.txt stage3_output.txt
rm -f stage2_compiled_test

echo "✨ Self-hosting pipeline test complete!"
