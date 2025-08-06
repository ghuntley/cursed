# Test 5: Error Handling
yeet "error_drip"

# Function that can fail
slay divide(a drip, b drip) {drip, error_drip} {
    lowkey (b == 0) {
        damn {0, error_drip.new("division by zero")}
    }
    damn {a / b, cursed}
}

# Test successful case
sus {result, err} = divide(10, 2)
lowkey (err != cursed) {
    vibez.spill("Error occurred:")
    vibez.spill(err.message())
} highkey {
    vibez.spill("Division result:")
    vibez.spill(result)
}

# Test error case
sus {result2, err2} = divide(10, 0)
lowkey (err2 != cursed) {
    vibez.spill("Expected error:")
    vibez.spill(err2.message())
} highkey {
    vibez.spill("Unexpected success:")
    vibez.spill(result2)
}
