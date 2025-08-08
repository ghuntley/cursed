sus value1 drip = 5
sus value2 drip = 25
sus value3 drip = 75

vibez.spill("Testing value1 =", value1)
ready (value1) { 
    0..10 => vibez.spill("small")
    11..50 => vibez.spill("medium")
    _ => vibez.spill("large")
}

vibez.spill("Testing value2 =", value2)
ready (value2) { 
    0..10 => vibez.spill("small")
    11..50 => vibez.spill("medium")
    _ => vibez.spill("large")
}

vibez.spill("Testing value3 =", value3)
ready (value3) { 
    0..10 => vibez.spill("small")
    11..50 => vibez.spill("medium")
    _ => vibez.spill("large")
}
