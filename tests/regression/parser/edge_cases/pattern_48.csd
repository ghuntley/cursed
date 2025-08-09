// Pattern matching test 48
sus value drip = 3
ready (value) {
    0 => vibez.spill("Pattern 48: zero")
    1 => vibez.spill("Pattern 48: one")
    2 => vibez.spill("Pattern 48: two")
    _ => vibez.spill("Pattern 48: other")
}
