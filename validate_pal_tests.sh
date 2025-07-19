#!/bin/bash
# validate_pal_tests.sh - Validates PAL test files without requiring compilation

echo "📋 CURSED PAL Test Validation"
echo "============================="

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
TOTAL_TESTS=0
VALID_TESTS=0
INVALID_TESTS=0

# Function to validate test file
validate_test_file() {
    local file=$1
    local name=$(basename "$file")
    
    echo -n "Validating $name... "
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ ! -f "$file" ]; then
        echo -e "${RED}❌ File not found${NC}"
        INVALID_TESTS=$((INVALID_TESTS + 1))
        return 1
    fi
    
    # Check for testz framework integration
    if ! grep -q 'yeet "testz"' "$file"; then
        echo -e "${YELLOW}⚠️  Missing testz import${NC}"
        INVALID_TESTS=$((INVALID_TESTS + 1))
        return 1
    fi
    
    if ! grep -q "test_start" "$file"; then
        echo -e "${YELLOW}⚠️  Missing test_start${NC}"
        INVALID_TESTS=$((INVALID_TESTS + 1))
        return 1
    fi
    
    if ! grep -q "print_test_summary" "$file"; then
        echo -e "${YELLOW}⚠️  Missing print_test_summary${NC}"
        INVALID_TESTS=$((INVALID_TESTS + 1))
        return 1
    fi
    
    # Check for basic CURSED syntax
    if ! grep -q -E "(sus|periodt|stan|vibez|assert_|damn)" "$file"; then
        echo -e "${YELLOW}⚠️  No CURSED syntax detected${NC}"
        INVALID_TESTS=$((INVALID_TESTS + 1))
        return 1
    fi
    
    echo -e "${GREEN}✅ Valid${NC}"
    VALID_TESTS=$((VALID_TESTS + 1))
    return 0
}

# Function to validate shell script
validate_shell_script() {
    local file=$1
    local name=$(basename "$file")
    
    echo -n "Validating $name... "
    
    if [ ! -f "$file" ]; then
        echo -e "${RED}❌ File not found${NC}"
        return 1
    fi
    
    if [ ! -x "$file" ]; then
        echo -e "${YELLOW}⚠️  Not executable${NC}"
        chmod +x "$file"
        echo -e "${GREEN}✅ Made executable${NC}"
    else
        echo -e "${GREEN}✅ Valid and executable${NC}"
    fi
    
    return 0
}

echo "🧪 Validating PAL test files..."
echo "==============================="

# PAL test files to validate
PAL_TESTS=(
    "test_feature_detection.csd"
    "test_memory_pal.csd"
    "test_memory_alignment.csd"
    "test_large_pages.csd"
    "test_scheduler_pal.csd"
    "test_apple_silicon_cores.csd"
    "test_numa_scheduling.csd"
    "test_wasm_memory.csd"
    "test_wasm_scheduling.csd"
    "test_simd_features.csd"
    "test_crypto_acceleration.csd"
    "test_memory_stress.csd"
    "test_scheduler_stress.csd"
    "benchmark_pal_performance.csd"
)

for test in "${PAL_TESTS[@]}"; do
    validate_test_file "$test"
done

echo ""
echo "🔧 Validating shell scripts..."
echo "=============================="

SHELL_SCRIPTS=(
    "run_pal_tests.sh"
    "test_cross_compilation.sh"
    "create_platform_benchmark.sh"
    "validate_pal_integration.sh"
    "run_comprehensive_pal_tests.sh"
    "validate_pal_tests.sh"
)

for script in "${SHELL_SCRIPTS[@]}"; do
    validate_shell_script "$script"
done

echo ""
echo "🐍 Validating Python scripts..."
echo "==============================="

if [ -f "analyze_pal_results.py" ]; then
    echo -n "Validating analyze_pal_results.py... "
    if python3 -m py_compile analyze_pal_results.py 2>/dev/null; then
        echo -e "${GREEN}✅ Valid Python syntax${NC}"
    else
        echo -e "${YELLOW}⚠️  Python syntax issues${NC}"
    fi
else
    echo -e "${RED}❌ analyze_pal_results.py not found${NC}"
fi

echo ""
echo "📊 Validation Summary"
echo "===================="
echo "Total PAL test files: $TOTAL_TESTS"
echo -e "Valid tests: ${GREEN}$VALID_TESTS${NC}"
echo -e "Invalid tests: ${RED}$INVALID_TESTS${NC}"

# Check platform-specific requirements
echo ""
echo "🖥️  Platform-Specific Validation"
echo "================================="

# Detect current platform
ARCH=$(uname -m)
OS=$(uname -s)

echo "Detected platform: $ARCH $OS"

# Check for WebAssembly support
if command -v wasmtime &> /dev/null; then
    echo -e "${GREEN}✅ WebAssembly runtime (wasmtime) available${NC}"
else
    echo -e "${YELLOW}⚠️  WebAssembly runtime not available${NC}"
    echo "   Install with: cargo install wasmtime-cli"
fi

# Check for Python 3
if command -v python3 &> /dev/null; then
    echo -e "${GREEN}✅ Python 3 available${NC}"
else
    echo -e "${RED}❌ Python 3 not available${NC}"
fi

# Check if testz framework exists
if [ -f "stdlib/testz/mod.csd" ]; then
    echo -e "${GREEN}✅ CURSED testz framework found${NC}"
else
    echo -e "${RED}❌ CURSED testz framework missing${NC}"
    echo "   Required for PAL tests to run"
fi

echo ""
echo "🎯 Test Framework Validation"
echo "============================"

# Create a simple test validation
cat > test_validation_check.csd << 'EOF'
yeet "testz"

test_start("PAL Test Framework Validation")

// Basic functionality test
sus test_value drip = 42
assert_eq_int(test_value, 42)
assert_true(test_value > 0)

vibez.spill("PAL test framework validation successful")

print_test_summary()
EOF

echo "Created test_validation_check.csd for framework testing"

echo ""
echo "📋 Recommended Next Steps"
echo "========================"

if [ $INVALID_TESTS -eq 0 ]; then
    echo -e "${GREEN}✅ All PAL tests are properly formatted${NC}"
    echo "✅ Ready to run PAL test suite"
    echo "🚀 Execute with: ./run_comprehensive_pal_tests.sh"
else
    echo -e "${YELLOW}⚠️  Some tests need attention before running${NC}"
    echo "📝 Fix test formatting issues above"
    echo "🔧 Then run: ./validate_pal_tests.sh"
fi

echo ""
echo "🔗 PAL Test Resources"
echo "===================="
echo "📖 Full documentation: PAL_TESTING_README.md"
echo "📋 Test specification: TEST_PAL.md"
echo "🧪 Test framework: stdlib/testz/mod.csd"

# Exit with appropriate code
if [ $INVALID_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi
