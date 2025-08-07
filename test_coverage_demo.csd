# Test file for code coverage demonstration
yeet "testz"
yeet "vibez"

# Function with conditional branches for coverage testing
slay test_conditional_coverage(value normie) lit {
    ready (value > 0) {
        vibez.spill("Positive value: " + value.toString())
        damn based
    } else ready (value < 0) {
        vibez.spill("Negative value: " + value.toString())
        damn based
    } else {
        vibez.spill("Zero value")
        damn cringe
    }
}

# Function with loop for coverage testing
slay test_loop_coverage(count normie) {
    sus i normie = 0
    bestie (i < count) {
        ready (i % 2 == 0) {
            vibez.spill("Even: " + i.toString())
        } else {
            vibez.spill("Odd: " + i.toString())
        }
        i = i + 1
    }
}

# Function that handles different cases
slay test_switch_coverage(type tea) tea {
    ready (type == "admin") {
        damn "Administrator access granted"
    } else ready (type == "user") {
        damn "User access granted"
    } else ready (type == "guest") {
        damn "Guest access granted"
    } else {
        damn "Access denied"
    }
}

# Function with nested conditions
slay test_nested_coverage(x normie, y normie) normie {
    ready (x > 0) {
        ready (y > 0) {
            damn x + y  # Both positive
        } else {
            damn x - y  # x positive, y negative/zero
        }
    } else {
        ready (y > 0) {
            damn y - x  # x negative/zero, y positive
        } else {
            damn 0      # Both negative/zero
        }
    }
}

# Function that might not be called (for uncovered function testing)
slay unused_function() {
    vibez.spill("This function is never called")
    damn "unreachable"
}

# Error handling function
slay test_error_handling(will_error lit) {
    ready (will_error) {
        vibez.spill("Simulating error condition")
        # This would trigger error handling in real code
    } else {
        vibez.spill("Normal execution path")
    }
}

# Test execution with comprehensive coverage scenarios
test_start("Coverage demo test")

# Test positive branch
assert_true(test_conditional_coverage(5))
vibez.spill("Tested positive branch")

# Test negative branch  
assert_true(test_conditional_coverage(-3))
vibez.spill("Tested negative branch")

# Test zero branch
assert_false(test_conditional_coverage(0))
vibez.spill("Tested zero branch")

# Test loop coverage with different counts
test_loop_coverage(3)
vibez.spill("Tested loop with count 3")

test_loop_coverage(0)
vibez.spill("Tested loop with count 0")

# Test switch coverage - test some but not all branches
assert_eq_string(test_switch_coverage("admin"), "Administrator access granted")
assert_eq_string(test_switch_coverage("user"), "User access granted")
# Note: "guest" and default branches are not tested (intentional coverage gap)

# Test nested conditions - partial coverage
assert_eq_int(test_nested_coverage(5, 3), 8)   # Both positive
assert_eq_int(test_nested_coverage(-2, 4), 6)  # x negative, y positive
# Note: Other branches not tested (intentional coverage gaps)

# Test error handling - only one branch
test_error_handling(cringe)  # Only test normal path, not error path

# Note: unused_function() is never called (intentional coverage gap)

vibez.spill("Coverage test completed")
print_test_summary()
