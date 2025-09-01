vibe main_character
yeet "vibez"

fr fr Comprehensive Control Flow Test  
fr fr Tests: if/else chains, nested conditions, while loops
fr fr Expected: All control structures work identically in both modes

slay main_character() {
    vibez.spill("=== Comprehensive Control Flow Test ===")
    
    fr fr Test basic if statements
    sus value normie = 15
    ready (value > 10) {
        vibez.spill("Value is greater than 10")
    } otherwise {
        vibez.spill("Value is 10 or less")
    }
    
    fr fr Test if-else chain
    sus grade normie = 75
    ready (grade >= 90) {
        vibez.spill("Grade: A")
    } otherwise ready (grade >= 80) {
        vibez.spill("Grade: B") 
    } otherwise ready (grade >= 70) {
        vibez.spill("Grade: C")
    } otherwise {
        vibez.spill("Grade: F")
    }
    
    fr fr Test nested conditions
    sus x normie = 12
    sus y normie = 8
    ready (x > 10) {
        ready (y < 10) {
            vibez.spill("Both conditions met")
        } otherwise {
            vibez.spill("Only first condition met")
        }
    } otherwise {
        vibez.spill("First condition not met")
    }
    
    fr fr Test boolean operations
    sus a normie = 5
    sus b normie = 3
    ready (a > b) {
        vibez.spill("Boolean test passed")
    }
    
    vibez.spill("=== Control Flow Test Complete ===")
}
