vibez.spill("Testing range patterns...")

# Test case 1: Value in first range
sus value1 drip = 5
ready (value1) { 
    0..10 => vibez.spill("Test 1: small (0-10)")
    11..50 => vibez.spill("Test 1: medium (11-50)") 
    _ => vibez.spill("Test 1: large")
}

# Test case 2: Value in second range
sus value2 drip = 25
ready (value2) { 
    0..10 => vibez.spill("Test 2: small (0-10)")
    11..50 => vibez.spill("Test 2: medium (11-50)") 
    _ => vibez.spill("Test 2: large")
}

# Test case 3: Value outside all ranges
sus value3 drip = 75
ready (value3) { 
    0..10 => vibez.spill("Test 3: small (0-10)")
    11..50 => vibez.spill("Test 3: medium (11-50)") 
    _ => vibez.spill("Test 3: large")
}

# Test case 4: Edge case - exact boundary
sus value4 drip = 50
ready (value4) { 
    0..10 => vibez.spill("Test 4: small (0-10)")
    11..50 => vibez.spill("Test 4: medium (11-50)") 
    _ => vibez.spill("Test 4: large")
}

# Test case 5: Single number exact match
sus value5 drip = 100
ready (value5) { 
    100 => vibez.spill("Test 5: exact match")
    0..50 => vibez.spill("Test 5: range match") 
    _ => vibez.spill("Test 5: no match")
}

vibez.spill("All range pattern tests completed!")
