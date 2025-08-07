// Test basic switch pattern matching with literals
sus x drip = 5

// Simple switch statement
switch (x) {
    case 1: vibez.spill("one")
    case 2: vibez.spill("two") 
    case 5: vibez.spill("matched five")
    case _: vibez.spill("no match")
}

// Test with multiple values in one case
sus y drip = 2
switch (y) {
    case 1, 2, 3: vibez.spill("small number")
    case 4, 5, 6: vibez.spill("medium number")
    case _: vibez.spill("other")
}

// Test with guard patterns
sus z drip = 15
switch (z) {
    case x ready (x > 10): vibez.spill("large:", x)
    case x ready (x < 5): vibez.spill("small:", x)
    case _: vibez.spill("medium")
}
