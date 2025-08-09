// Pattern matching test 46
sus value drip = 1
ready (value) {
    0 => vibez.spill("Pattern 46: zero")
    1 => vibez.spill("Pattern 46: one")
    2 => vibez.spill("Pattern 46: two")
    _ => vibez.spill("Pattern 46: other")
}
