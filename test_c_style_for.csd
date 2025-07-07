vibe test_c_style_for

slay main() {
    vibez.spill("Testing C-style for loops")
    
    fr fr Basic C-style for loop
    bestie i := 0; i < 5; i++ {
        vibez.spill("Value of i:")
        vibez.spill(i)
    }
    
    fr fr C-style for loop with different update
    bestie j := 10; j > 0; j-- {
        vibez.spill("Countdown:")
        vibez.spill(j)
    }
    
    fr fr C-style for loop with custom step
    bestie k := 0; k < 20; k = k + 3 {
        vibez.spill("Step by 3:")
        vibez.spill(k)
    }
    
    vibez.spill("All C-style for loops completed!")
}
