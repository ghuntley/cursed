// Pattern matching test 42
sus value drip = 2
ready (value) {
    0 => vibez.spill("Pattern 42: zero")
    1 => vibez.spill("Pattern 42: one")
    2 => vibez.spill("Pattern 42: two")
    _ => vibez.spill("Pattern 42: other")
}
