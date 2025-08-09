// Pattern matching test 27
sus value drip = 2
ready (value) {
    0 => vibez.spill("Pattern 27: zero")
    1 => vibez.spill("Pattern 27: one")
    2 => vibez.spill("Pattern 27: two")
    _ => vibez.spill("Pattern 27: other")
}
