// Pattern matching test 44
sus value drip = 4
ready (value) {
    0 => vibez.spill("Pattern 44: zero")
    1 => vibez.spill("Pattern 44: one")
    2 => vibez.spill("Pattern 44: two")
    _ => vibez.spill("Pattern 44: other")
}
