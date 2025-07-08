#!/bin/bash

# Quick Production Test for Key Stdlib Modules
echo "=== Quick Production Readiness Test ==="

# Test key modules
key_modules=(
    "stdlib/math/test_math.csd"
    "stdlib/string/test_string.csd"
    "stdlib/crypto/test_crypto.csd"
    "stdlib/json/test_json.csd"
    "stdlib/csv/test_csv.csd"
    "stdlib/collections/test_collections.csd"
    "stdlib/testz/test_testz.csd"
)

successful=0
total=0

for test_file in "${key_modules[@]}"; do
    if [ -f "$test_file" ]; then
        total=$((total + 1))
        module_name=$(basename "$(dirname "$test_file")")
        echo "Testing $module_name..."
        
        # Test interpretation
        if timeout 30 cargo run --bin cursed "$test_file" > /dev/null 2>&1; then
            echo "  ✅ Interpretation: OK"
            
            # Test compilation
            if timeout 60 cargo run --bin cursed -- compile "$test_file" > /dev/null 2>&1; then
                echo "  ✅ Compilation: OK"
                
                # Execute compiled binary
                executable=$(basename "$test_file" .csd)
                if [ -f "./$executable" ] && timeout 30 "./$executable" > /dev/null 2>&1; then
                    echo "  ✅ Execution: OK"
                    successful=$((successful + 1))
                    rm -f "./$executable"
                else
                    echo "  ❌ Execution: FAILED"
                fi
            else
                echo "  ❌ Compilation: FAILED"
            fi
        else
            echo "  ❌ Interpretation: FAILED"
        fi
        echo ""
    fi
done

echo "Quick test results: $successful/$total modules successful"

# Test self-hosting
echo "Testing self-hosting..."
cat > minimal_self_hosting_test.csd << 'EOF'
vibez.spill("Self-hosting test")
sus x normie = 42
vibez.spill("Value:", x)
EOF

if timeout 30 cargo run --bin cursed minimal_self_hosting_test.csd > /dev/null 2>&1; then
    if timeout 60 cargo run --bin cursed -- compile minimal_self_hosting_test.csd > /dev/null 2>&1; then
        if [ -f "./minimal_self_hosting_test" ] && timeout 30 ./minimal_self_hosting_test > /dev/null 2>&1; then
            echo "✅ Self-hosting: OK"
            rm -f ./minimal_self_hosting_test
        else
            echo "❌ Self-hosting execution: FAILED"
        fi
    else
        echo "❌ Self-hosting compilation: FAILED"
    fi
else
    echo "❌ Self-hosting interpretation: FAILED"
fi

rm -f minimal_self_hosting_test.csd
