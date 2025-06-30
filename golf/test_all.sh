#!/bin/bash
# Test Rule 30 Golf with all n values

echo "=== CURSED Rule 30 Golf Test Suite ==="
echo "Testing n ∈ [1‒12]"
echo ""

# Colors
BLUE='\033[34m'
GREEN='\033[32m'
RESET='\033[0m'

CURSED="../target/debug/cursed"

if [ ! -f "$CURSED" ]; then
    echo "Error: CURSED compiler not found at $CURSED"
    exit 1
fi

# Test each value of n
for n in {1..12}; do
    echo -e "${BLUE}Testing n=$n:${RESET}"
    
    # Create temporary test file
    sed "s/sus n=1;/sus n=$n;/" golf_final.csd > temp_n$n.csd
    
    # Run test
    result=$($CURSED temp_n$n.csd 2>/dev/null)
    echo -e "${GREEN}Result: $result${RESET}"
    
    # Clean up
    rm temp_n$n.csd
    
    echo ""
done

echo "Test completed!"
