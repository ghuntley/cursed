// Pattern matching test 26
sus value drip = 1
ready (value) {
    0 => vibez.spill("Pattern 26: zero")
    1 => vibez.spill("Pattern 26: one")
    2 => vibez.spill("Pattern 26: two")
    _ => vibez.spill("Pattern 26: other")
}
