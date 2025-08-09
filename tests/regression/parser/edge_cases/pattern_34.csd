// Pattern matching test 34
sus value drip = 4
ready (value) {
    0 => vibez.spill("Pattern 34: zero")
    1 => vibez.spill("Pattern 34: one")
    2 => vibez.spill("Pattern 34: two")
    _ => vibez.spill("Pattern 34: other")
}
