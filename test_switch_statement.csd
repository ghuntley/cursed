// Test switch statement compilation
sus value drip = 2

vibe_check value {
    mood 1: vibez.spill("One")
    mood 2: vibez.spill("Two") 
    mood 3: vibez.spill("Three")
    basic: vibez.spill("Default case")
}

vibez.spill("Switch completed")
