slay main() {
    vibez.spill("Testing C-style for loop edge cases")
    
    fr fr Test empty init
    sus i normie = 0
    bestie ; i < 3; i++ {
        vibez.spill("Empty init works")
        i = 3 fr fr Break the loop
    }
    
    fr fr Test empty condition - infinite loop with break
    bestie j := 0; ; j++ {
        vibez.spill("Infinite loop works")
        vibe_check j >= 2 {
            ghosted
        }
    }
    
    fr fr Test empty update
    bestie k := 0; k < 2; {
        vibez.spill("Empty update works")
        k++
    }
    
    vibez.spill("All edge cases passed!")
}
