yeet "testz"

# Test isolated C-style for loop parsing
slay test_for_loop_basic() lit {
    bestie i := 0; i < 5; i++ {
        vibez.spill("Loop iteration")
    }
    damn based
}

slay test_for_loop_power() lit {
    sus result drip = 2.0
    sus absExp normie = 3
    bestie i := 0; i < absExp; i++ {
        result = result * 2.0
    }
    damn based
}

slay test_for_loop_factorial() lit {
    sus result normie = 1
    sus n normie = 5
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    damn based
}

test_start("C-style for loop parsing tests")
assert_true(test_for_loop_basic())
assert_true(test_for_loop_power())
assert_true(test_for_loop_factorial())
print_test_summary()
