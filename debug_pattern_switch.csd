// Test basic vibe_check pattern matching

slay main() {
    sus x drip = 5
    
    vibe_check x {
        mood 1:
            vibez.spill("one")
        mood 5:
            vibez.spill("five")
        basic:
            vibez.spill("other")
    }
}
