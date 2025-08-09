// Pattern matching test 47
sus value drip = 2
ready (value) {
    0 => vibez.spill("Pattern 47: zero")
    1 => vibez.spill("Pattern 47: one")
    2 => vibez.spill("Pattern 47: two")
    _ => vibez.spill("Pattern 47: other")
}
