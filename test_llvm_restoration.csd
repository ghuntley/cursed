// Test program to verify LLVM code generation restoration
// This tests the key components that were restored

package main

import "fmt"
import "channel"

func test_vibez_operations() {
    // Test vibez.spill functionality
    vibez.spill("Hello, CURSED world!")
    
    // Test vibez.spillf functionality  
    vibez.spillf("Testing format: %s %d", "number", 42)
    
    // Test vibez.read functionality
    input := vibez.readln()
    vibez.spill("You entered: " + input)
}

func test_goroutines() {
    // Test goroutine spawning
    go func() {
        vibez.spill("Goroutine is running")
    }()
    
    // Test channel operations
    ch := make(chan int, 1)
    ch <- 42
    val := <-ch
    vibez.spillf("Channel value: %d", val)
}

func recursive_factorial(n int) int {
    // Test tail call optimization
    if n <= 1 {
        return 1
    }
    return n * recursive_factorial(n - 1)
}

func test_optimization_features() {
    // Test constant propagation
    const x = 10
    const y = 20
    result := x + y // Should be constant folded
    
    // Test dead code elimination
    unused_var := 100 // Should be eliminated if not used
    
    // Test loop optimization
    sum := 0
    for i := 0; i < 10; i++ {
        sum += i // Loop should be optimized
    }
    
    vibez.spillf("Sum: %d, Result: %d", sum, result)
}

func test_memory_management() {
    // Test GC allocation
    ptr := new(int)
    *ptr = 42
    vibez.spillf("Allocated value: %d", *ptr)
    // GC should handle deallocation
}

func main() {
    vibez.spill("=== CURSED LLVM Code Generation Test ===")
    
    test_vibez_operations()
    test_goroutines()
    
    fact := recursive_factorial(5)
    vibez.spillf("Factorial of 5: %d", fact)
    
    test_optimization_features()
    test_memory_management()
    
    vibez.spill("=== Test Complete ===")
}
