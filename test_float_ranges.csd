vibez.spill("Testing float range patterns...")

sus value normie = 3.14
ready (value) { 
    0.0..2.5 => vibez.spill("Small float")
    2.5..5.0 => vibez.spill("Medium float") 
    _ => vibez.spill("Large float")
}

sus value2 normie = 1.5
ready (value2) { 
    0.0..2.5 => vibez.spill("Small float 2")
    2.5..5.0 => vibez.spill("Medium float 2") 
    _ => vibez.spill("Large float 2")
}

vibez.spill("Float range tests completed!")
