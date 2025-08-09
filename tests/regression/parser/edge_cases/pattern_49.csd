// Pattern matching test 49
sus value drip = 4
ready (value) {
    0 => vibez.spill("Pattern 49: zero")
    1 => vibez.spill("Pattern 49: one")
    2 => vibez.spill("Pattern 49: two")
    _ => vibez.spill("Pattern 49: other")
}
