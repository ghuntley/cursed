yeet "testz"

test_start("Error Handling Tests - yikes/shook")

# Function that can fail
slay divide(a drip, b drip) (drip, tea) {
    ready (b == 0) {
        damn 0, "division by zero"
    }
    damn a / b, ""
}

# Function that might panic
slay risky_operation(x drip) drip {
    ready (x < 0) {
        yikes "negative value not allowed: " + str(x)
    }
    damn x * 2
}

# Test successful operation
(result, err) := divide(10, 2)
ready (err != "") {
    vibez.spill("Error: " + err)
} else {
    vibez.spill("10 / 2 = " + str(result))
    assert_eq_int(result, 5)
}

# Test error case
(result2, err2) := divide(10, 0)
ready (err2 != "") {
    vibez.spill("Expected error: " + err2)
    assert_eq_string(err2, "division by zero")
} else {
    vibez.spill("Unexpected success: " + str(result2))
    assert_true(cringe) # Should not reach here
}

# Test panic recovery with shook
sus recovered lit = cringe
sus panic_message tea = ""

shook {
    risky_operation(-5)
} catch (msg tea) {
    recovered = based
    panic_message = msg
    vibez.spill("Caught panic: " + msg)
}

assert_true(recovered)
assert_eq_string(panic_message, "negative value not allowed: -5")

# Test successful risky operation
sus safe_result drip = risky_operation(10)
vibez.spill("Safe result: " + str(safe_result))
assert_eq_int(safe_result, 20)

# Nested error handling
slay validate_and_process(value drip) (drip, tea) {
    ready (value < 0) {
        damn 0, "negative input"
    }
    ready (value > 100) {
        damn 0, "input too large"
    }
    
    # Could panic internally
    shook {
        sus processed drip = risky_operation(value)
        damn processed, ""
    } catch (msg tea) {
        damn 0, "processing failed: " + msg
    }
}

# Test nested error handling
(val1, err1) := validate_and_process(50)
ready (err1 != "") {
    vibez.spill("Error: " + err1)
} else {
    vibez.spill("Processed value: " + str(val1))
    assert_eq_int(val1, 100)
}

(val2, err2) := validate_and_process(-10)
ready (err2 != "") {
    vibez.spill("Expected validation error: " + err2)
    assert_eq_string(err2, "negative input")
}

print_test_summary()
