#!/usr/bin/env -S cursed
# Basic Memory & GC Test

yeet "testz"

test_start("Basic memory allocation test")

# Allocate some objects to test GC
sus numbers arrayz.Array = arrayz.new_array(normie, 50)
sus i drip = 0
bestie (i < 50) {
    numbers.push(i * 2)
    i = i + 1
}

assert_eq_int(numbers.length(), 50)
vibez.spill("✅ Memory allocation test passed")

test_start("GC stress test")

# Create temporary objects that can be collected
sus j drip = 0
bestie (j < 10) {
    sus temp_array arrayz.Array = arrayz.new_array(normie, 10)
    sus k drip = 0
    bestie (k < 10) {
        temp_array.push(j * k)
        k = k + 1
    }
    j = j + 1
}

vibez.spill("✅ GC stress test completed")

print_test_summary()
