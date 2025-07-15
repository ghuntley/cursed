sus count drip = 0

bestie i := 0; i < 10; i++ {
    lowkey i == 5 {
        simp  // continue statement
    }
    
    lowkey i == 8 {
        ghosted  // break statement
    }
    
    count = count + 1
    vibez.spill("Loop iteration: ", i)
}

vibez.spill("Final count: ", count)

// Test while loop with break/continue
sus j drip = 0
sus iterations drip = 0

lowkey j < 20 {
    j = j + 1
    iterations = iterations + 1
    
    lowkey j == 3 {
        simp  // continue
    }
    
    lowkey j == 7 {
        ghosted  // break
    }
    
    vibez.spill("While iteration: ", j)
}

vibez.spill("While iterations: ", iterations)
