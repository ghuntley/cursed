#!/bin/bash
# Lexer, Parser, and Code Generation Component Tests
# Tests individual compiler pipeline components in isolation

set -e

echo "🔬 Lexer, Parser, and Code Generation Component Tests"
echo "===================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test counters
COMPONENT_TESTS=0
COMPONENT_PASSED=0
COMPONENT_FAILED=0

# Helper function for component testing
test_component() {
    local component="$1"
    local test_name="$2"
    local test_input="$3"
    local expected_pattern="$4"
    
    COMPONENT_TESTS=$((COMPONENT_TESTS + 1))
    echo -e "${BLUE}🔍 Testing $component: $test_name${NC}"
    
    case "$component" in
        "lexer")
            # Test lexer by checking token generation
            echo "$test_input" > /tmp/lexer_test.csd
            if cargo run --bin cursed -- --lex /tmp/lexer_test.csd > /tmp/lexer_output.txt 2>&1; then
                if grep -q "$expected_pattern" /tmp/lexer_output.txt; then
                    echo -e "  ${GREEN}✅ Lexer test passed${NC}"
                    COMPONENT_PASSED=$((COMPONENT_PASSED + 1))
                else
                    echo -e "  ${RED}❌ Lexer test failed - pattern not found${NC}"
                    echo "    Expected pattern: $expected_pattern"
                    echo "    Got: $(cat /tmp/lexer_output.txt | head -n 3)"
                    COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
                fi
            else
                echo -e "  ${RED}❌ Lexer execution failed${NC}"
                cat /tmp/lexer_output.txt | head -n 5
                COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
            fi
            ;;
            
        "parser")
            # Test parser by checking AST generation
            echo "$test_input" > /tmp/parser_test.csd
            if cargo run --bin cursed -- --parse /tmp/parser_test.csd > /tmp/parser_output.txt 2>&1; then
                if grep -q "$expected_pattern" /tmp/parser_output.txt; then
                    echo -e "  ${GREEN}✅ Parser test passed${NC}"
                    COMPONENT_PASSED=$((COMPONENT_PASSED + 1))
                else
                    echo -e "  ${RED}❌ Parser test failed - pattern not found${NC}"
                    echo "    Expected pattern: $expected_pattern"
                    echo "    Got: $(cat /tmp/parser_output.txt | head -n 3)"
                    COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
                fi
            else
                echo -e "  ${RED}❌ Parser execution failed${NC}"
                cat /tmp/parser_output.txt | head -n 5
                COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
            fi
            ;;
            
        "codegen")
            # Test code generation by checking LLVM IR
            echo "$test_input" > /tmp/codegen_test.csd
            if cargo run --bin cursed -- --emit-llvm /tmp/codegen_test.csd > /tmp/codegen_output.txt 2>&1; then
                if [[ -f "codegen_test.ll" ]] && grep -q "$expected_pattern" codegen_test.ll; then
                    echo -e "  ${GREEN}✅ Codegen test passed${NC}"
                    COMPONENT_PASSED=$((COMPONENT_PASSED + 1))
                    rm -f codegen_test.ll
                else
                    echo -e "  ${RED}❌ Codegen test failed - pattern not found in IR${NC}"
                    echo "    Expected pattern: $expected_pattern"
                    if [[ -f "codegen_test.ll" ]]; then
                        echo "    Got: $(cat codegen_test.ll | head -n 3)"
                        rm -f codegen_test.ll
                    else
                        echo "    No LLVM IR file generated"
                    fi
                    COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
                fi
            else
                echo -e "  ${RED}❌ Codegen execution failed${NC}"
                cat /tmp/codegen_output.txt | head -n 5
                COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
            fi
            ;;
    esac
    echo
}

# Test error handling for malformed input
test_error_handling() {
    local component="$1"
    local test_name="$2"
    local malformed_input="$3"
    
    COMPONENT_TESTS=$((COMPONENT_TESTS + 1))
    echo -e "${BLUE}🔍 Testing $component Error Handling: $test_name${NC}"
    
    echo "$malformed_input" > /tmp/error_test.csd
    
    case "$component" in
        "lexer")
            if cargo run --bin cursed -- --lex /tmp/error_test.csd > /tmp/error_output.txt 2>&1; then
                echo -e "  ${RED}❌ Should have failed but didn't${NC}"
                COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
            else
                echo -e "  ${GREEN}✅ Correctly detected error${NC}"
                COMPONENT_PASSED=$((COMPONENT_PASSED + 1))
            fi
            ;;
        "parser")
            if cargo run --bin cursed -- --parse /tmp/error_test.csd > /tmp/error_output.txt 2>&1; then
                echo -e "  ${RED}❌ Should have failed but didn't${NC}"
                COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
            else
                echo -e "  ${GREEN}✅ Correctly detected error${NC}"
                COMPONENT_PASSED=$((COMPONENT_PASSED + 1))
            fi
            ;;
        "codegen")
            if cargo run --bin cursed -- --emit-llvm /tmp/error_test.csd > /tmp/error_output.txt 2>&1; then
                echo -e "  ${RED}❌ Should have failed but didn't${NC}"
                COMPONENT_FAILED=$((COMPONENT_FAILED + 1))
            else
                echo -e "  ${GREEN}✅ Correctly detected error${NC}"
                COMPONENT_PASSED=$((COMPONENT_PASSED + 1))
            fi
            ;;
    esac
    echo
}

echo "🔨 Building CURSED compiler..."
if ! cargo build --release > /tmp/build_output.txt 2>&1; then
    echo -e "${RED}❌ Build failed!${NC}"
    cat /tmp/build_output.txt
    exit 1
fi
echo -e "${GREEN}✅ Build successful${NC}"
echo

# Check if compiler supports individual component testing
echo "🔍 Checking component testing support..."
if cargo run --bin cursed -- --help 2>&1 | grep -q "\-\-lex\|--parse\|--emit-llvm"; then
    echo -e "${GREEN}✅ Component testing flags available${NC}"
else
    echo -e "${YELLOW}⚠️  Component testing flags not available, using alternative methods${NC}"
    # Alternative: use verbose compilation and check intermediate outputs
fi
echo

echo "📋 Running Lexer Component Tests..."
echo "=================================="

# Lexer Test 1: Basic tokens
test_component "lexer" "Basic Tokens" \
    "sus x drip = 42" \
    "IDENTIFIER\|INTEGER\|ASSIGN"

# Lexer Test 2: String literals
test_component "lexer" "String Literals" \
    'vibez.spill("Hello, World!")' \
    "STRING"

# Lexer Test 3: Keywords
test_component "lexer" "Keywords" \
    "slay main() { damn based }" \
    "SLAY\|DAMN\|BASED"

# Lexer Test 4: Comments
test_component "lexer" "Comments" \
    "fr fr This is a comment" \
    "COMMENT"

# Lexer Test 5: Operators
test_component "lexer" "Operators" \
    "sus result drip = a + b * c" \
    "PLUS\|MULTIPLY\|ASSIGN"

echo "📋 Running Parser Component Tests..."
echo "==================================="

# Parser Test 1: Variable declarations
test_component "parser" "Variable Declaration" \
    "sus name tea = \"CURSED\"" \
    "VariableDeclaration\|VarDecl"

# Parser Test 2: Function definitions
test_component "parser" "Function Definition" \
    "slay add(a drip, b drip) drip { damn a + b }" \
    "FunctionDeclaration\|FuncDecl"

# Parser Test 3: Struct definitions
test_component "parser" "Struct Definition" \
    "squad Point { spill x drip; spill y drip }" \
    "StructDeclaration\|StructDecl"

# Parser Test 4: Control flow
test_component "parser" "Control Flow" \
    "lowkey condition { vibez.spill(\"true\") }" \
    "IfStatement\|ConditionalStmt"

# Parser Test 5: Expressions
test_component "parser" "Expressions" \
    "sus result drip = (a + b) * c" \
    "BinaryExpression\|Expression"

echo "📋 Running Code Generation Tests..."
echo "=================================="

# Codegen Test 1: Variable allocation
test_component "codegen" "Variable Allocation" \
    "sus x drip = 42; vibez.spill(x)" \
    "alloca\|store\|load"

# Codegen Test 2: Function calls
test_component "codegen" "Function Calls" \
    "vibez.spill(\"Hello\")" \
    "call.*vibez_spill"

# Codegen Test 3: Arithmetic operations
test_component "codegen" "Arithmetic Operations" \
    "sus result drip = 5 + 3" \
    "add.*i32\|add.*i64"

# Codegen Test 4: Control flow
test_component "codegen" "Control Flow" \
    "lowkey true { vibez.spill(\"yes\") }" \
    "br.*label\|icmp"

# Codegen Test 5: Memory management
test_component "codegen" "Memory Management" \
    "sus array []drip = [1, 2, 3]" \
    "malloc\|alloca.*array"

echo "📋 Running Error Handling Tests..."
echo "================================="

# Error Test 1: Lexer errors
test_error_handling "lexer" "Invalid Characters" \
    "sus x @ = 42"

# Error Test 2: Parser errors
test_error_handling "parser" "Syntax Errors" \
    "sus x drip = ; vibez.spill(x)"

# Error Test 3: Codegen errors
test_error_handling "codegen" "Type Errors" \
    "sus x drip = \"string\""

echo "📊 Component Test Results"
echo "========================"
echo -e "Total component tests: ${BLUE}$COMPONENT_TESTS${NC}"
echo -e "Passed: ${GREEN}$COMPONENT_PASSED${NC}"
echo -e "Failed: ${RED}$COMPONENT_FAILED${NC}"

if [[ $COMPONENT_FAILED -eq 0 ]]; then
    echo -e "${GREEN}🎉 All component tests passed!${NC}"
    echo -e "${GREEN}✅ Lexer, Parser, and Codegen components are working correctly${NC}"
    exit 0
else
    echo -e "${RED}❌ Some component tests failed. Individual components need fixes.${NC}"
    exit 1
fi

# Cleanup
rm -f /tmp/lexer_test.csd /tmp/parser_test.csd /tmp/codegen_test.csd /tmp/error_test.csd
rm -f /tmp/lexer_output.txt /tmp/parser_output.txt /tmp/codegen_output.txt /tmp/error_output.txt
rm -f /tmp/build_output.txt
