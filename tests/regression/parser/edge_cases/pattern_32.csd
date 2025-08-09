// Pattern matching test 32
sus value drip = 2
ready (value) {
    0 => vibez.spill("Pattern 32: zero")
    1 => vibez.spill("Pattern 32: one")
    2 => vibez.spill("Pattern 32: two")
    _ => vibez.spill("Pattern 32: other")
}
