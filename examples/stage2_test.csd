// Simple test program for Stage 2 CURSED compiler
// Tests basic functionality of the self-hosting compiler

import "std/io"

// Simple arithmetic function
func add(a: int, b: int) int {
    return a + b
}

// Function with conditional logic
func max(a: int, b: int) int {
    if a > b {
        return a
    } else {
        return b
    }
}

// Function with loop
func factorial(n: int) int {
    if n <= 1 {
        return 1
    }
    
    let result: int = 1
    for i := 2; i <= n; i = i + 1 {
        result = result * i
    }
    return result
}

// Array operations
func sum_array(arr: []int) int {
    let total: int = 0
    for i := 0; i < len(arr); i = i + 1 {
        total = total + arr[i]
    }
    return total
}

// Main function demonstrating various features
func main() int {
    io.println("Stage 2 CURSED Compiler Test")
    
    // Test arithmetic
    let x: int = 10
    let y: int = 20
    let sum: int = add(x, y)
    
    // Test conditionals
    let maximum: int = max(x, y)
    
    // Test loops
    let fact5: int = factorial(5)
    
    // Test arrays
    let numbers: []int = [1, 2, 3, 4, 5]
    let array_sum: int = sum_array(numbers)
    
    // Simple output (Stage 2 has limited standard library)
    // Return success code
    return 0
}
