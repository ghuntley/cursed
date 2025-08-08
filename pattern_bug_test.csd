sus x drip = 5
vibez.spill("Testing pattern matching - should only execute matching branch")
ready (x) {
    1 => vibez.spill("This should NOT execute - branch 1")
    2 => vibez.spill("This should NOT execute - branch 2")
    5 => vibez.spill("This SHOULD execute - branch 5")
    _ => vibez.spill("This should NOT execute - default branch")
}
vibez.spill("Pattern matching test complete")
