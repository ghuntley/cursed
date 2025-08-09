// Pattern matching test 35
sus value drip = 0
ready (value) {
    0 => vibez.spill("Pattern 35: zero")
    1 => vibez.spill("Pattern 35: one")
    2 => vibez.spill("Pattern 35: two")
    _ => vibez.spill("Pattern 35: other")
}
