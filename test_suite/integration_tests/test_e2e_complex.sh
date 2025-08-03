#!/bin/bash

# Test end-to-end complex program functionality

set -e

# Create complex test program
cat > complex_test.csd << 'EOF'
fr fr Complex integration test program

squad Point {
    spill x meal
    spill y meal
}

slay distance(p1 Point, p2 Point) meal {
    sus dx meal = p1.x - p2.x
    sus dy meal = p1.y - p2.y
    damn dx * dx + dy * dy  fr fr simplified distance
}

slay main() {
    sus origin Point = Point{x: 0.0, y: 0.0}
    sus point Point = Point{x: 3.0, y: 4.0}
    
    sus dist meal = distance(origin, point)
    vibez.spill("Distance squared:", dist)
    
    bestie dist > 20.0 {
        vibez.spill("Far point")
    } else {
        vibez.spill("Near point")
    }
    
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus sum drip = 0
    
    bestie i := 0; i < numbers.len(); i = i + 1 {
        sum = sum + numbers[i]
    }
    
    vibez.spill("Sum:", sum)
    
    match sum {
        15 => vibez.spill("Perfect!")
        x if x > 10 => vibez.spill("Large sum")
        _ => vibez.spill("Small sum")
    }
}

main()
EOF

# Test interpretation pipeline
echo "Testing interpretation mode..."
INTERP_OUTPUT=$(./cursed-unified complex_test.csd 2>&1)
if [[ "$INTERP_OUTPUT" == *"Distance squared:"* ]] && [[ "$INTERP_OUTPUT" == *"Sum:"* ]] && [[ "$INTERP_OUTPUT" == *"Perfect!"* ]]; then
    echo "Interpretation mode: PASS"
else
    echo "Interpretation mode: FAIL"
    echo "Output: $INTERP_OUTPUT"
    exit 1
fi

# Test compilation pipeline
echo "Testing compilation mode..."
./cursed-unified --compile complex_test.csd
if [ -f ./complex_test ]; then
    COMP_OUTPUT=$(./complex_test 2>&1)
    if [[ "$COMP_OUTPUT" == *"Distance squared:"* ]] && [[ "$COMP_OUTPUT" == *"Sum:"* ]]; then
        echo "Compilation mode: PASS"
    else
        echo "Compilation mode: FAIL"
        echo "Output: $COMP_OUTPUT"
        exit 1
    fi
    rm -f complex_test
else
    echo "Compilation failed to produce executable"
    exit 1
fi

# Cleanup
rm -f complex_test.csd

exit 0
