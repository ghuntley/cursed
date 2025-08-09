// Pattern matching test 33
sus value drip = 3
ready (value) {
    0 => vibez.spill("Pattern 33: zero")
    1 => vibez.spill("Pattern 33: one")
    2 => vibez.spill("Pattern 33: two")
    _ => vibez.spill("Pattern 33: other")
}
