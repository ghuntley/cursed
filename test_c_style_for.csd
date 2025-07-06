slay main() {
    // Test basic C-style for loop
    bestie i := 0; i < 5; i++ {
        vibez.spill("i is ", i)
    }
    
    // Test for loop with manual increment
    bestie j := 0; j < 3; j = j + 1 {
        vibez.spill("j is ", j)
    }
    
    // Test for loop with different initialization
    sus start normie = 10
    bestie k := start; k > 5; k-- {
        vibez.spill("k is ", k)
    }
    
    // Test infinite loop with break
    bestie ; ; {
        vibez.spill("infinite loop")
        ghosted
    }
}
