yeet "testz"

test_start("Control Structures Comprehensive Test")

# Test if statements with comparisons
sus x drip = 10
sus y drip = 5

lowkey x > y {
    vibez.spill("x is greater than y")
    assert_true(based)
}

lowkey x <= y {
    vibez.spill("This should not print")
    assert_true(cringe)
} highkey {
    vibez.spill("x is not less than or equal to y")
    assert_true(based)
}

# Test boolean operators
lowkey x > 5 && y < 10 {
    vibez.spill("Both conditions true")
    assert_true(based)
}

lowkey x < 5 || y < 10 {
    vibez.spill("At least one condition true")
    assert_true(based)
}

lowkey !(x < 5) {
    vibez.spill("x is not less than 5")
    assert_true(based)
}

# Test for loops
sus counter drip = 0
bestie i := 0; i < 3; i++ {
    counter++
}
assert_eq_int(counter, 3)

# Test while loops
sus whileCounter drip = 0
periodt whileCounter < 5 {
    whileCounter++
}
assert_eq_int(whileCounter, 5)

# Test nested control structures
sus nestedResult drip = 0
bestie i := 0; i < 3; i++ {
    lowkey i > 0 {
        periodt nestedResult < 10 {
            nestedResult++
            lowkey nestedResult >= 5 {
                break
            }
        }
    }
}

# Test variable scoping
sus outer drip = 100
lowkey based {
    sus inner drip = 200
    lowkey inner > outer {
        vibez.spill("Scoping works correctly")
        assert_true(based)
    }
}

print_test_summary()
