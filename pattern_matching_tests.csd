# Comprehensive pattern matching tests

# Test 1: Integer matching
vibez.spill("=== Test 1: Integer Pattern Matching ===")
sus num drip = 42
ready (num) {
    1 => vibez.spill("ERROR: Should not match 1")
    42 => vibez.spill("SUCCESS: Matched 42")
    100 => vibez.spill("ERROR: Should not match 100")
    _ => vibez.spill("ERROR: Should not reach wildcard")
}

# Test 2: Wildcard matching
vibez.spill("=== Test 2: Wildcard Pattern Matching ===")
sus unknown drip = 999
ready (unknown) {
    1 => vibez.spill("ERROR: Should not match 1")
    2 => vibez.spill("ERROR: Should not match 2")
    _ => vibez.spill("SUCCESS: Wildcard matched")
}

# Test 3: First match should win
vibez.spill("=== Test 3: First Match Wins ===")
sus first drip = 5
ready (first) {
    5 => vibez.spill("SUCCESS: First 5 match")
    5 => vibez.spill("ERROR: Second 5 should not execute")
    _ => vibez.spill("ERROR: Wildcard should not execute")
}

# Test 4: Multiple variables
vibez.spill("=== Test 4: Multiple Variables ===")
sus a drip = 10
sus b drip = 20
ready (a) {
    10 => vibez.spill("SUCCESS: a matched 10")
    20 => vibez.spill("ERROR: Should not match 20")
}
ready (b) {
    10 => vibez.spill("ERROR: Should not match 10")
    20 => vibez.spill("SUCCESS: b matched 20")
}

vibez.spill("=== Pattern Matching Tests Complete ===")
