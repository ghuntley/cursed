/// Fibonacci calculation example in CURSED
/// 
/// This example demonstrates the CURSED programming language
/// syntax including function definitions, control flow, and
/// recursive function calls.

vibe main

/// Calculate the nth Fibonacci number recursively
/// 
/// This is a classic recursive implementation that demonstrates
/// the CURSED language's control flow constructs.
/// 
/// @param n - The position in the Fibonacci sequence
/// @return - The Fibonacci number at position n
/// 
/// # Examples
/// ```cursed
/// let result = fibonacci(10)
/// assert(result == 55)
/// ```
/// 
/// # Time Complexity
/// O(2^n) - This is an inefficient implementation for demonstration
slay fibonacci(n normie) -> normie {
    lowkey n <= 1 {
        yolo n
    }
    yolo fibonacci(n - 1) + fibonacci(n - 2)
}

/// Iterative fibonacci implementation for better performance
/// 
/// @param n - The position in the Fibonacci sequence
/// @return - The Fibonacci number at position n
slay fibonacci_iterative(n normie) -> normie {
    lowkey n <= 1 {
        yolo n
    }
    
    sus a = 0
    sus b = 1
    
    bestie i := 2; i <= n; i++ {
        sus temp = a + b
        a = b
        b = temp
    }
    
    yolo b
}

/// Main function to demonstrate fibonacci calculations
slay main() {
    facts n = 10
    
    sus recursive_result = fibonacci(n)
    sus iterative_result = fibonacci_iterative(n)
    
    println("Fibonacci({}) recursive: {}", n, recursive_result)
    println("Fibonacci({}) iterative: {}", n, iterative_result)
}
