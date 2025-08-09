# Pattern Matching Test
sus value drip = 5

# Basic pattern matching
ready (value) {
    1 => vibez.spill("one")
    2 => vibez.spill("two")
    5 => vibez.spill("five")
    _ => vibez.spill("other")
}

# Range patterns
sus score drip = 85
ready (score) {
    0..59 => vibez.spill("F")
    60..69 => vibez.spill("D") 
    70..79 => vibez.spill("C")
    80..89 => vibez.spill("B")
    90..100 => vibez.spill("A")
    _ => vibez.spill("Invalid score")
}

# When guards
sus number drip = 42
ready (number) {
    n when n > 100 => vibez.spill("large number")
    n when n > 50 => vibez.spill("medium number")
    n when n > 0 => vibez.spill("small positive number")
    _ => vibez.spill("zero or negative")
}
