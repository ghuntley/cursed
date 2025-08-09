// Pattern matching test 40
sus value drip = 0
ready (value) {
    0 => vibez.spill("Pattern 40: zero")
    1 => vibez.spill("Pattern 40: one")
    2 => vibez.spill("Pattern 40: two")
    _ => vibez.spill("Pattern 40: other")
}
