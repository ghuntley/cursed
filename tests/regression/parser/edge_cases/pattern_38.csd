// Pattern matching test 38
sus value drip = 3
ready (value) {
    0 => vibez.spill("Pattern 38: zero")
    1 => vibez.spill("Pattern 38: one")
    2 => vibez.spill("Pattern 38: two")
    _ => vibez.spill("Pattern 38: other")
}
