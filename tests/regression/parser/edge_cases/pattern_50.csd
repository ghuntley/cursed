// Pattern matching test 50
sus value drip = 0
ready (value) {
    0 => vibez.spill("Pattern 50: zero")
    1 => vibez.spill("Pattern 50: one")
    2 => vibez.spill("Pattern 50: two")
    _ => vibez.spill("Pattern 50: other")
}
