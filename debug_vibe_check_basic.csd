// Test extremely basic vibe_check with literals

yeet "vibez"

slay main() {
    sus value drip = 42
    
    vibez.spill("Testing value:", value)
    
    vibe_check value {
        mood 42:
            vibez.spill("Matched 42!")
        basic:
            vibez.spill("No match")
    }
    
    vibez.spill("Done")
}
