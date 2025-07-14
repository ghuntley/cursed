# Basic Pattern Matching Test
yeet "testz"

# Test basic literal pattern matching
sus x normie = 42

vibe_check x {
    mood 42:
        vibez.spill("Matched 42")
    mood 24:
        vibez.spill("Matched 24")
    basic:
        vibez.spill("No match")
}

# Test wildcard pattern
sus y normie = 100

vibe_check y {
    mood _:
        vibez.spill("Wildcard matches anything")
}

# Test boolean exhaustiveness
sus flag lit = based

vibe_check flag {
    mood based:
        vibez.spill("Flag is true")
    mood cap:
        vibez.spill("Flag is false")
}

vibez.spill("Basic pattern matching test completed")
