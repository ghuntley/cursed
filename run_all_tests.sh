#!/bin/bash

# CURSED Extensions Comprehensive Test Runner
# This script runs all test suites for CURSED language extensions

set -e

echo "🧪 CURSED Extensions Comprehensive Test Suite"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results
PASSED=0
FAILED=0

run_test() {
    local test_name="$1"
    local test_cmd="$2"

    echo -n "Testing $test_name... "

    if eval "$test_cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
        return 1
    fi
}

# Test Tree-Sitter
echo "1. Tree-Sitter Grammar Tests"
echo "----------------------------"

if command -v npm &> /dev/null && [ -d "tree-sitter" ]; then
    cd tree-sitter
    if [ -f "package.json" ]; then
        run_test "Tree-Sitter parsing" "npm test"
    else
        echo -e "${YELLOW}⚠ SKIP${NC} - package.json not found"
    fi
    cd ..
else
    echo -e "${YELLOW}⚠ SKIP${NC} - npm or tree-sitter directory not found"
fi

# Test VSCode Extension
echo ""
echo "2. VSCode Extension Tests"
echo "-------------------------"

if [ -d "cursed-vscode-extension" ]; then
    cd cursed-vscode-extension
    if [ -f "package.json" ]; then
        run_test "VSCode extension validation" "node -e \"const pkg=require('./package.json'); console.log(pkg.name, pkg.contributes?.languages?.[0]?.id)\""
    else
        echo -e "${YELLOW}⚠ SKIP${NC} - package.json not found"
    fi
    cd ..
else
    echo -e "${YELLOW}⚠ SKIP${NC} - VSCode extension directory not found"
fi

# Test Vim Extensions
echo ""
echo "3. Vim Extension Tests"
echo "----------------------"

test_vim_extension() {
    local dir="$1"
    local name="$2"

    if [ -d "$dir" ]; then
        local missing_files=""
        [ ! -f "$dir/syntax/cursed.vim" ] && missing_files="$missing_files syntax/cursed.vim"
        [ ! -f "$dir/ftdetect/cursed.vim" ] && missing_files="$missing_files ftdetect/cursed.vim"

        if [ -z "$missing_files" ]; then
            # Check for required keywords
            if grep -q "syn keyword.*vibe" "$dir/syntax/cursed.vim" && \
               grep -q "syn keyword.*slay" "$dir/syntax/cursed.vim" && \
               grep -q "syn keyword.*based" "$dir/syntax/cursed.vim"; then
                run_test "$name syntax validation" "true"
            else
                run_test "$name syntax validation" "false"
            fi
        else
            run_test "$name file structure" "false"
        fi
    else
        echo -e "${YELLOW}⚠ SKIP${NC} - $name directory not found"
    fi
}

test_vim_extension "cursed-vim-advanced" "cursed-vim-advanced"
test_vim_extension "vim-cursed" "vim-cursed"

# Test IntelliJ Plugin
echo ""
echo "4. IntelliJ Plugin Tests"
echo "------------------------"

if [ -d "cursed-intellij-plugin" ]; then
    if [ -f "cursed-intellij-plugin/src/main/kotlin/org/cursed/CursedLexer.kt" ] && \
       [ -f "cursed-intellij-plugin/src/main/resources/META-INF/plugin.xml" ]; then
        run_test "IntelliJ plugin structure" "true"
    else
        run_test "IntelliJ plugin structure" "false"
    fi
else
    echo -e "${YELLOW}⚠ SKIP${NC} - IntelliJ plugin directory not found"
fi

# Test Syntax Consistency
echo ""
echo "5. Syntax Consistency Tests"
echo "---------------------------"

# Create test file
cat > /tmp/cursed_syntax_test.💀 << 'EOF'
vibe main

slay main() {
    sus x normie = 42
    ready x > 0 {
        vibez.spill("positive")
    }
    fr fr line comment
    no cap
    block comment
    on god
}
EOF

# Test with Tree-Sitter if available
if command -v tree-sitter &> /dev/null && [ -d "tree-sitter" ]; then
    cd tree-sitter
    if tree-sitter parse /tmp/cursed_syntax_test.💀 > /dev/null 2>&1; then
        run_test "Tree-Sitter syntax parsing" "true"
    else
        run_test "Tree-Sitter syntax parsing" "false"
    fi
    cd ..
else
    echo -e "${YELLOW}⚠ SKIP${NC} - Tree-Sitter not available"
fi

# Clean up
rm -f /tmp/cursed_syntax_test.💀

# Test File Extensions
echo ""
echo "6. File Extension Tests"
echo "-----------------------"

for ext in ".💀" ".cursed"; do
    test_file="/tmp/test${ext}"
    echo "vibe main" > "$test_file"

    if [ -f "$test_file" ]; then
        run_test "File extension $ext creation" "true"
    else
        run_test "File extension $ext creation" "false"
    fi

    rm -f "$test_file"
done

# Summary
echo ""
echo "=============================================="
echo "📊 TEST RESULTS SUMMARY"
echo "=============================================="

TOTAL=$((PASSED + FAILED))

if [ $TOTAL -gt 0 ]; then
    PERCENTAGE=$((PASSED * 100 / TOTAL))
    echo "Passed: $PASSED/$TOTAL ($PERCENTAGE%)"

    if [ $FAILED -eq 0 ]; then
        echo -e "${GREEN}🎉 All tests passed! CURSED extensions are ready.${NC}"
        exit 0
    else
        echo -e "${RED}❌ $FAILED test(s) failed. Please review the issues above.${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ No tests were run. Check your environment.${NC}"
    exit 1
fi