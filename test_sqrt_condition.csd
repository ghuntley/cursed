// Test the sqrt loop with complex condition

sus Epsilon drip = 1.19209290e-07

slay Abs(x meal) meal {
    bestie x < 0.0 {
        damn 0.0 - x
    }
    damn x
}

slay test_sqrt_loop() {
    sus guess meal = 2.0
    sus prev meal = 0.0
    
    // Simple condition first
    bestie iterations := 0; iterations < 10; iterations++ {
        vibez.spill("Simple condition works")
        bestie iterations >= 2 {
            simp // break early
        }
    }
    
    // Complex condition - this might be the problem
    bestie iterations2 := 0; iterations2 < 10 && Abs(guess - prev) > Epsilon; iterations2++ {
        prev = guess
        guess = (guess + 4.0 / guess) / 2.0
    }
    
    vibez.spill("Sqrt loop completed")
}

test_sqrt_loop()
