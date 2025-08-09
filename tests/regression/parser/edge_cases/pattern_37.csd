// Pattern matching test 37
sus value drip = 2
ready (value) {
    0 => vibez.spill("Pattern 37: zero")
    1 => vibez.spill("Pattern 37: one")
    2 => vibez.spill("Pattern 37: two")
    _ => vibez.spill("Pattern 37: other")
}
