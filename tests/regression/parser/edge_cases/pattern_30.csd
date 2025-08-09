// Pattern matching test 30
sus value drip = 0
ready (value) {
    0 => vibez.spill("Pattern 30: zero")
    1 => vibez.spill("Pattern 30: one")
    2 => vibez.spill("Pattern 30: two")
    _ => vibez.spill("Pattern 30: other")
}
