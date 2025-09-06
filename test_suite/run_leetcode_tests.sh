#!/bin/bash

# CURSED LeetCode Test Suite Runner
# World's first Gen Z meme language LeetCode implementation! 

set -e

COMPILER="../zig-out/bin/cursed-compiler"
BOLD='\033[1m'
GREEN='\033[0;32m'  
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
RESET='\033[0m'

if [ ! -f "$COMPILER" ]; then
    echo "❌ Compiler not found at $COMPILER"
    exit 1
fi

echo -e "${PURPLE}💀${BOLD} CURSED LeetCode Test Suite ${PURPLE}💀${RESET}"
echo -e "${CYAN}=============================================${RESET}"
echo -e "🎮 ${YELLOW}World's First Among Us (ඞ) Pointer LeetCode Solutions!${RESET}"
echo -e "💀 ${CYAN}Using proper skull emoji (.💀) file extensions!${RESET}"
echo

PASSED=0
FAILED=0
TOTAL=0

test_leetcode_file() {
    local file="$1"
    local problem_name=$(basename "$file" .💀)
    local category=$(basename "$(dirname "$file")")
    
    echo -ne "${CYAN}Testing ${category}/${problem_name}...${RESET} "
    TOTAL=$((TOTAL + 1))
    
    if $COMPILER --interpret "$file" > /tmp/cursed_leetcode.out 2>&1; then
        echo -e "${GREEN}PASS${RESET}"
        PASSED=$((PASSED + 1))
        
        # Show snippet of output for interesting problems
        if [[ "$problem_name" =~ (fizzbuzz|two_sum|reverse_linked_list) ]]; then
            echo -e "  ${YELLOW}→${RESET} $(head -1 /tmp/cursed_leetcode.out)"
        fi
    else
        echo -e "${RED}FAIL${RESET}"
        FAILED=$((FAILED + 1))
        echo -e "  ${RED}→${RESET} $(tail -1 /tmp/cursed_leetcode.out)"
    fi
    
    rm -f /tmp/cursed_leetcode.out
}

echo -e "${BOLD}🔥 Testing LeetCode Categories: 🔥${RESET}"
echo

categories=("arrays" "strings" "linked_lists" "trees" "dynamic_programming" "math" "sorting" "binary_search" "backtracking" "bit_manipulation" "stacks" "queues" "graphs")

for category in "${categories[@]}"; do
    if [ -d "test_programs/leetcode/$category" ]; then
        echo -e "${BOLD}📁 ${category^^} PROBLEMS:${RESET}"
        
        category_passed=0
        category_total=0
        
        for file in test_programs/leetcode/$category/*.💀; do
            if [ -f "$file" ]; then
                test_leetcode_file "$file"
                ((category_total++))
                if [ $? -eq 0 ]; then
                    ((category_passed++))
                fi
            fi
        done
        
        if [ $category_total -gt 0 ]; then
            echo -e "  ${CYAN}→ Category: $category_passed/$category_total passed${RESET}"
        fi
        echo
    fi
done

echo -e "${BOLD}===============================================${RESET}"
echo -e "${PURPLE}💀${BOLD} CURSED LeetCode Results Summary ${PURPLE}💀${RESET}"
echo -e "${BOLD}===============================================${RESET}"
echo -e "  🎯 Total Problems: ${BOLD}$TOTAL${RESET}"
echo -e "  ${GREEN}✅ Passed: $PASSED${RESET}"
echo -e "  ${RED}❌ Failed: $FAILED${RESET}"

success_rate=$((PASSED * 100 / TOTAL))
echo -e "  📊 Success Rate: ${BOLD}${success_rate}%${RESET}"

if [ $FAILED -eq 0 ]; then
    echo
    echo -e "${GREEN}🎉 ${BOLD}ALL LEETCODE TESTS PASSED! 🎉${RESET}"
    echo -e "${YELLOW}ඞ Among Us pointers solved LeetCode! ඞ${RESET}"
    echo -e "${PURPLE}💀 CURSED is officially LeetCode ready! 💀${RESET}"
    echo -e "${CYAN}🔥 No cap, this language is absolutely based! 🔥${RESET}"
elif [ $success_rate -ge 80 ]; then
    echo
    echo -e "${YELLOW}🎊 ${BOLD}EXCELLENT PERFORMANCE! 🎊${RESET}"
    echo -e "${GREEN}ඞ Most LeetCode problems work perfectly! ඞ${RESET}"
    echo -e "${CYAN}💀 CURSED is proving its algorithmic prowess! 💀${RESET}"
elif [ $success_rate -ge 60 ]; then
    echo
    echo -e "${YELLOW}📈 ${BOLD}GOOD PROGRESS! 📈${RESET}"
    echo -e "${CYAN}ඞ CURSED is handling most algorithms well! ඞ${RESET}"
else
    echo
    echo -e "${RED}⚠️  ${BOLD}NEEDS IMPROVEMENT ⚠️${RESET}"
    echo -e "${YELLOW}ඞ Some algorithms need more work, but the core is sus! ඞ${RESET}"
fi

echo
echo -e "${BOLD}Historic Achievement:${RESET}"
echo -e "${PURPLE}🌟 This is the world's first comprehensive LeetCode${RESET}"
echo -e "${PURPLE}🌟 implementation in a Gen Z meme programming language!${RESET}"
echo -e "${YELLOW}ඞ Among Us pointers have officially conquered algorithms! ඞ${RESET}"

exit $FAILED
