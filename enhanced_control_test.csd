# Enhanced Control Structures Test
# Test complex boolean operators, comparison operators, and nested structures

# Test basic boolean operators
sus a drip = 5
sus b drip = 10
sus flag lit = based

# Test logical AND (&&)
ready (a > 3 && b < 15) {
    vibez.spill("AND condition works: a > 3 AND b < 15")
}

# Test logical OR (||)
ready (a > 10 || b < 5) {
    vibez.spill("This should not print")
} otherwise {
    vibez.spill("OR condition works: NOT (a > 10 OR b < 5)")
}

# Test logical NOT (!)
ready (!flag) {
    vibez.spill("This should not print")
} otherwise {
    vibez.spill("NOT condition works: flag is based")
}

# Test all comparison operators
sus x drip = 8
sus y drip = 8

# Equality
ready (x == y) {
    vibez.spill("Equality works: x equals y")
}

# Not equal
ready (x != y) {
    vibez.spill("This should not print")
} otherwise {
    vibez.spill("Not equal works: x does not differ from y")
}

# Less than or equal
ready (x <= y) {
    vibez.spill("Less than or equal works: x <= y")
}

# Greater than or equal
ready (x >= y) {
    vibez.spill("Greater than or equal works: x >= y")
}

# Test complex nested conditions with parentheses
ready ((a > 3 && b < 15) || (x == y && flag)) {
    vibez.spill("Complex nested condition works")
}

# Test nested control structures
sus i drip = 0
bestie (i < 3) {
    ready (i % 2 == 0) {
        vibez.spill("Even number:", i)
    } otherwise {
        vibez.spill("Odd number:", i)
    }
    i = i + 1
}

# Test nested loops and conditions
sus outer drip = 0
bestie (outer < 2) {
    vibez.spill("Outer loop:", outer)
    sus inner drip = 0
    bestie (inner < 2) {
        ready (outer == inner) {
            vibez.spill("  Diagonal:", outer, inner)
        } otherwise {
            vibez.spill("  Off-diagonal:", outer, inner)
        }
        inner = inner + 1
    }
    outer = outer + 1
}

# Test edge cases
ready (cringe) {
    vibez.spill("This should not print")
} otherwise {
    vibez.spill("False condition handled correctly")
}

# Empty condition should be false
sus empty_flag lit = cringe
bestie (empty_flag) {
    vibez.spill("This should not print")
}

vibez.spill("All enhanced control structure tests completed")
