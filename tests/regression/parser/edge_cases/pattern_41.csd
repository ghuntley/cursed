// Pattern matching test 41
sus value drip = 1
ready (value) {
    0 => vibez.spill("Pattern 41: zero")
    1 => vibez.spill("Pattern 41: one")
    2 => vibez.spill("Pattern 41: two")
    _ => vibez.spill("Pattern 41: other")
}
