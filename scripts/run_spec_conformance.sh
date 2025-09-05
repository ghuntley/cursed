#!/bin/bash

# Run comprehensive spec conformance tests

echo "🔍 Running CURSED Spec Conformance Tests..."
echo "============================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print test results
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        return 1
    fi
}

# Track overall success
OVERALL_SUCCESS=0

echo -e "${YELLOW}1. Grammar Conformance Tests${NC}"
echo "----------------------------"

# Run grammar conformance tests
cargo test test_spec_map_keyword_conformance --test spec_conformance
RESULT1=$?
print_result $RESULT1 "Keyword conformance against spec-map.json"
if [ $RESULT1 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_grammar_rule_conformance --test spec_conformance  
RESULT2=$?
print_result $RESULT2 "Grammar rule conformance"
if [ $RESULT2 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_comment_syntax_conformance --test spec_conformance
RESULT3=$?
print_result $RESULT3 "Comment syntax conformance"
if [ $RESULT3 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_literal_conformance --test spec_conformance
RESULT4=$?
print_result $RESULT4 "Literal syntax conformance"
if [ $RESULT4 -ne 0 ]; then OVERALL_SUCCESS=1; fi

echo -e "\n${YELLOW}2. Lexer/Parser Consistency Tests${NC}"
echo "--------------------------------"

cargo test test_periodt_consistency --test lexer_parser_consistency
RESULT5=$?
print_result $RESULT5 "Periodt keyword consistency"
if [ $RESULT5 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_flex_consistency --test lexer_parser_consistency
RESULT6=$?
print_result $RESULT6 "Flex keyword consistency"
if [ $RESULT6 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_bestie_vs_flex_disambiguation --test lexer_parser_consistency
RESULT7=$?
print_result $RESULT7 "Bestie vs Flex disambiguation"
if [ $RESULT7 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_basic_keyword_consistency --test lexer_parser_consistency
RESULT8=$?
print_result $RESULT8 "Basic keyword consistency"
if [ $RESULT8 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_type_keyword_consistency --test lexer_parser_consistency
RESULT9=$?
print_result $RESULT9 "Type keyword consistency"
if [ $RESULT9 -ne 0 ]; then OVERALL_SUCCESS=1; fi

echo -e "\n${YELLOW}3. Range-For Loop Syntax Tests${NC}"
echo "-----------------------------"

cargo test test_range_for_loop_conformance --test spec_conformance
RESULT10=$?
print_result $RESULT10 "Range-for loop syntax conformance"
if [ $RESULT10 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_range_for_syntax_edge_cases --test lexer_parser_consistency
RESULT11=$?
print_result $RESULT11 "Range-for syntax edge cases"
if [ $RESULT11 -ne 0 ]; then OVERALL_SUCCESS=1; fi

cargo test test_composite_for_range_syntax --test lexer_parser_consistency
RESULT12=$?
print_result $RESULT12 "Composite for-range syntax"
if [ $RESULT12 -ne 0 ]; then OVERALL_SUCCESS=1; fi

echo -e "\n${YELLOW}4. Round-Trip Validation Tests${NC}"
echo "-----------------------------"

cargo test test_round_trip_spec_validation --test spec_conformance
RESULT13=$?
print_result $RESULT13 "Round-trip spec validation"
if [ $RESULT13 -ne 0 ]; then OVERALL_SUCCESS=1; fi

echo -e "\n${YELLOW}5. Codegen Keyword Support Tests${NC}"
echo "-------------------------------"

cargo test test_codegen_keyword_support --test spec_conformance
RESULT14=$?
print_result $RESULT14 "Codegen keyword support"
if [ $RESULT14 -ne 0 ]; then OVERALL_SUCCESS=1; fi

echo -e "\n${YELLOW}6. Comment Edge Cases${NC}"
echo "-------------------"

cargo test test_comment_keyword_conflicts --test lexer_parser_consistency
RESULT15=$?
print_result $RESULT15 "Comment keyword conflicts"
if [ $RESULT15 -ne 0 ]; then OVERALL_SUCCESS=1; fi

# Overall result
echo "============================================"
if [ $OVERALL_SUCCESS -eq 0 ]; then
    echo -e "${GREEN}🎉 All spec conformance tests passed!${NC}"
    echo -e "${GREEN}✅ 100% conformance between specification and implementation${NC}"
else
    echo -e "${RED}❌ Some spec conformance tests failed${NC}"
    echo -e "${YELLOW}⚠️  Implementation does not fully match specification${NC}"
fi

# Run quick syntax validation on spec examples
echo -e "\n${YELLOW}7. Spec Example Validation${NC}"
echo "-------------------------"

# Create temp file with spec examples
cat > /tmp/spec_examples.💀 << 'EOF'
vibe main

yeet "fmt"
yeet (
    "os"
    "strings"
)

facts PI = 3.14159
facts (
    MAX_SIZE = 1000
    MIN_SIZE = 10
)

sus name tea = "World"
sus age, height = 25, 180.5

be_like Person squad {
    name tea
    age normie
}

slay add(x, y normie) normie {
    yolo x + y
}

slay main() {
    lowkey x > 0 {
        vibez.spill(x)
    } highkey {
        vibez.spill(0)
    }
    
    bestie i := 0; i < 10; i++ {
        vibez.spill(i)
    }
    
    periodt x > 0 {
        x--
    }
    
    vibe_check day {
        mood "Monday":
            vibez.spill("Start of week")
        basic:
            vibez.spill("Mid-week")
    }
    
    stan worker()
    
    ready {
        mood ch <- value:
            vibez.spill("sent")
        basic:
            vibez.spill("default")
    }
    
    x := 42
    (a, b) := tuple
    
    yolo result
}
EOF

# Test parsing the spec examples
echo "Testing spec examples parsing..."
cargo run --bin cursed /tmp/spec_examples.💀 > /dev/null 2>&1
SPEC_PARSE_RESULT=$?

if [ $SPEC_PARSE_RESULT -eq 0 ]; then
    echo -e "${GREEN}✅ All spec examples parse correctly${NC}"
else
    echo -e "${RED}❌ Some spec examples failed to parse${NC}"
    OVERALL_SUCCESS=1
fi

# Clean up
rm -f /tmp/spec_examples.💀

echo "============================================"
exit $OVERALL_SUCCESS
