sus x drip = 1
vibez.spill("Testing x = 1")
ready (x) {
    1 => vibez.spill("SHOULD execute - matched 1")
    2 => vibez.spill("Should NOT execute - matched 2")
    _ => vibez.spill("Should NOT execute - wildcard")
}

sus y drip = 2
vibez.spill("Testing y = 2")
ready (y) {
    1 => vibez.spill("Should NOT execute - matched 1")
    2 => vibez.spill("SHOULD execute - matched 2")
    _ => vibez.spill("Should NOT execute - wildcard")
}
