// Pattern matching test 39
sus value drip = 4
ready (value) {
    0 => vibez.spill("Pattern 39: zero")
    1 => vibez.spill("Pattern 39: one")
    2 => vibez.spill("Pattern 39: two")
    _ => vibez.spill("Pattern 39: other")
}
