# Test 6: Error handling constructs
slay divide(a drip, b drip) (drip, tea) {
    ready (b == 0) {
        damn 0, "division by zero"
    }
    damn a / b, ""
}

slay safe_sqrt(x drip) (drip, tea) {
    ready (x < 0) {
        damn 0, "negative number"
    }
    # Simplified sqrt approximation
    damn x / 2, ""
}

vibez.spill("Error handling test:")

# Test successful operations
sus result1, err1 = divide(10, 2)
ready (err1 == "") {
    vibez.spill("10 / 2 =", result1)
} otherwise {
    vibez.spill("Error:", err1)
}

# Test division by zero
sus result2, err2 = divide(10, 0)
ready (err2 == "") {
    vibez.spill("10 / 0 =", result2)
} otherwise {
    vibez.spill("Error:", err2)
}

# Test negative sqrt
sus result3, err3 = safe_sqrt(-5)
ready (err3 == "") {
    vibez.spill("sqrt(-5) =", result3)
} otherwise {
    vibez.spill("Error:", err3)
}

# Test successful sqrt
sus result4, err4 = safe_sqrt(16)
ready (err4 == "") {
    vibez.spill("sqrt(16) =", result4)
} otherwise {
    vibez.spill("Error:", err4)
}
