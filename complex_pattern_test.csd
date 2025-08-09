sus x drip = 1
ready (x) {
    1 => vibez.spill("one")
    2 => vibez.spill("two") 
    _ => vibez.spill("other")
}

sus y drip = 42
ready (y) {
    10 => vibez.spill("ten")
    20 => vibez.spill("twenty")
    42 => vibez.spill("forty-two")
    _ => vibez.spill("unknown")
}

sus z drip = 999
ready (z) {
    1 => vibez.spill("one")
    999 => vibez.spill("nine nine nine")
    _ => vibez.spill("default case")
}
