// Pattern matching test 36
sus value drip = 1
ready (value) {
    0 => vibez.spill("Pattern 36: zero")
    1 => vibez.spill("Pattern 36: one")
    2 => vibez.spill("Pattern 36: two")
    _ => vibez.spill("Pattern 36: other")
}
