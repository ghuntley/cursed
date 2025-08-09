// Pattern matching test 43
sus value drip = 3
ready (value) {
    0 => vibez.spill("Pattern 43: zero")
    1 => vibez.spill("Pattern 43: one")
    2 => vibez.spill("Pattern 43: two")
    _ => vibez.spill("Pattern 43: other")
}
