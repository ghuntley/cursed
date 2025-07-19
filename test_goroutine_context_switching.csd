// Test real goroutine context switching implementation
yeet "testz"

// Function to be executed by goroutine
slay goroutine_work(id normie) lit {
    vibez.spill("Goroutine")
    vibez.spill(id)
    vibez.spill("starting")
    
    // Simulate some work
    sus i normie = 0
    bestie i < 5; i++ {
        vibez.spill("Goroutine")
        vibez.spill(id)
        vibez.spill("working -")
        vibez.spill(i)
        
        // Yield to other goroutines
        yolo
    }
    
    vibez.spill("Goroutine")
    vibez.spill(id)
    vibez.spill("completed")
    damn based
}

// Main function to test context switching
slay main() {
    vibez.spill("Testing goroutine context switching")
    
    // Spawn multiple goroutines to test context switching
    sus goroutine1 normie = stan goroutine_work(1)
    sus goroutine2 normie = stan goroutine_work(2)
    sus goroutine3 normie = stan goroutine_work(3)
    
    vibez.spill("Spawned 3 goroutines")
    
    // Let goroutines run for a bit
    sus i normie = 0
    bestie i < 10; i++ {
        vibez.spill("Main thread -")
        vibez.spill(i)
        yolo  // Yield to goroutines
    }
    
    vibez.spill("Goroutine context switching test completed")
}

main()
