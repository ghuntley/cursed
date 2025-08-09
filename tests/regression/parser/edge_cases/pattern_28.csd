// Pattern matching test 28
sus value drip = 3
ready (value) {
    0 => vibez.spill("Pattern 28: zero")
    1 => vibez.spill("Pattern 28: one")
    2 => vibez.spill("Pattern 28: two")
    _ => vibez.spill("Pattern 28: other")
}
