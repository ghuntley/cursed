// Pattern matching test 31
sus value drip = 1
ready (value) {
    0 => vibez.spill("Pattern 31: zero")
    1 => vibez.spill("Pattern 31: one")
    2 => vibez.spill("Pattern 31: two")
    _ => vibez.spill("Pattern 31: other")
}
