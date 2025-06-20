slay loop_examples() {
    // For loop
    bestie i := 0; i < 10; i++ {
        print(i)
        lowkey i == 5 {
            ghosted  // break
        }
        lowkey i == 2 {
            simp     // continue
        }
    }
    
    // Range-based for
    sus items = [1, 2, 3, 4, 5]
    bestie _, value := flex items {
        print(value)
    }
    
    // While loop
    sus x = 10
    periodt x > 0 {
        print(x)
        x--
    }
    
    // Infinite loop
    bestie {
        print("Running...")
        lowkey should_stop() {
            ghosted
        }
    }
}