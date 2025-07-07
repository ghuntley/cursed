vibe main

slay main() {
    // Test basic if statement with short variable declaration
    lowkey x := 5; x > 0 {
        vibez.spill("x is positive")
    }
    
    // Test if statement with assignment
    sus count normie = 10
    lowkey count = count + 1; count > 10 {
        vibez.spill("count is greater than 10")
    }
    
    // Test if statement with increment
    sus i normie = 0
    lowkey i++; i > 0 {
        vibez.spill("i is now positive")
    }
    
    // Test switch statement with simple statement prefix
    vibe_check y := 42; y {
        mood 42:
            vibez.spill("y is 42")
        basic:
            vibez.spill("y is something else")
    }
    
    // Test tuple destructuring in if statement
    lowkey (a, b) := (1, 2); a + b > 2 {
        vibez.spill("sum is greater than 2")
    }
    
    // Test that old syntax still works
    lowkey 5 > 3 {
        vibez.spill("old syntax still works")
    }
    
    // Test nested if statements with prefixes
    lowkey outer := 10; outer > 5 {
        lowkey inner := 20; inner > outer {
            vibez.spill("inner is greater than outer")
        }
    }
}
