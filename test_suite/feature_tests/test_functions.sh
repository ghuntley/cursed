#!/bin/bash

# Test function feature functionality

set -e

# Test all function features
cat > function_features_test.csd << 'EOF'
fr fr Test function features

fr fr Simple function with no parameters
slay greet() {
    vibez.spill("Hello from function!")
}

fr fr Function with parameters
slay add(a drip, b drip) drip {
    damn a + b
}

fr fr Function with multiple parameter types
slay describe(name tea, age drip, active lit) {
    vibez.spill("Name:", name)
    vibez.spill("Age:", age)
    vibez.spill("Active:", active)
}

fr fr Function with complex return type
slay create_point(x meal, y meal) Point {
    damn Point{x: x, y: y}
}

squad Point {
    spill x meal
    spill y meal
}

fr fr Recursive function
slay factorial(n drip) drip {
    bestie n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

fr fr Higher-order function (function as parameter)
slay apply_operation(a drip, b drip, op slay(drip, drip) drip) drip {
    damn op(a, b)
}

slay multiply(x drip, y drip) drip {
    damn x * y
}

fr fr Test function calls
greet()
sus sum drip = add(5, 3)
describe("Alice", 25, based)
sus point Point = create_point(1.5, 2.5)
sus fact drip = factorial(5)
sus product drip = apply_operation(4, 6, multiply)

vibez.spill("Sum:", sum)
vibez.spill("Point x:", point.x)
vibez.spill("Point y:", point.y)
vibez.spill("Factorial 5:", fact)
vibez.spill("Product:", product)
EOF

# Test interpretation mode
./cursed-unified function_features_test.csd

# Test compilation mode
./cursed-unified --compile function_features_test.csd
if [ -f ./function_features_test ]; then
    ./function_features_test
    rm -f function_features_test
fi

# Cleanup
rm -f function_features_test.csd

exit 0
