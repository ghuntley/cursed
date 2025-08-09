// Pattern matching test 29
sus value drip = 4
ready (value) {
    0 => vibez.spill("Pattern 29: zero")
    1 => vibez.spill("Pattern 29: one")
    2 => vibez.spill("Pattern 29: two")
    _ => vibez.spill("Pattern 29: other")
}
