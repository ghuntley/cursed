slay main() {
    vibez.spill("Testing C-style for loops")
    
    fr fr Basic C-style for loop
    bestie i := 0; i < 5; i++ {
        vibez.spill("Loop iteration")
    }
    
    fr fr C-style for loop with decrement
    bestie j := 3; j > 0; j-- {
        vibez.spill("Countdown")
    }
    
    fr fr C-style for loop with custom assignment
    bestie k := 0; k < 10; k = k + 2 {
        vibez.spill("Step by 2")
    }
    
    vibez.spill("All C-style for loops completed!")
}
