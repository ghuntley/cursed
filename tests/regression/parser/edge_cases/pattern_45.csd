// Pattern matching test 45
sus value drip = 0
ready (value) {
    0 => vibez.spill("Pattern 45: zero")
    1 => vibez.spill("Pattern 45: one")
    2 => vibez.spill("Pattern 45: two")
    _ => vibez.spill("Pattern 45: other")
}
