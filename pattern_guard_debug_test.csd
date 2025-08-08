sus x drip = 8
vibez.spill("Testing x =", x)
ready (x) { 
    n when n > 5 => vibez.spill("big number")
    n when n <= 5 => vibez.spill("small number")
    _ => vibez.spill("fallback")
}

sus y drip = 3
vibez.spill("Testing y =", y)
ready (y) { 
    n when n > 5 => vibez.spill("big number")
    n when n <= 5 => vibez.spill("small number")
    _ => vibez.spill("fallback")
}
