// Test pattern matching implementation
sus value normie = 42

vibe_check value {
    mood 42:
        vibez.spill("Found the answer!")
    mood 1, 2, 3:
        vibez.spill("Small number")
    basic:
        vibez.spill("Other value")
}

// Test another value
sus another_value normie = 2

vibe_check another_value {
    mood 1, 2, 3:
        vibez.spill("Found small number: 2")
    basic:
        vibez.spill("Not a small number")
}
